use anyhow::Result;

use crate::Client;

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
    pub async fn get(&self) -> Result<String> {
        let url = "/emojis".to_string();
        self.client.get(&url, None).await
    }
}
