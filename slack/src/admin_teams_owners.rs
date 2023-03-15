use crate::Client;
use crate::ClientResult;

pub struct AdminTeamsOwners {
    pub client: Client,
}

impl AdminTeamsOwners {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AdminTeamsOwners { client }
    }

    /**
     * This function performs a `GET` to the `/admin.teams.owners.list` endpoint.
     *
     * List all of the owners on a given workspace.
     *
     * FROM: <https://api.slack.com/methods/admin.teams.owners.list>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.teams:read`.
     * * `team_id: &str`
     * * `limit: i64` -- The maximum number of items to return. Must be between 1 - 1000 both inclusive.
     * * `cursor: &str` -- Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
     */
    pub async fn list(
        &self,
        team_id: &str,
        limit: i64,
        cursor: &str,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
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
            .url(&format!("/admin.teams.owners.list?{}", query_), None);
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
