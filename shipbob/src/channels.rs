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
     * Get user-authorized channel info.
     *
     * This function performs a `GET` to the `/channel` endpoint.
     */
    pub async fn get_page(&self) -> ClientResult<Vec<crate::types::Channel>> {
        let url = self.client.url("/channel", None);
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
    /**
     * Get user-authorized channel info.
     *
     * This function performs a `GET` to the `/channel` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     */
    pub async fn get_all(&self) -> ClientResult<Vec<crate::types::Channel>> {
        let url = self.client.url("/channel", None);
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}
