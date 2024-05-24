use crate::Client;
use crate::ClientResult;

pub struct Emojis {
    pub client: Client,
}

impl Emojis {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Emojis { client }
    }

    /**
     * Get emojis.
     *
     * This function performs a `GET` to the `/emojis` endpoint.
     *
     * Lists all the emojis available to use on GitHub.
     *
     * FROM: <https://docs.github.com/rest/reference/emojis#get-emojis>
     */
    pub async fn get(&self) -> ClientResult<crate::Response<String>> {
        let url = self.client.url("/emojis", None);
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
