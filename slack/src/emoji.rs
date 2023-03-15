use crate::Client;
use crate::ClientResult;

pub struct Emoji {
    pub client: Client,
}

impl Emoji {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Emoji { client }
    }

    /**
     * This function performs a `GET` to the `/emoji.list` endpoint.
     *
     * Lists custom emoji for a team.
     *
     * FROM: <https://api.slack.com/methods/emoji.list>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `emoji:read`.
     */
    pub async fn list(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/emoji.list", None);
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
