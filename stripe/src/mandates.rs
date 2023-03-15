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
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `mandate: &str` -- The account's country.
     */
    pub async fn get(&self, mandate: &str) -> ClientResult<crate::types::Mandate> {
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
