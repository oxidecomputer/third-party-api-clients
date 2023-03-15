use crate::Client;
use crate::ClientResult;

pub struct PaymentIntents {
    pub client: Client,
}

impl PaymentIntents {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        PaymentIntents { client }
    }

    /**
     * This function performs a `GET` to the `/v1/payment_intents` endpoint.
     *
     * <p>Returns a list of PaymentIntents.</p>
     *
     * **Parameters:**
     *
     * * `created: &str` -- A filter on the list, based on the object `created` field. The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
     * * `customer: &str` -- Only return PaymentIntents for the customer specified by this customer ID.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_page(
        &self,
        _created: &str,
        customer: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::PaymentIntent>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
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
        let url = self
            .client
            .url(&format!("/v1/payment_intents?{}", query_), None);
        let resp: crate::types::PaymentFlowsIntentList = self
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
     * This function performs a `GET` to the `/v1/payment_intents` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of PaymentIntents.</p>
     */
    pub async fn get_all(
        &self,
        _created: &str,
        customer: &str,
    ) -> ClientResult<Vec<crate::types::PaymentIntent>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/payment_intents?{}", query_), None);
        let mut resp: crate::types::PaymentFlowsIntentList = self
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
     * This function performs a `POST` to the `/v1/payment_intents` endpoint.
     *
     * <p>Creates a PaymentIntent object.</p>
     *
     * <p>After the PaymentIntent is created, attach a payment method and <a href="/docs/api/payment_intents/confirm">confirm</a>
     * to continue the payment. You can read more about the different payment flows
     * available via the Payment Intents API <a href="/docs/payments/payment-intents">here</a>.</p>
     *
     * <p>When <code>confirm=true</code> is used during creation, it is equivalent to creating
     * and confirming the PaymentIntent in the same call. You may use any parameters
     * available in the <a href="/docs/api/payment_intents/confirm">confirm API</a> when <code>confirm=true</code>
     * is supplied.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::PaymentIntent> {
        let url = self.client.url("/v1/payment_intents", None);
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
     * This function performs a `GET` to the `/v1/payment_intents/search` endpoint.
     *
     * <p>Search for PaymentIntents you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
     * Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
     * conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
     * to an hour behind during outages. Search functionality is not available to merchants in India.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `page: &str` -- A cursor for pagination across multiple pages of results. Don't include this parameter on the first call. Use the next_page value returned in a previous response to request subsequent results.
     * * `query: &str` -- The search query string. See [search query language](https://stripe.com/docs/search#search-query-language) and the list of supported [query fields for payment intents](https://stripe.com/docs/search#query-fields-for-payment-intents).
     */
    pub async fn get_search(
        &self,
        limit: i64,
        page: &str,
        query: &str,
    ) -> ClientResult<Vec<crate::types::Charge>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page.is_empty() {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/payment_intents/search?{}", query_), None);
        let resp: crate::types::SearchResult = self
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
     * This function performs a `GET` to the `/v1/payment_intents/search` endpoint.
     *
     * As opposed to `get_search`, this function returns all the pages of the request at once.
     *
     * <p>Search for PaymentIntents you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
     * Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
     * conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
     * to an hour behind during outages. Search functionality is not available to merchants in India.</p>
     */
    pub async fn get_all_search(&self, query: &str) -> ClientResult<Vec<crate::types::Charge>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/payment_intents/search?{}", query_), None);
        let mut resp: crate::types::SearchResult = self
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
     * This function performs a `GET` to the `/v1/payment_intents/{intent}` endpoint.
     *
     * <p>Retrieves the details of a PaymentIntent that has previously been created. </p>
     *
     * <p>Client-side retrieval using a publishable key is allowed when the <code>client_secret</code> is provided in the query string. </p>
     *
     * <p>When retrieved with a publishable key, only a subset of properties will be returned. Please refer to the <a href="#payment_intent_object">payment intent</a> object reference for more details.</p>
     *
     * **Parameters:**
     *
     * * `client_secret: &str` -- The client secret of the PaymentIntent. Required if a publishable key is used to retrieve the source.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `intent: &str` -- The account's country.
     */
    pub async fn get_intent(
        &self,
        client_secret: &str,
        intent: &str,
    ) -> ClientResult<crate::types::PaymentIntent> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !client_secret.is_empty() {
            query_args.push(("client_secret".to_string(), client_secret.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v1/payment_intents/{}?{}",
                crate::progenitor_support::encode_path(intent),
                query_
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
     * This function performs a `POST` to the `/v1/payment_intents/{intent}` endpoint.
     *
     * <p>Updates properties on a PaymentIntent object without confirming.</p>
     *
     * <p>Depending on which properties you update, you may need to confirm the
     * PaymentIntent again. For example, updating the <code>payment_method</code> will
     * always require you to confirm the PaymentIntent again. If you prefer to
     * update and confirm at the same time, we recommend updating properties via
     * the <a href="/docs/api/payment_intents/confirm">confirm API</a> instead.</p>
     *
     * **Parameters:**
     *
     * * `intent: &str` -- The account's country.
     */
    pub async fn post_intent(&self, intent: &str) -> ClientResult<crate::types::PaymentIntent> {
        let url = self.client.url(
            &format!(
                "/v1/payment_intents/{}",
                crate::progenitor_support::encode_path(intent),
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
     * This function performs a `POST` to the `/v1/payment_intents/{intent}/cancel` endpoint.
     *
     * <p>A PaymentIntent object can be canceled when it is in one of these statuses: <code>requires_payment_method</code>, <code>requires_capture</code>, <code>requires_confirmation</code>, <code>requires_action</code>, or <code>processing</code>. </p>
     *
     * <p>Once canceled, no additional charges will be made by the PaymentIntent and any operations on the PaymentIntent will fail with an error. For PaymentIntents with <code>status=’requires_capture’</code>, the remaining <code>amount_capturable</code> will automatically be refunded. </p>
     *
     * <p>You cannot cancel the PaymentIntent for a Checkout Session. <a href="/docs/api/checkout/sessions/expire">Expire the Checkout Session</a> instead</p>
     *
     * **Parameters:**
     *
     * * `intent: &str` -- The account's country.
     */
    pub async fn post_intent_cancel(
        &self,
        intent: &str,
    ) -> ClientResult<crate::types::PaymentIntent> {
        let url = self.client.url(
            &format!(
                "/v1/payment_intents/{}/cancel",
                crate::progenitor_support::encode_path(intent),
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
     * This function performs a `POST` to the `/v1/payment_intents/{intent}/capture` endpoint.
     *
     * <p>Capture the funds of an existing uncaptured PaymentIntent when its status is <code>requires_capture</code>.</p>
     *
     * <p>Uncaptured PaymentIntents will be canceled a set number of days after they are created (7 by default).</p>
     *
     * <p>Learn more about <a href="/docs/payments/capture-later">separate authorization and capture</a>.</p>
     *
     * **Parameters:**
     *
     * * `intent: &str` -- The account's country.
     */
    pub async fn post_intent_capture(
        &self,
        intent: &str,
    ) -> ClientResult<crate::types::PaymentIntent> {
        let url = self.client.url(
            &format!(
                "/v1/payment_intents/{}/capture",
                crate::progenitor_support::encode_path(intent),
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
     * This function performs a `POST` to the `/v1/payment_intents/{intent}/confirm` endpoint.
     *
     * <p>Confirm that your customer intends to pay with current or provided
     * payment method. Upon confirmation, the PaymentIntent will attempt to initiate
     * a payment.</p>
     *
     * <p>If the selected payment method requires additional authentication steps, the
     * PaymentIntent will transition to the <code>requires_action</code> status and
     * suggest additional actions via <code>next_action</code>. If payment fails,
     * the PaymentIntent will transition to the <code>requires_payment_method</code> status. If
     * payment succeeds, the PaymentIntent will transition to the <code>succeeded</code>
     * status (or <code>requires_capture</code>, if <code>capture_method</code> is set to <code>manual</code>).</p>
     *
     * <p>If the <code>confirmation_method</code> is <code>automatic</code>, payment may be attempted
     * using our <a href="/docs/stripe-js/reference#stripe-handle-card-payment">client SDKs</a>
     * and the PaymentIntent’s <a href="#payment_intent_object-client_secret">client_secret</a>.
     * After <code>next_action</code>s are handled by the client, no additional
     * confirmation is required to complete the payment.</p>
     *
     * <p>If the <code>confirmation_method</code> is <code>manual</code>, all payment attempts must be
     * initiated using a secret key.
     * If any actions are required for the payment, the PaymentIntent will
     * return to the <code>requires_confirmation</code> state
     * after those actions are completed. Your server needs to then
     * explicitly re-confirm the PaymentIntent to initiate the next payment
     * attempt. Read the <a href="/docs/payments/payment-intents/web-manual">expanded documentation</a>
     * to learn more about manual confirmation.</p>
     *
     * **Parameters:**
     *
     * * `intent: &str` -- The account's country.
     */
    pub async fn post_intent_confirm(
        &self,
        intent: &str,
    ) -> ClientResult<crate::types::PaymentIntent> {
        let url = self.client.url(
            &format!(
                "/v1/payment_intents/{}/confirm",
                crate::progenitor_support::encode_path(intent),
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
     * This function performs a `POST` to the `/v1/payment_intents/{intent}/verify_microdeposits` endpoint.
     *
     * <p>Verifies microdeposits on a PaymentIntent object.</p>
     *
     * **Parameters:**
     *
     * * `intent: &str` -- The account's country.
     */
    pub async fn post_intent_verify_microdeposit(
        &self,
        intent: &str,
    ) -> ClientResult<crate::types::PaymentIntent> {
        let url = self.client.url(
            &format!(
                "/v1/payment_intents/{}/verify_microdeposits",
                crate::progenitor_support::encode_path(intent),
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
