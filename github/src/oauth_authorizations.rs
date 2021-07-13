use anyhow::Result;

use crate::Client;

pub struct OauthAuthorizations {
    client: Client,
}

impl OauthAuthorizations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        OauthAuthorizations { client }
    }

    /**
     * Update an existing authorization.
     *
     * This function performs a `PATCH` to the `/authorizations/{authorization_id}` endpoint.
     *
     * **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations/), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/developers/apps/authorizing-oauth-apps#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
     *
     * If you have two-factor authentication setup, Basic Authentication for this endpoint requires that you use a one-time password (OTP) and your username and password instead of tokens. For more information, see "[Working with two-factor authentication](https://docs.github.com/rest/overview/other-authentication-methods#working-with-two-factor-authentication)."
     *
     * You can only send one of these scope keys at a time.
     *
     * FROM: <https://docs.github.com/rest/reference/oauth-authorizations#update-an-existing-authorization>
     *
     * **Parameters:**
     *
     * * `authorization_id: i64` -- authorization_id parameter.
     */
    pub async fn update_authorization(
        &self,
        authorization_id: i64,
        body: &crate::types::OauthAuthorizationsUpdateAuthorizationRequest,
    ) -> Result<crate::types::Authorization> {
        let url = format!(
            "/authorizations/{}",
            crate::progenitor_support::encode_path(&authorization_id.to_string()),
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
