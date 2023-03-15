use crate::Client;
use crate::ClientResult;

pub struct AdminUsergroups {
    pub client: Client,
}

impl AdminUsergroups {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AdminUsergroups { client }
    }

    /**
     * This function performs a `POST` to the `/admin.usergroups.addChannels` endpoint.
     *
     * Add one or more default channels to an IDP group.
     *
     * FROM: <https://api.slack.com/methods/admin.usergroups.addChannels>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.usergroups:write`.
     */
    pub async fn add_channels(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.usergroups.addChannels", None);
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
     * This function performs a `POST` to the `/admin.usergroups.addTeams` endpoint.
     *
     * Associate one or more default workspaces with an organization-wide IDP group.
     *
     * FROM: <https://api.slack.com/methods/admin.usergroups.addTeams>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.teams:write`.
     */
    pub async fn add_teams(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.usergroups.addTeams", None);
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
     * This function performs a `GET` to the `/admin.usergroups.listChannels` endpoint.
     *
     * List the channels linked to an org-level IDP group (user group).
     *
     * FROM: <https://api.slack.com/methods/admin.usergroups.listChannels>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.usergroups:read`.
     * * `usergroup_id: &str` -- ID of the IDP group to list default channels for.
     * * `team_id: &str` -- ID of the the workspace.
     * * `include_num_members: bool` -- Flag to include or exclude the count of members per channel.
     */
    pub async fn list_channel(
        &self,
        usergroup_id: &str,
        team_id: &str,
        include_num_members: bool,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if include_num_members {
            query_args.push((
                "include_num_members".to_string(),
                include_num_members.to_string(),
            ));
        }
        if !team_id.is_empty() {
            query_args.push(("team_id".to_string(), team_id.to_string()));
        }
        if !usergroup_id.is_empty() {
            query_args.push(("usergroup_id".to_string(), usergroup_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin.usergroups.listChannels?{}", query_), None);
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
     * This function performs a `POST` to the `/admin.usergroups.removeChannels` endpoint.
     *
     * Remove one or more default channels from an org-level IDP group (user group).
     *
     * FROM: <https://api.slack.com/methods/admin.usergroups.removeChannels>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.usergroups:write`.
     */
    pub async fn remove_channels(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.usergroups.removeChannels", None);
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
