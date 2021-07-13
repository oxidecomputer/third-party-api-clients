use anyhow::Result;

use crate::Client;

pub struct Meta {
    client: Client,
}

impl Meta {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Meta { client }
    }

    /**
     * Get the Zen of GitHub.
     *
     * This function performs a `GET` to the `/zen` endpoint.
     *
     * Get a random sentence from the Zen of GitHub
     */
    pub async fn get_zen(&self) -> Result<String> {
        let url = "/zen".to_string();
        self.client.get(&url).await
    }
}
