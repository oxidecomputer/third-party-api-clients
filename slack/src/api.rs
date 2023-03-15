use crate::Client;
use crate::ClientResult;

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
     * * `foo_: &str` -- example property to return.
     */
    pub async fn test(&self, error: &str, foo_: &str) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !error.is_empty() {
            query_args.push(("error".to_string(), error.to_string()));
        }
        if !foo_.is_empty() {
            query_args.push(("foo".to_string(), foo_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/api.test?{}", query_), None);
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
