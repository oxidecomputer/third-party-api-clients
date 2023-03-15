use crate::Client;
use crate::ClientResult;

pub struct AppsEventAuthorizations {
    pub client: Client,
}

impl AppsEventAuthorizations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AppsEventAuthorizations { client }
    }

    /**
     * This function performs a `GET` to the `/apps.event.authorizations.list` endpoint.
     *
     * Get a list of authorizations for the given event context. Each authorization represents an app installation that the event is visible to.
     *
     * FROM: <https://api.slack.com/methods/apps.event.authorizations.list>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `authorizations:read`.
     * * `event_context: &str`
     * * `cursor: &str`
     * * `limit: i64`
     */
    pub async fn list(
        &self,
        event_context: &str,
        cursor: &str,
        limit: i64,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if !event_context.is_empty() {
            query_args.push(("event_context".to_string(), event_context.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/apps.event.authorizations.list?{}", query_), None);
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
