use crate::Client;
use crate::ClientResult;

pub struct AdminUsers {
    pub client: Client,
}

impl AdminUsers {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AdminUsers { client }
    }

    /**
     * This function performs a `POST` to the `/admin.users.assign` endpoint.
     *
     * Add an Enterprise user to a workspace.
     *
     * FROM: <https://api.slack.com/methods/admin.users.assign>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.users:write`.
     */
    pub async fn assign(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.users.assign", None);
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
     * This function performs a `POST` to the `/admin.users.invite` endpoint.
     *
     * Invite a user to a workspace.
     *
     * FROM: <https://api.slack.com/methods/admin.users.invite>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.users:write`.
     */
    pub async fn invite(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.users.invite", None);
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
     * This function performs a `GET` to the `/admin.users.list` endpoint.
     *
     * List users on a workspace
     *
     * FROM: <https://api.slack.com/methods/admin.users.list>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.users:read`.
     * * `team_id: &str` -- The ID (`T1234`) of the workspace.
     * * `cursor: &str` -- Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
     * * `limit: i64` -- Limit for how many users to be retrieved per page.
     */
    pub async fn list(
        &self,
        team_id: &str,
        cursor: &str,
        limit: i64,
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
            .url(&format!("/admin.users.list?{}", query_), None);
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
     * This function performs a `POST` to the `/admin.users.remove` endpoint.
     *
     * Remove a user from a workspace.
     *
     * FROM: <https://api.slack.com/methods/admin.users.remove>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.users:write`.
     */
    pub async fn remove(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.users.remove", None);
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
     * This function performs a `POST` to the `/admin.users.setAdmin` endpoint.
     *
     * Set an existing guest, regular user, or owner to be an admin user.
     *
     * FROM: <https://api.slack.com/methods/admin.users.setAdmin>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.users:write`.
     */
    pub async fn set(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.users.setAdmin", None);
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
     * This function performs a `POST` to the `/admin.users.setExpiration` endpoint.
     *
     * Set an expiration for a guest user
     *
     * FROM: <https://api.slack.com/methods/admin.users.setExpiration>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.users:write`.
     */
    pub async fn set_expiration(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.users.setExpiration", None);
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
     * This function performs a `POST` to the `/admin.users.setOwner` endpoint.
     *
     * Set an existing guest, regular user, or admin user to be a workspace owner.
     *
     * FROM: <https://api.slack.com/methods/admin.users.setOwner>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.users:write`.
     */
    pub async fn set_owner(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.users.setOwner", None);
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
     * This function performs a `POST` to the `/admin.users.setRegular` endpoint.
     *
     * Set an existing guest user, admin user, or owner to be a regular user.
     *
     * FROM: <https://api.slack.com/methods/admin.users.setRegular>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.users:write`.
     */
    pub async fn set_regular(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.users.setRegular", None);
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
