use crate::Client;
use crate::ClientResult;

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
    pub async fn admin_stop(&self, body: &crate::types::Channel) -> ClientResult<()> {
        let url = self.client.url("/admin/directory_v1/channels/stop", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
}
