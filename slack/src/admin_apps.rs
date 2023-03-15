use crate::Client;
use crate::ClientResult;

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
    pub async fn approve(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.apps.approve", None);
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
    pub async fn restrict(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.apps.restrict", None);
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
