use crate::Client;
use crate::ClientResult;

pub struct OauthV2 {
    pub client: Client,
}

impl OauthV2 {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        OauthV2 { client }
    }

    /**
     * This function performs a `GET` to the `/oauth.v2.access` endpoint.
     *
     * Exchanges a temporary OAuth verifier code for an access token.
     *
     * FROM: <https://api.slack.com/methods/oauth.v2.access>
     *
     * **Parameters:**
     *
     * * `client_id: &str` -- Issued when you created your application.
     * * `client_secret: &str` -- Issued when you created your application.
     * * `code: &str` -- The `code` param returned via the OAuth callback.
     * * `redirect_uri: &str` -- This must match the originally submitted URI (if one was sent).
     */
    pub async fn oauth_access(
        &self,
        client_id: &str,
        client_secret: &str,
        code: &str,
        redirect_uri: &str,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !client_id.is_empty() {
            query_args.push(("client_id".to_string(), client_id.to_string()));
        }
        if !client_secret.is_empty() {
            query_args.push(("client_secret".to_string(), client_secret.to_string()));
        }
        if !code.is_empty() {
            query_args.push(("code".to_string(), code.to_string()));
        }
        if !redirect_uri.is_empty() {
            query_args.push(("redirect_uri".to_string(), redirect_uri.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/oauth.v2.access?{}", query_), None);
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
