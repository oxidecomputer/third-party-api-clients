use anyhow::Result;

use crate::Client;

pub struct Colors {
    pub client: Client,
}

impl Colors {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Colors { client }
    }

    /**
     * This function performs a `GET` to the `/colors` endpoint.
     *
     * Returns the color definitions for calendars and events.
     */
    pub async fn get(&self) -> Result<crate::types::Colors> {
        let url = "/colors".to_string();
        let url = self.client.url(&url, None);
        self.client.get(&url, None, None).await
    }
}
