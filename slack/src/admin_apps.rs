use anyhow::Result;

use crate::Client;

pub struct AdminApps {
    pub client: Client,
}

impl AdminApps {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AdminApps { client }
    }

    /**
     * This function performs a `POST` to the `/admin.apps.approve` endpoint.
     *
     * Approve an app for installation on a workspace.
     *
     * FROM: <https://api.slack.com/methods/admin.apps.approve>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.apps:write`.
     */
    pub async fn approve(&self, token: &str) -> Result<crate::types::DndEndSchema> {
        let url = "/admin.apps.approve".to_string();
        self.client.post(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/admin.apps.restrict` endpoint.
     *
     * Restrict an app for installation on a workspace.
     *
     * FROM: <https://api.slack.com/methods/admin.apps.restrict>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.apps:write`.
     */
    pub async fn restrict(&self, token: &str) -> Result<crate::types::DndEndSchema> {
        let url = "/admin.apps.restrict".to_string();
        self.client.post(&url, None).await
    }
}
