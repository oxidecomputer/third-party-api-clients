use crate::Client;
use crate::ClientResult;

pub struct IdentityProviders {
    pub client: Client,
}

impl IdentityProviders {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        IdentityProviders { client }
    }

    /**
     * List Identity Providers.
     *
     * This function performs a `GET` to the `/api/v1/idps` endpoint.
     *
     * Enumerates IdPs in your organization with pagination. A subset of IdPs can be returned that match a supported filter expression or query.
     *
     * **Parameters:**
     *
     * * `q: &str` -- Searches the name property of IdPs for matching value.
     * * `after: &str` -- Specifies the pagination cursor for the next page of IdPs.
     * * `limit: i64` -- Specifies the number of IdP results in a page.
     * * `type_: &str` -- Filters IdPs by type.
     */
    pub async fn list(
        &self,
        q: &str,
        after: &str,
        limit: i64,
        type_: &str,
    ) -> ClientResult<Vec<crate::types::IdentityProvider>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/api/v1/idps?{}", query_), None);
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
     * List Identity Providers.
     *
     * This function performs a `GET` to the `/api/v1/idps` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Enumerates IdPs in your organization with pagination. A subset of IdPs can be returned that match a supported filter expression or query.
     */
    pub async fn list_all(
        &self,
        q: &str,
        type_: &str,
    ) -> ClientResult<Vec<crate::types::IdentityProvider>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/api/v1/idps?{}", query_), None);
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
     * Add Identity Provider.
     *
     * This function performs a `POST` to the `/api/v1/idps` endpoint.
     *
     * Adds a new IdP to your organization.
     */
    pub async fn create(
        &self,
        body: &crate::types::IdentityProvider,
    ) -> ClientResult<crate::types::IdentityProvider> {
        let url = self.client.url("/api/v1/idps", None);
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
     * List Keys.
     *
     * This function performs a `GET` to the `/api/v1/idps/credentials/keys` endpoint.
     *
     * Enumerates IdP key credentials.
     *
     * **Parameters:**
     *
     * * `after: &str` -- Specifies the pagination cursor for the next page of keys.
     * * `limit: i64` -- Specifies the number of key results in a page.
     */
    pub async fn list_keys(
        &self,
        after: &str,
        limit: i64,
    ) -> ClientResult<Vec<crate::types::JsonWebKey>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/api/v1/idps/credentials/keys?{}", query_), None);
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
     * List Keys.
     *
     * This function performs a `GET` to the `/api/v1/idps/credentials/keys` endpoint.
     *
     * As opposed to `list_keys`, this function returns all the pages of the request at once.
     *
     * Enumerates IdP key credentials.
     */
    pub async fn list_all_keys(&self) -> ClientResult<Vec<crate::types::JsonWebKey>> {
        let url = self.client.url("/api/v1/idps/credentials/keys", None);
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
     * Add X.509 Certificate Public Key.
     *
     * This function performs a `POST` to the `/api/v1/idps/credentials/keys` endpoint.
     *
     * Adds a new X.509 certificate credential to the IdP key store.
     */
    pub async fn create_key(
        &self,
        body: &crate::types::JsonWebKey,
    ) -> ClientResult<crate::types::JsonWebKey> {
        let url = self.client.url("/api/v1/idps/credentials/keys", None);
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
     * Get Key.
     *
     * This function performs a `GET` to the `/api/v1/idps/credentials/keys/{keyId}` endpoint.
     *
     * Gets a specific IdP Key Credential by `kid`
     *
     * **Parameters:**
     *
     * * `key_id: &str`
     */
    pub async fn get_key(&self, key_id: &str) -> ClientResult<crate::types::JsonWebKey> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/credentials/keys/{}",
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
     * Delete Key.
     *
     * This function performs a `DELETE` to the `/api/v1/idps/credentials/keys/{keyId}` endpoint.
     *
     * Deletes a specific IdP Key Credential by `kid` if it is not currently being used by an Active or Inactive IdP.
     *
     * **Parameters:**
     *
     * * `key_id: &str`
     */
    pub async fn delete_key(&self, key_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/credentials/keys/{}",
                crate::progenitor_support::encode_path(key_id),
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
     * Get Identity Provider.
     *
     * This function performs a `GET` to the `/api/v1/idps/{idpId}` endpoint.
     *
     * Fetches an IdP by `id`.
     *
     * **Parameters:**
     *
     * * `idp_id: &str`
     */
    pub async fn get(&self, idp_id: &str) -> ClientResult<crate::types::IdentityProvider> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}",
                crate::progenitor_support::encode_path(idp_id),
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
     * Update Identity Provider.
     *
     * This function performs a `PUT` to the `/api/v1/idps/{idpId}` endpoint.
     *
     * Updates the configuration for an IdP.
     *
     * **Parameters:**
     *
     * * `idp_id: &str`
     */
    pub async fn update(
        &self,
        idp_id: &str,
        body: &crate::types::IdentityProvider,
    ) -> ClientResult<crate::types::IdentityProvider> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}",
                crate::progenitor_support::encode_path(idp_id),
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
     * Delete Identity Provider.
     *
     * This function performs a `DELETE` to the `/api/v1/idps/{idpId}` endpoint.
     *
     * Removes an IdP from your organization.
     *
     * **Parameters:**
     *
     * * `idp_id: &str`
     */
    pub async fn delete(&self, idp_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}",
                crate::progenitor_support::encode_path(idp_id),
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
     * List Certificate Signing Requests for IdP.
     *
     * This function performs a `GET` to the `/api/v1/idps/{idpId}/credentials/csrs` endpoint.
     *
     * Enumerates Certificate Signing Requests for an IdP
     *
     * **Parameters:**
     *
     * * `idp_id: &str`
     */
    pub async fn list_csrs_fors(&self, idp_id: &str) -> ClientResult<Vec<crate::types::Csr>> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}/credentials/csrs",
                crate::progenitor_support::encode_path(idp_id),
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
     * List Certificate Signing Requests for IdP.
     *
     * This function performs a `GET` to the `/api/v1/idps/{idpId}/credentials/csrs` endpoint.
     *
     * As opposed to `list_csrs_for`, this function returns all the pages of the request at once.
     *
     * Enumerates Certificate Signing Requests for an IdP
     */
    pub async fn list_all_csrs_fors(&self, idp_id: &str) -> ClientResult<Vec<crate::types::Csr>> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}/credentials/csrs",
                crate::progenitor_support::encode_path(idp_id),
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
     * Generate Certificate Signing Request for IdP.
     *
     * This function performs a `POST` to the `/api/v1/idps/{idpId}/credentials/csrs` endpoint.
     *
     * Generates a new key pair and returns a Certificate Signing Request for it.
     *
     * **Parameters:**
     *
     * * `idp_id: &str`
     */
    pub async fn generate_csr_for(
        &self,
        idp_id: &str,
        body: &crate::types::CsrMetadata,
    ) -> ClientResult<crate::types::Csr> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}/credentials/csrs",
                crate::progenitor_support::encode_path(idp_id),
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
     * This function performs a `GET` to the `/api/v1/idps/{idpId}/credentials/csrs/{csrId}` endpoint.
     *
     * Gets a specific Certificate Signing Request model by id
     *
     * **Parameters:**
     *
     * * `idp_id: &str`
     * * `csr_id: &str`
     */
    pub async fn get_csr_for(&self, idp_id: &str, csr_id: &str) -> ClientResult<crate::types::Csr> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}/credentials/csrs/{}",
                crate::progenitor_support::encode_path(idp_id),
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
     * This function performs a `DELETE` to the `/api/v1/idps/{idpId}/credentials/csrs/{csrId}` endpoint.
     *
     * Revoke a Certificate Signing Request and delete the key pair from the IdP
     *
     * **Parameters:**
     *
     * * `idp_id: &str`
     * * `csr_id: &str`
     */
    pub async fn revoke_csr_for(&self, idp_id: &str, csr_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}/credentials/csrs/{}",
                crate::progenitor_support::encode_path(idp_id),
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
     * This function performs a `POST` to the `/api/v1/idps/{idpId}/credentials/csrs/{csrId}/lifecycle/publish` endpoint.
     *
     * Update the Certificate Signing Request with a signed X.509 certificate and add it into the signing key credentials for the IdP.
     *
     * **Parameters:**
     *
     * * `idp_id: &str`
     * * `csr_id: &str`
     */
    pub async fn post_idp_credentials_csr_lifecycle_publish(
        &self,
        idp_id: &str,
        csr_id: &str,
    ) -> ClientResult<crate::types::JsonWebKey> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}/credentials/csrs/{}/lifecycle/publish",
                crate::progenitor_support::encode_path(idp_id),
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
     * List Signing Key Credentials for IdP.
     *
     * This function performs a `GET` to the `/api/v1/idps/{idpId}/credentials/keys` endpoint.
     *
     * Enumerates signing key credentials for an IdP
     *
     * **Parameters:**
     *
     * * `idp_id: &str`
     */
    pub async fn list_signing_keys(
        &self,
        idp_id: &str,
    ) -> ClientResult<Vec<crate::types::JsonWebKey>> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}/credentials/keys",
                crate::progenitor_support::encode_path(idp_id),
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
     * List Signing Key Credentials for IdP.
     *
     * This function performs a `GET` to the `/api/v1/idps/{idpId}/credentials/keys` endpoint.
     *
     * As opposed to `list_signing_keys`, this function returns all the pages of the request at once.
     *
     * Enumerates signing key credentials for an IdP
     */
    pub async fn list_all_signing_keys(
        &self,
        idp_id: &str,
    ) -> ClientResult<Vec<crate::types::JsonWebKey>> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}/credentials/keys",
                crate::progenitor_support::encode_path(idp_id),
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
     * Generate New IdP Signing Key Credential.
     *
     * This function performs a `POST` to the `/api/v1/idps/{idpId}/credentials/keys/generate` endpoint.
     *
     * Generates a new X.509 certificate for an IdP signing key credential to be used for signing assertions sent to the IdP
     *
     * **Parameters:**
     *
     * * `idp_id: &str`
     * * `validity_years: i64` -- expiry of the IdP Key Credential.
     */
    pub async fn generate_signing_key(
        &self,
        idp_id: &str,
        validity_years: i64,
    ) -> ClientResult<crate::types::JsonWebKey> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if validity_years > 0 {
            query_args.push(("validityYears".to_string(), validity_years.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}/credentials/keys/generate?{}",
                crate::progenitor_support::encode_path(idp_id),
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
     * Get Signing Key Credential for IdP.
     *
     * This function performs a `GET` to the `/api/v1/idps/{idpId}/credentials/keys/{keyId}` endpoint.
     *
     * Gets a specific IdP Key Credential by `kid`
     *
     * **Parameters:**
     *
     * * `idp_id: &str`
     * * `key_id: &str`
     */
    pub async fn get_signing_key(
        &self,
        idp_id: &str,
        key_id: &str,
    ) -> ClientResult<crate::types::JsonWebKey> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}/credentials/keys/{}",
                crate::progenitor_support::encode_path(idp_id),
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
     * Clone Signing Key Credential for IdP.
     *
     * This function performs a `POST` to the `/api/v1/idps/{idpId}/credentials/keys/{keyId}/clone` endpoint.
     *
     * Clones a X.509 certificate for an IdP signing key credential from a source IdP to target IdP
     *
     * **Parameters:**
     *
     * * `idp_id: &str`
     * * `key_id: &str`
     * * `target_idp_id: &str`
     */
    pub async fn clone_key(
        &self,
        idp_id: &str,
        key_id: &str,
        target_idp_id: &str,
    ) -> ClientResult<crate::types::JsonWebKey> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !target_idp_id.is_empty() {
            query_args.push(("targetIdpId".to_string(), target_idp_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}/credentials/keys/{}/clone?{}",
                crate::progenitor_support::encode_path(idp_id),
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
     * Activate Identity Provider.
     *
     * This function performs a `POST` to the `/api/v1/idps/{idpId}/lifecycle/activate` endpoint.
     *
     * Activates an inactive IdP.
     *
     * **Parameters:**
     *
     * * `idp_id: &str`
     */
    pub async fn activate(&self, idp_id: &str) -> ClientResult<crate::types::IdentityProvider> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}/lifecycle/activate",
                crate::progenitor_support::encode_path(idp_id),
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
     * Deactivate Identity Provider.
     *
     * This function performs a `POST` to the `/api/v1/idps/{idpId}/lifecycle/deactivate` endpoint.
     *
     * Deactivates an active IdP.
     *
     * **Parameters:**
     *
     * * `idp_id: &str`
     */
    pub async fn deactivate(&self, idp_id: &str) -> ClientResult<crate::types::IdentityProvider> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}/lifecycle/deactivate",
                crate::progenitor_support::encode_path(idp_id),
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
     * Find Users.
     *
     * This function performs a `GET` to the `/api/v1/idps/{idpId}/users` endpoint.
     *
     * Find all the users linked to an identity provider
     *
     * **Parameters:**
     *
     * * `idp_id: &str`
     */
    pub async fn list_application_users(
        &self,
        idp_id: &str,
    ) -> ClientResult<Vec<crate::types::IdentityProviderApplicationUser>> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}/users",
                crate::progenitor_support::encode_path(idp_id),
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
     * Find Users.
     *
     * This function performs a `GET` to the `/api/v1/idps/{idpId}/users` endpoint.
     *
     * As opposed to `list_application_users`, this function returns all the pages of the request at once.
     *
     * Find all the users linked to an identity provider
     */
    pub async fn list_all_application_users(
        &self,
        idp_id: &str,
    ) -> ClientResult<Vec<crate::types::IdentityProviderApplicationUser>> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}/users",
                crate::progenitor_support::encode_path(idp_id),
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
     * This function performs a `GET` to the `/api/v1/idps/{idpId}/users/{userId}` endpoint.
     *
     * Fetches a linked IdP user by ID
     *
     * **Parameters:**
     *
     * * `idp_id: &str`
     * * `user_id: &str`
     */
    pub async fn get_application_user(
        &self,
        idp_id: &str,
        user_id: &str,
    ) -> ClientResult<crate::types::IdentityProviderApplicationUser> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}/users/{}",
                crate::progenitor_support::encode_path(idp_id),
                crate::progenitor_support::encode_path(user_id),
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
     * Link a user to a Social IdP without a transaction.
     *
     * This function performs a `POST` to the `/api/v1/idps/{idpId}/users/{userId}` endpoint.
     *
     * Links an Okta user to an existing Social Identity Provider. This does not support the SAML2 Identity Provider Type
     *
     * **Parameters:**
     *
     * * `idp_id: &str`
     * * `user_id: &str`
     */
    pub async fn link_user(
        &self,
        idp_id: &str,
        user_id: &str,
        body: &crate::types::UserIdentityProviderLinkRequest,
    ) -> ClientResult<crate::types::IdentityProviderApplicationUser> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}/users/{}",
                crate::progenitor_support::encode_path(idp_id),
                crate::progenitor_support::encode_path(user_id),
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
     * Unlink User from IdP.
     *
     * This function performs a `DELETE` to the `/api/v1/idps/{idpId}/users/{userId}` endpoint.
     *
     * Removes the link between the Okta user and the IdP user.
     *
     * **Parameters:**
     *
     * * `idp_id: &str`
     * * `user_id: &str`
     */
    pub async fn unlink_user_from(&self, idp_id: &str, user_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}/users/{}",
                crate::progenitor_support::encode_path(idp_id),
                crate::progenitor_support::encode_path(user_id),
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
     * Social Authentication Token Operation.
     *
     * This function performs a `GET` to the `/api/v1/idps/{idpId}/users/{userId}/credentials/tokens` endpoint.
     *
     * Fetches the tokens minted by the Social Authentication Provider when the user authenticates with Okta via Social Auth.
     *
     * **Parameters:**
     *
     * * `idp_id: &str`
     * * `user_id: &str`
     */
    pub async fn list_social_auth_tokens(
        &self,
        idp_id: &str,
        user_id: &str,
    ) -> ClientResult<Vec<crate::types::SocialAuthToken>> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}/users/{}/credentials/tokens",
                crate::progenitor_support::encode_path(idp_id),
                crate::progenitor_support::encode_path(user_id),
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
     * Social Authentication Token Operation.
     *
     * This function performs a `GET` to the `/api/v1/idps/{idpId}/users/{userId}/credentials/tokens` endpoint.
     *
     * As opposed to `list_social_auth_tokens`, this function returns all the pages of the request at once.
     *
     * Fetches the tokens minted by the Social Authentication Provider when the user authenticates with Okta via Social Auth.
     */
    pub async fn list_all_social_auth_tokens(
        &self,
        idp_id: &str,
        user_id: &str,
    ) -> ClientResult<Vec<crate::types::SocialAuthToken>> {
        let url = self.client.url(
            &format!(
                "/api/v1/idps/{}/users/{}/credentials/tokens",
                crate::progenitor_support::encode_path(idp_id),
                crate::progenitor_support::encode_path(user_id),
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
}
