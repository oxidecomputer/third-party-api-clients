use crate::Client;
use crate::ClientResult;

pub struct Groups {
    pub client: Client,
}

impl Groups {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Groups { client }
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/groups` endpoint.
     *
     * Retrieves all groups of a domain or of a user given a userKey (paginated).
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The unique ID for the customer's Google Workspace account. In case of a multi-domain account, to fetch all groups for a customer, fill this field instead of domain. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users](/admin-sdk/directory/v1/reference/users).
     * * `domain: &str` -- The domain name. Use this field to get fields from only one domain. To return all domains for a customer account, use the `customer` query parameter instead.
     * * `max_results: i64` -- Maximum number of results to return. Max allowed value is 200.
     * * `order_by: crate::types::DirectoryGroupsListOrderBy` -- Column to use for sorting results.
     * * `page_token: &str` -- Token to specify next page in the list.
     * * `query: &str` -- Query string search. Should be of the form "". Complete documentation is at https: //developers.google.com/admin-sdk/directory/v1/guides/search-groups.
     * * `sort_order: crate::types::SortOrder` -- Whether to return results in ascending or descending order. Must be used with the `orderBy` parameter.
     * * `user_key: &str` -- Email or immutable ID of the user if only those groups are to be listed, the given user is a member of. If it's an ID, it should match with the ID of the user object.
     */
    pub async fn list(
        &self,
        customer: &str,
        domain: &str,
        max_results: i64,
        order_by: crate::types::DirectoryGroupsListOrderBy,
        page_token: &str,
        query: &str,
        sort_order: crate::types::SortOrder,
        user_key: &str,
    ) -> ClientResult<Vec<crate::types::Group>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !domain.is_empty() {
            query_args.push(("domain".to_string(), domain.to_string()));
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
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        if !sort_order.to_string().is_empty() {
            query_args.push(("sortOrder".to_string(), sort_order.to_string()));
        }
        if !user_key.is_empty() {
            query_args.push(("userKey".to_string(), user_key.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/directory/v1/groups?{}", query_), None);
        let resp: crate::types::Groups = self
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
        Ok(resp.groups.to_vec())
    }
    /**
     * This function performs a `GET` to the `/admin/directory/v1/groups` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Retrieves all groups of a domain or of a user given a userKey (paginated).
     */
    pub async fn list_all(
        &self,
        customer: &str,
        domain: &str,
        order_by: crate::types::DirectoryGroupsListOrderBy,
        query: &str,
        sort_order: crate::types::SortOrder,
        user_key: &str,
    ) -> ClientResult<Vec<crate::types::Group>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !domain.is_empty() {
            query_args.push(("domain".to_string(), domain.to_string()));
        }
        if !order_by.to_string().is_empty() {
            query_args.push(("orderBy".to_string(), order_by.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        if !sort_order.to_string().is_empty() {
            query_args.push(("sortOrder".to_string(), sort_order.to_string()));
        }
        if !user_key.is_empty() {
            query_args.push(("userKey".to_string(), user_key.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/directory/v1/groups?{}", query_), None);
        let mut resp: crate::types::Groups = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut groups = resp.groups;
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

            groups.append(&mut resp.groups);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(groups)
    }
    /**
     * This function performs a `POST` to the `/admin/directory/v1/groups` endpoint.
     *
     * Creates a group.
     */
    pub async fn insert(&self, body: &crate::types::Group) -> ClientResult<crate::types::Group> {
        let url = self.client.url("/admin/directory/v1/groups", None);
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
     * This function performs a `GET` to the `/admin/directory/v1/groups/{groupKey}` endpoint.
     *
     * Retrieves a group's properties.
     *
     * **Parameters:**
     *
     * * `group_key: &str` -- Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
     */
    pub async fn get(&self, group_key: &str) -> ClientResult<crate::types::Group> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/groups/{}",
                crate::progenitor_support::encode_path(group_key),
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
     * This function performs a `PUT` to the `/admin/directory/v1/groups/{groupKey}` endpoint.
     *
     * Updates a group's properties.
     *
     * **Parameters:**
     *
     * * `group_key: &str` -- Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
     */
    pub async fn update(
        &self,
        group_key: &str,
        body: &crate::types::Group,
    ) -> ClientResult<crate::types::Group> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/groups/{}",
                crate::progenitor_support::encode_path(group_key),
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
     * This function performs a `DELETE` to the `/admin/directory/v1/groups/{groupKey}` endpoint.
     *
     * Deletes a group.
     *
     * **Parameters:**
     *
     * * `group_key: &str` -- Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
     */
    pub async fn delete(&self, group_key: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/groups/{}",
                crate::progenitor_support::encode_path(group_key),
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
     * This function performs a `PATCH` to the `/admin/directory/v1/groups/{groupKey}` endpoint.
     *
     * Updates a group's properties. This method supports [patch semantics](/admin-sdk/directory/v1/guides/performance#patch).
     *
     * **Parameters:**
     *
     * * `group_key: &str` -- Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
     */
    pub async fn patch(
        &self,
        group_key: &str,
        body: &crate::types::Group,
    ) -> ClientResult<crate::types::Group> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/groups/{}",
                crate::progenitor_support::encode_path(group_key),
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
     * This function performs a `GET` to the `/admin/directory/v1/groups/{groupKey}/aliases` endpoint.
     *
     * Lists all aliases for a group.
     *
     * **Parameters:**
     *
     * * `group_key: &str` -- Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
     */
    pub async fn aliases_list(&self, group_key: &str) -> ClientResult<crate::types::Aliases> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/groups/{}/aliases",
                crate::progenitor_support::encode_path(group_key),
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
     * This function performs a `POST` to the `/admin/directory/v1/groups/{groupKey}/aliases` endpoint.
     *
     * Adds an alias for the group.
     *
     * **Parameters:**
     *
     * * `group_key: &str` -- Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
     */
    pub async fn aliases_insert(
        &self,
        group_key: &str,
        body: &crate::types::Alias,
    ) -> ClientResult<crate::types::Alias> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/groups/{}/aliases",
                crate::progenitor_support::encode_path(group_key),
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
     * This function performs a `DELETE` to the `/admin/directory/v1/groups/{groupKey}/aliases/{alias}` endpoint.
     *
     * Removes an alias.
     *
     * **Parameters:**
     *
     * * `group_key: &str` -- Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
     * * `alias: &str` -- The alias to be removed.
     */
    pub async fn aliases_delete(&self, group_key: &str, alias: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/groups/{}/aliases/{}",
                crate::progenitor_support::encode_path(group_key),
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
}
