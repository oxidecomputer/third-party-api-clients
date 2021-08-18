use anyhow::Result;

use crate::Client;

pub struct CloudRecording {
    pub client: Client,
}

impl CloudRecording {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CloudRecording { client }
    }

    /**
     * List all recordings.
     *
     * This function performs a `GET` to the `/users/{userId}/recordings` endpoint.
     *
     * Use this API to list all [cloud recordings](https://support.zoom.us/hc/en-us/articles/203741855-Cloud-Recording) of a user. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note:</b> To access a user's password protected cloud recording, add an <code>access_token</code> parameter to the download URL and provide either the <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-jwt-app">JWT</a> or the user's OAuth access token as the value of the <code>access_token</code> parameter.</p>
     *
     * When a user records a meeting or a webinar by choosing the **Record to the Cloud** option, the video, audio, and chat text are recorded in the Zoom cloud.
     *
     * **Scopes:** `recording:read:admin`, `recording:read`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites:**
     * * Pro or a higher plan.
     * * Cloud Recording must be enabled on the user's account.
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `mc: &str` -- Query Metadata of Recording if an On-Premise Meeting Connector was used for the meeting.
     * * `trash: bool` -- Enable/disable the option for a sub account to use shared [Virtual Room Connector(s)](https://support.zoom.us/hc/en-us/articles/202134758-Getting-Started-With-Virtual-Room-Connector) that are set up by the master account. Virtual Room Connectors can only be used by On-prem users.
     * * `from: chrono::NaiveDate` -- The start date in 'yyyy-mm-dd' UTC format for the date range for which you would like to retrieve recordings. The maximum range can be a month. If no value is provided for this field, the default will be current date. For example, if you make the API request on June 30, 2020, without providing the “from” and “to” parameters, by default the value of 'from' field will be “2020-06-30” and the value of the 'to' field will be “2020-07-01”.
     *   
     *   **Note**: The "trash" files cannot be filtered by date range and thus, the "from" and "to" fields should not be used for trash files.
     * * `to: chrono::NaiveDate` -- End date in 'yyyy-mm-dd' 'yyyy-mm-dd' UTC format. .
     * * `trash_type: &str` -- The type of Cloud recording that you would like to retrieve from the trash. The value can be one of the following:<br>
     *       `meeting_recordings`: List all meeting recordings from the trash.<br>
     *       `recording_file`: List all individual recording files from the trash. .
     */
    pub async fn recordings_list(
        &self,
        user_id: &str,
        page_size: i64,
        next_page_token: &str,
        mc: &str,
        trash: bool,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
        trash_type: &str,
    ) -> Result<crate::types::Domains> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !mc.is_empty() {
            query_args.push(("mc".to_string(), mc.to_string()));
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
        if trash {
            query_args.push(("trash".to_string(), trash.to_string()));
        }
        if !trash_type.is_empty() {
            query_args.push(("trash_type".to_string(), trash_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/users/{}/recordings?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get meeting recordings.
     *
     * This function performs a `GET` to the `/meetings/{meetingId}/recordings` endpoint.
     *
     * Get all the [recordings](https://support.zoom.us/hc/en-us/articles/203741855-Cloud-Recording#h_7420acb5-1897-4061-87b4-5b76e99c03b4) from a meeting or Webinar instance. The recording files can be downloaded via the `download_url` property listed in the response.
     *
     * > To access a password-protected cloud recording, add an `access_token` parameter to the download URL and provide OAuth access token or [JWT](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-jwt-app) as the `access_token` value.
     *
     * **Scopes:** `recording:read:admin`, `recording:read`</br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- To get Cloud Recordings of a meeting, provide the meeting ID or meeting UUID. If the meeting ID is provided instead of UUID,the response will be for the latest meeting instance.
     *   
     *   To get Cloud Recordings of a webinar, provide the webinar ID or the webinar UUID. If the webinar ID is provided instead of UUID,the response will be for the latest webinar instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875=="), you must **double encode** the UUID before making an API request. .
     * * `include_fields: &str` -- Get the `download_access_token` field for downloading meeting recordings.
     * * `ttl: u64` -- Time to live (TTL) of the `download_access_token`. This is only valid if the `include_fields` query parameter contains `download_access_token`. The range is between 0-604800.
     */
    pub async fn recording_get(
        &self,
        meeting_id: &str,
        include_fields: &str,
        ttl: u64,
    ) -> Result<crate::types::RecordingGetResponseAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include_fields.is_empty() {
            query_args.push(("include_fields".to_string(), include_fields.to_string()));
        }
        if !ttl.to_string().is_empty() {
            query_args.push(("ttl".to_string(), ttl.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/meetings/{}/recordings?{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete meeting recordings.
     *
     * This function performs a `DELETE` to the `/meetings/{meetingId}/recordings` endpoint.
     *
     * Delete all recording files of a meeting.<br><br>
     *
     * **Scopes:** `recording:write:admin` `recording:write`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     * **Prerequisites**:
     * * Cloud Recording should be enabled on the user's account.<br>
     *
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- To get Cloud Recordings of a meeting, provide the meeting ID or meeting UUID. If the meeting ID is provided instead of UUID,the response will be for the latest meeting instance.
     *   
     *   To get Cloud Recordings of a webinar, provide the webinar ID or the webinar UUID. If the webinar ID is provided instead of UUID,the response will be for the latest webinar instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875=="), you must **double encode** the UUID before making an API request. .
     * * `action: crate::types::RecordingDeleteAction` -- The recording delete actions:<br>`trash` - Move recording to trash.<br>`delete` - Delete recording permanently.
     */
    pub async fn recording_delete(
        &self,
        meeting_id: &str,
        action: crate::types::RecordingDeleteAction,
    ) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !action.to_string().is_empty() {
            query_args.push(("action".to_string(), action.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/meetings/{}/recordings?{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            query_
        );

        self.client.delete(&url, None).await
    }

    /**
     * Delete a meeting recording file.
     *
     * This function performs a `DELETE` to the `/meetings/{meetingId}/recordings/{recordingId}` endpoint.
     *
     * Delete a specific recording file from a meeting.<p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note:</b> To use this API, you must enable the <b>The host can delete cloud recordings</b> setting. You can find this setting in the <b>Recording</b> tab of the <b>Settings</b> interface in the [Zoom web portal](https://zoom.us/).</p>
     *
     * **Scopes**: `recording:write:admin`, `recording:write`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- To get Cloud Recordings of a meeting, provide the meeting ID or meeting UUID. If the meeting ID is provided instead of UUID,the response will be for the latest meeting instance.
     *   
     *   To get Cloud Recordings of a webinar, provide the webinar ID or the webinar UUID. If the webinar ID is provided instead of UUID,the response will be for the latest webinar instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875=="), you must **double encode** the UUID before making an API request. .
     * * `recording_id: &str` -- User's first name.
     * * `action: crate::types::RecordingDeleteAction` -- The recording delete actions:<br>`trash` - Move recording to trash.<br>`delete` - Delete recording permanently.
     */
    pub async fn recording_delete_one(
        &self,
        meeting_id: &str,
        recording_id: &str,
        action: crate::types::RecordingDeleteAction,
    ) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !action.to_string().is_empty() {
            query_args.push(("action".to_string(), action.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/meetings/{}/recordings/{}?{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            crate::progenitor_support::encode_path(&recording_id.to_string()),
            query_
        );

        self.client.delete(&url, None).await
    }

    /**
     * Recover meeting recordings.
     *
     * This function performs a `PUT` to the `/meetings/{meetingId}/recordings/status` endpoint.
     *
     * Zoom allows users to recover recordings from trash for up to 30 days from the deletion date. Use this API to recover all deleted [Cloud Recordings](https://support.zoom.us/hc/en-us/articles/203741855-Cloud-Recording) of a specific meeting.<br><br>
     * **Scopes**: `recording:write:admin` `recording:write`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     * **Prerequisites**:<br>
     * * A Pro user with Cloud Recording enabled.
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- To get Cloud Recordings of a meeting, provide the meeting ID or meeting UUID. If the meeting ID is provided instead of UUID,the response will be for the latest meeting instance.
     *   
     *   To get Cloud Recordings of a webinar, provide the webinar ID or the webinar UUID. If the webinar ID is provided instead of UUID,the response will be for the latest webinar instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875=="), you must **double encode** the UUID before making an API request. .
     */
    pub async fn recording_status_update(
        &self,
        meeting_id: &str,
        body: &crate::types::RecordingStatusUpdateBodyRequest,
    ) -> Result<()> {
        let url = format!(
            "/meetings/{}/recordings/status",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Recover a single recording.
     *
     * This function performs a `PUT` to the `/meetings/{meetingId}/recordings/{recordingId}/status` endpoint.
     *
     * Zoom allows users to recover recordings from trash for up to 30 days from the deletion date. Use this API to recover a single recording file from the meeting.<br>
     * **Scopes:** `recording:write:admin` `recording:write`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     *
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- To get Cloud Recordings of a meeting, provide the meeting ID or meeting UUID. If the meeting ID is provided instead of UUID,the response will be for the latest meeting instance.
     *   
     *   To get Cloud Recordings of a webinar, provide the webinar ID or the webinar UUID. If the webinar ID is provided instead of UUID,the response will be for the latest webinar instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875=="), you must **double encode** the UUID before making an API request. .
     * * `recording_id: &str` -- User's first name.
     */
    pub async fn recording_status_update_one(
        &self,
        meeting_id: &str,
        recording_id: &str,
        body: &crate::types::RecordingStatusUpdateBodyRequest,
    ) -> Result<()> {
        let url = format!(
            "/meetings/{}/recordings/{}/status",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            crate::progenitor_support::encode_path(&recording_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get meeting recording settings.
     *
     * This function performs a `GET` to the `/meetings/{meetingId}/recordings/settings` endpoint.
     *
     * Retrieve settings applied to a meeting's [Cloud Recording](https://support.zoom.us/hc/en-us/articles/203741855-Cloud-Recording).<br><br>
     * **Scopes**: `recording:read:admin` `recording:read`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light` <br>
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- To get Cloud Recordings of a meeting, provide the meeting ID or meeting UUID. If the meeting ID is provided instead of UUID,the response will be for the latest meeting instance.
     *   
     *   To get Cloud Recordings of a webinar, provide the webinar ID or the webinar UUID. If the webinar ID is provided instead of UUID,the response will be for the latest webinar instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875=="), you must **double encode** the UUID before making an API request. .
     */
    pub async fn recording_setting_update(
        &self,
        meeting_id: &str,
    ) -> Result<crate::types::RecordingSettings> {
        let url = format!(
            "/meetings/{}/recordings/settings",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update meeting recording settings.
     *
     * This function performs a `PATCH` to the `/meetings/{meetingId}/recordings/settings` endpoint.
     *
     * Update settings applied to a meeting's [Cloud Recording](https://support.zoom.us/hc/en-us/articles/203741855-Cloud-Recording)<br><br>
     * **Scopes**: `recording:write:admin` `recording:write`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light` <br>
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- To get Cloud Recordings of a meeting, provide the meeting ID or meeting UUID. If the meeting ID is provided instead of UUID,the response will be for the latest meeting instance.
     *   
     *   To get Cloud Recordings of a webinar, provide the webinar ID or the webinar UUID. If the webinar ID is provided instead of UUID,the response will be for the latest webinar instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875=="), you must **double encode** the UUID before making an API request. .
     */
    pub async fn recording_settings_update(
        &self,
        meeting_id: &str,
        body: &crate::types::RecordingSettings,
    ) -> Result<()> {
        let url = format!(
            "/meetings/{}/recordings/settings",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * List recording registrants.
     *
     * This function performs a `GET` to the `/meetings/{meetingId}/recordings/registrants` endpoint.
     *
     * Cloud Recordings of past Zoom Meetings can be made [on-demand](https://support.zoom.us/hc/en-us/articles/360000488283-On-demand-Recordings). Users should be [registered](https://marketplace.zoom.us/docs/api-reference/zoom-api/cloud-recording/meetingrecordingregistrantcreate) to view these recordings.
     *
     * Use this API to list registrants of **On-demand Cloud Recordings** of a past meeting.<br>
     * **Scopes:** `recording:read:admin`, `recording:read`.<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     * * `status: crate::types::MeetingRegistrantsStatus` -- The registrant status:<br>`pending` - Registrant's status is pending.<br>`approved` - Registrant's status is approved.<br>`denied` - Registrant's status is denied.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `page_number: i64` --
     *   **Deprecated** - This field has been deprecated and we will stop supporting it completely in a future release. Please use "next_page_token" for pagination instead of this field.
     *   
     *   The page number of the current page in the returned records.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn meeting_recording_registrant(
        &self,
        meeting_id: i64,
        status: crate::types::MeetingRegistrantsStatus,
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
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/meetings/{}/recordings/registrants?{}",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Create a recording registrant.
     *
     * This function performs a `POST` to the `/meetings/{meetingId}/recordings/registrants` endpoint.
     *
     * Cloud Recordings of past Zoom Meetings can be made [on-demand](https://support.zoom.us/hc/en-us/articles/360000488283-On-demand-Recordings). Users should be [registered](https://marketplace.zoom.us/docs/api-reference/zoom-api/cloud-recording/meetingrecordingregistrantcreate) to view these recordings.
     *
     * Use this API to register a user to gain access to **On-demand Cloud Recordings** of a past meeting.<br>
     * **Scopes:** `recording:write:admin`, `recording:write`.<br>
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
    pub async fn meeting_recording_registrant_create(
        &self,
        meeting_id: i64,
    ) -> Result<crate::types::MeetingRecordingRegistrantCreateResponse> {
        let url = format!(
            "/meetings/{}/recordings/registrants",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
     * Update registrant's status.
     *
     * This function performs a `PUT` to the `/meetings/{meetingId}/recordings/registrants/status` endpoint.
     *
     * A registrant can either be approved or denied from viewing the [on-demand](https://support.zoom.us/hc/en-us/articles/360000488283-On-demand-Recordings) recording.
     * Use this API to update a registrant's status.
     *
     * **Scopes:** `recording:write:admin`, `recording:write`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     */
    pub async fn meeting_recording_registrant_status(
        &self,
        meeting_id: i64,
        body: &crate::types::RecordingRegistrantStatus,
    ) -> Result<()> {
        let url = format!(
            "/meetings/{}/recordings/registrants/status",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get registration questions.
     *
     * This function performs a `GET` to the `/meetings/{meetingId}/recordings/registrants/questions` endpoint.
     *
     * For [on-demand](https://support.zoom.us/hc/en-us/articles/360000488283-On-demand-Recordings) meeting recordings, you can include fields with questions that will be shown to registrants when they register to view the recording.
     *
     * Use this API to retrieve a list of questions that are displayed for users to complete when registering to view the recording of a specific meeting.<br>
     * **Scopes:** `recording:read:admin`, `recording:read`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- To get Cloud Recordings of a meeting, provide the meeting ID or meeting UUID. If the meeting ID is provided instead of UUID,the response will be for the latest meeting instance.
     *   
     *   To get Cloud Recordings of a webinar, provide the webinar ID or the webinar UUID. If the webinar ID is provided instead of UUID,the response will be for the latest webinar instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875=="), you must **double encode** the UUID before making an API request. .
     */
    pub async fn recording_registrants_questions_get(
        &self,
        meeting_id: &str,
    ) -> Result<crate::types::RecordingRegistrantQuestionsData> {
        let url = format!(
            "/meetings/{}/recordings/registrants/questions",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update registration questions.
     *
     * This function performs a `PATCH` to the `/meetings/{meetingId}/recordings/registrants/questions` endpoint.
     *
     * For [on-demand](https://support.zoom.us/hc/en-us/articles/360000488283-On-demand-Recordings) meeting recordings, you can include fields with questions that will be shown to registrants when they register to view the recording.
     *
     * Use this API to update registration questions that are to be answered by users while registering to view a recording.<br>
     * **Scopes:** `recording:write:admin`, `recording:write`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     *
     *
     * **Parameters:**
     *
     * * `meeting_id: &str` -- To get Cloud Recordings of a meeting, provide the meeting ID or meeting UUID. If the meeting ID is provided instead of UUID,the response will be for the latest meeting instance.
     *   
     *   To get Cloud Recordings of a webinar, provide the webinar ID or the webinar UUID. If the webinar ID is provided instead of UUID,the response will be for the latest webinar instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875=="), you must **double encode** the UUID before making an API request. .
     */
    pub async fn recording_registrant_question_update(
        &self,
        meeting_id: &str,
        body: &crate::types::RecordingRegistrantQuestionsData,
    ) -> Result<()> {
        let url = format!(
            "/meetings/{}/recordings/registrants/questions",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * List recordings of an account.
     *
     * This function performs a `GET` to the `/accounts/{accountId}/recordings` endpoint.
     *
     * List [Cloud Recordings](https://support.zoom.us/hc/en-us/articles/203741855-Cloud-Recording) available on an Account.
     *
     * > To access a password protected cloud recording, add an "access_token" parameter to the download URL and provide [JWT](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-jwt-app) as the value of the "access_token".
     * <br>
     * **Prerequisites**:<br>
     * * A Pro or a higher paid plan with Cloud Recording option enabled.<br>
     * **Scopes**: `recording:read:admin` or `account:read:admin`
     *
     * If the scope `recording:read:admin` is used, the Account ID of the Account must be provided in the `accountId` path parameter to list recordings that belong to the Account. This scope only works for sub accounts.
     *
     * To list recordings of a master account, the scope must be `account:read:admin` and the value of `accountId` should be `me`.<br>  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     *
     *
     * **Parameters:**
     *
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `from: chrono::DateTime<chrono::Utc>` -- The start date in UTC for the monthly range for which you would like to retrieve recordings. The maximum range can be a month. If no value is provided for this field, the default will be current date. For example, if you make the API request on June 30, 2020, without providing the “from” and “to” parameters, by default the value of 'from' field will be “2020-06-30” and the value of the 'to' field will be “2020-07-01”.
     * * `to: chrono::DateTime<chrono::Utc>` -- The end date for the monthly range for which you would like to retrieve recordings. The maximum range can be a month.
     * * `account_id: &str` -- Unique identifier of the account.
     */
    pub async fn get_account(
        &self,
        account_id: &str,
        page_size: i64,
        next_page_token: &str,
        from: Option<chrono::DateTime<chrono::Utc>>,
        to: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<crate::types::GetAccountCloudRecordingResponseMeetings>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if let Some(date) = from {
            query_args.push(("from".to_string(), date.to_rfc3339()));
        }
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if let Some(date) = to {
            query_args.push(("to".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/accounts/{}/recordings?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            query_
        );

        let resp: crate::types::GetAccountCloudRecordingResponse =
            self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.meetings)
    }

    /**
     * List recordings of an account.
     *
     * This function performs a `GET` to the `/accounts/{accountId}/recordings` endpoint.
     *
     * As opposed to `get_account`, this function returns all the pages of the request at once.
     *
     * List [Cloud Recordings](https://support.zoom.us/hc/en-us/articles/203741855-Cloud-Recording) available on an Account.
     *
     * > To access a password protected cloud recording, add an "access_token" parameter to the download URL and provide [JWT](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-jwt-app) as the value of the "access_token".
     * <br>
     * **Prerequisites**:<br>
     * * A Pro or a higher paid plan with Cloud Recording option enabled.<br>
     * **Scopes**: `recording:read:admin` or `account:read:admin`
     *
     * If the scope `recording:read:admin` is used, the Account ID of the Account must be provided in the `accountId` path parameter to list recordings that belong to the Account. This scope only works for sub accounts.
     *
     * To list recordings of a master account, the scope must be `account:read:admin` and the value of `accountId` should be `me`.<br>  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     *
     */
    pub async fn get_all_account(
        &self,
        account_id: &str,
        from: Option<chrono::DateTime<chrono::Utc>>,
        to: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<crate::types::GetAccountCloudRecordingResponseMeetings>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if let Some(date) = from {
            query_args.push(("from".to_string(), date.to_rfc3339()));
        }
        if let Some(date) = to {
            query_args.push(("to".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/accounts/{}/recordings?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            query_
        );

        let mut resp: crate::types::GetAccountCloudRecordingResponse =
            self.client.get(&url, None).await?;

        let mut meetings = resp.meetings;
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

            meetings.append(&mut resp.meetings);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(meetings)
    }
}
