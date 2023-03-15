use crate::Client;
use crate::ClientResult;

pub struct AdminAppsRestricted {
    pub client: Client,
}

impl AdminAppsRestricted {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AdminAppsRestricted { client }
    }

    /**
     * This function performs a `GET` to the `/admin.apps.restricted.list` endpoint.
     *
     * List restricted apps for an org or workspace.
     *
     * FROM: <https://api.slack.com/methods/admin.apps.restricted.list>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.apps:read`.
     * * `limit: i64` -- The maximum number of items to return. Must be between 1 - 1000 both inclusive.
     * * `cursor: &str` -- Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
     * * `team_id: &str`
     * * `enterprise_id: &str`
     */
    pub async fn list(
        &self,
        limit: i64,
        cursor: &str,
        team_id: &str,
        enterprise_id: &str,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if !enterprise_id.is_empty() {
            query_args.push(("enterprise_id".to_string(), enterprise_id.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !team_id.is_empty() {
            query_args.push(("team_id".to_string(), team_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin.apps.restricted.list?{}", query_), None);
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
