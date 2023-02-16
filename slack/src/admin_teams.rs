use anyhow::Result;

use crate::Client;

pub struct AdminTeams {
    pub client: Client,
}

impl AdminTeams {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AdminTeams { client }
    }

    /**
     * This function performs a `POST` to the `/admin.teams.create` endpoint.
     *
     * Create an Enterprise team.
     *
     * FROM: <https://api.slack.com/methods/admin.teams.create>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.teams:write`.
     */
    pub async fn create(&self) -> Result<crate::types::DndEndSchema> {
        let url = "/admin.teams.create".to_string();
        let url = self.client.url(&url, None);
        self.client.post(&url, None).await
    }
    /**
     * This function performs a `GET` to the `/admin.teams.list` endpoint.
     *
     * List all teams on an Enterprise organization
     *
     * FROM: <https://api.slack.com/methods/admin.teams.list>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.teams:read`.
     * * `limit: i64` -- The maximum number of items to return. Must be between 1 - 100 both inclusive.
     * * `cursor: &str` -- Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
     */
    pub async fn list(&self, limit: i64, cursor: &str) -> Result<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/admin.teams.list?{}", query_);
        let url = self.client.url(&url, None);
        self.client.get(&url, None).await
    }
}
