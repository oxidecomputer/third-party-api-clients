use anyhow::Result;

use crate::Client;

pub struct Auth {
    pub client: Client,
}

impl Auth {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Auth { client }
    }

    /**
     * This function performs a `GET` to the `/auth.revoke` endpoint.
     *
     * Revokes a token.
     *
     * FROM: <https://api.slack.com/methods/auth.revoke>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `none`.
     * * `test: bool` -- Setting this parameter to `1` triggers a _testing mode_ where the specified token will not actually be revoked.
     */
    pub async fn revoke(&self, test: bool) -> Result<crate::types::AuthRevokeSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if test {
            query_args.push(("test".to_string(), test.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/auth.revoke?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/auth.test` endpoint.
     *
     * Checks authentication & identity.
     *
     * FROM: <https://api.slack.com/methods/auth.test>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `none`.
     */
    pub async fn test(&self) -> Result<crate::types::AuthTestSuccessSchema> {
        let url = "/auth.test".to_string();
        self.client.get(&url, None).await
    }
}
