use crate::Client;
use crate::ClientResult;

pub struct Users {
    pub client: Client,
}

impl Users {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Users { client }
    }

    /**
     * List Users.
     *
     * This function performs a `GET` to the `/api/v1/users` endpoint.
     *
     * Lists users in your organization with pagination in most cases.  A subset of users can be returned that match a supported filter expression or search criteria.
     *
     * **Parameters:**
     *
     * * `q: &str` -- Finds a user that matches firstName, lastName, and email properties.
     * * `after: &str` -- Specifies the pagination cursor for the next page of users.
     * * `limit: i64` -- Specifies the number of results returned.
     * * `filter: &str` -- Filters users with a supported expression for a subset of properties.
     * * `search: &str` -- Searches for users with a supported filtering  expression for most properties.
     * * `sort_by: &str`
     * * `sort_order: &str`
     */
    pub async fn list(
        &self,
        q: &str,
        after: &str,
        limit: i64,
        filter: &str,
        search: &str,
        sort_by: &str,
        sort_order: &str,
    ) -> ClientResult<Vec<crate::types::User>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
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
        if !search.is_empty() {
            query_args.push(("search".to_string(), search.to_string()));
        }
        if !sort_by.is_empty() {
            query_args.push(("sortBy".to_string(), sort_by.to_string()));
        }
        if !sort_order.is_empty() {
            query_args.push(("sortOrder".to_string(), sort_order.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/api/v1/users?{}", query_), None);
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
     * List Users.
     *
     * This function performs a `GET` to the `/api/v1/users` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Lists users in your organization with pagination in most cases.  A subset of users can be returned that match a supported filter expression or search criteria.
     */
    pub async fn list_all(
        &self,
        q: &str,
        filter: &str,
        search: &str,
        sort_by: &str,
        sort_order: &str,
    ) -> ClientResult<Vec<crate::types::User>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if !search.is_empty() {
            query_args.push(("search".to_string(), search.to_string()));
        }
        if !sort_by.is_empty() {
            query_args.push(("sortBy".to_string(), sort_by.to_string()));
        }
        if !sort_order.is_empty() {
            query_args.push(("sortOrder".to_string(), sort_order.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/api/v1/users?{}", query_), None);
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
     * Create User.
     *
     * This function performs a `POST` to the `/api/v1/users` endpoint.
     *
     * Creates a new user in your Okta organization with or without credentials.
     *
     * **Parameters:**
     *
     * * `activate: bool` -- Executes activation lifecycle operation when creating the user.
     * * `provider: bool` -- Indicates whether to create a user with a specified authentication provider.
     * * `next_login: &str` -- With activate=true, set nextLogin to "changePassword" to have the password be EXPIRED, so user must change it the next time they log in.
     */
    pub async fn create(
        &self,
        activate: bool,
        provider: bool,
        next_login: &str,
        body: &crate::types::CreateUserRequest,
    ) -> ClientResult<crate::types::User> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if activate {
            query_args.push(("activate".to_string(), activate.to_string()));
        }
        if !next_login.is_empty() {
            query_args.push(("nextLogin".to_string(), next_login.to_string()));
        }
        if provider {
            query_args.push(("provider".to_string(), provider.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/api/v1/users?{}", query_), None);
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
     * This function performs a `PUT` to the `/api/v1/users/{associatedUserId}/linkedObjects/{primaryRelationshipName}/{primaryUserId}` endpoint.
     *
     * **Parameters:**
     *
     * * `associated_user_id: &str`
     * * `primary_relationship_name: &str`
     * * `primary_user_id: &str`
     */
    pub async fn set_linked_object_for(
        &self,
        associated_user_id: &str,
        primary_relationship_name: &str,
        primary_user_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/linkedObjects/{}/{}",
                crate::progenitor_support::encode_path(associated_user_id),
                crate::progenitor_support::encode_path(primary_relationship_name),
                crate::progenitor_support::encode_path(primary_user_id),
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
     * Get User.
     *
     * This function performs a `GET` to the `/api/v1/users/{userId}` endpoint.
     *
     * Fetches a user from your Okta organization.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     */
    pub async fn get(&self, user_id: &str) -> ClientResult<crate::types::User> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}",
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
     * Update User.
     *
     * This function performs a `PUT` to the `/api/v1/users/{userId}` endpoint.
     *
     * Update a user's profile and/or credentials using strict-update semantics.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `strict: bool`
     */
    pub async fn update(
        &self,
        user_id: &str,
        strict: bool,
        body: &crate::types::User,
    ) -> ClientResult<crate::types::User> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if strict {
            query_args.push(("strict".to_string(), strict.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}?{}",
                crate::progenitor_support::encode_path(user_id),
                query_
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
     * This function performs a `POST` to the `/api/v1/users/{userId}` endpoint.
     *
     * Fetch a user by `id`, `login`, or `login shortname` if the short name is unambiguous.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `strict: bool`
     */
    pub async fn partial_update(
        &self,
        user_id: &str,
        strict: bool,
        body: &crate::types::User,
    ) -> ClientResult<crate::types::User> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if strict {
            query_args.push(("strict".to_string(), strict.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}?{}",
                crate::progenitor_support::encode_path(user_id),
                query_
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
     * Delete User.
     *
     * This function performs a `DELETE` to the `/api/v1/users/{userId}` endpoint.
     *
     * Deletes a user permanently.  This operation can only be performed on users that have a `DEPROVISIONED` status.  **This action cannot be recovered!**
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `send_email: bool`
     */
    pub async fn deactivate_or_delete(&self, user_id: &str, send_email: bool) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if send_email {
            query_args.push(("sendEmail".to_string(), send_email.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}?{}",
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
    /**
     * Get Assigned App Links.
     *
     * This function performs a `GET` to the `/api/v1/users/{userId}/appLinks` endpoint.
     *
     * Fetches appLinks for all direct or indirect (via group membership) assigned applications.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     */
    pub async fn list_app_links(&self, user_id: &str) -> ClientResult<Vec<crate::types::AppLink>> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/appLinks",
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
     * Get Assigned App Links.
     *
     * This function performs a `GET` to the `/api/v1/users/{userId}/appLinks` endpoint.
     *
     * As opposed to `list_app_links`, this function returns all the pages of the request at once.
     *
     * Fetches appLinks for all direct or indirect (via group membership) assigned applications.
     */
    pub async fn list_all_app_links(
        &self,
        user_id: &str,
    ) -> ClientResult<Vec<crate::types::AppLink>> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/appLinks",
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
    /**
     * This function performs a `GET` to the `/api/v1/users/{userId}/clients` endpoint.
     *
     * Lists all client resources for which the specified user has grants or tokens.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     */
    pub async fn list_clients(
        &self,
        user_id: &str,
    ) -> ClientResult<Vec<crate::types::OAuth2Client>> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/clients",
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
     * This function performs a `GET` to the `/api/v1/users/{userId}/clients` endpoint.
     *
     * As opposed to `list_clients`, this function returns all the pages of the request at once.
     *
     * Lists all client resources for which the specified user has grants or tokens.
     */
    pub async fn list_all_clients(
        &self,
        user_id: &str,
    ) -> ClientResult<Vec<crate::types::OAuth2Client>> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/clients",
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
    /**
     * This function performs a `GET` to the `/api/v1/users/{userId}/clients/{clientId}/grants` endpoint.
     *
     * Lists all grants for a specified user and client
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `client_id: &str`
     * * `expand: &str`
     * * `after: &str`
     * * `limit: i64`
     */
    pub async fn list_grants_for_and_clients(
        &self,
        user_id: &str,
        client_id: &str,
        expand: &str,
        after: &str,
        limit: i64,
    ) -> ClientResult<Vec<crate::types::OAuth2ScopeConsentGrant>> {
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
                "/api/v1/users/{}/clients/{}/grants?{}",
                crate::progenitor_support::encode_path(user_id),
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
     * This function performs a `GET` to the `/api/v1/users/{userId}/clients/{clientId}/grants` endpoint.
     *
     * As opposed to `list_grants_for_and_client`, this function returns all the pages of the request at once.
     *
     * Lists all grants for a specified user and client
     */
    pub async fn list_all_grants_for_and_clients(
        &self,
        user_id: &str,
        client_id: &str,
        expand: &str,
    ) -> ClientResult<Vec<crate::types::OAuth2ScopeConsentGrant>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/clients/{}/grants?{}",
                crate::progenitor_support::encode_path(user_id),
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
     * This function performs a `DELETE` to the `/api/v1/users/{userId}/clients/{clientId}/grants` endpoint.
     *
     * Revokes all grants for the specified user and client
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `client_id: &str`
     */
    pub async fn revoke_grants_for_and_client(
        &self,
        user_id: &str,
        client_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/clients/{}/grants",
                crate::progenitor_support::encode_path(user_id),
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
     * This function performs a `GET` to the `/api/v1/users/{userId}/clients/{clientId}/tokens` endpoint.
     *
     * Lists all refresh tokens issued for the specified User and Client.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `client_id: &str`
     * * `expand: &str`
     * * `after: &str`
     * * `limit: i64`
     */
    pub async fn list_refresh_tokens_for_and_clients(
        &self,
        user_id: &str,
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
                "/api/v1/users/{}/clients/{}/tokens?{}",
                crate::progenitor_support::encode_path(user_id),
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
     * This function performs a `GET` to the `/api/v1/users/{userId}/clients/{clientId}/tokens` endpoint.
     *
     * As opposed to `list_refresh_tokens_for_and_client`, this function returns all the pages of the request at once.
     *
     * Lists all refresh tokens issued for the specified User and Client.
     */
    pub async fn list_all_refresh_tokens_for_and_clients(
        &self,
        user_id: &str,
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
                "/api/v1/users/{}/clients/{}/tokens?{}",
                crate::progenitor_support::encode_path(user_id),
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
     * This function performs a `DELETE` to the `/api/v1/users/{userId}/clients/{clientId}/tokens` endpoint.
     *
     * Revokes all refresh tokens issued for the specified User and Client.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `client_id: &str`
     */
    pub async fn revoke_tokens_for_and_client(
        &self,
        user_id: &str,
        client_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/clients/{}/tokens",
                crate::progenitor_support::encode_path(user_id),
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
     * This function performs a `GET` to the `/api/v1/users/{userId}/clients/{clientId}/tokens/{tokenId}` endpoint.
     *
     * Gets a refresh token issued for the specified User and Client.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `client_id: &str`
     * * `token_id: &str`
     * * `expand: &str`
     * * `limit: i64`
     * * `after: &str`
     */
    pub async fn get_refresh_token_for_and_client(
        &self,
        user_id: &str,
        client_id: &str,
        token_id: &str,
        expand: &str,
        limit: i64,
        after: &str,
    ) -> ClientResult<crate::types::OAuth2RefreshToken> {
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
                "/api/v1/users/{}/clients/{}/tokens/{}?{}",
                crate::progenitor_support::encode_path(user_id),
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
     * This function performs a `DELETE` to the `/api/v1/users/{userId}/clients/{clientId}/tokens/{tokenId}` endpoint.
     *
     * Revokes the specified refresh token.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `client_id: &str`
     * * `token_id: &str`
     */
    pub async fn revoke_token_for_and_client(
        &self,
        user_id: &str,
        client_id: &str,
        token_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/clients/{}/tokens/{}",
                crate::progenitor_support::encode_path(user_id),
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
     * Change Password.
     *
     * This function performs a `POST` to the `/api/v1/users/{userId}/credentials/change_password` endpoint.
     *
     * Changes a user's password by validating the user's current password. This operation can only be performed on users in `STAGED`, `ACTIVE`, `PASSWORD_EXPIRED`, or `RECOVERY` status that have a valid password credential
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `strict: bool`
     */
    pub async fn change_password(
        &self,
        user_id: &str,
        strict: bool,
        body: &crate::types::ChangePasswordRequest,
    ) -> ClientResult<crate::types::UserCredentials> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if strict {
            query_args.push(("strict".to_string(), strict.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/credentials/change_password?{}",
                crate::progenitor_support::encode_path(user_id),
                query_
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
     * Change Recovery Question.
     *
     * This function performs a `POST` to the `/api/v1/users/{userId}/credentials/change_recovery_question` endpoint.
     *
     * Changes a user's recovery question & answer credential by validating the user's current password.  This operation can only be performed on users in **STAGED**, **ACTIVE** or **RECOVERY** `status` that have a valid password credential
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     */
    pub async fn change_recovery_question(
        &self,
        user_id: &str,
        body: &crate::types::UserCredentials,
    ) -> ClientResult<crate::types::UserCredentials> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/credentials/change_recovery_question",
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
     * Forgot Password.
     *
     * This function performs a `POST` to the `/api/v1/users/{userId}/credentials/forgot_password` endpoint.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     */
    pub async fn post_credentials_forgot_password(
        &self,
        user_id: &str,
    ) -> ClientResult<crate::types::ResetPasswordToken> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/credentials/forgot_password",
                crate::progenitor_support::encode_path(user_id),
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
     * This function performs a `GET` to the `/api/v1/users/{userId}/grants` endpoint.
     *
     * Lists all grants for the specified user
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `scope_id: &str`
     * * `expand: &str`
     * * `after: &str`
     * * `limit: i64`
     */
    pub async fn list_grants(
        &self,
        user_id: &str,
        scope_id: &str,
        expand: &str,
        after: &str,
        limit: i64,
    ) -> ClientResult<Vec<crate::types::OAuth2ScopeConsentGrant>> {
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
        if !scope_id.is_empty() {
            query_args.push(("scopeId".to_string(), scope_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/grants?{}",
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
     * This function performs a `GET` to the `/api/v1/users/{userId}/grants` endpoint.
     *
     * As opposed to `list_grants`, this function returns all the pages of the request at once.
     *
     * Lists all grants for the specified user
     */
    pub async fn list_all_grants(
        &self,
        user_id: &str,
        scope_id: &str,
        expand: &str,
    ) -> ClientResult<Vec<crate::types::OAuth2ScopeConsentGrant>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        if !scope_id.is_empty() {
            query_args.push(("scopeId".to_string(), scope_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/grants?{}",
                crate::progenitor_support::encode_path(user_id),
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
     * This function performs a `DELETE` to the `/api/v1/users/{userId}/grants` endpoint.
     *
     * Revokes all grants for a specified user
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     */
    pub async fn revoke_grants(&self, user_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/grants",
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
     * This function performs a `GET` to the `/api/v1/users/{userId}/grants/{grantId}` endpoint.
     *
     * Gets a grant for the specified user
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `grant_id: &str`
     * * `expand: &str`
     */
    pub async fn get_grant(
        &self,
        user_id: &str,
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
                "/api/v1/users/{}/grants/{}?{}",
                crate::progenitor_support::encode_path(user_id),
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
     * This function performs a `DELETE` to the `/api/v1/users/{userId}/grants/{grantId}` endpoint.
     *
     * Revokes one grant for a specified user
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `grant_id: &str`
     */
    pub async fn revoke_grant(&self, user_id: &str, grant_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/grants/{}",
                crate::progenitor_support::encode_path(user_id),
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
     * Get Member Groups.
     *
     * This function performs a `GET` to the `/api/v1/users/{userId}/groups` endpoint.
     *
     * Fetches the groups of which the user is a member.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     */
    pub async fn list_groups(&self, user_id: &str) -> ClientResult<Vec<crate::types::Group>> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/groups",
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
     * Get Member Groups.
     *
     * This function performs a `GET` to the `/api/v1/users/{userId}/groups` endpoint.
     *
     * As opposed to `list_groups`, this function returns all the pages of the request at once.
     *
     * Fetches the groups of which the user is a member.
     */
    pub async fn list_all_groups(&self, user_id: &str) -> ClientResult<Vec<crate::types::Group>> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/groups",
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
    /**
     * Listing IdPs associated with a user.
     *
     * This function performs a `GET` to the `/api/v1/users/{userId}/idps` endpoint.
     *
     * Lists the IdPs associated with the user.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     */
    pub async fn list_identity_providers(
        &self,
        user_id: &str,
    ) -> ClientResult<Vec<crate::types::IdentityProvider>> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/idps",
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
     * Listing IdPs associated with a user.
     *
     * This function performs a `GET` to the `/api/v1/users/{userId}/idps` endpoint.
     *
     * As opposed to `list_identity_providers`, this function returns all the pages of the request at once.
     *
     * Lists the IdPs associated with the user.
     */
    pub async fn list_all_identity_providers(
        &self,
        user_id: &str,
    ) -> ClientResult<Vec<crate::types::IdentityProvider>> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/idps",
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
    /**
     * Activate User.
     *
     * This function performs a `POST` to the `/api/v1/users/{userId}/lifecycle/activate` endpoint.
     *
     * Activates a user.  This operation can only be performed on users with a `STAGED` status.  Activation of a user is an asynchronous operation. The user will have the `transitioningToStatus` property with a value of `ACTIVE` during activation to indicate that the user hasn't completed the asynchronous operation.  The user will have a status of `ACTIVE` when the activation process is complete.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `send_email: bool` -- Sends an activation email to the user if true.
     */
    pub async fn activate(
        &self,
        user_id: &str,
        send_email: bool,
    ) -> ClientResult<crate::types::UserActivationToken> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if send_email {
            query_args.push(("sendEmail".to_string(), send_email.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/lifecycle/activate?{}",
                crate::progenitor_support::encode_path(user_id),
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
     * Deactivate User.
     *
     * This function performs a `POST` to the `/api/v1/users/{userId}/lifecycle/deactivate` endpoint.
     *
     * Deactivates a user.  This operation can only be performed on users that do not have a `DEPROVISIONED` status.  Deactivation of a user is an asynchronous operation.  The user will have the `transitioningToStatus` property with a value of `DEPROVISIONED` during deactivation to indicate that the user hasn't completed the asynchronous operation.  The user will have a status of `DEPROVISIONED` when the deactivation process is complete.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `send_email: bool`
     */
    pub async fn deactivate(&self, user_id: &str, send_email: bool) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if send_email {
            query_args.push(("sendEmail".to_string(), send_email.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/lifecycle/deactivate?{}",
                crate::progenitor_support::encode_path(user_id),
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
     * Expire Password.
     *
     * This function performs a `POST` to the `/api/v1/users/{userId}/lifecycle/expire_password?tempPassword=false` endpoint.
     *
     * This operation transitions the user to the status of `PASSWORD_EXPIRED` so that the user is required to change their password at their next login.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     */
    pub async fn expire_password(&self, user_id: &str) -> ClientResult<crate::types::User> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/lifecycle/expire_password?tempPassword=false",
                crate::progenitor_support::encode_path(user_id),
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
     * Expire Password.
     *
     * This function performs a `POST` to the `/api/v1/users/{userId}/lifecycle/expire_password?tempPassword=true` endpoint.
     *
     * This operation transitions the user to the status of `PASSWORD_EXPIRED` and the user's password is reset to a temporary password that is returned.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     */
    pub async fn expire_password_and_get_temporary(
        &self,
        user_id: &str,
    ) -> ClientResult<crate::types::TempPassword> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/lifecycle/expire_password?tempPassword=true",
                crate::progenitor_support::encode_path(user_id),
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
     * Reactivate User.
     *
     * This function performs a `POST` to the `/api/v1/users/{userId}/lifecycle/reactivate` endpoint.
     *
     * Reactivates a user.  This operation can only be performed on users with a `PROVISIONED` status.  This operation restarts the activation workflow if for some reason the user activation was not completed when using the activationToken from [Activate User](#activate-user).
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `send_email: bool` -- Sends an activation email to the user if true.
     */
    pub async fn reactivate(
        &self,
        user_id: &str,
        send_email: bool,
    ) -> ClientResult<crate::types::UserActivationToken> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if send_email {
            query_args.push(("sendEmail".to_string(), send_email.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/lifecycle/reactivate?{}",
                crate::progenitor_support::encode_path(user_id),
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
     * Reset Factors.
     *
     * This function performs a `POST` to the `/api/v1/users/{userId}/lifecycle/reset_factors` endpoint.
     *
     * This operation resets all factors for the specified user. All MFA factor enrollments returned to the unenrolled state. The user's status remains ACTIVE. This link is present only if the user is currently enrolled in one or more MFA factors.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     */
    pub async fn reset_factors(&self, user_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/lifecycle/reset_factors",
                crate::progenitor_support::encode_path(user_id),
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
     * Reset Password.
     *
     * This function performs a `POST` to the `/api/v1/users/{userId}/lifecycle/reset_password` endpoint.
     *
     * Generates a one-time token (OTT) that can be used to reset a user's password.  The OTT link can be automatically emailed to the user or returned to the API caller and distributed using a custom flow.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `send_email: bool`
     */
    pub async fn reset_password(
        &self,
        user_id: &str,
        send_email: bool,
    ) -> ClientResult<crate::types::ResetPasswordToken> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if send_email {
            query_args.push(("sendEmail".to_string(), send_email.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/lifecycle/reset_password?{}",
                crate::progenitor_support::encode_path(user_id),
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
     * Suspend User.
     *
     * This function performs a `POST` to the `/api/v1/users/{userId}/lifecycle/suspend` endpoint.
     *
     * Suspends a user.  This operation can only be performed on users with an `ACTIVE` status.  The user will have a status of `SUSPENDED` when the process is complete.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     */
    pub async fn suspend(&self, user_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/lifecycle/suspend",
                crate::progenitor_support::encode_path(user_id),
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
     * Unlock User.
     *
     * This function performs a `POST` to the `/api/v1/users/{userId}/lifecycle/unlock` endpoint.
     *
     * Unlocks a user with a `LOCKED_OUT` status and returns them to `ACTIVE` status.  Users will be able to login with their current password.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     */
    pub async fn unlock(&self, user_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/lifecycle/unlock",
                crate::progenitor_support::encode_path(user_id),
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
     * Unsuspend User.
     *
     * This function performs a `POST` to the `/api/v1/users/{userId}/lifecycle/unsuspend` endpoint.
     *
     * Unsuspends a user and returns them to the `ACTIVE` state.  This operation can only be performed on users that have a `SUSPENDED` status.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     */
    pub async fn unsuspend(&self, user_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/lifecycle/unsuspend",
                crate::progenitor_support::encode_path(user_id),
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
     * This function performs a `GET` to the `/api/v1/users/{userId}/linkedObjects/{relationshipName}` endpoint.
     *
     * Get linked objects for a user, relationshipName can be a primary or associated relationship name
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `relationship_name: &str`
     * * `after: &str`
     * * `limit: i64`
     */
    pub async fn get_linked_objects_fors(
        &self,
        user_id: &str,
        relationship_name: &str,
        after: &str,
        limit: i64,
    ) -> ClientResult<Vec<crate::types::Links>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/linkedObjects/{}?{}",
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(relationship_name),
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
     * This function performs a `GET` to the `/api/v1/users/{userId}/linkedObjects/{relationshipName}` endpoint.
     *
     * As opposed to `get_linked_objects_for`, this function returns all the pages of the request at once.
     *
     * Get linked objects for a user, relationshipName can be a primary or associated relationship name
     */
    pub async fn get_all_linked_objects_fors(
        &self,
        user_id: &str,
        relationship_name: &str,
    ) -> ClientResult<Vec<crate::types::Links>> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/linkedObjects/{}",
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(relationship_name),
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
     * This function performs a `DELETE` to the `/api/v1/users/{userId}/linkedObjects/{relationshipName}` endpoint.
     *
     * Delete linked objects for a user, relationshipName can be ONLY a primary relationship name
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `relationship_name: &str`
     */
    pub async fn remove_linked_object_for(
        &self,
        user_id: &str,
        relationship_name: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/linkedObjects/{}",
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(relationship_name),
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
     * This function performs a `GET` to the `/api/v1/users/{userId}/roles` endpoint.
     *
     * Lists all roles assigned to a user.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `expand: &str`
     */
    pub async fn list_assigned_roles_fors(
        &self,
        user_id: &str,
        expand: &str,
    ) -> ClientResult<Vec<crate::types::Role>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/roles?{}",
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
     * This function performs a `GET` to the `/api/v1/users/{userId}/roles` endpoint.
     *
     * As opposed to `list_assigned_roles_for`, this function returns all the pages of the request at once.
     *
     * Lists all roles assigned to a user.
     */
    pub async fn list_all_assigned_roles_fors(
        &self,
        user_id: &str,
        expand: &str,
    ) -> ClientResult<Vec<crate::types::Role>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !expand.is_empty() {
            query_args.push(("expand".to_string(), expand.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/roles?{}",
                crate::progenitor_support::encode_path(user_id),
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
     * This function performs a `POST` to the `/api/v1/users/{userId}/roles` endpoint.
     *
     * Assigns a role to a user.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `disable_notifications: &str`
     */
    pub async fn assign_role(
        &self,
        user_id: &str,
        disable_notifications: &str,
        body: &crate::types::AssignRoleRequest,
    ) -> ClientResult<crate::types::Role> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !disable_notifications.is_empty() {
            query_args.push((
                "disableNotifications".to_string(),
                disable_notifications.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/roles?{}",
                crate::progenitor_support::encode_path(user_id),
                query_
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
     * This function performs a `DELETE` to the `/api/v1/users/{userId}/roles/{roleId}` endpoint.
     *
     * Unassigns a role from a user.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `role_id: &str`
     */
    pub async fn remove_role_from(&self, user_id: &str, role_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/roles/{}",
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(role_id),
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
     * This function performs a `GET` to the `/api/v1/users/{userId}/roles/{roleId}/targets/catalog/apps` endpoint.
     *
     * Lists all App targets for an `APP_ADMIN` Role assigned to a User. This methods return list may include full Applications or Instances. The response for an instance will have an `ID` value, while Application will not have an ID.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `role_id: &str`
     * * `after: &str`
     * * `limit: i64`
     */
    pub async fn list_application_targets_for_administrator_roles(
        &self,
        user_id: &str,
        role_id: &str,
        after: &str,
        limit: i64,
    ) -> ClientResult<Vec<crate::types::CatalogApplication>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/roles/{}/targets/catalog/apps?{}",
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(role_id),
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
     * This function performs a `GET` to the `/api/v1/users/{userId}/roles/{roleId}/targets/catalog/apps` endpoint.
     *
     * As opposed to `list_application_targets_for_administrator_role`, this function returns all the pages of the request at once.
     *
     * Lists all App targets for an `APP_ADMIN` Role assigned to a User. This methods return list may include full Applications or Instances. The response for an instance will have an `ID` value, while Application will not have an ID.
     */
    pub async fn list_all_application_targets_for_administrator_roles(
        &self,
        user_id: &str,
        role_id: &str,
    ) -> ClientResult<Vec<crate::types::CatalogApplication>> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/roles/{}/targets/catalog/apps",
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(role_id),
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
     * This function performs a `PUT` to the `/api/v1/users/{userId}/roles/{roleId}/targets/catalog/apps` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `role_id: &str`
     */
    pub async fn add_all_apps_as_target_role(
        &self,
        user_id: &str,
        role_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/roles/{}/targets/catalog/apps",
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(role_id),
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
     * This function performs a `PUT` to the `/api/v1/users/{userId}/roles/{roleId}/targets/catalog/apps/{appName}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `role_id: &str`
     * * `app_name: &str`
     */
    pub async fn add_application_target_admin_role_for(
        &self,
        user_id: &str,
        role_id: &str,
        app_name: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/roles/{}/targets/catalog/apps/{}",
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(role_id),
                crate::progenitor_support::encode_path(app_name),
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
     * This function performs a `DELETE` to the `/api/v1/users/{userId}/roles/{roleId}/targets/catalog/apps/{appName}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `role_id: &str`
     * * `app_name: &str`
     */
    pub async fn remove_application_target_from_administrator_role_for(
        &self,
        user_id: &str,
        role_id: &str,
        app_name: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/roles/{}/targets/catalog/apps/{}",
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(role_id),
                crate::progenitor_support::encode_path(app_name),
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
     * Add App Instance Target to App Administrator Role given to a User.
     *
     * This function performs a `PUT` to the `/api/v1/users/{userId}/roles/{roleId}/targets/catalog/apps/{appName}/{applicationId}` endpoint.
     *
     * Add App Instance Target to App Administrator Role given to a User
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `role_id: &str`
     * * `app_name: &str`
     * * `application_id: &str`
     */
    pub async fn add_application_target_app_admin_role_for(
        &self,
        user_id: &str,
        role_id: &str,
        app_name: &str,
        application_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/roles/{}/targets/catalog/apps/{}/{}",
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(role_id),
                crate::progenitor_support::encode_path(app_name),
                crate::progenitor_support::encode_path(application_id),
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
     * Remove App Instance Target to App Administrator Role given to a User.
     *
     * This function performs a `DELETE` to the `/api/v1/users/{userId}/roles/{roleId}/targets/catalog/apps/{appName}/{applicationId}` endpoint.
     *
     * Remove App Instance Target to App Administrator Role given to a User
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `role_id: &str`
     * * `app_name: &str`
     * * `application_id: &str`
     */
    pub async fn remove_application_target_from_administrator_role_for_users(
        &self,
        user_id: &str,
        role_id: &str,
        app_name: &str,
        application_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/roles/{}/targets/catalog/apps/{}/{}",
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(role_id),
                crate::progenitor_support::encode_path(app_name),
                crate::progenitor_support::encode_path(application_id),
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
     * This function performs a `GET` to the `/api/v1/users/{userId}/roles/{roleId}/targets/groups` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `role_id: &str`
     * * `after: &str`
     * * `limit: i64`
     */
    pub async fn list_group_targets_for_roles(
        &self,
        user_id: &str,
        role_id: &str,
        after: &str,
        limit: i64,
    ) -> ClientResult<Vec<crate::types::Group>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/roles/{}/targets/groups?{}",
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(role_id),
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
     * This function performs a `GET` to the `/api/v1/users/{userId}/roles/{roleId}/targets/groups` endpoint.
     *
     * As opposed to `list_group_targets_for_role`, this function returns all the pages of the request at once.
     *
     * Success
     */
    pub async fn list_all_group_targets_for_roles(
        &self,
        user_id: &str,
        role_id: &str,
    ) -> ClientResult<Vec<crate::types::Group>> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/roles/{}/targets/groups",
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(role_id),
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
     * This function performs a `PUT` to the `/api/v1/users/{userId}/roles/{roleId}/targets/groups/{groupId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `role_id: &str`
     * * `group_id: &str`
     */
    pub async fn add_group_target_role(
        &self,
        user_id: &str,
        role_id: &str,
        group_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/roles/{}/targets/groups/{}",
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(role_id),
                crate::progenitor_support::encode_path(group_id),
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
     * This function performs a `DELETE` to the `/api/v1/users/{userId}/roles/{roleId}/targets/groups/{groupId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `role_id: &str`
     * * `group_id: &str`
     */
    pub async fn remove_group_target_from_role(
        &self,
        user_id: &str,
        role_id: &str,
        group_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/roles/{}/targets/groups/{}",
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(role_id),
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
     * This function performs a `DELETE` to the `/api/v1/users/{userId}/sessions` endpoint.
     *
     * Removes all active identity provider sessions. This forces the user to authenticate on the next operation. Optionally revokes OpenID Connect and OAuth refresh and access tokens issued to the user.
     *
     * **Parameters:**
     *
     * * `user_id: &str`
     * * `oauth_tokens: bool` -- Revoke issued OpenID Connect and OAuth refresh and access tokens.
     */
    pub async fn clear_sessions(&self, user_id: &str, oauth_tokens: bool) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if oauth_tokens {
            query_args.push(("oauthTokens".to_string(), oauth_tokens.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/users/{}/sessions?{}",
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
