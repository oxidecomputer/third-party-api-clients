use crate::Client;
use crate::ClientResult;

pub struct Orders {
    pub client: Client,
}

impl Orders {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Orders { client }
    }

    /**
     * This function performs a `GET` to the `/v1/orders` endpoint.
     *
     * <p>Returns a list of your orders. The orders are returned sorted by creation date, with the most recently created orders appearing first.</p>
     *
     * **Parameters:**
     *
     * * `created: &str` -- Date this order was created.
     * * `customer: &str` -- Only return orders for the given customer.
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `ids: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     * * `status: &str` -- Only return orders that have the given status. One of `created`, `paid`, `fulfilled`, or `refunded`.
     * * `status_transitions: &str` -- Filter orders based on when they were paid, fulfilled, canceled, or returned.
     * * `upstream_ids: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     */
    pub async fn get_page(
        &self,
        _created: &str,
        customer: &str,
        ending_before: &str,
        _ids: &[String],
        limit: i64,
        starting_after: &str,
        status: &str,
        _status_transitions: &str,
        _upstream_ids: &[String],
    ) -> ClientResult<Vec<crate::types::Order>> {
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
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/orders?{}", query_), None);
        let resp: crate::types::OrdersLegacyResourceOrderList = self
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
     * This function performs a `GET` to the `/v1/orders` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of your orders. The orders are returned sorted by creation date, with the most recently created orders appearing first.</p>
     */
    pub async fn get_all(
        &self,
        _created: &str,
        customer: &str,
        _ids: &[String],
        status: &str,
        _status_transitions: &str,
        _upstream_ids: &[String],
    ) -> ClientResult<Vec<crate::types::Order>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !customer.is_empty() {
            query_args.push(("customer".to_string(), customer.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/v1/orders?{}", query_), None);
        let mut resp: crate::types::OrdersLegacyResourceOrderList = self
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
     * This function performs a `POST` to the `/v1/orders` endpoint.
     *
     * <p>Creates a new order object.</p>
     */
    pub async fn post(&self) -> ClientResult<crate::types::Order> {
        let url = self.client.url("/v1/orders", None);
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
     * This function performs a `GET` to the `/v1/orders/{id}` endpoint.
     *
     * <p>Retrieves the details of an existing order. Supply the unique order ID from either an order creation request or the order list, and Stripe will return the corresponding order information.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `id: &str` -- The account's country.
     */
    pub async fn get(&self, id: &str) -> ClientResult<crate::types::Order> {
        let url = self.client.url(
            &format!("/v1/orders/{}", crate::progenitor_support::encode_path(id),),
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
     * This function performs a `POST` to the `/v1/orders/{id}` endpoint.
     *
     * <p>Updates the specific order by setting the values of the parameters passed. Any parameters not provided will be left unchanged.</p>
     *
     * **Parameters:**
     *
     * * `id: &str` -- The account's country.
     */
    pub async fn post_orders(&self, id: &str) -> ClientResult<crate::types::Order> {
        let url = self.client.url(
            &format!("/v1/orders/{}", crate::progenitor_support::encode_path(id),),
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
     * This function performs a `POST` to the `/v1/orders/{id}/pay` endpoint.
     *
     * <p>Pay an order by providing a <code>source</code> to create a payment.</p>
     *
     * **Parameters:**
     *
     * * `id: &str` -- The account's country.
     */
    pub async fn post_pay(&self, id: &str) -> ClientResult<crate::types::Order> {
        let url = self.client.url(
            &format!(
                "/v1/orders/{}/pay",
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
     * This function performs a `POST` to the `/v1/orders/{id}/returns` endpoint.
     *
     * <p>Return all or part of an order. The order must have a status of <code>paid</code> or <code>fulfilled</code> before it can be returned. Once all items have been returned, the order will become <code>canceled</code> or <code>returned</code> depending on which status the order started in.</p>
     *
     * **Parameters:**
     *
     * * `id: &str` -- The account's country.
     */
    pub async fn post_return(&self, id: &str) -> ClientResult<crate::types::OrderReturn> {
        let url = self.client.url(
            &format!(
                "/v1/orders/{}/returns",
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
