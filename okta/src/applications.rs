use crate::Client;
use crate::ClientResult;

pub struct Applications {
    pub client: Client,
}

impl Applications {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Applications { client }
    }

    /**
     * List Applications.
     *
     * This function performs a `GET` to the `/api/v1/apps` endpoint.
     *
     * Enumerates apps added to your organization with pagination. A subset of apps can be returned that match a supported filter expression or query.
     *
     * **Parameters:**
     *
     * * `q: &str`
     * * `after: &str` -- Specifies the pagination cursor for the next page of apps.
     * * `limit: i64` -- Specifies the number of results for a page.
     * * `filter: &str` -- Filters apps by status, user.id, group.id or credentials.signing.kid expression.
     * * `expand: &str` -- Traverses users link relationship and optionally embeds Application User resource.
     * * `include_non_deleted: bool`
     */
    pub async fn list(
        &self,
        q: &str,
        after: &str,
        limit: i64,
        filter: &str,
        expand: &str,
        include_non_deleted: bool,
    ) -> ClientResult<Vec<crate::types::Application>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if include_non_deleted {
            query_args.push((
                "includeNonDeleted".to_string(),
                include_non_deleted.to_string(),
            ));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/api/v1/apps?{}", query_), None);
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
     * List Applications.
     *
     * This function performs a `GET` to the `/api/v1/apps` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Enumerates apps added to your organization with pagination. A subset of apps can be returned that match a supported filter expression or query.
     */
    pub async fn list_all(
        &self,
        q: &str,
        filter: &str,
        expand: &str,
        include_non_deleted: bool,
    ) -> ClientResult<Vec<crate::types::Application>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if include_non_deleted {
            query_args.push((
                "includeNonDeleted".to_string(),
                include_non_deleted.to_string(),
            ));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/api/v1/apps?{}", query_), None);
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
     * Add Application.
     *
     * This function performs a `POST` to the `/api/v1/apps` endpoint.
     *
     * Adds a new application to your Okta organization.
     *
     * **Parameters:**
     *
     * * `activate: bool` -- Executes activation lifecycle operation when creating the app.
     * * `okta_access_gateway_agent: &str`
     */
    pub async fn create(
        &self,
        activate: bool,
        body: &crate::types::Application,
    ) -> ClientResult<crate::types::Application> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if activate {
            query_args.push(("activate".to_string(), activate.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/api/v1/apps?{}", query_), None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get Application.
     *
     * This function performs a `GET` to the `/api/v1/apps/{appId}` endpoint.
     *
     * Fetches an application from your Okta organization by `id`.
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     * * `expand: &str`
     */
    pub async fn get(&self, app_id: &str, expand: &str) -> ClientResult<crate::types::Application> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}?{}",
                crate::progenitor_support::encode_path(app_id),
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
     * Update Application.
     *
     * This function performs a `PUT` to the `/api/v1/apps/{appId}` endpoint.
     *
     * Updates an application in your organization.
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     */
    pub async fn update(
        &self,
        app_id: &str,
        body: &crate::types::Application,
    ) -> ClientResult<crate::types::Application> {
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}",
                crate::progenitor_support::encode_path(app_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Delete Application.
     *
     * This function performs a `DELETE` to the `/api/v1/apps/{appId}` endpoint.
     *
     * Removes an inactive application.
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     */
    pub async fn delete(&self, app_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}",
                crate::progenitor_support::encode_path(app_id),
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
     * List Certificate Signing Requests for Application.
     *
     * This function performs a `GET` to the `/api/v1/apps/{appId}/credentials/csrs` endpoint.
     *
     * Enumerates Certificate Signing Requests for an application
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     */
    pub async fn list_csrs_fors(&self, app_id: &str) -> ClientResult<Vec<crate::types::Csr>> {
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/credentials/csrs",
                crate::progenitor_support::encode_path(app_id),
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
     * List Certificate Signing Requests for Application.
     *
     * This function performs a `GET` to the `/api/v1/apps/{appId}/credentials/csrs` endpoint.
     *
     * As opposed to `list_csrs_for`, this function returns all the pages of the request at once.
     *
     * Enumerates Certificate Signing Requests for an application
     */
    pub async fn list_all_csrs_fors(&self, app_id: &str) -> ClientResult<Vec<crate::types::Csr>> {
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/credentials/csrs",
                crate::progenitor_support::encode_path(app_id),
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
     * Generate Certificate Signing Request for Application.
     *
     * This function performs a `POST` to the `/api/v1/apps/{appId}/credentials/csrs` endpoint.
     *
     * Generates a new key pair and returns the Certificate Signing Request for it.
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     */
    pub async fn generate_csr_for(
        &self,
        app_id: &str,
        body: &crate::types::CsrMetadata,
    ) -> ClientResult<crate::types::Csr> {
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/credentials/csrs",
                crate::progenitor_support::encode_path(app_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/api/v1/apps/{appId}/credentials/csrs/{csrId}` endpoint.
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     * * `csr_id: &str`
     */
    pub async fn get_csr_for(&self, app_id: &str, csr_id: &str) -> ClientResult<crate::types::Csr> {
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/credentials/csrs/{}",
                crate::progenitor_support::encode_path(app_id),
                crate::progenitor_support::encode_path(csr_id),
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
     * This function performs a `DELETE` to the `/api/v1/apps/{appId}/credentials/csrs/{csrId}` endpoint.
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     * * `csr_id: &str`
     */
    pub async fn revoke_csr_from(&self, app_id: &str, csr_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/credentials/csrs/{}",
                crate::progenitor_support::encode_path(app_id),
                crate::progenitor_support::encode_path(csr_id),
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
     * This function performs a `POST` to the `/api/v1/apps/{appId}/credentials/csrs/{csrId}/lifecycle/publish` endpoint.
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     * * `csr_id: &str`
     */
    pub async fn post_app_credentials_csr_lifecycle_publish(
        &self,
        app_id: &str,
        csr_id: &str,
    ) -> ClientResult<crate::types::JsonWebKey> {
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/credentials/csrs/{}/lifecycle/publish",
                crate::progenitor_support::encode_path(app_id),
                crate::progenitor_support::encode_path(csr_id),
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
     * List Key Credentials for Application.
     *
     * This function performs a `GET` to the `/api/v1/apps/{appId}/credentials/keys` endpoint.
     *
     * Enumerates key credentials for an application
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     */
    pub async fn list_keys(&self, app_id: &str) -> ClientResult<Vec<crate::types::JsonWebKey>> {
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/credentials/keys",
                crate::progenitor_support::encode_path(app_id),
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
     * List Key Credentials for Application.
     *
     * This function performs a `GET` to the `/api/v1/apps/{appId}/credentials/keys` endpoint.
     *
     * As opposed to `list_keys`, this function returns all the pages of the request at once.
     *
     * Enumerates key credentials for an application
     */
    pub async fn list_all_keys(&self, app_id: &str) -> ClientResult<Vec<crate::types::JsonWebKey>> {
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/credentials/keys",
                crate::progenitor_support::encode_path(app_id),
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
     * This function performs a `POST` to the `/api/v1/apps/{appId}/credentials/keys/generate` endpoint.
     *
     * Generates a new X.509 certificate for an application key credential
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     * * `validity_years: i64`
     */
    pub async fn generate_key(
        &self,
        app_id: &str,
        validity_years: i64,
    ) -> ClientResult<crate::types::JsonWebKey> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if validity_years > 0 {
            query_args.push(("validityYears".to_string(), validity_years.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/credentials/keys/generate?{}",
                crate::progenitor_support::encode_path(app_id),
                query_
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
     * Get Key Credential for Application.
     *
     * This function performs a `GET` to the `/api/v1/apps/{appId}/credentials/keys/{keyId}` endpoint.
     *
     * Gets a specific application key credential by kid
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     * * `key_id: &str`
     */
    pub async fn get_key(
        &self,
        app_id: &str,
        key_id: &str,
    ) -> ClientResult<crate::types::JsonWebKey> {
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/credentials/keys/{}",
                crate::progenitor_support::encode_path(app_id),
                crate::progenitor_support::encode_path(key_id),
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
     * Clone Application Key Credential.
     *
     * This function performs a `POST` to the `/api/v1/apps/{appId}/credentials/keys/{keyId}/clone` endpoint.
     *
     * Clones a X.509 certificate for an application key credential from a source application to target application.
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     * * `key_id: &str`
     * * `target_aid: &str` -- Unique key of the target Application.
     */
    pub async fn clone_key(
        &self,
        app_id: &str,
        key_id: &str,
        target_aid: &str,
    ) -> ClientResult<crate::types::JsonWebKey> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !target_aid.is_empty() {
            query_args.push(("targetAid".to_string(), target_aid.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/credentials/keys/{}/clone?{}",
                crate::progenitor_support::encode_path(app_id),
                crate::progenitor_support::encode_path(key_id),
                query_
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
     * This function performs a `GET` to the `/api/v1/apps/{appId}/grants` endpoint.
     *
     * Lists all scope consent grants for the application
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     * * `expand: &str`
     */
    pub async fn list_scope_consent_grants(
        &self,
        app_id: &str,
        expand: &str,
    ) -> ClientResult<Vec<crate::types::OAuth2ScopeConsentGrant>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/grants?{}",
                crate::progenitor_support::encode_path(app_id),
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
     * This function performs a `GET` to the `/api/v1/apps/{appId}/grants` endpoint.
     *
     * As opposed to `list_scope_consent_grants`, this function returns all the pages of the request at once.
     *
     * Lists all scope consent grants for the application
     */
    pub async fn list_all_scope_consent_grants(
        &self,
        app_id: &str,
        expand: &str,
    ) -> ClientResult<Vec<crate::types::OAuth2ScopeConsentGrant>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/grants?{}",
                crate::progenitor_support::encode_path(app_id),
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
     * This function performs a `POST` to the `/api/v1/apps/{appId}/grants` endpoint.
     *
     * Grants consent for the application to request an OAuth 2.0 Okta scope
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     */
    pub async fn grant_consent_scope(
        &self,
        app_id: &str,
        body: &crate::types::OAuth2ScopeConsentGrant,
    ) -> ClientResult<crate::types::OAuth2ScopeConsentGrant> {
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/grants",
                crate::progenitor_support::encode_path(app_id),
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
     * This function performs a `GET` to the `/api/v1/apps/{appId}/grants/{grantId}` endpoint.
     *
     * Fetches a single scope consent grant for the application
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     * * `grant_id: &str`
     * * `expand: &str`
     */
    pub async fn get_scope_consent_grant(
        &self,
        app_id: &str,
        grant_id: &str,
        expand: &str,
    ) -> ClientResult<crate::types::OAuth2ScopeConsentGrant> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/grants/{}?{}",
                crate::progenitor_support::encode_path(app_id),
                crate::progenitor_support::encode_path(grant_id),
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
     * This function performs a `DELETE` to the `/api/v1/apps/{appId}/grants/{grantId}` endpoint.
     *
     * Revokes permission for the application to request the given scope
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     * * `grant_id: &str`
     */
    pub async fn revoke_scope_consent_grant(
        &self,
        app_id: &str,
        grant_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/grants/{}",
                crate::progenitor_support::encode_path(app_id),
                crate::progenitor_support::encode_path(grant_id),
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
     * List Groups Assigned to Application.
     *
     * This function performs a `GET` to the `/api/v1/apps/{appId}/groups` endpoint.
     *
     * Enumerates group assignments for an application.
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     * * `q: &str`
     * * `after: &str` -- Specifies the pagination cursor for the next page of assignments.
     * * `limit: i64` -- Specifies the number of results for a page.
     * * `expand: &str`
     */
    pub async fn list_group_assignments(
        &self,
        app_id: &str,
        q: &str,
        after: &str,
        limit: i64,
        expand: &str,
    ) -> ClientResult<Vec<crate::types::ApplicationGroupAssignment>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/groups?{}",
                crate::progenitor_support::encode_path(app_id),
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
     * List Groups Assigned to Application.
     *
     * This function performs a `GET` to the `/api/v1/apps/{appId}/groups` endpoint.
     *
     * As opposed to `list_group_assignments`, this function returns all the pages of the request at once.
     *
     * Enumerates group assignments for an application.
     */
    pub async fn list_all_group_assignments(
        &self,
        app_id: &str,
        q: &str,
        expand: &str,
    ) -> ClientResult<Vec<crate::types::ApplicationGroupAssignment>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/groups?{}",
                crate::progenitor_support::encode_path(app_id),
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
     * Get Assigned Group for Application.
     *
     * This function performs a `GET` to the `/api/v1/apps/{appId}/groups/{groupId}` endpoint.
     *
     * Fetches an application group assignment
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     * * `group_id: &str`
     * * `expand: &str`
     */
    pub async fn get_group_assignment(
        &self,
        app_id: &str,
        group_id: &str,
        expand: &str,
    ) -> ClientResult<crate::types::ApplicationGroupAssignment> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/groups/{}?{}",
                crate::progenitor_support::encode_path(app_id),
                crate::progenitor_support::encode_path(group_id),
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
     * Assign Group to Application.
     *
     * This function performs a `PUT` to the `/api/v1/apps/{appId}/groups/{groupId}` endpoint.
     *
     * Assigns a group to an application
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     * * `group_id: &str`
     */
    pub async fn create_group_assignment(
        &self,
        app_id: &str,
        group_id: &str,
        body: &crate::types::ApplicationGroupAssignment,
    ) -> ClientResult<crate::types::ApplicationGroupAssignment> {
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/groups/{}",
                crate::progenitor_support::encode_path(app_id),
                crate::progenitor_support::encode_path(group_id),
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
     * Remove Group from Application.
     *
     * This function performs a `DELETE` to the `/api/v1/apps/{appId}/groups/{groupId}` endpoint.
     *
     * Removes a group assignment from an application.
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     * * `group_id: &str`
     */
    pub async fn delete_group_assignment(&self, app_id: &str, group_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/groups/{}",
                crate::progenitor_support::encode_path(app_id),
                crate::progenitor_support::encode_path(group_id),
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
     * Activate Application.
     *
     * This function performs a `POST` to the `/api/v1/apps/{appId}/lifecycle/activate` endpoint.
     *
     * Activates an inactive application.
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     */
    pub async fn activate(&self, app_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/lifecycle/activate",
                crate::progenitor_support::encode_path(app_id),
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
     * Deactivate Application.
     *
     * This function performs a `POST` to the `/api/v1/apps/{appId}/lifecycle/deactivate` endpoint.
     *
     * Deactivates an active application.
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     */
    pub async fn deactivate(&self, app_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/lifecycle/deactivate",
                crate::progenitor_support::encode_path(app_id),
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
     * This function performs a `GET` to the `/api/v1/apps/{appId}/tokens` endpoint.
     *
     * Lists all tokens for the application
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     * * `expand: &str`
     * * `after: &str`
     * * `limit: i64`
     */
    pub async fn list_o_auth_2_tokens_fors(
        &self,
        app_id: &str,
        expand: &str,
        after: &str,
        limit: i64,
    ) -> ClientResult<Vec<crate::types::OAuth2Token>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/tokens?{}",
                crate::progenitor_support::encode_path(app_id),
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
     * This function performs a `GET` to the `/api/v1/apps/{appId}/tokens` endpoint.
     *
     * As opposed to `list_o_auth_2_tokens_for`, this function returns all the pages of the request at once.
     *
     * Lists all tokens for the application
     */
    pub async fn list_all_o_auth_2_tokens_fors(
        &self,
        app_id: &str,
        expand: &str,
    ) -> ClientResult<Vec<crate::types::OAuth2Token>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/tokens?{}",
                crate::progenitor_support::encode_path(app_id),
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
     * This function performs a `DELETE` to the `/api/v1/apps/{appId}/tokens` endpoint.
     *
     * Revokes all tokens for the specified application
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     */
    pub async fn revoke_o_auth_2_tokens_for(&self, app_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/tokens",
                crate::progenitor_support::encode_path(app_id),
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
     * This function performs a `GET` to the `/api/v1/apps/{appId}/tokens/{tokenId}` endpoint.
     *
     * Gets a token for the specified application
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     * * `token_id: &str`
     * * `expand: &str`
     */
    pub async fn get_o_auth_2_token_for(
        &self,
        app_id: &str,
        token_id: &str,
        expand: &str,
    ) -> ClientResult<crate::types::OAuth2Token> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/tokens/{}?{}",
                crate::progenitor_support::encode_path(app_id),
                crate::progenitor_support::encode_path(token_id),
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
     * This function performs a `DELETE` to the `/api/v1/apps/{appId}/tokens/{tokenId}` endpoint.
     *
     * Revokes the specified token for the specified application
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     * * `token_id: &str`
     */
    pub async fn revoke_o_auth_2_token_for(
        &self,
        app_id: &str,
        token_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/tokens/{}",
                crate::progenitor_support::encode_path(app_id),
                crate::progenitor_support::encode_path(token_id),
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
     * List Users Assigned to Application.
     *
     * This function performs a `GET` to the `/api/v1/apps/{appId}/users` endpoint.
     *
     * Enumerates all assigned [application users](#application-user-model) for an application.
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     * * `q: &str`
     * * `query_scope: &str`
     * * `after: &str` -- specifies the pagination cursor for the next page of assignments.
     * * `limit: i64` -- specifies the number of results for a page.
     * * `filter: &str`
     * * `expand: &str`
     */
    pub async fn list_users(
        &self,
        app_id: &str,
        q: &str,
        query_scope: &str,
        after: &str,
        limit: i64,
        filter: &str,
        expand: &str,
    ) -> ClientResult<Vec<crate::types::AppUser>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if !query_scope.is_empty() {
            query_args.push(("query_scope".to_string(), query_scope.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/users?{}",
                crate::progenitor_support::encode_path(app_id),
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
     * List Users Assigned to Application.
     *
     * This function performs a `GET` to the `/api/v1/apps/{appId}/users` endpoint.
     *
     * As opposed to `list_users`, this function returns all the pages of the request at once.
     *
     * Enumerates all assigned [application users](#application-user-model) for an application.
     */
    pub async fn list_all_users(
        &self,
        app_id: &str,
        q: &str,
        query_scope: &str,
        filter: &str,
        expand: &str,
    ) -> ClientResult<Vec<crate::types::AppUser>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if !query_scope.is_empty() {
            query_args.push(("query_scope".to_string(), query_scope.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/users?{}",
                crate::progenitor_support::encode_path(app_id),
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
     * Assign User to Application for SSO & Provisioning.
     *
     * This function performs a `POST` to the `/api/v1/apps/{appId}/users` endpoint.
     *
     * Assigns an user to an application with [credentials](#application-user-credentials-object) and an app-specific [profile](#application-user-profile-object). Profile mappings defined for the application are first applied before applying any profile properties specified in the request.
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     */
    pub async fn assign_user(
        &self,
        app_id: &str,
        body: &crate::types::AppUser,
    ) -> ClientResult<crate::types::AppUser> {
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/users",
                crate::progenitor_support::encode_path(app_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get Assigned User for Application.
     *
     * This function performs a `GET` to the `/api/v1/apps/{appId}/users/{userId}` endpoint.
     *
     * Fetches a specific user assignment for application by `id`.
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     * * `user_id: &str`
     * * `expand: &str`
     */
    pub async fn get_user(
        &self,
        app_id: &str,
        user_id: &str,
        expand: &str,
    ) -> ClientResult<crate::types::AppUser> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/users/{}?{}",
                crate::progenitor_support::encode_path(app_id),
                crate::progenitor_support::encode_path(user_id),
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
     * Update Application Profile for Assigned User.
     *
     * This function performs a `POST` to the `/api/v1/apps/{appId}/users/{userId}` endpoint.
     *
     * Updates a user's profile for an application
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     * * `user_id: &str`
     */
    pub async fn update_user(
        &self,
        app_id: &str,
        user_id: &str,
        body: &crate::types::AppUser,
    ) -> ClientResult<crate::types::AppUser> {
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/users/{}",
                crate::progenitor_support::encode_path(app_id),
                crate::progenitor_support::encode_path(user_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Remove User from Application.
     *
     * This function performs a `DELETE` to the `/api/v1/apps/{appId}/users/{userId}` endpoint.
     *
     * Removes an assignment for a user from an application.
     *
     * **Parameters:**
     *
     * * `app_id: &str`
     * * `user_id: &str`
     * * `send_email: bool`
     */
    pub async fn delete_user(
        &self,
        app_id: &str,
        user_id: &str,
        send_email: bool,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if send_email {
            query_args.push(("sendEmail".to_string(), send_email.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/apps/{}/users/{}?{}",
                crate::progenitor_support::encode_path(app_id),
                crate::progenitor_support::encode_path(user_id),
                query_
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
}
