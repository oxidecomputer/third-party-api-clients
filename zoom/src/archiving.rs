use anyhow::Result;

use crate::Client;

pub struct Archiving {
    pub client: Client,
}

impl Archiving {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Archiving { client }
    }

    /**
     * List archived files.
     *
     * This function performs a `GET` to the `/archive_files` endpoint.
     *
     * Zoom’s [archiving solution](https://support.zoom.us/hc/en-us/articles/360050431572-Archiving-Meeting-and-Webinar-data) allows account administrators to set up an automated mechanism to record, collect and archive meeting data to a 3rd party platform of their choice and hence, satisfy FINRA and/ or other compliance requirements.<br><br>
     * Use this API to retrieve archived meeting or webinar files of an account.
     *
     * **Scope:** `recording:read:admin`<br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br><br>
     * **Prerequisites:** <br>
     * * Enable cloud recording.
     * * Follow the [enablement process](https://support.zoom.us/hc/en-us/articles/360050431572-Archiving-Meeting-and-Webinar-data#h_01ENPBD3WR68D7FAKTBY92SG45) to access the archiving feature.
     *
     * **Parameters:**
     *
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `from: &str` -- Start date for the query in "yyyy-MM-dd'T'HH:mm:ss'Z'" format. The duration for the query defined using the "from" and "to" parameters should not exceed 7 days as this API only provides a week's data at once.
     * * `to: &str` -- End date for the query in "yyyy-MM-dd'T'HH:mm:ss'Z'" format. .
     * * `query_date_type: crate::types::ListArchivedFilesQueryDateType` -- The query date type for the `from` and `to` parameters.
     */
    pub async fn list_archived_files(
        &self,
        page_size: i64,
        next_page_token: &str,
        from: &str,
        to: &str,
        query_date_type: crate::types::ListArchivedFilesQueryDateType,
    ) -> Result<Vec<crate::types::ListArchivedFilesResponseMeetings>> {
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
        if !query_date_type.to_string().is_empty() {
            query_args.push(("query_date_type".to_string(), query_date_type.to_string()));
        }
        if !to.is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/archive_files?{}", query_);

        let resp: crate::types::ListArchivedFilesResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.meetings)
    }

    /**
     * List archived files.
     *
     * This function performs a `GET` to the `/archive_files` endpoint.
     *
     * As opposed to `list_archived_files`, this function returns all the pages of the request at once.
     *
     * Zoom’s [archiving solution](https://support.zoom.us/hc/en-us/articles/360050431572-Archiving-Meeting-and-Webinar-data) allows account administrators to set up an automated mechanism to record, collect and archive meeting data to a 3rd party platform of their choice and hence, satisfy FINRA and/ or other compliance requirements.<br><br>
     * Use this API to retrieve archived meeting or webinar files of an account.
     *
     * **Scope:** `recording:read:admin`<br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br><br>
     * **Prerequisites:** <br>
     * * Enable cloud recording.
     * * Follow the [enablement process](https://support.zoom.us/hc/en-us/articles/360050431572-Archiving-Meeting-and-Webinar-data#h_01ENPBD3WR68D7FAKTBY92SG45) to access the archiving feature.
     */
    pub async fn list_all_archived_files(
        &self,
        from: &str,
        to: &str,
        query_date_type: crate::types::ListArchivedFilesQueryDateType,
    ) -> Result<Vec<crate::types::ListArchivedFilesResponseMeetings>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !query_date_type.to_string().is_empty() {
            query_args.push(("query_date_type".to_string(), query_date_type.to_string()));
        }
        if !to.is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/archive_files?{}", query_);

        let mut resp: crate::types::ListArchivedFilesResponse = self.client.get(&url, None).await?;

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

    /**
     * Get meeting archived files.
     *
     * This function performs a `GET` to the `/past_meetings/{meetingUUID}/archive_files` endpoint.
     *
     * List the archived recording files of the specific meeting instance. For more information, read our [Managing archiving of meeting and webinar data](https://support.zoom.us/hc/en-us/articles/360050431572-Archiving-Meeting-and-Webinar-data) documentation.
     *
     * **Scopes:** `recording:read``24` — Apple OAuth</br>`27` — Microsoft OAuth</br>`97` — Mobile device</br>`98` — RingCentral OAuth</br>`99` — API user</br>`100` — Zoom Work email</br>`101` — Single Sign-On (SSO)
     *
     * The following login methods are only available in China:
     *
     * `11` — Phone number</br>`21` — WeChat</br>`23` — Alipay
     *
     * **Parameters:**
     *
     * * `meeting: &str` -- The meeting's universally unique identifier (UUID). Each meeting instance generates a UUID. For example, after a meeting ends, a new UUID is generated for the next meeting instance.
     *   
     *   If the meeting UUID begins with a `/` character or contains a `//` character, you **must** double-encode the meeting UUID when using the meeting UUID for other API calls.
     */
    pub async fn testget_record_archived_file(
        &self,
        meeting_uuid: &str,
    ) -> Result<crate::types::CloudArchivedFiles> {
        let url = format!(
            "/past_meetings/{}/archive_files",
            crate::progenitor_support::encode_path(&meeting_uuid.to_string()),
        );

        self.client.get(&url, None).await
    }
}
