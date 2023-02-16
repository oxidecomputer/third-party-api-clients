use anyhow::Result;

use crate::Client;

pub struct Calls {
    pub client: Client,
}

impl Calls {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Calls { client }
    }

    /**
     * This function performs a `POST` to the `/calls.add` endpoint.
     *
     * Registers a new Call.
     *
     * FROM: <https://api.slack.com/methods/calls.add>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `calls:write`.
     */
    pub async fn add(&self) -> Result<crate::types::DndEndSchema> {
        let url = "/calls.add".to_string();
        let url = self.client.url(&url, None);
        self.client.post(&url, None).await
    }
    /**
     * This function performs a `POST` to the `/calls.end` endpoint.
     *
     * Ends a Call.
     *
     * FROM: <https://api.slack.com/methods/calls.end>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `calls:write`.
     */
    pub async fn end(&self) -> Result<crate::types::DndEndSchema> {
        let url = "/calls.end".to_string();
        let url = self.client.url(&url, None);
        self.client.post(&url, None).await
    }
    /**
     * This function performs a `GET` to the `/calls.info` endpoint.
     *
     * Returns information about a Call.
     *
     * FROM: <https://api.slack.com/methods/calls.info>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `calls:read`.
     * * `id: &str` -- `id` of the Call returned by the [`calls.add`](/methods/calls.add) method.
     */
    pub async fn info(&self, id: &str) -> Result<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !id.is_empty() {
            query_args.push(("id".to_string(), id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/calls.info?{}", query_);
        let url = self.client.url(&url, None);
        self.client.get(&url, None).await
    }
    /**
     * This function performs a `POST` to the `/calls.update` endpoint.
     *
     * Updates information about a Call.
     *
     * FROM: <https://api.slack.com/methods/calls.update>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `calls:write`.
     */
    pub async fn update(&self) -> Result<crate::types::DndEndSchema> {
        let url = "/calls.update".to_string();
        let url = self.client.url(&url, None);
        self.client.post(&url, None).await
    }
}
