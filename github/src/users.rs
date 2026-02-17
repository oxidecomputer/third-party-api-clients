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
     * OAuth app tokens and personal access tokens (classic) need the `user` scope in order for the response to include private profile information.
     *
     * FROM: <https://docs.github.com/rest/users/users#get-the-authenticated-user>
     */
    pub async fn get_authenticated_public_user(
        &self,
    ) -> ClientResult<crate::Response<crate::types::PublicUser>> {
        let url = self.client.url(&"/user".to_string(), None);
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
     * OAuth app tokens and personal access tokens (classic) need the `user` scope in order for the response to include private profile information.
     *
     * FROM: <https://docs.github.com/rest/users/users#get-the-authenticated-user>
     */
    pub async fn get_authenticated_private_user(
        &self,
    ) -> ClientResult<crate::Response<crate::types::PrivateUser>> {
        let url = self.client.url(&"/user".to_string(), None);
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
     * OAuth app tokens and personal access tokens (classic) need the `user` scope in order for the response to include private profile information.
     *
     * FROM: <https://docs.github.com/rest/users/users#get-the-authenticated-user>
     */
    pub async fn get_authenticated(
        &self,
    ) -> ClientResult<crate::Response<crate::types::UsersGetByResponseOneOf>> {
        let url = self.client.url(&"/user".to_string(), None);
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
     * FROM: <https://docs.github.com/rest/users/users#update-the-authenticated-user>
     */
    pub async fn update_authenticated(
        &self,
        body: &crate::types::UsersUpdateAuthenticatedRequest,
    ) -> ClientResult<crate::Response<crate::types::PrivateUser>> {
        let url = self.client.url(&"/user".to_string(), None);
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
     * FROM: <https://docs.github.com/rest/users/blocking#list-users-blocked-by-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_blocked_by_authenticated_user(
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
        let url = self.client.url(&format!("/user/blocks?{}", query_), None);
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
     * As opposed to `list_blocked_by_authenticated_user`, this function returns all the pages of the request at once.
     *
     * List the users you've blocked on your personal account.
     *
     * FROM: <https://docs.github.com/rest/users/blocking#list-users-blocked-by-the-authenticated-user>
     */
    pub async fn list_all_blocked_by_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(&"/user/blocks".to_string(), None);
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
     * Returns a 204 if the given user is blocked by the authenticated user. Returns a 404 if the given user is not blocked by the authenticated user, or if the given user account has been identified as spam by GitHub.
     *
     * FROM: <https://docs.github.com/rest/users/blocking#check-if-a-user-is-blocked-by-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn check_blocked(&self, username: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/blocks/{}",
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
    /**
     * Block a user.
     *
     * This function performs a `PUT` to the `/user/blocks/{username}` endpoint.
     *
     * Blocks the given user and returns a 204. If the authenticated user cannot block the given user a 422 is returned.
     *
     * FROM: <https://docs.github.com/rest/users/blocking#block-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn block(&self, username: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/blocks/{}",
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Unblocks the given user and returns a 204.
     *
     * FROM: <https://docs.github.com/rest/users/blocking#unblock-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn unblock(&self, username: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/blocks/{}",
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * FROM: <https://docs.github.com/rest/users/emails#set-primary-email-visibility-for-the-authenticated-user>
     */
    pub async fn set_primary_email_visibility_for_authenticated_user(
        &self,
        body: &crate::types::UsersSetPrimaryEmailVisibilityRequest,
    ) -> ClientResult<crate::Response<Vec<crate::types::Email>>> {
        let url = self.client.url(&"/user/email/visibility".to_string(), None);
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
     * Lists all of your email addresses, and specifies which one is visible
     * to the public.
     *
     * OAuth app tokens and personal access tokens (classic) need the `user:email` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/emails#list-email-addresses-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_emails_for_authenticated_user(
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
     * As opposed to `list_emails_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * Lists all of your email addresses, and specifies which one is visible
     * to the public.
     *
     * OAuth app tokens and personal access tokens (classic) need the `user:email` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/emails#list-email-addresses-for-the-authenticated-user>
     */
    pub async fn list_all_emails_for_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::Email>>> {
        let url = self.client.url(&"/user/emails".to_string(), None);
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
     * OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/emails#add-an-email-address-for-the-authenticated-user>
     */
    pub async fn add_email_for_authenticated_user(
        &self,
        body: &crate::types::UsersAddEmailRequestOneOf,
    ) -> ClientResult<crate::Response<Vec<crate::types::Email>>> {
        let url = self.client.url(&"/user/emails".to_string(), None);
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
     * OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/emails#delete-an-email-address-for-the-authenticated-user>
     */
    pub async fn delete_email_for_authenticated_user(
        &self,
        body: &crate::types::UsersAddEmailRequestOneOf,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(&"/user/emails".to_string(), None);
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
     * FROM: <https://docs.github.com/rest/users/followers#list-followers-of-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
     * FROM: <https://docs.github.com/rest/users/followers#list-followers-of-the-authenticated-user>
     */
    pub async fn list_all_followers_for_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(&"/user/followers".to_string(), None);
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
     * FROM: <https://docs.github.com/rest/users/followers#list-the-people-the-authenticated-user-follows>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_followed_by_authenticated_user(
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
     * As opposed to `list_followed_by_authenticated_user`, this function returns all the pages of the request at once.
     *
     * Lists the people who the authenticated user follows.
     *
     * FROM: <https://docs.github.com/rest/users/followers#list-the-people-the-authenticated-user-follows>
     */
    pub async fn list_all_followed_by_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(&"/user/following".to_string(), None);
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
     * FROM: <https://docs.github.com/rest/users/followers#check-if-a-person-is-followed-by-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn check_person_is_followed_by_authenticated(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/following/{}",
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
    /**
     * Follow a user.
     *
     * This function performs a `PUT` to the `/user/following/{username}` endpoint.
     *
     * Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `user:follow` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/followers#follow-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn follow(&self, username: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/following/{}",
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * OAuth app tokens and personal access tokens (classic) need the `user:follow` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/followers#unfollow-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn unfollow(&self, username: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/following/{}",
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Lists the current user's GPG keys.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:gpg_key` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/gpg-keys#list-gpg-keys-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_gpg_keys_for_authenticated_user(
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
     * As opposed to `list_gpg_keys_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * Lists the current user's GPG keys.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:gpg_key` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/gpg-keys#list-gpg-keys-for-the-authenticated-user>
     */
    pub async fn list_all_gpg_keys_for_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::GpgKey>>> {
        let url = self.client.url(&"/user/gpg_keys".to_string(), None);
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
     * Adds a GPG key to the authenticated user's GitHub account.
     *
     * OAuth app tokens and personal access tokens (classic) need the `write:gpg_key` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/gpg-keys#create-a-gpg-key-for-the-authenticated-user>
     */
    pub async fn create_gpg_key_for_authenticated_user(
        &self,
        body: &crate::types::UsersCreateGpgKeyRequest,
    ) -> ClientResult<crate::Response<crate::types::GpgKey>> {
        let url = self.client.url(&"/user/gpg_keys".to_string(), None);
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
     * View extended details for a single GPG key.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:gpg_key` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/gpg-keys#get-a-gpg-key-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `gpg_key_id: i64` -- The unique identifier of the GPG key.
     */
    pub async fn get_gpg_key_for_authenticated_user(
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
     * Removes a GPG key from the authenticated user's GitHub account.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:gpg_key` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/gpg-keys#delete-a-gpg-key-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `gpg_key_id: i64` -- The unique identifier of the GPG key.
     */
    pub async fn delete_gpg_key_for_authenticated_user(
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
     * Lists the public SSH keys for the authenticated user's GitHub account.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:public_key` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/keys#list-public-ssh-keys-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_public_ssh_keys_for_authenticated_user(
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
     * As opposed to `list_public_ssh_keys_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * Lists the public SSH keys for the authenticated user's GitHub account.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:public_key` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/keys#list-public-ssh-keys-for-the-authenticated-user>
     */
    pub async fn list_all_public_ssh_keys_for_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::Key>>> {
        let url = self.client.url(&"/user/keys".to_string(), None);
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
     * Adds a public SSH key to the authenticated user's GitHub account.
     *
     * OAuth app tokens and personal access tokens (classic) need the `write:public_key` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/keys#create-a-public-ssh-key-for-the-authenticated-user>
     */
    pub async fn create_public_ssh_key_for_authenticated_user(
        &self,
        body: &crate::types::UsersCreatePublicSshKeyRequest,
    ) -> ClientResult<crate::Response<crate::types::Key>> {
        let url = self.client.url(&"/user/keys".to_string(), None);
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
     * View extended details for a single public SSH key.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:public_key` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/keys#get-a-public-ssh-key-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `key_id: i64` -- The unique identifier of the key.
     */
    pub async fn get_public_ssh_key_for_authenticated_user(
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
     * Removes a public SSH key from the authenticated user's GitHub account.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:public_key` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/keys#delete-a-public-ssh-key-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `key_id: i64` -- The unique identifier of the key.
     */
    pub async fn delete_public_ssh_key_for_authenticated_user(
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
     * Lists your publicly visible email address, which you can set with the
     * [Set primary email visibility for the authenticated user](https://docs.github.com/rest/users/emails#set-primary-email-visibility-for-the-authenticated-user)
     * endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `user:email` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/emails#list-public-email-addresses-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_public_emails_for_authenticated_user(
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
     * As opposed to `list_public_emails_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * Lists your publicly visible email address, which you can set with the
     * [Set primary email visibility for the authenticated user](https://docs.github.com/rest/users/emails#set-primary-email-visibility-for-the-authenticated-user)
     * endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `user:email` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/emails#list-public-email-addresses-for-the-authenticated-user>
     */
    pub async fn list_all_public_emails_for_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::Email>>> {
        let url = self.client.url(&"/user/public_emails".to_string(), None);
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
     * List social accounts for the authenticated user.
     *
     * This function performs a `GET` to the `/user/social_accounts` endpoint.
     *
     * Lists all of your social accounts.
     *
     * FROM: <https://docs.github.com/rest/users/social-accounts#list-social-accounts-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_social_accounts_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SocialAccount>>> {
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
            .url(&format!("/user/social_accounts?{}", query_), None);
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
     * List social accounts for the authenticated user.
     *
     * This function performs a `GET` to the `/user/social_accounts` endpoint.
     *
     * As opposed to `list_social_accounts_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * Lists all of your social accounts.
     *
     * FROM: <https://docs.github.com/rest/users/social-accounts#list-social-accounts-for-the-authenticated-user>
     */
    pub async fn list_all_social_accounts_for_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::SocialAccount>>> {
        let url = self.client.url(&"/user/social_accounts".to_string(), None);
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
     * Add social accounts for the authenticated user.
     *
     * This function performs a `POST` to the `/user/social_accounts` endpoint.
     *
     * Add one or more social accounts to the authenticated user's profile.
     *
     * OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/social-accounts#add-social-accounts-for-the-authenticated-user>
     */
    pub async fn add_social_account_for_authenticated_user(
        &self,
        body: &crate::types::UsersAddSocialAccountRequest,
    ) -> ClientResult<crate::Response<Vec<crate::types::SocialAccount>>> {
        let url = self.client.url(&"/user/social_accounts".to_string(), None);
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
     * Delete social accounts for the authenticated user.
     *
     * This function performs a `DELETE` to the `/user/social_accounts` endpoint.
     *
     * Deletes one or more social accounts from the authenticated user's profile.
     *
     * OAuth app tokens and personal access tokens (classic) need the `user` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/social-accounts#delete-social-accounts-for-the-authenticated-user>
     */
    pub async fn delete_social_account_for_authenticated_user(
        &self,
        body: &crate::types::UsersAddSocialAccountRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(&"/user/social_accounts".to_string(), None);
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
     * List SSH signing keys for the authenticated user.
     *
     * This function performs a `GET` to the `/user/ssh_signing_keys` endpoint.
     *
     * Lists the SSH signing keys for the authenticated user's GitHub account.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:ssh_signing_key` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/ssh-signing-keys#list-ssh-signing-keys-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_ssh_signing_keys_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SshSigningKey>>> {
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
            .url(&format!("/user/ssh_signing_keys?{}", query_), None);
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
     * List SSH signing keys for the authenticated user.
     *
     * This function performs a `GET` to the `/user/ssh_signing_keys` endpoint.
     *
     * As opposed to `list_ssh_signing_keys_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * Lists the SSH signing keys for the authenticated user's GitHub account.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:ssh_signing_key` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/ssh-signing-keys#list-ssh-signing-keys-for-the-authenticated-user>
     */
    pub async fn list_all_ssh_signing_keys_for_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::SshSigningKey>>> {
        let url = self.client.url(&"/user/ssh_signing_keys".to_string(), None);
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
     * Create a SSH signing key for the authenticated user.
     *
     * This function performs a `POST` to the `/user/ssh_signing_keys` endpoint.
     *
     * Creates an SSH signing key for the authenticated user's GitHub account.
     *
     * OAuth app tokens and personal access tokens (classic) need the `write:ssh_signing_key` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/ssh-signing-keys#create-a-ssh-signing-key-for-the-authenticated-user>
     */
    pub async fn create_ssh_signing_key_for_authenticated_user(
        &self,
        body: &crate::types::UsersCreatePublicSshKeyRequest,
    ) -> ClientResult<crate::Response<crate::types::SshSigningKey>> {
        let url = self.client.url(&"/user/ssh_signing_keys".to_string(), None);
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
     * Get an SSH signing key for the authenticated user.
     *
     * This function performs a `GET` to the `/user/ssh_signing_keys/{ssh_signing_key_id}` endpoint.
     *
     * Gets extended details for an SSH signing key.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:ssh_signing_key` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/ssh-signing-keys#get-an-ssh-signing-key-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `ssh_signing_key_id: i64` -- The unique identifier of the SSH signing key.
     */
    pub async fn get_ssh_signing_key_for_authenticated_user(
        &self,
        ssh_signing_key_id: i64,
    ) -> ClientResult<crate::Response<crate::types::SshSigningKey>> {
        let url = self.client.url(
            &format!(
                "/user/ssh_signing_keys/{}",
                crate::progenitor_support::encode_path(&ssh_signing_key_id.to_string()),
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
     * Delete an SSH signing key for the authenticated user.
     *
     * This function performs a `DELETE` to the `/user/ssh_signing_keys/{ssh_signing_key_id}` endpoint.
     *
     * Deletes an SSH signing key from the authenticated user's GitHub account.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:ssh_signing_key` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/ssh-signing-keys#delete-an-ssh-signing-key-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `ssh_signing_key_id: i64` -- The unique identifier of the SSH signing key.
     */
    pub async fn delete_ssh_signing_key_for_authenticated_user(
        &self,
        ssh_signing_key_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/ssh_signing_keys/{}",
                crate::progenitor_support::encode_path(&ssh_signing_key_id.to_string()),
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
     * Get a user using their ID.
     *
     * This function performs a `GET` to the `/user/{account_id}` endpoint.
     *
     * Provides publicly available information about someone with a GitHub account. This method takes their durable user `ID` instead of their `login`, which can change over time.
     *
     * If you are requesting information about an [Enterprise Managed User](https://docs.github.com/enterprise-cloud@latest/admin/managing-iam/understanding-iam-for-enterprises/about-enterprise-managed-users), or a GitHub App bot that is installed in an organization that uses Enterprise Managed Users, your requests must be authenticated as a user or GitHub App that has access to the organization to view that account's information. If you are not authorized, the request will return a `404 Not Found` status.
     *
     * The `email` key in the following response is the publicly visible email address from your GitHub [profile page](https://github.com/settings/profile). When setting up your profile, you can select a primary email address to be public which provides an email entry for this endpoint. If you do not set a public email address for `email`, then it will have a value of `null`. You only see publicly visible email addresses when authenticated with GitHub. For more information, see [Authentication](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#authentication).
     *
     * The Emails API enables you to list all of your email addresses, and toggle a primary email to be visible publicly. For more information, see [Emails API](https://docs.github.com/rest/users/emails).
     *
     * FROM: <https://docs.github.com/rest/users/users#get-a-user-using-their-id>
     *
     * **Parameters:**
     *
     * * `account_id: i64` -- account_id parameter.
     */
    pub async fn get_by_public_user(
        &self,
        account_id: i64,
    ) -> ClientResult<crate::Response<crate::types::PublicUser>> {
        let url = self.client.url(
            &format!(
                "/user/{}",
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
     * Get a user using their ID.
     *
     * This function performs a `GET` to the `/user/{account_id}` endpoint.
     *
     * Provides publicly available information about someone with a GitHub account. This method takes their durable user `ID` instead of their `login`, which can change over time.
     *
     * If you are requesting information about an [Enterprise Managed User](https://docs.github.com/enterprise-cloud@latest/admin/managing-iam/understanding-iam-for-enterprises/about-enterprise-managed-users), or a GitHub App bot that is installed in an organization that uses Enterprise Managed Users, your requests must be authenticated as a user or GitHub App that has access to the organization to view that account's information. If you are not authorized, the request will return a `404 Not Found` status.
     *
     * The `email` key in the following response is the publicly visible email address from your GitHub [profile page](https://github.com/settings/profile). When setting up your profile, you can select a primary email address to be public which provides an email entry for this endpoint. If you do not set a public email address for `email`, then it will have a value of `null`. You only see publicly visible email addresses when authenticated with GitHub. For more information, see [Authentication](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#authentication).
     *
     * The Emails API enables you to list all of your email addresses, and toggle a primary email to be visible publicly. For more information, see [Emails API](https://docs.github.com/rest/users/emails).
     *
     * FROM: <https://docs.github.com/rest/users/users#get-a-user-using-their-id>
     *
     * **Parameters:**
     *
     * * `account_id: i64` -- account_id parameter.
     */
    pub async fn get_by_private_user(
        &self,
        account_id: i64,
    ) -> ClientResult<crate::Response<crate::types::PrivateUser>> {
        let url = self.client.url(
            &format!(
                "/user/{}",
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
     * Get a user using their ID.
     *
     * This function performs a `GET` to the `/user/{account_id}` endpoint.
     *
     * Provides publicly available information about someone with a GitHub account. This method takes their durable user `ID` instead of their `login`, which can change over time.
     *
     * If you are requesting information about an [Enterprise Managed User](https://docs.github.com/enterprise-cloud@latest/admin/managing-iam/understanding-iam-for-enterprises/about-enterprise-managed-users), or a GitHub App bot that is installed in an organization that uses Enterprise Managed Users, your requests must be authenticated as a user or GitHub App that has access to the organization to view that account's information. If you are not authorized, the request will return a `404 Not Found` status.
     *
     * The `email` key in the following response is the publicly visible email address from your GitHub [profile page](https://github.com/settings/profile). When setting up your profile, you can select a primary email address to be public which provides an email entry for this endpoint. If you do not set a public email address for `email`, then it will have a value of `null`. You only see publicly visible email addresses when authenticated with GitHub. For more information, see [Authentication](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#authentication).
     *
     * The Emails API enables you to list all of your email addresses, and toggle a primary email to be visible publicly. For more information, see [Emails API](https://docs.github.com/rest/users/emails).
     *
     * FROM: <https://docs.github.com/rest/users/users#get-a-user-using-their-id>
     *
     * **Parameters:**
     *
     * * `account_id: i64` -- account_id parameter.
     */
    pub async fn get_by_id(
        &self,
        account_id: i64,
    ) -> ClientResult<crate::Response<crate::types::UsersGetByResponseOneOf>> {
        let url = self.client.url(
            &format!(
                "/user/{}",
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
     * List users.
     *
     * This function performs a `GET` to the `/users` endpoint.
     *
     * Lists all users, in the order that they signed up on GitHub. This list includes personal user accounts and organization accounts.
     *
     * Note: Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers) to get the URL for the next page of users.
     *
     * FROM: <https://docs.github.com/rest/users/users#list-users>
     *
     * **Parameters:**
     *
     * * `since: i64` -- A user ID. Only return users with an ID greater than this ID.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
     * Note: Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers) to get the URL for the next page of users.
     *
     * FROM: <https://docs.github.com/rest/users/users#list-users>
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
     * If you are requesting information about an [Enterprise Managed User](https://docs.github.com/enterprise-cloud@latest/admin/managing-iam/understanding-iam-for-enterprises/about-enterprise-managed-users), or a GitHub App bot that is installed in an organization that uses Enterprise Managed Users, your requests must be authenticated as a user or GitHub App that has access to the organization to view that account's information. If you are not authorized, the request will return a `404 Not Found` status.
     *
     * The `email` key in the following response is the publicly visible email address from your GitHub [profile page](https://github.com/settings/profile). When setting up your profile, you can select a primary email address to be public which provides an email entry for this endpoint. If you do not set a public email address for `email`, then it will have a value of `null`. You only see publicly visible email addresses when authenticated with GitHub. For more information, see [Authentication](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#authentication).
     *
     * The Emails API enables you to list all of your email addresses, and toggle a primary email to be visible publicly. For more information, see [Emails API](https://docs.github.com/rest/users/emails).
     *
     * FROM: <https://docs.github.com/rest/users/users#get-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn get_by_username_public_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::PublicUser>> {
        let url = self.client.url(
            &format!(
                "/users/{}",
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
    /**
     * Get a user.
     *
     * This function performs a `GET` to the `/users/{username}` endpoint.
     *
     * Provides publicly available information about someone with a GitHub account.
     *
     * If you are requesting information about an [Enterprise Managed User](https://docs.github.com/enterprise-cloud@latest/admin/managing-iam/understanding-iam-for-enterprises/about-enterprise-managed-users), or a GitHub App bot that is installed in an organization that uses Enterprise Managed Users, your requests must be authenticated as a user or GitHub App that has access to the organization to view that account's information. If you are not authorized, the request will return a `404 Not Found` status.
     *
     * The `email` key in the following response is the publicly visible email address from your GitHub [profile page](https://github.com/settings/profile). When setting up your profile, you can select a primary email address to be public which provides an email entry for this endpoint. If you do not set a public email address for `email`, then it will have a value of `null`. You only see publicly visible email addresses when authenticated with GitHub. For more information, see [Authentication](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#authentication).
     *
     * The Emails API enables you to list all of your email addresses, and toggle a primary email to be visible publicly. For more information, see [Emails API](https://docs.github.com/rest/users/emails).
     *
     * FROM: <https://docs.github.com/rest/users/users#get-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn get_by_username_private_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::PrivateUser>> {
        let url = self.client.url(
            &format!(
                "/users/{}",
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
    /**
     * Get a user.
     *
     * This function performs a `GET` to the `/users/{username}` endpoint.
     *
     * Provides publicly available information about someone with a GitHub account.
     *
     * If you are requesting information about an [Enterprise Managed User](https://docs.github.com/enterprise-cloud@latest/admin/managing-iam/understanding-iam-for-enterprises/about-enterprise-managed-users), or a GitHub App bot that is installed in an organization that uses Enterprise Managed Users, your requests must be authenticated as a user or GitHub App that has access to the organization to view that account's information. If you are not authorized, the request will return a `404 Not Found` status.
     *
     * The `email` key in the following response is the publicly visible email address from your GitHub [profile page](https://github.com/settings/profile). When setting up your profile, you can select a primary email address to be public which provides an email entry for this endpoint. If you do not set a public email address for `email`, then it will have a value of `null`. You only see publicly visible email addresses when authenticated with GitHub. For more information, see [Authentication](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#authentication).
     *
     * The Emails API enables you to list all of your email addresses, and toggle a primary email to be visible publicly. For more information, see [Emails API](https://docs.github.com/rest/users/emails).
     *
     * FROM: <https://docs.github.com/rest/users/users#get-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn get_by_username(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::UsersGetByResponseOneOf>> {
        let url = self.client.url(
            &format!(
                "/users/{}",
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
    /**
     * List attestations by bulk subject digests.
     *
     * This function performs a `POST` to the `/users/{username}/attestations/bulk-list` endpoint.
     *
     * List a collection of artifact attestations associated with any entry in a list of subject digests owned by a user.
     *
     * The collection of attestations returned by this endpoint is filtered according to the authenticated user's permissions; if the authenticated user cannot read a repository, the attestations associated with that repository will not be included in the response. In addition, when using a fine-grained access token the `attestations:read` permission is required.
     *
     * **Please note:** in order to offer meaningful security benefits, an attestation's signature and timestamps **must** be cryptographically verified, and the identity of the attestation signer **must** be validated. Attestations can be verified using the [GitHub CLI `attestation verify` command](https://cli.github.com/manual/gh_attestation_verify). For more information, see [our guide on how to use artifact attestations to establish a build's provenance](https://docs.github.com/actions/security-guides/using-artifact-attestations-to-establish-provenance-for-builds).
     *
     * FROM: <https://docs.github.com/rest/users/attestations#list-attestations-by-bulk-subject-digests>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn list_attestations_bulk(
        &self,
        per_page: i64,
        before: &str,
        after: &str,
        username: &str,
        body: &crate::types::OrgsListAttestationsBulkRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgsListAttestationsBulkResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/attestations/bulk-list?{}",
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Delete attestations in bulk.
     *
     * This function performs a `POST` to the `/users/{username}/attestations/delete-request` endpoint.
     *
     * Delete artifact attestations in bulk by either subject digests or unique ID.
     *
     * FROM: <https://docs.github.com/rest/users/attestations#delete-attestations-in-bulk>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn delete_attestations_bulk(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/users/{}/attestations/delete-request",
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Delete attestations by subject digest.
     *
     * This function performs a `DELETE` to the `/users/{username}/attestations/digest/{subject_digest}` endpoint.
     *
     * Delete an artifact attestation by subject digest.
     *
     * FROM: <https://docs.github.com/rest/users/attestations#delete-attestations-by-subject-digest>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     * * `subject_digest: &str` -- Subject Digest.
     */
    pub async fn delete_attestations_by_subject_digest(
        &self,
        username: &str,
        subject_digest: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/users/{}/attestations/digest/{}",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&subject_digest.to_string()),
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
     * Delete attestations by ID.
     *
     * This function performs a `DELETE` to the `/users/{username}/attestations/{attestation_id}` endpoint.
     *
     * Delete an artifact attestation by unique ID that is associated with a repository owned by a user.
     *
     * FROM: <https://docs.github.com/rest/users/attestations#delete-attestations-by-id>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     * * `attestation_id: i64` -- Attestation ID.
     */
    pub async fn delete_attestations_by_id(
        &self,
        username: &str,
        attestation_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/users/{}/attestations/{}",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&attestation_id.to_string()),
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
     * List attestations.
     *
     * This function performs a `GET` to the `/users/{username}/attestations/{subject_digest}` endpoint.
     *
     * List a collection of artifact attestations with a given subject digest that are associated with repositories owned by a user.
     *
     * The collection of attestations returned by this endpoint is filtered according to the authenticated user's permissions; if the authenticated user cannot read a repository, the attestations associated with that repository will not be included in the response. In addition, when using a fine-grained access token the `attestations:read` permission is required.
     *
     * **Please note:** in order to offer meaningful security benefits, an attestation's signature and timestamps **must** be cryptographically verified, and the identity of the attestation signer **must** be validated. Attestations can be verified using the [GitHub CLI `attestation verify` command](https://cli.github.com/manual/gh_attestation_verify). For more information, see [our guide on how to use artifact attestations to establish a build's provenance](https://docs.github.com/actions/security-guides/using-artifact-attestations-to-establish-provenance-for-builds).
     *
     * FROM: <https://docs.github.com/rest/users/attestations#list-attestations>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `username: &str` -- The handle for the GitHub user account.
     * * `subject_digest: &str` -- Subject Digest.
     * * `predicate_type: &str` -- Optional filter for fetching attestations with a given predicate type.
     *   This option accepts `provenance`, `sbom`, `release`, or freeform text
     *   for custom predicate types.
     */
    pub async fn list_attestations(
        &self,
        per_page: i64,
        before: &str,
        after: &str,
        username: &str,
        subject_digest: &str,
        predicate_type: &str,
    ) -> ClientResult<crate::Response<crate::types::UsersListAttestationsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !predicate_type.is_empty() {
            query_args.push(("predicate_type".to_string(), predicate_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/attestations/{}?{}",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&subject_digest.to_string()),
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
     * Lists the people following the specified user.
     *
     * FROM: <https://docs.github.com/rest/users/followers#list-followers-of-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * FROM: <https://docs.github.com/rest/users/followers#list-followers-of-a-user>
     */
    pub async fn list_all_followers_for_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/followers",
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * FROM: <https://docs.github.com/rest/users/followers#list-the-people-a-user-follows>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * FROM: <https://docs.github.com/rest/users/followers#list-the-people-a-user-follows>
     */
    pub async fn list_all_following_for_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/following",
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * FROM: <https://docs.github.com/rest/users/followers#check-if-a-user-follows-another-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
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
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&target_user.to_string()),
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
     * FROM: <https://docs.github.com/rest/users/gpg-keys#list-gpg-keys-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * FROM: <https://docs.github.com/rest/users/gpg-keys#list-gpg-keys-for-a-user>
     */
    pub async fn list_all_gpg_keys_for_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::GpgKey>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/gpg_keys",
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Provides hovercard information. You can find out more about someone in relation to their pull requests, issues, repositories, and organizations.
     *
     *   The `subject_type` and `subject_id` parameters provide context for the person's hovercard, which returns more information than without the parameters. For example, if you wanted to find out more about `octocat` who owns the `Spoon-Knife` repository, you would use a `subject_type` value of `repository` and a `subject_id` value of `1300192` (the ID of the `Spoon-Knife` repository).
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/users/users#get-contextual-information-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     * * `subject_type: crate::types::UsersGetContextUserSubjectType` -- Identifies which additional information you'd like to receive about the person's hovercard. Can be `organization`, `repository`, `issue`, `pull_request`. \*\*Required\*\* when using `subject_id`.
     * * `subject_id: &str` -- Uses the ID for the `subject_type` you specified. **Required** when using `subject_type`.
     */
    pub async fn get_context_for_user(
        &self,
        username: &str,
        subject_type: crate::types::UsersGetContextUserSubjectType,
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
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * FROM: <https://docs.github.com/rest/users/keys#list-public-keys-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * FROM: <https://docs.github.com/rest/users/keys#list-public-keys-for-a-user>
     */
    pub async fn list_all_public_keys_for_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::KeySimple>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/keys",
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * List social accounts for a user.
     *
     * This function performs a `GET` to the `/users/{username}/social_accounts` endpoint.
     *
     * Lists social media accounts for a user. This endpoint is accessible by anyone.
     *
     * FROM: <https://docs.github.com/rest/users/social-accounts#list-social-accounts-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_social_accounts_for_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SocialAccount>>> {
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
                "/users/{}/social_accounts?{}",
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * List social accounts for a user.
     *
     * This function performs a `GET` to the `/users/{username}/social_accounts` endpoint.
     *
     * As opposed to `list_social_accounts_for_user`, this function returns all the pages of the request at once.
     *
     * Lists social media accounts for a user. This endpoint is accessible by anyone.
     *
     * FROM: <https://docs.github.com/rest/users/social-accounts#list-social-accounts-for-a-user>
     */
    pub async fn list_all_social_accounts_for_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::SocialAccount>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/social_accounts",
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * List SSH signing keys for a user.
     *
     * This function performs a `GET` to the `/users/{username}/ssh_signing_keys` endpoint.
     *
     * Lists the SSH signing keys for a user. This operation is accessible by anyone.
     *
     * FROM: <https://docs.github.com/rest/users/ssh-signing-keys#list-ssh-signing-keys-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_ssh_signing_keys_for_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::SshSigningKey>>> {
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
                "/users/{}/ssh_signing_keys?{}",
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * List SSH signing keys for a user.
     *
     * This function performs a `GET` to the `/users/{username}/ssh_signing_keys` endpoint.
     *
     * As opposed to `list_ssh_signing_keys_for_user`, this function returns all the pages of the request at once.
     *
     * Lists the SSH signing keys for a user. This operation is accessible by anyone.
     *
     * FROM: <https://docs.github.com/rest/users/ssh-signing-keys#list-ssh-signing-keys-for-a-user>
     */
    pub async fn list_all_ssh_signing_keys_for_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::SshSigningKey>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/ssh_signing_keys",
                crate::progenitor_support::encode_path(&username.to_string()),
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
