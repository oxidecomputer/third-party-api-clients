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
     * Get the authenticated app.
     *
     * This function performs a `GET` to the `/app` endpoint.
     *
     * Returns the GitHub App associated with the authentication credentials used. To see how many app installations are associated with this GitHub App, see the `installations_count` in the response. For more details about your app's installations, see the "[List installations for the authenticated app](https://docs.github.com/rest/reference/apps#list-installations-for-the-authenticated-app)" endpoint.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#get-the-authenticated-app>
     */
    pub async fn get_authenticated(
        &self,
    ) -> ClientResult<crate::Response<crate::types::GitHubApp>> {
        let url = self.client.url("/app", None);
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
     * Create a GitHub App from a manifest.
     *
     * This function performs a `POST` to the `/app-manifests/{code}/conversions` endpoint.
     *
     * Use this endpoint to complete the handshake necessary when implementing the [GitHub App Manifest flow](https://docs.github.com/apps/building-github-apps/creating-github-apps-from-a-manifest/). When you create a GitHub App with the manifest flow, you receive a temporary `code` used to retrieve the GitHub App's `id`, `pem` (private key), and `webhook_secret`.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#create-a-github-app-from-a-manifest>
     *
     * **Parameters:**
     *
     * * `code: &str`
     */
    pub async fn create_from_manifest(
        &self,
        code: &str,
    ) -> ClientResult<crate::Response<crate::types::AppsCreateFromManifestResponseAllOf>> {
        let url = self.client.url(
            &format!(
                "/app-manifests/{}/conversions",
                crate::progenitor_support::encode_path(code),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Get a webhook configuration for an app.
     *
     * This function performs a `GET` to the `/app/hook/config` endpoint.
     *
     * Returns the webhook configuration for a GitHub App. For more information about configuring a webhook for your app, see "[Creating a GitHub App](/developers/apps/creating-a-github-app)."
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#get-a-webhook-configuration-for-an-app>
     */
    pub async fn get_webhook_config_for_app(
        &self,
    ) -> ClientResult<crate::Response<crate::types::WebhookConfig>> {
        let url = self.client.url("/app/hook/config", None);
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
     * Update a webhook configuration for an app.
     *
     * This function performs a `PATCH` to the `/app/hook/config` endpoint.
     *
     * Updates the webhook configuration for a GitHub App. For more information about configuring a webhook for your app, see "[Creating a GitHub App](/developers/apps/creating-a-github-app)."
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#update-a-webhook-configuration-for-an-app>
     */
    pub async fn update_webhook_config_for_app(
        &self,
        body: &crate::types::AppsUpdateWebhookConfigAppRequest,
    ) -> ClientResult<crate::Response<crate::types::WebhookConfig>> {
        let url = self.client.url("/app/hook/config", None);
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
    /**
     * List deliveries for an app webhook.
     *
     * This function performs a `GET` to the `/app/hook/deliveries` endpoint.
     *
     * Returns a list of webhook deliveries for the webhook configured for a GitHub App.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#list-deliveries-for-an-app-webhook>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- Results per page (max 100).
     * * `cursor: &str` -- Used for pagination: the starting delivery from which the page of deliveries is fetched. Refer to the `link` header for the next and previous page cursors.
     */
    pub async fn list_webhook_deliveries(
        &self,
        per_page: i64,
        cursor: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::HookDeliveryItem>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/app/hook/deliveries?{}", query_), None);
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
     * List deliveries for an app webhook.
     *
     * This function performs a `GET` to the `/app/hook/deliveries` endpoint.
     *
     * As opposed to `list_webhook_deliveries`, this function returns all the pages of the request at once.
     *
     * Returns a list of webhook deliveries for the webhook configured for a GitHub App.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#list-deliveries-for-an-app-webhook>
     */
    pub async fn list_all_webhook_deliveries(
        &self,
        cursor: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::HookDeliveryItem>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/app/hook/deliveries?{}", query_), None);
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
     * Get a delivery for an app webhook.
     *
     * This function performs a `GET` to the `/app/hook/deliveries/{delivery_id}` endpoint.
     *
     * Returns a delivery for the webhook configured for a GitHub App.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#get-a-delivery-for-an-app-webhook>
     *
     * **Parameters:**
     *
     * * `delivery_id: i64`
     */
    pub async fn get_webhook_delivery(
        &self,
        delivery_id: i64,
    ) -> ClientResult<crate::Response<crate::types::HookDelivery>> {
        let url = self.client.url(
            &format!(
                "/app/hook/deliveries/{}",
                crate::progenitor_support::encode_path(&delivery_id.to_string()),
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
     * Redeliver a delivery for an app webhook.
     *
     * This function performs a `POST` to the `/app/hook/deliveries/{delivery_id}/attempts` endpoint.
     *
     * Redeliver a delivery for the webhook configured for a GitHub App.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#redeliver-a-delivery-for-an-app-webhook>
     *
     * **Parameters:**
     *
     * * `delivery_id: i64`
     */
    pub async fn redeliver_webhook_delivery(
        &self,
        delivery_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/app/hook/deliveries/{}/attempts",
                crate::progenitor_support::encode_path(&delivery_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * List installations for the authenticated app.
     *
     * This function performs a `GET` to the `/app/installations` endpoint.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * The permissions the installation has are included under the `permissions` key.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#list-installations-for-the-authenticated-app>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `outdated: &str`
     */
    pub async fn list_installations(
        &self,
        per_page: i64,
        page: i64,
        since: Option<chrono::DateTime<chrono::Utc>>,
        outdated: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Installation>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !outdated.is_empty() {
            query_args.push(("outdated".to_string(), outdated.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/app/installations?{}", query_), None);
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
     * List installations for the authenticated app.
     *
     * This function performs a `GET` to the `/app/installations` endpoint.
     *
     * As opposed to `list_installations`, this function returns all the pages of the request at once.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * The permissions the installation has are included under the `permissions` key.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#list-installations-for-the-authenticated-app>
     */
    pub async fn list_all_installations(
        &self,
        since: Option<chrono::DateTime<chrono::Utc>>,
        outdated: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Installation>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !outdated.is_empty() {
            query_args.push(("outdated".to_string(), outdated.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/app/installations?{}", query_), None);
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
     * Get an installation for the authenticated app.
     *
     * This function performs a `GET` to the `/app/installations/{installation_id}` endpoint.
     *
     * Enables an authenticated GitHub App to find an installation's information using the installation id. The installation's account type (`target_type`) will be either an organization or a user account, depending which account the repository belongs to.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#get-an-installation-for-the-authenticated-app>
     *
     * **Parameters:**
     *
     * * `installation_id: i64` -- installation_id parameter.
     */
    pub async fn get_installation(
        &self,
        installation_id: i64,
    ) -> ClientResult<crate::Response<crate::types::Installation>> {
        let url = self.client.url(
            &format!(
                "/app/installations/{}",
                crate::progenitor_support::encode_path(&installation_id.to_string()),
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
     * Delete an installation for the authenticated app.
     *
     * This function performs a `DELETE` to the `/app/installations/{installation_id}` endpoint.
     *
     * Uninstalls a GitHub App on a user, organization, or business account. If you prefer to temporarily suspend an app's access to your account's resources, then we recommend the "[Suspend an app installation](https://docs.github.com/rest/reference/apps/#suspend-an-app-installation)" endpoint.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#delete-an-installation-for-the-authenticated-app>
     *
     * **Parameters:**
     *
     * * `installation_id: i64` -- installation_id parameter.
     */
    pub async fn delete_installation(
        &self,
        installation_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/app/installations/{}",
                crate::progenitor_support::encode_path(&installation_id.to_string()),
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
     * Create an installation access token for an app.
     *
     * This function performs a `POST` to the `/app/installations/{installation_id}/access_tokens` endpoint.
     *
     * Creates an installation access token that enables a GitHub App to make authenticated API requests for the app's installation on an organization or individual account. Installation tokens expire one hour from the time you create them. Using an expired token produces a status code of `401 - Unauthorized`, and requires creating a new installation token. By default the installation token has access to all repositories that the installation can access. To restrict the access to specific repositories, you can provide the `repository_ids` when creating the token. When you omit `repository_ids`, the response does not contain the `repositories` key.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps/#create-an-installation-access-token-for-an-app>
     *
     * **Parameters:**
     *
     * * `installation_id: i64` -- installation_id parameter.
     */
    #[async_recursion::async_recursion]
    pub async fn create_installation_access_token(
        &self,
        installation_id: i64,
        body: &crate::types::AppsCreateInstallationAccessTokenRequest,
    ) -> ClientResult<crate::Response<crate::types::InstallationToken>> {
        let url = self.client.url(
            &format!(
                "/app/installations/{}/access_tokens",
                crate::progenitor_support::encode_path(&installation_id.to_string()),
            ),
            None,
        );
        self.client
            .post_media(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
                crate::utils::MediaType::Json,
                crate::auth::AuthenticationConstraint::JWT,
            )
            .await
    }
    /**
     * Suspend an app installation.
     *
     * This function performs a `PUT` to the `/app/installations/{installation_id}/suspended` endpoint.
     *
     * Suspends a GitHub App on a user, organization, or business account, which blocks the app from accessing the account's resources. When a GitHub App is suspended, the app's access to the GitHub API or webhook events is blocked for that account.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#suspend-an-app-installation>
     *
     * **Parameters:**
     *
     * * `installation_id: i64` -- installation_id parameter.
     */
    pub async fn suspend_installation(
        &self,
        installation_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/app/installations/{}/suspended",
                crate::progenitor_support::encode_path(&installation_id.to_string()),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Unsuspend an app installation.
     *
     * This function performs a `DELETE` to the `/app/installations/{installation_id}/suspended` endpoint.
     *
     * Removes a GitHub App installation suspension.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#unsuspend-an-app-installation>
     *
     * **Parameters:**
     *
     * * `installation_id: i64` -- installation_id parameter.
     */
    pub async fn unsuspend_installation(
        &self,
        installation_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/app/installations/{}/suspended",
                crate::progenitor_support::encode_path(&installation_id.to_string()),
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
     * Delete an app authorization.
     *
     * This function performs a `DELETE` to the `/applications/{client_id}/grant` endpoint.
     *
     * OAuth application owners can revoke a grant for their OAuth application and a specific user. You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) when accessing this endpoint, using the OAuth application's `client_id` and `client_secret` as the username and password. You must also provide a valid OAuth `access_token` as an input parameter and the grant for the token's owner will be deleted.
     * Deleting an OAuth application's grant will also delete all OAuth tokens associated with the application for the user. Once deleted, the application will have no access to the user's account and will no longer be listed on [the application authorizations settings screen within GitHub](https://github.com/settings/applications#authorized).
     *
     * FROM: <https://docs.github.com/rest/reference/apps#delete-an-app-authorization>
     *
     * **Parameters:**
     *
     * * `client_id: &str` -- The client ID of your GitHub app.
     */
    pub async fn delete_authorization(
        &self,
        client_id: &str,
        body: &crate::types::AppsCheckTokenRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/applications/{}/grant",
                crate::progenitor_support::encode_path(client_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Revoke a grant for an application.
     *
     * This function performs a `DELETE` to the `/applications/{client_id}/grants/{access_token}` endpoint.
     *
     * **Deprecation Notice:** GitHub will discontinue OAuth endpoints that contain `access_token` in the path parameter. We have introduced new endpoints that allow you to securely manage tokens for OAuth Apps by moving `access_token` to the request body. For more information, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-app-endpoint/).
     *
     * OAuth application owners can revoke a grant for their OAuth application and a specific user. You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) when accessing this endpoint, using the OAuth application's `client_id` and `client_secret` as the username and password. You must also provide a valid token as `:access_token` and the grant for the token's owner will be deleted.
     *
     * Deleting an OAuth application's grant will also delete all OAuth tokens associated with the application for the user. Once deleted, the application will have no access to the user's account and will no longer be listed on [the Applications settings page under "Authorized OAuth Apps" on GitHub](https://github.com/settings/applications#authorized).
     *
     * FROM: <https://docs.github.com/rest/reference/apps#revoke-a-grant-for-an-application>
     *
     * **Parameters:**
     *
     * * `client_id: &str` -- The client ID of your GitHub app.
     * * `access_token: &str`
     */
    pub async fn revoke_grant_for_application(
        &self,
        client_id: &str,
        access_token: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/applications/{}/grants/{}",
                crate::progenitor_support::encode_path(client_id),
                crate::progenitor_support::encode_path(access_token),
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
     * Check a token.
     *
     * This function performs a `POST` to the `/applications/{client_id}/token` endpoint.
     *
     * OAuth applications can use a special API method for checking OAuth token validity without exceeding the normal rate limits for failed login attempts. Authentication works differently with this particular endpoint. You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) to use this endpoint, where the username is the OAuth application `client_id` and the password is its `client_secret`. Invalid tokens will return `404 NOT FOUND`.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#check-a-token>
     *
     * **Parameters:**
     *
     * * `client_id: &str` -- The client ID of your GitHub app.
     */
    pub async fn check_token(
        &self,
        client_id: &str,
        body: &crate::types::AppsCheckTokenRequest,
    ) -> ClientResult<crate::Response<crate::types::Authorization>> {
        let url = self.client.url(
            &format!(
                "/applications/{}/token",
                crate::progenitor_support::encode_path(client_id),
            ),
            None,
        );
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
     * Delete an app token.
     *
     * This function performs a `DELETE` to the `/applications/{client_id}/token` endpoint.
     *
     * OAuth application owners can revoke a single token for an OAuth application. You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) when accessing this endpoint, using the OAuth application's `client_id` and `client_secret` as the username and password.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#delete-an-app-token>
     *
     * **Parameters:**
     *
     * * `client_id: &str` -- The client ID of your GitHub app.
     */
    pub async fn delete_token(
        &self,
        client_id: &str,
        body: &crate::types::AppsCheckTokenRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/applications/{}/token",
                crate::progenitor_support::encode_path(client_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Reset a token.
     *
     * This function performs a `PATCH` to the `/applications/{client_id}/token` endpoint.
     *
     * OAuth applications can use this API method to reset a valid OAuth token without end-user involvement. Applications must save the "token" property in the response because changes take effect immediately. You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) when accessing this endpoint, using the OAuth application's `client_id` and `client_secret` as the username and password. Invalid tokens will return `404 NOT FOUND`.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#reset-a-token>
     *
     * **Parameters:**
     *
     * * `client_id: &str` -- The client ID of your GitHub app.
     */
    pub async fn reset_token(
        &self,
        client_id: &str,
        body: &crate::types::AppsCheckTokenRequest,
    ) -> ClientResult<crate::Response<crate::types::Authorization>> {
        let url = self.client.url(
            &format!(
                "/applications/{}/token",
                crate::progenitor_support::encode_path(client_id),
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
    /**
     * Create a scoped access token.
     *
     * This function performs a `POST` to the `/applications/{client_id}/token/scoped` endpoint.
     *
     * Use a non-scoped user-to-server OAuth access token to create a repository scoped and/or permission scoped user-to-server OAuth access token. You can specify which repositories the token can access and which permissions are granted to the token. You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) when accessing this endpoint, using the OAuth application's `client_id` and `client_secret` as the username and password. Invalid tokens will return `404 NOT FOUND`.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#create-a-scoped-access-token>
     *
     * **Parameters:**
     *
     * * `client_id: &str` -- The client ID of your GitHub app.
     */
    pub async fn scope_token(
        &self,
        client_id: &str,
        body: &crate::types::AppsScopeTokenRequest,
    ) -> ClientResult<crate::Response<crate::types::Authorization>> {
        let url = self.client.url(
            &format!(
                "/applications/{}/token/scoped",
                crate::progenitor_support::encode_path(client_id),
            ),
            None,
        );
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
     * Check an authorization.
     *
     * This function performs a `GET` to the `/applications/{client_id}/tokens/{access_token}` endpoint.
     *
     * **Deprecation Notice:** GitHub will discontinue OAuth endpoints that contain `access_token` in the path parameter. We have introduced new endpoints that allow you to securely manage tokens for OAuth Apps by moving `access_token` to the request body. For more information, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-app-endpoint/).
     *
     * OAuth applications can use a special API method for checking OAuth token validity without exceeding the normal rate limits for failed login attempts. Authentication works differently with this particular endpoint. You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) when accessing this endpoint, using the OAuth application's `client_id` and `client_secret` as the username and password. Invalid tokens will return `404 NOT FOUND`.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#check-an-authorization>
     *
     * **Parameters:**
     *
     * * `client_id: &str` -- The client ID of your GitHub app.
     * * `access_token: &str`
     */
    pub async fn check_authorization(
        &self,
        client_id: &str,
        access_token: &str,
    ) -> ClientResult<crate::Response<crate::types::Authorization>> {
        let url = self.client.url(
            &format!(
                "/applications/{}/tokens/{}",
                crate::progenitor_support::encode_path(client_id),
                crate::progenitor_support::encode_path(access_token),
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
     * Reset an authorization.
     *
     * This function performs a `POST` to the `/applications/{client_id}/tokens/{access_token}` endpoint.
     *
     * **Deprecation Notice:** GitHub will discontinue OAuth endpoints that contain `access_token` in the path parameter. We have introduced new endpoints that allow you to securely manage tokens for OAuth Apps by moving `access_token` to the request body. For more information, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-app-endpoint/).
     *
     * OAuth applications can use this API method to reset a valid OAuth token without end-user involvement. Applications must save the "token" property in the response because changes take effect immediately. You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) when accessing this endpoint, using the OAuth application's `client_id` and `client_secret` as the username and password. Invalid tokens will return `404 NOT FOUND`.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#reset-an-authorization>
     *
     * **Parameters:**
     *
     * * `client_id: &str` -- The client ID of your GitHub app.
     * * `access_token: &str`
     */
    pub async fn reset_authorization(
        &self,
        client_id: &str,
        access_token: &str,
    ) -> ClientResult<crate::Response<crate::types::Authorization>> {
        let url = self.client.url(
            &format!(
                "/applications/{}/tokens/{}",
                crate::progenitor_support::encode_path(client_id),
                crate::progenitor_support::encode_path(access_token),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Revoke an authorization for an application.
     *
     * This function performs a `DELETE` to the `/applications/{client_id}/tokens/{access_token}` endpoint.
     *
     * **Deprecation Notice:** GitHub will discontinue OAuth endpoints that contain `access_token` in the path parameter. We have introduced new endpoints that allow you to securely manage tokens for OAuth Apps by moving `access_token` to the request body. For more information, see the [blog post](https://developer.github.com/changes/2020-02-14-deprecating-oauth-app-endpoint/).
     *
     * OAuth application owners can revoke a single token for an OAuth application. You must use [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) when accessing this endpoint, using the OAuth application's `client_id` and `client_secret` as the username and password.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#revoke-an-authorization-for-an-application>
     *
     * **Parameters:**
     *
     * * `client_id: &str` -- The client ID of your GitHub app.
     * * `access_token: &str`
     */
    pub async fn revoke_authorization_for_application(
        &self,
        client_id: &str,
        access_token: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/applications/{}/tokens/{}",
                crate::progenitor_support::encode_path(client_id),
                crate::progenitor_support::encode_path(access_token),
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
     * Get an app.
     *
     * This function performs a `GET` to the `/apps/{app_slug}` endpoint.
     *
     * **Note**: The `:app_slug` is just the URL-friendly name of your GitHub App. You can find this on the settings page for your GitHub App (e.g., `https://github.com/settings/apps/:app_slug`).
     *
     * If the GitHub App you specify is public, you can access this endpoint without authenticating. If the GitHub App you specify is private, you must authenticate with a [personal access token](https://help.github.com/articles/creating-a-personal-access-token-for-the-command-line/) or an [installation access token](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-an-installation) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps/#get-an-app>
     *
     * **Parameters:**
     *
     * * `app_slug: &str`
     */
    pub async fn get_by_slug(
        &self,
        app_slug: &str,
    ) -> ClientResult<crate::Response<crate::types::GitHubApp>> {
        let url = self.client.url(
            &format!("/apps/{}", crate::progenitor_support::encode_path(app_slug),),
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
     * List repositories accessible to the app installation.
     *
     * This function performs a `GET` to the `/installation/repositories` endpoint.
     *
     * List repositories that an app installation can access.
     *
     * You must use an [installation access token](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-an-installation) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#list-repositories-accessible-to-the-app-installation>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_repos_accessible_to_installation(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::AppsListInstallationReposResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/installation/repositories?{}", query_), None);
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
     * Revoke an installation access token.
     *
     * This function performs a `DELETE` to the `/installation/token` endpoint.
     *
     * Revokes the installation token you're using to authenticate as an installation and access this endpoint.
     *
     * Once an installation token is revoked, the token is invalidated and cannot be used. Other endpoints that require the revoked installation token must have a new installation token to work. You can create a new token using the "[Create an installation access token for an app](https://docs.github.com/rest/reference/apps#create-an-installation-access-token-for-an-app)" endpoint.
     *
     * You must use an [installation access token](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-an-installation) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#revoke-an-installation-access-token>
     */
    pub async fn revoke_installation_access_token(&self) -> ClientResult<crate::Response<()>> {
        let url = self.client.url("/installation/token", None);
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
     * Get a subscription plan for an account.
     *
     * This function performs a `GET` to the `/marketplace_listing/accounts/{account_id}` endpoint.
     *
     * Shows whether the user or organization account actively subscribes to a plan listed by the authenticated GitHub App. When someone submits a plan change that won't be processed until the end of their billing cycle, you will also see the upcoming pending change.
     *
     * GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth Apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#get-a-subscription-plan-for-an-account>
     *
     * **Parameters:**
     *
     * * `account_id: i64` -- account_id parameter.
     */
    pub async fn get_subscription_plan_for_account(
        &self,
        account_id: i64,
    ) -> ClientResult<crate::Response<crate::types::MarketplacePurchaseData>> {
        let url = self.client.url(
            &format!(
                "/marketplace_listing/accounts/{}",
                crate::progenitor_support::encode_path(&account_id.to_string()),
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
     * List plans.
     *
     * This function performs a `GET` to the `/marketplace_listing/plans` endpoint.
     *
     * Lists all plans that are part of your GitHub Marketplace listing.
     *
     * GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth Apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#list-plans>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_plans(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::MarketplaceListingPlan>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/marketplace_listing/plans?{}", query_), None);
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
     * List plans.
     *
     * This function performs a `GET` to the `/marketplace_listing/plans` endpoint.
     *
     * As opposed to `list_plans`, this function returns all the pages of the request at once.
     *
     * Lists all plans that are part of your GitHub Marketplace listing.
     *
     * GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth Apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#list-plans>
     */
    pub async fn list_all_plans(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::MarketplaceListingPlan>>> {
        let url = self.client.url("/marketplace_listing/plans", None);
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
     * List accounts for a plan.
     *
     * This function performs a `GET` to the `/marketplace_listing/plans/{plan_id}/accounts` endpoint.
     *
     * Returns user and organization accounts associated with the specified plan, including free plans. For per-seat pricing, you see the list of accounts that have purchased the plan, including the number of seats purchased. When someone submits a plan change that won't be processed until the end of their billing cycle, you will also see the upcoming pending change.
     *
     * GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth Apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#list-accounts-for-a-plan>
     *
     * **Parameters:**
     *
     * * `plan_id: i64` -- plan_id parameter.
     * * `sort: crate::types::Sort` -- One of `created` (when the repository was starred) or `updated` (when it was last pushed to).
     * * `direction: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_accounts_for_plan(
        &self,
        plan_id: i64,
        sort: crate::types::Sort,
        direction: crate::types::Order,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::MarketplacePurchaseData>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/marketplace_listing/plans/{}/accounts?{}",
                crate::progenitor_support::encode_path(&plan_id.to_string()),
                query_
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
     * List accounts for a plan.
     *
     * This function performs a `GET` to the `/marketplace_listing/plans/{plan_id}/accounts` endpoint.
     *
     * As opposed to `list_accounts_for_plan`, this function returns all the pages of the request at once.
     *
     * Returns user and organization accounts associated with the specified plan, including free plans. For per-seat pricing, you see the list of accounts that have purchased the plan, including the number of seats purchased. When someone submits a plan change that won't be processed until the end of their billing cycle, you will also see the upcoming pending change.
     *
     * GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth Apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#list-accounts-for-a-plan>
     */
    pub async fn list_all_accounts_for_plan(
        &self,
        plan_id: i64,
        sort: crate::types::Sort,
        direction: crate::types::Order,
    ) -> ClientResult<crate::Response<Vec<crate::types::MarketplacePurchaseData>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/marketplace_listing/plans/{}/accounts?{}",
                crate::progenitor_support::encode_path(&plan_id.to_string()),
                query_
            ),
            None,
        );
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
     * Get a subscription plan for an account (stubbed).
     *
     * This function performs a `GET` to the `/marketplace_listing/stubbed/accounts/{account_id}` endpoint.
     *
     * Shows whether the user or organization account actively subscribes to a plan listed by the authenticated GitHub App. When someone submits a plan change that won't be processed until the end of their billing cycle, you will also see the upcoming pending change.
     *
     * GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth Apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#get-a-subscription-plan-for-an-account-stubbed>
     *
     * **Parameters:**
     *
     * * `account_id: i64` -- account_id parameter.
     */
    pub async fn get_subscription_plan_for_account_stubbed(
        &self,
        account_id: i64,
    ) -> ClientResult<crate::Response<crate::types::MarketplacePurchaseData>> {
        let url = self.client.url(
            &format!(
                "/marketplace_listing/stubbed/accounts/{}",
                crate::progenitor_support::encode_path(&account_id.to_string()),
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
     * List plans (stubbed).
     *
     * This function performs a `GET` to the `/marketplace_listing/stubbed/plans` endpoint.
     *
     * Lists all plans that are part of your GitHub Marketplace listing.
     *
     * GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth Apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#list-plans-stubbed>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_plans_stubbed(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::MarketplaceListingPlan>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/marketplace_listing/stubbed/plans?{}", query_),
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
     * List plans (stubbed).
     *
     * This function performs a `GET` to the `/marketplace_listing/stubbed/plans` endpoint.
     *
     * As opposed to `list_plans_stubbed`, this function returns all the pages of the request at once.
     *
     * Lists all plans that are part of your GitHub Marketplace listing.
     *
     * GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth Apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#list-plans-stubbed>
     */
    pub async fn list_all_plans_stubbed(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::MarketplaceListingPlan>>> {
        let url = self.client.url("/marketplace_listing/stubbed/plans", None);
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
     * List accounts for a plan (stubbed).
     *
     * This function performs a `GET` to the `/marketplace_listing/stubbed/plans/{plan_id}/accounts` endpoint.
     *
     * Returns repository and organization accounts associated with the specified plan, including free plans. For per-seat pricing, you see the list of accounts that have purchased the plan, including the number of seats purchased. When someone submits a plan change that won't be processed until the end of their billing cycle, you will also see the upcoming pending change.
     *
     * GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth Apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#list-accounts-for-a-plan-stubbed>
     *
     * **Parameters:**
     *
     * * `plan_id: i64` -- plan_id parameter.
     * * `sort: crate::types::Sort` -- One of `created` (when the repository was starred) or `updated` (when it was last pushed to).
     * * `direction: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_accounts_for_plan_stubbed(
        &self,
        plan_id: i64,
        sort: crate::types::Sort,
        direction: crate::types::Order,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::MarketplacePurchaseData>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/marketplace_listing/stubbed/plans/{}/accounts?{}",
                crate::progenitor_support::encode_path(&plan_id.to_string()),
                query_
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
     * List accounts for a plan (stubbed).
     *
     * This function performs a `GET` to the `/marketplace_listing/stubbed/plans/{plan_id}/accounts` endpoint.
     *
     * As opposed to `list_accounts_for_plan_stubbed`, this function returns all the pages of the request at once.
     *
     * Returns repository and organization accounts associated with the specified plan, including free plans. For per-seat pricing, you see the list of accounts that have purchased the plan, including the number of seats purchased. When someone submits a plan change that won't be processed until the end of their billing cycle, you will also see the upcoming pending change.
     *
     * GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth Apps must use [basic authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication) with their client ID and client secret to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#list-accounts-for-a-plan-stubbed>
     */
    pub async fn list_all_accounts_for_plan_stubbed(
        &self,
        plan_id: i64,
        sort: crate::types::Sort,
        direction: crate::types::Order,
    ) -> ClientResult<crate::Response<Vec<crate::types::MarketplacePurchaseData>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/marketplace_listing/stubbed/plans/{}/accounts?{}",
                crate::progenitor_support::encode_path(&plan_id.to_string()),
                query_
            ),
            None,
        );
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
     * Get an organization installation for the authenticated app.
     *
     * This function performs a `GET` to the `/orgs/{org}/installation` endpoint.
     *
     * Enables an authenticated GitHub App to find the organization's installation information.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#get-an-organization-installation-for-the-authenticated-app>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn get_org_installation(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::Installation>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/installation",
                crate::progenitor_support::encode_path(org),
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
     * Create a content attachment.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/content_references/{content_reference_id}/attachments` endpoint.
     *
     * Creates an attachment under a content reference URL in the body or comment of an issue or pull request. Use the `id` and `repository` `full_name` of the content reference from the [`content_reference` event](https://docs.github.com/webhooks/event-payloads/#content_reference) to create an attachment.
     *
     * The app must create a content attachment within six hours of the content reference URL being posted. See "[Using content attachments](https://docs.github.com/apps/using-content-attachments/)" for details about content attachments.
     *
     * You must use an [installation access token](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-an-installation) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#create-a-content-attachment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The owner of the repository. Determined from the `repository` `full_name` of the `content_reference` event.
     * * `repo: &str` -- The name of the repository. Determined from the `repository` `full_name` of the `content_reference` event.
     * * `content_reference_id: i64` -- The `id` of the `content_reference` event.
     */
    pub async fn create_content_attachment(
        &self,
        owner: &str,
        repo: &str,
        content_reference_id: i64,
        body: &crate::types::TeamsUpdateDiscussionInOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::ContentReferenceAttachment>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/content_references/{}/attachments",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&content_reference_id.to_string()),
            ),
            None,
        );
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
     * Get a repository installation for the authenticated app.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/installation` endpoint.
     *
     * Enables an authenticated GitHub App to find the repository's installation information. The installation's account type will be either an organization or a user account, depending which account the repository belongs to.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#get-a-repository-installation-for-the-authenticated-app>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get_repo_installation(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::Installation>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/installation",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List app installations accessible to the user access token.
     *
     * This function performs a `GET` to the `/user/installations` endpoint.
     *
     * Lists installations of your GitHub App that the authenticated user has explicit permission (`:read`, `:write`, or `:admin`) to access.
     *
     * You must use a [user-to-server OAuth access token](https://docs.github.com/apps/building-github-apps/identifying-and-authorizing-users-for-github-apps/#identifying-users-on-your-site), created for a user who has authorized your GitHub App, to access this endpoint.
     *
     * The authenticated user has explicit permission to access repositories they own, repositories where they are a collaborator, and repositories that they can access through an organization membership.
     *
     * You can find the permissions for the installation under the `permissions` key.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#list-app-installations-accessible-to-the-user-access-token>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_installations_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::AppsListInstallationsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/user/installations?{}", query_), None);
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
     * List repositories accessible to the user access token.
     *
     * This function performs a `GET` to the `/user/installations/{installation_id}/repositories` endpoint.
     *
     * List repositories that the authenticated user has explicit permission (`:read`, `:write`, or `:admin`) to access for an installation.
     *
     * The authenticated user has explicit permission to access repositories they own, repositories where they are a collaborator, and repositories that they can access through an organization membership.
     *
     * You must use a [user-to-server OAuth access token](https://docs.github.com/apps/building-github-apps/identifying-and-authorizing-users-for-github-apps/#identifying-users-on-your-site), created for a user who has authorized your GitHub App, to access this endpoint.
     *
     * The access the user has to each repository is included in the hash under the `permissions` key.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#list-repositories-accessible-to-the-user-access-token>
     *
     * **Parameters:**
     *
     * * `installation_id: i64` -- installation_id parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_installation_repos_for_authenticated_user(
        &self,
        installation_id: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::AppsListInstallationReposResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/user/installations/{}/repositories?{}",
                crate::progenitor_support::encode_path(&installation_id.to_string()),
                query_
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
     * Add a repository to an app installation.
     *
     * This function performs a `PUT` to the `/user/installations/{installation_id}/repositories/{repository_id}` endpoint.
     *
     * Add a single repository to an installation. The authenticated user must have admin access to the repository.
     *
     * You must use a personal access token (which you can create via the [command line](https://docs.github.com/github/authenticating-to-github/creating-a-personal-access-token) or [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication)) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#add-a-repository-to-an-app-installation>
     *
     * **Parameters:**
     *
     * * `installation_id: i64` -- installation_id parameter.
     * * `repository_id: i64`
     */
    pub async fn add_repo_to_installation(
        &self,
        installation_id: i64,
        repository_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/installations/{}/repositories/{}",
                crate::progenitor_support::encode_path(&installation_id.to_string()),
                crate::progenitor_support::encode_path(&repository_id.to_string()),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Remove a repository from an app installation.
     *
     * This function performs a `DELETE` to the `/user/installations/{installation_id}/repositories/{repository_id}` endpoint.
     *
     * Remove a single repository from an installation. The authenticated user must have admin access to the repository.
     *
     * You must use a personal access token (which you can create via the [command line](https://docs.github.com/github/authenticating-to-github/creating-a-personal-access-token) or [Basic Authentication](https://docs.github.com/rest/overview/other-authentication-methods#basic-authentication)) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#remove-a-repository-from-an-app-installation>
     *
     * **Parameters:**
     *
     * * `installation_id: i64` -- installation_id parameter.
     * * `repository_id: i64`
     */
    pub async fn remove_repo_from_installation(
        &self,
        installation_id: i64,
        repository_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/installations/{}/repositories/{}",
                crate::progenitor_support::encode_path(&installation_id.to_string()),
                crate::progenitor_support::encode_path(&repository_id.to_string()),
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
     * List subscriptions for the authenticated user.
     *
     * This function performs a `GET` to the `/user/marketplace_purchases` endpoint.
     *
     * Lists the active subscriptions for the authenticated user. You must use a [user-to-server OAuth access token](https://docs.github.com/apps/building-github-apps/identifying-and-authorizing-users-for-github-apps/#identifying-users-on-your-site), created for a user who has authorized your GitHub App, to access this endpoint. . OAuth Apps must authenticate using an [OAuth token](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/apps#list-subscriptions-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_subscriptions_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::UserMarketplacePurchase>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/user/marketplace_purchases?{}", query_), None);
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
     * List subscriptions for the authenticated user.
     *
     * This function performs a `GET` to the `/user/marketplace_purchases` endpoint.
     *
     * As opposed to `list_subscriptions_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * Lists the active subscriptions for the authenticated user. You must use a [user-to-server OAuth access token](https://docs.github.com/apps/building-github-apps/identifying-and-authorizing-users-for-github-apps/#identifying-users-on-your-site), created for a user who has authorized your GitHub App, to access this endpoint. . OAuth Apps must authenticate using an [OAuth token](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/apps#list-subscriptions-for-the-authenticated-user>
     */
    pub async fn list_all_subscriptions_for_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::UserMarketplacePurchase>>> {
        let url = self.client.url("/user/marketplace_purchases", None);
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
     * List subscriptions for the authenticated user (stubbed).
     *
     * This function performs a `GET` to the `/user/marketplace_purchases/stubbed` endpoint.
     *
     * Lists the active subscriptions for the authenticated user. You must use a [user-to-server OAuth access token](https://docs.github.com/apps/building-github-apps/identifying-and-authorizing-users-for-github-apps/#identifying-users-on-your-site), created for a user who has authorized your GitHub App, to access this endpoint. . OAuth Apps must authenticate using an [OAuth token](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/apps#list-subscriptions-for-the-authenticated-user-stubbed>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_subscriptions_for_authenticated_user_stubbed(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::UserMarketplacePurchase>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/user/marketplace_purchases/stubbed?{}", query_),
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
     * List subscriptions for the authenticated user (stubbed).
     *
     * This function performs a `GET` to the `/user/marketplace_purchases/stubbed` endpoint.
     *
     * As opposed to `list_subscriptions_for_authenticated_user_stubbed`, this function returns all the pages of the request at once.
     *
     * Lists the active subscriptions for the authenticated user. You must use a [user-to-server OAuth access token](https://docs.github.com/apps/building-github-apps/identifying-and-authorizing-users-for-github-apps/#identifying-users-on-your-site), created for a user who has authorized your GitHub App, to access this endpoint. . OAuth Apps must authenticate using an [OAuth token](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/apps#list-subscriptions-for-the-authenticated-user-stubbed>
     */
    pub async fn list_all_subscriptions_for_authenticated_user_stubbed(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::UserMarketplacePurchase>>> {
        let url = self.client.url("/user/marketplace_purchases/stubbed", None);
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
     * Get a user installation for the authenticated app.
     *
     * This function performs a `GET` to the `/users/{username}/installation` endpoint.
     *
     * Enables an authenticated GitHub App to find the user’s installation information.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#get-a-user-installation-for-the-authenticated-app>
     *
     * **Parameters:**
     *
     * * `username: &str`
     */
    pub async fn get_user_installation(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::Installation>> {
        let url = self.client.url(
            &format!(
                "/users/{}/installation",
                crate::progenitor_support::encode_path(username),
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
}
