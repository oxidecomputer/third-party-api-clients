use crate::Client;
use crate::ClientResult;

pub struct Events {
    pub client: Client,
}

impl Events {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Events { client }
    }

    /**
     * This function performs a `GET` to the `/calendars/{calendarId}/events` endpoint.
     *
     * Returns events on the specified calendar.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     * * `always_include_email: bool` -- Deprecated and ignored. A value will always be returned in the email field for the organizer, creator and attendees, even if no real email address is available (i.e. a generated, non-working value will be provided).
     * * `i_cal_uid: &str` -- Specifies event ID in the iCalendar format to be included in the response. Optional.
     * * `max_attendees: i64` -- The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.
     * * `max_results: i64` -- Maximum number of events returned on one result page. The number of events in the resulting page may be less than this value, or none at all, even if there are more events matching the query. Incomplete pages can be detected by a non-empty nextPageToken field in the response. By default the value is 250 events. The page size can never be larger than 2500 events. Optional.
     * * `order_by: crate::types::OrderBy` -- The order of the events returned in the result. Optional. The default is an unspecified, stable order.
     * * `page_token: &str` -- Token specifying which result page to return. Optional.
     * * `private_extended_property: &[String]` -- Extended properties constraint specified as propertyName=value. Matches only private properties. This parameter might be repeated multiple times to return events that match all given constraints.
     * * `q: &str` -- Free text search terms to find events that match these terms in any field, except for extended properties. Optional.
     * * `shared_extended_property: &[String]` -- Extended properties constraint specified as propertyName=value. Matches only shared properties. This parameter might be repeated multiple times to return events that match all given constraints.
     * * `show_deleted: bool` -- Whether to include deleted events (with status equals "cancelled") in the result. Cancelled instances of recurring events (but not the underlying recurring event) will still be included if showDeleted and singleEvents are both False. If showDeleted and singleEvents are both True, only single instances of deleted events (but not the underlying recurring events) are returned. Optional. The default is False.
     * * `show_hidden_invitations: bool` -- Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     * * `single_events: bool` -- Whether to expand recurring events into instances and only return single one-off events and instances of recurring events, but not the underlying recurring events themselves. Optional. The default is False.
     * * `sync_token: &str` -- Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. All events deleted since the previous list request will always be in the result set and it is not allowed to set showDeleted to False.
     *   There are several query parameters that cannot be specified together with nextSyncToken to ensure consistency of the client state.
     *   
     *   These are:
     *   - iCalUID
     *   - orderBy
     *   - privateExtendedProperty
     *   - q
     *   - sharedExtendedProperty
     *   - timeMin
     *   - timeMax
     *   - updatedMin If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.
     *   Learn more about incremental synchronization.
     *   Optional. The default is to return all entries.
     * * `time_max: &str` -- Upper bound (exclusive) for an event's start time to filter by. Optional. The default is not to filter by start time. Must be an RFC3339 timestamp with mandatory time zone offset, for example, 2011-06-03T10:00:00-07:00, 2011-06-03T10:00:00Z. Milliseconds may be provided but are ignored. If timeMin is set, timeMax must be greater than timeMin.
     * * `time_min: &str` -- Lower bound (exclusive) for an event's end time to filter by. Optional. The default is not to filter by end time. Must be an RFC3339 timestamp with mandatory time zone offset, for example, 2011-06-03T10:00:00-07:00, 2011-06-03T10:00:00Z. Milliseconds may be provided but are ignored. If timeMax is set, timeMin must be smaller than timeMax.
     * * `time_zone: &str` -- Time zone used in the response. Optional. The default is the time zone of the calendar.
     * * `updated_min: &str` -- Lower bound for an event's last modification time (as a RFC3339 timestamp) to filter by. When specified, entries deleted since this time will always be included regardless of showDeleted. Optional. The default is not to filter by last modification time.
     */
    pub async fn list(
        &self,
        calendar_id: &str,
        i_cal_uid: &str,
        max_attendees: i64,
        max_results: i64,
        order_by: crate::types::OrderBy,
        page_token: &str,
        private_extended_property: &[String],
        q: &str,
        shared_extended_property: &[String],
        show_deleted: bool,
        show_hidden_invitations: bool,
        single_events: bool,
        time_max: &str,
        time_min: &str,
        time_zone: &str,
        updated_min: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !i_cal_uid.is_empty() {
            query_args.push(("iCalUID".to_string(), i_cal_uid.to_string()));
        }
        if max_attendees > 0 {
            query_args.push(("maxAttendees".to_string(), max_attendees.to_string()));
        }
        if max_results > 0 {
            query_args.push(("maxResults".to_string(), max_results.to_string()));
        }
        if !order_by.to_string().is_empty() {
            query_args.push(("orderBy".to_string(), order_by.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        if !private_extended_property.is_empty() {
            query_args.push((
                "privateExtendedProperty".to_string(),
                private_extended_property.join(" "),
            ));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if !shared_extended_property.is_empty() {
            query_args.push((
                "sharedExtendedProperty".to_string(),
                shared_extended_property.join(" "),
            ));
        }
        if show_deleted {
            query_args.push(("showDeleted".to_string(), show_deleted.to_string()));
        }
        if show_hidden_invitations {
            query_args.push((
                "showHiddenInvitations".to_string(),
                show_hidden_invitations.to_string(),
            ));
        }
        if single_events {
            query_args.push(("singleEvents".to_string(), single_events.to_string()));
        }
        if !time_max.is_empty() {
            query_args.push(("timeMax".to_string(), time_max.to_string()));
        }
        if !time_min.is_empty() {
            query_args.push(("timeMin".to_string(), time_min.to_string()));
        }
        if !time_zone.is_empty() {
            query_args.push(("timeZone".to_string(), time_zone.to_string()));
        }
        if !updated_min.is_empty() {
            query_args.push(("updatedMin".to_string(), updated_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/calendars/{}/events?{}",
                crate::progenitor_support::encode_path(calendar_id),
                query_
            ),
            None,
        );
        let resp: crate::Response<crate::types::Events> = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(crate::Response::new(
            resp.status,
            resp.headers,
            resp.body.items.to_vec(),
        ))
    }
    /**
     * This function performs a `GET` to the `/calendars/{calendarId}/events` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Returns events on the specified calendar.
     */
    pub async fn list_all(
        &self,
        calendar_id: &str,
        i_cal_uid: &str,
        max_attendees: i64,
        order_by: crate::types::OrderBy,
        private_extended_property: &[String],
        q: &str,
        shared_extended_property: &[String],
        show_deleted: bool,
        show_hidden_invitations: bool,
        single_events: bool,
        time_max: &str,
        time_min: &str,
        time_zone: &str,
        updated_min: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !i_cal_uid.is_empty() {
            query_args.push(("iCalUID".to_string(), i_cal_uid.to_string()));
        }
        if max_attendees > 0 {
            query_args.push(("maxAttendees".to_string(), max_attendees.to_string()));
        }
        if !order_by.to_string().is_empty() {
            query_args.push(("orderBy".to_string(), order_by.to_string()));
        }
        if !private_extended_property.is_empty() {
            query_args.push((
                "privateExtendedProperty".to_string(),
                private_extended_property.join(" "),
            ));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if !shared_extended_property.is_empty() {
            query_args.push((
                "sharedExtendedProperty".to_string(),
                shared_extended_property.join(" "),
            ));
        }
        if show_deleted {
            query_args.push(("showDeleted".to_string(), show_deleted.to_string()));
        }
        if show_hidden_invitations {
            query_args.push((
                "showHiddenInvitations".to_string(),
                show_hidden_invitations.to_string(),
            ));
        }
        if single_events {
            query_args.push(("singleEvents".to_string(), single_events.to_string()));
        }
        if !time_max.is_empty() {
            query_args.push(("timeMax".to_string(), time_max.to_string()));
        }
        if !time_min.is_empty() {
            query_args.push(("timeMin".to_string(), time_min.to_string()));
        }
        if !time_zone.is_empty() {
            query_args.push(("timeZone".to_string(), time_zone.to_string()));
        }
        if !updated_min.is_empty() {
            query_args.push(("updatedMin".to_string(), updated_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/calendars/{}/events?{}",
                crate::progenitor_support::encode_path(calendar_id),
                query_
            ),
            None,
        );
        let crate::Response::<crate::types::Events> {
            mut status,
            mut headers,
            mut body,
        } = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut items = body.items;
        let mut page = body.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                crate::Response::<crate::types::Events> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}?pageToken={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                crate::Response::<crate::types::Events> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}&pageToken={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            items.append(&mut body.items);

            if !body.next_page_token.is_empty() && body.next_page_token != page {
                page = body.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(crate::Response::new(status, headers, items))
    }
    /**
     * This function performs a `POST` to the `/calendars/{calendarId}/events` endpoint.
     *
     * Creates an event.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     * * `conference_data_version: u64` -- Version number of conference data supported by the API client. Version 0 assumes no conference data support and ignores conference data in the event's body. Version 1 enables support for copying of ConferenceData as well as for creating new conferences using the createRequest field of conferenceData. The default is 0.
     * * `max_attendees: i64` -- The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.
     * * `send_notifications: bool` -- Deprecated. Please use sendUpdates instead.
     *   
     *   Whether to send notifications about the creation of the new event. Note that some emails might still be sent even if you set the value to false. The default is false.
     * * `send_updates: crate::types::SendUpdates` -- Whether to send notifications about the creation of the new event. Note that some emails might still be sent. The default is false.
     * * `supports_attachments: bool` -- Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    pub async fn insert(
        &self,
        calendar_id: &str,
        conference_data_version: u64,
        max_attendees: i64,
        send_notifications: bool,
        send_updates: crate::types::SendUpdates,
        supports_attachments: bool,
        body: &crate::types::Event,
    ) -> ClientResult<crate::Response<crate::types::Event>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !conference_data_version.to_string().is_empty() {
            query_args.push((
                "conferenceDataVersion".to_string(),
                conference_data_version.to_string(),
            ));
        }
        if max_attendees > 0 {
            query_args.push(("maxAttendees".to_string(), max_attendees.to_string()));
        }
        if send_notifications {
            query_args.push((
                "sendNotifications".to_string(),
                send_notifications.to_string(),
            ));
        }
        if !send_updates.to_string().is_empty() {
            query_args.push(("sendUpdates".to_string(), send_updates.to_string()));
        }
        if supports_attachments {
            query_args.push((
                "supportsAttachments".to_string(),
                supports_attachments.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/calendars/{}/events?{}",
                crate::progenitor_support::encode_path(calendar_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/calendars/{calendarId}/events/import` endpoint.
     *
     * Imports an event. This operation is used to add a private copy of an existing event to a calendar.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     * * `conference_data_version: u64` -- Version number of conference data supported by the API client. Version 0 assumes no conference data support and ignores conference data in the event's body. Version 1 enables support for copying of ConferenceData as well as for creating new conferences using the createRequest field of conferenceData. The default is 0.
     * * `supports_attachments: bool` -- Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    pub async fn import(
        &self,
        calendar_id: &str,
        conference_data_version: u64,
        supports_attachments: bool,
        body: &crate::types::Event,
    ) -> ClientResult<crate::Response<crate::types::Event>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !conference_data_version.to_string().is_empty() {
            query_args.push((
                "conferenceDataVersion".to_string(),
                conference_data_version.to_string(),
            ));
        }
        if supports_attachments {
            query_args.push((
                "supportsAttachments".to_string(),
                supports_attachments.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/calendars/{}/events/import?{}",
                crate::progenitor_support::encode_path(calendar_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/calendars/{calendarId}/events/quickAdd` endpoint.
     *
     * Creates an event based on a simple text string.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     * * `text: &str` -- The text describing the event to be created.
     * * `send_notifications: bool` -- Deprecated. Please use sendUpdates instead.
     *   
     *   Whether to send notifications about the creation of the event. Note that some emails might still be sent even if you set the value to false. The default is false.
     * * `send_updates: crate::types::SendUpdates` -- Whether to send notifications about the creation of the new event. Note that some emails might still be sent. The default is false.
     */
    pub async fn quick_add(
        &self,
        calendar_id: &str,
        text: &str,
        send_notifications: bool,
        send_updates: crate::types::SendUpdates,
    ) -> ClientResult<crate::Response<crate::types::Event>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if send_notifications {
            query_args.push((
                "sendNotifications".to_string(),
                send_notifications.to_string(),
            ));
        }
        if !send_updates.to_string().is_empty() {
            query_args.push(("sendUpdates".to_string(), send_updates.to_string()));
        }
        if !text.is_empty() {
            query_args.push(("text".to_string(), text.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/calendars/{}/events/quickAdd?{}",
                crate::progenitor_support::encode_path(calendar_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/calendars/{calendarId}/events/watch` endpoint.
     *
     * Watch for changes to Events resources.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     * * `always_include_email: bool` -- Deprecated and ignored. A value will always be returned in the email field for the organizer, creator and attendees, even if no real email address is available (i.e. a generated, non-working value will be provided).
     * * `i_cal_uid: &str` -- Specifies event ID in the iCalendar format to be included in the response. Optional.
     * * `max_attendees: i64` -- The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.
     * * `max_results: i64` -- Maximum number of events returned on one result page. The number of events in the resulting page may be less than this value, or none at all, even if there are more events matching the query. Incomplete pages can be detected by a non-empty nextPageToken field in the response. By default the value is 250 events. The page size can never be larger than 2500 events. Optional.
     * * `order_by: crate::types::OrderBy` -- The order of the events returned in the result. Optional. The default is an unspecified, stable order.
     * * `page_token: &str` -- Token specifying which result page to return. Optional.
     * * `private_extended_property: &[String]` -- Extended properties constraint specified as propertyName=value. Matches only private properties. This parameter might be repeated multiple times to return events that match all given constraints.
     * * `q: &str` -- Free text search terms to find events that match these terms in any field, except for extended properties. Optional.
     * * `shared_extended_property: &[String]` -- Extended properties constraint specified as propertyName=value. Matches only shared properties. This parameter might be repeated multiple times to return events that match all given constraints.
     * * `show_deleted: bool` -- Whether to include deleted events (with status equals "cancelled") in the result. Cancelled instances of recurring events (but not the underlying recurring event) will still be included if showDeleted and singleEvents are both False. If showDeleted and singleEvents are both True, only single instances of deleted events (but not the underlying recurring events) are returned. Optional. The default is False.
     * * `show_hidden_invitations: bool` -- Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     * * `single_events: bool` -- Whether to expand recurring events into instances and only return single one-off events and instances of recurring events, but not the underlying recurring events themselves. Optional. The default is False.
     * * `sync_token: &str` -- Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. All events deleted since the previous list request will always be in the result set and it is not allowed to set showDeleted to False.
     *   There are several query parameters that cannot be specified together with nextSyncToken to ensure consistency of the client state.
     *   
     *   These are:
     *   - iCalUID
     *   - orderBy
     *   - privateExtendedProperty
     *   - q
     *   - sharedExtendedProperty
     *   - timeMin
     *   - timeMax
     *   - updatedMin If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.
     *   Learn more about incremental synchronization.
     *   Optional. The default is to return all entries.
     * * `time_max: &str` -- Upper bound (exclusive) for an event's start time to filter by. Optional. The default is not to filter by start time. Must be an RFC3339 timestamp with mandatory time zone offset, for example, 2011-06-03T10:00:00-07:00, 2011-06-03T10:00:00Z. Milliseconds may be provided but are ignored. If timeMin is set, timeMax must be greater than timeMin.
     * * `time_min: &str` -- Lower bound (exclusive) for an event's end time to filter by. Optional. The default is not to filter by end time. Must be an RFC3339 timestamp with mandatory time zone offset, for example, 2011-06-03T10:00:00-07:00, 2011-06-03T10:00:00Z. Milliseconds may be provided but are ignored. If timeMax is set, timeMin must be smaller than timeMax.
     * * `time_zone: &str` -- Time zone used in the response. Optional. The default is the time zone of the calendar.
     * * `updated_min: &str` -- Lower bound for an event's last modification time (as a RFC3339 timestamp) to filter by. When specified, entries deleted since this time will always be included regardless of showDeleted. Optional. The default is not to filter by last modification time.
     */
    pub async fn watch(
        &self,
        calendar_id: &str,
        i_cal_uid: &str,
        max_attendees: i64,
        max_results: i64,
        order_by: crate::types::OrderBy,
        page_token: &str,
        private_extended_property: &[String],
        q: &str,
        shared_extended_property: &[String],
        show_deleted: bool,
        show_hidden_invitations: bool,
        single_events: bool,
        time_max: &str,
        time_min: &str,
        time_zone: &str,
        updated_min: &str,
        body: &crate::types::Channel,
    ) -> ClientResult<crate::Response<crate::types::Channel>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !i_cal_uid.is_empty() {
            query_args.push(("iCalUID".to_string(), i_cal_uid.to_string()));
        }
        if max_attendees > 0 {
            query_args.push(("maxAttendees".to_string(), max_attendees.to_string()));
        }
        if max_results > 0 {
            query_args.push(("maxResults".to_string(), max_results.to_string()));
        }
        if !order_by.to_string().is_empty() {
            query_args.push(("orderBy".to_string(), order_by.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        if !private_extended_property.is_empty() {
            query_args.push((
                "privateExtendedProperty".to_string(),
                private_extended_property.join(" "),
            ));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        if !shared_extended_property.is_empty() {
            query_args.push((
                "sharedExtendedProperty".to_string(),
                shared_extended_property.join(" "),
            ));
        }
        if show_deleted {
            query_args.push(("showDeleted".to_string(), show_deleted.to_string()));
        }
        if show_hidden_invitations {
            query_args.push((
                "showHiddenInvitations".to_string(),
                show_hidden_invitations.to_string(),
            ));
        }
        if single_events {
            query_args.push(("singleEvents".to_string(), single_events.to_string()));
        }
        if !time_max.is_empty() {
            query_args.push(("timeMax".to_string(), time_max.to_string()));
        }
        if !time_min.is_empty() {
            query_args.push(("timeMin".to_string(), time_min.to_string()));
        }
        if !time_zone.is_empty() {
            query_args.push(("timeZone".to_string(), time_zone.to_string()));
        }
        if !updated_min.is_empty() {
            query_args.push(("updatedMin".to_string(), updated_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/calendars/{}/events/watch?{}",
                crate::progenitor_support::encode_path(calendar_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/calendars/{calendarId}/events/{eventId}` endpoint.
     *
     * Returns an event.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     * * `event_id: &str` -- ETag of the collection.
     * * `always_include_email: bool` -- Deprecated and ignored. A value will always be returned in the email field for the organizer, creator and attendees, even if no real email address is available (i.e. a generated, non-working value will be provided).
     * * `max_attendees: i64` -- The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.
     * * `time_zone: &str` -- Time zone used in the response. Optional. The default is the time zone of the calendar.
     */
    pub async fn get(
        &self,
        calendar_id: &str,
        event_id: &str,
        max_attendees: i64,
        time_zone: &str,
    ) -> ClientResult<crate::Response<crate::types::Event>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if max_attendees > 0 {
            query_args.push(("maxAttendees".to_string(), max_attendees.to_string()));
        }
        if !time_zone.is_empty() {
            query_args.push(("timeZone".to_string(), time_zone.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/calendars/{}/events/{}?{}",
                crate::progenitor_support::encode_path(calendar_id),
                crate::progenitor_support::encode_path(event_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * This function performs a `PUT` to the `/calendars/{calendarId}/events/{eventId}` endpoint.
     *
     * Updates an event.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     * * `event_id: &str` -- ETag of the collection.
     * * `always_include_email: bool` -- Deprecated and ignored. A value will always be returned in the email field for the organizer, creator and attendees, even if no real email address is available (i.e. a generated, non-working value will be provided).
     * * `conference_data_version: u64` -- Version number of conference data supported by the API client. Version 0 assumes no conference data support and ignores conference data in the event's body. Version 1 enables support for copying of ConferenceData as well as for creating new conferences using the createRequest field of conferenceData. The default is 0.
     * * `max_attendees: i64` -- The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.
     * * `send_notifications: bool` -- Deprecated. Please use sendUpdates instead.
     *   
     *   Whether to send notifications about the event update (for example, description changes, etc.). Note that some emails might still be sent even if you set the value to false. The default is false.
     * * `send_updates: crate::types::SendUpdates` -- Whether to send notifications about the creation of the new event. Note that some emails might still be sent. The default is false.
     * * `supports_attachments: bool` -- Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    pub async fn update(
        &self,
        calendar_id: &str,
        event_id: &str,
        conference_data_version: u64,
        max_attendees: i64,
        send_notifications: bool,
        send_updates: crate::types::SendUpdates,
        supports_attachments: bool,
        body: &crate::types::Event,
    ) -> ClientResult<crate::Response<crate::types::Event>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !conference_data_version.to_string().is_empty() {
            query_args.push((
                "conferenceDataVersion".to_string(),
                conference_data_version.to_string(),
            ));
        }
        if max_attendees > 0 {
            query_args.push(("maxAttendees".to_string(), max_attendees.to_string()));
        }
        if send_notifications {
            query_args.push((
                "sendNotifications".to_string(),
                send_notifications.to_string(),
            ));
        }
        if !send_updates.to_string().is_empty() {
            query_args.push(("sendUpdates".to_string(), send_updates.to_string()));
        }
        if supports_attachments {
            query_args.push((
                "supportsAttachments".to_string(),
                supports_attachments.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/calendars/{}/events/{}?{}",
                crate::progenitor_support::encode_path(calendar_id),
                crate::progenitor_support::encode_path(event_id),
                query_
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `DELETE` to the `/calendars/{calendarId}/events/{eventId}` endpoint.
     *
     * Deletes an event.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     * * `event_id: &str` -- ETag of the collection.
     * * `send_notifications: bool` -- Deprecated. Please use sendUpdates instead.
     *   
     *   Whether to send notifications about the deletion of the event. Note that some emails might still be sent even if you set the value to false. The default is false.
     * * `send_updates: crate::types::SendUpdates` -- Whether to send notifications about the creation of the new event. Note that some emails might still be sent. The default is false.
     */
    pub async fn delete(
        &self,
        calendar_id: &str,
        event_id: &str,
        send_notifications: bool,
        send_updates: crate::types::SendUpdates,
    ) -> ClientResult<crate::Response<()>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if send_notifications {
            query_args.push((
                "sendNotifications".to_string(),
                send_notifications.to_string(),
            ));
        }
        if !send_updates.to_string().is_empty() {
            query_args.push(("sendUpdates".to_string(), send_updates.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/calendars/{}/events/{}?{}",
                crate::progenitor_support::encode_path(calendar_id),
                crate::progenitor_support::encode_path(event_id),
                query_
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * This function performs a `PATCH` to the `/calendars/{calendarId}/events/{eventId}` endpoint.
     *
     * Updates an event. This method supports patch semantics.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     * * `event_id: &str` -- ETag of the collection.
     * * `always_include_email: bool` -- Deprecated and ignored. A value will always be returned in the email field for the organizer, creator and attendees, even if no real email address is available (i.e. a generated, non-working value will be provided).
     * * `conference_data_version: u64` -- Version number of conference data supported by the API client. Version 0 assumes no conference data support and ignores conference data in the event's body. Version 1 enables support for copying of ConferenceData as well as for creating new conferences using the createRequest field of conferenceData. The default is 0.
     * * `max_attendees: i64` -- The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.
     * * `send_notifications: bool` -- Deprecated. Please use sendUpdates instead.
     *   
     *   Whether to send notifications about the event update (for example, description changes, etc.). Note that some emails might still be sent even if you set the value to false. The default is false.
     * * `send_updates: crate::types::SendUpdates` -- Whether to send notifications about the creation of the new event. Note that some emails might still be sent. The default is false.
     * * `supports_attachments: bool` -- Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    pub async fn patch(
        &self,
        calendar_id: &str,
        event_id: &str,
        conference_data_version: u64,
        max_attendees: i64,
        send_notifications: bool,
        send_updates: crate::types::SendUpdates,
        supports_attachments: bool,
        body: &crate::types::Event,
    ) -> ClientResult<crate::Response<crate::types::Event>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !conference_data_version.to_string().is_empty() {
            query_args.push((
                "conferenceDataVersion".to_string(),
                conference_data_version.to_string(),
            ));
        }
        if max_attendees > 0 {
            query_args.push(("maxAttendees".to_string(), max_attendees.to_string()));
        }
        if send_notifications {
            query_args.push((
                "sendNotifications".to_string(),
                send_notifications.to_string(),
            ));
        }
        if !send_updates.to_string().is_empty() {
            query_args.push(("sendUpdates".to_string(), send_updates.to_string()));
        }
        if supports_attachments {
            query_args.push((
                "supportsAttachments".to_string(),
                supports_attachments.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/calendars/{}/events/{}?{}",
                crate::progenitor_support::encode_path(calendar_id),
                crate::progenitor_support::encode_path(event_id),
                query_
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/calendars/{calendarId}/events/{eventId}/instances` endpoint.
     *
     * Returns instances of the specified recurring event.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     * * `event_id: &str` -- Recurring event identifier.
     * * `always_include_email: bool` -- Deprecated and ignored. A value will always be returned in the email field for the organizer, creator and attendees, even if no real email address is available (i.e. a generated, non-working value will be provided).
     * * `max_attendees: i64` -- The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.
     * * `max_results: i64` -- Maximum number of events returned on one result page. By default the value is 250 events. The page size can never be larger than 2500 events. Optional.
     * * `original_start: &str` -- The original start time of the instance in the result. Optional.
     * * `page_token: &str` -- Token specifying which result page to return. Optional.
     * * `show_deleted: bool` -- Whether to include deleted events (with status equals "cancelled") in the result. Cancelled instances of recurring events will still be included if singleEvents is False. Optional. The default is False.
     * * `time_max: &str` -- Upper bound (exclusive) for an event's start time to filter by. Optional. The default is not to filter by start time. Must be an RFC3339 timestamp with mandatory time zone offset.
     * * `time_min: &str` -- Lower bound (inclusive) for an event's end time to filter by. Optional. The default is not to filter by end time. Must be an RFC3339 timestamp with mandatory time zone offset.
     * * `time_zone: &str` -- Time zone used in the response. Optional. The default is the time zone of the calendar.
     */
    pub async fn instances(
        &self,
        calendar_id: &str,
        event_id: &str,
        max_attendees: i64,
        max_results: i64,
        original_start: &str,
        page_token: &str,
        show_deleted: bool,
        time_max: &str,
        time_min: &str,
        time_zone: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if max_attendees > 0 {
            query_args.push(("maxAttendees".to_string(), max_attendees.to_string()));
        }
        if max_results > 0 {
            query_args.push(("maxResults".to_string(), max_results.to_string()));
        }
        if !original_start.is_empty() {
            query_args.push(("originalStart".to_string(), original_start.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        if show_deleted {
            query_args.push(("showDeleted".to_string(), show_deleted.to_string()));
        }
        if !time_max.is_empty() {
            query_args.push(("timeMax".to_string(), time_max.to_string()));
        }
        if !time_min.is_empty() {
            query_args.push(("timeMin".to_string(), time_min.to_string()));
        }
        if !time_zone.is_empty() {
            query_args.push(("timeZone".to_string(), time_zone.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/calendars/{}/events/{}/instances?{}",
                crate::progenitor_support::encode_path(calendar_id),
                crate::progenitor_support::encode_path(event_id),
                query_
            ),
            None,
        );
        let resp: crate::Response<crate::types::Events> = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(crate::Response::new(
            resp.status,
            resp.headers,
            resp.body.items.to_vec(),
        ))
    }
    /**
     * This function performs a `GET` to the `/calendars/{calendarId}/events/{eventId}/instances` endpoint.
     *
     * As opposed to `instances`, this function returns all the pages of the request at once.
     *
     * Returns instances of the specified recurring event.
     */
    pub async fn get_all_instances(
        &self,
        calendar_id: &str,
        event_id: &str,
        max_attendees: i64,
        original_start: &str,
        show_deleted: bool,
        time_max: &str,
        time_min: &str,
        time_zone: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Event>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if max_attendees > 0 {
            query_args.push(("maxAttendees".to_string(), max_attendees.to_string()));
        }
        if !original_start.is_empty() {
            query_args.push(("originalStart".to_string(), original_start.to_string()));
        }
        if show_deleted {
            query_args.push(("showDeleted".to_string(), show_deleted.to_string()));
        }
        if !time_max.is_empty() {
            query_args.push(("timeMax".to_string(), time_max.to_string()));
        }
        if !time_min.is_empty() {
            query_args.push(("timeMin".to_string(), time_min.to_string()));
        }
        if !time_zone.is_empty() {
            query_args.push(("timeZone".to_string(), time_zone.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/calendars/{}/events/{}/instances?{}",
                crate::progenitor_support::encode_path(calendar_id),
                crate::progenitor_support::encode_path(event_id),
                query_
            ),
            None,
        );
        let crate::Response::<crate::types::Events> {
            mut status,
            mut headers,
            mut body,
        } = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut items = body.items;
        let mut page = body.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                crate::Response::<crate::types::Events> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}?pageToken={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                crate::Response::<crate::types::Events> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}&pageToken={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            items.append(&mut body.items);

            if !body.next_page_token.is_empty() && body.next_page_token != page {
                page = body.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(crate::Response::new(status, headers, items))
    }
    /**
     * This function performs a `POST` to the `/calendars/{calendarId}/events/{eventId}/move` endpoint.
     *
     * Moves an event to another calendar, i.e. changes an event's organizer.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier of the source calendar where the event currently is on.
     * * `event_id: &str` -- ETag of the collection.
     * * `destination: &str` -- Calendar identifier of the target calendar where the event is to be moved to.
     * * `send_notifications: bool` -- Deprecated. Please use sendUpdates instead.
     *   
     *   Whether to send notifications about the change of the event's organizer. Note that some emails might still be sent even if you set the value to false. The default is false.
     * * `send_updates: crate::types::SendUpdates` -- Whether to send notifications about the creation of the new event. Note that some emails might still be sent. The default is false.
     */
    pub async fn mv(
        &self,
        calendar_id: &str,
        event_id: &str,
        destination: &str,
        send_notifications: bool,
        send_updates: crate::types::SendUpdates,
    ) -> ClientResult<crate::Response<crate::types::Event>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !destination.is_empty() {
            query_args.push(("destination".to_string(), destination.to_string()));
        }
        if send_notifications {
            query_args.push((
                "sendNotifications".to_string(),
                send_notifications.to_string(),
            ));
        }
        if !send_updates.to_string().is_empty() {
            query_args.push(("sendUpdates".to_string(), send_updates.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/calendars/{}/events/{}/move?{}",
                crate::progenitor_support::encode_path(calendar_id),
                crate::progenitor_support::encode_path(event_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}
