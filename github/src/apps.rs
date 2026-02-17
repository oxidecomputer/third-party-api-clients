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
     * Returns the GitHub App associated with the authentication credentials used. To see how many app installations are associated with this GitHub App, see the `installations_count` in the response. For more details about your app's installations, see the "[List installations for the authenticated app](https://docs.github.com/rest/apps/apps#list-installations-for-the-authenticated-app)" endpoint.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/apps/apps#get-the-authenticated-app>
     */
    pub async fn get_authenticated(
        &self,
    ) -> ClientResult<crate::Response<crate::types::GitHubApp>> {
        let url = self.client.url(&"/app".to_string(), None);
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
     * FROM: <https://docs.github.com/rest/apps/apps#create-a-github-app-from-a-manifest>
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
                crate::progenitor_support::encode_path(&code.to_string()),
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
     * Get a webhook configuration for an app.
     *
     * This function performs a `GET` to the `/app/hook/config` endpoint.
     *
     * Returns the webhook configuration for a GitHub App. For more information about configuring a webhook for your app, see "[Creating a GitHub App](/developers/apps/creating-a-github-app)."
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/apps/webhooks#get-a-webhook-configuration-for-an-app>
     */
    pub async fn get_webhook_config_for_app(
        &self,
    ) -> ClientResult<crate::Response<crate::types::WebhookConfig>> {
        let url = self.client.url(&"/app/hook/config".to_string(), None);
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
     * FROM: <https://docs.github.com/rest/apps/webhooks#update-a-webhook-configuration-for-an-app>
     */
    pub async fn update_webhook_config_for_app(
        &self,
        body: &crate::types::AppsUpdateWebhookConfigAppRequest,
    ) -> ClientResult<crate::Response<crate::types::WebhookConfig>> {
        let url = self.client.url(&"/app/hook/config".to_string(), None);
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
     * FROM: <https://docs.github.com/rest/apps/webhooks#list-deliveries-for-an-app-webhook>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
     * FROM: <https://docs.github.com/rest/apps/webhooks#list-deliveries-for-an-app-webhook>
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
     * FROM: <https://docs.github.com/rest/apps/webhooks#get-a-delivery-for-an-app-webhook>
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
     * FROM: <https://docs.github.com/rest/apps/webhooks#redeliver-a-delivery-for-an-app-webhook>
     *
     * **Parameters:**
     *
     * * `delivery_id: i64`
     */
    pub async fn redeliver_webhook_delivery(
        &self,
        delivery_id: i64,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
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
     * List installation requests for the authenticated app.
     *
     * This function performs a `GET` to the `/app/installation-requests` endpoint.
     *
     * Lists all the pending installation requests for the authenticated GitHub App.
     *
     * FROM: <https://docs.github.com/rest/apps/apps#list-installation-requests-for-the-authenticated-app>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_installation_requests_for_authenticated_app(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::IntegrationInstallationRequest>>> {
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
            .url(&format!("/app/installation-requests?{}", query_), None);
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
     * List installation requests for the authenticated app.
     *
     * This function performs a `GET` to the `/app/installation-requests` endpoint.
     *
     * As opposed to `list_installation_requests_for_authenticated_app`, this function returns all the pages of the request at once.
     *
     * Lists all the pending installation requests for the authenticated GitHub App.
     *
     * FROM: <https://docs.github.com/rest/apps/apps#list-installation-requests-for-the-authenticated-app>
     */
    pub async fn list_all_installation_requests_for_authenticated_app(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::IntegrationInstallationRequest>>> {
        let url = self
            .client
            .url(&"/app/installation-requests".to_string(), None);
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
     * List installations for the authenticated app.
     *
     * This function performs a `GET` to the `/app/installations` endpoint.
     *
     * The permissions the installation has are included under the `permissions` key.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/apps/apps#list-installations-for-the-authenticated-app>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
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
     * The permissions the installation has are included under the `permissions` key.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/apps/apps#list-installations-for-the-authenticated-app>
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
     * Enables an authenticated GitHub App to find an installation's information using the installation id.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/apps/apps#get-an-installation-for-the-authenticated-app>
     *
     * **Parameters:**
     *
     * * `installation_id: i64` -- The unique identifier of the installation.
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
     * Uninstalls a GitHub App on a user, organization, or enterprise account. If you prefer to temporarily suspend an app's access to your account's resources, then we recommend the "[Suspend an app installation](https://docs.github.com/rest/apps/apps#suspend-an-app-installation)" endpoint.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/apps/apps#delete-an-installation-for-the-authenticated-app>
     *
     * **Parameters:**
     *
     * * `installation_id: i64` -- The unique identifier of the installation.
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
     * Creates an installation access token that enables a GitHub App to make authenticated API requests for the app's installation on an organization or individual account. Installation tokens expire one hour from the time you create them. Using an expired token produces a status code of `401 - Unauthorized`, and requires creating a new installation token. By default the installation token has access to all repositories that the installation can access.
     *
     * Optionally, you can use the `repositories` or `repository_ids` body parameters to specify individual repositories that the installation access token can access. If you don't use `repositories` or `repository_ids` to grant access to specific repositories, the installation access token will have access to all repositories that the installation was granted access to. The installation access token cannot be granted access to repositories that the installation was not granted access to. Up to 500 repositories can be listed in this manner.
     *
     * Optionally, use the `permissions` body parameter to specify the permissions that the installation access token should have. If `permissions` is not specified, the installation access token will have all of the permissions that were granted to the app. The installation access token cannot be granted permissions that the app was not granted.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/apps/apps#create-an-installation-access-token-for-an-app>
     *
     * **Parameters:**
     *
     * * `installation_id: i64` -- The unique identifier of the installation.
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
     * Suspends a GitHub App on a user, organization, or enterprise account, which blocks the app from accessing the account's resources. When a GitHub App is suspended, the app's access to the GitHub API or webhook events is blocked for that account.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/apps/apps#suspend-an-app-installation>
     *
     * **Parameters:**
     *
     * * `installation_id: i64` -- The unique identifier of the installation.
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
     * FROM: <https://docs.github.com/rest/apps/apps#unsuspend-an-app-installation>
     *
     * **Parameters:**
     *
     * * `installation_id: i64` -- The unique identifier of the installation.
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
     * OAuth and GitHub application owners can revoke a grant for their application and a specific user. You must provide a valid OAuth `access_token` as an input parameter and the grant for the token's owner will be deleted.
     * Deleting an application's grant will also delete all OAuth tokens associated with the application for the user. Once deleted, the application will have no access to the user's account and will no longer be listed on [the application authorizations settings screen within GitHub](https://github.com/settings/applications#authorized).
     *
     * FROM: <https://docs.github.com/rest/apps/oauth-applications#delete-an-app-authorization>
     *
     * **Parameters:**
     *
     * * `client_id: &str` -- The client ID of the GitHub app.
     */
    pub async fn delete_authorization(
        &self,
        client_id: &str,
        body: &crate::types::AppsCheckTokenRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/applications/{}/grant",
                crate::progenitor_support::encode_path(&client_id.to_string()),
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
     * Check a token.
     *
     * This function performs a `POST` to the `/applications/{client_id}/token` endpoint.
     *
     * OAuth applications and GitHub applications with OAuth authorizations can use this API method for checking OAuth token validity without exceeding the normal rate limits for failed login attempts. Authentication works differently with this particular endpoint. Invalid tokens will return `404 NOT FOUND`.
     *
     * FROM: <https://docs.github.com/rest/apps/oauth-applications#check-a-token>
     *
     * **Parameters:**
     *
     * * `client_id: &str` -- The client ID of the GitHub app.
     */
    pub async fn check_token(
        &self,
        client_id: &str,
        body: &crate::types::AppsCheckTokenRequest,
    ) -> ClientResult<crate::Response<crate::types::Authorization>> {
        let url = self.client.url(
            &format!(
                "/applications/{}/token",
                crate::progenitor_support::encode_path(&client_id.to_string()),
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
     * OAuth  or GitHub application owners can revoke a single token for an OAuth application or a GitHub application with an OAuth authorization.
     *
     * FROM: <https://docs.github.com/rest/apps/oauth-applications#delete-an-app-token>
     *
     * **Parameters:**
     *
     * * `client_id: &str` -- The client ID of the GitHub app.
     */
    pub async fn delete_token(
        &self,
        client_id: &str,
        body: &crate::types::AppsCheckTokenRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/applications/{}/token",
                crate::progenitor_support::encode_path(&client_id.to_string()),
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
     * OAuth applications and GitHub applications with OAuth authorizations can use this API method to reset a valid OAuth token without end-user involvement. Applications must save the "token" property in the response because changes take effect immediately. Invalid tokens will return `404 NOT FOUND`.
     *
     * FROM: <https://docs.github.com/rest/apps/oauth-applications#reset-a-token>
     *
     * **Parameters:**
     *
     * * `client_id: &str` -- The client ID of the GitHub app.
     */
    pub async fn reset_token(
        &self,
        client_id: &str,
        body: &crate::types::AppsCheckTokenRequest,
    ) -> ClientResult<crate::Response<crate::types::Authorization>> {
        let url = self.client.url(
            &format!(
                "/applications/{}/token",
                crate::progenitor_support::encode_path(&client_id.to_string()),
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
     * Use a non-scoped user access token to create a repository-scoped and/or permission-scoped user access token. You can specify
     * which repositories the token can access and which permissions are granted to the
     * token.
     *
     * Invalid tokens will return `404 NOT FOUND`.
     *
     * FROM: <https://docs.github.com/rest/apps/apps#create-a-scoped-access-token>
     *
     * **Parameters:**
     *
     * * `client_id: &str` -- The client ID of the GitHub app.
     */
    pub async fn scope_token(
        &self,
        client_id: &str,
        body: &crate::types::AppsScopeTokenRequest,
    ) -> ClientResult<crate::Response<crate::types::Authorization>> {
        let url = self.client.url(
            &format!(
                "/applications/{}/token/scoped",
                crate::progenitor_support::encode_path(&client_id.to_string()),
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
     * Get an app.
     *
     * This function performs a `GET` to the `/apps/{app_slug}` endpoint.
     *
     * > [!NOTE]
     * > The `:app_slug` is just the URL-friendly name of your GitHub App. You can find this on the settings page for your GitHub App (e.g., `https://github.com/settings/apps/:app_slug`).
     *
     * FROM: <https://docs.github.com/rest/apps/apps#get-an-app>
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
            &format!(
                "/apps/{}",
                crate::progenitor_support::encode_path(&app_slug.to_string()),
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
     * List repositories accessible to the app installation.
     *
     * This function performs a `GET` to the `/installation/repositories` endpoint.
     *
     * List repositories that an app installation can access.
     *
     * FROM: <https://docs.github.com/rest/apps/installations#list-repositories-accessible-to-the-app-installation>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
     * Once an installation token is revoked, the token is invalidated and cannot be used. Other endpoints that require the revoked installation token must have a new installation token to work. You can create a new token using the "[Create an installation access token for an app](https://docs.github.com/rest/apps/apps#create-an-installation-access-token-for-an-app)" endpoint.
     *
     * FROM: <https://docs.github.com/rest/apps/installations#revoke-an-installation-access-token>
     */
    pub async fn revoke_installation_access_token(&self) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(&"/installation/token".to_string(), None);
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
     * GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/authentication/authenticating-to-the-rest-api#using-basic-authentication) with their client ID and client secret to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/apps/marketplace#get-a-subscription-plan-for-an-account>
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
     * GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/authentication/authenticating-to-the-rest-api#using-basic-authentication) with their client ID and client secret to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/apps/marketplace#list-plans>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
     * GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/authentication/authenticating-to-the-rest-api#using-basic-authentication) with their client ID and client secret to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/apps/marketplace#list-plans>
     */
    pub async fn list_all_plans(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::MarketplaceListingPlan>>> {
        let url = self
            .client
            .url(&"/marketplace_listing/plans".to_string(), None);
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
     * GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/authentication/authenticating-to-the-rest-api#using-basic-authentication) with their client ID and client secret to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/apps/marketplace#list-accounts-for-a-plan>
     *
     * **Parameters:**
     *
     * * `plan_id: i64` -- The unique identifier of the plan.
     * * `sort: crate::types::SortData` -- The property to sort the results by.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_accounts_for_plan(
        &self,
        plan_id: i64,
        sort: crate::types::SortData,
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
     * GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/authentication/authenticating-to-the-rest-api#using-basic-authentication) with their client ID and client secret to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/apps/marketplace#list-accounts-for-a-plan>
     */
    pub async fn list_all_accounts_for_plan(
        &self,
        plan_id: i64,
        sort: crate::types::SortData,
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
     * GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/authentication/authenticating-to-the-rest-api#using-basic-authentication) with their client ID and client secret to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/apps/marketplace#get-a-subscription-plan-for-an-account-stubbed>
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
     * GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/authentication/authenticating-to-the-rest-api#using-basic-authentication) with their client ID and client secret to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/apps/marketplace#list-plans-stubbed>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
     * GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/authentication/authenticating-to-the-rest-api#using-basic-authentication) with their client ID and client secret to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/apps/marketplace#list-plans-stubbed>
     */
    pub async fn list_all_plans_stubbed(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::MarketplaceListingPlan>>> {
        let url = self
            .client
            .url(&"/marketplace_listing/stubbed/plans".to_string(), None);
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
     * GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/authentication/authenticating-to-the-rest-api#using-basic-authentication) with their client ID and client secret to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/apps/marketplace#list-accounts-for-a-plan-stubbed>
     *
     * **Parameters:**
     *
     * * `plan_id: i64` -- The unique identifier of the plan.
     * * `sort: crate::types::SortData` -- The property to sort the results by.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_accounts_for_plan_stubbed(
        &self,
        plan_id: i64,
        sort: crate::types::SortData,
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
     * GitHub Apps must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint. OAuth apps must use [basic authentication](https://docs.github.com/rest/authentication/authenticating-to-the-rest-api#using-basic-authentication) with their client ID and client secret to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/apps/marketplace#list-accounts-for-a-plan-stubbed>
     */
    pub async fn list_all_accounts_for_plan_stubbed(
        &self,
        plan_id: i64,
        sort: crate::types::SortData,
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
     * FROM: <https://docs.github.com/rest/apps/apps#get-an-organization-installation-for-the-authenticated-app>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_org_installation(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::Installation>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/installation",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Get a repository installation for the authenticated app.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/installation` endpoint.
     *
     * Enables an authenticated GitHub App to find the repository's installation information. The installation's account type will be either an organization or a user account, depending which account the repository belongs to.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/apps/apps#get-a-repository-installation-for-the-authenticated-app>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_repo_installation(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::Installation>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/installation",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * The authenticated user has explicit permission to access repositories they own, repositories where they are a collaborator, and repositories that they can access through an organization membership.
     *
     * You can find the permissions for the installation under the `permissions` key.
     *
     * FROM: <https://docs.github.com/rest/apps/installations#list-app-installations-accessible-to-the-user-access-token>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
     * The access the user has to each repository is included in the hash under the `permissions` key.
     *
     * FROM: <https://docs.github.com/rest/apps/installations#list-repositories-accessible-to-the-user-access-token>
     *
     * **Parameters:**
     *
     * * `installation_id: i64` -- The unique identifier of the installation.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
     * This endpoint only works for PATs (classic) with the `repo` scope.
     *
     * FROM: <https://docs.github.com/rest/apps/installations#add-a-repository-to-an-app-installation>
     *
     * **Parameters:**
     *
     * * `installation_id: i64` -- The unique identifier of the installation.
     * * `repository_id: i64` -- The unique identifier of the repository.
     */
    pub async fn add_repo_to_installation_for_authenticated_user(
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
     * Remove a single repository from an installation. The authenticated user must have admin access to the repository. The installation must have the `repository_selection` of `selected`.
     *
     * This endpoint only works for PATs (classic) with the `repo` scope.
     *
     * FROM: <https://docs.github.com/rest/apps/installations#remove-a-repository-from-an-app-installation>
     *
     * **Parameters:**
     *
     * * `installation_id: i64` -- The unique identifier of the installation.
     * * `repository_id: i64` -- The unique identifier of the repository.
     */
    pub async fn remove_repo_from_installation_for_authenticated_user(
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
     * Lists the active subscriptions for the authenticated user.
     *
     * FROM: <https://docs.github.com/rest/apps/marketplace#list-subscriptions-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
     * Lists the active subscriptions for the authenticated user.
     *
     * FROM: <https://docs.github.com/rest/apps/marketplace#list-subscriptions-for-the-authenticated-user>
     */
    pub async fn list_all_subscriptions_for_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::UserMarketplacePurchase>>> {
        let url = self
            .client
            .url(&"/user/marketplace_purchases".to_string(), None);
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
     * Lists the active subscriptions for the authenticated user.
     *
     * FROM: <https://docs.github.com/rest/apps/marketplace#list-subscriptions-for-the-authenticated-user-stubbed>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
     * Lists the active subscriptions for the authenticated user.
     *
     * FROM: <https://docs.github.com/rest/apps/marketplace#list-subscriptions-for-the-authenticated-user-stubbed>
     */
    pub async fn list_all_subscriptions_for_authenticated_user_stubbed(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::UserMarketplacePurchase>>> {
        let url = self
            .client
            .url(&"/user/marketplace_purchases/stubbed".to_string(), None);
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
     * FROM: <https://docs.github.com/rest/apps/apps#get-a-user-installation-for-the-authenticated-app>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn get_user_installation(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::Installation>> {
        let url = self.client.url(
            &format!(
                "/users/{}/installation",
                crate::progenitor_support::encode_path(&username.to_string()),
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
