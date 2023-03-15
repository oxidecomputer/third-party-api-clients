use crate::Client;
use crate::ClientResult;

pub struct Oauth {
    pub client: Client,
}

impl Oauth {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Oauth { client }
    }

    /**
     * This function performs a `GET` to the `/oauth.access` endpoint.
     *
     * Exchanges a temporary OAuth verifier code for an access token.
     *
     * FROM: <https://api.slack.com/methods/oauth.access>
     *
     * **Parameters:**
     *
     * * `client_id: &str` -- Issued when you created your application.
     * * `client_secret: &str` -- Issued when you created your application.
     * * `code: &str` -- The `code` param returned via the OAuth callback.
     * * `redirect_uri: &str` -- This must match the originally submitted URI (if one was sent).
     * * `single_channel: bool` -- Request the user to add your app only to a single channel. Only valid with a [legacy workspace app](https://api.slack.com/legacy-workspace-apps).
     */
    pub async fn access(
        &self,
        client_id: &str,
        client_secret: &str,
        code: &str,
        redirect_uri: &str,
        single_channel: bool,
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
        if single_channel {
            query_args.push(("single_channel".to_string(), single_channel.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/oauth.access?{}", query_), None);
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
     * This function performs a `GET` to the `/oauth.token` endpoint.
     *
     * Exchanges a temporary OAuth verifier code for a workspace token.
     *
     * FROM: <https://api.slack.com/methods/oauth.token>
     *
     * **Parameters:**
     *
     * * `client_id: &str` -- Issued when you created your application.
     * * `client_secret: &str` -- Issued when you created your application.
     * * `code: &str` -- The `code` param returned via the OAuth callback.
     * * `redirect_uri: &str` -- This must match the originally submitted URI (if one was sent).
     * * `single_channel: bool` -- Request the user to add your app only to a single channel.
     */
    pub async fn token(
        &self,
        client_id: &str,
        client_secret: &str,
        code: &str,
        redirect_uri: &str,
        single_channel: bool,
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
        if single_channel {
            query_args.push(("single_channel".to_string(), single_channel.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/oauth.token?{}", query_), None);
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
