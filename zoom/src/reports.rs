use anyhow::Result;

use crate::Client;

pub struct Reports {
    pub client: Client,
}

impl Reports {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Reports { client }
    }

    /**
     * Get daily usage report.
     *
     * This function performs a `GET` to the `/report/daily` endpoint.
     *
     * Retrieve daily report to access the account-wide usage of Zoom services for each day in a given month. It lists the number of new users, meetings, participants, and meeting minutes.<br>
     * **Prerequisites**<br>
     * * Pro or higher plan.<br>
     * **Scopes:** `report:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     *
     * **Parameters:**
     *
     * * `year: i64` -- Year for this report.
     * * `month: i64` -- Month for this report.
     */
    pub async fn daily(&self, year: i64, month: i64) -> Result<crate::types::ReportDailyResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if month > 0 {
            query_args.push(("month".to_string(), month.to_string()));
        }
        if year > 0 {
            query_args.push(("year".to_string(), year.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/report/daily?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Get active/inactive host reports.
     *
     * This function performs a `GET` to the `/report/users` endpoint.
     *
     * A user is considered to be an active host during the month specified in the "from" and "to" range, if the user has hosted at least one meeting during this period. If the user didn't host any meetings during this period, the user is considered to be inactive.<br>The Active Hosts report displays a list of meetings, participants, and meeting minutes for a specific time range, up to one month. The month should fall within the last six months.<br>The Inactive Hosts report pulls a list of users who were not active during a specific period of time.
     * Use this API to retrieve an active or inactive host report for a specified period of time. The time range for the report is limited to a month and the month should fall under the past six months. <br>You can specify the type of report and date range using the query parameters.<br>
     * **Scopes:** `report:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * **Prerequisites:**<br>
     * * Pro or higher plan.
     *
     * **Parameters:**
     *
     * * `type_: crate::types::ReportUsersType` -- Active or inactive hosts.<br>`active` - Active hosts. <br>`inactive` - Inactive hosts.
     * * `from: chrono::NaiveDate` -- Start date in 'yyyy-mm-dd' format. The date range defined by the "from" and "to" parameters should only be one month as the report includes only one month worth of data at once.
     * * `to: chrono::NaiveDate` -- Start Date.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `page_number: i64` -- The page number of the current page in the returned records.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn user(
        &self,
        type_: crate::types::ReportUsersType,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
        page_size: i64,
        page_number: i64,
        next_page_token: &str,
    ) -> Result<crate::types::Domains> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_number > 0 {
            query_args.push(("page_number".to_string(), page_number.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !to.to_string().is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/report/users?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Get meeting reports.
     *
     * This function performs a `GET` to the `/report/users/{userId}/meetings` endpoint.
     *
     * Retrieve [report](https://support.zoom.us/hc/en-us/articles/216378603-Meeting-Reporting) on past meetings and webinars for a specified time period. The time range for the report is limited to a month and the month must fall within the past six months.
     *
     * Meetings and webinars are returned only if they have two or more unique participants.  <br><br>
     * **Scopes:** `report:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * **Prerequisites:**<br>
     * * Pro or higher plan.
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     * * `from: chrono::NaiveDate` -- Start date in 'yyyy-mm-dd' format. The date range defined by the "from" and "to" parameters should only be one month as the report includes only one month worth of data at once.
     * * `to: chrono::NaiveDate` -- Start Date.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `type_: crate::types::ReportMeetingsType` -- The meeting types: <br>`past` - Past meetings.<br>`pastOne` - Past one user meetings.
     */
    pub async fn meeting(
        &self,
        user_id: &str,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
        page_size: i64,
        next_page_token: &str,
        type_: crate::types::ReportMeetingsType,
    ) -> Result<crate::types::ReportMeetingsResponseAllOf> {
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
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/report/users/{}/meetings?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get meeting detail reports.
     *
     * This function performs a `GET` to the `/report/meetings/{meetingId}` endpoint.
     *
     * Get a detailed report for a past meeting. <br>
     * **Scopes:** `report:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * **Prerequisites:**<br>
     * * Pro or a higher plan.<br>
     *  
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- The meeting ID or the meeting UUID.  If a meeting ID is provided in the request instead of a UUID, the response will be for the latest meeting instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     */
    pub async fn meeting_details(
        &self,
        meeting_id: &str,
    ) -> Result<crate::types::ReportMeetingDetailsResponse> {
        let url = format!(
            "/report/meetings/{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Get meeting participant reports.
     *
     * This function performs a `GET` to the `/report/meetings/{meetingId}/participants` endpoint.
     *
     * Use this API to return a report of a past meeting with two or more participants, including the host.
     *
     * To return a report for past meeting with only **one** participant, use the [List meeting participants](https://marketplace.zoom.us/docs/api-reference/zoom-api/dashboards/dashboardmeetingparticipants) API.
     *
     * **Scopes:** `report:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     *
     * **Prerequisites:**
     * * Pro or a higher plan.
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- The meeting ID or the meeting UUID.  If a meeting ID is provided in the request instead of a UUID, the response will be for the latest meeting instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `include_fields: crate::types::DashboardMeetingParticipantsIncludeFields` -- Provide `registrant_id` as the value for this field if you would like to see the registrant ID attribute in the response of this API call. A registrant ID is a unique identifier of a [meeting registrant](https://marketplace.zoom.us/docs/api-reference/zoom-api/meetings/meetingregistrants). This is not supported for `live` meeting types.
     */
    pub async fn meeting_participant(
        &self,
        meeting_id: &str,
        page_size: i64,
        next_page_token: &str,
        include_fields: crate::types::DashboardMeetingParticipantsIncludeFields,
    ) -> Result<crate::types::ReportMeetingParticipantsResponseAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include_fields.to_string().is_empty() {
            query_args.push(("include_fields".to_string(), include_fields.to_string()));
        }
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/report/meetings/{}/participants?{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get meeting poll reports.
     *
     * This function performs a `GET` to the `/report/meetings/{meetingId}/polls` endpoint.
     *
     * Retrieve a report of [poll](https://support.zoom.us/hc/en-us/articles/213756303-Polling-for-Meetings) results for a past meeting. <br><br>
     * **Scopes:** `report:read:admin`<br>
     *  
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * **Prerequisites:**<br>
     * * Pro or a higher plan.<br>
     *  
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- The meeting ID or the meeting UUID.  If a meeting ID is provided in the request instead of a UUID, the response will be for the latest meeting instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     */
    pub async fn meeting_polls(
        &self,
        meeting_id: &str,
    ) -> Result<crate::types::ReportMeetingPollsResponse> {
        let url = format!(
            "/report/meetings/{}/polls",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Get webinar detail reports.
     *
     * This function performs a `GET` to the `/report/webinars/{webinarId}` endpoint.
     *
     * Retrieve a [report](https://support.zoom.us/hc/en-us/articles/201393719-Webinar-Reporting) containing past webinar details.  <br><br>
     * **Scopes:** `report:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * **Prerequisites:**<br>
     * * Pro or higher plan with Webinar add-on.
     *
     * **Parameters:**
     *
     * * `webinar_id: &str` -- The webinar ID or the webinar UUID.  If a webinar ID is provided in the request instead of a UUID, the response will be for the latest webinar instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     */
    pub async fn webinar_details(
        &self,
        webinar_id: &str,
    ) -> Result<crate::types::ReportMeetingDetailsResponse> {
        let url = format!(
            "/report/webinars/{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Get webinar participant reports.
     *
     * This function performs a `GET` to the `/report/webinars/{webinarId}/participants` endpoint.
     *
     * Get detailed report on each attendee of a webinar.<br><br>
     * **Scopes:** `report:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * **Prerequisites:**<br>
     * * Pro or a higher plan with Webinar add-on enabled.
     *
     * **Parameters:**
     *
     * * `webinar_id: &str` -- The webinar ID or the webinar UUID.  If a webinar ID is provided in the request instead of a UUID, the response will be for the latest webinar instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `include_fields: crate::types::DashboardMeetingParticipantsIncludeFields` -- Provide `registrant_id` as the value for this field if you would like to see the registrant ID attribute in the response of this API call. A registrant ID is a unique identifier of a [meeting registrant](https://marketplace.zoom.us/docs/api-reference/zoom-api/meetings/meetingregistrants). This is not supported for `live` meeting types.
     */
    pub async fn webinar_participant(
        &self,
        webinar_id: &str,
        page_size: i64,
        next_page_token: &str,
        include_fields: crate::types::DashboardMeetingParticipantsIncludeFields,
    ) -> Result<crate::types::ReportWebinarParticipantsResponseAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include_fields.to_string().is_empty() {
            query_args.push(("include_fields".to_string(), include_fields.to_string()));
        }
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/report/webinars/{}/participants?{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get webinar poll reports.
     *
     * This function performs a `GET` to the `/report/webinars/{webinarId}/polls` endpoint.
     *
     * Retrieve a report on past [webinar polls](https://support.zoom.us/hc/en-us/articles/203749865-Polling-for-Webinars).<br><br>
     * **Scopes:** `report:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * **Prerequisites:**<br>
     * * Pro or a higher plan with Webinar add-on enabled.
     *
     * **Parameters:**
     *
     * * `webinar_id: &str` -- The webinar ID or the webinar UUID.  If a webinar ID is provided in the request instead of a UUID, the response will be for the latest webinar instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     */
    pub async fn webinar_polls(
        &self,
        webinar_id: &str,
    ) -> Result<crate::types::ReportMeetingPollsResponse> {
        let url = format!(
            "/report/webinars/{}/polls",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Get webinar Q&A report.
     *
     * This function performs a `GET` to the `/report/webinars/{webinarId}/qa` endpoint.
     *
     * The Question & Answer (Q&A) feature for webinars allows attendees to ask questions during the webinar and for the panelists, co-hosts and host to answer their questions.
     *
     * Use this API to retrieve a report on question and answers from past webinars. <br><br>
     * **Scopes:** `report:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * **Prerequisites:**<br>
     * * Pro or a higher plan with Webinar add-on enabled.
     *
     * **Parameters:**
     *
     * * `webinar_id: &str` -- The webinar ID or the webinar UUID.  If a webinar ID is provided in the request instead of a UUID, the response will be for the latest webinar instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     */
    pub async fn webinar_qa(
        &self,
        webinar_id: &str,
    ) -> Result<crate::types::ReportWebinarQaResponse> {
        let url = format!(
            "/report/webinars/{}/qa",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Get telephone reports.
     *
     * This function performs a `GET` to the `/report/telephone` endpoint.
     *
     * The [telephone report](https://support.zoom.us/hc/en-us/articles/206514816-Telephone-reports) allows you to view who dialed into meetings via phone (Audio Conferencing or SIP Connected Audio) and which number they dialed into and other details. Use this API to get telephone report for a specified period of time.
     *
     * **Scopes:** `report:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>**Prerequisites:**<br>
     * * Pro or higher plan.
     *
     * **Parameters:**
     *
     * * `type_: &str` -- Audio types:<br>`1` - Toll-free Call-in & Call-out.<br>`2` - Toll <br>
     *  `3` - SIP Connected Audio.
     * * `query_date_type: crate::types::QueryDateType` -- Date types:<br>`start_time` - Query by call start time.<br>`end_time` - Query by call end time.
     * * `from: chrono::NaiveDate` -- Start date in 'yyyy-mm-dd' format. The date range defined by the "from" and "to" parameters should only be one month as the report includes only one month worth of data at once.
     * * `to: chrono::NaiveDate` -- Start Date.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `page_number: i64` --
     *   **Deprecated** - This field has been deprecated and we will stop supporting it completely in a future release. Please use "next_page_token" for pagination instead of this field.
     *   
     *   The page number of the current page in the returned records.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn telephone(
        &self,
        type_: &str,
        query_date_type: crate::types::QueryDateType,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
        page_size: i64,
        page_number: i64,
        next_page_token: &str,
    ) -> Result<crate::types::Domains> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_number > 0 {
            query_args.push(("page_number".to_string(), page_number.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !query_date_type.to_string().is_empty() {
            query_args.push(("query_date_type".to_string(), query_date_type.to_string()));
        }
        if !to.to_string().is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/report/telephone?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Get cloud recording usage report.
     *
     * This function performs a `GET` to the `/report/cloud_recording` endpoint.
     *
     * Retrieve cloud recording usage report for a specified period. You can only get cloud recording reports that is one day ealier than the current date and for the most recent period of 6 months. The date gap between from and to dates should be smaller or equal to 30 days. <br>
     * **Prerequisites**<br>
     * * Pro or higher plan.<br>
     * **Scopes:** `report:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     *
     * **Parameters:**
     *
     * * `from: chrono::NaiveDate` -- Start date in 'yyyy-mm-dd' format. The date range defined by the "from" and "to" parameters should only be one month as the report includes only one month worth of data at once.
     * * `to: chrono::NaiveDate` -- Start Date.
     */
    pub async fn cloud_recording(
        &self,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    ) -> Result<crate::types::ReportCloudRecordingResponseAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !to.to_string().is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/report/cloud_recording?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Get operation logs report.
     *
     * This function performs a `GET` to the `/report/operationlogs` endpoint.
     *
     * The [Operations Logs](https://support.zoom.us/hc/en-us/articles/360032748331-Operation-Logs) report allows you to audit admin and user activity, such as adding a new user, changing account settings, and deleting recordings.<br>
     * Use this API to retrieve operation logs report for a specified period of time.<br>
     * **Scopes:** `report:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * **Prerequisites:**<br>
     * * Pro or higher plan.
     *
     * **Parameters:**
     *
     * * `from: chrono::NaiveDate` -- Start date in 'yyyy-mm-dd' format. The date range defined by the "from" and "to" parameters should only be one month as the report includes only one month worth of data at once.
     * * `to: chrono::NaiveDate` -- Start Date.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `category_type: crate::types::CategoryType` -- \*\*Optional\*\*<br>
     *  Filter your response by a category type to see reports for a specific category.
     *  The value for this field can be one of the following:<br> `all`<br>`user`<br>`user_settings`<br>`account`<br>`billing`<br>`im`<br>`recording`<br>`phone_contacts`<br>`webinar`<br>`sub_account`<br>`role`<br>`zoom_rooms`.
     */
    pub async fn operation_log(
        &self,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
        page_size: i64,
        next_page_token: &str,
        category_type: crate::types::CategoryType,
    ) -> Result<crate::types::ReportOperationLogsResponseAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !category_type.to_string().is_empty() {
            query_args.push(("category_type".to_string(), category_type.to_string()));
        }
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
        let url = format!("/report/operationlogs?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Get sign In / sign out activity report.
     *
     * This function performs a `GET` to the `/report/activities` endpoint.
     *
     * Retrieve a list of sign in / sign out activity logs [report](https://support.zoom.us/hc/en-us/articles/201363213-Getting-Started-with-Reports) of users under a Zoom account.<br>
     * **Prerequisites**<br>
     * * Pro or higher plan.<br>
     * **Scopes:** `report:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     *
     * **Parameters:**
     *
     * * `from: chrono::NaiveDate` -- Start date for which you would like to view the activity logs report. Using the `from` and `to` parameters, specify a monthly date range for the report as the API only provides one month worth of data in one request. The specified date range should fall within the last six months.
     * * `to: chrono::NaiveDate` -- End date up to which you would like to view the activity logs report.
     * * `page_size: i64` -- The number of records to be returned within a single API call.
     * * `next_page_token: &str` -- Next page token is used to paginate through large result sets.
     */
    pub async fn sign_out_activities(
        &self,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<Vec<crate::types::ActivityLogs>> {
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
        let url = format!("/report/activities?{}", query_);

        let resp: crate::types::ReportSignInOutActivitiesResponse =
            self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.activity_logs)
    }

    /**
     * Get sign In / sign out activity report.
     *
     * This function performs a `GET` to the `/report/activities` endpoint.
     *
     * As opposed to `sign_out_activities`, this function returns all the pages of the request at once.
     *
     * Retrieve a list of sign in / sign out activity logs [report](https://support.zoom.us/hc/en-us/articles/201363213-Getting-Started-with-Reports) of users under a Zoom account.<br>
     * **Prerequisites**<br>
     * * Pro or higher plan.<br>
     * **Scopes:** `report:read:admin`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     */
    pub async fn get_all_sign_out_activities(
        &self,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    ) -> Result<Vec<crate::types::ActivityLogs>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !to.to_string().is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/report/activities?{}", query_);

        let mut resp: crate::types::ReportSignInOutActivitiesResponse =
            self.client.get(&url, None).await?;

        let mut activity_logs = resp.activity_logs;
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

            activity_logs.append(&mut resp.activity_logs);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(activity_logs)
    }

    /**
     * Get billing reports.
     *
     * This function performs a `GET` to the `/report/billing` endpoint.
     *
     * Get department billing reports of a Zoom account.
     *
     * **Prerequisites:**<br>
     * * Pro or a higher account with Department Billing option enabled. Contact Zoom Support team for details.
     *
     * **Scopes:** `report:read:admin`, `report:master`
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     */
    pub async fn get_billing(&self) -> Result<crate::types::GetBillingReportResponse> {
        let url = "/report/billing".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Get billing invoice reports.
     *
     * This function performs a `GET` to the `/report/billing/invoices` endpoint.
     *
     * Get department billing invoices reports for a specific billing period. Provide the `billing_id` of the billing period for which you would like to retrieve the invoices for. This ID can be retrieved from **Get Billing Reports** API.
     *
     * **Prerequisites:**<br>
     * * Pro or a higher account with Department Billing option enabled. Contact the Zoom Support team to enable this feature.
     *
     * **Scopes:** `report:read:admin`, `report:master`
     *
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     *
     * **Parameters:**
     *
     * * `billing_id: &str` -- Unique Identifier of the Billing Report. Retrieve this ID from the response of **Get Billing Reports** API request.
     *   
     *   .
     */
    pub async fn get_billing_invoices(
        &self,
        billing_id: &str,
    ) -> Result<crate::types::GetBillingInvoicesReportsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !billing_id.is_empty() {
            query_args.push(("billing_id".to_string(), billing_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/report/billing/invoices?{}", query_);

        self.client.get(&url, None).await
    }
}
