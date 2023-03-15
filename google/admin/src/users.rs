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
     * This function performs a `GET` to the `/admin/directory/v1/users` endpoint.
     *
     * Retrieves a paginated list of either deleted users or all users in a domain.
     *
     * **Parameters:**
     *
     * * `custom_field_mask: &str` -- A comma-separated list of schema names. All fields from these schemas are fetched. This should only be set when `projection=custom`.
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. In case of a multi-domain account, to fetch all groups for a customer, fill this field instead of domain. You can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users). Either the `customer` or the `domain` parameter must be provided.
     * * `domain: &str` -- The domain name. Use this field to get fields from only one domain. To return all domains for a customer account, use the `customer` query parameter instead. Either the `customer` or the `domain` parameter must be provided.
     * * `event: crate::types::Event` -- Event on which subscription is intended (if subscribing).
     * * `max_results: i64` -- Maximum number of results to return.
     * * `order_by: crate::types::DirectoryUsersListOrderBy` -- Property to use for sorting results.
     * * `page_token: &str` -- Token to specify next page in the list.
     * * `projection: crate::types::DirectoryUsersListProjection` -- What subset of fields to fetch for this user.
     * * `query: &str` -- Query string for searching user fields. For more information on constructing user queries, see [Search for Users](/admin-sdk/directory/v1/guides/search-users).
     * * `show_deleted: &str` -- If set to `true`, retrieves the list of deleted users. (Default: `false`).
     * * `sort_order: crate::types::SortOrder` -- Whether to return results in ascending or descending order. Must be used with the `orderBy` parameter.
     * * `view_type: crate::types::ViewType` -- Whether to fetch the administrator-only or domain-wide public view of the user. For more information, see [Retrieve a user as a non-administrator](/admin-sdk/directory/v1/guides/manage-users#retrieve_users_non_admin).
     */
    pub async fn list(
        &self,
        customer: &str,
        domain: &str,
        event: crate::types::Event,
        max_results: i64,
        order_by: crate::types::DirectoryUsersListOrderBy,
        page_token: &str,
        projection: crate::types::DirectoryUsersListProjection,
        query: &str,
        show_deleted: &str,
        sort_order: crate::types::SortOrder,
        view_type: crate::types::ViewType,
    ) -> ClientResult<Vec<crate::types::User>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !domain.is_empty() {
            query_args.push(("domain".to_string(), domain.to_string()));
        }
        if !event.to_string().is_empty() {
            query_args.push(("event".to_string(), event.to_string()));
        }
        if max_results > 0 {
            query_args.push(("maxResults".to_string(), max_results.to_string()));
        }
        if !order_by.to_string().is_empty() {
            query_args.push(("orderBy".to_string(), order_by.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        if !projection.to_string().is_empty() {
            query_args.push(("projection".to_string(), projection.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        if !show_deleted.is_empty() {
            query_args.push(("showDeleted".to_string(), show_deleted.to_string()));
        }
        if !sort_order.to_string().is_empty() {
            query_args.push(("sortOrder".to_string(), sort_order.to_string()));
        }
        if !view_type.to_string().is_empty() {
            query_args.push(("viewType".to_string(), view_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/directory/v1/users?{}", query_), None);
        let resp: crate::types::Users = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(resp.users.to_vec())
    }
    /**
     * This function performs a `GET` to the `/admin/directory/v1/users` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Retrieves a paginated list of either deleted users or all users in a domain.
     */
    pub async fn list_all(
        &self,
        customer: &str,
        domain: &str,
        event: crate::types::Event,
        order_by: crate::types::DirectoryUsersListOrderBy,
        projection: crate::types::DirectoryUsersListProjection,
        query: &str,
        show_deleted: &str,
        sort_order: crate::types::SortOrder,
        view_type: crate::types::ViewType,
    ) -> ClientResult<Vec<crate::types::User>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !domain.is_empty() {
            query_args.push(("domain".to_string(), domain.to_string()));
        }
        if !event.to_string().is_empty() {
            query_args.push(("event".to_string(), event.to_string()));
        }
        if !order_by.to_string().is_empty() {
            query_args.push(("orderBy".to_string(), order_by.to_string()));
        }
        if !projection.to_string().is_empty() {
            query_args.push(("projection".to_string(), projection.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        if !show_deleted.is_empty() {
            query_args.push(("showDeleted".to_string(), show_deleted.to_string()));
        }
        if !sort_order.to_string().is_empty() {
            query_args.push(("sortOrder".to_string(), sort_order.to_string()));
        }
        if !view_type.to_string().is_empty() {
            query_args.push(("viewType".to_string(), view_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/directory/v1/users?{}", query_), None);
        let mut resp: crate::types::Users = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut users = resp.users;
        let mut page = resp.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                resp = self
                    .client
                    .get(
                        &format!("{}?pageToken={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                resp = self
                    .client
                    .get(
                        &format!("{}&pageToken={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            users.append(&mut resp.users);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(users)
    }
    /**
     * This function performs a `POST` to the `/admin/directory/v1/users` endpoint.
     *
     * Creates a user.
     */
    pub async fn insert(&self, body: &crate::types::User) -> ClientResult<crate::types::User> {
        let url = self.client.url("/admin/directory/v1/users", None);
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
     * This function performs a `POST` to the `/admin/directory/v1/users/watch` endpoint.
     *
     * Watches for changes in users list.
     *
     * **Parameters:**
     *
     * * `custom_field_mask: &str` -- Comma-separated list of schema names. All fields from these schemas are fetched. This should only be set when projection=custom.
     * * `customer: &str` -- Immutable ID of the Google Workspace account. In case of multi-domain, to fetch all users for a customer, fill this field instead of domain.
     * * `domain: &str` -- Name of the domain. Fill this field to get users from only this domain. To return all users in a multi-domain fill customer field instead.".
     * * `event: crate::types::Event` -- Event on which subscription is intended (if subscribing).
     * * `max_results: i64` -- Maximum number of results to return.
     * * `order_by: crate::types::DirectoryUsersListOrderBy` -- Property to use for sorting results.
     * * `page_token: &str` -- Token to specify next page in the list.
     * * `projection: crate::types::DirectoryUsersListProjection` -- What subset of fields to fetch for this user.
     * * `query: &str` -- Query string search. Should be of the form "". Complete documentation is at https: //developers.google.com/admin-sdk/directory/v1/guides/search-users.
     * * `show_deleted: &str` -- If set to true, retrieves the list of deleted users. (Default: false).
     * * `sort_order: crate::types::SortOrder` -- Whether to return results in ascending or descending order. Must be used with the `orderBy` parameter.
     * * `view_type: crate::types::ViewType` -- Whether to fetch the administrator-only or domain-wide public view of the user. For more information, see [Retrieve a user as a non-administrator](/admin-sdk/directory/v1/guides/manage-users#retrieve_users_non_admin).
     */
    pub async fn watch(
        &self,
        customer: &str,
        domain: &str,
        event: crate::types::Event,
        max_results: i64,
        order_by: crate::types::DirectoryUsersListOrderBy,
        page_token: &str,
        projection: crate::types::DirectoryUsersListProjection,
        query: &str,
        show_deleted: &str,
        sort_order: crate::types::SortOrder,
        view_type: crate::types::ViewType,
        body: &crate::types::Channel,
    ) -> ClientResult<crate::types::Channel> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !domain.is_empty() {
            query_args.push(("domain".to_string(), domain.to_string()));
        }
        if !event.to_string().is_empty() {
            query_args.push(("event".to_string(), event.to_string()));
        }
        if max_results > 0 {
            query_args.push(("maxResults".to_string(), max_results.to_string()));
        }
        if !order_by.to_string().is_empty() {
            query_args.push(("orderBy".to_string(), order_by.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        if !projection.to_string().is_empty() {
            query_args.push(("projection".to_string(), projection.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        if !show_deleted.is_empty() {
            query_args.push(("showDeleted".to_string(), show_deleted.to_string()));
        }
        if !sort_order.to_string().is_empty() {
            query_args.push(("sortOrder".to_string(), sort_order.to_string()));
        }
        if !view_type.to_string().is_empty() {
            query_args.push(("viewType".to_string(), view_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/directory/v1/users/watch?{}", query_), None);
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
     * This function performs a `GET` to the `/admin/directory/v1/users/{userKey}` endpoint.
     *
     * Retrieves a user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     * * `custom_field_mask: &str` -- A comma-separated list of schema names. All fields from these schemas are fetched. This should only be set when `projection=custom`.
     * * `projection: crate::types::DirectoryUsersListProjection` -- What subset of fields to fetch for this user.
     * * `view_type: crate::types::ViewType` -- Whether to fetch the administrator-only or domain-wide public view of the user. For more information, see [Retrieve a user as a non-administrator](/admin-sdk/directory/v1/guides/manage-users#retrieve_users_non_admin).
     */
    pub async fn get(
        &self,
        user_key: &str,
        projection: crate::types::DirectoryUsersListProjection,
        view_type: crate::types::ViewType,
    ) -> ClientResult<crate::types::User> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !projection.to_string().is_empty() {
            query_args.push(("projection".to_string(), projection.to_string()));
        }
        if !view_type.to_string().is_empty() {
            query_args.push(("viewType".to_string(), view_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/users/{}?{}",
                crate::progenitor_support::encode_path(user_key),
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
     * This function performs a `PUT` to the `/admin/directory/v1/users/{userKey}` endpoint.
     *
     * Updates a user. This method supports patch semantics, meaning you only need to include the fields you wish to update. Fields that are not present in the request will be preserved, and fields set to `null` will be cleared.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn update(
        &self,
        user_key: &str,
        body: &crate::types::User,
    ) -> ClientResult<crate::types::User> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/users/{}",
                crate::progenitor_support::encode_path(user_key),
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
     * This function performs a `DELETE` to the `/admin/directory/v1/users/{userKey}` endpoint.
     *
     * Deletes a user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn delete(&self, user_key: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/users/{}",
                crate::progenitor_support::encode_path(user_key),
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
     * This function performs a `PATCH` to the `/admin/directory/v1/users/{userKey}` endpoint.
     *
     * Updates a user using patch semantics. The update method should be used instead, since it also supports patch semantics and has better performance. This method is unable to clear fields that contain repeated objects (`addresses`, `phones`, etc). Use the update method instead.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn patch(
        &self,
        user_key: &str,
        body: &crate::types::User,
    ) -> ClientResult<crate::types::User> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/users/{}",
                crate::progenitor_support::encode_path(user_key),
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
     * This function performs a `GET` to the `/admin/directory/v1/users/{userKey}/aliases` endpoint.
     *
     * Lists all aliases for a user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     * * `event: crate::types::DirectoryUsersAliasesListEvent` -- Events to watch for.
     */
    pub async fn aliases_list(
        &self,
        user_key: &str,
        event: crate::types::DirectoryUsersAliasesListEvent,
    ) -> ClientResult<crate::types::Aliases> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !event.to_string().is_empty() {
            query_args.push(("event".to_string(), event.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/users/{}/aliases?{}",
                crate::progenitor_support::encode_path(user_key),
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
     * This function performs a `POST` to the `/admin/directory/v1/users/{userKey}/aliases` endpoint.
     *
     * Adds an alias.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn aliases_insert(
        &self,
        user_key: &str,
        body: &crate::types::Alias,
    ) -> ClientResult<crate::types::Alias> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/users/{}/aliases",
                crate::progenitor_support::encode_path(user_key),
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
     * This function performs a `POST` to the `/admin/directory/v1/users/{userKey}/aliases/watch` endpoint.
     *
     * Watches for changes in users list.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Email or immutable ID of the user.
     * * `event: crate::types::DirectoryUsersAliasesListEvent` -- Events to watch for.
     */
    pub async fn aliases_watch(
        &self,
        user_key: &str,
        event: crate::types::DirectoryUsersAliasesListEvent,
        body: &crate::types::Channel,
    ) -> ClientResult<crate::types::Channel> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !event.to_string().is_empty() {
            query_args.push(("event".to_string(), event.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/users/{}/aliases/watch?{}",
                crate::progenitor_support::encode_path(user_key),
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
     * This function performs a `DELETE` to the `/admin/directory/v1/users/{userKey}/aliases/{alias}` endpoint.
     *
     * Removes an alias.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     * * `alias: &str` -- The alias to be removed.
     */
    pub async fn aliases_delete(&self, user_key: &str, alias: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/users/{}/aliases/{}",
                crate::progenitor_support::encode_path(user_key),
                crate::progenitor_support::encode_path(alias),
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
     * This function performs a `POST` to the `/admin/directory/v1/users/{userKey}/makeAdmin` endpoint.
     *
     * Makes a user a super administrator.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn make_admin(
        &self,
        user_key: &str,
        body: &crate::types::UserMakeAdmin,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/users/{}/makeAdmin",
                crate::progenitor_support::encode_path(user_key),
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
     * This function performs a `GET` to the `/admin/directory/v1/users/{userKey}/photos/thumbnail` endpoint.
     *
     * Retrieves the user's photo.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn photos_get(&self, user_key: &str) -> ClientResult<crate::types::UserPhoto> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/users/{}/photos/thumbnail",
                crate::progenitor_support::encode_path(user_key),
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
     * This function performs a `PUT` to the `/admin/directory/v1/users/{userKey}/photos/thumbnail` endpoint.
     *
     * Adds a photo for the user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn photos_update(
        &self,
        user_key: &str,
        body: &crate::types::UserPhoto,
    ) -> ClientResult<crate::types::UserPhoto> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/users/{}/photos/thumbnail",
                crate::progenitor_support::encode_path(user_key),
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
     * This function performs a `DELETE` to the `/admin/directory/v1/users/{userKey}/photos/thumbnail` endpoint.
     *
     * Removes the user's photo.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn photos_delete(&self, user_key: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/users/{}/photos/thumbnail",
                crate::progenitor_support::encode_path(user_key),
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
     * This function performs a `PATCH` to the `/admin/directory/v1/users/{userKey}/photos/thumbnail` endpoint.
     *
     * Adds a photo for the user. This method supports [patch semantics](/admin-sdk/directory/v1/guides/performance#patch).
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn photos_patch(
        &self,
        user_key: &str,
        body: &crate::types::UserPhoto,
    ) -> ClientResult<crate::types::UserPhoto> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/users/{}/photos/thumbnail",
                crate::progenitor_support::encode_path(user_key),
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
     * This function performs a `POST` to the `/admin/directory/v1/users/{userKey}/signOut` endpoint.
     *
     * Signs a user out of all web and device sessions and reset their sign-in cookies. User will have to sign in by authenticating again.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the target user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn sign_out(&self, user_key: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/users/{}/signOut",
                crate::progenitor_support::encode_path(user_key),
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
     * This function performs a `POST` to the `/admin/directory/v1/users/{userKey}/undelete` endpoint.
     *
     * Undeletes a deleted user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- The immutable id of the user.
     */
    pub async fn undelete(
        &self,
        user_key: &str,
        body: &crate::types::UserUndelete,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/users/{}/undelete",
                crate::progenitor_support::encode_path(user_key),
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
}
