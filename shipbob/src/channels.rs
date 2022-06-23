use anyhow::Result;

use crate::Client;

pub struct Channels {
    pub client: Client,
}

impl Channels {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Channels { client }
    }

    /**
     * Get user-authorized channel info.
     *
     * This function performs a `GET` to the `/channel` endpoint.
     */
    pub async fn get_page(&self) -> Result<Vec<crate::types::Channel>> {
        let url = "/channel".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Get user-authorized channel info.
     *
     * This function performs a `GET` to the `/channel` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     */
    pub async fn get_all(&self) -> Result<Vec<crate::types::Channel>> {
        let url = "/channel".to_string();
        self.client.get_all_pages(&url, None).await
    }
}
