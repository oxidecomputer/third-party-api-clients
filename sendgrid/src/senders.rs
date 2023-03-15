use crate::Client;
use crate::ClientResult;

pub struct Senders {
    pub client: Client,
}

impl Senders {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Senders { client }
    }

    /**
     * Create a Sender Identity.
     *
     * This function performs a `POST` to the `/marketing/senders` endpoint.
     *
     * **This endpoint allows you to create a new sender identity.**
     *
     * *You may create up to 100 unique sender identities.*
     *
     * Sender identities are required to be verified before use. If your domain has been authenticated, a new sender identity will auto verify on creation. Otherwise an email will be sent to the `from.email`.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_marketing(
        &self,
        body: &crate::types::PostMarketingSendersRequest,
    ) -> ClientResult<crate::types::SenderAllOf> {
        let url = self.client.url("/marketing/senders", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
}
