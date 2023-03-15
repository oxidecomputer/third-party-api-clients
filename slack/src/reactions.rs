use crate::Client;
use crate::ClientResult;

pub struct Reactions {
    pub client: Client,
}

impl Reactions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Reactions { client }
    }

    /**
     * This function performs a `POST` to the `/reactions.add` endpoint.
     *
     * Adds a reaction to an item.
     *
     * FROM: <https://api.slack.com/methods/reactions.add>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `reactions:write`.
     */
    pub async fn add(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/reactions.add", None);
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
     * This function performs a `GET` to the `/reactions.get` endpoint.
     *
     * Gets reactions for an item.
     *
     * FROM: <https://api.slack.com/methods/reactions.get>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `reactions:read`.
     * * `channel: &str` -- Channel where the message to get reactions for was posted.
     * * `file: &str` -- File to get reactions for.
     * * `file_comment: &str` -- File comment to get reactions for.
     * * `full: bool` -- If true always return the complete reaction list.
     * * `timestamp: &str` -- Timestamp of the message to get reactions for.
     */
    pub async fn get(
        &self,
        channel: &str,
        file: &str,
        file_comment: &str,
        full: bool,
        timestamp: &str,
    ) -> ClientResult<crate::types::Fields> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !channel.is_empty() {
            query_args.push(("channel".to_string(), channel.to_string()));
        }
        if !file.is_empty() {
            query_args.push(("file".to_string(), file.to_string()));
        }
        if !file_comment.is_empty() {
            query_args.push(("file_comment".to_string(), file_comment.to_string()));
        }
        if full {
            query_args.push(("full".to_string(), full.to_string()));
        }
        if !timestamp.is_empty() {
            query_args.push(("timestamp".to_string(), timestamp.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/reactions.get?{}", query_), None);
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
     * This function performs a `GET` to the `/reactions.list` endpoint.
     *
     * Lists reactions made by a user.
     *
     * FROM: <https://api.slack.com/methods/reactions.list>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `reactions:read`.
     * * `user: &str` -- Show reactions made by this user. Defaults to the authed user.
     * * `full: bool` -- If true always return the complete reaction list.
     * * `count: i64`
     * * `page: i64`
     * * `cursor: &str` -- Parameter for pagination. Set `cursor` equal to the `next_cursor` attribute returned by the previous request's `response_metadata`. This parameter is optional, but pagination is mandatory: the default value simply fetches the first "page" of the collection. See [pagination](/docs/pagination) for more details.
     * * `limit: i64` -- The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached.
     */
    pub async fn list(
        &self,
        user: &str,
        full: bool,
        count: i64,
        page: i64,
        cursor: &str,
        limit: i64,
    ) -> ClientResult<crate::types::ReactionsListSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if full {
            query_args.push(("full".to_string(), full.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if !user.is_empty() {
            query_args.push(("user".to_string(), user.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/reactions.list?{}", query_), None);
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
     * This function performs a `POST` to the `/reactions.remove` endpoint.
     *
     * Removes a reaction from an item.
     *
     * FROM: <https://api.slack.com/methods/reactions.remove>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `reactions:write`.
     */
    pub async fn remove(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/reactions.remove", None);
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
