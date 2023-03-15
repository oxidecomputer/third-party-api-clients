use crate::Client;
use crate::ClientResult;

pub struct AppsPermissionsResources {
    pub client: Client,
}

impl AppsPermissionsResources {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AppsPermissionsResources { client }
    }

    /**
     * This function performs a `GET` to the `/apps.permissions.resources.list` endpoint.
     *
     * Returns list of resource grants this app has on a team.
     *
     * FROM: <https://api.slack.com/methods/apps.permissions.resources.list>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `none`.
     * * `cursor: &str` -- Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](/docs/pagination) for more detail.
     * * `limit: i64` -- The maximum number of items to return.
     */
    pub async fn list(
        &self,
        cursor: &str,
        limit: i64,
    ) -> ClientResult<crate::types::AppsPermissionsResourcesListSuccessSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/apps.permissions.resources.list?{}", query_),
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
}
