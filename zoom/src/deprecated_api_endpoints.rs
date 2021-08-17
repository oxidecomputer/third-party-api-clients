use anyhow::Result;

use crate::Client;

pub struct DeprecatedApiEndpoints {
    pub client: Client,
}

impl DeprecatedApiEndpoints {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        DeprecatedApiEndpoints { client }
    }

    /**
     * List past meeting's files.
     *
     * This function performs a `GET` to the `/past_meetings/{meetingId}/files` endpoint.
     *
     * **Note: This API has been deprecated and is no longer supported due to GCM encryption updates for security purposes.** To learn about saving the in-meeting chat files via Zoom Client, refer to the [Saving in-meeting chat](https://support.zoom.us/hc/en-us/articles/115004792763-Saving-in-meeting-chat) guide.
     *
     * List files sent via in-meeting chat during a meeting. The in-meeting files are deleted after 24 hours of the meeting completion time.
     * <br><br>
     * **Scope:** `meeting:read`, `meeting:read:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `meeting_id: i64` -- The meeting ID in **long** format. The data type of this field is "long"(represented as int64 in JSON).
     *   
     *   While storing it in your database, store it as a **long** data type and **not as an integer**, as the Meeting IDs can be longer than 10 digits.
     */
    pub async fn list_past_meeting_files(
        &self,
        meeting_id: &str,
    ) -> Result<crate::types::ListPastMeetingFilesResponse> {
        let url = format!(
            "/past_meetings/{}/files",
            crate::progenitor_support::encode_path(&meeting_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * List past webinar files.
     *
     * This function performs a `GET` to the `/past_webinars/{webinarId}/files` endpoint.
     *
     * **Note: This API has been deprecated and is no longer supported due to GCM encryption updates for security purposes.**
     *
     * List files sent via in-meeting chat during a meeting. The in-meeting files are deleted after 24 hours of the meeting completion time.
     * <br><br>
     * **Scope:** `webinar:read`, `webinar:read:admin`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     *
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: &str` -- User's first name.
     */
    pub async fn list_past_webinar_files(
        &self,
        webinar_id: &str,
    ) -> Result<crate::types::ListPastMeetingFilesResponse> {
        let url = format!(
            "/past_webinars/{}/files",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        self.client.get(&url, None).await
    }
}
