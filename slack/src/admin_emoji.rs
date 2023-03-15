use crate::Client;
use crate::ClientResult;

pub struct AdminEmoji {
    pub client: Client,
}

impl AdminEmoji {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AdminEmoji { client }
    }

    /**
     * This function performs a `POST` to the `/admin.emoji.add` endpoint.
     *
     * Add an emoji.
     *
     * FROM: <https://api.slack.com/methods/admin.emoji.add>
     */
    pub async fn add(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.emoji.add", None);
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
     * This function performs a `POST` to the `/admin.emoji.addAlias` endpoint.
     *
     * Add an emoji alias.
     *
     * FROM: <https://api.slack.com/methods/admin.emoji.addAlias>
     */
    pub async fn add_alias(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.emoji.addAlias", None);
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
     * This function performs a `GET` to the `/admin.emoji.list` endpoint.
     *
     * List emoji for an Enterprise Grid organization.
     *
     * FROM: <https://api.slack.com/methods/admin.emoji.list>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.teams:read`.
     * * `cursor: &str` -- Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
     * * `limit: i64` -- The maximum number of items to return. Must be between 1 - 1000 both inclusive.
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
            .url(&format!("/admin.emoji.list?{}", query_), None);
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
     * This function performs a `POST` to the `/admin.emoji.remove` endpoint.
     *
     * Remove an emoji across an Enterprise Grid organization
     *
     * FROM: <https://api.slack.com/methods/admin.emoji.remove>
     */
    pub async fn remove(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.emoji.remove", None);
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
     * This function performs a `POST` to the `/admin.emoji.rename` endpoint.
     *
     * Rename an emoji.
     *
     * FROM: <https://api.slack.com/methods/admin.emoji.rename>
     */
    pub async fn rename(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.emoji.rename", None);
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
