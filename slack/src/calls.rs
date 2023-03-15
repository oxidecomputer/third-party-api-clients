use crate::Client;
use crate::ClientResult;

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
    pub async fn add(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/calls.add", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
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
    pub async fn end(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/calls.end", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
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
    pub async fn info(&self, id: &str) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !id.is_empty() {
            query_args.push(("id".to_string(), id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/calls.info?{}", query_), None);
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
    pub async fn update(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/calls.update", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
}
