use crate::Client;
use crate::ClientResult;

pub struct SetupIntents {
    pub client: Client,
}

impl SetupIntents {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SetupIntents { client }
    }

    /**
     * This function performs a `GET` to the `/v1/setup_intents` endpoint.
     *
     * <p>Returns a list of SetupIntents.</p>
     *
     * **Parameters:**
     *
     * * `created: &str` -- A filter on the list, based on the object `created` field. The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
     * * `customer: &str` -- Only return SetupIntents for the customer specified by this customer ID.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `payment_method: &str` -- Only return SetupIntents associated with the specified payment method.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_page(
        &self,
        _created: &str,
        customer: &str,
        ending_before: &str,
        limit: i64,
        payment_method: &str,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::SetupIntent>> {
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
        if !payment_method.is_empty() {
            query_args.push(("payment_method".to_string(), payment_method.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/setup_intents?{}", query_), None);
        let resp: crate::types::PaymentFlowsSetupIntentList = self
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
     * This function performs a `GET` to the `/v1/setup_intents` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of SetupIntents.</p>
     */
    pub async fn get_all(
        &self,
        _created: &str,
        customer: &str,
        payment_method: &str,
    ) -> ClientResult<Vec<crate::types::SetupIntent>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !payment_method.is_empty() {
            query_args.push(("payment_method".to_string(), payment_method.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/setup_intents?{}", query_), None);
        let mut resp: crate::types::PaymentFlowsSetupIntentList = self
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
     * This function performs a `POST` to the `/v1/setup_intents` endpoint.
     *
     * <p>Creates a SetupIntent object.</p>
     *
     * <p>After the SetupIntent is created, attach a payment method and <a href="/docs/api/setup_intents/confirm">confirm</a>
     * to collect any required permissions to charge the payment method later.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::SetupIntent> {
        let url = self.client.url("/v1/setup_intents", None);
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
     * This function performs a `GET` to the `/v1/setup_intents/{intent}` endpoint.
     *
     * <p>Retrieves the details of a SetupIntent that has previously been created. </p>
     *
     * <p>Client-side retrieval using a publishable key is allowed when the <code>client_secret</code> is provided in the query string. </p>
     *
     * <p>When retrieved with a publishable key, only a subset of properties will be returned. Please refer to the <a href="#setup_intent_object">SetupIntent</a> object reference for more details.</p>
     *
     * **Parameters:**
     *
     * * `client_secret: &str` -- The client secret of the SetupIntent. Required if a publishable key is used to retrieve the SetupIntent.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `intent: &str` -- The account's country.
     */
    pub async fn get_intent(
        &self,
        client_secret: &str,
        intent: &str,
    ) -> ClientResult<crate::types::SetupIntent> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !client_secret.is_empty() {
            query_args.push(("client_secret".to_string(), client_secret.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v1/setup_intents/{}?{}",
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
     * This function performs a `POST` to the `/v1/setup_intents/{intent}` endpoint.
     *
     * <p>Updates a SetupIntent object.</p>
     *
     * **Parameters:**
     *
     * * `intent: &str` -- The account's country.
     */
    pub async fn post_intent(&self, intent: &str) -> ClientResult<crate::types::SetupIntent> {
        let url = self.client.url(
            &format!(
                "/v1/setup_intents/{}",
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
     * This function performs a `POST` to the `/v1/setup_intents/{intent}/cancel` endpoint.
     *
     * <p>A SetupIntent object can be canceled when it is in one of these statuses: <code>requires_payment_method</code>, <code>requires_confirmation</code>, or <code>requires_action</code>. </p>
     *
     * <p>Once canceled, setup is abandoned and any operations on the SetupIntent will fail with an error.</p>
     *
     * **Parameters:**
     *
     * * `intent: &str` -- The account's country.
     */
    pub async fn post_intent_cancel(
        &self,
        intent: &str,
    ) -> ClientResult<crate::types::SetupIntent> {
        let url = self.client.url(
            &format!(
                "/v1/setup_intents/{}/cancel",
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
     * This function performs a `POST` to the `/v1/setup_intents/{intent}/confirm` endpoint.
     *
     * <p>Confirm that your customer intends to set up the current or
     * provided payment method. For example, you would confirm a SetupIntent
     * when a customer hits the “Save” button on a payment method management
     * page on your website.</p>
     *
     * <p>If the selected payment method does not require any additional
     * steps from the customer, the SetupIntent will transition to the
     * <code>succeeded</code> status.</p>
     *
     * <p>Otherwise, it will transition to the <code>requires_action</code> status and
     * suggest additional actions via <code>next_action</code>. If setup fails,
     * the SetupIntent will transition to the
     * <code>requires_payment_method</code> status.</p>
     *
     * **Parameters:**
     *
     * * `intent: &str` -- The account's country.
     */
    pub async fn post_intent_confirm(
        &self,
        intent: &str,
    ) -> ClientResult<crate::types::SetupIntent> {
        let url = self.client.url(
            &format!(
                "/v1/setup_intents/{}/confirm",
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
     * This function performs a `POST` to the `/v1/setup_intents/{intent}/verify_microdeposits` endpoint.
     *
     * <p>Verifies microdeposits on a SetupIntent object.</p>
     *
     * **Parameters:**
     *
     * * `intent: &str` -- The account's country.
     */
    pub async fn post_intent_verify_microdeposit(
        &self,
        intent: &str,
    ) -> ClientResult<crate::types::SetupIntent> {
        let url = self.client.url(
            &format!(
                "/v1/setup_intents/{}/verify_microdeposits",
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
