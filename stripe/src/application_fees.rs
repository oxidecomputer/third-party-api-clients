use crate::Client;
use crate::ClientResult;

pub struct ApplicationFees {
    pub client: Client,
}

impl ApplicationFees {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ApplicationFees { client }
    }

    /**
     * This function performs a `GET` to the `/v1/application_fees` endpoint.
     *
     * <p>Returns a list of application fees you’ve previously collected. The application fees are returned in sorted order, with the most recent fees appearing first.</p>
     *
     * **Parameters:**
     *
     * * `charge: &str` -- Only return application fees for the charge specified by this charge ID.
     * * `created: &str`
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_page(
        &self,
        charge: &str,
        _created: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::PlatformFee>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !charge.is_empty() {
            query_args.push(("charge".to_string(), charge.to_string()));
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
            .url(&format!("/v1/application_fees?{}", query_), None);
        let resp: crate::types::GetApplicationFeesResponse = self
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
     * This function performs a `GET` to the `/v1/application_fees` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of application fees you’ve previously collected. The application fees are returned in sorted order, with the most recent fees appearing first.</p>
     */
    pub async fn get_all(
        &self,
        charge: &str,
        _created: &str,
    ) -> ClientResult<Vec<crate::types::PlatformFee>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !charge.is_empty() {
            query_args.push(("charge".to_string(), charge.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/application_fees?{}", query_), None);
        let mut resp: crate::types::GetApplicationFeesResponse = self
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
     * This function performs a `GET` to the `/v1/application_fees/{fee}/refunds/{id}` endpoint.
     *
     * <p>By default, you can see the 10 most recent refunds stored directly on the application fee object, but you can also retrieve details about a specific refund stored on the application fee.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `fee: &str` -- The account's country.
     * * `id: &str` -- The account's country.
     */
    pub async fn get_fee_refund(
        &self,
        fee: &str,
        id: &str,
    ) -> ClientResult<crate::types::FeeRefund> {
        let url = self.client.url(
            &format!(
                "/v1/application_fees/{}/refunds/{}",
                crate::progenitor_support::encode_path(fee),
                crate::progenitor_support::encode_path(id),
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
     * This function performs a `POST` to the `/v1/application_fees/{fee}/refunds/{id}` endpoint.
     *
     * <p>Updates the specified application fee refund by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
     *
     * <p>This request only accepts metadata as an argument.</p>
     *
     * **Parameters:**
     *
     * * `fee: &str` -- The account's country.
     * * `id: &str` -- The account's country.
     */
    pub async fn post_fee_refund(
        &self,
        fee: &str,
        id: &str,
    ) -> ClientResult<crate::types::FeeRefund> {
        let url = self.client.url(
            &format!(
                "/v1/application_fees/{}/refunds/{}",
                crate::progenitor_support::encode_path(fee),
                crate::progenitor_support::encode_path(id),
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
     * This function performs a `GET` to the `/v1/application_fees/{id}` endpoint.
     *
     * <p>Retrieves the details of an application fee that your account has collected. The same information is returned when refunding the application fee.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `id: &str` -- The account's country.
     */
    pub async fn get(&self, id: &str) -> ClientResult<crate::types::PlatformFee> {
        let url = self.client.url(
            &format!(
                "/v1/application_fees/{}",
                crate::progenitor_support::encode_path(id),
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
     * This function performs a `POST` to the `/v1/application_fees/{id}/refund` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `id: &str` -- The account's country.
     */
    pub async fn post_refund(&self, id: &str) -> ClientResult<crate::types::PlatformFee> {
        let url = self.client.url(
            &format!(
                "/v1/application_fees/{}/refund",
                crate::progenitor_support::encode_path(id),
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
     * This function performs a `GET` to the `/v1/application_fees/{id}/refunds` endpoint.
     *
     * <p>You can see a list of the refunds belonging to a specific application fee. Note that the 10 most recent refunds are always available by default on the application fee object. If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional refunds.</p>
     *
     * **Parameters:**
     *
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `id: &str` -- The account's country.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_refunds(
        &self,
        ending_before: &str,
        id: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::FeeRefund>> {
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
                "/v1/application_fees/{}/refunds?{}",
                crate::progenitor_support::encode_path(id),
                query_
            ),
            None,
        );
        let resp: crate::types::Refunds = self
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
     * This function performs a `GET` to the `/v1/application_fees/{id}/refunds` endpoint.
     *
     * As opposed to `get_refunds`, this function returns all the pages of the request at once.
     *
     * <p>You can see a list of the refunds belonging to a specific application fee. Note that the 10 most recent refunds are always available by default on the application fee object. If you need more than those 10, you can use this API method and the <code>limit</code> and <code>starting_after</code> parameters to page through additional refunds.</p>
     */
    pub async fn get_all_refunds(&self, id: &str) -> ClientResult<Vec<crate::types::FeeRefund>> {
        let url = self.client.url(
            &format!(
                "/v1/application_fees/{}/refunds",
                crate::progenitor_support::encode_path(id),
            ),
            None,
        );
        let mut resp: crate::types::Refunds = self
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
     * This function performs a `POST` to the `/v1/application_fees/{id}/refunds` endpoint.
     *
     * <p>Refunds an application fee that has previously been collected but not yet refunded.
     * Funds will be refunded to the Stripe account from which the fee was originally collected.</p>
     *
     * <p>You can optionally refund only part of an application fee.
     * You can do so multiple times, until the entire fee has been refunded.</p>
     *
     * <p>Once entirely refunded, an application fee can’t be refunded again.
     * This method will raise an error when called on an already-refunded application fee,
     * or when trying to refund more money than is left on an application fee.</p>
     *
     * **Parameters:**
     *
     * * `id: &str` -- The account's country.
     */
    pub async fn post_refund_application_fees(
        &self,
        id: &str,
    ) -> ClientResult<crate::types::FeeRefund> {
        let url = self.client.url(
            &format!(
                "/v1/application_fees/{}/refunds",
                crate::progenitor_support::encode_path(id),
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
