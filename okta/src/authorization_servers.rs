use crate::Client;
use crate::ClientResult;

pub struct AuthorizationServers {
    pub client: Client,
}

impl AuthorizationServers {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AuthorizationServers { client }
    }

    /**
     * This function performs a `GET` to the `/api/v1/authorizationServers` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `q: &str`
     * * `limit: &str`
     * * `after: &str`
     */
    pub async fn list(
        &self,
        q: &str,
        limit: &str,
        after: &str,
    ) -> ClientResult<Vec<crate::types::AuthorizationServer>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/api/v1/authorizationServers?{}", query_), None);
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
     * This function performs a `GET` to the `/api/v1/authorizationServers` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Success
     */
    pub async fn list_all(&self, q: &str) -> ClientResult<Vec<crate::types::AuthorizationServer>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/api/v1/authorizationServers?{}", query_), None);
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
     * This function performs a `POST` to the `/api/v1/authorizationServers` endpoint.
     *
     * Success
     */
    pub async fn create(
        &self,
        body: &crate::types::AuthorizationServer,
    ) -> ClientResult<crate::types::AuthorizationServer> {
        let url = self.client.url("/api/v1/authorizationServers", None);
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
     * This function performs a `GET` to the `/api/v1/authorizationServers/{authServerId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     */
    pub async fn get(
        &self,
        auth_server_id: &str,
    ) -> ClientResult<crate::types::AuthorizationServer> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}",
                crate::progenitor_support::encode_path(auth_server_id),
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
     * This function performs a `PUT` to the `/api/v1/authorizationServers/{authServerId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     */
    pub async fn update(
        &self,
        auth_server_id: &str,
        body: &crate::types::AuthorizationServer,
    ) -> ClientResult<crate::types::AuthorizationServer> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}",
                crate::progenitor_support::encode_path(auth_server_id),
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
     * This function performs a `DELETE` to the `/api/v1/authorizationServers/{authServerId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     */
    pub async fn delete(&self, auth_server_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}",
                crate::progenitor_support::encode_path(auth_server_id),
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
     * This function performs a `GET` to the `/api/v1/authorizationServers/{authServerId}/claims` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     */
    pub async fn list_o_auth_2_claims(
        &self,
        auth_server_id: &str,
    ) -> ClientResult<Vec<crate::types::OAuth2Claim>> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/claims",
                crate::progenitor_support::encode_path(auth_server_id),
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
     * This function performs a `GET` to the `/api/v1/authorizationServers/{authServerId}/claims` endpoint.
     *
     * As opposed to `list_o_auth_2_claims`, this function returns all the pages of the request at once.
     *
     * Success
     */
    pub async fn list_all_o_auth_2_claims(
        &self,
        auth_server_id: &str,
    ) -> ClientResult<Vec<crate::types::OAuth2Claim>> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/claims",
                crate::progenitor_support::encode_path(auth_server_id),
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
     * This function performs a `POST` to the `/api/v1/authorizationServers/{authServerId}/claims` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     */
    pub async fn create_o_auth_2_claim(
        &self,
        auth_server_id: &str,
        body: &crate::types::OAuth2Claim,
    ) -> ClientResult<crate::types::OAuth2Claim> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/claims",
                crate::progenitor_support::encode_path(auth_server_id),
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
     * This function performs a `GET` to the `/api/v1/authorizationServers/{authServerId}/claims/{claimId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     * * `claim_id: &str`
     */
    pub async fn get_o_auth_2_claim(
        &self,
        auth_server_id: &str,
        claim_id: &str,
    ) -> ClientResult<crate::types::OAuth2Claim> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/claims/{}",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(claim_id),
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
     * This function performs a `PUT` to the `/api/v1/authorizationServers/{authServerId}/claims/{claimId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     * * `claim_id: &str`
     */
    pub async fn update_o_auth_2_claim(
        &self,
        auth_server_id: &str,
        claim_id: &str,
        body: &crate::types::OAuth2Claim,
    ) -> ClientResult<crate::types::OAuth2Claim> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/claims/{}",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(claim_id),
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
     * This function performs a `DELETE` to the `/api/v1/authorizationServers/{authServerId}/claims/{claimId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     * * `claim_id: &str`
     */
    pub async fn delete_o_auth_2_claim(
        &self,
        auth_server_id: &str,
        claim_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/claims/{}",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(claim_id),
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
     * This function performs a `GET` to the `/api/v1/authorizationServers/{authServerId}/clients` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     */
    pub async fn list_o_auth_2_clients_fors(
        &self,
        auth_server_id: &str,
    ) -> ClientResult<Vec<crate::types::OAuth2Client>> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/clients",
                crate::progenitor_support::encode_path(auth_server_id),
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
     * This function performs a `GET` to the `/api/v1/authorizationServers/{authServerId}/clients` endpoint.
     *
     * As opposed to `list_o_auth_2_clients_for`, this function returns all the pages of the request at once.
     *
     * Success
     */
    pub async fn list_all_o_auth_2_clients_fors(
        &self,
        auth_server_id: &str,
    ) -> ClientResult<Vec<crate::types::OAuth2Client>> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/clients",
                crate::progenitor_support::encode_path(auth_server_id),
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
     * This function performs a `GET` to the `/api/v1/authorizationServers/{authServerId}/clients/{clientId}/tokens` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     * * `client_id: &str`
     * * `expand: &str`
     * * `after: &str`
     * * `limit: i64`
     */
    pub async fn list_refresh_tokens_for_and_clients(
        &self,
        auth_server_id: &str,
        client_id: &str,
        expand: &str,
        after: &str,
        limit: i64,
    ) -> ClientResult<Vec<crate::types::OAuth2RefreshToken>> {
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
                "/api/v1/authorizationServers/{}/clients/{}/tokens?{}",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(client_id),
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
     * This function performs a `GET` to the `/api/v1/authorizationServers/{authServerId}/clients/{clientId}/tokens` endpoint.
     *
     * As opposed to `list_refresh_tokens_for_and_client`, this function returns all the pages of the request at once.
     *
     * Success
     */
    pub async fn list_all_refresh_tokens_for_and_clients(
        &self,
        auth_server_id: &str,
        client_id: &str,
        expand: &str,
    ) -> ClientResult<Vec<crate::types::OAuth2RefreshToken>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/clients/{}/tokens?{}",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(client_id),
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
     * This function performs a `DELETE` to the `/api/v1/authorizationServers/{authServerId}/clients/{clientId}/tokens` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     * * `client_id: &str`
     */
    pub async fn revoke_refresh_tokens_for_and_client(
        &self,
        auth_server_id: &str,
        client_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/clients/{}/tokens",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(client_id),
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
     * This function performs a `GET` to the `/api/v1/authorizationServers/{authServerId}/clients/{clientId}/tokens/{tokenId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     * * `client_id: &str`
     * * `token_id: &str`
     * * `expand: &str`
     */
    pub async fn get_refresh_token_for_and_client(
        &self,
        auth_server_id: &str,
        client_id: &str,
        token_id: &str,
        expand: &str,
    ) -> ClientResult<crate::types::OAuth2RefreshToken> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/clients/{}/tokens/{}?{}",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(client_id),
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
     * This function performs a `DELETE` to the `/api/v1/authorizationServers/{authServerId}/clients/{clientId}/tokens/{tokenId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     * * `client_id: &str`
     * * `token_id: &str`
     */
    pub async fn revoke_refresh_token_for_and_client(
        &self,
        auth_server_id: &str,
        client_id: &str,
        token_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/clients/{}/tokens/{}",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(client_id),
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
     * This function performs a `GET` to the `/api/v1/authorizationServers/{authServerId}/credentials/keys` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     */
    pub async fn list_keys(
        &self,
        auth_server_id: &str,
    ) -> ClientResult<Vec<crate::types::JsonWebKey>> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/credentials/keys",
                crate::progenitor_support::encode_path(auth_server_id),
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
     * This function performs a `GET` to the `/api/v1/authorizationServers/{authServerId}/credentials/keys` endpoint.
     *
     * As opposed to `list_keys`, this function returns all the pages of the request at once.
     *
     * Success
     */
    pub async fn list_all_keys(
        &self,
        auth_server_id: &str,
    ) -> ClientResult<Vec<crate::types::JsonWebKey>> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/credentials/keys",
                crate::progenitor_support::encode_path(auth_server_id),
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
     * This function performs a `POST` to the `/api/v1/authorizationServers/{authServerId}/credentials/lifecycle/keyRotate` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     */
    pub async fn rotate_keys(
        &self,
        auth_server_id: &str,
        body: &crate::types::JwkUse,
    ) -> ClientResult<Vec<crate::types::JsonWebKey>> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/credentials/lifecycle/keyRotate",
                crate::progenitor_support::encode_path(auth_server_id),
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
     * This function performs a `POST` to the `/api/v1/authorizationServers/{authServerId}/lifecycle/activate` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     */
    pub async fn activate(&self, auth_server_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/lifecycle/activate",
                crate::progenitor_support::encode_path(auth_server_id),
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
     * This function performs a `POST` to the `/api/v1/authorizationServers/{authServerId}/lifecycle/deactivate` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     */
    pub async fn deactivate(&self, auth_server_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/lifecycle/deactivate",
                crate::progenitor_support::encode_path(auth_server_id),
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
     * This function performs a `GET` to the `/api/v1/authorizationServers/{authServerId}/policies` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     */
    pub async fn list_policies(
        &self,
        auth_server_id: &str,
    ) -> ClientResult<Vec<crate::types::AuthorizationServerPolicy>> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/policies",
                crate::progenitor_support::encode_path(auth_server_id),
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
     * This function performs a `GET` to the `/api/v1/authorizationServers/{authServerId}/policies` endpoint.
     *
     * As opposed to `list_policies`, this function returns all the pages of the request at once.
     *
     * Success
     */
    pub async fn list_all_policies(
        &self,
        auth_server_id: &str,
    ) -> ClientResult<Vec<crate::types::AuthorizationServerPolicy>> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/policies",
                crate::progenitor_support::encode_path(auth_server_id),
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
     * This function performs a `POST` to the `/api/v1/authorizationServers/{authServerId}/policies` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     */
    pub async fn create_policy(
        &self,
        auth_server_id: &str,
        body: &crate::types::AuthorizationServerPolicy,
    ) -> ClientResult<crate::types::AuthorizationServerPolicy> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/policies",
                crate::progenitor_support::encode_path(auth_server_id),
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
     * This function performs a `GET` to the `/api/v1/authorizationServers/{authServerId}/policies/{policyId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     * * `policy_id: &str`
     */
    pub async fn get_policy(
        &self,
        auth_server_id: &str,
        policy_id: &str,
    ) -> ClientResult<crate::types::AuthorizationServerPolicy> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/policies/{}",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(policy_id),
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
     * This function performs a `PUT` to the `/api/v1/authorizationServers/{authServerId}/policies/{policyId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     * * `policy_id: &str`
     */
    pub async fn update_policy(
        &self,
        auth_server_id: &str,
        policy_id: &str,
        body: &crate::types::AuthorizationServerPolicy,
    ) -> ClientResult<crate::types::AuthorizationServerPolicy> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/policies/{}",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(policy_id),
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
     * This function performs a `DELETE` to the `/api/v1/authorizationServers/{authServerId}/policies/{policyId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     * * `policy_id: &str`
     */
    pub async fn delete_policy(&self, auth_server_id: &str, policy_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/policies/{}",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(policy_id),
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
     * This function performs a `POST` to the `/api/v1/authorizationServers/{authServerId}/policies/{policyId}/lifecycle/activate` endpoint.
     *
     * Activate Authorization Server Policy
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     * * `policy_id: &str`
     */
    pub async fn activate_policy(&self, auth_server_id: &str, policy_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/policies/{}/lifecycle/activate",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(policy_id),
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
     * This function performs a `POST` to the `/api/v1/authorizationServers/{authServerId}/policies/{policyId}/lifecycle/deactivate` endpoint.
     *
     * Deactivate Authorization Server Policy
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     * * `policy_id: &str`
     */
    pub async fn deactivate_policy(
        &self,
        auth_server_id: &str,
        policy_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/policies/{}/lifecycle/deactivate",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(policy_id),
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
     * This function performs a `GET` to the `/api/v1/authorizationServers/{authServerId}/policies/{policyId}/rules` endpoint.
     *
     * Enumerates all policy rules for the specified Custom Authorization Server and Policy.
     *
     * **Parameters:**
     *
     * * `policy_id: &str`
     * * `auth_server_id: &str`
     */
    pub async fn list_policy_rules(
        &self,
        policy_id: &str,
        auth_server_id: &str,
    ) -> ClientResult<Vec<crate::types::AuthorizationServerPolicyRule>> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/policies/{}/rules",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(policy_id),
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
     * This function performs a `GET` to the `/api/v1/authorizationServers/{authServerId}/policies/{policyId}/rules` endpoint.
     *
     * As opposed to `list_policy_rules`, this function returns all the pages of the request at once.
     *
     * Enumerates all policy rules for the specified Custom Authorization Server and Policy.
     */
    pub async fn list_all_policy_rules(
        &self,
        policy_id: &str,
        auth_server_id: &str,
    ) -> ClientResult<Vec<crate::types::AuthorizationServerPolicyRule>> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/policies/{}/rules",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(policy_id),
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
     * This function performs a `POST` to the `/api/v1/authorizationServers/{authServerId}/policies/{policyId}/rules` endpoint.
     *
     * Creates a policy rule for the specified Custom Authorization Server and Policy.
     *
     * **Parameters:**
     *
     * * `policy_id: &str`
     * * `auth_server_id: &str`
     */
    pub async fn create_policy_rule(
        &self,
        policy_id: &str,
        auth_server_id: &str,
        body: &crate::types::AuthorizationServerPolicyRule,
    ) -> ClientResult<crate::types::AuthorizationServerPolicyRule> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/policies/{}/rules",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(policy_id),
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
     * This function performs a `GET` to the `/api/v1/authorizationServers/{authServerId}/policies/{policyId}/rules/{ruleId}` endpoint.
     *
     * Returns a Policy Rule by ID that is defined in the specified Custom Authorization Server and Policy.
     *
     * **Parameters:**
     *
     * * `policy_id: &str`
     * * `auth_server_id: &str`
     * * `rule_id: &str`
     */
    pub async fn get_policy_rule(
        &self,
        policy_id: &str,
        auth_server_id: &str,
        rule_id: &str,
    ) -> ClientResult<crate::types::AuthorizationServerPolicyRule> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/policies/{}/rules/{}",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(policy_id),
                crate::progenitor_support::encode_path(rule_id),
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
     * This function performs a `PUT` to the `/api/v1/authorizationServers/{authServerId}/policies/{policyId}/rules/{ruleId}` endpoint.
     *
     * Updates the configuration of the Policy Rule defined in the specified Custom Authorization Server and Policy.
     *
     * **Parameters:**
     *
     * * `policy_id: &str`
     * * `auth_server_id: &str`
     * * `rule_id: &str`
     */
    pub async fn update_policy_rule(
        &self,
        policy_id: &str,
        auth_server_id: &str,
        rule_id: &str,
        body: &crate::types::AuthorizationServerPolicyRule,
    ) -> ClientResult<crate::types::AuthorizationServerPolicyRule> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/policies/{}/rules/{}",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(policy_id),
                crate::progenitor_support::encode_path(rule_id),
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
     * This function performs a `DELETE` to the `/api/v1/authorizationServers/{authServerId}/policies/{policyId}/rules/{ruleId}` endpoint.
     *
     * Deletes a Policy Rule defined in the specified Custom Authorization Server and Policy.
     *
     * **Parameters:**
     *
     * * `policy_id: &str`
     * * `auth_server_id: &str`
     * * `rule_id: &str`
     */
    pub async fn delete_policy_rule(
        &self,
        policy_id: &str,
        auth_server_id: &str,
        rule_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/policies/{}/rules/{}",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(policy_id),
                crate::progenitor_support::encode_path(rule_id),
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
     * This function performs a `POST` to the `/api/v1/authorizationServers/{authServerId}/policies/{policyId}/rules/{ruleId}/lifecycle/activate` endpoint.
     *
     * Activate Authorization Server Policy Rule
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     * * `policy_id: &str`
     * * `rule_id: &str`
     */
    pub async fn activate_policy_rule(
        &self,
        auth_server_id: &str,
        policy_id: &str,
        rule_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/policies/{}/rules/{}/lifecycle/activate",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(policy_id),
                crate::progenitor_support::encode_path(rule_id),
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
     * This function performs a `POST` to the `/api/v1/authorizationServers/{authServerId}/policies/{policyId}/rules/{ruleId}/lifecycle/deactivate` endpoint.
     *
     * Deactivate Authorization Server Policy Rule
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     * * `policy_id: &str`
     * * `rule_id: &str`
     */
    pub async fn deactivate_policy_rule(
        &self,
        auth_server_id: &str,
        policy_id: &str,
        rule_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/policies/{}/rules/{}/lifecycle/deactivate",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(policy_id),
                crate::progenitor_support::encode_path(rule_id),
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
     * This function performs a `GET` to the `/api/v1/authorizationServers/{authServerId}/scopes` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     * * `q: &str`
     * * `filter: &str`
     * * `cursor: &str`
     * * `limit: i64`
     */
    pub async fn list_o_auth_2_scopes(
        &self,
        auth_server_id: &str,
        q: &str,
        filter: &str,
        cursor: &str,
        limit: i64,
    ) -> ClientResult<Vec<crate::types::OAuth2Scope>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
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
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/scopes?{}",
                crate::progenitor_support::encode_path(auth_server_id),
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
     * This function performs a `GET` to the `/api/v1/authorizationServers/{authServerId}/scopes` endpoint.
     *
     * As opposed to `list_o_auth_2_scopes`, this function returns all the pages of the request at once.
     *
     * Success
     */
    pub async fn list_all_o_auth_2_scopes(
        &self,
        auth_server_id: &str,
        q: &str,
        filter: &str,
        cursor: &str,
    ) -> ClientResult<Vec<crate::types::OAuth2Scope>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/scopes?{}",
                crate::progenitor_support::encode_path(auth_server_id),
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
     * This function performs a `POST` to the `/api/v1/authorizationServers/{authServerId}/scopes` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     */
    pub async fn create_o_auth_2_scope(
        &self,
        auth_server_id: &str,
        body: &crate::types::OAuth2Scope,
    ) -> ClientResult<crate::types::OAuth2Scope> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/scopes",
                crate::progenitor_support::encode_path(auth_server_id),
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
     * This function performs a `GET` to the `/api/v1/authorizationServers/{authServerId}/scopes/{scopeId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     * * `scope_id: &str`
     */
    pub async fn get_o_auth_2_scope(
        &self,
        auth_server_id: &str,
        scope_id: &str,
    ) -> ClientResult<crate::types::OAuth2Scope> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/scopes/{}",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(scope_id),
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
     * This function performs a `PUT` to the `/api/v1/authorizationServers/{authServerId}/scopes/{scopeId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     * * `scope_id: &str`
     */
    pub async fn update_o_auth_2_scope(
        &self,
        auth_server_id: &str,
        scope_id: &str,
        body: &crate::types::OAuth2Scope,
    ) -> ClientResult<crate::types::OAuth2Scope> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/scopes/{}",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(scope_id),
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
     * This function performs a `DELETE` to the `/api/v1/authorizationServers/{authServerId}/scopes/{scopeId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `auth_server_id: &str`
     * * `scope_id: &str`
     */
    pub async fn delete_o_auth_2_scope(
        &self,
        auth_server_id: &str,
        scope_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/authorizationServers/{}/scopes/{}",
                crate::progenitor_support::encode_path(auth_server_id),
                crate::progenitor_support::encode_path(scope_id),
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
