use crate::Client;
use crate::ClientResult;

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
    pub async fn get(&self) -> ClientResult<crate::types::About> {
        let url = self.client.url("/about", None);
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
