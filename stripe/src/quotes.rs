use crate::Client;
use crate::ClientResult;

pub struct Quotes {
    pub client: Client,
}

impl Quotes {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Quotes { client }
    }

    /**
     * This function performs a `GET` to the `/v1/quotes` endpoint.
     *
     * <p>Returns a list of your quotes.</p>
     *
     * **Parameters:**
     *
     * * `customer: &str` -- The ID of the customer whose quotes will be retrieved.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `status: crate::types::QuoteStatus` -- The status of the quote.
     * * `test_clock: &str` -- Provides a list of quotes that are associated with the specified test clock. The response will not include quotes with test clocks if this and the customer parameter is not set.
     */
    pub async fn get_page(
        &self,
        customer: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
        status: crate::types::QuoteStatus,
        test_clock: &str,
    ) -> ClientResult<Vec<crate::types::Quote>> {
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
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !test_clock.is_empty() {
            query_args.push(("test_clock".to_string(), test_clock.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/quotes?{}", query_), None);
        let resp: crate::types::GetQuotesResponse = self
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
     * This function performs a `GET` to the `/v1/quotes` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of your quotes.</p>
     */
    pub async fn get_all(
        &self,
        customer: &str,
        status: crate::types::QuoteStatus,
        test_clock: &str,
    ) -> ClientResult<Vec<crate::types::Quote>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !test_clock.is_empty() {
            query_args.push(("test_clock".to_string(), test_clock.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/quotes?{}", query_), None);
        let mut resp: crate::types::GetQuotesResponse = self
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
     * This function performs a `POST` to the `/v1/quotes` endpoint.
     *
     * <p>A quote models prices and services for a customer. Default options for <code>header</code>, <code>description</code>, <code>footer</code>, and <code>expires_at</code> can be set in the dashboard via the <a href="https://dashboard.stripe.com/settings/billing/quote">quote template</a>.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::Quote> {
        let url = self.client.url("/v1/quotes", None);
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
     * This function performs a `GET` to the `/v1/quotes/{quote}` endpoint.
     *
     * <p>Retrieves the quote with the given ID.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `quote: &str` -- The account's country.
     */
    pub async fn get(&self, quote: &str) -> ClientResult<crate::types::Quote> {
        let url = self.client.url(
            &format!(
                "/v1/quotes/{}",
                crate::progenitor_support::encode_path(quote),
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
     * This function performs a `POST` to the `/v1/quotes/{quote}` endpoint.
     *
     * <p>A quote models prices and services for a customer.</p>
     *
     * **Parameters:**
     *
     * * `quote: &str` -- The account's country.
     */
    pub async fn post_quotes(&self, quote: &str) -> ClientResult<crate::types::Quote> {
        let url = self.client.url(
            &format!(
                "/v1/quotes/{}",
                crate::progenitor_support::encode_path(quote),
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
     * This function performs a `POST` to the `/v1/quotes/{quote}/accept` endpoint.
     *
     * <p>Accepts the specified quote.</p>
     *
     * **Parameters:**
     *
     * * `quote: &str` -- The account's country.
     */
    pub async fn post_accept(&self, quote: &str) -> ClientResult<crate::types::Quote> {
        let url = self.client.url(
            &format!(
                "/v1/quotes/{}/accept",
                crate::progenitor_support::encode_path(quote),
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
     * This function performs a `POST` to the `/v1/quotes/{quote}/cancel` endpoint.
     *
     * <p>Cancels the quote.</p>
     *
     * **Parameters:**
     *
     * * `quote: &str` -- The account's country.
     */
    pub async fn post_cancel(&self, quote: &str) -> ClientResult<crate::types::Quote> {
        let url = self.client.url(
            &format!(
                "/v1/quotes/{}/cancel",
                crate::progenitor_support::encode_path(quote),
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
     * This function performs a `GET` to the `/v1/quotes/{quote}/computed_upfront_line_items` endpoint.
     *
     * <p>When retrieving a quote, there is an includable <a href="https://stripe.com/docs/api/quotes/object#quote_object-computed-upfront-line_items"><strong>computed.upfront.line_items</strong></a> property containing the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of upfront line items.</p>
     *
     * **Parameters:**
     *
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `quote: &str` -- The account's country.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_computed_upfront_line_items(
        &self,
        ending_before: &str,
        limit: i64,
        quote: &str,
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
                "/v1/quotes/{}/computed_upfront_line_items?{}",
                crate::progenitor_support::encode_path(quote),
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
     * This function performs a `GET` to the `/v1/quotes/{quote}/computed_upfront_line_items` endpoint.
     *
     * As opposed to `get_computed_upfront_line_items`, this function returns all the pages of the request at once.
     *
     * <p>When retrieving a quote, there is an includable <a href="https://stripe.com/docs/api/quotes/object#quote_object-computed-upfront-line_items"><strong>computed.upfront.line_items</strong></a> property containing the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of upfront line items.</p>
     */
    pub async fn get_all_computed_upfront_line_items(
        &self,
        quote: &str,
    ) -> ClientResult<Vec<crate::types::Item>> {
        let url = self.client.url(
            &format!(
                "/v1/quotes/{}/computed_upfront_line_items",
                crate::progenitor_support::encode_path(quote),
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
    /**
     * This function performs a `POST` to the `/v1/quotes/{quote}/finalize` endpoint.
     *
     * <p>Finalizes the quote.</p>
     *
     * **Parameters:**
     *
     * * `quote: &str` -- The account's country.
     */
    pub async fn post_finalize(&self, quote: &str) -> ClientResult<crate::types::Quote> {
        let url = self.client.url(
            &format!(
                "/v1/quotes/{}/finalize",
                crate::progenitor_support::encode_path(quote),
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
     * This function performs a `GET` to the `/v1/quotes/{quote}/line_items` endpoint.
     *
     * <p>When retrieving a quote, there is an includable <strong>line_items</strong> property containing the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
     *
     * **Parameters:**
     *
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `quote: &str` -- The account's country.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_line_items(
        &self,
        ending_before: &str,
        limit: i64,
        quote: &str,
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
                "/v1/quotes/{}/line_items?{}",
                crate::progenitor_support::encode_path(quote),
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
     * This function performs a `GET` to the `/v1/quotes/{quote}/line_items` endpoint.
     *
     * As opposed to `get_line_items`, this function returns all the pages of the request at once.
     *
     * <p>When retrieving a quote, there is an includable <strong>line_items</strong> property containing the first handful of those items. There is also a URL where you can retrieve the full (paginated) list of line items.</p>
     */
    pub async fn get_all_line_items(&self, quote: &str) -> ClientResult<Vec<crate::types::Item>> {
        let url = self.client.url(
            &format!(
                "/v1/quotes/{}/line_items",
                crate::progenitor_support::encode_path(quote),
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
    /**
     * This function performs a `GET` to the `/v1/quotes/{quote}/pdf` endpoint.
     *
     * <p>Download the PDF for a finalized quote</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `quote: &str` -- The account's country.
     */
    pub async fn get_pdf(&self, quote: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v1/quotes/{}/pdf",
                crate::progenitor_support::encode_path(quote),
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
}
