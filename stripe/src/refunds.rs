use crate::Client;
use crate::ClientResult;

pub struct Refunds {
    pub client: Client,
}

impl Refunds {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Refunds { client }
    }

    /**
     * This function performs a `GET` to the `/v1/refunds` endpoint.
     *
     * <p>Returns a list of all refunds you’ve previously created. The refunds are returned in sorted order, with the most recent refunds appearing first. For convenience, the 10 most recent refunds are always available by default on the charge object.</p>
     *
     * **Parameters:**
     *
     * * `charge: &str` -- Only return refunds for the charge specified by this charge ID.
     * * `created: &str`
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `payment_intent: &str` -- Only return refunds for the PaymentIntent specified by this ID.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_page(
        &self,
        charge: &str,
        _created: &str,
        ending_before: &str,
        limit: i64,
        payment_intent: &str,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::Refund>> {
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
        if !payment_intent.is_empty() {
            query_args.push(("payment_intent".to_string(), payment_intent.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/refunds?{}", query_), None);
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
     * This function performs a `GET` to the `/v1/refunds` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of all refunds you’ve previously created. The refunds are returned in sorted order, with the most recent refunds appearing first. For convenience, the 10 most recent refunds are always available by default on the charge object.</p>
     */
    pub async fn get_all(
        &self,
        charge: &str,
        _created: &str,
        payment_intent: &str,
    ) -> ClientResult<Vec<crate::types::Refund>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !charge.is_empty() {
            query_args.push(("charge".to_string(), charge.to_string()));
        }
        if !payment_intent.is_empty() {
            query_args.push(("payment_intent".to_string(), payment_intent.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/refunds?{}", query_), None);
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
     * This function performs a `POST` to the `/v1/refunds` endpoint.
     *
     * <p>Create a refund.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::Refund> {
        let url = self.client.url("/v1/refunds", None);
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
     * This function performs a `GET` to the `/v1/refunds/{refund}` endpoint.
     *
     * <p>Retrieves the details of an existing refund.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `refund: &str` -- The account's country.
     */
    pub async fn get(&self, refund: &str) -> ClientResult<crate::types::Refund> {
        let url = self.client.url(
            &format!(
                "/v1/refunds/{}",
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
     * This function performs a `POST` to the `/v1/refunds/{refund}` endpoint.
     *
     * <p>Updates the specified refund by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
     *
     * <p>This request only accepts <code>metadata</code> as an argument.</p>
     *
     * **Parameters:**
     *
     * * `refund: &str` -- The account's country.
     */
    pub async fn post_refunds(&self, refund: &str) -> ClientResult<crate::types::Refund> {
        let url = self.client.url(
            &format!(
                "/v1/refunds/{}",
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
    /**
     * This function performs a `POST` to the `/v1/refunds/{refund}/cancel` endpoint.
     *
     * <p>Cancels a refund with a status of <code>requires_action</code>.</p>
     *
     * <p>Refunds in other states cannot be canceled, and only refunds for payment methods that require customer action will enter the <code>requires_action</code> state.</p>
     *
     * **Parameters:**
     *
     * * `refund: &str` -- The account's country.
     */
    pub async fn post_cancel(&self, refund: &str) -> ClientResult<crate::types::Refund> {
        let url = self.client.url(
            &format!(
                "/v1/refunds/{}/cancel",
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
