use crate::Client;
use crate::ClientResult;

pub struct Checkout {
    pub client: Client,
}

impl Checkout {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Checkout { client }
    }

    /**
     * This function performs a `GET` to the `/v1/checkout/sessions` endpoint.
     *
     * <p>Returns a list of Checkout Sessions.</p>
     *
     * **Parameters:**
     *
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `payment_intent: &str` -- Only return the Checkout Session for the PaymentIntent specified.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `subscription: &str` -- Only return the Checkout Session for the subscription specified.
     */
    pub async fn get_sessions(
        &self,
        ending_before: &str,
        limit: i64,
        payment_intent: &str,
        starting_after: &str,
        subscription: &str,
    ) -> ClientResult<Vec<crate::types::Session>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !payment_intent.is_empty() {
            query_args.push(("payment_intent".to_string(), payment_intent.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !subscription.is_empty() {
            query_args.push(("subscription".to_string(), subscription.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/checkout/sessions?{}", query_), None);
        let resp: crate::types::PaymentPagesCheckoutSessionList = self
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
     * This function performs a `GET` to the `/v1/checkout/sessions` endpoint.
     *
     * As opposed to `get_sessions`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of Checkout Sessions.</p>
     */
    pub async fn get_all_sessions(
        &self,
        payment_intent: &str,
        subscription: &str,
    ) -> ClientResult<Vec<crate::types::Session>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !payment_intent.is_empty() {
            query_args.push(("payment_intent".to_string(), payment_intent.to_string()));
        }
        if !subscription.is_empty() {
            query_args.push(("subscription".to_string(), subscription.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/checkout/sessions?{}", query_), None);
        let mut resp: crate::types::PaymentPagesCheckoutSessionList = self
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
     * This function performs a `POST` to the `/v1/checkout/sessions` endpoint.
     *
     * <p>Creates a Session object.</p>
     */
    pub async fn post_session(&self) -> ClientResult<crate::types::Session> {
        let url = self.client.url("/v1/checkout/sessions", None);
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
     * This function performs a `GET` to the `/v1/checkout/sessions/{session}` endpoint.
     *
     * <p>Retrieves a Session object.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `session: &str` -- The account's country.
     */
    pub async fn get_sessions_session(&self, session: &str) -> ClientResult<crate::types::Session> {
        let url = self.client.url(
            &format!(
                "/v1/checkout/sessions/{}",
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
     * This function performs a `POST` to the `/v1/checkout/sessions/{session}/expire` endpoint.
     *
     * <p>A Session can be expired when it is in one of these statuses: <code>open</code> </p>
     *
     * <p>After it expires, a customer can’t complete a Session and customers loading the Session see a message saying the Session is expired.</p>
     *
     * **Parameters:**
     *
     * * `session: &str` -- The account's country.
     */
    pub async fn post_sessions_session_expire(
        &self,
        session: &str,
    ) -> ClientResult<crate::types::Session> {
        let url = self.client.url(
            &format!(
                "/v1/checkout/sessions/{}/expire",
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
     * This function performs a `GET` to the `/v1/checkout/sessions/{session}/line_items` endpoint.
     *
     * <p>When retrieving a Checkout Session, there is an includable <strong>line_items</strong> property containing the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
     *
     * **Parameters:**
     *
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `session: &str` -- The account's country.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_sessions_session_line_items(
        &self,
        ending_before: &str,
        limit: i64,
        session: &str,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::Item>> {
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
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v1/checkout/sessions/{}/line_items?{}",
                crate::progenitor_support::encode_path(session),
                query_
            ),
            None,
        );
        let resp: crate::types::LineItems = self
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
     * This function performs a `GET` to the `/v1/checkout/sessions/{session}/line_items` endpoint.
     *
     * As opposed to `get_sessions_session_line_items`, this function returns all the pages of the request at once.
     *
     * <p>When retrieving a Checkout Session, there is an includable <strong>line_items</strong> property containing the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
     */
    pub async fn get_all_sessions_session_line_items(
        &self,
        session: &str,
    ) -> ClientResult<Vec<crate::types::Item>> {
        let url = self.client.url(
            &format!(
                "/v1/checkout/sessions/{}/line_items",
                crate::progenitor_support::encode_path(session),
            ),
            None,
        );
        let mut resp: crate::types::LineItems = self
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
}
