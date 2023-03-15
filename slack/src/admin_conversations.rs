use crate::Client;
use crate::ClientResult;

pub struct AdminConversations {
    pub client: Client,
}

impl AdminConversations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AdminConversations { client }
    }

    /**
     * This function performs a `POST` to the `/admin.conversations.archive` endpoint.
     *
     * Archive a public or private channel.
     *
     * FROM: <https://api.slack.com/methods/admin.conversations.archive>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.conversations:write`.
     */
    pub async fn archive(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.conversations.archive", None);
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
     * This function performs a `POST` to the `/admin.conversations.convertToPrivate` endpoint.
     *
     * Convert a public channel to a private channel.
     *
     * FROM: <https://api.slack.com/methods/admin.conversations.convertToPrivate>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.conversations:write`.
     */
    pub async fn convert_private(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self
            .client
            .url("/admin.conversations.convertToPrivate", None);
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
     * This function performs a `POST` to the `/admin.conversations.create` endpoint.
     *
     * Create a public or private channel-based conversation.
     *
     * FROM: <https://api.slack.com/methods/admin.conversations.create>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.conversations:write`.
     */
    pub async fn create(&self) -> ClientResult<crate::types::AdminConversationsCreateSchema> {
        let url = self.client.url("/admin.conversations.create", None);
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
     * This function performs a `POST` to the `/admin.conversations.delete` endpoint.
     *
     * Delete a public or private channel.
     *
     * FROM: <https://api.slack.com/methods/admin.conversations.delete>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.conversations:write`.
     */
    pub async fn delete(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.conversations.delete", None);
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
     * This function performs a `POST` to the `/admin.conversations.disconnectShared` endpoint.
     *
     * Disconnect a connected channel from one or more workspaces.
     *
     * FROM: <https://api.slack.com/methods/admin.conversations.disconnectShared>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.conversations:write`.
     */
    pub async fn disconnect_shared(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self
            .client
            .url("/admin.conversations.disconnectShared", None);
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
     * This function performs a `GET` to the `/admin.conversations.getConversationPrefs` endpoint.
     *
     * Get conversation preferences for a public or private channel.
     *
     * FROM: <https://api.slack.com/methods/admin.conversations.getConversationPrefs>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.conversations:read`.
     * * `channel_id: &str` -- The channel to get preferences for.
     */
    pub async fn get_conversation_pref(
        &self,
        channel_id: &str,
    ) -> ClientResult<crate::types::AdminConversationsGetConversationPrefsSchemaData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !channel_id.is_empty() {
            query_args.push(("channel_id".to_string(), channel_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin.conversations.getConversationPrefs?{}", query_),
            None,
        );
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
     * This function performs a `GET` to the `/admin.conversations.getTeams` endpoint.
     *
     * Get all the workspaces a given public or private channel is connected to within this Enterprise org.
     *
     * FROM: <https://api.slack.com/methods/admin.conversations.getTeams>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.conversations:read`.
     * * `channel_id: &str` -- The channel to determine connected workspaces within the organization for.
     * * `cursor: &str` -- Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
     * * `limit: i64` -- The maximum number of items to return. Must be between 1 - 1000 both inclusive.
     */
    pub async fn get_team(
        &self,
        channel_id: &str,
        cursor: &str,
        limit: i64,
    ) -> ClientResult<crate::types::AdminConversationsGetTeamsSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !channel_id.is_empty() {
            query_args.push(("channel_id".to_string(), channel_id.to_string()));
        }
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin.conversations.getTeams?{}", query_), None);
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
     * This function performs a `POST` to the `/admin.conversations.invite` endpoint.
     *
     * Invite a user to a public or private channel.
     *
     * FROM: <https://api.slack.com/methods/admin.conversations.invite>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.conversations:write`.
     */
    pub async fn invite(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.conversations.invite", None);
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
     * This function performs a `POST` to the `/admin.conversations.rename` endpoint.
     *
     * Rename a public or private channel.
     *
     * FROM: <https://api.slack.com/methods/admin.conversations.rename>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.conversations:write`.
     */
    pub async fn rename(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.conversations.rename", None);
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
     * This function performs a `GET` to the `/admin.conversations.search` endpoint.
     *
     * Search for public or private channels in an Enterprise organization.
     *
     * FROM: <https://api.slack.com/methods/admin.conversations.search>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.conversations:read`.
     * * `team_ids: &str` -- Comma separated string of team IDs, signifying the workspaces to search through.
     * * `query: &str` -- Name of the the channel to query by.
     * * `limit: i64` -- Maximum number of items to be returned. Must be between 1 - 20 both inclusive. Default is 10.
     * * `cursor: &str` -- Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
     * * `search_channel_types: &str` -- The type of channel to include or exclude in the search. For example `private` will search private channels, while `private_exclude` will exclude them. For a full list of types, check the [Types section](#types).
     * * `sort: &str` -- Possible values are `relevant` (search ranking based on what we think is closest), `name` (alphabetical), `member_count` (number of users in the channel), and `created` (date channel was created). You can optionally pair this with the `sort_dir` arg to change how it is sorted .
     * * `sort_dir: &str` -- Sort direction. Possible values are `asc` for ascending order like (1, 2, 3) or (a, b, c), and `desc` for descending order like (3, 2, 1) or (c, b, a).
     */
    pub async fn search(
        &self,
        team_ids: &str,
        query: &str,
        limit: i64,
        cursor: &str,
        search_channel_types: &str,
        sort: &str,
        sort_dir: &str,
    ) -> ClientResult<crate::types::AdminConversationsSearchSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        if !search_channel_types.is_empty() {
            query_args.push((
                "search_channel_types".to_string(),
                search_channel_types.to_string(),
            ));
        }
        if !sort.is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !sort_dir.is_empty() {
            query_args.push(("sort_dir".to_string(), sort_dir.to_string()));
        }
        if !team_ids.is_empty() {
            query_args.push(("team_ids".to_string(), team_ids.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin.conversations.search?{}", query_), None);
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
     * This function performs a `POST` to the `/admin.conversations.setConversationPrefs` endpoint.
     *
     * Set the posting permissions for a public or private channel.
     *
     * FROM: <https://api.slack.com/methods/admin.conversations.setConversationPrefs>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.conversations:write`.
     */
    pub async fn set_conversation_prefs(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self
            .client
            .url("/admin.conversations.setConversationPrefs", None);
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
     * This function performs a `POST` to the `/admin.conversations.setTeams` endpoint.
     *
     * Set the workspaces in an Enterprise grid org that connect to a public or private channel.
     *
     * FROM: <https://api.slack.com/methods/admin.conversations.setTeams>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.conversations:write`.
     */
    pub async fn set_teams(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.conversations.setTeams", None);
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
     * This function performs a `POST` to the `/admin.conversations.unarchive` endpoint.
     *
     * Unarchive a public or private channel.
     *
     * FROM: <https://api.slack.com/methods/admin.conversations.unarchive>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.conversations:write`.
     */
    pub async fn unarchive(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.conversations.unarchive", None);
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
