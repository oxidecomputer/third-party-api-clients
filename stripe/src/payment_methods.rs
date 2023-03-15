use crate::Client;
use crate::ClientResult;

pub struct PaymentMethods {
    pub client: Client,
}

impl PaymentMethods {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        PaymentMethods { client }
    }

    /**
     * This function performs a `GET` to the `/v1/payment_methods` endpoint.
     *
     * <p>Returns a list of PaymentMethods. For listing a customer’s payment methods, you should use <a href="/docs/api/payment_methods/customer_list">List a Customer’s PaymentMethods</a></p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The ID of the customer whose PaymentMethods will be retrieved. If not provided, the response list will be empty.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `type_: crate::types::GetCustomersCustomerPaymentMethodsType` -- A required filter on the list, based on the object `type` field.
     */
    pub async fn get_page(
        &self,
        customer: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
        type_: crate::types::GetCustomersCustomerPaymentMethodsType,
    ) -> ClientResult<Vec<crate::types::PaymentMethod>> {
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
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/payment_methods?{}", query_), None);
        let resp: crate::types::PaymentFlowsMethodList = self
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
     * This function performs a `GET` to the `/v1/payment_methods` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of PaymentMethods. For listing a customer’s payment methods, you should use <a href="/docs/api/payment_methods/customer_list">List a Customer’s PaymentMethods</a></p>
     */
    pub async fn get_all(
        &self,
        customer: &str,
        type_: crate::types::GetCustomersCustomerPaymentMethodsType,
    ) -> ClientResult<Vec<crate::types::PaymentMethod>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/payment_methods?{}", query_), None);
        let mut resp: crate::types::PaymentFlowsMethodList = self
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
     * This function performs a `POST` to the `/v1/payment_methods` endpoint.
     *
     * <p>Creates a PaymentMethod object. Read the <a href="/docs/stripe-js/reference#stripe-create-payment-method">Stripe.js reference</a> to learn how to create PaymentMethods via Stripe.js.</p>
     *
     * <p>Instead of creating a PaymentMethod directly, we recommend using the <a href="/docs/payments/accept-a-payment">PaymentIntents</a> API to accept a payment immediately or the <a href="/docs/payments/save-and-reuse">SetupIntent</a> API to collect payment method details ahead of a future payment.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::PaymentMethod> {
        let url = self.client.url("/v1/payment_methods", None);
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
     * This function performs a `GET` to the `/v1/payment_methods/{payment_method}` endpoint.
     *
     * <p>Retrieves a PaymentMethod object.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `payment_method: &str` -- The account's country.
     */
    pub async fn get_method(
        &self,
        payment_method: &str,
    ) -> ClientResult<crate::types::PaymentMethod> {
        let url = self.client.url(
            &format!(
                "/v1/payment_methods/{}",
                crate::progenitor_support::encode_path(payment_method),
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
     * This function performs a `POST` to the `/v1/payment_methods/{payment_method}` endpoint.
     *
     * <p>Updates a PaymentMethod object. A PaymentMethod must be attached a customer to be updated.</p>
     *
     * **Parameters:**
     *
     * * `payment_method: &str` -- The account's country.
     */
    pub async fn post_method(
        &self,
        payment_method: &str,
    ) -> ClientResult<crate::types::PaymentMethod> {
        let url = self.client.url(
            &format!(
                "/v1/payment_methods/{}",
                crate::progenitor_support::encode_path(payment_method),
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
     * This function performs a `POST` to the `/v1/payment_methods/{payment_method}/attach` endpoint.
     *
     * <p>Attaches a PaymentMethod object to a Customer.</p>
     *
     * <p>To attach a new PaymentMethod to a customer for future payments, we recommend you use a <a href="/docs/api/setup_intents">SetupIntent</a>
     * or a PaymentIntent with <a href="/docs/api/payment_intents/create#create_payment_intent-setup_future_usage">setup_future_usage</a>.
     * These approaches will perform any necessary steps to ensure that the PaymentMethod can be used in a future payment. Using the
     * <code>/v1/payment_methods/:id/attach</code> endpoint does not ensure that future payments can be made with the attached PaymentMethod.
     * See <a href="/docs/payments/payment-intents#future-usage">Optimizing cards for future payments</a> for more information about setting up future payments.</p>
     *
     * <p>To use this PaymentMethod as the default for invoice or subscription payments,
     * set <a href="/docs/api/customers/update#update_customer-invoice_settings-default_payment_method"><code>invoice_settings.default_payment_method</code></a>,
     * on the Customer to the PaymentMethod’s ID.</p>
     *
     * **Parameters:**
     *
     * * `payment_method: &str` -- The account's country.
     */
    pub async fn post_method_attach(
        &self,
        payment_method: &str,
    ) -> ClientResult<crate::types::PaymentMethod> {
        let url = self.client.url(
            &format!(
                "/v1/payment_methods/{}/attach",
                crate::progenitor_support::encode_path(payment_method),
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
     * This function performs a `POST` to the `/v1/payment_methods/{payment_method}/detach` endpoint.
     *
     * <p>Detaches a PaymentMethod object from a Customer. After a PaymentMethod is detached, it can no longer be used for a payment or re-attached to a Customer.</p>
     *
     * **Parameters:**
     *
     * * `payment_method: &str` -- The account's country.
     */
    pub async fn post_method_detach(
        &self,
        payment_method: &str,
    ) -> ClientResult<crate::types::PaymentMethod> {
        let url = self.client.url(
            &format!(
                "/v1/payment_methods/{}/detach",
                crate::progenitor_support::encode_path(payment_method),
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
