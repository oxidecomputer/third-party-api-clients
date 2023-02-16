use anyhow::Result;

use crate::Client;

pub struct Account {
    pub client: Client,
}

impl Account {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Account { client }
    }

    /**
     * Get Account.
     *
     * This function performs a `GET` to the `/account` endpoint.
     *
     * Get the developer's account information
     */
    pub async fn get(&self) -> Result<crate::types::Account> {
        let url = "/account".to_string();
        let url = self.client.url(&url, None);
        self.client.get(&url, None).await
    }
}
