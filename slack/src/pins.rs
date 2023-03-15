use crate::Client;
use crate::ClientResult;

pub struct Pins {
    pub client: Client,
}

impl Pins {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Pins { client }
    }

    /**
     * This function performs a `POST` to the `/pins.add` endpoint.
     *
     * Pins an item to a channel.
     *
     * FROM: <https://api.slack.com/methods/pins.add>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `pins:write`.
     */
    pub async fn add(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/pins.add", None);
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
     * This function performs a `GET` to the `/pins.list` endpoint.
     *
     * Lists items pinned to a channel.
     *
     * FROM: <https://api.slack.com/methods/pins.list>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `pins:read`.
     * * `channel: &str` -- Channel to get pinned items for.
     */
    pub async fn list(
        &self,
        channel: &str,
    ) -> ClientResult<Vec<crate::types::PinsListResponseAnyOf>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !channel.is_empty() {
            query_args.push(("channel".to_string(), channel.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/pins.list?{}", query_), None);
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
     * This function performs a `GET` to the `/pins.list` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Lists items pinned to a channel.
     *
     * FROM: <https://api.slack.com/methods/pins.list>
     */
    pub async fn list_all(
        &self,
        channel: &str,
    ) -> ClientResult<Vec<crate::types::PinsListResponseAnyOf>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !channel.is_empty() {
            query_args.push(("channel".to_string(), channel.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/pins.list?{}", query_), None);
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
    /**
     * This function performs a `POST` to the `/pins.remove` endpoint.
     *
     * Un-pins an item from a channel.
     *
     * FROM: <https://api.slack.com/methods/pins.remove>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `pins:write`.
     */
    pub async fn remove(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/pins.remove", None);
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
