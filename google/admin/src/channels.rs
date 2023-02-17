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
     * This function performs a `POST` to the `/admin/directory_v1/channels/stop` endpoint.
     *
     * Stops watching resources through this channel.
     */
    pub async fn admin_stop(&self, body: &crate::types::Channel) -> Result<()> {
        let url = self.client.url("/admin/directory_v1/channels/stop", None);
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                Some("application/json"),
            )
            .await
    }
}
