use crate::Client;
use crate::ClientResult;

pub struct OauthAuthorizations {
    pub client: Client,
}

impl OauthAuthorizations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        OauthAuthorizations { client }
    }

    /**
     * List your grants.
     *
     * This function performs a `GET` to the `/applications/grants` endpoint.
     *
     * **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations/), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/developers/apps/authorizing-oauth-apps#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
     *
     * You can use this API to list the set of OAuth applications that have been granted access to your account. Unlike the [list your authorizations](https://docs.github.com/rest/reference/oauth-authorizations#list-your-authorizations) API, this API does not manage individual tokens. This API will return one entry for each OAuth application that has been granted access to your account, regardless of the number of tokens an application has generated for your user. The list of OAuth applications returned matches what is shown on [the application authorizations settings screen within GitHub](https://github.com/settings/applications#authorized). The `scopes` returned are the union of scopes authorized for the application. For example, if an application has one token with `repo` scope and another token with `user` scope, the grant will return `["repo", "user"]`.
     *
     * FROM: <https://docs.github.com/rest/reference/oauth-authorizations#list-your-grants>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     * * `client_id: &str` -- The client ID of your GitHub app.
     */
    pub async fn list_grants(
        &self,
        per_page: i64,
        page: i64,
        client_id: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ApplicationGrant>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !client_id.is_empty() {
            query_args.push(("client_id".to_string(), client_id.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/applications/grants?{}", query_), None);
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
     * List your grants.
     *
     * This function performs a `GET` to the `/applications/grants` endpoint.
     *
     * As opposed to `list_grants`, this function returns all the pages of the request at once.
     *
     * **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations/), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/developers/apps/authorizing-oauth-apps#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
     *
     * You can use this API to list the set of OAuth applications that have been granted access to your account. Unlike the [list your authorizations](https://docs.github.com/rest/reference/oauth-authorizations#list-your-authorizations) API, this API does not manage individual tokens. This API will return one entry for each OAuth application that has been granted access to your account, regardless of the number of tokens an application has generated for your user. The list of OAuth applications returned matches what is shown on [the application authorizations settings screen within GitHub](https://github.com/settings/applications#authorized). The `scopes` returned are the union of scopes authorized for the application. For example, if an application has one token with `repo` scope and another token with `user` scope, the grant will return `["repo", "user"]`.
     *
     * FROM: <https://docs.github.com/rest/reference/oauth-authorizations#list-your-grants>
     */
    pub async fn list_all_grants(
        &self,
        client_id: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ApplicationGrant>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !client_id.is_empty() {
            query_args.push(("client_id".to_string(), client_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/applications/grants?{}", query_), None);
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get a single grant.
     *
     * This function performs a `GET` to the `/applications/grants/{grant_id}` endpoint.
     *
     * **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/apps/building-oauth-apps/authorizing-oauth-apps/#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
     *
     * FROM: <https://docs.github.com/rest/reference/oauth-authorizations#get-a-single-grant>
     *
     * **Parameters:**
     *
     * * `grant_id: i64` -- grant_id parameter.
     */
    pub async fn get_grant(
        &self,
        grant_id: i64,
    ) -> ClientResult<crate::Response<crate::types::ApplicationGrant>> {
        let url = self.client.url(
            &format!(
                "/applications/grants/{}",
                crate::progenitor_support::encode_path(&grant_id.to_string()),
            ),
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
    /**
     * Delete a grant.
     *
     * This function performs a `DELETE` to the `/applications/grants/{grant_id}` endpoint.
     *
     * **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations/), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/developers/apps/authorizing-oauth-apps#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations/) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
     *
     * Deleting an OAuth application's grant will also delete all OAuth tokens associated with the application for your user. Once deleted, the application has no access to your account and is no longer listed on [the application authorizations settings screen within GitHub](https://github.com/settings/applications#authorized).
     *
     * FROM: <https://docs.github.com/rest/reference/oauth-authorizations#delete-a-grant>
     *
     * **Parameters:**
     *
     * * `grant_id: i64` -- grant_id parameter.
     */
    pub async fn delete_grant(&self, grant_id: i64) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/applications/grants/{}",
                crate::progenitor_support::encode_path(&grant_id.to_string()),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * List your authorizations.
     *
     * This function performs a `GET` to the `/authorizations` endpoint.
     *
     * **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/apps/building-oauth-apps/authorizing-oauth-apps/#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
     *
     * FROM: <https://docs.github.com/rest/reference/oauth-authorizations#list-your-authorizations>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     * * `client_id: &str` -- The client ID of your GitHub app.
     */
    pub async fn list_authorizations(
        &self,
        per_page: i64,
        page: i64,
        client_id: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Authorization>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !client_id.is_empty() {
            query_args.push(("client_id".to_string(), client_id.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/authorizations?{}", query_), None);
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
     * List your authorizations.
     *
     * This function performs a `GET` to the `/authorizations` endpoint.
     *
     * As opposed to `list_authorizations`, this function returns all the pages of the request at once.
     *
     * **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/apps/building-oauth-apps/authorizing-oauth-apps/#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
     *
     * FROM: <https://docs.github.com/rest/reference/oauth-authorizations#list-your-authorizations>
     */
    pub async fn list_all_authorizations(
        &self,
        client_id: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Authorization>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !client_id.is_empty() {
            query_args.push(("client_id".to_string(), client_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/authorizations?{}", query_), None);
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Create a new authorization.
     *
     * This function performs a `POST` to the `/authorizations` endpoint.
     *
     * **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/developers/apps/authorizing-oauth-apps#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
     *
     * **Warning:** Apps must use the [web application flow](https://docs.github.com/apps/building-oauth-apps/authorizing-oauth-apps/#web-application-flow) to obtain OAuth tokens that work with GitHub SAML organizations. OAuth tokens created using the Authorizations API will be unable to access GitHub SAML organizations. For more information, see the [blog post](https://developer.github.com/changes/2019-11-05-deprecated-passwords-and-authorizations-api).
     *
     * Creates OAuth tokens using [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication). If you have two-factor authentication setup, Basic Authentication for this endpoint requires that you use a one-time password (OTP) and your username and password instead of tokens. For more information, see "[Working with two-factor authentication](https://docs.github.com/rest/overview/other-authentication-methods#working-with-two-factor-authentication)."
     *
     * To create tokens for a particular OAuth application using this endpoint, you must authenticate as the user you want to create an authorization for and provide the app's client ID and secret, found on your OAuth application's settings page. If your OAuth application intends to create multiple tokens for one user, use `fingerprint` to differentiate between them.
     *
     * You can also create tokens on GitHub from the [personal access tokens settings](https://github.com/settings/tokens) page. Read more about these tokens in [the GitHub Help documentation](https://help.github.com/articles/creating-an-access-token-for-command-line-use).
     *
     * Organizations that enforce SAML SSO require personal access tokens to be allowed. Read more about allowing tokens in [the GitHub Help documentation](https://help.github.com/articles/about-identity-and-access-management-with-saml-single-sign-on).
     *
     * FROM: <https://docs.github.com/rest/reference/oauth-authorizations#create-a-new-authorization>
     */
    pub async fn create_authorization(
        &self,
        body: &crate::types::OauthAuthorizationsCreateAuthorizationRequest,
    ) -> ClientResult<crate::Response<crate::types::Authorization>> {
        let url = self.client.url("/authorizations", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Get-or-create an authorization for a specific app.
     *
     * This function performs a `PUT` to the `/authorizations/clients/{client_id}` endpoint.
     *
     * **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations/), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/developers/apps/authorizing-oauth-apps#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
     *
     * **Warning:** Apps must use the [web application flow](https://docs.github.com/apps/building-oauth-apps/authorizing-oauth-apps/#web-application-flow) to obtain OAuth tokens that work with GitHub SAML organizations. OAuth tokens created using the Authorizations API will be unable to access GitHub SAML organizations. For more information, see the [blog post](https://developer.github.com/changes/2019-11-05-deprecated-passwords-and-authorizations-api).
     *
     * Creates a new authorization for the specified OAuth application, only if an authorization for that application doesn't already exist for the user. The URL includes the 20 character client ID for the OAuth app that is requesting the token. It returns the user's existing authorization for the application if one is present. Otherwise, it creates and returns a new one.
     *
     * If you have two-factor authentication setup, Basic Authentication for this endpoint requires that you use a one-time password (OTP) and your username and password instead of tokens. For more information, see "[Working with two-factor authentication](https://docs.github.com/rest/overview/other-authentication-methods#working-with-two-factor-authentication)."
     *
     * **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations/), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/developers/apps/authorizing-oauth-apps#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
     *
     * FROM: <https://docs.github.com/rest/reference/oauth-authorizations#get-or-create-an-authorization-for-a-specific-app>
     *
     * **Parameters:**
     *
     * * `client_id: &str` -- The client ID of your GitHub app.
     */
    pub async fn get_or_create_authorization_for_app(
        &self,
        client_id: &str,
        body: &crate::types::OauthAuthorizationsGetCreateAuthorizationAppRequest,
    ) -> ClientResult<crate::Response<crate::types::Authorization>> {
        let url = self.client.url(
            &format!(
                "/authorizations/clients/{}",
                crate::progenitor_support::encode_path(client_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Get-or-create an authorization for a specific app and fingerprint.
     *
     * This function performs a `PUT` to the `/authorizations/clients/{client_id}/{fingerprint}` endpoint.
     *
     * **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations/), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/developers/apps/authorizing-oauth-apps#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
     *
     * **Warning:** Apps must use the [web application flow](https://docs.github.com/apps/building-oauth-apps/authorizing-oauth-apps/#web-application-flow) to obtain OAuth tokens that work with GitHub SAML organizations. OAuth tokens created using the Authorizations API will be unable to access GitHub SAML organizations. For more information, see the [blog post](https://developer.github.com/changes/2019-11-05-deprecated-passwords-and-authorizations-api).
     *
     * This method will create a new authorization for the specified OAuth application, only if an authorization for that application and fingerprint do not already exist for the user. The URL includes the 20 character client ID for the OAuth app that is requesting the token. `fingerprint` is a unique string to distinguish an authorization from others created for the same client ID and user. It returns the user's existing authorization for the application if one is present. Otherwise, it creates and returns a new one.
     *
     * If you have two-factor authentication setup, Basic Authentication for this endpoint requires that you use a one-time password (OTP) and your username and password instead of tokens. For more information, see "[Working with two-factor authentication](https://docs.github.com/rest/overview/other-authentication-methods#working-with-two-factor-authentication)."
     *
     * FROM: <https://docs.github.com/rest/reference/oauth-authorizations#get-or-create-an-authorization-for-a-specific-app-and-fingerprint>
     *
     * **Parameters:**
     *
     * * `client_id: &str` -- The client ID of your GitHub app.
     * * `fingerprint: &str`
     */
    pub async fn get_or_create_authorization_for_app_and_fingerprint(
        &self,
        client_id: &str,
        fingerprint: &str,
        body: &crate::types::OauthAuthorizationsGetCreateAuthorizationAppFingerprintRequest,
    ) -> ClientResult<crate::Response<crate::types::Authorization>> {
        let url = self.client.url(
            &format!(
                "/authorizations/clients/{}/{}",
                crate::progenitor_support::encode_path(client_id),
                crate::progenitor_support::encode_path(fingerprint),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Get a single authorization.
     *
     * This function performs a `GET` to the `/authorizations/{authorization_id}` endpoint.
     *
     * **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/apps/building-oauth-apps/authorizing-oauth-apps/#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
     *
     * FROM: <https://docs.github.com/rest/reference/oauth-authorizations#get-a-single-authorization>
     *
     * **Parameters:**
     *
     * * `authorization_id: i64` -- authorization_id parameter.
     */
    pub async fn get_authorization(
        &self,
        authorization_id: i64,
    ) -> ClientResult<crate::Response<crate::types::Authorization>> {
        let url = self.client.url(
            &format!(
                "/authorizations/{}",
                crate::progenitor_support::encode_path(&authorization_id.to_string()),
            ),
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
    /**
     * Delete an authorization.
     *
     * This function performs a `DELETE` to the `/authorizations/{authorization_id}` endpoint.
     *
     * **Deprecation Notice:** GitHub will discontinue the [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations), which is used by integrations to create personal access tokens and OAuth tokens, and you must now create these tokens using our [web application flow](https://docs.github.com/apps/building-oauth-apps/authorizing-oauth-apps/#web-application-flow). The [OAuth Authorizations API](https://docs.github.com/rest/reference/oauth-authorizations) will be removed on November, 13, 2020. For more information, including scheduled brownouts, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-auth-endpoint/).
     *
     * FROM: <https://docs.github.com/rest/reference/oauth-authorizations#delete-an-authorization>
     *
     * **Parameters:**
     *
     * * `authorization_id: i64` -- authorization_id parameter.
     */
    pub async fn delete_authorization(
        &self,
        authorization_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/authorizations/{}",
                crate::progenitor_support::encode_path(&authorization_id.to_string()),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
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
    ) -> ClientResult<crate::Response<crate::types::Authorization>> {
        let url = self.client.url(
            &format!(
                "/authorizations/{}",
                crate::progenitor_support::encode_path(&authorization_id.to_string()),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
}
