use crate::Client;
use crate::ClientResult;

pub struct AppsPermissionsUsers {
    pub client: Client,
}

impl AppsPermissionsUsers {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AppsPermissionsUsers { client }
    }

    /**
     * This function performs a `GET` to the `/apps.permissions.users.list` endpoint.
     *
     * Returns list of user grants and corresponding scopes this app has on a team.
     *
     * FROM: <https://api.slack.com/methods/apps.permissions.users.list>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `none`.
     * * `cursor: &str` -- Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](/docs/pagination) for more detail.
     * * `limit: i64` -- The maximum number of items to return.
     */
    pub async fn list(&self, cursor: &str, limit: i64) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/apps.permissions.users.list?{}", query_), None);
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
     * This function performs a `GET` to the `/apps.permissions.users.request` endpoint.
     *
     * Enables an app to trigger a permissions modal to grant an app access to a user access scope.
     *
     * FROM: <https://api.slack.com/methods/apps.permissions.users.request>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `none`.
     * * `scopes: &str` -- A comma separated list of user scopes to request for.
     * * `trigger_id: &str` -- Token used to trigger the request.
     * * `user: &str` -- The user this scope is being requested for.
     */
    pub async fn request(
        &self,
        scopes: &str,
        trigger_id: &str,
        user: &str,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !scopes.is_empty() {
            query_args.push(("scopes".to_string(), scopes.to_string()));
        }
        if !trigger_id.is_empty() {
            query_args.push(("trigger_id".to_string(), trigger_id.to_string()));
        }
        if !user.is_empty() {
            query_args.push(("user".to_string(), user.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/apps.permissions.users.request?{}", query_), None);
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
