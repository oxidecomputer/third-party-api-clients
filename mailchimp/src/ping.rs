use crate::Client;
use crate::ClientResult;

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
    pub async fn get(&self) -> ClientResult<crate::types::ApiHealthStatus> {
        let url = self.client.url("/ping", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}
