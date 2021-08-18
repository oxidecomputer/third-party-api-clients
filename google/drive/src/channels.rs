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
     * This function performs a `POST` to the `/channels/stop` endpoint.
     *
     * Stop watching resources through this channel
     */
    pub async fn stop(&self, body: &crate::types::Channel) -> Result<()> {
        let url = "/channels/stop".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
