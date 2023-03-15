use crate::Client;
use crate::ClientResult;

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
    pub async fn get(&self) -> ClientResult<crate::types::Account> {
        let url = self.client.url("/account", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}
