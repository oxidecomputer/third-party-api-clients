use anyhow::Result;

use crate::Client;

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
    pub async fn list(&self) -> Result<crate::types::ApiPermissionsScopesListSuccessSchema> {
        let url = "/apps.permissions.scopes.list".to_string();
        self.client.get(&url, None).await
    }
}
