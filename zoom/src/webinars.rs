use anyhow::Result;

use crate::Client;

pub struct Webinars {
    pub client: Client,
}

impl Webinars {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Webinars { client }
    }

    /**
     * List webinars.
     *
     * This function performs a `GET` to the `/users/{userId}/webinars` endpoint.
     *
     * Use this API to list all the webinars that are scheduled by or on-behalf a user (webinar host). For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * Zoom users with a [Webinar Plan](https://zoom.us/webinar) have access to creating and managing webinars. Webinars allow a host to broadcast a Zoom meeting to up to 10,000 attendees.
     *
     * **Scopes:** `webinar:read:admin`, `webinar:read`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites:**
     * * Pro or higher plan with a Webinar Add-on.
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `page_number: i64` --
     *   **Deprecated** - This field has been deprecated and we will stop supporting it completely in a future release. Please use "next_page_token" for pagination instead of this field.
     *   
     *   The page number of the current page in the returned records.
     */
    pub async fn get(
        &self,
        user_id: &str,
        page_size: i64,
        page_number: i64,
    ) -> Result<crate::types::Domains> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page_number > 0 {
            query_args.push(("page_number".to_string(), page_number.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/users/{}/webinars?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Create a webinar.
     *
     * This function performs a `POST` to the `/users/{userId}/webinars` endpoint.
     *
     * Use this API to schedule a webinar for a user (host). For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * Zoom users with a [Webinar Plan](https://zoom.us/webinar) have access to creating and managing webinars. Webinars allow a host to broadcast a Zoom meeting to up to 10,000 attendees.
     *
     * **Scopes:** `webinar:write:admin`, `webinar:write`</br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Pro or higher plan with a Webinar add-on.
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     */
    pub async fn create(&self, user_id: &str) -> Result<crate::types::WebinarCreateResponseAllOf> {
        let url = format!(
            "/users/{}/webinars",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
     * Get a webinar.
     *
     * This function performs a `GET` to the `/webinars/{webinarId}` endpoint.
     *
     * Zoom users with a [Webinar Plan](https://zoom.us/webinar) have access to creating and managing Webinars. Webinar allows a host to broadcast a Zoom meeting to up to 10,000 attendees.<br>Use this API to get details of a scheduled webinar.<br><br>
     * **Scopes:** `webinar:read:admin` `webinar:read`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>**Prerequisites:**
     * * Pro or higher plan with a Webinar Add-on.
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     * * `occurrence_id: &str` -- Unique Identifier that identifies an occurrence of a recurring webinar. [Recurring webinars](https://support.zoom.us/hc/en-us/articles/216354763-How-to-Schedule-A-Recurring-Webinar) can have a maximum of 50 occurrences. When you create a recurring Webinar using [Create a Webinar API](https://marketplace.zoom.us/docs/api-reference/zoom-api/webinars/webinarcreate), you can retrieve the Occurrence ID from the response of the API call.
     * * `show_previous_occurrences: bool` -- Enable/disable the option for a sub account to use shared [Virtual Room Connector(s)](https://support.zoom.us/hc/en-us/articles/202134758-Getting-Started-With-Virtual-Room-Connector) that are set up by the master account. Virtual Room Connectors can only be used by On-prem users.
     */
    pub async fn webinar(
        &self,
        webinar_id: i64,
        occurrence_id: &str,
        show_previous_occurrences: bool,
    ) -> Result<crate::types::WebinarResponseAllOf> {
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
            "/webinars/{}?{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a webinar.
     *
     * This function performs a `DELETE` to the `/webinars/{webinarId}` endpoint.
     *
     * Delete a Webinar.<br><br>
     * **Scopes:** `webinar:write:admin` `webinar:write`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     * **Prerequisites:**<br>
     * * Pro or higher plan with a Webinar Add-on.
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     * * `occurrence_id: &str` -- The meeting occurrence ID.
     * * `cancel_webinar_reminder: &str` -- `true`: Notify panelists and registrants about the webinar cancellation via email.
     *   
     *   `false`: Do not send any email notification to webinar registrants and panelists.
     *   
     *   The default value of this field is `false`.
     */
    pub async fn delete(
        &self,
        webinar_id: i64,
        occurrence_id: &str,
        cancel_webinar_reminder: &str,
    ) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cancel_webinar_reminder.is_empty() {
            query_args.push((
                "cancel_webinar_reminder".to_string(),
                cancel_webinar_reminder.to_string(),
            ));
        }
        if !occurrence_id.is_empty() {
            query_args.push(("occurrence_id".to_string(), occurrence_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/webinars/{}?{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
            query_
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update a webinar.
     *
     * This function performs a `PATCH` to the `/webinars/{webinarId}` endpoint.
     *
     * Zoom users with a [Webinar Plan](https://zoom.us/webinar) have access to creating and managing Webinars. Webinar allows a host to broadcast a Zoom meeting to up to 10,000 attendees.<br>
     * Use this API to make updates to a scheduled Webinar.<br><br>
     * **Scopes:** `webinar:write:admin` `webinar:write`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     * **Prerequisites:**<br>
     * * Pro or higher plan with a Webinar Add-on.
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     * * `occurrence_id: &str` -- Webinar occurrence id. Support change of agenda, start_time, duration, settings: {host_video, panelist_video, hd_video, watermark, auto_recording}.
     */
    pub async fn update(&self, webinar_id: i64, occurrence_id: &str) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !occurrence_id.is_empty() {
            query_args.push(("occurrence_id".to_string(), occurrence_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/webinars/{}?{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
            query_
        );

        self.client.patch(&url, None).await
    }

    /**
     * List webinar participants.
     *
     * This function performs a `GET` to the `/past_webinars/{webinarId}/participants` endpoint.
     *
     * Use this API to list all the participants who attended a webinar hosted in the past. <br>
     *
     * **Prerequisites:**
     * * Pro or higher plan with a Webinar Add-on.<br>
     * **Scopes:** `webinar:read:admin` `webinar:read`<br>
     * <br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     *
     *
     *
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: &str` -- Unique identifier of the webinar. You can retrieve the value of this field by calling the [list webinars](https://marketplace.zoom.us/docs/api-reference/zoom-api/webinars/webinars) API.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn list_participants(
        &self,
        webinar_id: &str,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<Vec<crate::types::ListWebinarParticipantsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/past_webinars/{}/participants?{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
            query_
        );

        let resp: crate::types::ListWebinarParticipantsResponseData =
            self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.participants)
    }

    /**
     * List webinar participants.
     *
     * This function performs a `GET` to the `/past_webinars/{webinarId}/participants` endpoint.
     *
     * As opposed to `list_participants`, this function returns all the pages of the request at once.
     *
     * Use this API to list all the participants who attended a webinar hosted in the past. <br>
     *
     * **Prerequisites:**
     * * Pro or higher plan with a Webinar Add-on.<br>
     * **Scopes:** `webinar:read:admin` `webinar:read`<br>
     * <br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     *
     *
     *
     *
     */
    pub async fn list_all_participants(
        &self,
        webinar_id: &str,
    ) -> Result<Vec<crate::types::ListWebinarParticipantsResponse>> {
        let url = format!(
            "/past_webinars/{}/participants",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        let mut resp: crate::types::ListWebinarParticipantsResponseData =
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
     * Update webinar status.
     *
     * This function performs a `PUT` to the `/webinars/{webinarId}/status` endpoint.
     *
     * Update a webinar's status. Use this API to end an ongoing webinar.<br><br>
     * **Scopes:** `webinar:write:admin` `webinar:write`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     * **Prerequisites:**<br>
     * * The account must hold a valid [Webinar plan](https://zoom.us/webinar).
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     */
    pub async fn status(
        &self,
        webinar_id: i64,
        body: &crate::types::WebinarStatusRequest,
    ) -> Result<()> {
        let url = format!(
            "/webinars/{}/status",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * List panelists.
     *
     * This function performs a `GET` to the `/webinars/{webinarId}/panelists` endpoint.
     *
     * Panelists in a Webinar can view and send video, screen share, annotate, etc and do much more compared to attendees in a Webinar.
     *
     * Use this API to list all the panelists of a Webinar.<br><br>
     * **Scopes:** `webinar:read:admin` `webinar:read`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     * **Prerequisites:**<br>
     * * Pro or a higher plan with [Webinar Add-on](https://zoom.us/webinar).<br>
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     */
    pub async fn panelist(&self, webinar_id: i64) -> Result<crate::types::Domains> {
        let url = format!(
            "/webinars/{}/panelists",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Add panelists.
     *
     * This function performs a `POST` to the `/webinars/{webinarId}/panelists` endpoint.
     *
     * Panelists in a Webinar can view and send video, screen share, annotate, etc and do much more compared to attendees in a webinar.<br>Use this API to [add panelists](https://support.zoom.us/hc/en-us/articles/115005657826-Inviting-Panelists-to-a-Webinar#h_7550d59e-23f5-4703-9e22-e76bded1ed70) to a scheduled webinar.<br><br>
     * **Scopes:** `webinar:write:admin` `webinar:write`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     *
     *
     * **Prerequisites:**
     * * Pro or a higher plan with [Webinar Add-on](https://zoom.us/webinar).<br>
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     */
    pub async fn panelist_create(
        &self,
        webinar_id: i64,
        body: &crate::types::WebinarPanelist,
    ) -> Result<()> {
        let url = format!(
            "/webinars/{}/panelists",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Remove panelists.
     *
     * This function performs a `DELETE` to the `/webinars/{webinarId}/panelists` endpoint.
     *
     * Remove all the panelists from a Webinar.<br>
     * **Scopes:** `webinar:write:admin` `webinar:write`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     * **Prerequisites:**<br>
     * * Pro or a higher plan with [Webinar Add-on](https://zoom.us/webinar).<br>
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     */
    pub async fn panelists_delete(&self, webinar_id: i64) -> Result<()> {
        let url = format!(
            "/webinars/{}/panelists",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Remove a panelist.
     *
     * This function performs a `DELETE` to the `/webinars/{webinarId}/panelists/{panelistId}` endpoint.
     *
     * [Remove](https://support.zoom.us/hc/en-us/articles/115005657826-Inviting-Panelists-to-a-Webinar#h_de31f237-a91c-4fb2-912b-ecfba8ec5ffb) a single panelist from a webinar.<br> You can retrieve the `panelistId` by calling **List Panelists API**.<br><br>
     * **Scopes:** `webinar:write:admin` `webinar:write`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     *
     *
     * **Prerequisites:**<br>
     * * Pro or a higher plan with [Webinar Add-on](https://zoom.us/webinar).<br>
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     * * `panelist_id: i64` -- The panelist ID or panelist email.
     */
    pub async fn panelist_delete(&self, webinar_id: i64, panelist_id: i64) -> Result<()> {
        let url = format!(
            "/webinars/{}/panelists/{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
            crate::progenitor_support::encode_path(&panelist_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * List webinar registrants.
     *
     * This function performs a `GET` to the `/webinars/{webinarId}/registrants` endpoint.
     *
     * Zoom users with a [Webinar Plan](https://zoom.us/webinar) have access to creating and managing Webinars. Webinar allows a host to broadcast a Zoom meeting to up to 10,000 attendees. Scheduling a [Webinar with registration](https://support.zoom.us/hc/en-us/articles/204619915-Scheduling-a-Webinar-with-Registration) requires your registrants to complete a brief form before receiving the link to join the Webinar.<br>
     * Use this API to list all the users that have registered for a webinar.<br><br>
     * **Prerequisites:**
     * * Pro or higher plan with a Webinar Add-on.<br>
     * **Scopes:** `webinar:read:admin` `webinar:read`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     *
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     * * `occurrence_id: &str` -- The meeting occurrence ID.
     * * `status: crate::types::MeetingRegistrantsStatus` -- The registrant status:<br>`pending` - Registrant's status is pending.<br>`approved` - Registrant's status is approved.<br>`denied` - Registrant's status is denied.
     * * `tracking_source_id: &str` -- The tracking source ID for the registrants. Useful if you share the webinar registration page in multiple locations. See [Creating source tracking links for webinar registration](https://support.zoom.us/hc/en-us/articles/360000315683-Creating-source-tracking-links-for-webinar-registration) for details.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `page_number: i64` --
     *   **Deprecated** - This field has been deprecated and we will stop supporting it completely in a future release. Please use "next_page_token" for pagination instead of this field.
     *   
     *   The page number of the current page in the returned records.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn registrant(
        &self,
        webinar_id: &str,
        occurrence_id: &str,
        status: crate::types::MeetingRegistrantsStatus,
        tracking_source_id: &str,
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
        if !tracking_source_id.is_empty() {
            query_args.push((
                "tracking_source_id".to_string(),
                tracking_source_id.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/webinars/{}/registrants?{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Add a webinar registrant.
     *
     * This function performs a `POST` to the `/webinars/{webinarId}/registrants` endpoint.
     *
     * Zoom users with a [Webinar Plan](https://zoom.us/webinar) have access to creating and managing Webinars. Webinar allows a host to broadcast a Zoom meeting to up to 10,000 attendees. Scheduling a [Webinar with registration](https://support.zoom.us/hc/en-us/articles/204619915-Scheduling-a-Webinar-with-Registration) requires your registrants to complete a brief form before receiving the link to join the Webinar.<br>Use this API to create and submit the registration of a user for a webinar.<br><br>
     * **Scopes:** `webinar:write:admin` `webinar:write`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     * **Prerequisites:**
     * * Pro or higher plan with a Webinar Add-on.
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     * * `occurrence_ids: &str` -- Occurrence ID. Get this value from the webinar get API. Multiple values separated by a comma.
     */
    pub async fn registrant_create(
        &self,
        webinar_id: &str,
        occurrence_ids: &str,
    ) -> Result<crate::types::WebinarRegistrantCreateResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !occurrence_ids.is_empty() {
            query_args.push(("occurrence_ids".to_string(), occurrence_ids.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/webinars/{}/registrants?{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
            query_
        );

        self.client.post(&url, None).await
    }

    /**
     * Perform batch registration.
     *
     * This function performs a `POST` to the `/webinars/{webinarId}/batch_registrants` endpoint.
     *
     * Use this API to register up to 30 registrants at once for a scheduled webinar that requires [registration](https://support.zoom.us/hc/en-us/articles/204619915-Scheduling-a-webinar-with-registration). <br>
     *
     * **Prerequisites:**<br>
     * * The webinar host must be a Licensed user.
     * * The webinar should be of type `5`, i.e., it should be a scheduled webinar. Other types of webinars are not supported by this API.<br><br>
     * **Scope:** `webinar:write`, `webinar:write:admin`<br>
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
     * * `webinar_id: &str` -- Unique identifier of the webinar.
     */
    pub async fn add_batch_registrants(
        &self,
        webinar_id: &str,
        body: &crate::types::AddBatchRegistrantsRequest,
    ) -> Result<crate::types::AddBatchRegistrantsResponse> {
        let url = format!(
            "/webinars/{}/batch_registrants",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Update registrant's status.
     *
     * This function performs a `PUT` to the `/webinars/{webinarId}/registrants/status` endpoint.
     *
     * Update a webinar registrant's status. Using this API, you can specify whether you want to approve a registration, deny a registration or cancel a previously approved registration.<br><br>
     * **Scopes:** `webinar:write:admin` `webinar:write`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     *
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     * * `occurrence_id: &str` -- The meeting occurrence ID.
     */
    pub async fn registrant_status(
        &self,
        webinar_id: i64,
        occurrence_id: &str,
        body: &crate::types::RegistrantStatus,
    ) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !occurrence_id.is_empty() {
            query_args.push(("occurrence_id".to_string(), occurrence_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/webinars/{}/registrants/status?{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
            query_
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * List past webinar instances.
     *
     * This function performs a `GET` to the `/past_webinars/{webinarId}/instances` endpoint.
     *
     * List past webinar instances.<br><br>
     * **Scopes:** `webinar:read:admin` `webinar:read`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     *
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     */
    pub async fn past(&self, webinar_id: i64) -> Result<crate::types::Domains> {
        let url = format!(
            "/past_webinars/{}/instances",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * List a webinar's polls .
     *
     * This function performs a `GET` to the `/webinars/{webinarId}/polls` endpoint.
     *
     * List all the [polls](https://support.zoom.us/hc/en-us/articles/203749865-Polling-for-Webinars) of a Webinar.<br><br>
     * **Scopes:** `webinar:read:admin` `webinar:read`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     *
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     */
    pub async fn poll(&self, webinar_id: i64) -> Result<crate::types::Domains> {
        let url = format!(
            "/webinars/{}/polls",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Create a webinar's poll.
     *
     * This function performs a `POST` to the `/webinars/{webinarId}/polls` endpoint.
     *
     * Create a [poll](https://support.zoom.us/hc/en-us/articles/203749865-Polling-for-Webinars) for a webinar.<br><br>
     * **Scopes:** `webinar:write:admin` `webinar:write`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     *
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     */
    pub async fn poll_create(
        &self,
        webinar_id: i64,
        body: &crate::types::Poll,
    ) -> Result<crate::types::MeetingPollGetResponseAllOf> {
        let url = format!(
            "/webinars/{}/polls",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get a webinar poll.
     *
     * This function performs a `GET` to the `/webinars/{webinarId}/polls/{pollId}` endpoint.
     *
     * Get a webinar's [poll](https://support.zoom.us/hc/en-us/articles/203749865-Polling-for-Webinars) details.<br><br>
     * **Scopes:** `webinar:read:admin` `webinar:read`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     *
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     * * `poll_id: &str` -- User's first name.
     */
    pub async fn poll_get(
        &self,
        webinar_id: i64,
        poll_id: &str,
    ) -> Result<crate::types::MeetingPollGetResponseAllOf> {
        let url = format!(
            "/webinars/{}/polls/{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
            crate::progenitor_support::encode_path(&poll_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update a webinar poll.
     *
     * This function performs a `PUT` to the `/webinars/{webinarId}/polls/{pollId}` endpoint.
     *
     * Update a webinar's [poll](https://support.zoom.us/hc/en-us/articles/203749865-Polling-for-Webinars).<br><br>
     * **Scopes:** `webinar:write:admin` `webinar:write`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     *
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     * * `poll_id: &str` -- User's first name.
     */
    pub async fn poll_update(
        &self,
        webinar_id: i64,
        poll_id: &str,
        body: &crate::types::Poll,
    ) -> Result<()> {
        let url = format!(
            "/webinars/{}/polls/{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
            crate::progenitor_support::encode_path(&poll_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete a webinar poll.
     *
     * This function performs a `DELETE` to the `/webinars/{webinarId}/polls/{pollId}` endpoint.
     *
     * Delete a webinar's [poll](https://support.zoom.us/hc/en-us/articles/203749865-Polling-for-Webinars).<br><br>
     * **Scopes:** `webinar:write:admin` `webinar:write`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     *
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     * * `poll_id: &str` -- User's first name.
     */
    pub async fn poll_delete(&self, webinar_id: i64, poll_id: &str) -> Result<()> {
        let url = format!(
            "/webinars/{}/polls/{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
            crate::progenitor_support::encode_path(&poll_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * List registration questions.
     *
     * This function performs a `GET` to the `/webinars/{webinarId}/registrants/questions` endpoint.
     *
     * Scheduling a [Webinar with registration](https://support.zoom.us/hc/en-us/articles/204619915-Scheduling-a-Webinar-with-Registration) requires your registrants to complete a brief form with fields and questions before they can receive the link to join the Webinar.<br>Use this API to list registration questions and fields that are to be answered by users while registering for a Webinar.<br>
     * **Prerequisites:**<br>  
     * * Pro or higher plan with a Webinar Add-on.
     * **Scopes:** `webinar:read:admin` `webinar:read`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     *
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     */
    pub async fn registrants_questions_get(
        &self,
        webinar_id: i64,
    ) -> Result<crate::types::WebinarRegistrantQuestions> {
        let url = format!(
            "/webinars/{}/registrants/questions",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update registration questions.
     *
     * This function performs a `PATCH` to the `/webinars/{webinarId}/registrants/questions` endpoint.
     *
     * Scheduling a [Webinar with registration](https://support.zoom.us/hc/en-us/articles/204619915-Scheduling-a-Webinar-with-Registration) requires your registrants to complete a brief form with fields and questions before they can receive the link to join the Webinar.<br>Use this API to update registration questions and fields of a scheduled Webinar that are to be answered by users while registering for a Webinar.<br><br>
     * **Prerequisites:**<br>  
     * * Pro or higher plan with a Webinar Add-on.
     * * Registration option for Webinar should be set as required to use this API.
     * **Scopes:** `webinar:write:admin` `webinar:write`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     */
    pub async fn registrant_question_update(
        &self,
        webinar_id: i64,
        body: &crate::types::WebinarRegistrantQuestions,
    ) -> Result<()> {
        let url = format!(
            "/webinars/{}/registrants/questions",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get a webinar registrant.
     *
     * This function performs a `GET` to the `/webinars/{webinarId}/registrants/{registrantId}` endpoint.
     *
     * Zoom users with a [Webinar Plan](https://zoom.us/webinar) have access to creating and managing Webinars. Webinar allows a host to broadcast a Zoom meeting to up to 10,000 attendees. Scheduling a [Webinar with registration](https://support.zoom.us/hc/en-us/articles/204619915-Scheduling-a-Webinar-with-Registration) requires your registrants to complete a brief form before receiving the link to join the Webinar.<br>Use this API to get details on a specific user who has registered for the Webinar.<br><br>
     * **Scopes:** `webinar:read:admin` `webinar:read`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     * **Prerequisites:**<br>
     * * The account must have a Webinar plan.
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     * * `registrant_id: &str` -- User's first name.
     * * `occurrence_id: &str` -- The meeting occurrence ID.
     */
    pub async fn registrant_get(
        &self,
        webinar_id: &str,
        registrant_id: &str,
        occurrence_id: &str,
    ) -> Result<crate::types::Domains> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !occurrence_id.is_empty() {
            query_args.push(("occurrence_id".to_string(), occurrence_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/webinars/{}/registrants/{}?{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
            crate::progenitor_support::encode_path(&registrant_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a webinar registrant.
     *
     * This function performs a `DELETE` to the `/webinars/{webinarId}/registrants/{registrantId}` endpoint.
     *
     * Delete a webinar registrant.<br><br>
     * **Scopes**: `webinar:write:admin` `webinar:write`<br>
     *  <br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- Account seats.
     * * `registrant_id: &str` -- User's first name.
     * * `occurrence_id: &str` -- The webinar occurence ID.
     */
    pub async fn delete_registrant(
        &self,
        webinar_id: &str,
        registrant_id: &str,
        occurrence_id: &str,
    ) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !occurrence_id.is_empty() {
            query_args.push(("occurrence_id".to_string(), occurrence_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/webinars/{}/registrants/{}?{}",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
            crate::progenitor_support::encode_path(&registrant_id.to_string()),
            query_
        );

        self.client.delete(&url, None).await
    }

    /**
     * Get webinar absentees.
     *
     * This function performs a `GET` to the `/past_webinars/{WebinarUUID}/absentees` endpoint.
     *
     * List absentees of a webinar.<br><br>
     * **Scopes:** `webinar:read:admin` `webinar:read`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     *
     * **Parameters:**
     *
     * * `occurrence_id: &str` -- The meeting occurrence ID.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `webinar: &str` -- The Webinar UUID. Each Webinar instance will generate its own Webinar UUID (i.e., after a Webinar ends, a new UUID will be generated for the next instance of the Webinar). Please double encode your UUID when using it for API calls if the UUID begins with a '/' or contains '//' in it.
     */
    pub async fn absentee(
        &self,
        occurrence_id: &str,
        page_size: i64,
        next_page_token: &str,
        webinar_uuid: &str,
    ) -> Result<crate::types::Domains> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if !occurrence_id.is_empty() {
            query_args.push(("occurrence_id".to_string(), occurrence_id.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/past_webinars/{}/absentees?{}",
            crate::progenitor_support::encode_path(&webinar_uuid.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get webinar tracking sources.
     *
     * This function performs a `GET` to the `/webinars/{webinarId}/tracking_sources` endpoint.
     *
     * [Webinar Registration Tracking Sources](https://support.zoom.us/hc/en-us/articles/360000315683-Webinar-Registration-Source-Tracking) allow you to see where your registrants are coming from if you share the webinar registration page in multiple platforms. You can then use the source tracking to see the number of registrants generated from each platform.<br> Use this API to list information on all the tracking sources of a Webinar.<br>
     * **Scopes:** `webinar:read:admin`, `webinar:read`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     * **Prerequisites**:<br>
     * * [Webinar license](https://zoom.us/webinar).
     * * Registration must be required for the Webinar.
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     */
    pub async fn get_tracking_sources(
        &self,
        webinar_id: &str,
    ) -> Result<crate::types::GetTrackingSourcesResponse> {
        let url = format!(
            "/webinars/{}/tracking_sources",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * List past webinar poll results.
     *
     * This function performs a `GET` to the `/past_webinars/{webinarId}/polls` endpoint.
     *
     * The polling feature for webinar allows you to create single choice or multiple choice polling questions for your webinars. Use this API to retrieve the results for Webinar Polls of a specific Webinar.
     *
     * **Prerequisites:**<br>
     * * [Webinar license](https://zoom.us/webinar)<br>
     * **Scopes**: `webinar:read:admin`, `webinar:read`<br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: &str` -- The webinar ID or the webinar UUID.  If a webinar ID is provided in the request instead of a UUID, the response will be for the latest webinar instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     */
    pub async fn list_past_poll_results(
        &self,
        webinar_id: &str,
    ) -> Result<crate::types::ListPastMeetingPollsResponse> {
        let url = format!(
            "/past_webinars/{}/polls",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * List Q&A of past webinar.
     *
     * This function performs a `GET` to the `/past_webinars/{webinarId}/qa` endpoint.
     *
     * The [Question & Answer (Q&A)](https://support.zoom.us/hc/en-us/articles/203686015-Getting-Started-with-Question-Answer) feature for Webinars allows attendees to ask questions during the Webinar and for the panelists, co-hosts and host to answer their questions.<br>
     * Use this API to list Q&A of a specific Webinar.
     *
     * **Prerequisites:**<br>
     * * [Webinar license](https://zoom.us/webinar)<br>
     * **Scopes**: `webinar:read:admin`, `webinar:read`<br>
     *  <br>
     *
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `webinar_id: &str` -- The webinar ID or the webinar UUID.  If a webinar ID is provided in the request instead of a UUID, the response will be for the latest webinar instance.
     *   
     *   If a UUID starts with "/" or contains "//" (example: "/ajXp112QmuoKj4854875==\"), you must **double encode** the UUID before making an API request.
     */
    pub async fn list_past_qa(
        &self,
        webinar_id: &str,
    ) -> Result<crate::types::ListPastWebinarQaResponse> {
        let url = format!(
            "/past_webinars/{}/qa",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * List webinar templates.
     *
     * This function performs a `GET` to the `/users/{userId}/webinar_templates` endpoint.
     *
     * Use this API to list a user's existing [webinar templates'](https://support.zoom.us/hc/en-us/articles/115001079746-Webinar-Templates) information. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * When you schedule a webinar, you can save the settings for that webinar as a template for scheduling future webinars.
     *
     * **Prerequisites:**
     * * Pro or a higher account with Webinar plan enabled.
     */
    pub async fn list_templates(
        &self,
        user_id: &str,
    ) -> Result<crate::types::ListWebinarTemplatesResponse> {
        let url = format!(
            "/users/{}/webinar_templates",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Get live stream details.
     *
     * This function performs a `GET` to the `/webinars/{webinarId}/livestream` endpoint.
     *
     * Zoom allows users to [live stream a webinar](https://support.zoom.us/hc/en-us/articles/115001777826-Live-Streaming-Meetings-or-Webinars-Using-a-Custom-Service) to a custom platform. Use this API to get a webinar's live stream configuration details such as Stream URL, Stream Key and Page URL.<br><br>
     * **Prerequisites:**<br>
     * * Pro or higher plan with a Webinar Add-on.<br>
     * * Live streaming details must have been [configured](https://support.zoom.us/hc/en-us/articles/115001777826-Live-Streaming-Meetings-or-Webinars-Using-a-Custom-Service#h_01589a6f-a40a-4e18-a448-cb746e52ebc5) for the webinar.<br><br>
     * **Scopes:** `webinar:read:admin` `webinar:read`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: &str` -- The webinar's unique ID.
     */
    pub async fn get_live_stream_details(
        &self,
        webinar_id: &str,
    ) -> Result<crate::types::GetLiveStreamDetailsResponse> {
        let url = format!(
            "/webinars/{}/livestream",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update a live stream.
     *
     * This function performs a `PATCH` to the `/webinars/{webinarId}/livestream` endpoint.
     *
     * Zoom allows users to [live stream a webinar](https://support.zoom.us/hc/en-us/articles/115001777826-Live-Streaming-Meetings-or-Webinars-Using-a-Custom-Service) to a custom platform. Use this API to update a webinar's live stream information.<br><br>
     * **Prerequisites:**<br>
     * * Pro or higher plan with a Webinar Add-on.<br>
     * * Live streaming details must have been [configured](https://support.zoom.us/hc/en-us/articles/115001777826-Live-Streaming-Meetings-or-Webinars-Using-a-Custom-Service#h_01589a6f-a40a-4e18-a448-cb746e52ebc5) for the webinar.<br><br>
     * **Scopes:** `webinar:write:admin` `webinar:write`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     */
    pub async fn live_stream_update(
        &self,
        webinar_id: &str,
        body: &crate::types::MeetingLiveStream,
    ) -> Result<()> {
        let url = format!(
            "/webinars/{}/livestream",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Update Live Stream Status.
     *
     * This function performs a `PATCH` to the `/webinars/{webinarId}/livestream/status` endpoint.
     *
     * Zoom allows users to [live stream a webinar](https://support.zoom.us/hc/en-us/articles/115001777826-Live-Streaming-Meetings-or-Webinars-Using-a-Custom-Service) to a custom platform. Use this API to update the status of a webinar's live stream.<br><br>
     * **Prerequisites:**<br>
     * * Pro or higher plan with a Webinar Add-on.<br>
     * * Live streaming details must have been [configured](https://support.zoom.us/hc/en-us/articles/115001777826-Live-Streaming-Meetings-or-Webinars-Using-a-Custom-Service#h_01589a6f-a40a-4e18-a448-cb746e52ebc5) for the webinar.<br><br>
     * **Scopes:** `webinar:write:admin` `webinar:write`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     */
    pub async fn live_stream_status_update(
        &self,
        webinar_id: i64,
        body: &crate::types::WebinarLiveStreamStatus,
    ) -> Result<()> {
        let url = format!(
            "/webinars/{}/livestream/status",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Create webinar's invite links.
     *
     * This function performs a `POST` to the `/webinars/{webinarId}/invite_links` endpoint.
     *
     * Use this API to create a batch of invitation links for a webinar.
     *
     * **Scopes**: `webinar:write:admin`, `webinar:write`</br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     *
     * * Business, Education or API Plan with Webinar add-on.
     *
     * **Parameters:**
     *
     * * `webinar_id: i64` -- The webinar ID in "**long**" format(represented as int64 data type in JSON). .
     */
    pub async fn invite_links_create(
        &self,
        webinar_id: i64,
        body: &crate::types::InviteLink,
    ) -> Result<crate::types::InviteLinks> {
        let url = format!(
            "/webinars/{}/invite_links",
            crate::progenitor_support::encode_path(&webinar_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
