use anyhow::Result;

use crate::Client;

pub struct Api {
    pub client: Client,
}

impl Api {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Api { client }
    }

    /**
     * This function performs a `GET` to the `/api.test` endpoint.
     *
     * Checks API calling code.
     *
     * FROM: <https://api.slack.com/methods/api.test>
     *
     * **Parameters:**
     *
     * * `error: &str` -- Error response to return.
     * * `foo: &str` -- example property to return.
     */
    pub async fn test(&self, error: &str, foo: &str) -> Result<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !error.is_empty() {
            query_args.push(("error".to_string(), error.to_string()));
        }
        if !foo.is_empty() {
            query_args.push(("foo".to_string(), foo.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/api.test?{}", query_);

        self.client.get(&url, None).await
    }
}
