use anyhow::Result;

use crate::Client;

pub struct ImGroups {
    pub client: Client,
}

impl ImGroups {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ImGroups { client }
    }

    /**
     * List IM directory groups.
     *
     * This function performs a `GET` to the `/im/groups` endpoint.
     *
     * List [IM directory groups](https://support.zoom.us/hc/en-us/articles/203749815-IM-Management).<br><br>
     * **Scopes**: `imgroup:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     */
    pub async fn get(&self) -> Result<crate::types::Domains> {
        let url = "/im/groups".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Create an IM directory group.
     *
     * This function performs a `POST` to the `/im/groups` endpoint.
     *
     * Create an [IM directory group](https://support.zoom.us/hc/en-us/articles/203749815-IM-Management) under your account.<br><br>
     * **Scopes**: `imgroup:write:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     */
    pub async fn create(&self, body: &crate::types::ImGroupCreateRequest) -> Result<()> {
        let url = "/im/groups".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Retrieve an IM directory group.
     *
     * This function performs a `GET` to the `/im/groups/{groupId}` endpoint.
     *
     * Retrieve an [IM directory group](https://support.zoom.us/hc/en-us/articles/203749815-IM-Management) under your account.<br><br>
     * Scopes: `imgroup:read:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `group_id: &str` -- The group ID.<br>
     *   Can be retrieved by calling [GET /groups](https://marketplace.zoom.us/docs/api-reference/zoom-api/groups/groups).
     */
    pub async fn im_group(&self, group_id: &str) -> Result<crate::types::ImGroupResponseAllOf> {
        let url = format!(
            "/im/groups/{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete an IM directory group.
     *
     * This function performs a `DELETE` to the `/im/groups/{groupId}` endpoint.
     *
     * Delete an [IM directory group](https://support.zoom.us/hc/en-us/articles/203749815-IM-Management) under your account.<br><br>
     * Scopes: `imgroup:write:admin`<br>
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
            "/im/groups/{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update an IM directory group.
     *
     * This function performs a `PATCH` to the `/im/groups/{groupId}` endpoint.
     *
     * Update an [IM directory group](https://support.zoom.us/hc/en-us/articles/203749815-IM-Management) under your account.<br><br>
     * **Scopes**: `imgroup:write:admin`<br>
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
        body: &crate::types::ImGroupCreateRequest,
    ) -> Result<()> {
        let url = format!(
            "/im/groups/{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * List IM directory group members.
     *
     * This function performs a `GET` to the `/im/groups/{groupId}/members` endpoint.
     *
     * List the members of an [IM directory group](https://support.zoom.us/hc/en-us/articles/203749815-IM-Management).<br><br>
     * **Scope:** `imgroup:read:admin`<br>
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
    pub async fn member(
        &self,
        group_id: &str,
        page_size: i64,
        page_number: i64,
        next_page_token: &str,
    ) -> Result<crate::types::Domains> {
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
            "/im/groups/{}/members?{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Add IM directory group members.
     *
     * This function performs a `POST` to the `/im/groups/{groupId}/members` endpoint.
     *
     * Add members to an [IM directory group](https://support.zoom.us/hc/en-us/articles/203749815-IM-Management) under an account.<br><br>
     * **Scope:** `imgroup:write:admin`<br>
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
            "/im/groups/{}/members",
            crate::progenitor_support::encode_path(&group_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete IM directory group member.
     *
     * This function performs a `DELETE` to the `/im/groups/{groupId}/members/{memberId}` endpoint.
     *
     * Delete a member from an [IM directory group](https://support.zoom.us/hc/en-us/articles/203749815-IM-Management) under an account.<br><br>
     * Scopes: `imgroup:write:admin`<br>
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
            "/im/groups/{}/members/{}",
            crate::progenitor_support::encode_path(&group_id.to_string()),
            crate::progenitor_support::encode_path(&member_id.to_string()),
        );

        self.client.delete(&url, None).await
    }
}
