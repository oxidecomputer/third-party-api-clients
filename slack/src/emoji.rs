use anyhow::Result;

use crate::Client;

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
    pub async fn list(&self) -> Result<crate::types::DndEndSchema> {
        let url = "/emoji.list".to_string();
        self.client.get(&url, None).await
    }
}
