use rand::RngCore;
use rsa::{pkcs1::EncodeRsaPrivateKey, RsaPrivateKey};
use std::{mem, time::Duration, time::SystemTime};

use wiremock::{
    http::{HeaderName, HeaderValue},
    matchers::{bearer_token, header, method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

use octorust::{
    auth::{Credentials, InstallationTokenGenerator, JWTCredentials},
    types::InstallationToken,
    Client, ClientError,
};

fn app_id() -> i64 {
    let mut rng = rand::thread_rng();
    rng.next_u32() as i64
}

fn installation_id() -> i64 {
    let mut rng = rand::thread_rng();
    rng.next_u32() as i64
}

fn private_key() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, 2048)
        .unwrap()
        .to_pkcs1_der()
        .unwrap()
        .to_bytes();
    private_key.to_vec()
}

fn empty_issue() -> octorust::types::Issue {
    octorust::types::Issue {
        active_lock_reason: String::new(),
        assignee: None,
        assignees: vec![],
        author_association: octorust::types::AuthorAssociation::None,
        body: String::new(),
        body_html: String::new(),
        body_text: String::new(),
        closed_at: None,
        closed_by: None,
        comments: 0,
        comments_url: String::new(),
        created_at: None,
        events_url: String::new(),
        html_url: String::new(),
        id: 0,
        labels: vec![],
        labels_url: String::new(),
        locked: false,
        milestone: None,
        node_id: String::new(),
        number: 0,
        performed_via_github_app: None,
        pull_request: None,
        reactions: None,
        repository: None,
        repository_url: String::new(),
        state: String::new(),
        timeline_url: String::new(),
        title: String::new(),
        updated_at: None,
        url: String::new(),
        user: None,
    }
}

#[tokio::test]
async fn test_follows_next_links_during_unfold() {
    let installation_id = installation_id();
    let jwt = JWTCredentials::new(app_id(), private_key()).expect("JWT creation should succeed");

    let server = MockServer::start().await;
    let auth_response = ResponseTemplate::new(200)
        .set_delay(Duration::from_secs(1))
        .set_body_json(InstallationToken {
            token: "test-token".to_owned(),
            expires_at: Default::default(),
            has_multiple_single_files: Default::default(),
            permissions: Default::default(),
            repositories: Default::default(),
            repository_selection: Default::default(),
            single_file: Default::default(),
            single_file_paths: Default::default(),
        });

    Mock::given(method("POST"))
        .and(path(format!(
            "/app/installations/{installation_id}/access_tokens"
        )))
        .and(bearer_token(jwt.token()))
        .respond_with(auth_response)
        .expect(1)
        .mount(&server)
        .await;

    let next_url = format!("{}/issues?per_page=100&page=2; rel=\"next\"", server.uri());

    let with_next = ResponseTemplate::new(200)
        .set_delay(Duration::from_secs(1))
        .append_header(
            HeaderName::from_bytes("link".as_bytes().to_vec()).unwrap(),
            HeaderValue::from_bytes(next_url.into_bytes()).unwrap(),
        )
        .set_body_json(vec![empty_issue(), empty_issue()]);

    let without_next = ResponseTemplate::new(200)
        .set_delay(Duration::from_secs(1))
        .set_body_json(vec![empty_issue()]);

    Mock::given(method("GET"))
        .and(path("/issues".to_string()))
        .and(query_param("direction", "asc"))
        .and(query_param("filter", "all"))
        .and(query_param("sort", "created"))
        .and(query_param("state", "all"))
        .respond_with(with_next)
        .expect(1)
        .named("Get first page")
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/issues".to_string()))
        .and(query_param("per_page", "100"))
        .and(query_param("page", "2"))
        .respond_with(without_next)
        .expect(1)
        .named("Get second page")
        .mount(&server)
        .await;

    let token_generator = InstallationTokenGenerator::new(installation_id, jwt);
    let mut client = Client::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Credentials::InstallationToken(token_generator),
    )
    .expect("Client creation should succeed");
    client.with_host_override(server.uri());

    let res = client
        .issues()
        .list_all(
            octorust::types::Filter::All,
            octorust::types::IssuesListState::All,
            "",
            octorust::types::IssuesListSort::Created,
            octorust::types::Order::Asc,
            None,
            false,
            false,
            false,
            false,
        )
        .await;

    mem::drop(server);

    assert_eq!(res.unwrap().body.len(), 3)
}

#[tokio::test]
async fn test_refreshes_installation_token_once() {
    let installation_id = installation_id();

    let server = MockServer::start().await;

    let jwt = JWTCredentials::new(app_id(), private_key()).expect("JWT creation should succeed");

    // The JWT should be used to retrieve an installation token only once, even if requesting the
    // the token takes long enough for a second task to ask for one.
    let auth_response = ResponseTemplate::new(200)
        .set_delay(Duration::from_secs(1))
        .set_body_json(InstallationToken {
            token: "test-token".to_owned(),
            expires_at: Default::default(),
            has_multiple_single_files: Default::default(),
            permissions: Default::default(),
            repositories: Default::default(),
            repository_selection: Default::default(),
            single_file: Default::default(),
            single_file_paths: Default::default(),
        });
    Mock::given(method("POST"))
        .and(path(format!(
            "/app/installations/{installation_id}/access_tokens"
        )))
        .and(bearer_token(jwt.token()))
        .respond_with(auth_response)
        .expect(1)
        .mount(&server)
        .await;

    // We'll use the zen endpoint just to exercise the installation token generation.
    Mock::given(method("GET"))
        .and(path("/zen"))
        .and(header("authorization", "token test-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json("ok"))
        .mount(&server)
        .await;

    let token_generator = InstallationTokenGenerator::new(installation_id, jwt);
    let mut client = Client::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Credentials::InstallationToken(token_generator),
    )
    .expect("Client creation should succeed");
    client.with_host_override(server.uri());

    // Request zen twice at the same time.
    let meta = client.meta();
    let result = tokio::try_join!(meta.get_zen(), meta.get_zen());

    // Drop the server now because the server gives more useful errors on authentication failure.
    mem::drop(server);

    // Ensure the requests both completed successfully.
    result.expect("Should get zen successfully");
}

#[tokio::test]
async fn test_ratelimit_error() {
    let installation_id = installation_id();

    let server = MockServer::start().await;

    let jwt = JWTCredentials::new(app_id(), private_key()).expect("JWT creation should succeed");

    // The JWT should be used to retrieve an installation token only once, even if requesting the
    // the token takes long enough for a second task to ask for one.
    let auth_response = ResponseTemplate::new(200)
        .set_delay(Duration::from_secs(1))
        .set_body_json(InstallationToken {
            token: "test-token".to_owned(),
            expires_at: Default::default(),
            has_multiple_single_files: Default::default(),
            permissions: Default::default(),
            repositories: Default::default(),
            repository_selection: Default::default(),
            single_file: Default::default(),
            single_file_paths: Default::default(),
        });
    Mock::given(method("POST"))
        .and(path(format!(
            "/app/installations/{installation_id}/access_tokens"
        )))
        .and(bearer_token(jwt.token()))
        .respond_with(auth_response)
        .expect(1)
        .mount(&server)
        .await;

    // We'll use the zen endpoint just to exercise the rate limit error.
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        + Duration::from_secs(60);
    Mock::given(method("GET"))
        .and(path("/zen"))
        .and(header("authorization", "token test-token"))
        .respond_with(
            ResponseTemplate::new(400)
                .append_header("x-ratelimit-remaining", "0")
                .append_header("x-ratelimit-reset", format!("{}", now.as_secs()).as_str()),
        )
        .mount(&server)
        .await;

    let token_generator = InstallationTokenGenerator::new(installation_id, jwt);
    let mut client = Client::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Credentials::InstallationToken(token_generator),
    )
    .expect("Client creation should succeed");
    client.with_host_override(server.uri());

    // Request zen.
    let result = client.meta().get_zen().await;

    // Drop the server now because the server gives more useful errors on authentication failure.
    mem::drop(server);

    // Ensure the request failed.
    let err = result.expect_err("get zen should fail");
    if let ClientError::RateLimited { duration } = err {
        /*
        We should expect a duration of 60, but between the delay in the auth request and
        possible change of seconds between the computation of `now` and the actual call,
        this should be returning 59 most of the time, and 58 when the second changes.
        */
        assert!(
            (58..=60).contains(&duration),
            "duration {} is not within range",
            duration
        );
    } else {
        unreachable!("Expected Ratelimiting error, got {:?}", err)
    }
}

#[tokio::test]
async fn test_does_not_follow_redirects() {
    let installation_id = installation_id();
    let jwt = JWTCredentials::new(app_id(), private_key()).expect("JWT creation should succeed");

    let server = MockServer::start().await;
    let auth_response = ResponseTemplate::new(200)
        .set_delay(Duration::from_secs(1))
        .set_body_json(InstallationToken {
            token: "test-token".to_owned(),
            expires_at: Default::default(),
            has_multiple_single_files: Default::default(),
            permissions: Default::default(),
            repositories: Default::default(),
            repository_selection: Default::default(),
            single_file: Default::default(),
            single_file_paths: Default::default(),
        });

    Mock::given(method("POST"))
        .and(path(format!(
            "/app/installations/{installation_id}/access_tokens"
        )))
        .and(bearer_token(jwt.token()))
        .respond_with(auth_response)
        .expect(1)
        .mount(&server)
        .await;

    let download_path = "/fake-download-path";

    Mock::given(method("GET"))
        .and(path(
            "/repos/test_owner/test_repo/actions/artifacts/12345/test_fmt".to_string(),
        ))
        .respond_with(
            ResponseTemplate::new(302)
                .set_delay(Duration::from_secs(1))
                .append_header(
                    HeaderName::from_bytes("Location".as_bytes().to_vec()).unwrap(),
                    HeaderValue::from_bytes(
                        format!("{}{}", server.uri(), download_path).into_bytes(),
                    )
                    .unwrap(),
                )
                .set_body_bytes(vec![]),
        )
        .expect(1)
        .named("Get artifact location")
        .mount(&server)
        .await;

    let token_generator = InstallationTokenGenerator::new(installation_id, jwt);
    let mut client = Client::new(
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Credentials::InstallationToken(token_generator),
    )
    .expect("Client creation should succeed");
    client.with_host_override(server.uri());

    let res = client
        .actions()
        .download_artifact("test_owner", "test_repo", 12345, "test_fmt")
        .await;

    mem::drop(server);

    assert!(res
        .unwrap()
        .headers
        .get("Location")
        .unwrap()
        .to_str()
        .unwrap()
        .ends_with("/fake-download-path"));
}
