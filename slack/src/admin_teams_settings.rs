use crate::Client;
use crate::ClientResult;

pub struct AdminTeamsSettings {
    pub client: Client,
}

impl AdminTeamsSettings {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AdminTeamsSettings { client }
    }

    /**
     * This function performs a `GET` to the `/admin.teams.settings.info` endpoint.
     *
     * Fetch information about settings in a workspace
     *
     * FROM: <https://api.slack.com/methods/admin.teams.settings.info>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.teams:read`.
     * * `team_id: &str`
     */
    pub async fn info(&self, team_id: &str) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !team_id.is_empty() {
            query_args.push(("team_id".to_string(), team_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin.teams.settings.info?{}", query_), None);
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
     * This function performs a `POST` to the `/admin.teams.settings.setDefaultChannels` endpoint.
     *
     * Set the default channels of a workspace.
     *
     * FROM: <https://api.slack.com/methods/admin.teams.settings.setDefaultChannels>
     */
    pub async fn set_default_channels(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self
            .client
            .url("/admin.teams.settings.setDefaultChannels", None);
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
     * This function performs a `POST` to the `/admin.teams.settings.setDescription` endpoint.
     *
     * Set the description of a given workspace.
     *
     * FROM: <https://api.slack.com/methods/admin.teams.settings.setDescription>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.teams:write`.
     */
    pub async fn set_description(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self
            .client
            .url("/admin.teams.settings.setDescription", None);
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
     * This function performs a `POST` to the `/admin.teams.settings.setDiscoverability` endpoint.
     *
     * An API method that allows admins to set the discoverability of a given workspace
     *
     * FROM: <https://api.slack.com/methods/admin.teams.settings.setDiscoverability>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.teams:write`.
     */
    pub async fn set_discoverability(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self
            .client
            .url("/admin.teams.settings.setDiscoverability", None);
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
     * This function performs a `POST` to the `/admin.teams.settings.setIcon` endpoint.
     *
     * Sets the icon of a workspace.
     *
     * FROM: <https://api.slack.com/methods/admin.teams.settings.setIcon>
     */
    pub async fn set_icon(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.teams.settings.setIcon", None);
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
     * This function performs a `POST` to the `/admin.teams.settings.setName` endpoint.
     *
     * Set the name of a given workspace.
     *
     * FROM: <https://api.slack.com/methods/admin.teams.settings.setName>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.teams:write`.
     */
    pub async fn set_name(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.teams.settings.setName", None);
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
