use anyhow::Result;

use crate::Client;

pub struct Dashboards {
    pub client: Client,
}

impl Dashboards {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Dashboards { client }
    }

    /**
     * List meetings.
     *
     * This function performs a `GET` to the `/metrics/meetings` endpoint.
     *
     * List total live or past meetings that occurred during a specified period of time. This overview will show if features such as audio, video, screen sharing, and recording were being used in the meeting. You can also see the license types of each user on your account.<br> You can specify a monthly date range for the dashboard data using the `from` and `to` query parameters. The month should fall within the last six months.<br>
     * **Scopes:** `dashboard_meetings:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Resource-intensive`<br><br>
     * **Prerequisites:** <br>
     * * Business or a higher plan.<br><br>
     *
     * **Parameters:**
     *
     * * `type_: crate::types::DashboardMeetingsType` -- Specify a value to get the response for the corresponding meeting type. The value of this field can be one of the following:<br> <br>`past` - Meeting that already occurred in the specified date range.<br>`pastOne` - Past meetings that were attended by only one user. <br>`live` - Live meetings.<br><br>
     *  
     *  If you do not provide this field, the default value will be `live` and thus, the API will only query responses for live meetings.
     * * `from: chrono::NaiveDate` -- Start date in 'yyyy-mm-dd' format. The date range defined by the "from" and "to" parameters should only be one month as the report includes only one month worth of data at once.
     * * `to: chrono::NaiveDate` -- Start Date.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `include_fields: crate::types::IncludeFields` -- Set the value of this field to "tracking_fields" if you would like to include tracking fields of each meeting in the response.
     */
    pub async fn meeting(
        &self,
        type_: crate::types::DashboardMeetingsType,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
        page_size: i64,
        next_page_token: &str,
        include_fields: crate::types::IncludeFields,
    ) -> Result<crate::types::DashboardMeetingsResponseAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !include_fields.to_string().is_empty() {
            query_args.push(("include_fields".to_string(), include_fields.to_string()));
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
        let url = format!("/metrics/meetings?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Get meeting details.
     *
     * This function performs a `GET` to the `/metrics/meetings/{meetingId}` endpoint.
     *
     * Get details on live or past meetings. This overview will show if features such as audio, video, screen sharing, and recording were being used in the meeting. You can also see the license types of each user on your account.<br> You can specify a monthly date range for the dashboard data using the `from` and `to` query parameters. The month should fall within the last six months.  <br>
     * **Scopes:** `dashboard_meetings:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * **Prerequisites:** <br>
     * * Business or a higher plan.
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- The meeting ID or the meeting UUID.  If a meeting ID is provided in the request instead of a UUID, the response will be for the latest meeting instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     * * `type_: crate::types::DashboardMeetingsType` -- Specify a value to get the response for the corresponding meeting type. The value of this field can be one of the following:<br> <br>`past` - Meeting that already occurred in the specified date range.<br>`pastOne` - Past meetings that were attended by only one user. <br>`live` - Live meetings.<br><br>
     *  
     *  If you do not provide this field, the default value will be `live` and thus, the API will only query responses for live meetings.
     */
    pub async fn meeting_detail(
        &self,
        meeting_id: &str,
        type_: crate::types::DashboardMeetingsType,
    ) -> Result<crate::types::MeetingMetric> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/metrics/meetings/{}?{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * List meeting participants.
     *
     * This function performs a `GET` to the `/metrics/meetings/{meetingId}/participants` endpoint.
     *
     * Get a list of participants from live or past meetings.<br><br>
     * If you do not provide the `type` query parameter, the default value will be set to `live` and thus, you will only see metrics for participants in a live meeting, if any meeting is currently being conducted. To view metrics on past meeting participants, provide the appropriate value for `type`. <br> You can specify a monthly date range for the dashboard data using the `from` and `to` query parameters. The month should fall within the last six months.
     *
     * **Scopes:** `dashboard_meetings:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * **Prerequisites:** Business or a higher plan.
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- The meeting ID or the meeting UUID.  If a meeting ID is provided in the request instead of a UUID, the response will be for the latest meeting instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     * * `type_: crate::types::DashboardMeetingsType` -- Specify a value to get the response for the corresponding meeting type. The value of this field can be one of the following:<br> <br>`past` - Meeting that already occurred in the specified date range.<br>`pastOne` - Past meetings that were attended by only one user. <br>`live` - Live meetings.<br><br>
     *  
     *  If you do not provide this field, the default value will be `live` and thus, the API will only query responses for live meetings.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `include_fields: crate::types::DashboardMeetingParticipantsIncludeFields` -- Provide `registrant_id` as the value for this field if you would like to see the registrant ID attribute in the response of this API call. A registrant ID is a unique identifier of a [meeting registrant](https://marketplace.zoom.us/docs/api-reference/zoom-api/meetings/meetingregistrants). This is not supported for `live` meeting types.
     */
    pub async fn meeting_participant(
        &self,
        meeting_id: &str,
        type_: crate::types::DashboardMeetingsType,
        page_size: i64,
        next_page_token: &str,
        include_fields: crate::types::DashboardMeetingParticipantsIncludeFields,
    ) -> Result<crate::types::DashboardMeetingParticipantsResponseAllOf> {
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
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/metrics/meetings/{}/participants?{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get meeting participant QoS.
     *
     * This function performs a `GET` to the `/metrics/meetings/{meetingId}/participants/{participantId}/qos` endpoint.
     *
     * Use this API to retrieve the quality of service (QoS) for participants from live or past meetings. The data returned indicates the connection quality for sending/receiving video, audio, and shared content. The API returns this data for either the API request or when the API request was last received.
     *
     * This API will **not** return data if there is no data being sent or received at the time of request.
     * <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note:</b> When the sender sends its data, a timestamp is attached to the sender’s data packet. The receiver then returns this timestamp to the sender. This helps determine the upstream and downstream latency, which includes the application processing time. The latency data returned is the five second average and five second maximum.</p>
     *
     * **Scopes:** `dashboard_meetings:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- The meeting ID or the meeting UUID.  If a meeting ID is provided in the request instead of a UUID, the response will be for the latest meeting instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     * * `participant_id: &str` -- User's first name.
     * * `type_: crate::types::DashboardMeetingsType` -- Specify a value to get the response for the corresponding meeting type. The value of this field can be one of the following:<br> <br>`past` - Meeting that already occurred in the specified date range.<br>`pastOne` - Past meetings that were attended by only one user. <br>`live` - Live meetings.<br><br>
     *  
     *  If you do not provide this field, the default value will be `live` and thus, the API will only query responses for live meetings.
     */
    pub async fn meeting_participant_qo(
        &self,
        meeting_id: &str,
        participant_id: &str,
        type_: crate::types::DashboardMeetingsType,
    ) -> Result<crate::types::ParticipantQos> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/metrics/meetings/{}/participants/{}/qos?{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            crate::progenitor_support::encode_path(&participant_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * List meeting participants QoS.
     *
     * This function performs a `GET` to the `/metrics/meetings/{meetingId}/participants/qos` endpoint.
     *
     * Get a list of meeting participants from live or past meetings along with the quality of service they recieve during the meeting such as connection quality for sending/receiving video, audio, and shared content.<br>If you do not provide the `type` query parameter, the default value will be set to `live` and thus, you will only see metrics for participants in a live meeting, if any meeting is currently being conducted. To view metrics on past meeting participants, provide the appropriate value for `type`.<br> <br> You can specify a monthly date range for the dashboard data using the `from` and `to` query parameters. The month should fall within the last six months.<br><br>
     * **Scopes:** `dashboard_meetings:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * **Prerequisites:** <br>
     * * Business or a higher plan.
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- The meeting ID or the meeting UUID.  If a meeting ID is provided in the request instead of a UUID, the response will be for the latest meeting instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     * * `type_: crate::types::DashboardMeetingsType` -- Specify a value to get the response for the corresponding meeting type. The value of this field can be one of the following:<br> <br>`past` - Meeting that already occurred in the specified date range.<br>`pastOne` - Past meetings that were attended by only one user. <br>`live` - Live meetings.<br><br>
     *  
     *  If you do not provide this field, the default value will be `live` and thus, the API will only query responses for live meetings.
     * * `page_size: i64` -- The number of items returned per page.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn meeting_participants_qo(
        &self,
        meeting_id: &str,
        type_: crate::types::DashboardMeetingsType,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<crate::types::Domains> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/metrics/meetings/{}/participants/qos?{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get sharing/recording details.
     *
     * This function performs a `GET` to the `/metrics/meetings/{meetingId}/participants/sharing` endpoint.
     *
     * Retrieve the sharing and recording details of participants from live or past meetings.<br>
     * **Scopes:** `dashboard_meetings:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * **Prerequisites:** <br>
     * * Business or a higher plan.
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- The meeting ID or the meeting UUID.  If a meeting ID is provided in the request instead of a UUID, the response will be for the latest meeting instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     * * `type_: crate::types::DashboardMeetingsType` -- Specify a value to get the response for the corresponding meeting type. The value of this field can be one of the following:<br> <br>`past` - Meeting that already occurred in the specified date range.<br>`pastOne` - Past meetings that were attended by only one user. <br>`live` - Live meetings.<br><br>
     *  
     *  If you do not provide this field, the default value will be `live` and thus, the API will only query responses for live meetings.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceed the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn meeting_participant_share(
        &self,
        meeting_id: &str,
        type_: crate::types::DashboardMeetingsType,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<crate::types::DashboardMeetingParticipantShareResponseAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/metrics/meetings/{}/participants/sharing?{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * List webinars.
     *
     * This function performs a `GET` to the `/metrics/webinars` endpoint.
     *
     * List all the live or past webinars from a specified period of time. <br><br>
     * **Scopes:** `dashboard_webinars:read:admin`<br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Resource-intensive`<br>
     * **Prerequisites:**<br>
     * * Business, Education or API Plan with Webinar add-on.
     *
     *
     *
     *
     * **Parameters:**
     *
     * * `type_: crate::types::DashboardWebinarsType` -- The webinar type.
     * * `from: chrono::NaiveDate` -- Start date in 'yyyy-mm-dd' format. The date range defined by the "from" and "to" parameters should only be one month as the report includes only one month worth of data at once.
     * * `to: chrono::NaiveDate` -- Start Date.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn webinar(
        &self,
        type_: crate::types::DashboardWebinarsType,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<crate::types::DashboardWebinarsResponseAllOf> {
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
        let url = format!("/metrics/webinars?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Get webinar details.
     *
     * This function performs a `GET` to the `/metrics/webinars/{webinarId}` endpoint.
     *
     * Retrieve details from live or past webinars.<br><br>
     * **Scopes:** `dashboard_webinars:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * **Prerequisites:**<br>
     * * Business, Education or API Plan with Webinar add-on.
     *
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: &str` -- The webinar ID or the webinar UUID.  If a webinar ID is provided in the request instead of a UUID, the response will be for the latest webinar instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     * * `type_: crate::types::DashboardWebinarsType` -- The webinar type.
     */
    pub async fn webinar_detail(
        &self,
        webinar_id: &str,
        type_: crate::types::DashboardWebinarsType,
    ) -> Result<crate::types::Webinars> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/metrics/webinars/{}?{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get webinar participants.
     *
     * This function performs a `GET` to the `/metrics/webinars/{webinarId}/participants` endpoint.
     *
     * Retrieve details on participants from live or past webinars.<br><br>
     * **Scopes:** `dashboard_webinars:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * **Prerequisites:**<br>
     * * Business, Education or API Plan with Webinar add-on.
     *
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: &str` -- The webinar ID or the webinar UUID.  If a webinar ID is provided in the request instead of a UUID, the response will be for the latest webinar instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     * * `type_: crate::types::DashboardWebinarsType` -- The webinar type.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `include_fields: crate::types::DashboardMeetingParticipantsIncludeFields` -- Provide `registrant_id` as the value for this field if you would like to see the registrant ID attribute in the response of this API call. A registrant ID is a unique identifier of a [meeting registrant](https://marketplace.zoom.us/docs/api-reference/zoom-api/meetings/meetingregistrants). This is not supported for `live` meeting types.
     */
    pub async fn webinar_participant(
        &self,
        webinar_id: &str,
        type_: crate::types::DashboardWebinarsType,
        page_size: i64,
        next_page_token: &str,
        include_fields: crate::types::DashboardMeetingParticipantsIncludeFields,
    ) -> Result<crate::types::DashboardWebinarParticipantsResponseAllOf> {
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
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/metrics/webinars/{}/participants?{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get webinar participant QoS.
     *
     * This function performs a `GET` to the `/metrics/webinars/{webinarId}/participants/{participantId}/qos` endpoint.
     *
     * Retrieve details on the quality of service that participants from live or past webinars recieved.<br>This data indicates the connection quality for sending/receiving video, audio, and shared content. If nothing is being sent or received at that time, no information will be shown in the fields.<br>
     * **Scopes:** `dashboard_webinars:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy` <br>
     * **Prerequisites:** <br>
     * * Business, Education or API Plan with Zoom Rooms set up.
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: &str` -- The webinar ID or the webinar UUID.  If a webinar ID is provided in the request instead of a UUID, the response will be for the latest webinar instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     * * `participant_id: &str` -- User's first name.
     * * `type_: crate::types::DashboardWebinarsType` -- The webinar type.
     */
    pub async fn webinar_participant_qo(
        &self,
        webinar_id: &str,
        participant_id: &str,
        type_: crate::types::DashboardWebinarsType,
    ) -> Result<crate::types::ParticipantQos> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/metrics/webinars/{}/participants/{}/qos?{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
            crate::progenitor_support::encode_path(&participant_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * List webinar participant QoS.
     *
     * This function performs a `GET` to the `/metrics/webinars/{webinarId}/participants/qos` endpoint.
     *
     * Retrieve a list of participants from live or past webinars and the quality of service they received.<br>This data indicates the connection quality for sending/receiving video, audio, and shared content. If nothing is being sent or received at that time, no information will be shown in the fields.<br>
     * **Scopes:** `dashboard_webinars:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * **Prerequisites:**
     * * Business, Education or API Plan with Webinar add-on.
     *
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: &str` -- The webinar ID or the webinar UUID.  If a webinar ID is provided in the request instead of a UUID, the response will be for the latest webinar instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     * * `type_: crate::types::DashboardWebinarsType` -- The webinar type.
     * * `page_size: i64` -- The number of items returned per page.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn webinar_participants_qo(
        &self,
        webinar_id: &str,
        type_: crate::types::DashboardWebinarsType,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<crate::types::Domains> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/metrics/webinars/{}/participants/qos?{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get sharing/recording details.
     *
     * This function performs a `GET` to the `/metrics/webinars/{webinarId}/participants/sharing` endpoint.
     *
     * Retrieve the sharing and recording details of participants from live or past webinars. <br><br>
     * **Scopes:** `dashboard_webinars:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy` <br>
     * **Prerequisites:**<br>
     * * Business, Education or API Plan with Webinar add-on.
     *
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: &str` -- The webinar ID or the webinar UUID.  If a webinar ID is provided in the request instead of a UUID, the response will be for the latest webinar instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     * * `type_: crate::types::DashboardWebinarsType` -- The webinar type.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceed the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn webinar_participant_share(
        &self,
        webinar_id: &str,
        type_: crate::types::DashboardWebinarsType,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<crate::types::DashboardMeetingParticipantShareResponseAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/metrics/webinars/{}/participants/sharing?{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * List Zoom Rooms.
     *
     * This function performs a `GET` to the `/metrics/zoomrooms` endpoint.
     *
     * List information on all Zoom Rooms in an account.<br><br>
     * **Scopes:** `dashboard_zr:read:admin`
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Resource-intensive`<br>
     *  **Prerequisites:**<br>
     * * Business, Education or API Plan with Zoom Rooms set up.
     *
     *
     *
     *
     * **Parameters:**
     *
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `page_number: i64` -- The page number of the current page in the returned records.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn zoom_room(
        &self,
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
        let url = format!("/metrics/zoomrooms?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Get Zoom Rooms details.
     *
     * This function performs a `GET` to the `/metrics/zoomrooms/{zoomroomId}` endpoint.
     *
     * The Zoom Rooms dashboard metrics lets you know the type of configuration a Zoom room has and details on the meetings held in that room.
     *
     * Use this API to retrieve information on a specific room.<br><br>
     * **Scopes:** `dashboard_zr:read:admin`<br> <br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`**Prerequisites:**<br>
     * * Business, Education or API Plan with Zoom Rooms set up.
     *
     *
     * **Parameters:**
     *
     * * `zoomroom_id: &str` -- User's first name.
     * * `from: chrono::NaiveDate` -- Start date in 'yyyy-mm-dd' format. The date range defined by the "from" and "to" parameters should only be one month as the report includes only one month worth of data at once.
     * * `to: chrono::NaiveDate` -- Start Date.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn zoom_room_dashboards(
        &self,
        zoomroom_id: &str,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<crate::types::Domains> {
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
            "/metrics/zoomrooms/{}?{}",
            crate::progenitor_support::encode_path(&zoomroom_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get CRC port usage.
     *
     * This function performs a `GET` to the `/metrics/crc` endpoint.
     *
     * A Cloud Room Connector allows H.323/SIP endpoints to connect to a Zoom meeting.
     *
     * Use this API to get the hour by hour CRC Port usage for a specified period of time. <aside class='notice'>We will provide the report for a maximum of one month. For example, if "from" is set to "2017-08-05" and "to" is set to "2017-10-10", we will adjust "from" to "2017-09-10".</aside><br><br>
     * **Prerequisites:**<br>
     * * Business, Education or API Plan.
     * * Room Connector must be enabled on the account.<br><br>
     * **Scopes:** `dashboard_crc:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     *
     * **Parameters:**
     *
     * * `from: chrono::NaiveDate` -- Start date in 'yyyy-mm-dd' format. The date range defined by the "from" and "to" parameters should only be one month as the report includes only one month worth of data at once.
     * * `to: chrono::NaiveDate` -- Start Date.
     */
    pub async fn crc(
        &self,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    ) -> Result<crate::types::Domains> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !to.to_string().is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/metrics/crc?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Get IM metrics.
     *
     * This function performs a `GET` to the `/metrics/im` endpoint.
     *
     * Get [metrics](https://support.zoom.us/hc/en-us/articles/204654719-Dashboard#h_cc7e9749-1c70-4afb-a9a2-9680654821e4) on how users are utilizing the Zoom Chat client.
     *
     * You can specify a monthly date range for the dashboard data using the `from` and `to` query parameters. The month should fall within the last six months.<p style="background-color:#e1f5fe; color:#000000; padding:8px"><b>Deprecated:</b> We will completely deprecate this endpoint in a future release. You can continue using this endpoint to query data for messages sent <b>before</b> July 1, 2021.</br></br>To get metrics on chat messages sent <b>on and after</b> July 1, 2021, use the <a href="https://marketplace.zoom.us/docs/api-reference/zoom-api/dashboards/dashboardchat">Get chat metrics API</a>.</p>
     *
     * **Scopes:** `dashboard_im:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Resource-intensive`
     *
     * **Prerequisites:**
     *
     * * Business or a higher plan.
     *
     * **Parameters:**
     *
     * * `from: chrono::NaiveDate` -- Start date in 'yyyy-mm-dd' format. The date range defined by the "from" and "to" parameters should only be one month as the report includes only one month worth of data at once.
     * * `to: chrono::NaiveDate` -- Start Date.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn im(
        &self,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<crate::types::DashboardImResponseAllOf> {
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
        let url = format!("/metrics/im?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Get chat metrics.
     *
     * This function performs a `GET` to the `/metrics/chat` endpoint.
     *
     * Get [metrics](https://support.zoom.us/hc/en-us/articles/204654719-Dashboard#h_cc7e9749-1c70-4afb-a9a2-9680654821e4) for how users are utilizing Zoom Chat to send messages.
     *
     * Use the `from` and `to` query parameters to specify a monthly date range for the dashboard data. The monthly date range must be within the last six months.
     *
     * > **Note:** To query chat metrics from July 1, 2021 and later, use this endpoint instead of the [Get IM metrics API](https://marketplace.zoom.us/docs/api-reference/zoom-api/dashboards/dashboardim).
     *
     * **Scope:** `dashboard_im:read:admin`</br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Resource-intensive`
     *
     * **Prerequisites:**
     *
     * * Business or a higher plan
     *
     * **Parameters:**
     *
     * * `from: chrono::NaiveDate` -- Start date in 'yyyy-mm-dd' format. The date range defined by the "from" and "to" parameters should only be one month as the report includes only one month worth of data at once.
     * * `to: chrono::NaiveDate` -- Start Date.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn chat(
        &self,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<crate::types::DashboardChatResponseAllOf> {
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
        let url = format!("/metrics/chat?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * List Zoom meetings client feedback.
     *
     * This function performs a `GET` to the `/metrics/client/feedback` endpoint.
     *
     * Retrieve survey results from [Zoom meetings client feedback](https://support.zoom.us/hc/en-us/articles/115005855266-End-of-Meeting-Feedback-Survey#h_e30d552b-6d8e-4e0a-a588-9ca8180c4dbf). <br> You can specify a monthly date range for the dashboard data using the `from` and `to` query parameters. The month should fall within the last six months.
     *
     * **Prerequisites:**
     * * Business or higher account
     * * [Feedback to Zoom](https://support.zoom.us/hc/en-us/articles/115005838023) enabled.
     *
     * **Scope:** `account:read:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     *
     * **Parameters:**
     *
     * * `from: chrono::NaiveDate` -- Start date in 'yyyy-mm-dd' format. The date range defined by the "from" and "to" parameters should only be one month as the report includes only one month worth of data at once.
     * * `to: chrono::NaiveDate` -- Start Date.
     */
    pub async fn client_feedback(
        &self,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    ) -> Result<crate::types::DashboardClientFeedbackResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !to.to_string().is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/metrics/client/feedback?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Get top 25 issues of Zoom Rooms.
     *
     * This function performs a `GET` to the `/metrics/zoomrooms/issues` endpoint.
     *
     * Get top 25 issues of Zoom Rooms.<br>
     * **Scopes:** `dashboard_zr:read:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     *  **Prerequisites:**<br>
     * * Business, Education or API Plan with Zoom Rooms set up.
     *
     *
     *
     * **Parameters:**
     *
     * * `from: chrono::NaiveDate` -- Start date in 'yyyy-mm-dd' format. The date range defined by the "from" and "to" parameters should only be one month as the report includes only one month worth of data at once.
     * * `to: chrono::NaiveDate` -- Start Date.
     */
    pub async fn zoom_room_issue(
        &self,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    ) -> Result<crate::types::Domains> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !to.to_string().is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/metrics/zoomrooms/issues?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Get top 25 Zoom Rooms with issues.
     *
     * This function performs a `GET` to the `/metrics/issues/zoomrooms` endpoint.
     *
     * Get information on top 25 Zoom Rooms with issues in a month. The month specified with the "from" and "to" range should fall within the last six months.<br>
     * **Scope:** `dashboard_home:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * **Prerequisites:**<br>
     * * Business or a higher plan.
     * * Zoom Room must be enabled in the account.
     *
     * **Parameters:**
     *
     * * `from: chrono::NaiveDate` -- Start date in 'yyyy-mm-dd' format. The date range defined by the "from" and "to" parameters should only be one month as the report includes only one month worth of data at once.
     * * `to: chrono::NaiveDate` -- Start Date.
     */
    pub async fn issue_zoom_room(
        &self,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    ) -> Result<crate::types::DashboardIssueZoomRoomResponseAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !to.to_string().is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/metrics/issues/zoomrooms?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Get issues of Zoom Rooms.
     *
     * This function performs a `GET` to the `/metrics/issues/zoomrooms/{zoomroomId}` endpoint.
     *
     * Get information about the issues that occured on the Top 25 **Zoom Rooms with issues** in an acount. <br> You can specify a monthly date range for the dashboard data using the `from` and `to` query parameters. The month should fall within the last six months.
     *
     * **Scope:** `dashboard_home:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * **Prerequisites:** <br>
     * * Business or a higher plan.
     * * Zoom Room must be enabled in the account.
     *
     * **Parameters:**
     *
     * * `zoomroom_id: &str` -- User's first name.
     * * `from: chrono::NaiveDate` -- Start date in 'yyyy-mm-dd' format. The date range defined by the "from" and "to" parameters should only be one month as the report includes only one month worth of data at once.
     * * `to: chrono::NaiveDate` -- Start Date.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn issue_detail_zoom_room(
        &self,
        zoomroom_id: &str,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<crate::types::DashboardIssueDetailZoomRoomResponseAllOf> {
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
            "/metrics/issues/zoomrooms/{}?{}",
            crate::progenitor_support::encode_path(&zoomroom_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get zoom meetings client feedback.
     *
     * This function performs a `GET` to the `/metrics/client/feedback/{feedbackId}` endpoint.
     *
     * Retrieve detailed information on a [Zoom meetings client feedback](https://support.zoom.us/hc/en-us/articles/115005855266-End-of-Meeting-Feedback-Survey#h_e30d552b-6d8e-4e0a-a588-9ca8180c4dbf). <br> You can specify a monthly date range for the dashboard data using the `from` and `to` query parameters. The month should fall within the last six months.
     *
     * **Prerequisites:**
     * * Business or higher account
     * * [Feedback to Zoom](https://support.zoom.us/hc/en-us/articles/115005838023) enabled.
     *
     * **Scope:** `dashboard_home:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     *
     * `
     *
     * **Parameters:**
     *
     * * `feedback_id: &str` -- User's first name.
     * * `from: chrono::NaiveDate` -- Start Date.
     * * `to: chrono::NaiveDate` -- Start Date.
     * * `page_size: i64` -- Account seats.
     * * `next_page_token: &str` -- User's first name.
     */
    pub async fn client_feedback_detail(
        &self,
        feedback_id: &str,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<crate::types::DashboardClientFeedbackDetailResponseAllOf> {
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
            "/metrics/client/feedback/{}?{}",
            crate::progenitor_support::encode_path(&feedback_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * List client meeting satisfaction.
     *
     * This function performs a `GET` to the `/metrics/client/satisfaction` endpoint.
     *
     * If the [End of Meeting Feedback Survey](https://support.zoom.us/hc/en-us/articles/115005855266) option is enabled, attendees will be prompted with a survey window where they can tap either the **Thumbs Up** or **Thumbs Down** button that indicates their Zoom meeting experience. With this API, you can get information on the attendees' meeting satisfaction. Specify a monthly date range for the query using the from and to query parameters. The month should fall within the last six months.
     *
     * To get information on the survey results with negative experiences (indicated by **Thumbs Down**), use the [Get Zoom Meetings Client Feedback API](https://marketplace.zoom.us/docs/api-reference/zoom-api/dashboards/dashboardclientfeedbackdetail).<br>
     * **Scopes:** `dashboard:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     *
     * **Parameters:**
     *
     * * `from: chrono::NaiveDate` -- The start date for the query in “yyyy-mm-dd” format. .
     * * `to: chrono::NaiveDate` -- The end date for the query in “yyyy-mm-dd” format. .
     */
    pub async fn list_meeting_satisfaction(
        &self,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    ) -> Result<crate::types::ListMeetingSatisfactionResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !to.to_string().is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/metrics/client/satisfaction?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * List call logs.
     *
     * This function performs a `GET` to the `/phone/metrics/call_logs` endpoint.
     *
     * Call logs provide a record of all incoming and outgoing calls over Zoom Phone in an account.
     *
     * Use this API to list monthly call logs metrics. You can use query parameters to filter the response by date, site and MOS(Mean Opinion Score) of the call.
     *
     * **Prerequisites:**
     * * Business, or Education account
     * * Zoom Phone license <br><br>
     *
     * **Scopes:** `phone:read:admin`, `phone:write:admin`<br>
     * **Rate Limit Label:** `Heavy`
     *
     * **Parameters:**
     *
     * * `from: &str` -- Start date for the report in `yyyy-mm-dd` format. Specify a 30 day range using the `from` and `to` parameters as the response provides a maximum of a month worth of data per API request.
     * * `to: &str` -- End date for the report in `yyyy-mm-dd` format.
     * * `site_id: &str` -- Unique identifier of the [site](https://support.zoom.us/hc/en-us/articles/360020809672-Managing-multiple-sites). Use this query parameter if you have enabled multiple sites and would like to filter the response of this API call by call logs of a specific phone site.
     * * `quality_type: &str` -- Filter call logs by voice quality. Zoom uses MOS of 3.5 as a general baseline to categorize calls by call quality. A MOS greater than or equal to 3.5 means good quality, while below 3.5 means poor quality. <br><br>The value of this field can be one of the following:<br>
     *   * `good`: Retrieve call logs of the call(s) with good quality of voice.<br>
     *   * `bad`: Retrieve call logs of the call(s) with good quality of voice.<br>
     *   * `all`: Retrieve all call logs without filtering by voice quality.
     *   
     *   
     *   
     *   .
     * * `page_size: i64` -- The number of records returned within a single call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn list_call_logs_metrics(
        &self,
        from: &str,
        to: &str,
        site_id: &str,
        quality_type: &str,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<Vec<crate::types::ListCallLogsMetricsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !quality_type.is_empty() {
            query_args.push(("quality_type".to_string(), quality_type.to_string()));
        }
        if !site_id.is_empty() {
            query_args.push(("site_id".to_string(), site_id.to_string()));
        }
        if !to.is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/metrics/call_logs?{}", query_);

        let resp: crate::types::ListCallLogsMetricsResponseData =
            self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.call_logs)
    }

    /**
     * List call logs.
     *
     * This function performs a `GET` to the `/phone/metrics/call_logs` endpoint.
     *
     * As opposed to `list_call_logs_metrics`, this function returns all the pages of the request at once.
     *
     * Call logs provide a record of all incoming and outgoing calls over Zoom Phone in an account.
     *
     * Use this API to list monthly call logs metrics. You can use query parameters to filter the response by date, site and MOS(Mean Opinion Score) of the call.
     *
     * **Prerequisites:**
     * * Business, or Education account
     * * Zoom Phone license <br><br>
     *
     * **Scopes:** `phone:read:admin`, `phone:write:admin`<br>
     * **Rate Limit Label:** `Heavy`
     */
    pub async fn list_all_call_logs_metrics(
        &self,
        from: &str,
        to: &str,
        site_id: &str,
        quality_type: &str,
    ) -> Result<Vec<crate::types::ListCallLogsMetricsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !quality_type.is_empty() {
            query_args.push(("quality_type".to_string(), quality_type.to_string()));
        }
        if !site_id.is_empty() {
            query_args.push(("site_id".to_string(), site_id.to_string()));
        }
        if !to.is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/metrics/call_logs?{}", query_);

        let mut resp: crate::types::ListCallLogsMetricsResponseData =
            self.client.get(&url, None).await?;

        let mut call_logs = resp.call_logs;
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

            call_logs.append(&mut resp.call_logs);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(call_logs)
    }

    /**
     * Get call details from call log.
     *
     * This function performs a `GET` to the `/phone/metrics/call_logs/{call_id}` endpoint.
     *
     * Call logs provide a record of all incoming and outgoing calls over Zoom Phone in an account.
     *
     * Use this API to list call log details of a specific call.
     *
     * **Prerequisites:**
     * * Business, or Education account
     * * Zoom Phone license <br><br>
     *
     * **Scopes:** `phone:read:admin`, `phone:write:admin`<br>
     * **Rate Limit Label:** `Light`
     *
     *
     * **Parameters:**
     *
     * * `call_id: &str` -- Unique identifier of the phone call. The value of this field can be retrieved from [List Call Logs]() API.
     */
    pub async fn get_call_log_metrics_details(
        &self,
        call_id: &str,
    ) -> Result<crate::types::ListCallLogsMetricsResponse> {
        let url = format!(
            "/phone/metrics/call_logs/{}",
            crate::progenitor_support::encode_path(&call_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Get call QoS.
     *
     * This function performs a `GET` to the `/phone/metrics/call_logs/{callId}/qos` endpoint.
     *
     * Get call quality of service(QoS) data for a call made or received by a Zoom phone user in the account.
     *
     * **Prerequisites:**
     * * Business, or Education account
     * * Zoom Phone license <br><br>
     * **Scopes:** `phone:read:admin`, `phone:write:admin`<br>
     * **Rate Limit Label:** `Light`
     *
     * **Parameters:**
     *
     * * `call_id: &str` -- Unique identifier of the call.
     */
    pub async fn get_call_qo(&self, call_id: &str) -> Result<crate::types::GetCallQoSResponse> {
        let url = format!(
            "/phone/metrics/call_logs/{}/qos",
            crate::progenitor_support::encode_path(&call_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Get post meeting feedback.
     *
     * This function performs a `GET` to the `/metrics/meetings/{meetingId}/participants/satisfaction` endpoint.
     *
     * When a meeting ends, each attendee will be prompted to share their meeting experience by clicking either thumbs up or thumbs down. Use this API to retrieve the feedback submitted for a specific meeting. Note that this API only works for meetings scheduled after December 20, 2020.
     *
     * **Prerequisites:**
     * * [Feedback to Zoom](https://support.zoom.us/hc/en-us/articles/115005838023) setting must be enabled by the participant prior to the meeting.
     * * The user making the API request must be enrolled in a Business or a higher plan.
     *
     * <br> **Scope:** `dashboard_meetings:read:admiin`
     *
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- The meeting ID or the meeting UUID.  If a meeting ID is provided in the request instead of a UUID, the response will be for the latest meeting instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     * * `type_: crate::types::DashboardMeetingsType` -- Specify a value to get the response for the corresponding meeting type. The value of this field can be one of the following:<br> <br>`past` - Meeting that already occurred in the specified date range.<br>`pastOne` - Past meetings that were attended by only one user. <br>`live` - Live meetings.<br><br>
     *  
     *  If you do not provide this field, the default value will be `live` and thus, the API will only query responses for live meetings.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `page_size: i64` -- The number of records returned within a single API call.
     */
    pub async fn participant_feedback(
        &self,
        meeting_id: &str,
        type_: crate::types::DashboardMeetingsType,
        next_page_token: &str,
        page_size: i64,
    ) -> Result<Vec<crate::types::ParticipantFeedbackResponseParticipants>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/metrics/meetings/{}/participants/satisfaction?{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            query_
        );

        let resp: crate::types::ParticipantFeedbackResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.participants)
    }

    /**
     * Get post meeting feedback.
     *
     * This function performs a `GET` to the `/metrics/meetings/{meetingId}/participants/satisfaction` endpoint.
     *
     * As opposed to `participant_feedback`, this function returns all the pages of the request at once.
     *
     * When a meeting ends, each attendee will be prompted to share their meeting experience by clicking either thumbs up or thumbs down. Use this API to retrieve the feedback submitted for a specific meeting. Note that this API only works for meetings scheduled after December 20, 2020.
     *
     * **Prerequisites:**
     * * [Feedback to Zoom](https://support.zoom.us/hc/en-us/articles/115005838023) setting must be enabled by the participant prior to the meeting.
     * * The user making the API request must be enrolled in a Business or a higher plan.
     *
     * <br> **Scope:** `dashboard_meetings:read:admiin`
     *
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     */
    pub async fn get_all_participant_feedback(
        &self,
        meeting_id: &str,
        type_: crate::types::DashboardMeetingsType,
    ) -> Result<Vec<crate::types::ParticipantFeedbackResponseParticipants>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/metrics/meetings/{}/participants/satisfaction?{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            query_
        );

        let mut resp: crate::types::ParticipantFeedbackResponse =
            self.client.get(&url, None).await?;

        let mut participants = resp.participants;
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

            participants.append(&mut resp.participants);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(participants)
    }

    /**
     * Get post webinar feedback.
     *
     * This function performs a `GET` to the `/metrics/webinars/{webinarId}/participants/satisfaction` endpoint.
     *
     * When a Webinar ends, each attendee will be prompted to share their Webinar experience by clicking either thumbs up or thumbs down. Use this API to retrieve the feedback submitted for a specific webinar. Note that this API only works for meetings scheduled after December 20, 2020.
     *
     * **Prerequisites:**
     * * [Feedback to Zoom](https://support.zoom.us/hc/en-us/articles/115005838023) setting must be enabled by the participant prior to the meeting.
     * * The user making the API request must be enrolled in a Business or a higher plan.
     *
     *
     * <br> **Scope:** `dashboard_webinars:read:admin`
     *
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     *
     * **Parameters:**
     *
     * * `type_: crate::types::DashboardMeetingsType` -- Specify a value to get the response for the corresponding meeting type. The value of this field can be one of the following:<br> <br>`past` - Meeting that already occurred in the specified date range.<br>`pastOne` - Past meetings that were attended by only one user. <br>`live` - Live meetings.<br><br>
     *  
     *  If you do not provide this field, the default value will be `live` and thus, the API will only query responses for live meetings.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `webinar_id: &str` -- The webinar ID or the webinar UUID.  If a webinar ID is provided in the request instead of a UUID, the response will be for the latest webinar instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     */
    pub async fn participant_webinar_feedback(
        &self,
        type_: crate::types::DashboardMeetingsType,
        page_size: i64,
        next_page_token: &str,
        webinar_id: &str,
    ) -> Result<Vec<crate::types::ParticipantFeedbackResponseParticipants>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/metrics/webinars/{}/participants/satisfaction?{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
            query_
        );

        let resp: crate::types::ParticipantFeedbackResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.participants)
    }

    /**
     * Get post webinar feedback.
     *
     * This function performs a `GET` to the `/metrics/webinars/{webinarId}/participants/satisfaction` endpoint.
     *
     * As opposed to `participant_webinar_feedback`, this function returns all the pages of the request at once.
     *
     * When a Webinar ends, each attendee will be prompted to share their Webinar experience by clicking either thumbs up or thumbs down. Use this API to retrieve the feedback submitted for a specific webinar. Note that this API only works for meetings scheduled after December 20, 2020.
     *
     * **Prerequisites:**
     * * [Feedback to Zoom](https://support.zoom.us/hc/en-us/articles/115005838023) setting must be enabled by the participant prior to the meeting.
     * * The user making the API request must be enrolled in a Business or a higher plan.
     *
     *
     * <br> **Scope:** `dashboard_webinars:read:admin`
     *
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     */
    pub async fn get_all_participant_webinar_feedback(
        &self,
        type_: crate::types::DashboardMeetingsType,
        webinar_id: &str,
    ) -> Result<Vec<crate::types::ParticipantFeedbackResponseParticipants>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/metrics/webinars/{}/participants/satisfaction?{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
            query_
        );

        let mut resp: crate::types::ParticipantFeedbackResponse =
            self.client.get(&url, None).await?;

        let mut participants = resp.participants;
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

            participants.append(&mut resp.participants);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(participants)
    }
}
