use crate::Client;
use crate::ClientResult;

pub struct Mandates {
    pub client: Client,
}

impl Mandates {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Mandates { client }
    }

    /**
     * This function performs a `GET` to the `/v1/mandates/{mandate}` endpoint.
     *
     * <p>Retrieves a Mandate object.</p>
     *
     * **Parameters:**
     *
     * * `mandate` -- The account's country.
     */
    pub async fn get(&self, mandate: &str) -> ClientResult<crate::Response<crate::types::Mandate>> {
        let url = self.client.url(
            &format!(
                "/v1/mandates/{}",
                crate::progenitor_support::encode_path(mandate),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
}
