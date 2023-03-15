use crate::Client;
use crate::ClientResult;

pub struct Stars {
    pub client: Client,
}

impl Stars {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Stars { client }
    }

    /**
     * This function performs a `POST` to the `/stars.add` endpoint.
     *
     * Adds a star to an item.
     *
     * FROM: <https://api.slack.com/methods/stars.add>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `stars:write`.
     */
    pub async fn add(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/stars.add", None);
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
     * This function performs a `GET` to the `/stars.list` endpoint.
     *
     * Lists stars for a user.
     *
     * FROM: <https://api.slack.com/methods/stars.list>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `stars:read`.
     * * `count: &str`
     * * `page: &str`
     * * `cursor: &str` -- Parameter for pagination. Set `cursor` equal to the `next_cursor` attribute returned by the previous request's `response_metadata`. This parameter is optional, but pagination is mandatory: the default value simply fetches the first "page" of the collection. See [pagination](/docs/pagination) for more details.
     * * `limit: i64` -- The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached.
     */
    pub async fn list(
        &self,
        count: &str,
        page: &str,
        cursor: &str,
        limit: i64,
    ) -> ClientResult<crate::types::StarsListSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !count.is_empty() {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page.is_empty() {
            query_args.push(("page".to_string(), page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/stars.list?{}", query_), None);
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
     * This function performs a `POST` to the `/stars.remove` endpoint.
     *
     * Removes a star from an item.
     *
     * FROM: <https://api.slack.com/methods/stars.remove>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `stars:write`.
     */
    pub async fn remove(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/stars.remove", None);
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
