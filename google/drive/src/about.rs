use anyhow::Result;

use crate::Client;

pub struct About {
    pub client: Client,
}

impl About {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        About { client }
    }

    /**
     * This function performs a `GET` to the `/about` endpoint.
     *
     * Gets information about the user, the user's Drive, and system capabilities.
     */
    pub async fn get(&self) -> Result<crate::types::About> {
        let url = "/about".to_string();
        self.client.get(&url, None).await
    }
}
