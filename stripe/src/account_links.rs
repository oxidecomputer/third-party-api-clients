use crate::Client;
use crate::ClientResult;

pub struct AccountLinks {
    pub client: Client,
}

impl AccountLinks {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AccountLinks { client }
    }

    /**
     * This function performs a `POST` to the `/v1/account_links` endpoint.
     *
     * <p>Creates an AccountLink object that includes a single-use Stripe URL that the platform can redirect their user to in order to take them through the Connect Onboarding flow.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::AccountLink> {
        let url = self.client.url("/v1/account_links", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
}
