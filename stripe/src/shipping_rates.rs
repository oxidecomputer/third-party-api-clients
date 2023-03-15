use crate::Client;
use crate::ClientResult;

pub struct ShippingRates {
    pub client: Client,
}

impl ShippingRates {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ShippingRates { client }
    }

    /**
     * This function performs a `GET` to the `/v1/shipping_rates` endpoint.
     *
     * <p>Returns a list of your shipping rates.</p>
     *
     * **Parameters:**
     *
     * * `active: bool` -- Only return shipping rates that are active or inactive.
     * * `created: &str` -- A filter on the list, based on the object `created` field. The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
     * * `currency: &str` -- Only return shipping rates for the given currency.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_page(
        &self,
        active: bool,
        _created: &str,
        currency: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::ShippingRate>> {
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
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/shipping_rates?{}", query_), None);
        let resp: crate::types::ShippingResourcesRateList = self
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
     * This function performs a `GET` to the `/v1/shipping_rates` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of your shipping rates.</p>
     */
    pub async fn get_all(
        &self,
        active: bool,
        _created: &str,
        currency: &str,
    ) -> ClientResult<Vec<crate::types::ShippingRate>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if active {
            query_args.push(("active".to_string(), active.to_string()));
        }
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/shipping_rates?{}", query_), None);
        let mut resp: crate::types::ShippingResourcesRateList = self
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
     * This function performs a `POST` to the `/v1/shipping_rates` endpoint.
     *
     * <p>Creates a new shipping rate object.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::ShippingRate> {
        let url = self.client.url("/v1/shipping_rates", None);
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
     * This function performs a `GET` to the `/v1/shipping_rates/{shipping_rate_token}` endpoint.
     *
     * <p>Returns the shipping rate object with the given ID.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `shipping_rate_token: &str` -- The account's country.
     */
    pub async fn get_rate_token(
        &self,
        shipping_rate_token: &str,
    ) -> ClientResult<crate::types::ShippingRate> {
        let url = self.client.url(
            &format!(
                "/v1/shipping_rates/{}",
                crate::progenitor_support::encode_path(shipping_rate_token),
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
     * This function performs a `POST` to the `/v1/shipping_rates/{shipping_rate_token}` endpoint.
     *
     * <p>Updates an existing shipping rate object.</p>
     *
     * **Parameters:**
     *
     * * `shipping_rate_token: &str` -- The account's country.
     */
    pub async fn post_rate_token(
        &self,
        shipping_rate_token: &str,
    ) -> ClientResult<crate::types::ShippingRate> {
        let url = self.client.url(
            &format!(
                "/v1/shipping_rates/{}",
                crate::progenitor_support::encode_path(shipping_rate_token),
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
