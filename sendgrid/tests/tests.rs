use std::{mem, time::Duration};

use wiremock::{
    matchers::{method, path},
    Mock, MockServer, ResponseTemplate,
};

use sendgrid_api::{types::GetScopesResponse, Client};

#[tokio::test]
async fn test_uses_host_override() {
    let server = MockServer::start().await;
    let response = ResponseTemplate::new(200)
        .set_delay(Duration::from_secs(1))
        .set_body_json(GetScopesResponse { scopes: vec![] });

    Mock::given(method("GET"))
        .and(path("/scopes".to_string()))
        .respond_with(response)
        .expect(1)
        .mount(&server)
        .await;

    let mut client = Client::new("token");
    client.with_host_override(server.uri());

    let _ = client.api_key_permissions().get_scopes().await.unwrap();

    mem::drop(server)
}
