use std::{mem, time::Duration};

use wiremock::{
    matchers::{bearer_token, header, method, path},
    Mock, MockServer, ResponseTemplate,
};

use crate::{
    auth::{Credentials, InstallationTokenGenerator, JWTCredentials},
    types::InstallationToken,
    Client,
};

const APP_ID: u64 = 432376605;
const INSTALLATION_ID: u64 = 260162224;
const PRIVATE_KEY: &[u8] = include_bytes!("dummy-rsa.der");

#[tokio::test]
async fn refreshes_installation_token_once() {
    let server = MockServer::start().await;

    let jwt =
        JWTCredentials::new(APP_ID, Vec::from(PRIVATE_KEY)).expect("JWT creation should succeed");

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
            "/app/installations/{INSTALLATION_ID}/access_tokens"
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

    let token_generator = InstallationTokenGenerator::new(INSTALLATION_ID, jwt);
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
