use crate::Client;
use crate::ClientResult;

pub struct Charges {
    pub client: Client,
}

impl Charges {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Charges { client }
    }

    /**
     * This function performs a `GET` to the `/v1/charges` endpoint.
     *
     * <p>Returns a list of charges you’ve previously created. The charges are returned in sorted order, with the most recent charges appearing first.</p>
     *
     * **Parameters:**
     *
     * * `created: &str`
     * * `customer: &str` -- Only return charges for the customer specified by this customer ID.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `payment_intent: &str` -- Only return charges that were created by the PaymentIntent specified by this PaymentIntent ID.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `transfer_group: &str` -- Only return charges for this transfer group.
     */
    pub async fn get_page(
        &self,
        _created: &str,
        customer: &str,
        ending_before: &str,
        limit: i64,
        payment_intent: &str,
        starting_after: &str,
        transfer_group: &str,
    ) -> ClientResult<Vec<crate::types::Charge>> {
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
        if !payment_intent.is_empty() {
            query_args.push(("payment_intent".to_string(), payment_intent.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !transfer_group.is_empty() {
            query_args.push(("transfer_group".to_string(), transfer_group.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/charges?{}", query_), None);
        let resp: crate::types::Charges = self
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
     * This function performs a `GET` to the `/v1/charges` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of charges you’ve previously created. The charges are returned in sorted order, with the most recent charges appearing first.</p>
     */
    pub async fn get_all(
        &self,
        _created: &str,
        customer: &str,
        payment_intent: &str,
        transfer_group: &str,
    ) -> ClientResult<Vec<crate::types::Charge>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !payment_intent.is_empty() {
            query_args.push(("payment_intent".to_string(), payment_intent.to_string()));
        }
        if !transfer_group.is_empty() {
            query_args.push(("transfer_group".to_string(), transfer_group.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/charges?{}", query_), None);
        let mut resp: crate::types::Charges = self
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
     * This function performs a `POST` to the `/v1/charges` endpoint.
     *
     * <p>To charge a credit card or other payment source, you create a <code>Charge</code> object. If your API key is in test mode, the supplied payment source (e.g., card) won’t actually be charged, although everything else will occur as if in live mode. (Stripe assumes that the charge would have completed successfully).</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::Charge> {
        let url = self.client.url("/v1/charges", None);
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
     * This function performs a `GET` to the `/v1/charges/search` endpoint.
     *
     * <p>Search for charges you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
     * Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
     * conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
     * to an hour behind during outages. Search functionality is not available to merchants in India.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `page: &str` -- A cursor for pagination across multiple pages of results. Don't include this parameter on the first call. Use the next_page value returned in a previous response to request subsequent results.
     * * `query: &str` -- The search query string. See [search query language](https://stripe.com/docs/search#search-query-language) and the list of supported [query fields for charges](https://stripe.com/docs/search#query-fields-for-charges).
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
            .url(&format!("/v1/charges/search?{}", query_), None);
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
     * This function performs a `GET` to the `/v1/charges/search` endpoint.
     *
     * As opposed to `get_search`, this function returns all the pages of the request at once.
     *
     * <p>Search for charges you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
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
            .url(&format!("/v1/charges/search?{}", query_), None);
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
     * This function performs a `GET` to the `/v1/charges/{charge}` endpoint.
     *
     * <p>Retrieves the details of a charge that has previously been created. Supply the unique charge ID that was returned from your previous request, and Stripe will return the corresponding charge information. The same information is returned when creating or refunding the charge.</p>
     *
     * **Parameters:**
     *
     * * `charge: &str` -- The account's country.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     */
    pub async fn get(&self, charge: &str) -> ClientResult<crate::types::Charge> {
        let url = self.client.url(
            &format!(
                "/v1/charges/{}",
                crate::progenitor_support::encode_path(charge),
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
     * This function performs a `POST` to the `/v1/charges/{charge}` endpoint.
     *
     * <p>Updates the specified charge by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
     *
     * **Parameters:**
     *
     * * `charge: &str` -- The account's country.
     */
    pub async fn post_charges(&self, charge: &str) -> ClientResult<crate::types::Charge> {
        let url = self.client.url(
            &format!(
                "/v1/charges/{}",
                crate::progenitor_support::encode_path(charge),
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
     * This function performs a `POST` to the `/v1/charges/{charge}/capture` endpoint.
     *
     * <p>Capture the payment of an existing, uncaptured, charge. This is the second half of the two-step payment flow, where first you <a href="#create_charge">created a charge</a> with the capture option set to false.</p>
     *
     * <p>Uncaptured payments expire a set number of days after they are created (<a href="/docs/charges/placing-a-hold">7 by default</a>). If they are not captured by that point in time, they will be marked as refunded and will no longer be capturable.</p>
     *
     * **Parameters:**
     *
     * * `charge: &str` -- The account's country.
     */
    pub async fn post_capture(&self, charge: &str) -> ClientResult<crate::types::Charge> {
        let url = self.client.url(
            &format!(
                "/v1/charges/{}/capture",
                crate::progenitor_support::encode_path(charge),
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
     * This function performs a `GET` to the `/v1/charges/{charge}/dispute` endpoint.
     *
     * <p>Retrieve a dispute for a specified charge.</p>
     *
     * **Parameters:**
     *
     * * `charge: &str` -- The account's country.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     */
    pub async fn get_dispute(&self, charge: &str) -> ClientResult<crate::types::Dispute> {
        let url = self.client.url(
            &format!(
                "/v1/charges/{}/dispute",
                crate::progenitor_support::encode_path(charge),
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
     * This function performs a `POST` to the `/v1/charges/{charge}/dispute` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `charge: &str` -- The account's country.
     */
    pub async fn post_dispute(&self, charge: &str) -> ClientResult<crate::types::Dispute> {
        let url = self.client.url(
            &format!(
                "/v1/charges/{}/dispute",
                crate::progenitor_support::encode_path(charge),
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
     * This function performs a `POST` to the `/v1/charges/{charge}/dispute/close` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `charge: &str` -- The account's country.
     */
    pub async fn post_dispute_close(&self, charge: &str) -> ClientResult<crate::types::Dispute> {
        let url = self.client.url(
            &format!(
                "/v1/charges/{}/dispute/close",
                crate::progenitor_support::encode_path(charge),
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
     * This function performs a `POST` to the `/v1/charges/{charge}/refund` endpoint.
     *
     * <p>When you create a new refund, you must specify a Charge or a PaymentIntent object on which to create it.</p>
     *
     * <p>Creating a new refund will refund a charge that has previously been created but not yet refunded.
     * Funds will be refunded to the credit or debit card that was originally charged.</p>
     *
     * <p>You can optionally refund only part of a charge.
     * You can do so multiple times, until the entire charge has been refunded.</p>
     *
     * <p>Once entirely refunded, a charge can’t be refunded again.
     * This method will raise an error when called on an already-refunded charge,
     * or when trying to refund more money than is left on a charge.</p>
     *
     * **Parameters:**
     *
     * * `charge: &str` -- The account's country.
     */
    pub async fn post_refund(&self, charge: &str) -> ClientResult<crate::types::Charge> {
        let url = self.client.url(
            &format!(
                "/v1/charges/{}/refund",
                crate::progenitor_support::encode_path(charge),
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
     * This function performs a `GET` to the `/v1/charges/{charge}/refunds` endpoint.
     *
     * <p>You can see a list of the refunds belonging to a specific charge. Note that the 10 most recent refunds are always available by default on the charge object. If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional refunds.</p>
     *
     * **Parameters:**
     *
     * * `charge: &str` -- The account's country.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_refunds(
        &self,
        charge: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::Refund>> {
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
                "/v1/charges/{}/refunds?{}",
                crate::progenitor_support::encode_path(charge),
                query_
            ),
            None,
        );
        let resp: crate::types::RefundList = self
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
     * This function performs a `GET` to the `/v1/charges/{charge}/refunds` endpoint.
     *
     * As opposed to `get_refunds`, this function returns all the pages of the request at once.
     *
     * <p>You can see a list of the refunds belonging to a specific charge. Note that the 10 most recent refunds are always available by default on the charge object. If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional refunds.</p>
     */
    pub async fn get_all_refunds(&self, charge: &str) -> ClientResult<Vec<crate::types::Refund>> {
        let url = self.client.url(
            &format!(
                "/v1/charges/{}/refunds",
                crate::progenitor_support::encode_path(charge),
            ),
            None,
        );
        let mut resp: crate::types::RefundList = self
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
     * This function performs a `POST` to the `/v1/charges/{charge}/refunds` endpoint.
     *
     * <p>Create a refund.</p>
     *
     * **Parameters:**
     *
     * * `charge: &str` -- The account's country.
     */
    pub async fn post_refund_charges(&self, charge: &str) -> ClientResult<crate::types::Refund> {
        let url = self.client.url(
            &format!(
                "/v1/charges/{}/refunds",
                crate::progenitor_support::encode_path(charge),
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
     * This function performs a `GET` to the `/v1/charges/{charge}/refunds/{refund}` endpoint.
     *
     * <p>Retrieves the details of an existing refund.</p>
     *
     * **Parameters:**
     *
     * * `charge: &str` -- The account's country.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `refund: &str` -- The account's country.
     */
    pub async fn get_refunds_refund(
        &self,
        charge: &str,
        refund: &str,
    ) -> ClientResult<crate::types::Refund> {
        let url = self.client.url(
            &format!(
                "/v1/charges/{}/refunds/{}",
                crate::progenitor_support::encode_path(charge),
                crate::progenitor_support::encode_path(refund),
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
     * This function performs a `POST` to the `/v1/charges/{charge}/refunds/{refund}` endpoint.
     *
     * <p>Update a specified refund.</p>
     *
     * **Parameters:**
     *
     * * `charge: &str` -- The account's country.
     * * `refund: &str` -- The account's country.
     */
    pub async fn post_refunds_refund(
        &self,
        charge: &str,
        refund: &str,
    ) -> ClientResult<crate::types::Refund> {
        let url = self.client.url(
            &format!(
                "/v1/charges/{}/refunds/{}",
                crate::progenitor_support::encode_path(charge),
                crate::progenitor_support::encode_path(refund),
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
