use crate::Client;
use crate::ClientResult;

pub struct Acl {
    pub client: Client,
}

impl Acl {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Acl { client }
    }

    /**
     * This function performs a `GET` to the `/calendars/{calendarId}/acl` endpoint.
     *
     * Returns the rules in the access control list for the calendar.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     * * `max_results: i64` -- Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional.
     * * `page_token: &str` -- Token specifying which result page to return. Optional.
     * * `show_deleted: bool` -- Whether to include deleted ACLs in the result. Deleted ACLs are represented by role equal to "none". Deleted ACLs will always be included if syncToken is provided. Optional. The default is False.
     * * `sync_token: &str` -- Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. All entries deleted since the previous list request will always be in the result set and it is not allowed to set showDeleted to False.
     *   If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.
     *   Learn more about incremental synchronization.
     *   Optional. The default is to return all entries.
     */
    pub async fn list(
        &self,
        calendar_id: &str,
        max_results: i64,
        page_token: &str,
        show_deleted: bool,
    ) -> ClientResult<crate::Response<Vec<crate::types::AclRule>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if max_results > 0 {
            query_args.push(("maxResults".to_string(), max_results.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        if show_deleted {
            query_args.push(("showDeleted".to_string(), show_deleted.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/calendars/{}/acl?{}",
                crate::progenitor_support::encode_path(calendar_id),
                query_
            ),
            None,
        );
        let resp: crate::Response<crate::types::Acl> = self
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
     * This function performs a `GET` to the `/calendars/{calendarId}/acl` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Returns the rules in the access control list for the calendar.
     */
    pub async fn list_all(
        &self,
        calendar_id: &str,
        show_deleted: bool,
    ) -> ClientResult<crate::Response<Vec<crate::types::AclRule>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if show_deleted {
            query_args.push(("showDeleted".to_string(), show_deleted.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/calendars/{}/acl?{}",
                crate::progenitor_support::encode_path(calendar_id),
                query_
            ),
            None,
        );
        let crate::Response::<crate::types::Acl> {
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
                crate::Response::<crate::types::Acl> {
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
                crate::Response::<crate::types::Acl> {
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
     * This function performs a `POST` to the `/calendars/{calendarId}/acl` endpoint.
     *
     * Creates an access control rule.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     * * `send_notifications: bool` -- Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     */
    pub async fn insert(
        &self,
        calendar_id: &str,
        send_notifications: bool,
        body: &crate::types::AclRule,
    ) -> ClientResult<crate::Response<crate::types::AclRule>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if send_notifications {
            query_args.push((
                "sendNotifications".to_string(),
                send_notifications.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/calendars/{}/acl?{}",
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
     * This function performs a `POST` to the `/calendars/{calendarId}/acl/watch` endpoint.
     *
     * Watch for changes to ACL resources.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     * * `max_results: i64` -- Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional.
     * * `page_token: &str` -- Token specifying which result page to return. Optional.
     * * `show_deleted: bool` -- Whether to include deleted ACLs in the result. Deleted ACLs are represented by role equal to "none". Deleted ACLs will always be included if syncToken is provided. Optional. The default is False.
     * * `sync_token: &str` -- Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. All entries deleted since the previous list request will always be in the result set and it is not allowed to set showDeleted to False.
     *   If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.
     *   Learn more about incremental synchronization.
     *   Optional. The default is to return all entries.
     */
    pub async fn watch(
        &self,
        calendar_id: &str,
        max_results: i64,
        page_token: &str,
        show_deleted: bool,
        body: &crate::types::Channel,
    ) -> ClientResult<crate::Response<crate::types::Channel>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if max_results > 0 {
            query_args.push(("maxResults".to_string(), max_results.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        if show_deleted {
            query_args.push(("showDeleted".to_string(), show_deleted.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/calendars/{}/acl/watch?{}",
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
     * This function performs a `GET` to the `/calendars/{calendarId}/acl/{ruleId}` endpoint.
     *
     * Returns an access control rule.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     * * `rule_id: &str` -- ETag of the collection.
     */
    pub async fn get(
        &self,
        calendar_id: &str,
        rule_id: &str,
    ) -> ClientResult<crate::Response<crate::types::AclRule>> {
        let url = self.client.url(
            &format!(
                "/calendars/{}/acl/{}",
                crate::progenitor_support::encode_path(calendar_id),
                crate::progenitor_support::encode_path(rule_id),
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
     * This function performs a `PUT` to the `/calendars/{calendarId}/acl/{ruleId}` endpoint.
     *
     * Updates an access control rule.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     * * `rule_id: &str` -- ETag of the collection.
     * * `send_notifications: bool` -- Whether to send notifications about the calendar sharing change. Note that there are no notifications on access removal. Optional. The default is True.
     */
    pub async fn update(
        &self,
        calendar_id: &str,
        rule_id: &str,
        send_notifications: bool,
        body: &crate::types::AclRule,
    ) -> ClientResult<crate::Response<crate::types::AclRule>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if send_notifications {
            query_args.push((
                "sendNotifications".to_string(),
                send_notifications.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/calendars/{}/acl/{}?{}",
                crate::progenitor_support::encode_path(calendar_id),
                crate::progenitor_support::encode_path(rule_id),
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
     * This function performs a `DELETE` to the `/calendars/{calendarId}/acl/{ruleId}` endpoint.
     *
     * Deletes an access control rule.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     * * `rule_id: &str` -- ETag of the collection.
     */
    pub async fn delete(
        &self,
        calendar_id: &str,
        rule_id: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/calendars/{}/acl/{}",
                crate::progenitor_support::encode_path(calendar_id),
                crate::progenitor_support::encode_path(rule_id),
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
     * This function performs a `PATCH` to the `/calendars/{calendarId}/acl/{ruleId}` endpoint.
     *
     * Updates an access control rule. This method supports patch semantics.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     * * `rule_id: &str` -- ETag of the collection.
     * * `send_notifications: bool` -- Whether to send notifications about the calendar sharing change. Note that there are no notifications on access removal. Optional. The default is True.
     */
    pub async fn patch(
        &self,
        calendar_id: &str,
        rule_id: &str,
        send_notifications: bool,
        body: &crate::types::AclRule,
    ) -> ClientResult<crate::Response<crate::types::AclRule>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if send_notifications {
            query_args.push((
                "sendNotifications".to_string(),
                send_notifications.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/calendars/{}/acl/{}?{}",
                crate::progenitor_support::encode_path(calendar_id),
                crate::progenitor_support::encode_path(rule_id),
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
