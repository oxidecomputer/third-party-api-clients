use anyhow::Result;

use crate::Client;

pub struct AdminConversationsRestrictAccess {
    pub client: Client,
}

impl AdminConversationsRestrictAccess {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AdminConversationsRestrictAccess { client }
    }

    /**
     * This function performs a `POST` to the `/admin.conversations.restrictAccess.addGroup` endpoint.
     *
     * Add an allowlist of IDP groups for accessing a channel
     *
     * FROM: <https://api.slack.com/methods/admin.conversations.restrictAccess.addGroup>
     */
    pub async fn add_group(&self) -> Result<crate::types::DndEndSchema> {
        let url = "/admin.conversations.restrictAccess.addGroup".to_string();
        let url = self.client.url(&url, None);
        self.client
            .post(&url, None, Some("application/x-www-form-urlencoded"))
            .await
    }
    /**
     * This function performs a `GET` to the `/admin.conversations.restrictAccess.listGroups` endpoint.
     *
     * List all IDP Groups linked to a channel
     *
     * FROM: <https://api.slack.com/methods/admin.conversations.restrictAccess.listGroups>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.conversations:read`.
     * * `channel_id: &str`
     * * `team_id: &str` -- The workspace where the channel exists. This argument is required for channels only tied to one workspace, and optional for channels that are shared across an organization.
     */
    pub async fn list_group(
        &self,
        channel_id: &str,
        team_id: &str,
    ) -> Result<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !channel_id.is_empty() {
            query_args.push(("channel_id".to_string(), channel_id.to_string()));
        }
        if !team_id.is_empty() {
            query_args.push(("team_id".to_string(), team_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/admin.conversations.restrictAccess.listGroups?{}", query_);
        let url = self.client.url(&url, None);
        self.client.get(&url, None, None).await
    }
    /**
     * This function performs a `POST` to the `/admin.conversations.restrictAccess.removeGroup` endpoint.
     *
     * Remove a linked IDP group linked from a private channel
     *
     * FROM: <https://api.slack.com/methods/admin.conversations.restrictAccess.removeGroup>
     */
    pub async fn remove_group(&self) -> Result<crate::types::DndEndSchema> {
        let url = "/admin.conversations.restrictAccess.removeGroup".to_string();
        let url = self.client.url(&url, None);
        self.client
            .post(&url, None, Some("application/x-www-form-urlencoded"))
            .await
    }
}
