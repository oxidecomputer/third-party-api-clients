use anyhow::Result;

use crate::Client;

pub struct Ping {
    pub client: Client,
}

impl Ping {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Ping { client }
    }

    /**
    * Ping.
    *
    * This function performs a `GET` to the `/ping` endpoint.
    *
    * A health check for the API that won't return any account-specific information.
    */
    pub async fn get(&self) -> Result<crate::types::ApiHealthStatus> {
        let url = "/ping".to_string();
        self.client.get(&url, None).await
    }
}
