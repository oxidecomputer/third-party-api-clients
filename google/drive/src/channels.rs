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
     * This function performs a `POST` to the `/channels/stop` endpoint.
     *
     * Stop watching resources through this channel
     */
    pub async fn stop(&self, body: &crate::types::Channel) -> ClientResult<crate::Response<()>> {
        let url = self.client.url("/channels/stop", None);
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
