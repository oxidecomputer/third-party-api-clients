use crate::Client;
use crate::ClientResult;

pub struct CalendarList {
    pub client: Client,
}

impl CalendarList {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CalendarList { client }
    }

    /**
     * This function performs a `GET` to the `/users/me/calendarList` endpoint.
     *
     * Returns the calendars on the user's calendar list.
     *
     * **Parameters:**
     *
     * * `max_results: i64` -- Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional.
     * * `min_access_role: crate::types::MinAccessRole` -- The minimum access role for the user in the returned entries. Optional. The default is no restriction.
     * * `page_token: &str` -- Token specifying which result page to return. Optional.
     * * `show_deleted: bool` -- Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     * * `show_hidden: bool` -- Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     * * `sync_token: &str` -- Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. If only read-only fields such as calendar properties or ACLs have changed, the entry won't be returned. All entries deleted and hidden since the previous list request will always be in the result set and it is not allowed to set showDeleted neither showHidden to False.
     *   To ensure client state consistency minAccessRole query parameter cannot be specified together with nextSyncToken.
     *   If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.
     *   Learn more about incremental synchronization.
     *   Optional. The default is to return all entries.
     */
    pub async fn list(
        &self,
        max_results: i64,
        min_access_role: crate::types::MinAccessRole,
        page_token: &str,
        show_deleted: bool,
        show_hidden: bool,
    ) -> ClientResult<crate::Response<Vec<crate::types::CalendarListEntry>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if max_results > 0 {
            query_args.push(("maxResults".to_string(), max_results.to_string()));
        }
        if !min_access_role.to_string().is_empty() {
            query_args.push(("minAccessRole".to_string(), min_access_role.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        if show_deleted {
            query_args.push(("showDeleted".to_string(), show_deleted.to_string()));
        }
        if show_hidden {
            query_args.push(("showHidden".to_string(), show_hidden.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/users/me/calendarList?{}", query_), None);
        let resp: crate::Response<crate::types::CalendarList> = self
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
     * This function performs a `GET` to the `/users/me/calendarList` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Returns the calendars on the user's calendar list.
     */
    pub async fn list_all(
        &self,
        min_access_role: crate::types::MinAccessRole,
        show_deleted: bool,
        show_hidden: bool,
    ) -> ClientResult<crate::Response<Vec<crate::types::CalendarListEntry>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !min_access_role.to_string().is_empty() {
            query_args.push(("minAccessRole".to_string(), min_access_role.to_string()));
        }
        if show_deleted {
            query_args.push(("showDeleted".to_string(), show_deleted.to_string()));
        }
        if show_hidden {
            query_args.push(("showHidden".to_string(), show_hidden.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/users/me/calendarList?{}", query_), None);
        let crate::Response::<crate::types::CalendarList> {
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
                crate::Response::<crate::types::CalendarList> {
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
                crate::Response::<crate::types::CalendarList> {
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
     * This function performs a `POST` to the `/users/me/calendarList` endpoint.
     *
     * Inserts an existing calendar into the user's calendar list.
     *
     * **Parameters:**
     *
     * * `color_rgb_format: bool` -- Whether to use the foregroundColor and backgroundColor fields to write the calendar colors (RGB). If this feature is used, the index-based colorId field will be set to the best matching option automatically. Optional. The default is False.
     */
    pub async fn list_insert(
        &self,
        color_rgb_format: bool,
        body: &crate::types::CalendarListEntry,
    ) -> ClientResult<crate::Response<crate::types::CalendarListEntry>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if color_rgb_format {
            query_args.push(("colorRgbFormat".to_string(), color_rgb_format.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/users/me/calendarList?{}", query_), None);
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
     * This function performs a `POST` to the `/users/me/calendarList/watch` endpoint.
     *
     * Watch for changes to CalendarList resources.
     *
     * **Parameters:**
     *
     * * `max_results: i64` -- Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional.
     * * `min_access_role: crate::types::MinAccessRole` -- The minimum access role for the user in the returned entries. Optional. The default is no restriction.
     * * `page_token: &str` -- Token specifying which result page to return. Optional.
     * * `show_deleted: bool` -- Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     * * `show_hidden: bool` -- Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     * * `sync_token: &str` -- Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. If only read-only fields such as calendar properties or ACLs have changed, the entry won't be returned. All entries deleted and hidden since the previous list request will always be in the result set and it is not allowed to set showDeleted neither showHidden to False.
     *   To ensure client state consistency minAccessRole query parameter cannot be specified together with nextSyncToken.
     *   If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.
     *   Learn more about incremental synchronization.
     *   Optional. The default is to return all entries.
     */
    pub async fn list_watch(
        &self,
        max_results: i64,
        min_access_role: crate::types::MinAccessRole,
        page_token: &str,
        show_deleted: bool,
        show_hidden: bool,
        body: &crate::types::Channel,
    ) -> ClientResult<crate::Response<crate::types::Channel>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if max_results > 0 {
            query_args.push(("maxResults".to_string(), max_results.to_string()));
        }
        if !min_access_role.to_string().is_empty() {
            query_args.push(("minAccessRole".to_string(), min_access_role.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        if show_deleted {
            query_args.push(("showDeleted".to_string(), show_deleted.to_string()));
        }
        if show_hidden {
            query_args.push(("showHidden".to_string(), show_hidden.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/users/me/calendarList/watch?{}", query_), None);
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
     * This function performs a `GET` to the `/users/me/calendarList/{calendarId}` endpoint.
     *
     * Returns a calendar from the user's calendar list.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     */
    pub async fn list_get(
        &self,
        calendar_id: &str,
    ) -> ClientResult<crate::Response<crate::types::CalendarListEntry>> {
        let url = self.client.url(
            &format!(
                "/users/me/calendarList/{}",
                crate::progenitor_support::encode_path(calendar_id),
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
     * This function performs a `PUT` to the `/users/me/calendarList/{calendarId}` endpoint.
     *
     * Updates an existing calendar on the user's calendar list.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     * * `color_rgb_format: bool` -- Whether to use the foregroundColor and backgroundColor fields to write the calendar colors (RGB). If this feature is used, the index-based colorId field will be set to the best matching option automatically. Optional. The default is False.
     */
    pub async fn list_update(
        &self,
        calendar_id: &str,
        color_rgb_format: bool,
        body: &crate::types::CalendarListEntry,
    ) -> ClientResult<crate::Response<crate::types::CalendarListEntry>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if color_rgb_format {
            query_args.push(("colorRgbFormat".to_string(), color_rgb_format.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/me/calendarList/{}?{}",
                crate::progenitor_support::encode_path(calendar_id),
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
     * This function performs a `DELETE` to the `/users/me/calendarList/{calendarId}` endpoint.
     *
     * Removes a calendar from the user's calendar list.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     */
    pub async fn list_delete(&self, calendar_id: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/users/me/calendarList/{}",
                crate::progenitor_support::encode_path(calendar_id),
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
     * This function performs a `PATCH` to the `/users/me/calendarList/{calendarId}` endpoint.
     *
     * Updates an existing calendar on the user's calendar list. This method supports patch semantics.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     * * `color_rgb_format: bool` -- Whether to use the foregroundColor and backgroundColor fields to write the calendar colors (RGB). If this feature is used, the index-based colorId field will be set to the best matching option automatically. Optional. The default is False.
     */
    pub async fn list_patch(
        &self,
        calendar_id: &str,
        color_rgb_format: bool,
        body: &crate::types::CalendarListEntry,
    ) -> ClientResult<crate::Response<crate::types::CalendarListEntry>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if color_rgb_format {
            query_args.push(("colorRgbFormat".to_string(), color_rgb_format.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/me/calendarList/{}?{}",
                crate::progenitor_support::encode_path(calendar_id),
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
}
