use anyhow::Result;

use crate::Client;

pub struct AdminUsersSession {
    pub client: Client,
}

impl AdminUsersSession {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AdminUsersSession { client }
    }

    /**
     * This function performs a `POST` to the `/admin.users.session.invalidate` endpoint.
     *
     * Invalidate a single session for a user by session_id
     *
     * FROM: <https://api.slack.com/methods/admin.users.session.invalidate>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.users:write`.
     */
    pub async fn invalidate(&self, token: &str) -> Result<crate::types::DndEndSchema> {
        let url = "/admin.users.session.invalidate".to_string();
        self.client.post(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/admin.users.session.reset` endpoint.
     *
     * Wipes all valid sessions on all devices for a given user
     *
     * FROM: <https://api.slack.com/methods/admin.users.session.reset>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.users:write`.
     */
    pub async fn reset(&self, token: &str) -> Result<crate::types::DndEndSchema> {
        let url = "/admin.users.session.reset".to_string();
        self.client.post(&url, None).await
    }
}
