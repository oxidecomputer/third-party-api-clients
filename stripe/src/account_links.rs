use anyhow::Result;

use crate::Client;

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
    pub async fn post(&self) -> Result<crate::types::AccountLink> {
        let url = "/v1/account_links".to_string();
        let url = self.client.url(&url, None);
        self.client.post(&url, None).await
    }
}
