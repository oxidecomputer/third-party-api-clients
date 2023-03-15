use crate::Client;
use crate::ClientResult;

pub struct Prices {
    pub client: Client,
}

impl Prices {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Prices { client }
    }

    /**
     * This function performs a `GET` to the `/v1/prices` endpoint.
     *
     * <p>Returns a list of your prices.</p>
     *
     * **Parameters:**
     *
     * * `active: bool` -- Only return prices that are active or inactive (e.g., pass `false` to list all inactive prices).
     * * `created: &str` -- A filter on the list, based on the object `created` field. The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
     * * `currency: &str` -- Only return prices for the given currency.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `lookup_keys: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `product: &str` -- Only return prices for the given product.
     * * `recurring: &str` -- Only return prices with these recurring fields.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `type_: crate::types::PriceType` -- One of `one_time` or `recurring` depending on whether the price is for a one-time purchase or a recurring (subscription) purchase.
     */
    pub async fn get_page(
        &self,
        active: bool,
        _created: &str,
        currency: &str,
        ending_before: &str,
        limit: i64,
        _lookup_keys: &[String],
        product: &str,
        _recurring: &str,
        starting_after: &str,
        type_: crate::types::PriceType,
    ) -> ClientResult<Vec<crate::types::PriceData>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if active {
            query_args.push(("active".to_string(), active.to_string()));
        }
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !product.is_empty() {
            query_args.push(("product".to_string(), product.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/prices?{}", query_), None);
        let resp: crate::types::PriceList = self
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
     * This function performs a `GET` to the `/v1/prices` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of your prices.</p>
     */
    pub async fn get_all(
        &self,
        active: bool,
        _created: &str,
        currency: &str,
        _lookup_keys: &[String],
        product: &str,
        _recurring: &str,
        type_: crate::types::PriceType,
    ) -> ClientResult<Vec<crate::types::PriceData>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if active {
            query_args.push(("active".to_string(), active.to_string()));
        }
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        if !product.is_empty() {
            query_args.push(("product".to_string(), product.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/prices?{}", query_), None);
        let mut resp: crate::types::PriceList = self
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
     * This function performs a `POST` to the `/v1/prices` endpoint.
     *
     * <p>Creates a new price for an existing product. The price can be recurring or one-time.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::PriceData> {
        let url = self.client.url("/v1/prices", None);
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
     * This function performs a `GET` to the `/v1/prices/search` endpoint.
     *
     * <p>Search for prices you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
     * Don’t use search in read-after-write flows where strict consistency is necessary. Under normal operating
     * conditions, data is searchable in less than a minute. Occasionally, propagation of new or updated data can be up
     * to an hour behind during outages. Search functionality is not available to merchants in India.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `page: &str` -- A cursor for pagination across multiple pages of results. Don't include this parameter on the first call. Use the next_page value returned in a previous response to request subsequent results.
     * * `query: &str` -- The search query string. See [search query language](https://stripe.com/docs/search#search-query-language) and the list of supported [query fields for prices](https://stripe.com/docs/search#query-fields-for-prices).
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
            .url(&format!("/v1/prices/search?{}", query_), None);
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
     * This function performs a `GET` to the `/v1/prices/search` endpoint.
     *
     * As opposed to `get_search`, this function returns all the pages of the request at once.
     *
     * <p>Search for prices you’ve previously created using Stripe’s <a href="/docs/search#search-query-language">Search Query Language</a>.
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
            .url(&format!("/v1/prices/search?{}", query_), None);
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
     * This function performs a `GET` to the `/v1/prices/{price}` endpoint.
     *
     * <p>Retrieves the price with the given ID.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `price: &str` -- The account's country.
     */
    pub async fn get(&self, price: &str) -> ClientResult<crate::types::PriceData> {
        let url = self.client.url(
            &format!(
                "/v1/prices/{}",
                crate::progenitor_support::encode_path(price),
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
     * This function performs a `POST` to the `/v1/prices/{price}` endpoint.
     *
     * <p>Updates the specified price by setting the values of the parameters passed. Any parameters not provided are left unchanged.</p>
     *
     * **Parameters:**
     *
     * * `price: &str` -- The account's country.
     */
    pub async fn post_prices(&self, price: &str) -> ClientResult<crate::types::PriceData> {
        let url = self.client.url(
            &format!(
                "/v1/prices/{}",
                crate::progenitor_support::encode_path(price),
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
