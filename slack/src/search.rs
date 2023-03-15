use crate::Client;
use crate::ClientResult;

pub struct Search {
    pub client: Client,
}

impl Search {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Search { client }
    }

    /**
     * This function performs a `GET` to the `/search.messages` endpoint.
     *
     * Searches for messages matching a query.
     *
     * FROM: <https://api.slack.com/methods/search.messages>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `search:read`.
     * * `count: i64` -- Pass the number of results you want per "page". Maximum of `100`.
     * * `highlight: bool` -- Pass a value of `true` to enable query highlight markers (see below).
     * * `page: i64`
     * * `query: &str` -- Search query.
     * * `sort: &str` -- Return matches sorted by either `score` or `timestamp`.
     * * `sort_dir: &str` -- Change sort direction to ascending (`asc`) or descending (`desc`).
     */
    pub async fn message(
        &self,
        count: i64,
        highlight: bool,
        page: i64,
        query: &str,
        sort: &str,
        sort_dir: &str,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if highlight {
            query_args.push(("highlight".to_string(), highlight.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        if !sort.is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !sort_dir.is_empty() {
            query_args.push(("sort_dir".to_string(), sort_dir.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/search.messages?{}", query_), None);
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
