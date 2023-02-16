use anyhow::Result;

use crate::Client;

pub struct Dnd {
    pub client: Client,
}

impl Dnd {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Dnd { client }
    }

    /**
     * This function performs a `POST` to the `/dnd.endDnd` endpoint.
     *
     * Ends the current user's Do Not Disturb session immediately.
     *
     * FROM: <https://api.slack.com/methods/dnd.endDnd>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `dnd:write`.
     */
    pub async fn end(&self) -> Result<crate::types::DndEndSchema> {
        let url = "/dnd.endDnd".to_string();
        let url = self.client.url(&url, None);
        self.client.post(&url, None, None).await
    }
    /**
     * This function performs a `POST` to the `/dnd.endSnooze` endpoint.
     *
     * Ends the current user's snooze mode immediately.
     *
     * FROM: <https://api.slack.com/methods/dnd.endSnooze>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `dnd:write`.
     */
    pub async fn end_snooze(&self) -> Result<crate::types::DndEndSnoozeSchema> {
        let url = "/dnd.endSnooze".to_string();
        let url = self.client.url(&url, None);
        self.client.post(&url, None, None).await
    }
    /**
     * This function performs a `GET` to the `/dnd.info` endpoint.
     *
     * Retrieves a user's current Do Not Disturb status.
     *
     * FROM: <https://api.slack.com/methods/dnd.info>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `dnd:read`.
     * * `user: &str` -- User to fetch status for (defaults to current user).
     */
    pub async fn info(&self, user: &str) -> Result<crate::types::DndInfoSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !user.is_empty() {
            query_args.push(("user".to_string(), user.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/dnd.info?{}", query_);
        let url = self.client.url(&url, None);
        self.client.get(&url, None, None).await
    }
    /**
     * This function performs a `POST` to the `/dnd.setSnooze` endpoint.
     *
     * Turns on Do Not Disturb mode for the current user, or changes its duration.
     *
     * FROM: <https://api.slack.com/methods/dnd.setSnooze>
     */
    pub async fn set_snooze(&self) -> Result<crate::types::DndSetSnoozeSchema> {
        let url = "/dnd.setSnooze".to_string();
        let url = self.client.url(&url, None);
        self.client
            .post(&url, None, Some("application/x-www-form-urlencoded"))
            .await
    }
    /**
     * This function performs a `GET` to the `/dnd.teamInfo` endpoint.
     *
     * Retrieves the Do Not Disturb status for up to 50 users on a team.
     *
     * FROM: <https://api.slack.com/methods/dnd.teamInfo>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `dnd:read`.
     * * `users: &str` -- Comma-separated list of users to fetch Do Not Disturb status for.
     */
    pub async fn team_info(&self, users: &str) -> Result<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !users.is_empty() {
            query_args.push(("users".to_string(), users.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/dnd.teamInfo?{}", query_);
        let url = self.client.url(&url, None);
        self.client.get(&url, None, None).await
    }
}
