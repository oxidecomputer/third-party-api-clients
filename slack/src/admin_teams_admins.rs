use anyhow::Result;

use crate::Client;

pub struct AdminTeamsAdmins {
    pub client: Client,
}

impl AdminTeamsAdmins {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AdminTeamsAdmins { client }
    }

    /**
    * This function performs a `GET` to the `/admin.teams.admins.list` endpoint.
    *
    * List all of the admins on a given workspace.
    *
    * FROM: <https://api.slack.com/methods/admin.teams.admins.list>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `admin.teams:read`.
    * * `limit: i64` -- The maximum number of items to return.
    * * `cursor: &str` -- Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
    * * `team_id: &str`
    */
    pub async fn list(
        &self,
        limit: i64,
        cursor: &str,
        team_id: &str,
    ) -> Result<crate::types::DndEndSchema> {
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
        let url = format!("/admin.teams.admins.list?{}", query_);

        self.client.get(&url, None).await
    }
}
