use anyhow::Result;

use crate::Client;

pub struct Roles {
    pub client: Client,
}

impl Roles {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Roles { client }
    }

    /**
     * List roles.
     *
     * This function performs a `GET` to the `/roles` endpoint.
     *
     * List [roles](https://support.zoom.us/hc/en-us/articles/115001078646-Role-Based-Access-Control) on your account
     *
     * **Scopes:** `role:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     * **Prerequisites** :
     * *  Pro or higher plan.
     * *  For setting the initial role, you must be the Account Owner.
     * *  For subsequent role management, you must be the Account Owner or user with role management permissions.
     */
    pub async fn get(&self) -> Result<crate::types::Domains> {
        let url = "/roles".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Create a role.
     *
     * This function performs a `POST` to the `/roles` endpoint.
     *
     * Each Zoom user automatically has a role which can either be owner, administrator, or a member.
     *
     * **Pre-requisite:**<br>
     * * Pro or higher plan.
     * * For setting the initial role, you must be the Account Owner.<br>
     * * For subsequent role management, you must be the Account Owner or user with role management permissions.<br>
     * **Scopes:** `role:write:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     */
    pub async fn create(&self, body: &crate::types::CreateRoleRequest) -> Result<()> {
        let url = "/roles".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * List members in a role.
     *
     * This function performs a `GET` to the `/roles/{roleId}/members` endpoint.
     *
     * User [roles](https://support.zoom.us/hc/en-us/articles/115001078646-Role-Based-Access-Control) can have a set of permissions that allows access only to the pages a user needs to view or edit. Use this API to list all the members that are assigned a specific role.
     *
     * **Scope:** `role:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>**Prerequisites:**<br>
     * * A Pro or a higher plan.
     *
     * **Parameters:**
     *
     * * `role_id: &str` -- User's first name.
     * * `page_count: &str` -- The number of pages returned for this request.
     * * `page_number: i64` --
     *   **Deprecated** - This field has been deprecated and we will stop supporting it completely in a future release. Please use "next_page_token" for pagination instead of this field.
     *   
     *   The page number of the current page in the returned records.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `page_size: i64` -- The number of records returned within a single API call.
     */
    pub async fn members(
        &self,
        role_id: &str,
        page_count: &str,
        page_number: i64,
        next_page_token: &str,
        page_size: i64,
    ) -> Result<Vec<crate::types::Domains>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if !page_count.is_empty() {
            query_args.push(("page_count".to_string(), page_count.to_string()));
        }
        if page_number > 0 {
            query_args.push(("page_number".to_string(), page_number.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/roles/{}/members?{}",
            crate::progenitor_support::encode_path(&role_id.to_string()),
            query_
        );

        let resp: crate::types::RoleMembersList = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.members)
    }

    /**
     * List members in a role.
     *
     * This function performs a `GET` to the `/roles/{roleId}/members` endpoint.
     *
     * As opposed to `members`, this function returns all the pages of the request at once.
     *
     * User [roles](https://support.zoom.us/hc/en-us/articles/115001078646-Role-Based-Access-Control) can have a set of permissions that allows access only to the pages a user needs to view or edit. Use this API to list all the members that are assigned a specific role.
     *
     * **Scope:** `role:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>**Prerequisites:**<br>
     * * A Pro or a higher plan.
     */
    pub async fn get_all_members(
        &self,
        role_id: &str,
        page_count: &str,
    ) -> Result<Vec<crate::types::Domains>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !page_count.is_empty() {
            query_args.push(("page_count".to_string(), page_count.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/roles/{}/members?{}",
            crate::progenitor_support::encode_path(&role_id.to_string()),
            query_
        );

        let mut resp: crate::types::RoleMembersList = self.client.get(&url, None).await?;

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
     * Assign a role.
     *
     * This function performs a `POST` to the `/roles/{roleId}/members` endpoint.
     *
     * User [roles](https://support.zoom.us/hc/en-us/articles/115001078646-Role-Based-Access-Control) can have a set of permissions that allows access only to the pages a user needs to view or edit. Use this API to [assign a role](https://support.zoom.us/hc/en-us/articles/115001078646-Role-Based-Access-Control#h_748b6fd8-5057-4cf4-bbfd-787909c09db0) to members.
     *
     * **Scopes:** `role:write:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     * **Prerequisites:**<br>
     * * A Pro or a higher plan.
     *
     * **Parameters:**
     *
     * * `role_id: &str` -- User's first name.
     */
    pub async fn add_members(
        &self,
        role_id: &str,
        body: &crate::types::AddRoleMembersRequest,
    ) -> Result<crate::types::AddRoleMembersResponse> {
        let url = format!(
            "/roles/{}/members",
            crate::progenitor_support::encode_path(&role_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Unassign a role.
     *
     * This function performs a `DELETE` to the `/roles/{roleId}/members/{memberId}` endpoint.
     *
     * User [roles](https://support.zoom.us/hc/en-us/articles/115001078646-Role-Based-Access-Control) can have a set of permissions that allows access only to the pages a user needs to view or edit. Use this API to unassign a user's role.
     *
     * **Scope:** `role:write:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     * **Prerequisites:**<br>
     * * A Pro or a higher plan.
     *
     * **Parameters:**
     *
     * * `role_id: &str` -- User's first name.
     * * `member_id: &str` -- User's first name.
     */
    pub async fn member_delete(&self, role_id: &str, member_id: &str) -> Result<()> {
        let url = format!(
            "/roles/{}/members/{}",
            crate::progenitor_support::encode_path(&role_id.to_string()),
            crate::progenitor_support::encode_path(&member_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Get role information.
     *
     * This function performs a `GET` to the `/roles/{roleId}` endpoint.
     *
     * Each Zoom user automatically has a role which can either be owner, administrator, or a member. Account Owners and users with edit privileges for Role management can add customized roles with a list of privileges.
     *
     * Use this API to get information including specific privileges assigned to a [role](https://support.zoom.us/hc/en-us/articles/115001078646-Role-Based-Access-Control).<br>
     * **Pre-requisite:**<br>
     * * A Pro or higher plan.<br>
     * * For role management and updates, you must be the Account Owner or user with role management permissions.
     *
     * **Scopes:** `role:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `role_id: &str` -- User's first name.
     */
    pub async fn get_information(
        &self,
        role_id: &str,
    ) -> Result<crate::types::GetRoleInformationResponse> {
        let url = format!(
            "/roles/{}",
            crate::progenitor_support::encode_path(&role_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a role.
     *
     * This function performs a `DELETE` to the `/roles/{roleId}` endpoint.
     *
     * Each Zoom user automatically has a role which can either be owner, administrator, or a member. Account Owners and users with edit privileges for Role management can add customized roles with a list.
     *
     * Use this API to delete a role.<br>
     * **Pre-requisite:**<br>
     * * A Pro or higher plan.<br>
     * * For role management and updates, you must be the Account Owner or user with role management permissions.
     *
     * **Scopes:** `role:write:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `role_id: &str` -- User's first name.
     */
    pub async fn delete(&self, role_id: &str) -> Result<()> {
        let url = format!(
            "/roles/{}",
            crate::progenitor_support::encode_path(&role_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update role information.
     *
     * This function performs a `PATCH` to the `/roles/{roleId}` endpoint.
     *
     * Each Zoom user automatically has a [role](https://support.zoom.us/hc/en-us/articles/115001078646-Role-Based-Access-Control) which can either be owner, administrator, or a member. Account Owners and users with edit privileges for Role management can add customized roles with a list.
     *
     * Use this API to change the privileges, name and description of a specific role.<br>
     * **Pre-requisite:**<br>
     * * A Pro or higher plan.<br>
     * * For role management and updates, you must be the Account Owner or user with role management permissions.<br>**Scopes:** `role:write:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `role_id: &str` -- User's first name.
     */
    pub async fn update(
        &self,
        role_id: &str,
        body: &crate::types::UpdateRoleRequest,
    ) -> Result<()> {
        let url = format!(
            "/roles/{}",
            crate::progenitor_support::encode_path(&role_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
