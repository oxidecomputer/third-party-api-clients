use crate::Client;
use crate::ClientResult;

pub struct Apps {
    pub client: Client,
}

impl Apps {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Apps { client }
    }

    /**
     * This function performs a `GET` to the `/apps.uninstall` endpoint.
     *
     * Uninstalls your app from a workspace.
     *
     * FROM: <https://api.slack.com/methods/apps.uninstall>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `none`.
     * * `client_id: &str` -- Issued when you created your application.
     * * `client_secret: &str` -- Issued when you created your application.
     */
    pub async fn uninstall(
        &self,
        client_id: &str,
        client_secret: &str,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !client_id.is_empty() {
            query_args.push(("client_id".to_string(), client_id.to_string()));
        }
        if !client_secret.is_empty() {
            query_args.push(("client_secret".to_string(), client_secret.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/apps.uninstall?{}", query_), None);
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
