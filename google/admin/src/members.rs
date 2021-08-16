use anyhow::Result;

use crate::Client;

pub struct Members {
    client: Client,
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
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        group_key: &str,
        member_key: &str,
    ) -> Result<crate::types::MembersHasMember> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/groups/{}/hasMember/{}?{}",
            crate::progenitor_support::encode_path(&group_key.to_string()),
            crate::progenitor_support::encode_path(&member_key.to_string()),
            query_
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
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        group_key: &str,
        include_derived_membership: bool,
        max_results: i64,
        page_token: &str,
        roles: &str,
    ) -> Result<Vec<crate::types::Member>> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if include_derived_membership {
            query_args.push(format!(
                "include_derived_membership={}",
                include_derived_membership
            ));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if max_results > 0 {
            query_args.push(format!("max_results={}", max_results));
        }
        if !page_token.is_empty() {
            query_args.push(format!("page_token={}", page_token));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !roles.is_empty() {
            query_args.push(format!("roles={}", roles));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
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
    pub async fn directory_list_members(
        &self,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        group_key: &str,
        include_derived_membership: bool,
        roles: &str,
    ) -> Result<Vec<crate::types::Member>> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if include_derived_membership {
            query_args.push(format!(
                "include_derived_membership={}",
                include_derived_membership
            ));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !roles.is_empty() {
            query_args.push(format!("roles={}", roles));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
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
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        group_key: &str,
        body: &crate::types::Member,
    ) -> Result<crate::types::Member> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/groups/{}/members?{}",
            crate::progenitor_support::encode_path(&group_key.to_string()),
            query_
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
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        group_key: &str,
        member_key: &str,
    ) -> Result<crate::types::Member> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/groups/{}/members/{}?{}",
            crate::progenitor_support::encode_path(&group_key.to_string()),
            crate::progenitor_support::encode_path(&member_key.to_string()),
            query_
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
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        group_key: &str,
        member_key: &str,
        body: &crate::types::Member,
    ) -> Result<crate::types::Member> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/groups/{}/members/{}?{}",
            crate::progenitor_support::encode_path(&group_key.to_string()),
            crate::progenitor_support::encode_path(&member_key.to_string()),
            query_
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
    pub async fn directory_delete(
        &self,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        group_key: &str,
        member_key: &str,
    ) -> Result<()> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/groups/{}/members/{}?{}",
            crate::progenitor_support::encode_path(&group_key.to_string()),
            crate::progenitor_support::encode_path(&member_key.to_string()),
            query_
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
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        group_key: &str,
        member_key: &str,
        body: &crate::types::Member,
    ) -> Result<crate::types::Member> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/groups/{}/members/{}?{}",
            crate::progenitor_support::encode_path(&group_key.to_string()),
            crate::progenitor_support::encode_path(&member_key.to_string()),
            query_
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
