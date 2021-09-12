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
    pub async fn list(
        &self,
        token: &str,
    ) -> Result<crate::types::ApiPermissionsScopesListSuccessSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !token.is_empty() {
            query_args.push(("token".to_string(), token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/apps.permissions.scopes.list?{}", query_);

        self.client.get(&url, None).await
    }
}
