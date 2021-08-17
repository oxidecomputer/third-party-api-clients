use anyhow::Result;

use crate::Client;

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
    pub async fn directory_has_member(
        &self,
        group_key: &str,
        member_key: &str,
    ) -> Result<crate::types::MembersHasMember> {
        let url = format!(
            "/admin/directory/v1/groups/{}/hasMember/{}",
            crate::progenitor_support::encode_path(&group_key.to_string()),
            crate::progenitor_support::encode_path(&member_key.to_string()),
        );

        self.client.get(&url, None).await
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
    pub async fn directory_list(
        &self,
        group_key: &str,
        include_derived_membership: bool,
        max_results: i64,
        page_token: &str,
        roles: &str,
    ) -> Result<Vec<crate::types::Member>> {
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
        let url = format!(
            "/admin/directory/v1/groups/{}/members?{}",
            crate::progenitor_support::encode_path(&group_key.to_string()),
            query_
        );

        let resp: crate::types::Members = self.client.get(&url, None).await.unwrap();

        // Return our response data.
        Ok(resp.members)
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/groups/{groupKey}/members` endpoint.
     *
     * As opposed to `directory_list`, this function returns all the pages of the request at once.
     *
     * Retrieves a paginated list of all members in a group.
     */
    pub async fn directory_list_all(
        &self,
        group_key: &str,
        include_derived_membership: bool,
        roles: &str,
    ) -> Result<Vec<crate::types::Member>> {
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
        let url = format!(
            "/admin/directory/v1/groups/{}/members?{}",
            crate::progenitor_support::encode_path(&group_key.to_string()),
            query_
        );

        let mut resp: crate::types::Members = self.client.get(&url, None).await.unwrap();

        let mut members = resp.members;
        let mut page = resp.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                resp = self
                    .client
                    .get(&format!("{}?pageToken={}", url, page), None)
                    .await
                    .unwrap();
            } else {
                resp = self
                    .client
                    .get(&format!("{}&pageToken={}", url, page), None)
                    .await
                    .unwrap();
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
    pub async fn directory_insert(
        &self,
        group_key: &str,
        body: &crate::types::Member,
    ) -> Result<crate::types::Member> {
        let url = format!(
            "/admin/directory/v1/groups/{}/members",
            crate::progenitor_support::encode_path(&group_key.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
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
    pub async fn directory_get(
        &self,
        group_key: &str,
        member_key: &str,
    ) -> Result<crate::types::Member> {
        let url = format!(
            "/admin/directory/v1/groups/{}/members/{}",
            crate::progenitor_support::encode_path(&group_key.to_string()),
            crate::progenitor_support::encode_path(&member_key.to_string()),
        );

        self.client.get(&url, None).await
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
    pub async fn directory_update(
        &self,
        group_key: &str,
        member_key: &str,
        body: &crate::types::Member,
    ) -> Result<crate::types::Member> {
        let url = format!(
            "/admin/directory/v1/groups/{}/members/{}",
            crate::progenitor_support::encode_path(&group_key.to_string()),
            crate::progenitor_support::encode_path(&member_key.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
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
    pub async fn directory_delete(&self, group_key: &str, member_key: &str) -> Result<()> {
        let url = format!(
            "/admin/directory/v1/groups/{}/members/{}",
            crate::progenitor_support::encode_path(&group_key.to_string()),
            crate::progenitor_support::encode_path(&member_key.to_string()),
        );

        self.client.delete(&url, None).await
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
    pub async fn directory_patch(
        &self,
        group_key: &str,
        member_key: &str,
        body: &crate::types::Member,
    ) -> Result<crate::types::Member> {
        let url = format!(
            "/admin/directory/v1/groups/{}/members/{}",
            crate::progenitor_support::encode_path(&group_key.to_string()),
            crate::progenitor_support::encode_path(&member_key.to_string()),
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
