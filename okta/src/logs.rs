use crate::Client;
use crate::ClientResult;

pub struct Logs {
    pub client: Client,
}

impl Logs {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Logs { client }
    }

    /**
     * Fetch a list of events from your Okta organization system log.
     *
     * This function performs a `GET` to the `/api/v1/logs` endpoint.
     *
     * The Okta System Log API provides read access to your organization’s system log. This API provides more functionality than the Events API
     *
     * **Parameters:**
     *
     * * `since: chrono::DateTime<chrono::Utc>`
     * * `until: chrono::DateTime<chrono::Utc>`
     * * `filter: &str`
     * * `q: &str`
     * * `limit: i64`
     * * `sort_order: &str`
     * * `after: &str`
     */
    pub async fn get_page(
        &self,
        since: Option<chrono::DateTime<chrono::Utc>>,
        until: Option<chrono::DateTime<chrono::Utc>>,
        filter: &str,
        q: &str,
        limit: i64,
        sort_order: &str,
        after: &str,
    ) -> ClientResult<Vec<crate::types::LogEvent>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        if !sort_order.is_empty() {
            query_args.push(("sortOrder".to_string(), sort_order.to_string()));
        }
        if let Some(date) = until {
            query_args.push(("until".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/api/v1/logs?{}", query_), None);
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
     * Fetch a list of events from your Okta organization system log.
     *
     * This function performs a `GET` to the `/api/v1/logs` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * The Okta System Log API provides read access to your organization’s system log. This API provides more functionality than the Events API
     */
    pub async fn get_all(
        &self,
        since: Option<chrono::DateTime<chrono::Utc>>,
        until: Option<chrono::DateTime<chrono::Utc>>,
        filter: &str,
        q: &str,
        sort_order: &str,
    ) -> ClientResult<Vec<crate::types::LogEvent>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        if !sort_order.is_empty() {
            query_args.push(("sortOrder".to_string(), sort_order.to_string()));
        }
        if let Some(date) = until {
            query_args.push(("until".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/api/v1/logs?{}", query_), None);
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
}
