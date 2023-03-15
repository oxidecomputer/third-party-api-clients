use crate::Client;
use crate::ClientResult;

pub struct Members {
    pub client: Client,
}

impl Members {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Members { client }
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/groups/{groupKey}/hasMember/{memberKey}` endpoint.
     *
     * Checks whether the given user is a member of the group. Membership can be direct or nested.
     *
     * **Parameters:**
     *
     * * `group_key: &str` -- Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
     * * `member_key: &str` -- Identifies the user member in the API request. The value can be the user's primary email address, alias, or unique ID.
     */
    pub async fn has(
        &self,
        group_key: &str,
        member_key: &str,
    ) -> ClientResult<crate::types::MembersHasMember> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/groups/{}/hasMember/{}",
                crate::progenitor_support::encode_path(group_key),
                crate::progenitor_support::encode_path(member_key),
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
     * This function performs a `GET` to the `/admin/directory/v1/groups/{groupKey}/members` endpoint.
     *
     * Retrieves a paginated list of all members in a group.
     *
     * **Parameters:**
     *
     * * `group_key: &str` -- Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
     * * `include_derived_membership: bool` -- A Boolean value to indicate whether payload is wanted. Optional.
     * * `max_results: i64` -- Maximum number of results to return. Max allowed value is 200.
     * * `page_token: &str` -- Token to specify next page in the list.
     * * `roles: &str` -- The `roles` query parameter allows you to retrieve group members by role. Allowed values are `OWNER`, `MANAGER`, and `MEMBER`.
     */
    pub async fn list(
        &self,
        group_key: &str,
        include_derived_membership: bool,
        max_results: i64,
        page_token: &str,
        roles: &str,
    ) -> ClientResult<Vec<crate::types::Member>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if include_derived_membership {
            query_args.push((
                "includeDerivedMembership".to_string(),
                include_derived_membership.to_string(),
            ));
        }
        if max_results > 0 {
            query_args.push(("maxResults".to_string(), max_results.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        if !roles.is_empty() {
            query_args.push(("roles".to_string(), roles.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/groups/{}/members?{}",
                crate::progenitor_support::encode_path(group_key),
                query_
            ),
            None,
        );
        let resp: crate::types::Members = self
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
        Ok(resp.members.to_vec())
    }
    /**
     * This function performs a `GET` to the `/admin/directory/v1/groups/{groupKey}/members` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Retrieves a paginated list of all members in a group.
     */
    pub async fn list_all(
        &self,
        group_key: &str,
        include_derived_membership: bool,
        roles: &str,
    ) -> ClientResult<Vec<crate::types::Member>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if include_derived_membership {
            query_args.push((
                "includeDerivedMembership".to_string(),
                include_derived_membership.to_string(),
            ));
        }
        if !roles.is_empty() {
            query_args.push(("roles".to_string(), roles.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/groups/{}/members?{}",
                crate::progenitor_support::encode_path(group_key),
                query_
            ),
            None,
        );
        let mut resp: crate::types::Members = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut members = resp.members;
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

            members.append(&mut resp.members);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(members)
    }
    /**
     * This function performs a `POST` to the `/admin/directory/v1/groups/{groupKey}/members` endpoint.
     *
     * Adds a user to the specified group.
     *
     * **Parameters:**
     *
     * * `group_key: &str` -- Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
     */
    pub async fn insert(
        &self,
        group_key: &str,
        body: &crate::types::Member,
    ) -> ClientResult<crate::types::Member> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/groups/{}/members",
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
     * This function performs a `GET` to the `/admin/directory/v1/groups/{groupKey}/members/{memberKey}` endpoint.
     *
     * Retrieves a group member's properties.
     *
     * **Parameters:**
     *
     * * `group_key: &str` -- Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
     * * `member_key: &str` -- Identifies the group member in the API request. A group member can be a user or another group. The value can be the member's (group or user) primary email address, alias, or unique ID.
     */
    pub async fn get(
        &self,
        group_key: &str,
        member_key: &str,
    ) -> ClientResult<crate::types::Member> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/groups/{}/members/{}",
                crate::progenitor_support::encode_path(group_key),
                crate::progenitor_support::encode_path(member_key),
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
     * This function performs a `PUT` to the `/admin/directory/v1/groups/{groupKey}/members/{memberKey}` endpoint.
     *
     * Updates the membership of a user in the specified group.
     *
     * **Parameters:**
     *
     * * `group_key: &str` -- Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
     * * `member_key: &str` -- Identifies the group member in the API request. A group member can be a user or another group. The value can be the member's (group or user) primary email address, alias, or unique ID.
     */
    pub async fn update(
        &self,
        group_key: &str,
        member_key: &str,
        body: &crate::types::Member,
    ) -> ClientResult<crate::types::Member> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/groups/{}/members/{}",
                crate::progenitor_support::encode_path(group_key),
                crate::progenitor_support::encode_path(member_key),
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
     * This function performs a `DELETE` to the `/admin/directory/v1/groups/{groupKey}/members/{memberKey}` endpoint.
     *
     * Removes a member from a group.
     *
     * **Parameters:**
     *
     * * `group_key: &str` -- Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
     * * `member_key: &str` -- Identifies the group member in the API request. A group member can be a user or another group. The value can be the member's (group or user) primary email address, alias, or unique ID.
     */
    pub async fn delete(&self, group_key: &str, member_key: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/groups/{}/members/{}",
                crate::progenitor_support::encode_path(group_key),
                crate::progenitor_support::encode_path(member_key),
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
     * This function performs a `PATCH` to the `/admin/directory/v1/groups/{groupKey}/members/{memberKey}` endpoint.
     *
     * Updates the membership properties of a user in the specified group. This method supports [patch semantics](/admin-sdk/directory/v1/guides/performance#patch).
     *
     * **Parameters:**
     *
     * * `group_key: &str` -- Identifies the group in the API request. The value can be the group's email address, group alias, or the unique group ID.
     * * `member_key: &str` -- Identifies the group member in the API request. A group member can be a user or another group. The value can be the member's (group or user) primary email address, alias, or unique ID.
     */
    pub async fn patch(
        &self,
        group_key: &str,
        member_key: &str,
        body: &crate::types::Member,
    ) -> ClientResult<crate::types::Member> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/groups/{}/members/{}",
                crate::progenitor_support::encode_path(group_key),
                crate::progenitor_support::encode_path(member_key),
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
}
