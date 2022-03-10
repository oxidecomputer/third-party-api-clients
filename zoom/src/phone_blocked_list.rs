use anyhow::Result;

use crate::Client;

pub struct PhoneBlockedList {
    pub client: Client,
}

impl PhoneBlockedList {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        PhoneBlockedList { client }
    }

    /**
     * List blocked lists.
     *
     * This function performs a `GET` to the `/phone/blocked_list` endpoint.
     *
     * A Zoom account owner or a user with admin privilege can block phone numbers for phone users in an account. Blocked numbers can be inbound (numbers will be blocked from calling in) and outbound (phone users in your account won't be able to dial those numbers). Blocked callers will hear a generic message stating that the person they are calling is not available.<br>Use this API to list all the blocked lists in an acccount.<br>
     * **Prerequisites:**
     * * Pro or higher account plan with Zoom phone license<br>
     * **Scope:** `phone:read:admin`<br>
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `page_size: i64` -- The total number of records returned from a single API call.
     */
    pub async fn list_blocked(
        &self,
        next_page_token: &str,
        page_size: i64,
    ) -> Result<Vec<crate::types::BlockedList>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/blocked_list?{}", query_);

        let resp: crate::types::ListBlockedResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.blocked_list)
    }

    /**
     * List blocked lists.
     *
     * This function performs a `GET` to the `/phone/blocked_list` endpoint.
     *
     * As opposed to `list_blocked`, this function returns all the pages of the request at once.
     *
     * A Zoom account owner or a user with admin privilege can block phone numbers for phone users in an account. Blocked numbers can be inbound (numbers will be blocked from calling in) and outbound (phone users in your account won't be able to dial those numbers). Blocked callers will hear a generic message stating that the person they are calling is not available.<br>Use this API to list all the blocked lists in an acccount.<br>
     * **Prerequisites:**
     * * Pro or higher account plan with Zoom phone license<br>
     * **Scope:** `phone:read:admin`<br>
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     */
    pub async fn list_all_blocked(&self) -> Result<Vec<crate::types::BlockedList>> {
        let url = "/phone/blocked_list".to_string();
        let mut resp: crate::types::ListBlockedResponse = self.client.get(&url, None).await?;

        let mut blocked_list = resp.blocked_list;
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

            blocked_list.append(&mut resp.blocked_list);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(blocked_list)
    }

    /**
     * Create a blocked list.
     *
     * This function performs a `POST` to the `/phone/blocked_list` endpoint.
     *
     * A Zoom account owner or a user with admin privilege can block phone numbers for phone users in an account. Blocked numbers can be inbound (numbers will be blocked from calling in) and outbound (phone users in your account won't be able to dial those numbers). Blocked callers will hear a generic message stating that the person they are calling is not available.<br>Use this API to create a blocked list and add a number to that blocked list.<br>
     * **Prerequisites:**
     * * Pro or higher account plan with Zoom phone license<br>
     * **Scope:** `phone:write:admin`<br>
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     */
    pub async fn add_anumber_blocked_list(
        &self,
        body: &crate::types::UpdateBlockedListRequest,
    ) -> Result<crate::types::Groups> {
        let url = "/phone/blocked_list".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get blocked list details.
     *
     * This function performs a `GET` to the `/phone/blocked_list/{blockedListId}` endpoint.
     *
     * A Zoom account owner or a user with admin privilege can block phone numbers for phone users in an account. Blocked numbers can be inbound (numbers will be blocked from calling in) and outbound (phone users in your account won't be able to dial those numbers). Blocked callers will hear a generic message stating that the person they are calling is not available.<br>Use this API to get information about a specific blocked list.<br>
     * **Prerequisites:**
     * * Pro or higher account plan with Zoom phone license<br>
     * **Scope:** `phone:read:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `blocked_list_id: &str` -- Unique Identifier of the blocked list.
     */
    pub async fn get_blocked_list(
        &self,
        blocked_list_id: &str,
    ) -> Result<crate::types::BlockedList> {
        let url = format!(
            "/phone/blocked_list/{}",
            crate::progenitor_support::encode_path(&blocked_list_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a blocked list.
     *
     * This function performs a `DELETE` to the `/phone/blocked_list/{blockedListId}` endpoint.
     *
     * A Zoom account owner or a user with admin privilege can block phone numbers for phone users in an account. Blocked numbers can be inbound (numbers will be blocked from calling in) and outbound (phone users in your account won't be able to dial those numbers).
     * <br>Use this API to delete a blocked list and therefore removing the associated number from the blocked list. The number will be unblocked after the deletion.<br>
     * **Prerequisites:**
     * * Pro or higher account plan with Zoom phone license<br>
     * **Scope:** `phone:write:admin`<br>
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `blocked_list_id: &str` -- Unique Identifier of the blocked list. This can be retrieved from the List Blocked List API.
     */
    pub async fn delete_blocked_list(&self, blocked_list_id: &str) -> Result<()> {
        let url = format!(
            "/phone/blocked_list/{}",
            crate::progenitor_support::encode_path(&blocked_list_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update a blocked list.
     *
     * This function performs a `PATCH` to the `/phone/blocked_list/{blockedListId}` endpoint.
     *
     * A Zoom account owner or a user with admin privilege can block phone numbers for phone users in an account. Blocked numbers can be inbound (numbers will be blocked from calling in) and outbound (phone users in your account won't be able to dial those numbers). Blocked callers will hear a generic message stating that the person they are calling is not available.<br>Use this API to update information on the blocked list.<br>
     * **Prerequisites:**
     * * Pro or higher account plan with Zoom phone license<br>
     * **Scope:** `phone:write:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     * **Parameters:**
     *
     * * `blocked_list_id: &str` -- Unique Identifier of the blocked list.
     */
    pub async fn update_blocked_list(
        &self,
        blocked_list_id: &str,
        body: &crate::types::UpdateBlockedListRequest,
    ) -> Result<()> {
        let url = format!(
            "/phone/blocked_list/{}",
            crate::progenitor_support::encode_path(&blocked_list_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
