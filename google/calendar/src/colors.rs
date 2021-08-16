use anyhow::Result;

use crate::Client;

pub struct Colors {
    client: Client,
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
    pub async fn calendar_get(&self) -> Result<crate::types::Colors> {
        let url = "/colors".to_string();
        self.client.get(&url, None).await
    }
}
