use anyhow::Result;

use crate::Client;

pub struct Meetings {
    pub client: Client,
}

impl Meetings {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Meetings { client }
    }

    /**
     * List meetings.
     *
     * This function performs a `GET` to the `/users/{userId}/meetings` endpoint.
     *
     * List all the meetings that were scheduled for a user (meeting host). For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * This API **only** supports scheduled meetings. This API does not return information about instant meetings.
     *
     * **Scopes:** `meeting:read:admin`, `meeting:read`</br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     * * `type_: crate::types::MeetingsType` -- The meeting types: <br>`scheduled` - This includes all valid past meetings (unexpired), live meetings and upcoming scheduled meetings. It is equivalent to the combined list of "Previous Meetings" and "Upcoming Meetings" displayed in the user's [Meetings page](https://zoom.us/meeting) on the Zoom Web Portal.<br>`live` - All the ongoing meetings.<br>`upcoming` - All upcoming meetings including live meetings.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `page_number: &str` -- The page number of the current page in the returned records.
     */
    pub async fn get(
        &self,
        user_id: &str,
        type_: crate::types::MeetingsType,
        page_size: i64,
        next_page_token: &str,
        page_number: &str,
    ) -> Result<crate::types::Domains> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if !page_number.is_empty() {
            query_args.push(("page_number".to_string(), page_number.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/users/{}/meetings?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Create a meeting.
     *
     * This function performs a `POST` to the `/users/{userId}/meetings` endpoint.
     *
     * [Create a meeting](https://support.zoom.us/hc/en-us/articles/201362413-Scheduling-meetings) for a user. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * This API has a daily rate limit of 100 requests per day. Therefore, only 100 **Create a Meeting** API requests are permitted within a 24 hour window for a user.<br>
     *
     * <aside>The <code>start_url</code> of a meeting is a URL using which a host or an alternative host can start a meeting. The expiration time for the <code>start_url</code> field is two hours for all regular users.
     *
     * For custCreate meeting hosts( i.e., users created using the <code>custCreate</code> option via the [Create Users](https://marketplace.zoom.us/docs/api-reference/zoom-api/users/usercreate) API), the expiration time of the <code>start_url</code> field is 90 days from the generation of the <code>start_url</code>.
     *
     * For security reasons, the recommended way to retrieve the updated value for the <code>start_url</code> field programmatically (after expiry) is by calling the [Retrieve a Meeting API](https://marketplace.zoom.us/docs/api-reference/zoom-api/meetings/meeting) and referring to the value of the <code>start_url</code> field in the response.</aside><br><br>
     * Scopes: `meeting:write:admin` `meeting:write`
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     */
    pub async fn create(
        &self,
        user_id: &str,
        body: &crate::types::MeetingCreate,
    ) -> Result<crate::types::MeetingCreateResponseAllOf> {
        let url = format!(
            "/users/{}/meetings",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get a meeting.
     *
     * This function performs a `GET` to the `/meetings/{meetingId}` endpoint.
     *
     * Retrieve the details of a meeting.<br><br>
     * **Scopes:** `meeting:read:admin` `meeting:read`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     * * `occurrence_id: &str` -- Meeting Occurrence ID. Provide this field to view meeting details of a particular occurrence of the [recurring meeting](https://support.zoom.us/hc/en-us/articles/214973206-Scheduling-Recurring-Meetings).
     * * `show_previous_occurrences: bool` -- Enable/disable the option for a sub account to use shared [Virtual Room Connector(s)](https://support.zoom.us/hc/en-us/articles/202134758-Getting-Started-With-Virtual-Room-Connector) that are set up by the master account. Virtual Room Connectors can only be used by On-prem users.
     */
    pub async fn meeting(
        &self,
        meeting_id: i64,
        occurrence_id: &str,
        show_previous_occurrences: bool,
    ) -> Result<crate::types::MeetingResponseAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !occurrence_id.is_empty() {
            query_args.push(("occurrence_id".to_string(), occurrence_id.to_string()));
        }
        if show_previous_occurrences {
            query_args.push((
                "show_previous_occurrences".to_string(),
                show_previous_occurrences.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/meetings/{}?{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a meeting.
     *
     * This function performs a `DELETE` to the `/meetings/{meetingId}` endpoint.
     *
     * Delete a meeting.<br><br>
     * **Scopes:** `meeting:write:admin` `meeting:write`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     * * `occurrence_id: &str` -- The meeting occurrence ID.
     * * `schedule_for_reminder: bool` -- Enable/disable the option for a sub account to use shared [Virtual Room Connector(s)](https://support.zoom.us/hc/en-us/articles/202134758-Getting-Started-With-Virtual-Room-Connector) that are set up by the master account. Virtual Room Connectors can only be used by On-prem users.
     * * `cancel_meeting_reminder: &str` -- `true`: Notify registrants about the meeting cancellation via email.
     *   
     *   `false`: Do not send any email notification to meeting registrants.
     *   
     *   The default value of this field is `false`.
     */
    pub async fn delete(
        &self,
        meeting_id: i64,
        occurrence_id: &str,
        schedule_for_reminder: bool,
        cancel_meeting_reminder: &str,
    ) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cancel_meeting_reminder.is_empty() {
            query_args.push((
                "cancel_meeting_reminder".to_string(),
                cancel_meeting_reminder.to_string(),
            ));
        }
        if !occurrence_id.is_empty() {
            query_args.push(("occurrence_id".to_string(), occurrence_id.to_string()));
        }
        if schedule_for_reminder {
            query_args.push((
                "schedule_for_reminder".to_string(),
                schedule_for_reminder.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/meetings/{}?{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            query_
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update a meeting.
     *
     * This function performs a `PATCH` to the `/meetings/{meetingId}` endpoint.
     *
     * Update the details of a meeting. This API has a rate limit of 100 requests per day. Because of this, a meeting can only be updated for a maximum of 100 times within a 24 hour period.
     *
     * **Scopes:** `meeting:write:admin`, `meeting:write`</br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     * * `occurrence_id: &str` -- Meeting occurrence id. Support change of agenda, start_time, duration, settings: {host_video, participant_video, join_before_host, mute_upon_entry, waiting_room, watermark, auto_recording}.
     */
    pub async fn update(
        &self,
        meeting_id: i64,
        occurrence_id: &str,
        body: &crate::types::MeetingUpdateRequestAllOf,
    ) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !occurrence_id.is_empty() {
            query_args.push(("occurrence_id".to_string(), occurrence_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/meetings/{}?{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            query_
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Update meeting status.
     *
     * This function performs a `PUT` to the `/meetings/{meetingId}/status` endpoint.
     *
     * Update the status of a meeting.<br><br>
     * **Scopes:** `meeting:write:admin` `meeting:write`
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     */
    pub async fn status(
        &self,
        meeting_id: i64,
        body: &crate::types::MeetingStatusRequest,
    ) -> Result<()> {
        let url = format!(
            "/meetings/{}/status",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * List meeting registrants.
     *
     * This function performs a `GET` to the `/meetings/{meetingId}/registrants` endpoint.
     *
     * A host or a user with admin permission can require [registration for a Zoom meeting](https://support.zoom.us/hc/en-us/articles/211579443-Registration-for-Meetings). Use this API to list users that have registered for a meeting.<br><br>
     * **Scopes**: `meeting:read:admin` `meeting:read`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     * * `occurrence_id: &str` -- The meeting occurrence ID.
     * * `status: crate::types::MeetingRegistrantsStatus` -- The registrant status:<br>`pending` - Registrant's status is pending.<br>`approved` - Registrant's status is approved.<br>`denied` - Registrant's status is denied.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `page_number: i64` --
     *   **Deprecated** - This field has been deprecated and we will stop supporting it completely in a future release. Please use "next_page_token" for pagination instead of this field.
     *   
     *   The page number of the current page in the returned records.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn registrant(
        &self,
        meeting_id: i64,
        occurrence_id: &str,
        status: crate::types::MeetingRegistrantsStatus,
        page_size: i64,
        page_number: i64,
        next_page_token: &str,
    ) -> Result<crate::types::Domains> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if !occurrence_id.is_empty() {
            query_args.push(("occurrence_id".to_string(), occurrence_id.to_string()));
        }
        if page_number > 0 {
            query_args.push(("page_number".to_string(), page_number.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/meetings/{}/registrants?{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Add meeting registrant.
     *
     * This function performs a `POST` to the `/meetings/{meetingId}/registrants` endpoint.
     *
     * Register a participant for a meeting.<br><br> Note that there is a maximum limit of 4999 registrants per meeting and users will see an error if the capacity has reached.
     *
     * **Prerequisite:**<br>
     * * Host user type must be "Licensed".
     *
     * **Scopes:** `meeting:write:admin` `meeting:write`
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     * * `occurrence_ids: &str` -- Occurrence IDs. You can find these with the meeting get API. Multiple values separated by comma.
     */
    pub async fn registrant_create(
        &self,
        meeting_id: i64,
        occurrence_ids: &str,
    ) -> Result<crate::types::MeetingRegistrantCreateResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !occurrence_ids.is_empty() {
            query_args.push(("occurrence_ids".to_string(), occurrence_ids.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/meetings/{}/registrants?{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            query_
        );

        self.client.post(&url, None).await
    }

    /**
     * Delete a meeting registrant.
     *
     * This function performs a `DELETE` to the `/meetings/{meetingId}/registrants/{registrantId}` endpoint.
     *
     * Delete a meeting registrant.<br><br>
     * **Scopes**: `meeting:write:admin` `meeting:write`<br>
     *  <br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `occurrence_id: &str` -- The meeting occurence ID.
     * * `meeting_id: i64` -- Account seats.
     * * `registrant_id: &str` -- The meeting registrant ID.
     */
    pub async fn meetingregistrantdelete(
        &self,
        occurrence_id: &str,
        meeting_id: i64,
        registrant_id: &str,
    ) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !occurrence_id.is_empty() {
            query_args.push(("occurrence_id".to_string(), occurrence_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/meetings/{}/registrants/{}?{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            crate::progenitor_support::encode_path(&registrant_id.to_string()),
            query_
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update registrant's status.
     *
     * This function performs a `PUT` to the `/meetings/{meetingId}/registrants/status` endpoint.
     *
     * Update a meeting registrant's status by either approving, cancelling or denying a registrant from joining the meeting.<br><br>
     * **Scopes:** `meeting:write:admin` `meeting:write`
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     * * `occurrence_id: &str` -- The meeting occurrence ID.
     */
    pub async fn registrant_status(
        &self,
        meeting_id: i64,
        occurrence_id: &str,
        body: &crate::types::RegistrantStatus,
    ) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !occurrence_id.is_empty() {
            query_args.push(("occurrence_id".to_string(), occurrence_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/meetings/{}/registrants/status?{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            query_
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get past meeting details.
     *
     * This function performs a `GET` to the `/past_meetings/{meetingUUID}` endpoint.
     *
     * Get details on a past meeting. <br><br>
     * **Scopes:** `meeting:read:admin` `meeting:read`
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     * > **Note**: Please double encode your UUID when using this API if the UUID begins with a '/'or contains '//' in it.
     *
     * **Parameters:**
     *
     * * `meeting: &str` -- The meeting's universally unique identifier (UUID). Each meeting instance generates a UUID. For example, after a meeting ends, a new UUID is generated for the next meeting instance.
     *   
     *   If the meeting UUID begins with a `/` character or contains a `//` character, you **must** double-encode the meeting UUID when using the meeting UUID for other API calls.
     */
    pub async fn past_details(
        &self,
        meeting_uuid: &str,
    ) -> Result<crate::types::PastMeetingDetailsResponse> {
        let url = format!(
            "/past_meetings/{}",
            crate::progenitor_support::encode_path(&meeting_uuid.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Get past meeting participants.
     *
     * This function performs a `GET` to the `/past_meetings/{meetingUUID}/participants` endpoint.
     *
     * Retrieve information on participants from a past meeting. <br><br>
     * **Scopes:** `meeting:read:admin` `meeting:read`
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     * **Prerequisites:**<br>
     * * Paid account on a Pro or higher plan.
     *
     * <br> <br>  **Note**: Please double encode your UUID when using this API if the UUID begins with a '/'or contains '//' in it.
     *
     *
     * **Parameters:**
     *
     * * `meeting: &str` -- The meeting's universally unique identifier (UUID). Each meeting instance generates a UUID. For example, after a meeting ends, a new UUID is generated for the next meeting instance.
     *   
     *   If the meeting UUID begins with a `/` character or contains a `//` character, you **must** double-encode the meeting UUID when using the meeting UUID for other API calls.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn past_participant(
        &self,
        meeting_uuid: &str,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<crate::types::PastMeetingParticipantsResponseAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/past_meetings/{}/participants?{}",
            crate::progenitor_support::encode_path(&meeting_uuid.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * List ended meeting instances.
     *
     * This function performs a `GET` to the `/past_meetings/{meetingId}/instances` endpoint.
     *
     * Get a list of ended meeting instances<br><br>
     * **Scopes:** `meeting:read:admin` `meeting:read`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     */
    pub async fn past(&self, meeting_id: i64) -> Result<crate::types::Domains> {
        let url = format!(
            "/past_meetings/{}/instances",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * List meeting polls.
     *
     * This function performs a `GET` to the `/meetings/{meetingId}/polls` endpoint.
     *
     * Polls allow the meeting host to survey attendees. Use this API to list [polls](https://support.zoom.us/hc/en-us/articles/213756303-Polling-for-Meetings) of a meeting.<br><br>
     *
     * **Scopes**: `meeting:read:admin` `meeting:read`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     * **Prerequisites**:<br>
     * * Host user type must be **Pro** or higher plan.
     * * Meeting must be a scheduled meeting. Instant meetings do not have polling features enabled.
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     */
    pub async fn poll(&self, meeting_id: i64) -> Result<crate::types::Domains> {
        let url = format!(
            "/meetings/{}/polls",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Create a meeting poll.
     *
     * This function performs a `POST` to the `/meetings/{meetingId}/polls` endpoint.
     *
     * Polls allow the meeting host to survey attendees. Use this API to create a [poll](https://support.zoom.us/hc/en-us/articles/213756303-Polling-for-Meetings) for a meeting.<br><br>
     *
     * **Scopes**: `meeting:write:admin` `meeting:write`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     * **Prerequisites**:<br>
     * * Host user type must be **Pro** or higher plan.
     * * Polling feature must be enabled in the host's account.
     * * Meeting must be a scheduled meeting. Instant meetings do not have polling features enabled.
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     */
    pub async fn poll_create(
        &self,
        meeting_id: i64,
        body: &crate::types::Poll,
    ) -> Result<crate::types::MeetingPollGetResponseAllOf> {
        let url = format!(
            "/meetings/{}/polls",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get a meeting poll.
     *
     * This function performs a `GET` to the `/meetings/{meetingId}/polls/{pollId}` endpoint.
     *
     * Polls allow the meeting host to survey attendees. Use this API to get information about a specific meeting [poll](https://support.zoom.us/hc/en-us/articles/213756303-Polling-for-Meetings).<br><br>
     * **Scopes**: `meeting:read:admin` `meeting:read`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     * * `poll_id: &str` -- User's first name.
     */
    pub async fn poll_get(
        &self,
        meeting_id: i64,
        poll_id: &str,
    ) -> Result<crate::types::MeetingPollGetResponseAllOf> {
        let url = format!(
            "/meetings/{}/polls/{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            crate::progenitor_support::encode_path(&poll_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update a meeting poll.
     *
     * This function performs a `PUT` to the `/meetings/{meetingId}/polls/{pollId}` endpoint.
     *
     * Polls allow the meeting host to survey attendees. Use this API to update information of a specific meeting [poll](https://support.zoom.us/hc/en-us/articles/213756303-Polling-for-Meetings)<br><br>
     * **Scopes**: `meeting:write:admin` `meeting:write`
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     * * `poll_id: &str` -- User's first name.
     */
    pub async fn poll_update(
        &self,
        meeting_id: i64,
        poll_id: &str,
        body: &crate::types::Poll,
    ) -> Result<()> {
        let url = format!(
            "/meetings/{}/polls/{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            crate::progenitor_support::encode_path(&poll_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete a meeting poll.
     *
     * This function performs a `DELETE` to the `/meetings/{meetingId}/polls/{pollId}` endpoint.
     *
     * Polls allow the meeting host to survey attendees. Use this API to delete a meeting [poll](https://support.zoom.us/hc/en-us/articles/213756303-Polling-for-Meetings).<br>
     * **Scopes**: `meeting:write:admin` `meeting:write`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light` <br>
     * **Prerequisites**:<br>
     * * Host user type must be **Pro**.
     * * Polling feature should be enabled in the host's account.
     * * Meeting must be a scheduled meeting. Instant meetings do not have polling features enabled.
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     * * `poll_id: &str` -- User's first name.
     */
    pub async fn poll_delete(&self, meeting_id: i64, poll_id: &str) -> Result<()> {
        let url = format!(
            "/meetings/{}/polls/{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            crate::progenitor_support::encode_path(&poll_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * List registration questions .
     *
     * This function performs a `GET` to the `/meetings/{meetingId}/registrants/questions` endpoint.
     *
     * List registration questions that will be displayed to users while [registering for a meeting](https://support.zoom.us/hc/en-us/articles/211579443-Registration-for-Meetings).<br>
     *
     * **Scopes:** `meeting:read`, `meeting:read:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     */
    pub async fn registrants_questions_get(
        &self,
        meeting_id: i64,
    ) -> Result<crate::types::MeetingRegistrantQuestionsData> {
        let url = format!(
            "/meetings/{}/registrants/questions",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update registration questions.
     *
     * This function performs a `PATCH` to the `/meetings/{meetingId}/registrants/questions` endpoint.
     *
     * Update registration questions that will be displayed to users while [registering for a meeting](https://support.zoom.us/hc/en-us/articles/211579443-Registration-for-Meetings).<br><br>
     * **Scopes:** `meeting:write`, `meeting:write:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *  
     *
     *
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     */
    pub async fn registrant_question_update(
        &self,
        meeting_id: i64,
        body: &crate::types::MeetingRegistrantQuestionsData,
    ) -> Result<()> {
        let url = format!(
            "/meetings/{}/registrants/questions",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get meeting invitation.
     *
     * This function performs a `GET` to the `/meetings/{meetingId}/invitation` endpoint.
     *
     * Retrieve the meeting invite note that was sent for a specific meeting.<br><br>
     * **Scopes:** `meeting:read:admin` `meeting:read`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     */
    pub async fn invitation(&self, meeting_id: i64) -> Result<crate::types::MeetingInvitation> {
        let url = format!(
            "/meetings/{}/invitation",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Get live stream details.
     *
     * This function performs a `GET` to the `/meetings/{meetingId}/livestream` endpoint.
     *
     * Zoom allows users to [live stream a meeting](https://support.zoom.us/hc/en-us/articles/115001777826-Live-Streaming-Meetings-or-Webinars-Using-a-Custom-Service) to a custom platform. Use this API to get a meeting's live stream configuration details such as Stream URL, Stream Key and Page URL.<br><br>
     * **Prerequisites:**<br>
     * * Meeting host must be a licensed user with a Pro or higher plan.<br>
     * * Live streaming details must have been [configured](https://support.zoom.us/hc/en-us/articles/115001777826-Live-Streaming-Meetings-or-Webinars-Using-a-Custom-Service#h_01589a6f-a40a-4e18-a448-cb746e52ebc5) for the meeting.<br><br>
     * **Scopes:** `meeting:read:admin` `meeting:read`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     *
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- Unique identifier of the meeting.
     */
    pub async fn get_live_stream_details(
        &self,
        meeting_id: &str,
    ) -> Result<crate::types::GetLiveStreamDetailsResponse> {
        let url = format!(
            "/meetings/{}/livestream",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update a live stream.
     *
     * This function performs a `PATCH` to the `/meetings/{meetingId}/livestream` endpoint.
     *
     * Use this API to update a meeting's live stream information. Zoom allows users to [live stream a meeting](https://support.zoom.us/hc/en-us/articles/115001777826-Live-Streaming-Meetings-or-Webinars-Using-a-Custom-Service) to a custom platform.
     *
     * **Scopes:** `meeting:write:admin`, `meeting:write`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Meeting host must have a Pro license.
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     */
    pub async fn live_stream_update(
        &self,
        meeting_id: &str,
        body: &crate::types::MeetingLiveStream,
    ) -> Result<()> {
        let url = format!(
            "/meetings/{}/livestream",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Update Live Stream Status.
     *
     * This function performs a `PATCH` to the `/meetings/{meetingId}/livestream/status` endpoint.
     *
     * Zoom allows users to [live stream a meeting](https://support.zoom.us/hc/en-us/articles/115001777826-Live-Streaming-Meetings-or-Webinars-Using-a-Custom-Service) to a custom platform. Use this API to update the status of a meeting's live stream.<br><br>
     * **Prerequisites:**<br>
     * * Meeting host must have a Pro license.<br>
     * **Scopes:** `meeting:write:admin` `meeting:write`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     */
    pub async fn live_stream_status_update(
        &self,
        meeting_id: i64,
        body: &crate::types::MeetingLiveStreamStatus,
    ) -> Result<()> {
        let url = format!(
            "/meetings/{}/livestream/status",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * List past meeting's poll results.
     *
     * This function performs a `GET` to the `/past_meetings/{meetingId}/polls` endpoint.
     *
     * [Polls](https://support.zoom.us/hc/en-us/articles/213756303-Polling-for-Meetings) allow the meeting host to survey attendees. Use this API to list poll results of a meeting.<br><br>
     *
     * **Scopes**: `meeting:read:admin`, `meeting:read`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium` <br>
     * **Prerequisites**:<br>
     * * Host user type must be **Pro**.
     * * Meeting must be a scheduled meeting. Instant meetings do not have polling features enabled.
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- The meeting ID or the meeting UUID.  If a meeting ID is provided in the request instead of a UUID, the response will be for the latest meeting instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     */
    pub async fn list_past_polls(
        &self,
        meeting_id: &str,
    ) -> Result<crate::types::ListPastMeetingPollsResponse> {
        let url = format!(
            "/past_meetings/{}/polls",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Perform batch registration.
     *
     * This function performs a `POST` to the `/meetings/{meetingId}/batch_registrants` endpoint.
     *
     * Register up to 30 registrants at once for a meeting that requires [registration](https://support.zoom.us/hc/en-us/articles/211579443-Registration-for-Meetings). <br>
     *
     * **Prerequisites:**<br>
     * * The meeting host must be a Licensed user.
     * * The meeting must require registration and should be of type `2`, i.e., they should be scheduled meetings. Instant meetings and Recurring meetings are not supported by this API.<br><br>
     * **Scope:** `meeting:write`, `meeting:write:admin`<br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     *
     *
     *
     *
     *
     *
     *
     *
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- Unique identifier of the meeting (Meeting Number).
     */
    pub async fn add_batch_registrants(
        &self,
        meeting_id: &str,
        body: &crate::types::AddBatchRegistrantsRequest,
    ) -> Result<crate::types::AddBatchRegistrantsResponse> {
        let url = format!(
            "/meetings/{}/batch_registrants",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Use in-Meeting recording controls.
     *
     * This function performs a `PATCH` to the `/live_meetings/{meetingId}/events` endpoint.
     *
     * Use this API to control the [in-meeting](https://support.zoom.us/hc/en-us/articles/360021921032-In-Meeting-Controls) **recording features** such as starting a recording, stopping a recording, pausing a recording, and resuming a recording. This API only works for Cloud Recordings and not for local recordings.
     *
     *
     * **Prerequisite:**
     * * The meeting must be a live meeting.
     * * Cloud Recording must be enabled.
     * * The user using this API must either be the host or alternative host of the meeting.
     *
     * **Scopes:** `meeting:write`, `meeting:write:admin`, `meeting:master`
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- Unique identifier of the live meeting.
     */
    pub async fn recording_control(
        &self,
        meeting_id: &str,
        body: &crate::types::InMeetingRecordingControlRequest,
    ) -> Result<()> {
        let url = format!(
            "/live_meetings/{}/events",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get meeting quality score.
     *
     * This function performs a `GET` to the `/metrics/quality` endpoint.
     *
     * Get the quality scores of a meeting.
     */
    pub async fn quality_score(&self) -> Result<crate::types::Domains> {
        let url = "/metrics/quality".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Perform batch poll creation.
     *
     * This function performs a `POST` to the `/meetings/{meetingId}/batch_polls` endpoint.
     *
     * Polls allow the meeting host to survey attendees. Use this API to create batch [polls](https://support.zoom.us/hc/en-us/articles/213756303-Polling-for-Meetings) for a meeting.<br><br>
     *
     * **Scopes**: `meeting:write:admin` `meeting:write`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     * **Prerequisites**:<br>
     * * Host user type must be **Pro** or higher plan.
     * * Polling feature must be enabled in the host's account.
     * * Meeting must be a scheduled meeting. Instant meetings do not have polling features enabled.
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- User's first name.
     */
    pub async fn create_batch_polls(
        &self,
        meeting_id: &str,
        body: &crate::types::CreateBatchPollsRequest,
    ) -> Result<crate::types::CreateBatchPollsResponse> {
        let url = format!(
            "/meetings/{}/batch_polls",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * List meeting templates.
     *
     * This function performs a `GET` to the `/users/{userId}/meeting_templates` endpoint.
     *
     * Use this API to list [meeting templates](https://support.zoom.us/hc/en-us/articles/360036559151-Meeting-templates) that are available to be used by a user. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `meeting:read` or `meeting:read:admin`</br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- Unique identifier of the user. Retrieve the value of this field by calling the [List users](https://marketplace.zoom.us/docs/api-reference/zoom-api/users/users) API.
     */
    pub async fn list_template(
        &self,
        user_id: &str,
    ) -> Result<crate::types::ListMeetingTemplatesResponseData> {
        let url = format!(
            "/users/{}/meeting_templates",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Create meeting's invite links.
     *
     * This function performs a `POST` to the `/meetings/{meetingId}/invite_links` endpoint.
     *
     * Use this API to create a batch of invitation links for a meeting.
     *
     * **Scopes**: `meeting:write:admin`, `meeting:write`</br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     */
    pub async fn invite_links_create(
        &self,
        meeting_id: i64,
        body: &crate::types::InviteLink,
    ) -> Result<crate::types::InviteLinks> {
        let url = format!(
            "/meetings/{}/invite_links",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
