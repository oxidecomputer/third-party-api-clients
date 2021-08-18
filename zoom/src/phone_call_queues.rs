use anyhow::Result;

use crate::Client;

pub struct PhoneCallQueues {
    pub client: Client,
}

impl PhoneCallQueues {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        PhoneCallQueues { client }
    }

    /**
     * List call queues.
     *
     * This function performs a `GET` to the `/phone/call_queues` endpoint.
     *
     * Call queues allow you to route incoming calls to a group of users. For instance, you can use call queues to route calls to various departments in your organization such as sales, engineering, billing, customer service etc.<br> Use this API to list Call queues.<br><br>
     * **Prerequisites:**<br>
     * * Pro, Business, or Education account
     * * Account owner or admin permissions
     * * Zoom Phone license<br>
     * **Scopes:** `phone:read:admin`<br>
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     *
     * **Parameters:**
     *
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `page_size: i64` -- The number of records returned from a single API call.
     */
    pub async fn list_call_queues(
        &self,
        next_page_token: &str,
        page_size: i64,
    ) -> Result<Vec<crate::types::CallQueues>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/call_queues?{}", query_);

        let resp: crate::types::ListCallQueuesResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.call_queues)
    }

    /**
     * List call queues.
     *
     * This function performs a `GET` to the `/phone/call_queues` endpoint.
     *
     * As opposed to `list_call_queues`, this function returns all the pages of the request at once.
     *
     * Call queues allow you to route incoming calls to a group of users. For instance, you can use call queues to route calls to various departments in your organization such as sales, engineering, billing, customer service etc.<br> Use this API to list Call queues.<br><br>
     * **Prerequisites:**<br>
     * * Pro, Business, or Education account
     * * Account owner or admin permissions
     * * Zoom Phone license<br>
     * **Scopes:** `phone:read:admin`<br>
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     */
    pub async fn list_all_call_queues(&self) -> Result<Vec<crate::types::CallQueues>> {
        let url = "/phone/call_queues".to_string();
        let mut resp: crate::types::ListCallQueuesResponse = self.client.get(&url, None).await?;

        let mut call_queues = resp.call_queues;
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

            call_queues.append(&mut resp.call_queues);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(call_queues)
    }

    /**
     * Create a call queue.
     *
     * This function performs a `POST` to the `/phone/call_queues` endpoint.
     *
     * Call queues allow you to route incoming calls to a group of users. For instance, you can use call queues to route calls to various departments in your organization such as sales, engineering, billing, customer service etc.<br> Use this API to [create a call queue](https://support.zoom.us/hc/en-us/articles/360021524831-Managing-Call-Queues#h_e81faeeb-9184-429a-aaea-df49ff5ff413).<br> You can add phone users or common area phones to call queues.
     *
     * **Prerequisites:**<br>
     * * Pro, Business, or Education account
     * * Account owner or admin permissions
     * * Zoom Phone license<br>
     * **Scopes:** `phone:write:admin`<br>
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     */
    pub async fn create_call_queue(
        &self,
        body: &crate::types::CreateCallQueueRequest,
    ) -> Result<crate::types::CreateCallQueueResponse> {
        let url = "/phone/call_queues".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get call queue details.
     *
     * This function performs a `GET` to the `/phone/call_queues/{callQueueId}` endpoint.
     *
     * Call queues allow you to route incoming calls to a group of users. For instance, you can use call queues to route calls to various departments in your organization such as sales, engineering, billing, customer service etc.<br> Use this API to get information on a specific Call Queue.<br><br>
     * **Prerequisites:**<br>
     * * Pro, Business, or Education account
     * * Account owner or admin permissions
     * * Zoom Phone license<br>
     * **Scopes:** `phone:read:admin`<br>
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `call_queue_id: &str` -- Unique Identifier of the Call Queue. This can be retrieved from [List Call Queues API](https://marketplace.zoom.us/docs/api-reference/zoom-api/phone-call-queues/listcallqueues).
     */
    pub async fn get_call_queue(
        &self,
        call_queue_id: &str,
    ) -> Result<crate::types::GetCallQueueResponse> {
        let url = format!(
            "/phone/call_queues/{}",
            crate::progenitor_support::encode_path(&call_queue_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a call queue.
     *
     * This function performs a `DELETE` to the `/phone/call_queues/{callQueueId}` endpoint.
     *
     * Call queues allow you to route incoming calls to a group of users. For instance, you can use call queues to route calls to various departments in your organization such as sales, engineering, billing, customer service etc.<br> Use this API to delete a Call Queue.<br>
     * **Prerequisites:**<br>
     * * Pro, Business, or Education account
     * * Account owner or admin permissions
     * * Zoom Phone license<br>
     * **Scopes:** `phone:write:admin`<br>
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     * **Parameters:**
     *
     * * `call_queue_id: &str` -- Unique Identifier of the call queue.
     */
    pub async fn delete_call_queue(&self, call_queue_id: &str) -> Result<()> {
        let url = format!(
            "/phone/call_queues/{}",
            crate::progenitor_support::encode_path(&call_queue_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update call queue details.
     *
     * This function performs a `PATCH` to the `/phone/call_queues/{callQueueId}` endpoint.
     *
     * Call queues allow you to route incoming calls to a group of users. For instance, you can use call queues to route calls to various departments in your organization such as sales, engineering, billing, customer service etc.<br> Use this API to update information of a specific Call Queue.<br>
     * **Prerequisites:**<br>
     * * Pro, Business, or Education account
     * * Account owner or admin permissions
     * * Zoom Phone license<br>
     * **Scopes:** `phone:write:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     * **Parameters:**
     *
     * * `call_queue_id: &str` -- Unique Identifier of the Call Queue.
     */
    pub async fn update_call_queue(
        &self,
        call_queue_id: &str,
        body: &crate::types::UpdateCallQueueRequest,
    ) -> Result<()> {
        let url = format!(
            "/phone/call_queues/{}",
            crate::progenitor_support::encode_path(&call_queue_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Assign numbers to a call queue.
     *
     * This function performs a `POST` to the `/phone/call_queues/{callQueueId}/phone_numbers` endpoint.
     *
     * After [buying phone number(s)](https://support.zoom.us/hc/en-us/articles/360020808292#h_007ec8c2-0914-4265-8351-96ab23efa3ad), you can assign it, allowing callers to directly dial a number to reach a [call queue](https://support.zoom.us/hc/en-us/articles/360021524831-Managing-Call-Queues).<br><br>
     * **Prerequisites:**<br>
     * * Pro or higher account plan.
     * * Account owner or admin permissions
     * * Zoom Phone license<br>
     * **Scopes:** `phone:write:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     * **Parameters:**
     *
     * * `call_queue_id: &str` -- Unique Identifier of the Call Queue.
     */
    pub async fn assign(
        &self,
        call_queue_id: &str,
        body: &crate::types::AddByocNumberResponse,
    ) -> Result<crate::types::Domains> {
        let url = format!(
            "/phone/call_queues/{}/phone_numbers",
            crate::progenitor_support::encode_path(&call_queue_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Unassign all phone numbers.
     *
     * This function performs a `DELETE` to the `/phone/call_queues/{callQueueId}/phone_numbers` endpoint.
     *
     * Use this API to unbind all phone numbers that are assigned to a [Call Queue](https://support.zoom.us/hc/en-us/articles/360021524831-Managing-Call-Queues) After successful unbinding, the numbers will appear in the [Unassigned tab](https://zoom.us/signin#/numbers/unassigned).<br> If you only need to unassign a specific phone number, use the Unassign a Phone Number API instead. <br>
     * **Prerequisites:**
     * * Pro or higher account palan
     * * Account owner or admin permissions
     * * Zoom Phone license <br> **Scopes:** `phone:write:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     * **Parameters:**
     *
     * * `call_queue_id: &str` -- Unique Identifier of the Call Queue. This can be retrieved from List Call Queues API.
     */
    pub async fn unassign_phone_num_call_queue(
        &self,
        call_queue_id: &str,
    ) -> Result<crate::types::Domains> {
        let url = format!(
            "/phone/call_queues/{}/phone_numbers",
            crate::progenitor_support::encode_path(&call_queue_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Unassign a phone number.
     *
     * This function performs a `DELETE` to the `/phone/call_queues/{callQueueId}/phone_numbers/{phoneNumberId}` endpoint.
     *
     * After assigning a phone number, you can unbind it if you don't want it to be assigned to a [Call Queue](https://support.zoom.us/hc/en-us/articles/360021524831-Managing-Call-Queues). Use this API to unbind a phone number from a Call Queue. After successful unbinding, the number will appear in the [Unassigned tab](https://zoom.us/signin#/numbers/unassigned).<br><br>
     * **Prerequisites:**
     * * Pro or higher account palan
     * * Account owner or admin permissions
     * * Zoom Phone license <br> **Scopes:** `phone:write:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     * **Parameters:**
     *
     * * `call_queue_id: &str` -- Unique Identifier of the Call Queue. This can be retrieved from the List Call Queues API.
     * * `phone_number_id: &str` -- Unique Identifier of the Phone Number. .
     */
    pub async fn un_assign_phone_num_call_queue(
        &self,
        call_queue_id: &str,
        phone_number_id: &str,
    ) -> Result<crate::types::Domains> {
        let url = format!(
            "/phone/call_queues/{}/phone_numbers/{}",
            crate::progenitor_support::encode_path(&call_queue_id.to_string()),
            crate::progenitor_support::encode_path(&phone_number_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Add members to a call queue.
     *
     * This function performs a `POST` to the `/phone/call_queues/{callQueueId}/members` endpoint.
     *
     * Add phone users and/or [common area phones](https://support.zoom.us/hc/en-us/articles/360028516231-Managing-Common-Area-Phones) as members to a specific Call Queue.<br><br>
     * **Prerequisites:**<br>
     * * Pro or higher account plan.
     * * Zoom Phone license<br>
     * **Scopes:** `phone:write:admin`<br>
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `call_queue_id: &str` -- Unique Identifier of the Call Queue.
     */
    pub async fn add_members_call_queue(
        &self,
        call_queue_id: &str,
        body: &crate::types::AddMembersCallQueueRequestData,
    ) -> Result<crate::types::Domains> {
        let url = format!(
            "/phone/call_queues/{}/members",
            crate::progenitor_support::encode_path(&call_queue_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Unassign all members.
     *
     * This function performs a `DELETE` to the `/phone/call_queues/{callQueueId}/members` endpoint.
     *
     * Use this API to remove all members from a Call Queue who were previously assigned to that Call Queue. The members could be phone users or [common area phones](https://support.zoom.us/hc/en-us/articles/360028516231-Managing-Common-Area-Phones).
     * **Prerequisites:**<br>
     * * Pro or higher account plan.
     * * Zoom Phone license<br>
     * **Scopes:** `phone:write:admin`<br>
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `call_queue_id: &str` -- User's first name.
     */
    pub async fn unassign_all_members(&self, call_queue_id: &str) -> Result<()> {
        let url = format!(
            "/phone/call_queues/{}/members",
            crate::progenitor_support::encode_path(&call_queue_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Unassign a member.
     *
     * This function performs a `DELETE` to the `/phone/call_queues/{callQueueId}/members/{memberId}` endpoint.
     *
     * Use this API to remove a member from a Call Queue who was previously added to that Call Queue. The member could be a phone user or a [common area phone](https://support.zoom.us/hc/en-us/articles/360028516231-Managing-Common-Area-Phones). A member who is a Call Queue Manager cannot be unassigned from the Call Queue using this API.
     * **Prerequisites:**<br>
     * * Pro or higher account plan.
     * * Zoom Phone license<br>
     * **Scopes:** `phone:write:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     * **Parameters:**
     *
     * * `call_queue_id: &str` -- Unique Identifier of the Call Queue from which the member needs to be unassigned.
     * * `member_id: &str` -- Unique Identifier of the member who needs to be unassigned.
     */
    pub async fn unassign_member_from_call_queue(
        &self,
        call_queue_id: &str,
        member_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/phone/call_queues/{}/members/{}",
            crate::progenitor_support::encode_path(&call_queue_id.to_string()),
            crate::progenitor_support::encode_path(&member_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Change call queue manager.
     *
     * This function performs a `PUT` to the `/phone/call_queues/{callQueueId}/manager` endpoint.
     *
     * A call queue manager has the privileges to maanage the call queue's voicemail inbox and recordings, change all call queue settings and call queue policy settings.<br><br> Use this API to to set another phone user as the [call queue manager](https://support.zoom.us/hc/en-us/articles/360021524831-Managing-Call-Queues#h_db06854b-e6a3-4afe-ba15-baf58f31f90c).
     * **Prerequisites:**<br>
     * * Pro or higher account plan.
     * * Account owner or admin permissions
     * * Zoom Phone license<br>
     * **Scopes:** `phone:write:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     * **Parameters:**
     *
     * * `call_queue_id: &str` -- Unique Identifier of the Call Queue.
     */
    pub async fn change_call_queue_manager(
        &self,
        call_queue_id: &str,
        body: &crate::types::ChangeCallQueueManagerRequest,
    ) -> Result<()> {
        let url = format!(
            "/phone/call_queues/{}/manager",
            crate::progenitor_support::encode_path(&call_queue_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get call queue recordings.
     *
     * This function performs a `GET` to the `/phone/call_queues/{callQueueId}/recordings` endpoint.
     *
     * Use this API to view [call recordings](https://support.zoom.us/hc/en-us/articles/360038521091#h_cbc9f2a3-e06c-4daa-83d4-ddbceef9c77b) from the call queue.<br><br>
     * **Prerequisites:**<br>
     * * Pro or higher account with Zoom Phone license.
     * * [Automatic call recordings](https://support.zoom.us/hc/en-us/articles/360033511872#h_fcb297bb-14e8-4094-91ca-dc61e1a18734) must be enabled in the Policy Settings for call queues. <br> **Scope:** `phone:read:admin`<br> **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     *
     *
     *
     *
     * **Parameters:**
     *
     * * `call_queue_id: &str` -- Unique Identifier of the Call Queue.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `from: chrono::NaiveDate` -- Start date (within a 6 month range).
     * * `to: chrono::NaiveDate` -- End date (within a 6 month range).
     */
    pub async fn get_call_queue_recordings(
        &self,
        call_queue_id: &str,
        page_size: i64,
        next_page_token: &str,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    ) -> Result<Vec<crate::types::GetCallQueueRecordingsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !to.to_string().is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/phone/call_queues/{}/recordings?{}",
            crate::progenitor_support::encode_path(&call_queue_id.to_string()),
            query_
        );

        let resp: crate::types::GetCallQueueRecordingsResponseData =
            self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.recordings)
    }

    /**
     * Get call queue recordings.
     *
     * This function performs a `GET` to the `/phone/call_queues/{callQueueId}/recordings` endpoint.
     *
     * As opposed to `get_call_queue_recordings`, this function returns all the pages of the request at once.
     *
     * Use this API to view [call recordings](https://support.zoom.us/hc/en-us/articles/360038521091#h_cbc9f2a3-e06c-4daa-83d4-ddbceef9c77b) from the call queue.<br><br>
     * **Prerequisites:**<br>
     * * Pro or higher account with Zoom Phone license.
     * * [Automatic call recordings](https://support.zoom.us/hc/en-us/articles/360033511872#h_fcb297bb-14e8-4094-91ca-dc61e1a18734) must be enabled in the Policy Settings for call queues. <br> **Scope:** `phone:read:admin`<br> **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     *
     *
     *
     */
    pub async fn get_all_call_queue_recordings(
        &self,
        call_queue_id: &str,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    ) -> Result<Vec<crate::types::GetCallQueueRecordingsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !to.to_string().is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/phone/call_queues/{}/recordings?{}",
            crate::progenitor_support::encode_path(&call_queue_id.to_string()),
            query_
        );

        let mut resp: crate::types::GetCallQueueRecordingsResponseData =
            self.client.get(&url, None).await?;

        let mut recordings = resp.recordings;
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

            recordings.append(&mut resp.recordings);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(recordings)
    }
}
