use crate::Client;
use crate::ClientResult;

pub struct PromotionCodes {
    pub client: Client,
}

impl PromotionCodes {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        PromotionCodes { client }
    }

    /**
     * This function performs a `GET` to the `/v1/promotion_codes` endpoint.
     *
     * <p>Returns a list of your promotion codes.</p>
     *
     * **Parameters:**
     *
     * * `active: bool` -- Filter promotion codes by whether they are active.
     * * `code: &str` -- Only return promotion codes that have this case-insensitive code.
     * * `coupon: &str` -- Only return promotion codes for this coupon.
     * * `created: &str` -- A filter on the list, based on the object `created` field. The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
     * * `customer: &str` -- Only return promotion codes that are restricted to this customer.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_page(
        &self,
        active: bool,
        code: &str,
        coupon: &str,
        _created: &str,
        customer: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::PromotionCode>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if active {
            query_args.push(("active".to_string(), active.to_string()));
        }
        if !code.is_empty() {
            query_args.push(("code".to_string(), code.to_string()));
        }
        if !coupon.is_empty() {
            query_args.push(("coupon".to_string(), coupon.to_string()));
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
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/promotion_codes?{}", query_), None);
        let resp: crate::types::GetPromotionCodesResponse = self
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
     * This function performs a `GET` to the `/v1/promotion_codes` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of your promotion codes.</p>
     */
    pub async fn get_all(
        &self,
        active: bool,
        code: &str,
        coupon: &str,
        _created: &str,
        customer: &str,
    ) -> ClientResult<Vec<crate::types::PromotionCode>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if active {
            query_args.push(("active".to_string(), active.to_string()));
        }
        if !code.is_empty() {
            query_args.push(("code".to_string(), code.to_string()));
        }
        if !coupon.is_empty() {
            query_args.push(("coupon".to_string(), coupon.to_string()));
        }
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/promotion_codes?{}", query_), None);
        let mut resp: crate::types::GetPromotionCodesResponse = self
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
     * This function performs a `POST` to the `/v1/promotion_codes` endpoint.
     *
     * <p>A promotion code points to a coupon. You can optionally restrict the code to a specific customer, redemption limit, and expiration date.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::PromotionCode> {
        let url = self.client.url("/v1/promotion_codes", None);
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
     * This function performs a `GET` to the `/v1/promotion_codes/{promotion_code}` endpoint.
     *
     * <p>Retrieves the promotion code with the given ID. In order to retrieve a promotion code by the customer-facing <code>code</code> use <a href="/docs/api/promotion_codes/list">list</a> with the desired <code>code</code>.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `promotion_code: &str` -- The account's country.
     */
    pub async fn get_code(
        &self,
        promotion_code: &str,
    ) -> ClientResult<crate::types::PromotionCode> {
        let url = self.client.url(
            &format!(
                "/v1/promotion_codes/{}",
                crate::progenitor_support::encode_path(promotion_code),
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
     * This function performs a `POST` to the `/v1/promotion_codes/{promotion_code}` endpoint.
     *
     * <p>Updates the specified promotion code by setting the values of the parameters passed. Most fields are, by design, not editable.</p>
     *
     * **Parameters:**
     *
     * * `promotion_code: &str` -- The account's country.
     */
    pub async fn post_code(
        &self,
        promotion_code: &str,
    ) -> ClientResult<crate::types::PromotionCode> {
        let url = self.client.url(
            &format!(
                "/v1/promotion_codes/{}",
                crate::progenitor_support::encode_path(promotion_code),
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
