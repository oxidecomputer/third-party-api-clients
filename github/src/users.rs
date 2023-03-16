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
     * Get the authenticated user.
     *
     * This function performs a `GET` to the `/user` endpoint.
     *
     * If the authenticated user is authenticated through basic authentication or OAuth with the `user` scope, then the response lists public and private profile information.
     *
     * If the authenticated user is authenticated through OAuth without the `user` scope, then the response lists only public profile information.
     *
     * FROM: <https://docs.github.com/rest/reference/users#get-the-authenticated-user>
     */
    pub async fn get_authenticated_public_user(
        &self,
    ) -> ClientResult<crate::Response<crate::types::PublicUser>> {
        let url = self.client.url("/user", None);
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
     * Get the authenticated user.
     *
     * This function performs a `GET` to the `/user` endpoint.
     *
     * If the authenticated user is authenticated through basic authentication or OAuth with the `user` scope, then the response lists public and private profile information.
     *
     * If the authenticated user is authenticated through OAuth without the `user` scope, then the response lists only public profile information.
     *
     * FROM: <https://docs.github.com/rest/reference/users#get-the-authenticated-user>
     */
    pub async fn get_authenticated_private_user(
        &self,
    ) -> ClientResult<crate::Response<crate::types::PrivateUser>> {
        let url = self.client.url("/user", None);
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
     * Get the authenticated user.
     *
     * This function performs a `GET` to the `/user` endpoint.
     *
     * If the authenticated user is authenticated through basic authentication or OAuth with the `user` scope, then the response lists public and private profile information.
     *
     * If the authenticated user is authenticated through OAuth without the `user` scope, then the response lists only public profile information.
     *
     * FROM: <https://docs.github.com/rest/reference/users#get-the-authenticated-user>
     */
    pub async fn get_authenticated(
        &self,
    ) -> ClientResult<crate::Response<crate::types::UsersGetByUsernameResponseOneOf>> {
        let url = self.client.url("/user", None);
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
     * Update the authenticated user.
     *
     * This function performs a `PATCH` to the `/user` endpoint.
     *
     * **Note:** If your email is set to private and you send an `email` parameter as part of this request to update your profile, your privacy settings are still enforced: the email address will not be displayed on your public profile or via the API.
     *
     * FROM: <https://docs.github.com/rest/reference/users/#update-the-authenticated-user>
     */
    pub async fn update_authenticated(
        &self,
        body: &crate::types::UsersUpdateAuthenticatedRequest,
    ) -> ClientResult<crate::Response<crate::types::PrivateUser>> {
        let url = self.client.url("/user", None);
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
     * List users blocked by the authenticated user.
     *
     * This function performs a `GET` to the `/user/blocks` endpoint.
     *
     * List the users you've blocked on your personal account.
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-users-blocked-by-the-authenticated-user>
     */
    pub async fn list_blocked_by_authenticated(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url("/user/blocks", None);
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
     * List users blocked by the authenticated user.
     *
     * This function performs a `GET` to the `/user/blocks` endpoint.
     *
     * As opposed to `list_blocked_by_authenticated`, this function returns all the pages of the request at once.
     *
     * List the users you've blocked on your personal account.
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-users-blocked-by-the-authenticated-user>
     */
    pub async fn list_all_blocked_by_authenticated(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url("/user/blocks", None);
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
     * Check if a user is blocked by the authenticated user.
     *
     * This function performs a `GET` to the `/user/blocks/{username}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/users#check-if-a-user-is-blocked-by-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     */
    pub async fn check_blocked(&self, username: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/blocks/{}",
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
    /**
     * Block a user.
     *
     * This function performs a `PUT` to the `/user/blocks/{username}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/users#block-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     */
    pub async fn block(&self, username: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/blocks/{}",
                crate::progenitor_support::encode_path(username),
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
     * Unblock a user.
     *
     * This function performs a `DELETE` to the `/user/blocks/{username}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/users#unblock-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     */
    pub async fn unblock(&self, username: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/blocks/{}",
                crate::progenitor_support::encode_path(username),
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
     * Set primary email visibility for the authenticated user.
     *
     * This function performs a `PATCH` to the `/user/email/visibility` endpoint.
     *
     * Sets the visibility for your primary email addresses.
     *
     * FROM: <https://docs.github.com/rest/reference/users#set-primary-email-visibility-for-the-authenticated-user>
     */
    pub async fn set_primary_email_visibility_for_authenticated(
        &self,
        body: &crate::types::UsersSetPrimaryEmailVisibilityAuthenticatedRequest,
    ) -> ClientResult<crate::Response<Vec<crate::types::Email>>> {
        let url = self.client.url("/user/email/visibility", None);
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
     * List email addresses for the authenticated user.
     *
     * This function performs a `GET` to the `/user/emails` endpoint.
     *
     * Lists all of your email addresses, and specifies which one is visible to the public. This endpoint is accessible with the `user:email` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-email-addresses-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_emails_for_authenticated(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Email>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/user/emails?{}", query_), None);
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
     * List email addresses for the authenticated user.
     *
     * This function performs a `GET` to the `/user/emails` endpoint.
     *
     * As opposed to `list_emails_for_authenticated`, this function returns all the pages of the request at once.
     *
     * Lists all of your email addresses, and specifies which one is visible to the public. This endpoint is accessible with the `user:email` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-email-addresses-for-the-authenticated-user>
     */
    pub async fn list_all_emails_for_authenticated(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::Email>>> {
        let url = self.client.url("/user/emails", None);
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
     * Add an email address for the authenticated user.
     *
     * This function performs a `POST` to the `/user/emails` endpoint.
     *
     * This endpoint is accessible with the `user` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/users#add-an-email-address-for-the-authenticated-user>
     */
    pub async fn add_email_for_authenticated(
        &self,
        body: &crate::types::UsersAddEmailAuthenticatedRequestOneOf,
    ) -> ClientResult<crate::Response<Vec<crate::types::Email>>> {
        let url = self.client.url("/user/emails", None);
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
     * Delete an email address for the authenticated user.
     *
     * This function performs a `DELETE` to the `/user/emails` endpoint.
     *
     * This endpoint is accessible with the `user` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/users#delete-an-email-address-for-the-authenticated-user>
     */
    pub async fn delete_email_for_authenticated(
        &self,
        body: &crate::types::UsersAddEmailAuthenticatedRequestOneOf,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url("/user/emails", None);
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
     * List followers of the authenticated user.
     *
     * This function performs a `GET` to the `/user/followers` endpoint.
     *
     * Lists the people following the authenticated user.
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-followers-of-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_followers_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
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
            .url(&format!("/user/followers?{}", query_), None);
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
     * List followers of the authenticated user.
     *
     * This function performs a `GET` to the `/user/followers` endpoint.
     *
     * As opposed to `list_followers_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * Lists the people following the authenticated user.
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-followers-of-the-authenticated-user>
     */
    pub async fn list_all_followers_for_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url("/user/followers", None);
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
     * List the people the authenticated user follows.
     *
     * This function performs a `GET` to the `/user/following` endpoint.
     *
     * Lists the people who the authenticated user follows.
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-the-people-the-authenticated-user-follows>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_followed_by_authenticated(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
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
            .url(&format!("/user/following?{}", query_), None);
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
     * List the people the authenticated user follows.
     *
     * This function performs a `GET` to the `/user/following` endpoint.
     *
     * As opposed to `list_followed_by_authenticated`, this function returns all the pages of the request at once.
     *
     * Lists the people who the authenticated user follows.
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-the-people-the-authenticated-user-follows>
     */
    pub async fn list_all_followed_by_authenticated(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url("/user/following", None);
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
     * Check if a person is followed by the authenticated user.
     *
     * This function performs a `GET` to the `/user/following/{username}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/users#check-if-a-person-is-followed-by-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     */
    pub async fn check_person_is_followed_by_authenticated(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/following/{}",
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
    /**
     * Follow a user.
     *
     * This function performs a `PUT` to the `/user/following/{username}` endpoint.
     *
     * Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs)."
     *
     * Following a user requires the user to be logged in and authenticated with basic auth or OAuth with the `user:follow` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/users#follow-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     */
    pub async fn follow(&self, username: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/following/{}",
                crate::progenitor_support::encode_path(username),
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
     * Unfollow a user.
     *
     * This function performs a `DELETE` to the `/user/following/{username}` endpoint.
     *
     * Unfollowing a user requires the user to be logged in and authenticated with basic auth or OAuth with the `user:follow` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/users#unfollow-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     */
    pub async fn unfollow(&self, username: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/following/{}",
                crate::progenitor_support::encode_path(username),
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
     * List GPG keys for the authenticated user.
     *
     * This function performs a `GET` to the `/user/gpg_keys` endpoint.
     *
     * Lists the current user's GPG keys. Requires that you are authenticated via Basic Auth or via OAuth with at least `read:gpg_key` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-gpg-keys-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_gpg_keys_for_authenticated(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::GpgKey>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/user/gpg_keys?{}", query_), None);
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
     * List GPG keys for the authenticated user.
     *
     * This function performs a `GET` to the `/user/gpg_keys` endpoint.
     *
     * As opposed to `list_gpg_keys_for_authenticated`, this function returns all the pages of the request at once.
     *
     * Lists the current user's GPG keys. Requires that you are authenticated via Basic Auth or via OAuth with at least `read:gpg_key` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-gpg-keys-for-the-authenticated-user>
     */
    pub async fn list_all_gpg_keys_for_authenticated(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::GpgKey>>> {
        let url = self.client.url("/user/gpg_keys", None);
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
     * Create a GPG key for the authenticated user.
     *
     * This function performs a `POST` to the `/user/gpg_keys` endpoint.
     *
     * Adds a GPG key to the authenticated user's GitHub account. Requires that you are authenticated via Basic Auth, or OAuth with at least `write:gpg_key` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/users#create-a-gpg-key-for-the-authenticated-user>
     */
    pub async fn create_gpg_key_for_authenticated(
        &self,
        body: &crate::types::UsersCreateGpgKeyAuthenticatedRequest,
    ) -> ClientResult<crate::Response<crate::types::GpgKey>> {
        let url = self.client.url("/user/gpg_keys", None);
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
     * Get a GPG key for the authenticated user.
     *
     * This function performs a `GET` to the `/user/gpg_keys/{gpg_key_id}` endpoint.
     *
     * View extended details for a single GPG key. Requires that you are authenticated via Basic Auth or via OAuth with at least `read:gpg_key` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/users#get-a-gpg-key-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `gpg_key_id: i64` -- gpg_key_id parameter.
     */
    pub async fn get_gpg_key_for_authenticated(
        &self,
        gpg_key_id: i64,
    ) -> ClientResult<crate::Response<crate::types::GpgKey>> {
        let url = self.client.url(
            &format!(
                "/user/gpg_keys/{}",
                crate::progenitor_support::encode_path(&gpg_key_id.to_string()),
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
     * Delete a GPG key for the authenticated user.
     *
     * This function performs a `DELETE` to the `/user/gpg_keys/{gpg_key_id}` endpoint.
     *
     * Removes a GPG key from the authenticated user's GitHub account. Requires that you are authenticated via Basic Auth or via OAuth with at least `admin:gpg_key` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/users#delete-a-gpg-key-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `gpg_key_id: i64` -- gpg_key_id parameter.
     */
    pub async fn delete_gpg_key_for_authenticated(
        &self,
        gpg_key_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/gpg_keys/{}",
                crate::progenitor_support::encode_path(&gpg_key_id.to_string()),
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
     * List public SSH keys for the authenticated user.
     *
     * This function performs a `GET` to the `/user/keys` endpoint.
     *
     * Lists the public SSH keys for the authenticated user's GitHub account. Requires that you are authenticated via Basic Auth or via OAuth with at least `read:public_key` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-public-ssh-keys-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_public_ssh_keys_for_authenticated(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Key>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/user/keys?{}", query_), None);
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
     * List public SSH keys for the authenticated user.
     *
     * This function performs a `GET` to the `/user/keys` endpoint.
     *
     * As opposed to `list_public_ssh_keys_for_authenticated`, this function returns all the pages of the request at once.
     *
     * Lists the public SSH keys for the authenticated user's GitHub account. Requires that you are authenticated via Basic Auth or via OAuth with at least `read:public_key` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-public-ssh-keys-for-the-authenticated-user>
     */
    pub async fn list_all_public_ssh_keys_for_authenticated(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::Key>>> {
        let url = self.client.url("/user/keys", None);
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
     * Create a public SSH key for the authenticated user.
     *
     * This function performs a `POST` to the `/user/keys` endpoint.
     *
     * Adds a public SSH key to the authenticated user's GitHub account. Requires that you are authenticated via Basic Auth, or OAuth with at least `write:public_key` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/users#create-a-public-ssh-key-for-the-authenticated-user>
     */
    pub async fn create_public_ssh_key_for_authenticated(
        &self,
        body: &crate::types::UsersCreatePublicSshKeyAuthenticatedRequest,
    ) -> ClientResult<crate::Response<crate::types::Key>> {
        let url = self.client.url("/user/keys", None);
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
     * Get a public SSH key for the authenticated user.
     *
     * This function performs a `GET` to the `/user/keys/{key_id}` endpoint.
     *
     * View extended details for a single public SSH key. Requires that you are authenticated via Basic Auth or via OAuth with at least `read:public_key` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/users#get-a-public-ssh-key-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `key_id: i64` -- key_id parameter.
     */
    pub async fn get_public_ssh_key_for_authenticated(
        &self,
        key_id: i64,
    ) -> ClientResult<crate::Response<crate::types::Key>> {
        let url = self.client.url(
            &format!(
                "/user/keys/{}",
                crate::progenitor_support::encode_path(&key_id.to_string()),
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
     * Delete a public SSH key for the authenticated user.
     *
     * This function performs a `DELETE` to the `/user/keys/{key_id}` endpoint.
     *
     * Removes a public SSH key from the authenticated user's GitHub account. Requires that you are authenticated via Basic Auth or via OAuth with at least `admin:public_key` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/users#delete-a-public-ssh-key-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `key_id: i64` -- key_id parameter.
     */
    pub async fn delete_public_ssh_key_for_authenticated(
        &self,
        key_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/keys/{}",
                crate::progenitor_support::encode_path(&key_id.to_string()),
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
     * List public email addresses for the authenticated user.
     *
     * This function performs a `GET` to the `/user/public_emails` endpoint.
     *
     * Lists your publicly visible email address, which you can set with the [Set primary email visibility for the authenticated user](https://docs.github.com/rest/reference/users#set-primary-email-visibility-for-the-authenticated-user) endpoint. This endpoint is accessible with the `user:email` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-public-email-addresses-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_public_emails_for_authenticated(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Email>>> {
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
            .url(&format!("/user/public_emails?{}", query_), None);
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
     * List public email addresses for the authenticated user.
     *
     * This function performs a `GET` to the `/user/public_emails` endpoint.
     *
     * As opposed to `list_public_emails_for_authenticated`, this function returns all the pages of the request at once.
     *
     * Lists your publicly visible email address, which you can set with the [Set primary email visibility for the authenticated user](https://docs.github.com/rest/reference/users#set-primary-email-visibility-for-the-authenticated-user) endpoint. This endpoint is accessible with the `user:email` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-public-email-addresses-for-the-authenticated-user>
     */
    pub async fn list_all_public_emails_for_authenticated(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::Email>>> {
        let url = self.client.url("/user/public_emails", None);
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
     * List users.
     *
     * This function performs a `GET` to the `/users` endpoint.
     *
     * Lists all users, in the order that they signed up on GitHub. This list includes personal user accounts and organization accounts.
     *
     * Note: Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/overview/resources-in-the-rest-api#link-header) to get the URL for the next page of users.
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-users>
     *
     * **Parameters:**
     *
     * * `since: i64` -- A user ID. Only return users with an ID greater than this ID.
     * * `per_page: i64` -- Results per page (max 100).
     */
    pub async fn list(
        &self,
        since: i64,
        per_page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if since > 0 {
            query_args.push(("since".to_string(), since.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/users?{}", query_), None);
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
     * List users.
     *
     * This function performs a `GET` to the `/users` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Lists all users, in the order that they signed up on GitHub. This list includes personal user accounts and organization accounts.
     *
     * Note: Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/overview/resources-in-the-rest-api#link-header) to get the URL for the next page of users.
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-users>
     */
    pub async fn list_all(
        &self,
        since: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if since > 0 {
            query_args.push(("since".to_string(), since.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/users?{}", query_), None);
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
     * Get a user.
     *
     * This function performs a `GET` to the `/users/{username}` endpoint.
     *
     * Provides publicly available information about someone with a GitHub account.
     *
     * GitHub Apps with the `Plan` user permission can use this endpoint to retrieve information about a user's GitHub plan. The GitHub App must be authenticated as a user. See "[Identifying and authorizing users for GitHub Apps](https://docs.github.com/apps/building-github-apps/identifying-and-authorizing-users-for-github-apps/)" for details about authentication. For an example response, see 'Response with GitHub plan information' below"
     *
     * The `email` key in the following response is the publicly visible email address from your GitHub [profile page](https://github.com/settings/profile). When setting up your profile, you can select a primary email address to be “public” which provides an email entry for this endpoint. If you do not set a public email address for `email`, then it will have a value of `null`. You only see publicly visible email addresses when authenticated with GitHub. For more information, see [Authentication](https://docs.github.com/rest/overview/resources-in-the-rest-api#authentication).
     *
     * The Emails API enables you to list all of your email addresses, and toggle a primary email to be visible publicly. For more information, see "[Emails API](https://docs.github.com/rest/reference/users#emails)".
     *
     * FROM: <https://docs.github.com/rest/reference/users#get-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     */
    pub async fn get_by_username_public_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::PublicUser>> {
        let url = self.client.url(
            &format!(
                "/users/{}",
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
    /**
     * Get a user.
     *
     * This function performs a `GET` to the `/users/{username}` endpoint.
     *
     * Provides publicly available information about someone with a GitHub account.
     *
     * GitHub Apps with the `Plan` user permission can use this endpoint to retrieve information about a user's GitHub plan. The GitHub App must be authenticated as a user. See "[Identifying and authorizing users for GitHub Apps](https://docs.github.com/apps/building-github-apps/identifying-and-authorizing-users-for-github-apps/)" for details about authentication. For an example response, see 'Response with GitHub plan information' below"
     *
     * The `email` key in the following response is the publicly visible email address from your GitHub [profile page](https://github.com/settings/profile). When setting up your profile, you can select a primary email address to be “public” which provides an email entry for this endpoint. If you do not set a public email address for `email`, then it will have a value of `null`. You only see publicly visible email addresses when authenticated with GitHub. For more information, see [Authentication](https://docs.github.com/rest/overview/resources-in-the-rest-api#authentication).
     *
     * The Emails API enables you to list all of your email addresses, and toggle a primary email to be visible publicly. For more information, see "[Emails API](https://docs.github.com/rest/reference/users#emails)".
     *
     * FROM: <https://docs.github.com/rest/reference/users#get-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     */
    pub async fn get_by_username_private_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::PrivateUser>> {
        let url = self.client.url(
            &format!(
                "/users/{}",
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
    /**
     * Get a user.
     *
     * This function performs a `GET` to the `/users/{username}` endpoint.
     *
     * Provides publicly available information about someone with a GitHub account.
     *
     * GitHub Apps with the `Plan` user permission can use this endpoint to retrieve information about a user's GitHub plan. The GitHub App must be authenticated as a user. See "[Identifying and authorizing users for GitHub Apps](https://docs.github.com/apps/building-github-apps/identifying-and-authorizing-users-for-github-apps/)" for details about authentication. For an example response, see 'Response with GitHub plan information' below"
     *
     * The `email` key in the following response is the publicly visible email address from your GitHub [profile page](https://github.com/settings/profile). When setting up your profile, you can select a primary email address to be “public” which provides an email entry for this endpoint. If you do not set a public email address for `email`, then it will have a value of `null`. You only see publicly visible email addresses when authenticated with GitHub. For more information, see [Authentication](https://docs.github.com/rest/overview/resources-in-the-rest-api#authentication).
     *
     * The Emails API enables you to list all of your email addresses, and toggle a primary email to be visible publicly. For more information, see "[Emails API](https://docs.github.com/rest/reference/users#emails)".
     *
     * FROM: <https://docs.github.com/rest/reference/users#get-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     */
    pub async fn get_by_username(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::UsersGetByUsernameResponseOneOf>> {
        let url = self.client.url(
            &format!(
                "/users/{}",
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
    /**
     * List followers of a user.
     *
     * This function performs a `GET` to the `/users/{username}/followers` endpoint.
     *
     * Lists the people following the specified user.
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-followers-of-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_followers_for_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
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
                "/users/{}/followers?{}",
                crate::progenitor_support::encode_path(username),
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
     * List followers of a user.
     *
     * This function performs a `GET` to the `/users/{username}/followers` endpoint.
     *
     * As opposed to `list_followers_for_user`, this function returns all the pages of the request at once.
     *
     * Lists the people following the specified user.
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-followers-of-a-user>
     */
    pub async fn list_all_followers_for_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/followers",
                crate::progenitor_support::encode_path(username),
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
     * List the people a user follows.
     *
     * This function performs a `GET` to the `/users/{username}/following` endpoint.
     *
     * Lists the people who the specified user follows.
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-the-people-a-user-follows>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_following_for_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
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
                "/users/{}/following?{}",
                crate::progenitor_support::encode_path(username),
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
     * List the people a user follows.
     *
     * This function performs a `GET` to the `/users/{username}/following` endpoint.
     *
     * As opposed to `list_following_for_user`, this function returns all the pages of the request at once.
     *
     * Lists the people who the specified user follows.
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-the-people-a-user-follows>
     */
    pub async fn list_all_following_for_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/following",
                crate::progenitor_support::encode_path(username),
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
     * Check if a user follows another user.
     *
     * This function performs a `GET` to the `/users/{username}/following/{target_user}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/users#check-if-a-user-follows-another-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `target_user: &str`
     */
    pub async fn check_following_for_user(
        &self,
        username: &str,
        target_user: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/users/{}/following/{}",
                crate::progenitor_support::encode_path(username),
                crate::progenitor_support::encode_path(target_user),
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
     * List GPG keys for a user.
     *
     * This function performs a `GET` to the `/users/{username}/gpg_keys` endpoint.
     *
     * Lists the GPG keys for a user. This information is accessible by anyone.
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-gpg-keys-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_gpg_keys_for_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::GpgKey>>> {
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
                "/users/{}/gpg_keys?{}",
                crate::progenitor_support::encode_path(username),
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
     * List GPG keys for a user.
     *
     * This function performs a `GET` to the `/users/{username}/gpg_keys` endpoint.
     *
     * As opposed to `list_gpg_keys_for_user`, this function returns all the pages of the request at once.
     *
     * Lists the GPG keys for a user. This information is accessible by anyone.
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-gpg-keys-for-a-user>
     */
    pub async fn list_all_gpg_keys_for_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::GpgKey>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/gpg_keys",
                crate::progenitor_support::encode_path(username),
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
     * Get contextual information for a user.
     *
     * This function performs a `GET` to the `/users/{username}/hovercard` endpoint.
     *
     * Provides hovercard information when authenticated through basic auth or OAuth with the `repo` scope. You can find out more about someone in relation to their pull requests, issues, repositories, and organizations.
     *
     * The `subject_type` and `subject_id` parameters provide context for the person's hovercard, which returns more information than without the parameters. For example, if you wanted to find out more about `octocat` who owns the `Spoon-Knife` repository via cURL, it would look like this:
     *
     * ```shell
     *  curl -u username:token
     *   https://api.github.com/users/octocat/hovercard?subject_type=repository&subject_id=1300192
     * ```
     *
     * FROM: <https://docs.github.com/rest/reference/users#get-contextual-information-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `subject_type: crate::types::SubjectType` -- Identifies which additional information you'd like to receive about the person's hovercard. Can be `organization`, `repository`, `issue`, `pull_request`. \*\*Required\*\* when using `subject_id`.
     * * `subject_id: &str` -- Uses the ID for the `subject_type` you specified. **Required** when using `subject_type`.
     */
    pub async fn get_context_for_user(
        &self,
        username: &str,
        subject_type: crate::types::SubjectType,
        subject_id: &str,
    ) -> ClientResult<crate::Response<crate::types::Hovercard>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !subject_id.is_empty() {
            query_args.push(("subject_id".to_string(), subject_id.to_string()));
        }
        if !subject_type.to_string().is_empty() {
            query_args.push(("subject_type".to_string(), subject_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/hovercard?{}",
                crate::progenitor_support::encode_path(username),
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
     * List public keys for a user.
     *
     * This function performs a `GET` to the `/users/{username}/keys` endpoint.
     *
     * Lists the _verified_ public SSH keys for a user. This is accessible by anyone.
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-public-keys-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_public_keys_for_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::KeySimple>>> {
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
                "/users/{}/keys?{}",
                crate::progenitor_support::encode_path(username),
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
     * List public keys for a user.
     *
     * This function performs a `GET` to the `/users/{username}/keys` endpoint.
     *
     * As opposed to `list_public_keys_for_user`, this function returns all the pages of the request at once.
     *
     * Lists the _verified_ public SSH keys for a user. This is accessible by anyone.
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-public-keys-for-a-user>
     */
    pub async fn list_all_public_keys_for_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::KeySimple>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/keys",
                crate::progenitor_support::encode_path(username),
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
