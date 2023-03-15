use crate::Client;
use crate::ClientResult;

pub struct AppsPermissionsScopes {
    pub client: Client,
}

impl AppsPermissionsScopes {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AppsPermissionsScopes { client }
    }

    /**
     * This function performs a `GET` to the `/apps.permissions.scopes.list` endpoint.
     *
     * Returns list of scopes this app has on a team.
     *
     * FROM: <https://api.slack.com/methods/apps.permissions.scopes.list>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `none`.
     */
    pub async fn list(&self) -> ClientResult<crate::types::ApiPermissionsScopesListSuccessSchema> {
        let url = self.client.url("/apps.permissions.scopes.list", None);
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
}
