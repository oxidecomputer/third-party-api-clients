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
    pub async fn list(&self, token: &str) -> Result<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !token.is_empty() {
            query_args.push(("token".to_string(), token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/emoji.list?{}", query_);

        self.client.get(&url, None).await
    }
}
