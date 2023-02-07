use anyhow::Result;

use crate::Client;

pub struct AppsPermissions {
    pub client: Client,
}

impl AppsPermissions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AppsPermissions { client }
    }

    /**
    * This function performs a `GET` to the `/apps.permissions.info` endpoint.
    *
    * Returns list of permissions this app has on a team.
    *
    * FROM: <https://api.slack.com/methods/apps.permissions.info>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `none`.
    */
    pub async fn info(&self) -> Result<crate::types::AppsPermissionsInfoSchema> {
        let url = "/apps.permissions.info".to_string();
        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/apps.permissions.request` endpoint.
    *
    * Allows an app to request additional scopes
    *
    * FROM: <https://api.slack.com/methods/apps.permissions.request>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `none`.
    * * `scopes: &str` -- A comma separated list of scopes to request for.
    * * `trigger_id: &str` -- Token used to trigger the permissions API.
    */
    pub async fn request(
        &self,
        scopes: &str,
        trigger_id: &str,
    ) -> Result<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !scopes.is_empty() {
            query_args.push(("scopes".to_string(), scopes.to_string()));
        }
        if !trigger_id.is_empty() {
            query_args.push(("trigger_id".to_string(), trigger_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/apps.permissions.request?{}", query_);

        self.client.get(&url, None).await
    }
}
