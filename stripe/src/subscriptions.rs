use crate::Client;
use crate::ClientResult;

pub struct Subscriptions {
    pub client: Client,
}

impl Subscriptions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Subscriptions { client }
    }

    /**
     * This function performs a `GET` to the `/v1/subscriptions` endpoint.
     *
     * <p>By default, returns a list of subscriptions that have not been canceled. In order to list canceled subscriptions, specify <code>status=canceled</code>.</p>
     *
     * **Parameters:**
     *
     * * `collection_method: crate::types::CollectionMethod` -- Either `charge_automatically`, or `send_invoice`. When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer. When sending an invoice, Stripe will email this invoice to the customer with payment instructions.
     * * `created: &str`
     * * `current_period_end: &str`
     * * `current_period_start: &str`
     * * `customer: &str` -- The ID of the customer whose subscriptions will be retrieved.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `price: &str` -- Filter for subscriptions that contain this recurring price ID.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `status: crate::types::GetSubscriptionsStatus` -- The status of the subscriptions to retrieve. Passing in a value of `canceled` will return all canceled subscriptions, including those belonging to deleted customers. Pass `ended` to find subscriptions that are canceled and subscriptions that are expired due to [incomplete payment](https://stripe.com/docs/billing/subscriptions/overview#subscription-statuses). Passing in a value of `all` will return subscriptions of all statuses. If no value is supplied, all subscriptions that have not been canceled are returned.
     * * `test_clock: &str` -- Filter for subscriptions that are associated with the specified test clock. The response will not include subscriptions with test clocks if this and the customer parameter is not set.
     */
    pub async fn get_page(
        &self,
        collection_method: crate::types::CollectionMethod,
        _created: &str,
        _current_period_end: &str,
        _current_period_start: &str,
        customer: &str,
        ending_before: &str,
        limit: i64,
        price: &str,
        starting_after: &str,
        status: crate::types::GetSubscriptionsStatus,
        test_clock: &str,
    ) -> ClientResult<Vec<crate::types::Subscription>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !collection_method.to_string().is_empty() {
            query_args.push((
                "collection_method".to_string(),
                collection_method.to_string(),
            ));
        }
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !price.is_empty() {
            query_args.push(("price".to_string(), price.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !test_clock.is_empty() {
            query_args.push(("test_clock".to_string(), test_clock.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/subscriptions?{}", query_), None);
        let resp: crate::types::Subscriptions = self
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
     * This function performs a `GET` to the `/v1/subscriptions` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>By default, returns a list of subscriptions that have not been canceled. In order to list canceled subscriptions, specify <code>status=canceled</code>.</p>
     */
    pub async fn get_all(
        &self,
        collection_method: crate::types::CollectionMethod,
        _created: &str,
        _current_period_end: &str,
        _current_period_start: &str,
        customer: &str,
        price: &str,
        status: crate::types::GetSubscriptionsStatus,
        test_clock: &str,
    ) -> ClientResult<Vec<crate::types::Subscription>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !collection_method.to_string().is_empty() {
            query_args.push((
                "collection_method".to_string(),
                collection_method.to_string(),
            ));
        }
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !price.is_empty() {
            query_args.push(("price".to_string(), price.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !test_clock.is_empty() {
            query_args.push(("test_clock".to_string(), test_clock.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/subscriptions?{}", query_), None);
        let mut resp: crate::types::Subscriptions = self
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
     * This function performs a `POST` to the `/v1/subscriptions` endpoint.
     *
     * <p>Creates a new subscription on an existing customer. Each customer can have up to 500 active or scheduled subscriptions.</p>
     *
     * <p>When you create a subscription with <code>collection_method=charge_automatically</code>, the first invoice is finalized as part of the request.
     * The <code>payment_behavior</code> parameter determines the exact behavior of the initial payment.</p>
     *
     * <p>To start subscriptions where the first invoice always begins in a <code>draft</code> status, use <a href="/docs/billing/subscriptions/subscription-schedules#managing">subscription schedules</a> instead.
     * Schedules provide the flexibility to model more complex billing configurations that change over time.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::Subscription> {
        let url = self.client.url("/v1/subscriptions", None);
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
     * This function performs a `GET` to the `/v1/subscriptions/search` endpoint.
     *
     * <p>Search for subscriptions you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
     * Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
     * conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
     * to an hour behind during outages. Search functionality is not available to merchants in India.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `page: &str` -- A cursor for pagination across multiple pages of results. Don't include this parameter on the first call. Use the next_page value returned in a previous response to request subsequent results.
     * * `query: &str` -- The search query string. See [search query language](https://stripe.com/docs/search#search-query-language) and the list of supported [query fields for subscriptions](https://stripe.com/docs/search#query-fields-for-subscriptions).
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
            .url(&format!("/v1/subscriptions/search?{}", query_), None);
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
     * This function performs a `GET` to the `/v1/subscriptions/search` endpoint.
     *
     * As opposed to `get_search`, this function returns all the pages of the request at once.
     *
     * <p>Search for subscriptions you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
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
            .url(&format!("/v1/subscriptions/search?{}", query_), None);
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
     * This function performs a `GET` to the `/v1/subscriptions/{subscription_exposed_id}` endpoint.
     *
     * <p>Retrieves the subscription with the given ID.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `subscription_exposed_id: &str` -- The account's country.
     */
    pub async fn get_exposed(
        &self,
        subscription_exposed_id: &str,
    ) -> ClientResult<crate::types::Subscription> {
        let url = self.client.url(
            &format!(
                "/v1/subscriptions/{}",
                crate::progenitor_support::encode_path(subscription_exposed_id),
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
     * This function performs a `POST` to the `/v1/subscriptions/{subscription_exposed_id}` endpoint.
     *
     * <p>Updates an existing subscription on a customer to match the specified parameters. When changing plans or quantities, we will optionally prorate the price we charge next month to make up for any price changes. To preview how the proration will be calculated, use the <a href="#upcoming_invoice">upcoming invoice</a> endpoint.</p>
     *
     * **Parameters:**
     *
     * * `subscription_exposed_id: &str` -- The account's country.
     */
    pub async fn post_exposed(
        &self,
        subscription_exposed_id: &str,
    ) -> ClientResult<crate::types::Subscription> {
        let url = self.client.url(
            &format!(
                "/v1/subscriptions/{}",
                crate::progenitor_support::encode_path(subscription_exposed_id),
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
     * This function performs a `DELETE` to the `/v1/subscriptions/{subscription_exposed_id}` endpoint.
     *
     * <p>Cancels a customer’s subscription immediately. The customer will not be charged again for the subscription.</p>
     *
     * <p>Note, however, that any pending invoice items that you’ve created will still be charged for at the end of the period, unless manually <a href="#delete_invoiceitem">deleted</a>. If you’ve set the subscription to cancel at the end of the period, any pending prorations will also be left in place and collected at the end of the period. But if the subscription is set to cancel immediately, pending prorations will be removed.</p>
     *
     * <p>By default, upon subscription cancellation, Stripe will stop automatic collection of all finalized invoices for the customer. This is intended to prevent unexpected payment attempts after the customer has canceled a subscription. However, you can resume automatic collection of the invoices manually after subscription cancellation to have us proceed. Or, you could check for unpaid invoices before allowing the customer to cancel the subscription at all.</p>
     *
     * **Parameters:**
     *
     * * `subscription_exposed_id: &str` -- The account's country.
     */
    pub async fn delete_exposed(
        &self,
        subscription_exposed_id: &str,
    ) -> ClientResult<crate::types::Subscription> {
        let url = self.client.url(
            &format!(
                "/v1/subscriptions/{}",
                crate::progenitor_support::encode_path(subscription_exposed_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `DELETE` to the `/v1/subscriptions/{subscription_exposed_id}/discount` endpoint.
     *
     * <p>Removes the currently applied discount on a subscription.</p>
     *
     * **Parameters:**
     *
     * * `subscription_exposed_id: &str` -- The account's country.
     */
    pub async fn delete_exposed_discount(
        &self,
        subscription_exposed_id: &str,
    ) -> ClientResult<crate::types::DeletedDiscount> {
        let url = self.client.url(
            &format!(
                "/v1/subscriptions/{}/discount",
                crate::progenitor_support::encode_path(subscription_exposed_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
}
