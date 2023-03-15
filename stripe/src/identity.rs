use crate::Client;
use crate::ClientResult;

pub struct Identity {
    pub client: Client,
}

impl Identity {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Identity { client }
    }

    /**
     * This function performs a `GET` to the `/v1/identity/verification_reports` endpoint.
     *
     * <p>List all verification reports.</p>
     *
     * **Parameters:**
     *
     * * `created: &str`
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `type_: crate::types::GelatoVerificationReportType` -- Only return VerificationReports of this type.
     * * `verification_session: &str` -- Only return VerificationReports created by this VerificationSession ID. It is allowed to provide a VerificationIntent ID.
     */
    pub async fn get_verification_reports(
        &self,
        _created: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
        type_: crate::types::GelatoVerificationReportType,
        verification_session: &str,
    ) -> ClientResult<Vec<crate::types::GelatoVerificationReport>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        if !verification_session.is_empty() {
            query_args.push((
                "verification_session".to_string(),
                verification_session.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/v1/identity/verification_reports?{}", query_),
            None,
        );
        let resp: crate::types::GetIdentityVerificationReportsResponse = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await?;

        // Return our response data.
        Ok(resp.data.to_vec())
    }
    /**
     * This function performs a `GET` to the `/v1/identity/verification_reports` endpoint.
     *
     * As opposed to `get_verification_reports`, this function returns all the pages of the request at once.
     *
     * <p>List all verification reports.</p>
     */
    pub async fn get_all_verification_reports(
        &self,
        _created: &str,
        type_: crate::types::GelatoVerificationReportType,
        verification_session: &str,
    ) -> ClientResult<Vec<crate::types::GelatoVerificationReport>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        if !verification_session.is_empty() {
            query_args.push((
                "verification_session".to_string(),
                verification_session.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/v1/identity/verification_reports?{}", query_),
            None,
        );
        let mut resp: crate::types::GetIdentityVerificationReportsResponse = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut data = resp.data;
        let mut has_more = resp.has_more;
        let mut page = "".to_string();

        // Paginate if we should.
        while has_more {
            if !data.is_empty() {
                let last = data.last().unwrap();
                let j = serde_json::json!(last);
                if let serde_json::Value::Object(o) = j {
                    if let Some(serde_json::Value::String(s)) = o.get("id") {
                        page = s.to_string();
                    }
                }
            }

            if !url.contains('?') {
                resp = self
                    .client
                    .get(
                        &format!("{}?startng_after={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                resp = self
                    .client
                    .get(
                        &format!("{}&starting_after={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            data.append(&mut resp.data);

            has_more = resp.has_more;
        }

        // Return our response data.
        Ok(data.to_vec())
    }
    /**
     * This function performs a `GET` to the `/v1/identity/verification_reports/{report}` endpoint.
     *
     * <p>Retrieves an existing VerificationReport</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `report: &str` -- The account's country.
     */
    pub async fn get_verification_reports_report(
        &self,
        report: &str,
    ) -> ClientResult<crate::types::GelatoVerificationReport> {
        let url = self.client.url(
            &format!(
                "/v1/identity/verification_reports/{}",
                crate::progenitor_support::encode_path(report),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/v1/identity/verification_sessions` endpoint.
     *
     * <p>Returns a list of VerificationSessions</p>
     *
     * **Parameters:**
     *
     * * `created: &str`
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `status: crate::types::GelatoVerificationSessionStatus` -- Only return VerificationSessions with this status. [Learn more about the lifecycle of sessions](https://stripe.com/docs/identity/how-sessions-work).
     */
    pub async fn get_verification_sessions(
        &self,
        _created: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
        status: crate::types::GelatoVerificationSessionStatus,
    ) -> ClientResult<Vec<crate::types::GelatoVerificationSession>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/v1/identity/verification_sessions?{}", query_),
            None,
        );
        let resp: crate::types::GetIdentityVerificationSessionsResponse = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await?;

        // Return our response data.
        Ok(resp.data.to_vec())
    }
    /**
     * This function performs a `GET` to the `/v1/identity/verification_sessions` endpoint.
     *
     * As opposed to `get_verification_sessions`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of VerificationSessions</p>
     */
    pub async fn get_all_verification_sessions(
        &self,
        _created: &str,
        status: crate::types::GelatoVerificationSessionStatus,
    ) -> ClientResult<Vec<crate::types::GelatoVerificationSession>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/v1/identity/verification_sessions?{}", query_),
            None,
        );
        let mut resp: crate::types::GetIdentityVerificationSessionsResponse = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut data = resp.data;
        let mut has_more = resp.has_more;
        let mut page = "".to_string();

        // Paginate if we should.
        while has_more {
            if !data.is_empty() {
                let last = data.last().unwrap();
                let j = serde_json::json!(last);
                if let serde_json::Value::Object(o) = j {
                    if let Some(serde_json::Value::String(s)) = o.get("id") {
                        page = s.to_string();
                    }
                }
            }

            if !url.contains('?') {
                resp = self
                    .client
                    .get(
                        &format!("{}?startng_after={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                resp = self
                    .client
                    .get(
                        &format!("{}&starting_after={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            data.append(&mut resp.data);

            has_more = resp.has_more;
        }

        // Return our response data.
        Ok(data.to_vec())
    }
    /**
     * This function performs a `POST` to the `/v1/identity/verification_sessions` endpoint.
     *
     * <p>Creates a VerificationSession object.</p>
     *
     * <p>After the VerificationSession is created, display a verification modal using the session <code>client_secret</code> or send your users to the session’s <code>url</code>.</p>
     *
     * <p>If your API key is in test mode, verification checks won’t actually process, though everything else will occur as if in live mode.</p>
     *
     * <p>Related guide: <a href="/docs/identity/verify-identity-documents">Verify your users’ identity documents</a>.</p>
     */
    pub async fn post_verification_session(
        &self,
    ) -> ClientResult<crate::types::GelatoVerificationSession> {
        let url = self.client.url("/v1/identity/verification_sessions", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/v1/identity/verification_sessions/{session}` endpoint.
     *
     * <p>Retrieves the details of a VerificationSession that was previously created.</p>
     *
     * <p>When the session status is <code>requires_input</code>, you can use this method to retrieve a valid
     * <code>client_secret</code> or <code>url</code> to allow re-submission.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `session: &str` -- The account's country.
     */
    pub async fn get_verification_sessions_session(
        &self,
        session: &str,
    ) -> ClientResult<crate::types::GelatoVerificationSession> {
        let url = self.client.url(
            &format!(
                "/v1/identity/verification_sessions/{}",
                crate::progenitor_support::encode_path(session),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/v1/identity/verification_sessions/{session}` endpoint.
     *
     * <p>Updates a VerificationSession object.</p>
     *
     * <p>When the session status is <code>requires_input</code>, you can use this method to update the
     * verification check and options.</p>
     *
     * **Parameters:**
     *
     * * `session: &str` -- The account's country.
     */
    pub async fn post_verification_sessions_session(
        &self,
        session: &str,
    ) -> ClientResult<crate::types::GelatoVerificationSession> {
        let url = self.client.url(
            &format!(
                "/v1/identity/verification_sessions/{}",
                crate::progenitor_support::encode_path(session),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/v1/identity/verification_sessions/{session}/cancel` endpoint.
     *
     * <p>A VerificationSession object can be canceled when it is in <code>requires_input</code> <a href="/docs/identity/how-sessions-work">status</a>.</p>
     *
     * <p>Once canceled, future submission attempts are disabled. This cannot be undone. <a href="/docs/identity/verification-sessions#cancel">Learn more</a>.</p>
     *
     * **Parameters:**
     *
     * * `session: &str` -- The account's country.
     */
    pub async fn post_verification_sessions_session_cancel(
        &self,
        session: &str,
    ) -> ClientResult<crate::types::GelatoVerificationSession> {
        let url = self.client.url(
            &format!(
                "/v1/identity/verification_sessions/{}/cancel",
                crate::progenitor_support::encode_path(session),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/v1/identity/verification_sessions/{session}/redact` endpoint.
     *
     * <p>Redact a VerificationSession to remove all collected information from Stripe. This will redact
     * the VerificationSession and all objects related to it, including VerificationReports, Events,
     * request logs, etc.</p>
     *
     * <p>A VerificationSession object can be redacted when it is in <code>requires_input</code> or <code>verified</code>
     * <a href="/docs/identity/how-sessions-work">status</a>. Redacting a VerificationSession in <code>requires_action</code>
     * state will automatically cancel it.</p>
     *
     * <p>The redaction process may take up to four days. When the redaction process is in progress, the
     * VerificationSession’s <code>redaction.status</code> field will be set to <code>processing</code>; when the process is
     * finished, it will change to <code>redacted</code> and an <code>identity.verification_session.redacted</code> event
     * will be emitted.</p>
     *
     * <p>Redaction is irreversible. Redacted objects are still accessible in the Stripe API, but all the
     * fields that contain personal data will be replaced by the string <code>[redacted]</code> or a similar
     * placeholder. The <code>metadata</code> field will also be erased. Redacted objects cannot be updated or
     * used for any purpose.</p>
     *
     * <p><a href="/docs/identity/verification-sessions#redact">Learn more</a>.</p>
     *
     * **Parameters:**
     *
     * * `session: &str` -- The account's country.
     */
    pub async fn post_verification_sessions_session_redact(
        &self,
        session: &str,
    ) -> ClientResult<crate::types::GelatoVerificationSession> {
        let url = self.client.url(
            &format!(
                "/v1/identity/verification_sessions/{}/redact",
                crate::progenitor_support::encode_path(session),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
}
