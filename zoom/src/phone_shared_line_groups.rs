use anyhow::Result;

use crate::Client;

pub struct PhoneSharedLineGroups {
    pub client: Client,
}

impl PhoneSharedLineGroups {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        PhoneSharedLineGroups { client }
    }

    /**
     * List shared line groups.
     *
     * This function performs a `GET` to the `/phone/shared_line_groups` endpoint.
     *
     * A [shared line group](https://support.zoom.us/hc/en-us/articles/360038850792) allows Zoom Phone admins to share a phone number and extension with a group of phone users or common area phones. This gives members of the shared line group access to the group's direct phone number and voicemail. Use this API to list all the Shared Line Groups.
     *
     * **Prerequisties:** <br>
     * * Pro or higher account with Zoom Phone license.
     * * Account owner or admin privileges  <br>
     *
     * **Scopes:** `phone:read:admin`, `phone:write:admin`
     *
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn list_shared_line_groups(
        &self,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<Vec<crate::types::SharedLineGroups>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/shared_line_groups?{}", query_);

        let resp: crate::types::ListSharedLineGroupsResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.shared_line_groups)
    }

    /**
     * List shared line groups.
     *
     * This function performs a `GET` to the `/phone/shared_line_groups` endpoint.
     *
     * As opposed to `list_shared_line_groups`, this function returns all the pages of the request at once.
     *
     * A [shared line group](https://support.zoom.us/hc/en-us/articles/360038850792) allows Zoom Phone admins to share a phone number and extension with a group of phone users or common area phones. This gives members of the shared line group access to the group's direct phone number and voicemail. Use this API to list all the Shared Line Groups.
     *
     * **Prerequisties:** <br>
     * * Pro or higher account with Zoom Phone license.
     * * Account owner or admin privileges  <br>
     *
     * **Scopes:** `phone:read:admin`, `phone:write:admin`
     *
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     */
    pub async fn list_all_shared_line_groups(&self) -> Result<Vec<crate::types::SharedLineGroups>> {
        let url = "/phone/shared_line_groups".to_string();
        let mut resp: crate::types::ListSharedLineGroupsResponse =
            self.client.get(&url, None).await?;

        let mut shared_line_groups = resp.shared_line_groups;
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

            shared_line_groups.append(&mut resp.shared_line_groups);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(shared_line_groups)
    }

    /**
     * Create a shared line group.
     *
     * This function performs a `POST` to the `/phone/shared_line_groups` endpoint.
     *
     * A [shared line group](https://support.zoom.us/hc/en-us/articles/360038850792) allows Zoom Phone admins to share a phone number and extension with a group of phone users or common area phones. This gives members of the shared line group access to the group's direct phone number and voicemail. Use this API to create a shared line group.
     *
     * **Prerequisties:** <br>
     * * Pro or higher account with Zoom Phone license.
     * * Account owner or admin privileges
     *
     * **Scopes:** `phone:write:admin`
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     */
    pub async fn create_shared_line_group(
        &self,
        body: &crate::types::CreateSharedLineGroupRequest,
    ) -> Result<()> {
        let url = "/phone/shared_line_groups".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get a shared line group.
     *
     * This function performs a `GET` to the `/phone/shared_line_groups/{sharedLineGroupId}` endpoint.
     *
     * A [shared line group](https://support.zoom.us/hc/en-us/articles/360038850792) allows Zoom Phone admins to share a phone number and extension with a group of phone users or common area phones. This gives members of the shared line group access to the group's direct phone number and voicemail. Use this API to list all the Shared Line Groups.
     *
     * **Prerequisties:** <br>
     * * Pro or higher account with Zoom Phone license.
     * * Account owner or admin privileges
     *
     * **Scopes:** `phone:read:admin` or `phone:write:admin`
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `shared_line_group_id: &str` -- Unique Identifier of the Shared Line Group.
     */
    pub async fn get_shared_line_group(
        &self,
        shared_line_group_id: &str,
    ) -> Result<crate::types::GetSharedLineGroupResponse> {
        let url = format!(
            "/phone/shared_line_groups/{}",
            crate::progenitor_support::encode_path(&shared_line_group_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a shared line group.
     *
     * This function performs a `DELETE` to the `/phone/shared_line_groups/{sharedLineGroupId}` endpoint.
     *
     * A [shared line group](https://support.zoom.us/hc/en-us/articles/360038850792) allows Zoom Phone admins to share a phone number and extension with a group of phone users or common area phones. Use this API to delete a Shared Line Group.
     * **Prerequisties:** <br>
     * * Pro or higher account with Zoom Phone license.
     * * Account owner or admin privileges
     *
     * **Scopes:** `phone:write:admin`
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `shared_line_group_id: &str` -- Unique Identifier of the shared line group that you would like to delete.
     */
    pub async fn delete_shared_line_group(&self, shared_line_group_id: &str) -> Result<()> {
        let url = format!(
            "/phone/shared_line_groups/{}",
            crate::progenitor_support::encode_path(&shared_line_group_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update a shared line group.
     *
     * This function performs a `PATCH` to the `/phone/shared_line_groups/{sharedLineGroupId}` endpoint.
     *
     * A [shared line group](https://support.zoom.us/hc/en-us/articles/360038850792) allows Zoom Phone admins to share a phone number and extension with a group of phone users or common area phones. This gives members of the shared line group access to the group's direct phone number and voicemail. Use this API to update information of a Shared Line Group.
     * **Prerequisties:** <br>
     * * Pro or higher account with Zoom Phone license.
     * * Account owner or admin privileges
     *
     * **Scopes:** `phone:write:admin`
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `shared_line_group_id: &str` -- Unique identifier of the shared line group that is to be updated.
     */
    pub async fn update_shared_line_group(
        &self,
        shared_line_group_id: &str,
        body: &crate::types::UpdateSharedLineGroupRequest,
    ) -> Result<()> {
        let url = format!(
            "/phone/shared_line_groups/{}",
            crate::progenitor_support::encode_path(&shared_line_group_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Add members to a shared line group.
     *
     * This function performs a `POST` to the `/phone/shared_line_groups/{sharedLineGroupId}/members` endpoint.
     *
     * A [shared line group](https://support.zoom.us/hc/en-us/articles/360038850792) allows Zoom Phone admins to share a phone number and extension with a group of phone users or common area phones. This gives members of the shared line group access to the group's direct phone number and voicemail. Use this API to [add members](https://support.zoom.us/hc/en-us/articles/360038850792-Setting-up-shared-line-groups#h_7cb42370-48f6-4a8f-84f4-c6eee4d9f0ca) to a Shared Line Group. Note that a member can only be added to one shared line group.
     *
     * **Prerequisties:** <br>
     * * Pro or higher account with Zoom Phone license.
     * * A valid Shared Line Group
     * * Account owner or admin privileges
     *
     * **Scopes:** `phone:write:admin`
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `shared_line_group_id: &str` -- Unique Identifier of the shared line group.
     */
    pub async fn add_members_shared_line_group(
        &self,
        shared_line_group_id: &str,
        body: &crate::types::AddMembersSharedLineGroupRequestData,
    ) -> Result<()> {
        let url = format!(
            "/phone/shared_line_groups/{}/members",
            crate::progenitor_support::encode_path(&shared_line_group_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Unassign members of a shared line group.
     *
     * This function performs a `DELETE` to the `/phone/shared_line_groups/{sharedLineGroupId}/members` endpoint.
     *
     * Members of the [shared line group](https://support.zoom.us/hc/en-us/articles/360038850792) have access to the group's phone number and voicemail. Use this API to unassign **all** the existing members from a Shared Line Group.
     * **Prerequisties:** <br>
     * * Pro or higher account with Zoom Phone license.
     * * A valid Shared Line Group
     * * Account owner or admin privileges
     *
     * **Scopes:** `phone:write:admin`
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `shared_line_group_id: &str` -- Unique identifier of the Shared Line Group that you would like to delete.
     */
    pub async fn delete_members_of_slg(&self, shared_line_group_id: &str) -> Result<()> {
        let url = format!(
            "/phone/shared_line_groups/{}/members",
            crate::progenitor_support::encode_path(&shared_line_group_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Unassign a member from a shared line group.
     *
     * This function performs a `DELETE` to the `/phone/shared_line_groups/{sharedLineGroupId}/members/{memberId}` endpoint.
     *
     * Members of the [shared line group](https://support.zoom.us/hc/en-us/articles/360038850792) have access to the group's phone number and voicemail. Use this API to unassign **a specific member** from a Shared Line Group.
     * **Prerequisties:** <br>
     * * Pro or higher account with Zoom Phone license.
     * * A valid Shared Line Group
     * * Account owner or admin privileges
     *
     * **Scopes:** `phone:write:admin`
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `shared_line_group_id: &str` -- Unique Identifier of the shared line group from which you would like to remove a member.
     * * `member_id: &str` -- Unique identifier of the member who is to be removed.
     */
    pub async fn delete_member_slg(
        &self,
        shared_line_group_id: &str,
        member_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/phone/shared_line_groups/{}/members/{}",
            crate::progenitor_support::encode_path(&shared_line_group_id.to_string()),
            crate::progenitor_support::encode_path(&member_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Assign phone numbers.
     *
     * This function performs a `POST` to the `/phone/shared_line_groups/{sharedLineGroupId}/phone_numbers` endpoint.
     *
     * Use this API to assign phone numbers to a shared line groups. These direct phone numbers will be shared among members of the [shared line group](https://support.zoom.us/hc/en-us/articles/360038850792-Setting-up-shared-line-groups).
     * **Prerequisties:** <br>
     * * Pro or higher account with Zoom Phone license.
     * * A valid Shared Line Group
     * * Account owner or admin privileges
     *
     * **Scopes:** `phone:write:admin`
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `shared_line_group_id: &str` -- Unique Identifier of the Shared Line Group.
     */
    pub async fn assign_phone_numbers_slg(
        &self,
        shared_line_group_id: &str,
        body: &crate::types::AddByocNumberResponse,
    ) -> Result<()> {
        let url = format!(
            "/phone/shared_line_groups/{}/phone_numbers",
            crate::progenitor_support::encode_path(&shared_line_group_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Unassign all phone numbers.
     *
     * This function performs a `DELETE` to the `/phone/shared_line_groups/{sharedLineGroupId}/phone_numbers` endpoint.
     *
     * Use this API to unassign all the phone numbers that have been assigned to the [shared line group](https://support.zoom.us/hc/en-us/articles/360038850792-Setting-up-shared-line-groups).
     * **Prerequisties:** <br>
     * * Pro or higher account with Zoom Phone license.
     * * A valid Shared Line Group
     * * Account owner or admin privileges
     *
     * **Scopes:** `phone:write:admin`
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `shared_line_group_id: &str` -- Unique Identifier of the Shared Line Group.
     */
    pub async fn delete_phone_numbers_slg(&self, shared_line_group_id: &str) -> Result<()> {
        let url = format!(
            "/phone/shared_line_groups/{}/phone_numbers",
            crate::progenitor_support::encode_path(&shared_line_group_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Unassign a phone number.
     *
     * This function performs a `DELETE` to the `/phone/shared_line_groups/{sharedLineGroupId}/phone_numbers/{phoneNumberId}` endpoint.
     *
     * Use this API to unassign a specific phone number that was assigned to the [shared line group](https://support.zoom.us/hc/en-us/articles/360038850792-Setting-up-shared-line-groups).
     * **Prerequisties:** <br>
     * * Pro or higher account with Zoom Phone license.
     * * A valid Shared Line Group
     * * Account owner or admin privileges
     *
     * **Scopes:** `phone:write:admin`
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `shared_line_group_id: &str` -- Unique identifier of the shared line group from which you would like to unassign a phone number.
     * * `phone_number_id: &str` -- Unique identifier of the phone number which is to be unassigned. This can be retrieved from Get a Shared Line Group API.
     */
    pub async fn delete_phone_number_slg(
        &self,
        shared_line_group_id: &str,
        phone_number_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/phone/shared_line_groups/{}/phone_numbers/{}",
            crate::progenitor_support::encode_path(&shared_line_group_id.to_string()),
            crate::progenitor_support::encode_path(&phone_number_id.to_string()),
        );

        self.client.delete(&url, None).await
    }
}
