use rand::RngCore;
use rsa::{RsaPrivateKey, pkcs1::EncodeRsaPrivateKey};
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

pub(crate) fn app_id() -> u64 {
    let mut rng = rand::thread_rng();
    rng.next_u64()
}

pub(crate) fn installation_id() -> u64 {
    let mut rng = rand::thread_rng();
    rng.next_u64()
}

pub(crate) fn private_key() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap().to_pkcs1_der().unwrap().to_bytes();
    private_key.to_vec()
}

#[tokio::test]
async fn refreshes_installation_token_once() {
    let installation_id = installation_id();

    let server = MockServer::start().await;

    let jwt =
        JWTCredentials::new(app_id(), private_key()).expect("JWT creation should succeed");

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
    let client = Client::host(
        server.uri(),
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
        Credentials::InstallationToken(token_generator),
    )
    .expect("Client creation should succeed");

    // Request zen twice at the same time.
    let meta = client.meta();
    let result = tokio::try_join!(meta.get_zen(), meta.get_zen());

    // Drop the server now because the server gives more useful errors on authentication failure.
    mem::drop(server);

    // Ensure the requests both completed successfully.
    result.expect("Should get zen successfully");
}
