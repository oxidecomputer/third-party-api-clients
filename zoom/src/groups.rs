use anyhow::Result;

use crate::Client;

pub struct Groups {
    pub client: Client,
}

impl Groups {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Groups { client }
    }

    /**
     * List groups.
     *
     * This function performs a `GET` to the `/groups` endpoint.
     *
     * List [groups](https://support.zoom.us/hc/en-us/articles/204519819-Group-Management-) under an account.
     *
     * **Prerequisite**: Pro or higher account.<br>
     * **Scopes**: `group:read:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     */
    pub async fn get(&self) -> Result<crate::types::GroupList> {
        let url = "/groups".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Create a group.
     *
     * This function performs a `POST` to the `/groups` endpoint.
     *
     * Create a [group](https://support.zoom.us/hc/en-us/articles/204519819-Group-Management-) under an account.
     *
     * You can add a maximum of 100 groups in one account per day. If you go over, you will get an error. You can add a maximum of 5000 groups in one account.
     *
     * **Prerequisite**: Pro or higher account.<br>
     * **Scopes**: `group:write:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     */
    pub async fn create(&self, body: &crate::types::GroupCreateRequest) -> Result<()> {
        let url = "/groups".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get a group.
     *
     * This function performs a `GET` to the `/groups/{groupId}` endpoint.
     *
     * Get a [group](https://support.zoom.us/hc/en-us/articles/204519819-Group-Management-) under an account.
     *
     * **Prerequisite**: Pro, Business, or Education account<br>
     * **Scopes**: `group:read:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `group_id: &str` -- The group ID.<br>
     *   Can be retrieved by calling [GET /groups](https://marketplace.zoom.us/docs/api-reference/zoom-api/groups/groups).
     */
    pub async fn group(&self, group_id: &str) -> Result<crate::types::GroupResponse> {
        let url = format!(
            "/groups/{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a group.
     *
     * This function performs a `DELETE` to the `/groups/{groupId}` endpoint.
     *
     * Delete a [group](https://support.zoom.us/hc/en-us/articles/204519819-Group-Management-).
     *
     * **Prerequisite**: Pro, Business, or Education account<br>
     * **Scopes**: `group:write:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `group_id: &str` -- The group ID.<br>
     *   Can be retrieved by calling [GET /groups](https://marketplace.zoom.us/docs/api-reference/zoom-api/groups/groups).
     */
    pub async fn delete(&self, group_id: &str) -> Result<()> {
        let url = format!(
            "/groups/{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update a group.
     *
     * This function performs a `PATCH` to the `/groups/{groupId}` endpoint.
     *
     * Update a [group](https://support.zoom.us/hc/en-us/articles/204519819-Group-Management-) under your account.
     *
     * **Prerequisite**: Pro, Business, or Education account<br>
     * **Scopes**: `group:write:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `group_id: &str` -- The group ID.<br>
     *   Can be retrieved by calling [GET /groups](https://marketplace.zoom.us/docs/api-reference/zoom-api/groups/groups).
     */
    pub async fn update(
        &self,
        group_id: &str,
        body: &crate::types::GroupCreateRequest,
    ) -> Result<()> {
        let url = format!(
            "/groups/{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * List group members .
     *
     * This function performs a `GET` to the `/groups/{groupId}/members` endpoint.
     *
     * List the members of a [group](https://support.zoom.us/hc/en-us/articles/204519819-Group-Management-) under your account.
     *
     * **Prerequisite**: Pro, Business, or Education account<br>
     * **Scopes**: `group:read:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `group_id: &str` -- The group ID.<br>
     *   Can be retrieved by calling [GET /groups](https://marketplace.zoom.us/docs/api-reference/zoom-api/groups/groups).
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `page_number: i64` --
     *   **Deprecated** - This field has been deprecated and we will stop supporting it completely in a future release. Please use "next_page_token" for pagination instead of this field.
     *   
     *   The page number of the current page in the returned records.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn members(
        &self,
        group_id: &str,
        page_size: i64,
        page_number: i64,
        next_page_token: &str,
    ) -> Result<Vec<crate::types::UserCreateResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_number > 0 {
            query_args.push(("page_number".to_string(), page_number.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/groups/{}/members?{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
            query_
        );

        let resp: crate::types::GroupMembersResponseData = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.members)
    }

    /**
     * List group members .
     *
     * This function performs a `GET` to the `/groups/{groupId}/members` endpoint.
     *
     * As opposed to `members`, this function returns all the pages of the request at once.
     *
     * List the members of a [group](https://support.zoom.us/hc/en-us/articles/204519819-Group-Management-) under your account.
     *
     * **Prerequisite**: Pro, Business, or Education account<br>
     * **Scopes**: `group:read:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     */
    pub async fn get_all_members(
        &self,
        group_id: &str,
    ) -> Result<Vec<crate::types::UserCreateResponse>> {
        let url = format!(
            "/groups/{}/members",
            crate::progenitor_support::encode_path(&group_id.to_string()),
        );

        let mut resp: crate::types::GroupMembersResponseData = self.client.get(&url, None).await?;

        let mut members = resp.members;
        let mut page = resp.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            // Check if we already have URL params and need to concat the token.
            if !url.contains('?') {
                resp = self
                    .client
                    .get(&format!("{}?next_page_token={}", url, page), None)
                    .await?;
            } else {
                resp = self
                    .client
                    .get(&format!("{}&next_page_token={}", url, page), None)
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
     * Add group members.
     *
     * This function performs a `POST` to the `/groups/{groupId}/members` endpoint.
     *
     * Add members to a [group](https://support.zoom.us/hc/en-us/articles/204519819-Group-Management-) under your account.
     *
     * **Prerequisite**: Pro, Business, or Education account<br>
     * **Scopes**: `group:write:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `group_id: &str` -- The group ID.<br>
     *   Can be retrieved by calling [GET /groups](https://marketplace.zoom.us/docs/api-reference/zoom-api/groups/groups).
     */
    pub async fn members_create(
        &self,
        group_id: &str,
        body: &crate::types::AddRoleMembersRequest,
    ) -> Result<()> {
        let url = format!(
            "/groups/{}/members",
            crate::progenitor_support::encode_path(&group_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete a group member.
     *
     * This function performs a `DELETE` to the `/groups/{groupId}/members/{memberId}` endpoint.
     *
     * Delete a member from a [group](https://support.zoom.us/hc/en-us/articles/204519819-Group-Management-) in a Zoom account.
     *
     * **Prerequisite**: Pro, Business, or Education account<br>
     * **Scopes**: `group:write:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `group_id: &str` -- The group ID.<br>
     *   Can be retrieved by calling [GET /groups](https://marketplace.zoom.us/docs/api-reference/zoom-api/groups/groups).
     * * `member_id: &str` -- User's first name.
     */
    pub async fn members_delete(&self, group_id: &str, member_id: &str) -> Result<()> {
        let url = format!(
            "/groups/{}/members/{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
            crate::progenitor_support::encode_path(&member_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update a group member.
     *
     * This function performs a `PATCH` to the `/groups/{groupId}/members/{memberId}` endpoint.
     *
     * Use this API to perform either of the following tasks:
     * * Remove a group member from one group and move them to a different group.
     * * Set a user's primary group. By default, the primary group is the first group that user is added to.
     *
     * If a user is a member of multiple groups, you can [assign the user a primary group](https://support.zoom.us/hc/en-us/articles/204519819-Group-Management-#h_d07c7dcd-4fd8-485a-b5fe-a322e8d21c09). The user will use the primary group’s settings by default. However, if the user is a member of a group with locked settings, those group settings will remain locked to the user.
     *
     * **Scopes:** `group:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites:**
     * * A Pro or higher account
     *
     * **Parameters:**
     *
     * * `group_id: &str` -- The group's unique ID. To get this value, use the [List Groups](https://marketplace.zoom.us/docs/api-reference/zoom-api/groups/groups) API.
     *   * To set a user's primary group, use the `target_group_id` value for this parameter's value.
     *   * To move a group member from one group to another, use the `groupId` of the designated group.
     * * `member_id: &str` -- The group member's unique ID. To get this value, use the [List Group Members](https://marketplace.zoom.us/docs/api-reference/zoom-api/groups/groupmembers) API.
     */
    pub async fn update_member(
        &self,
        group_id: &str,
        member_id: &str,
        body: &crate::types::UpdateGroupMemberRequest,
    ) -> Result<()> {
        let url = format!(
            "/groups/{}/members/{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
            crate::progenitor_support::encode_path(&member_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get a group's settings.
     *
     * This function performs a `GET` to the `/groups/{groupId}/settings` endpoint.
     *
     * Get settings for a [group](https://support.zoom.us/hc/en-us/articles/204519819-Group-Management-).
     * **Prerequisite**: Pro, Business, or Education account<br>
     * **Scopes**: `group:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `option: crate::types::OptionData` -- Use the following options to filter the results of the account's information:
     *  \* `meeting_authentication` — View the account's [meeting authentication settings](https://support.zoom.us/hc/en-us/articles/360037117472-Authentication-Profiles-for-Meetings-and-Webinars).
     *  \* `recording_authentication` — View the account's [recording authentication settings](https://support.zoom.us/hc/en-us/articles/360037756671-Authentication-Profiles-for-Cloud-Recordings).
     *  \* `security` — View the account's security settings. For example, password requirements for user login or two-factor authentication.<br>
     *  \* `meeting_security` — View the account's meeting security settings.
     */
    pub async fn get_settings_domains(
        &self,
        group_id: &str,
        custom_query_fields: &str,
        option: crate::types::OptionData,
    ) -> Result<crate::types::Domains> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !custom_query_fields.is_empty() {
            query_args.push((
                "custom_query_fields".to_string(),
                custom_query_fields.to_string(),
            ));
        }
        if !option.to_string().is_empty() {
            query_args.push(("option".to_string(), option.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/groups/{}/settings?{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get a group's settings.
     *
     * This function performs a `GET` to the `/groups/{groupId}/settings` endpoint.
     *
     * Get settings for a [group](https://support.zoom.us/hc/en-us/articles/204519819-Group-Management-).
     * **Prerequisite**: Pro, Business, or Education account<br>
     * **Scopes**: `group:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `option: crate::types::OptionData` -- Use the following options to filter the results of the account's information:
     *  \* `meeting_authentication` — View the account's [meeting authentication settings](https://support.zoom.us/hc/en-us/articles/360037117472-Authentication-Profiles-for-Meetings-and-Webinars).
     *  \* `recording_authentication` — View the account's [recording authentication settings](https://support.zoom.us/hc/en-us/articles/360037756671-Authentication-Profiles-for-Cloud-Recordings).
     *  \* `security` — View the account's security settings. For example, password requirements for user login or two-factor authentication.<br>
     *  \* `meeting_security` — View the account's meeting security settings.
     */
    pub async fn get_settings_meeting_security(
        &self,
        group_id: &str,
        custom_query_fields: &str,
        option: crate::types::OptionData,
    ) -> Result<crate::types::MeetingSecuritySettings> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !custom_query_fields.is_empty() {
            query_args.push((
                "custom_query_fields".to_string(),
                custom_query_fields.to_string(),
            ));
        }
        if !option.to_string().is_empty() {
            query_args.push(("option".to_string(), option.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/groups/{}/settings?{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get a group's settings.
     *
     * This function performs a `GET` to the `/groups/{groupId}/settings` endpoint.
     *
     * Get settings for a [group](https://support.zoom.us/hc/en-us/articles/204519819-Group-Management-).
     * **Prerequisite**: Pro, Business, or Education account<br>
     * **Scopes**: `group:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `option: crate::types::OptionData` -- Use the following options to filter the results of the account's information:
     *  \* `meeting_authentication` — View the account's [meeting authentication settings](https://support.zoom.us/hc/en-us/articles/360037117472-Authentication-Profiles-for-Meetings-and-Webinars).
     *  \* `recording_authentication` — View the account's [recording authentication settings](https://support.zoom.us/hc/en-us/articles/360037756671-Authentication-Profiles-for-Cloud-Recordings).
     *  \* `security` — View the account's security settings. For example, password requirements for user login or two-factor authentication.<br>
     *  \* `meeting_security` — View the account's meeting security settings.
     */
    pub async fn get_settings_group_response(
        &self,
        group_id: &str,
        custom_query_fields: &str,
        option: crate::types::OptionData,
    ) -> Result<crate::types::GetGroupSettingsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !custom_query_fields.is_empty() {
            query_args.push((
                "custom_query_fields".to_string(),
                custom_query_fields.to_string(),
            ));
        }
        if !option.to_string().is_empty() {
            query_args.push(("option".to_string(), option.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/groups/{}/settings?{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get a group's settings.
     *
     * This function performs a `GET` to the `/groups/{groupId}/settings` endpoint.
     *
     * Get settings for a [group](https://support.zoom.us/hc/en-us/articles/204519819-Group-Management-).
     * **Prerequisite**: Pro, Business, or Education account<br>
     * **Scopes**: `group:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `option: crate::types::OptionData` -- Use the following options to filter the results of the account's information:
     *  \* `meeting_authentication` — View the account's [meeting authentication settings](https://support.zoom.us/hc/en-us/articles/360037117472-Authentication-Profiles-for-Meetings-and-Webinars).
     *  \* `recording_authentication` — View the account's [recording authentication settings](https://support.zoom.us/hc/en-us/articles/360037756671-Authentication-Profiles-for-Cloud-Recordings).
     *  \* `security` — View the account's security settings. For example, password requirements for user login or two-factor authentication.<br>
     *  \* `meeting_security` — View the account's meeting security settings.
     */
    pub async fn get_setting(
        &self,
        group_id: &str,
        custom_query_fields: &str,
        option: crate::types::OptionData,
    ) -> Result<crate::types::GetGroupSettingsResponseOneOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !custom_query_fields.is_empty() {
            query_args.push((
                "custom_query_fields".to_string(),
                custom_query_fields.to_string(),
            ));
        }
        if !option.to_string().is_empty() {
            query_args.push(("option".to_string(), option.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/groups/{}/settings?{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Update a group's settings.
     *
     * This function performs a `PATCH` to the `/groups/{groupId}/settings` endpoint.
     *
     * Update settings for a [group](https://support.zoom.us/hc/en-us/articles/204519819-Group-Management-).<p style="background-color:#FEEFB3; color:#9F6000"><br>Note:</b> The `force_pmi_jbh_password` field under meeting settings is planned to be deprecated on September 22, 2019. This field will be replaced by another field that will provide the same functionality.</p>
     * **Prerequisite**: Pro, Business, or Education account<br>
     * **Scopes**: `group:write:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `group_id: &str` -- User's first name.
     * * `option: crate::types::UpdateGroupSettingsOption`
     */
    pub async fn update_settings(
        &self,
        group_id: &str,
        custom_query_fields: &str,
        option: crate::types::UpdateGroupSettingsOption,
        body: &crate::types::UpdateGroupSettingsRequestOneOf,
    ) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !custom_query_fields.is_empty() {
            query_args.push((
                "custom_query_fields".to_string(),
                custom_query_fields.to_string(),
            ));
        }
        if !option.to_string().is_empty() {
            query_args.push(("option".to_string(), option.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/groups/{}/settings?{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
            query_
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get locked settings.
     *
     * This function performs a `GET` to the `/groups/{groupId}/lock_settings` endpoint.
     *
     * Retrieve a [group's](https://support.zoom.us/hc/en-us/articles/204519819-Group-Management-) locked settings. If you lock a setting, the group members will not be able to modify it individually. <p style="background-color:#FEEFB3; color:#9F6000"><br>Note:</b> The `force_pmi_jbh_password` field under meeting settings is planned to be deprecated on September 22, 2019. This field will be replaced by another field that will provide the same functionality.</p>
     *
     * **Prerequisite**: Pro, Business, or Education account<br>
     * **Scopes**: `group:read:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `group_id: &str` -- User's first name.
     * * `option: &str` -- Specify `meeting_security` as the value of this field if you would like to view security settings applied on a meeting hosted by the users in this group.
     */
    pub async fn get_lock_settings_meeting_security(
        &self,
        group_id: &str,
        custom_query_fields: &str,
        option: &str,
    ) -> Result<crate::types::MeetingSecuritySettings> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !custom_query_fields.is_empty() {
            query_args.push((
                "custom_query_fields".to_string(),
                custom_query_fields.to_string(),
            ));
        }
        if !option.is_empty() {
            query_args.push(("option".to_string(), option.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/groups/{}/lock_settings?{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get locked settings.
     *
     * This function performs a `GET` to the `/groups/{groupId}/lock_settings` endpoint.
     *
     * Retrieve a [group's](https://support.zoom.us/hc/en-us/articles/204519819-Group-Management-) locked settings. If you lock a setting, the group members will not be able to modify it individually. <p style="background-color:#FEEFB3; color:#9F6000"><br>Note:</b> The `force_pmi_jbh_password` field under meeting settings is planned to be deprecated on September 22, 2019. This field will be replaced by another field that will provide the same functionality.</p>
     *
     * **Prerequisite**: Pro, Business, or Education account<br>
     * **Scopes**: `group:read:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `group_id: &str` -- User's first name.
     * * `option: &str` -- Specify `meeting_security` as the value of this field if you would like to view security settings applied on a meeting hosted by the users in this group.
     */
    pub async fn get_lock_settings_group_response(
        &self,
        group_id: &str,
        custom_query_fields: &str,
        option: &str,
    ) -> Result<crate::types::GetGroupLockSettingsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !custom_query_fields.is_empty() {
            query_args.push((
                "custom_query_fields".to_string(),
                custom_query_fields.to_string(),
            ));
        }
        if !option.is_empty() {
            query_args.push(("option".to_string(), option.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/groups/{}/lock_settings?{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get locked settings.
     *
     * This function performs a `GET` to the `/groups/{groupId}/lock_settings` endpoint.
     *
     * Retrieve a [group's](https://support.zoom.us/hc/en-us/articles/204519819-Group-Management-) locked settings. If you lock a setting, the group members will not be able to modify it individually. <p style="background-color:#FEEFB3; color:#9F6000"><br>Note:</b> The `force_pmi_jbh_password` field under meeting settings is planned to be deprecated on September 22, 2019. This field will be replaced by another field that will provide the same functionality.</p>
     *
     * **Prerequisite**: Pro, Business, or Education account<br>
     * **Scopes**: `group:read:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `group_id: &str` -- User's first name.
     * * `option: &str` -- Specify `meeting_security` as the value of this field if you would like to view security settings applied on a meeting hosted by the users in this group.
     */
    pub async fn get_lock_setting(
        &self,
        group_id: &str,
        custom_query_fields: &str,
        option: &str,
    ) -> Result<crate::types::GetGroupLockSettingsResponseOneOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !custom_query_fields.is_empty() {
            query_args.push((
                "custom_query_fields".to_string(),
                custom_query_fields.to_string(),
            ));
        }
        if !option.is_empty() {
            query_args.push(("option".to_string(), option.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/groups/{}/lock_settings?{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Update locked settings.
     *
     * This function performs a `PATCH` to the `/groups/{groupId}/lock_settings` endpoint.
     *
     * Update a [group's](https://support.zoom.us/hc/en-us/articles/204519819-Group-Management-) locked settings. If you lock a setting, the group members will not be able to modify it individually. <p style="background-color:#FEEFB3; color:#9F6000"><br>Note:</b> The `force_pmi_jbh_password` field under meeting settings is planned to be deprecated on September 22, 2019. This field will be replaced by another field that will provide the same functionality.</p>
     *
     * **Prerequisite**: Pro, Business, or Education account<br>
     * **Scopes**: `group:write:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `group_id: &str` -- User's first name.
     * * `option: &str` -- Specify `meeting_security` as the value of this field if you would like to view security settings applied on a meeting hosted by the users in this group.
     */
    pub async fn locked_settings(
        &self,
        group_id: &str,
        custom_query_fields: &str,
        option: &str,
        body: &crate::types::GroupLockedSettingsRequestOneOf,
    ) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !custom_query_fields.is_empty() {
            query_args.push((
                "custom_query_fields".to_string(),
                custom_query_fields.to_string(),
            ));
        }
        if !option.is_empty() {
            query_args.push(("option".to_string(), option.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/groups/{}/lock_settings?{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
            query_
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Upload virtual background files.
     *
     * This function performs a `POST` to the `/groups/{groupId}/settings/virtual_backgrounds` endpoint.
     *
     * Use this API to [upload virtual background files](https://support.zoom.us/hc/en-us/articles/210707503-Virtual-Background#h_01EJF3YFEWGT8YA0ZJ079JEDQE) for all users in a group to use.
     *
     *
     * **Prerequisites:**<br>
     * * Virtual background feature must be [enabled](https://support.zoom.us/hc/en-us/articles/210707503-Virtual-Background#h_2ef28080-fce9-4ac2-b567-dc958afab1b7) on the account.
     * <br> **Scope:** `group:write:admin`<br><br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     *
     *
     * `
     *
     * **Parameters:**
     *
     * * `group_id: &str` -- Unique identifier of the group. Retrieve the value for this field by calling the [List groups](https://marketplace.zoom.us/docs/api-reference/zoom-api/groups/groups) API.
     */
    pub async fn upload_vb(
        &self,
        file_ids: &str,
        group_id: &str,
        body: &crate::types::UploadVbRequest,
    ) -> Result<crate::types::Files> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !file_ids.is_empty() {
            query_args.push(("file_ids".to_string(), file_ids.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/groups/{}/settings/virtual_backgrounds?{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
            query_
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete virtual background files.
     *
     * This function performs a `DELETE` to the `/groups/{groupId}/settings/virtual_backgrounds` endpoint.
     *
     * Delete existing virtual background file(s) from an account.
     *
     * **Prerequisites:**<br>
     * * Virtual background feature must be [enabled](https://support.zoom.us/hc/en-us/articles/210707503-Virtual-Background#h_2ef28080-fce9-4ac2-b567-dc958afab1b7) on the account.
     * <br> **Scope:** `group:write:admin`<br><br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     *
     *
     *
     * **Parameters:**
     *
     * * `group_id: &str` -- Unique identifier of the group. Retrieve the value for this field by calling the [List groups](https://marketplace.zoom.us/docs/api-reference/zoom-api/groups/groups) API.
     * * `file_ids: &str` -- Provide the id of the file that is to be deleted. To delete multiple files, provide comma separated values for this field.
     */
    pub async fn del_vb(&self, file_ids: &str, group_id: &str) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !file_ids.is_empty() {
            query_args.push(("file_ids".to_string(), file_ids.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/groups/{}/settings/virtual_backgrounds?{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
            query_
        );

        self.client.delete(&url, None).await
    }
}
