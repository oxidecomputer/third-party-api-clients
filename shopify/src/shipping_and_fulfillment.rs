use crate::Client;
use crate::ClientResult;

pub struct ShippingAndFulfillment {
    pub client: Client,
}

impl ShippingAndFulfillment {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ShippingAndFulfillment { client }
    }

    /**
     * Retrieves a list of fulfillment orders on a shop for a specific app.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/assigned_fulfillment_orders.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/assignedfulfillmentorder#index-2020-01
     *
     * **Parameters:**
     *
     * * `assignment_status: &str` -- The assigment status of the fulfillment orders that should be returned:
     *                       
     *                           cancellation_requested: Fulfillment orders for which the merchant has requested cancellation of the previously accepted fulfillment request.
     *                           fulfillment_requested: Fulfillment orders for which the merchant has requested fulfillment.
     *                           fulfillment_accepted: Fulfillment orders for which the merchant's fulfillment request has been accepted. Any number of fulfillments can be created on these fulfillment orders to completely fulfill the requested items.
     * * `location_ids: &str` -- The IDs of the assigned locations of the fulfillment orders that should be returned.
     * * `location_ids: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202001_get_assigned_fulfillment_order(
        &self,
        assignment_status: &str,
        location_ids: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !assignment_status.is_empty() {
            query_args.push((
                "assignment_status".to_string(),
                assignment_status.to_string(),
            ));
        }
        if !location_ids.is_empty() {
            query_args.push(("location_ids".to_string(), location_ids.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/assigned_fulfillment_orders.json?{}",
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of fulfillment orders on a shop for a specific app.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/assigned_fulfillment_orders.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/assignedfulfillmentorder#index-2020-04
     *
     * **Parameters:**
     *
     * * `assignment_status: &str` -- The assigment status of the fulfillment orders that should be returned:
     *                       
     *                           cancellation_requested: Fulfillment orders for which the merchant has requested cancellation of the previously accepted fulfillment request.
     *                           fulfillment_requested: Fulfillment orders for which the merchant has requested fulfillment.
     *                           fulfillment_accepted: Fulfillment orders for which the merchant's fulfillment request has been accepted. Any number of fulfillments can be created on these fulfillment orders to completely fulfill the requested items.
     * * `location_ids: &str` -- The IDs of the assigned locations of the fulfillment orders that should be returned.
     * * `location_ids: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202004_get_assigned_fulfillment_order(
        &self,
        assignment_status: &str,
        location_ids: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !assignment_status.is_empty() {
            query_args.push((
                "assignment_status".to_string(),
                assignment_status.to_string(),
            ));
        }
        if !location_ids.is_empty() {
            query_args.push(("location_ids".to_string(), location_ids.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/assigned_fulfillment_orders.json?{}",
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of fulfillment orders on a shop for a specific app.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/assigned_fulfillment_orders.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/assignedfulfillmentorder#index-2020-07
     *
     * **Parameters:**
     *
     * * `assignment_status: &str` -- The assigment status of the fulfillment orders that should be returned:
     *                       
     *                           cancellation_requested: Fulfillment orders for which the merchant has requested cancellation of the previously accepted fulfillment request.
     *                           fulfillment_requested: Fulfillment orders for which the merchant has requested fulfillment.
     *                           fulfillment_accepted: Fulfillment orders for which the merchant's fulfillment request has been accepted. Any number of fulfillments can be created on these fulfillment orders to completely fulfill the requested items.
     * * `location_ids: &str` -- The IDs of the assigned locations of the fulfillment orders that should be returned.
     * * `location_ids: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202007_get_assigned_fulfillment_order(
        &self,
        assignment_status: &str,
        location_ids: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !assignment_status.is_empty() {
            query_args.push((
                "assignment_status".to_string(),
                assignment_status.to_string(),
            ));
        }
        if !location_ids.is_empty() {
            query_args.push(("location_ids".to_string(), location_ids.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/assigned_fulfillment_orders.json?{}",
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of fulfillment orders on a shop for a specific app.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/assigned_fulfillment_orders.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/assignedfulfillmentorder#index-2020-10
     *
     * **Parameters:**
     *
     * * `assignment_status: &str` -- The assigment status of the fulfillment orders that should be returned:
     *                       
     *                           cancellation_requested: Fulfillment orders for which the merchant has requested cancellation of the previously accepted fulfillment request.
     *                           fulfillment_requested: Fulfillment orders for which the merchant has requested fulfillment.
     *                           fulfillment_accepted: Fulfillment orders for which the merchant's fulfillment request has been accepted. Any number of fulfillments can be created on these fulfillment orders to completely fulfill the requested items.
     * * `location_ids: &str` -- The IDs of the assigned locations of the fulfillment orders that should be returned.
     * * `location_ids: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn get_assigned_fulfillment_order(
        &self,
        assignment_status: &str,
        location_ids: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !assignment_status.is_empty() {
            query_args.push((
                "assignment_status".to_string(),
                assignment_status.to_string(),
            ));
        }
        if !location_ids.is_empty() {
            query_args.push(("location_ids".to_string(), location_ids.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/assigned_fulfillment_orders.json?{}",
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of fulfillment orders on a shop for a specific app.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/assigned_fulfillment_orders.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/assignedfulfillmentorder#index-2021-01
     *
     * **Parameters:**
     *
     * * `assignment_status: &str` -- The assigment status of the fulfillment orders that should be returned:
     *                       
     *                           cancellation_requested: Fulfillment orders for which the merchant has requested cancellation of the previously accepted fulfillment request.
     *                           fulfillment_requested: Fulfillment orders for which the merchant has requested fulfillment.
     *                           fulfillment_accepted: Fulfillment orders for which the merchant's fulfillment request has been accepted. Any number of fulfillments can be created on these fulfillment orders to completely fulfill the requested items.
     * * `location_ids: &str` -- The IDs of the assigned locations of the fulfillment orders that should be returned.
     * * `location_ids: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202101_get_assigned_fulfillment_order(
        &self,
        assignment_status: &str,
        location_ids: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !assignment_status.is_empty() {
            query_args.push((
                "assignment_status".to_string(),
                assignment_status.to_string(),
            ));
        }
        if !location_ids.is_empty() {
            query_args.push(("location_ids".to_string(), location_ids.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/assigned_fulfillment_orders.json?{}",
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of fulfillment orders on a shop for a specific app.
     *
     * This function performs a `GET` to the `/admin/api/unstable/assigned_fulfillment_orders.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/assignedfulfillmentorder#index-unstable
     *
     * **Parameters:**
     *
     * * `assignment_status: &str` -- The assigment status of the fulfillment orders that should be returned:
     *                       
     *                           cancellation_requested: Fulfillment orders for which the merchant has requested cancellation of the previously accepted fulfillment request.
     *                           fulfillment_requested: Fulfillment orders for which the merchant has requested fulfillment.
     *                           fulfillment_accepted: Fulfillment orders for which the merchant's fulfillment request has been accepted. Any number of fulfillments can be created on these fulfillment orders to completely fulfill the requested items.
     * * `location_ids: &str` -- The IDs of the assigned locations of the fulfillment orders that should be returned.
     * * `location_ids: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_unstable_get_assigned_fulfillment_order(
        &self,
        assignment_status: &str,
        location_ids: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !assignment_status.is_empty() {
            query_args.push((
                "assignment_status".to_string(),
                assignment_status.to_string(),
            ));
        }
        if !location_ids.is_empty() {
            query_args.push(("location_ids".to_string(), location_ids.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/assigned_fulfillment_orders.json?{}",
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Sends a cancellation request to the fulfillment service of a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/fulfillment_orders/{fulfillment_order_id}/cancellation_request.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/cancellationrequest#create-2020-01
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for the cancellation request.
     */
    pub async fn deprecated_202001_create_fulfillment_orders_param_order_cancellation_request(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/fulfillment_orders/{}/cancellation_request.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Accepts a cancellation request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/fulfillment_orders/{fulfillment_order_id}/cancellation_request/accept.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/cancellationrequest#accept-2020-01
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for accepting the cancellation request.
     */
    pub async fn deprecated_202001_create_fulfillment_orders_param_order_cancellation_request_accept(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/fulfillment_orders/{}/cancellation_request/accept.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Rejects a cancellation request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/fulfillment_orders/{fulfillment_order_id}/cancellation_request/reject.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/cancellationrequest#reject-2020-01
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for rejecting the cancellation request.
     */
    pub async fn deprecated_202001_create_fulfillment_orders_param_order_cancellation_request_reject(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/fulfillment_orders/{}/cancellation_request/reject.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Sends a cancellation request to the fulfillment service of a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/fulfillment_orders/{fulfillment_order_id}/cancellation_request.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/cancellationrequest#create-2020-04
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for the cancellation request.
     */
    pub async fn deprecated_202004_create_fulfillment_orders_param_order_cancellation_request(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/fulfillment_orders/{}/cancellation_request.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Accepts a cancellation request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/fulfillment_orders/{fulfillment_order_id}/cancellation_request/accept.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/cancellationrequest#accept-2020-04
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for accepting the cancellation request.
     */
    pub async fn deprecated_202004_create_fulfillment_orders_param_order_cancellation_request_accept(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/fulfillment_orders/{}/cancellation_request/accept.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Rejects a cancellation request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/fulfillment_orders/{fulfillment_order_id}/cancellation_request/reject.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/cancellationrequest#reject-2020-04
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for rejecting the cancellation request.
     */
    pub async fn deprecated_202004_create_fulfillment_orders_param_order_cancellation_request_reject(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/fulfillment_orders/{}/cancellation_request/reject.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Sends a cancellation request to the fulfillment service of a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/fulfillment_orders/{fulfillment_order_id}/cancellation_request.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/cancellationrequest#create-2020-07
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for the cancellation request.
     */
    pub async fn deprecated_202007_create_fulfillment_orders_param_order_cancellation_request(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/fulfillment_orders/{}/cancellation_request.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Accepts a cancellation request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/fulfillment_orders/{fulfillment_order_id}/cancellation_request/accept.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/cancellationrequest#accept-2020-07
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for accepting the cancellation request.
     */
    pub async fn deprecated_202007_create_fulfillment_orders_param_order_cancellation_request_accept(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/fulfillment_orders/{}/cancellation_request/accept.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Rejects a cancellation request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/fulfillment_orders/{fulfillment_order_id}/cancellation_request/reject.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/cancellationrequest#reject-2020-07
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for rejecting the cancellation request.
     */
    pub async fn deprecated_202007_create_fulfillment_orders_param_order_cancellation_request_reject(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/fulfillment_orders/{}/cancellation_request/reject.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Sends a cancellation request to the fulfillment service of a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/fulfillment_orders/{fulfillment_order_id}/cancellation_request.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/cancellationrequest#create-2020-10
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for the cancellation request.
     */
    pub async fn create_fulfillment_orders_param_order_cancellation_request(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/fulfillment_orders/{}/cancellation_request.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Accepts a cancellation request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/fulfillment_orders/{fulfillment_order_id}/cancellation_request/accept.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/cancellationrequest#accept-2020-10
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for accepting the cancellation request.
     */
    pub async fn create_fulfillment_orders_param_order_cancellation_request_accept(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/fulfillment_orders/{}/cancellation_request/accept.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Rejects a cancellation request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/fulfillment_orders/{fulfillment_order_id}/cancellation_request/reject.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/cancellationrequest#reject-2020-10
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for rejecting the cancellation request.
     */
    pub async fn create_fulfillment_orders_param_order_cancellation_request_reject(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/fulfillment_orders/{}/cancellation_request/reject.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Sends a cancellation request to the fulfillment service of a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/fulfillment_orders/{fulfillment_order_id}/cancellation_request.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/cancellationrequest#create-2021-01
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for the cancellation request.
     */
    pub async fn deprecated_202101_create_fulfillment_orders_param_order_cancellation_request(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/fulfillment_orders/{}/cancellation_request.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Accepts a cancellation request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/fulfillment_orders/{fulfillment_order_id}/cancellation_request/accept.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/cancellationrequest#accept-2021-01
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for accepting the cancellation request.
     */
    pub async fn deprecated_202101_create_fulfillment_orders_param_order_cancellation_request_accept(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/fulfillment_orders/{}/cancellation_request/accept.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Rejects a cancellation request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/fulfillment_orders/{fulfillment_order_id}/cancellation_request/reject.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/cancellationrequest#reject-2021-01
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for rejecting the cancellation request.
     */
    pub async fn deprecated_202101_create_fulfillment_orders_param_order_cancellation_request_reject(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/fulfillment_orders/{}/cancellation_request/reject.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Sends a cancellation request to the fulfillment service of a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/unstable/fulfillment_orders/{fulfillment_order_id}/cancellation_request.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/cancellationrequest#create-unstable
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for the cancellation request.
     */
    pub async fn deprecated_unstable_create_fulfillment_orders_param_order_cancellation_request(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/fulfillment_orders/{}/cancellation_request.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Accepts a cancellation request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/unstable/fulfillment_orders/{fulfillment_order_id}/cancellation_request/accept.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/cancellationrequest#accept-unstable
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for accepting the cancellation request.
     */
    pub async fn deprecated_unstable_create_fulfillment_orders_param_order_cancellation_request_accept(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/fulfillment_orders/{}/cancellation_request/accept.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Rejects a cancellation request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/unstable/fulfillment_orders/{fulfillment_order_id}/cancellation_request/reject.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/cancellationrequest#reject-unstable
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for rejecting the cancellation request.
     */
    pub async fn deprecated_unstable_create_fulfillment_orders_param_order_cancellation_request_reject(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/fulfillment_orders/{}/cancellation_request/reject.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves a list of carrier services.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/carrier_services.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#index-2020-01
     */
    pub async fn deprecated_202001_get_carrier_service(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-01/carrier_services.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a carrier service.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/carrier_services.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#create-2020-01
     */
    pub async fn deprecated_202001_create_carrier_services(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-01/carrier_services.json", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves a single carrier service by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/carrier_services/{carrier_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#show-2020-01
     *
     * **Parameters:**
     *
     * * `carrier_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_carrier_services_param_service(
        &self,
        carrier_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/carrier_services/{}/json",
                crate::progenitor_support::encode_path(carrier_service_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Updates a carrier service. Only the app that creates a carrier service can update it.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/carrier_services/{carrier_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#update-2020-01
     *
     * **Parameters:**
     *
     * * `carrier_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_carrier_services_param_service(
        &self,
        carrier_service_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/carrier_services/{}/json",
                crate::progenitor_support::encode_path(carrier_service_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Deletes a carrier service.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/carrier_services/{carrier_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `carrier_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_carrier_services_param_service(
        &self,
        carrier_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/carrier_services/{}/json",
                crate::progenitor_support::encode_path(carrier_service_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of carrier services.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/carrier_services.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#index-2020-04
     */
    pub async fn deprecated_202004_get_carrier_service(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-04/carrier_services.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a carrier service.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/carrier_services.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#create-2020-04
     */
    pub async fn deprecated_202004_create_carrier_services(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-04/carrier_services.json", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves a single carrier service by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/carrier_services/{carrier_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#show-2020-04
     *
     * **Parameters:**
     *
     * * `carrier_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_carrier_services_param_service(
        &self,
        carrier_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/carrier_services/{}/json",
                crate::progenitor_support::encode_path(carrier_service_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Updates a carrier service. Only the app that creates a carrier service can update it.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/carrier_services/{carrier_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#update-2020-04
     *
     * **Parameters:**
     *
     * * `carrier_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_carrier_services_param_service(
        &self,
        carrier_service_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/carrier_services/{}/json",
                crate::progenitor_support::encode_path(carrier_service_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Deletes a carrier service.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/carrier_services/{carrier_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `carrier_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_carrier_services_param_service(
        &self,
        carrier_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/carrier_services/{}/json",
                crate::progenitor_support::encode_path(carrier_service_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of carrier services.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/carrier_services.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#index-2020-07
     */
    pub async fn deprecated_202007_get_carrier_service(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-07/carrier_services.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a carrier service.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/carrier_services.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#create-2020-07
     */
    pub async fn deprecated_202007_create_carrier_services(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-07/carrier_services.json", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves a single carrier service by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/carrier_services/{carrier_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#show-2020-07
     *
     * **Parameters:**
     *
     * * `carrier_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_carrier_services_param_service(
        &self,
        carrier_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/carrier_services/{}/json",
                crate::progenitor_support::encode_path(carrier_service_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Updates a carrier service. Only the app that creates a carrier service can update it.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/carrier_services/{carrier_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#update-2020-07
     *
     * **Parameters:**
     *
     * * `carrier_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_carrier_services_param_service(
        &self,
        carrier_service_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/carrier_services/{}/json",
                crate::progenitor_support::encode_path(carrier_service_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Deletes a carrier service.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/carrier_services/{carrier_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `carrier_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_carrier_services_param_service(
        &self,
        carrier_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/carrier_services/{}/json",
                crate::progenitor_support::encode_path(carrier_service_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of carrier services.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/carrier_services.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#index-2020-10
     */
    pub async fn get_carrier_service(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-10/carrier_services.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a carrier service.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/carrier_services.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#create-2020-10
     */
    pub async fn create_carrier_services(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-10/carrier_services.json", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves a single carrier service by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/carrier_services/{carrier_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#show-2020-10
     *
     * **Parameters:**
     *
     * * `carrier_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn get_carrier_services_param_service(
        &self,
        carrier_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/carrier_services/{}/json",
                crate::progenitor_support::encode_path(carrier_service_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Updates a carrier service. Only the app that creates a carrier service can update it.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/carrier_services/{carrier_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#update-2020-10
     *
     * **Parameters:**
     *
     * * `carrier_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn update_carrier_services_param_service(
        &self,
        carrier_service_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/carrier_services/{}/json",
                crate::progenitor_support::encode_path(carrier_service_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Deletes a carrier service.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/carrier_services/{carrier_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `carrier_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_carrier_services_param_service(
        &self,
        carrier_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/carrier_services/{}/json",
                crate::progenitor_support::encode_path(carrier_service_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of carrier services.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/carrier_services.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#index-2021-01
     */
    pub async fn deprecated_202101_get_carrier_service(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2021-01/carrier_services.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a carrier service.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/carrier_services.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#create-2021-01
     */
    pub async fn deprecated_202101_create_carrier_services(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2021-01/carrier_services.json", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves a single carrier service by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/carrier_services/{carrier_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#show-2021-01
     *
     * **Parameters:**
     *
     * * `carrier_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_carrier_services_param_service(
        &self,
        carrier_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/carrier_services/{}/json",
                crate::progenitor_support::encode_path(carrier_service_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Updates a carrier service. Only the app that creates a carrier service can update it.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/carrier_services/{carrier_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#update-2021-01
     *
     * **Parameters:**
     *
     * * `carrier_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_carrier_services_param_service(
        &self,
        carrier_service_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/carrier_services/{}/json",
                crate::progenitor_support::encode_path(carrier_service_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Deletes a carrier service.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/carrier_services/{carrier_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `carrier_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_carrier_services_param_service(
        &self,
        carrier_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/carrier_services/{}/json",
                crate::progenitor_support::encode_path(carrier_service_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of carrier services.
     *
     * This function performs a `GET` to the `/admin/api/unstable/carrier_services.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#index-unstable
     */
    pub async fn deprecated_unstable_get_carrier_service(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/unstable/carrier_services.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a carrier service.
     *
     * This function performs a `POST` to the `/admin/api/unstable/carrier_services.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#create-unstable
     */
    pub async fn deprecated_unstable_create_carrier_services(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/unstable/carrier_services.json", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves a single carrier service by its ID.
     *
     * This function performs a `GET` to the `/admin/api/unstable/carrier_services/{carrier_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#show-unstable
     *
     * **Parameters:**
     *
     * * `carrier_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_carrier_services_param_service(
        &self,
        carrier_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/carrier_services/{}/json",
                crate::progenitor_support::encode_path(carrier_service_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Updates a carrier service. Only the app that creates a carrier service can update it.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/carrier_services/{carrier_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#update-unstable
     *
     * **Parameters:**
     *
     * * `carrier_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_carrier_services_param_service(
        &self,
        carrier_service_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/carrier_services/{}/json",
                crate::progenitor_support::encode_path(carrier_service_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Deletes a carrier service.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/carrier_services/{carrier_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/carrierservice#destroy-unstable
     *
     * **Parameters:**
     *
     * * `carrier_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_carrier_services_param_service(
        &self,
        carrier_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/carrier_services/{}/json",
                crate::progenitor_support::encode_path(carrier_service_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves fulfillments associated with an order. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/orders/{order_id}/fulfillments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#index-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `created_at_max: &str` -- Show fulfillments created before date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_min: &str` -- Show fulfillments created after date (format: 2014-04-25T16:15:47-04:00).
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     * * `limit: &str` -- Limit the amount of results.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `updated_at_max: &str` -- Show fulfillments last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show fulfillments last updated after date (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn deprecated_202001_get_orders_param_order_fulfillment(
        &self,
        order_id: &str,
        created_at_max: &str,
        created_at_min: &str,
        fields: &str,
        limit: &str,
        since_id: &str,
        updated_at_max: &str,
        updated_at_min: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/fulfillments.json?{}",
                crate::progenitor_support::encode_path(order_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create a fulfillment for the specified order and line items.
              The fulfillment's status depends on the line items in the order:

              If the line items in the fulfillment use a manual or custom fulfillment service, then the status of the returned fulfillment will be set immediately.
              If the line items use an external fulfillment service, then they will be queued for fulfillment and the status will be set to pending until the external fulfillment service has been invoked.


              A fulfillment might then transition to open, which implies it is being processed by the service, before transitioning to success when the items have shipped.
              If you don't specify line item IDs, then all unfulfilled and partially fulfilled line items for the order will be fulfilled.
              However, if an order is refunded or if any of its individual line items are refunded, then the order can't be fulfilled.

              All line items being fulfilled must have the same fulfillment service.


                Note
                If you are using this endpoint with a Partner development store or a trial store, then you can create no more than 5 new fulfillments per minute.

              About tracking urls
               If you're creating a fulfillment for a supported carrier, then you can send the tracking_company and tracking_numbers fields, and Shopify will generate the tracking_url for you. If you're creating a fulfillment for an unsupported carrier (not in the tracking_company list), then send the tracking_company, tracking_numbers, and tracking_urls fields.


                Note
                If you send an unsupported carrier without a tracking URL, then Shopify will still try to generate a valid tracking URL by using pattern matching on the tracking number. However, Shopify does not validate the tracking URL, so you should make sure that your tracking URL is correct for the order and fulfillment.
    *
    * This function performs a `POST` to the `/admin/api/2020-01/orders/{order_id}/fulfillments.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#create-2020-01
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202001_create_orders_param_order_fulfillments(
        &self,
        order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/fulfillments.json",
                crate::progenitor_support::encode_path(order_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves fulfillments associated with a fulfillment order.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/fulfillment_orders/{fulfillment_order_id}/fulfillments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#indexV2-2020-01
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_order_id: &str` -- The ID of the fulfillment order that is associated with the fulfillments.
     */
    pub async fn deprecated_202001_get_fulfillment_orders_param_order_fulfillment(
        &self,
        fulfillment_order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/fulfillment_orders/{}/fulfillments.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a count of fulfillments associated with a specific order.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/orders/{order_id}/fulfillments/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#count-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `created_at_min: &str` -- Count fulfillments created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count fulfillments created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Count fulfillments last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count fulfillments last updated before date (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn deprecated_202001_get_orders_param_order_fulfillments_count(
        &self,
        order_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/fulfillments/count.json?{}",
                crate::progenitor_support::encode_path(order_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve a specific fulfillment.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/orders/{order_id}/fulfillments/{fulfillment_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#show-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202001_get_orders_param_order_fulfillments_fulfillment(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/fulfillments/{}/json?{}",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Update information associated with a fulfillment.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/orders/{order_id}/fulfillments/{fulfillment_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#update-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_orders_param_order_fulfillments_fulfillment(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/fulfillments/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Creates a fulfillment for one or many fulfillment orders. The fulfillment orders are associated with the same order and are assigned to the same location.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/fulfillments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#createV2-2020-01
     */
    pub async fn deprecated_202001_create_fulfillments(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-01/fulfillments.json", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Updates the tracking information for a fulfillment.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/fulfillments/{fulfillment_id}/update_tracking.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#update_tracking-2020-01
     *
     * **Parameters:**
     *
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_fulfillments_param_fulfillment_update_tracking(
        &self,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/fulfillments/{}/update_tracking.json",
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Mark a fulfillment as complete.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/orders/{order_id}/fulfillments/{fulfillment_id}/complete.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#complete-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_orders_param_order_fulfillments_fulfillment_complete(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/fulfillments/{}/complete.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Mark a fulfillment as open.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/orders/{order_id}/fulfillments/{fulfillment_id}/open.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#open-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_orders_param_order_fulfillments_fulfillment_open(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/fulfillments/{}/open.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Cancel a fulfillment for a specific order ID.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/orders/{order_id}/fulfillments/{fulfillment_id}/cancel.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#cancel-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_orders_param_order_fulfillments_fulfillment_cancel(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/fulfillments/{}/cancel.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Cancels a fulfillment.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/fulfillments/{fulfillment_id}/cancel.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#cancelV2-2020-01
     *
     * **Parameters:**
     *
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_fulfillments_param_fulfillment_cancel(
        &self,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/fulfillments/{}/cancel.json",
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves fulfillments associated with an order. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/orders/{order_id}/fulfillments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#index-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `created_at_max: &str` -- Show fulfillments created before date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_min: &str` -- Show fulfillments created after date (format: 2014-04-25T16:15:47-04:00).
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     * * `limit: &str` -- Limit the amount of results.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `updated_at_max: &str` -- Show fulfillments last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show fulfillments last updated after date (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn deprecated_202004_get_orders_param_order_fulfillment(
        &self,
        order_id: &str,
        created_at_max: &str,
        created_at_min: &str,
        fields: &str,
        limit: &str,
        since_id: &str,
        updated_at_max: &str,
        updated_at_min: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/fulfillments.json?{}",
                crate::progenitor_support::encode_path(order_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create a fulfillment for the specified order and line items.
              The fulfillment's status depends on the line items in the order:

              If the line items in the fulfillment use a manual or custom fulfillment service, then the status of the returned fulfillment will be set immediately.
              If the line items use an external fulfillment service, then they will be queued for fulfillment and the status will be set to pending until the external fulfillment service has been invoked.


              A fulfillment might then transition to open, which implies it is being processed by the service, before transitioning to success when the items have shipped.
              If you don't specify line item IDs, then all unfulfilled and partially fulfilled line items for the order will be fulfilled.
              However, if an order is refunded or if any of its individual line items are refunded, then the order can't be fulfilled.

              All line items being fulfilled must have the same fulfillment service.


                Note
                If you are using this endpoint with a Partner development store or a trial store, then you can create no more than 5 new fulfillments per minute.

              About tracking urls
               If you're creating a fulfillment for a supported carrier, then you can send the tracking_company and tracking_numbers fields, and Shopify will generate the tracking_url for you. If you're creating a fulfillment for an unsupported carrier (not in the tracking_company list), then send the tracking_company, tracking_numbers, and tracking_urls fields.


                Note
                If you send an unsupported carrier without a tracking URL, then Shopify will still try to generate a valid tracking URL by using pattern matching on the tracking number. However, Shopify does not validate the tracking URL, so you should make sure that your tracking URL is correct for the order and fulfillment.
    *
    * This function performs a `POST` to the `/admin/api/2020-04/orders/{order_id}/fulfillments.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#create-2020-04
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202004_create_orders_param_order_fulfillments(
        &self,
        order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/fulfillments.json",
                crate::progenitor_support::encode_path(order_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves fulfillments associated with a fulfillment order.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/fulfillment_orders/{fulfillment_order_id}/fulfillments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#indexV2-2020-04
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_order_id: &str` -- The ID of the fulfillment order that is associated with the fulfillments.
     */
    pub async fn deprecated_202004_get_fulfillment_orders_param_order_fulfillment(
        &self,
        fulfillment_order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/fulfillment_orders/{}/fulfillments.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a count of fulfillments associated with a specific order.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/orders/{order_id}/fulfillments/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#count-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `created_at_min: &str` -- Count fulfillments created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count fulfillments created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Count fulfillments last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count fulfillments last updated before date (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn deprecated_202004_get_orders_param_order_fulfillments_count(
        &self,
        order_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/fulfillments/count.json?{}",
                crate::progenitor_support::encode_path(order_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve a specific fulfillment.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/orders/{order_id}/fulfillments/{fulfillment_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#show-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202004_get_orders_param_order_fulfillments_fulfillment(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/fulfillments/{}/json?{}",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Update information associated with a fulfillment.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/orders/{order_id}/fulfillments/{fulfillment_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#update-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_orders_param_order_fulfillments_fulfillment(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/fulfillments/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Creates a fulfillment for one or many fulfillment orders. The fulfillment orders are associated with the same order and are assigned to the same location.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/fulfillments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#createV2-2020-04
     */
    pub async fn deprecated_202004_create_fulfillments(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-04/fulfillments.json", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Updates the tracking information for a fulfillment.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/fulfillments/{fulfillment_id}/update_tracking.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#update_tracking-2020-04
     *
     * **Parameters:**
     *
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_create_fulfillments_param_fulfillment_update_tracking(
        &self,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/fulfillments/{}/update_tracking.json",
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Mark a fulfillment as complete.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/orders/{order_id}/fulfillments/{fulfillment_id}/complete.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#complete-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_create_orders_param_order_fulfillments_fulfillment_complete(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/fulfillments/{}/complete.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Mark a fulfillment as open.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/orders/{order_id}/fulfillments/{fulfillment_id}/open.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#open-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_create_orders_param_order_fulfillments_fulfillment_open(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/fulfillments/{}/open.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Cancel a fulfillment for a specific order ID.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/orders/{order_id}/fulfillments/{fulfillment_id}/cancel.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#cancel-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_create_orders_param_order_fulfillments_fulfillment_cancel(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/fulfillments/{}/cancel.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Cancels a fulfillment.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/fulfillments/{fulfillment_id}/cancel.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#cancelV2-2020-04
     *
     * **Parameters:**
     *
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_create_fulfillments_param_fulfillment_cancel(
        &self,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/fulfillments/{}/cancel.json",
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves fulfillments associated with an order. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/orders/{order_id}/fulfillments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#index-2020-07
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `created_at_max: &str` -- Show fulfillments created before date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_min: &str` -- Show fulfillments created after date (format: 2014-04-25T16:15:47-04:00).
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     * * `limit: &str` -- Limit the amount of results.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `updated_at_max: &str` -- Show fulfillments last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show fulfillments last updated after date (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn deprecated_202007_get_orders_param_order_fulfillment(
        &self,
        order_id: &str,
        created_at_max: &str,
        created_at_min: &str,
        fields: &str,
        limit: &str,
        since_id: &str,
        updated_at_max: &str,
        updated_at_min: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/fulfillments.json?{}",
                crate::progenitor_support::encode_path(order_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create a fulfillment for the specified order and line items.
              The fulfillment's status depends on the line items in the order:

              If the line items in the fulfillment use a manual or custom fulfillment service, then the status of the returned fulfillment will be set immediately.
              If the line items use an external fulfillment service, then they will be queued for fulfillment and the status will be set to pending until the external fulfillment service has been invoked.


              A fulfillment might then transition to open, which implies it is being processed by the service, before transitioning to success when the items have shipped.
              If you don't specify line item IDs, then all unfulfilled and partially fulfilled line items for the order will be fulfilled.
              However, if an order is refunded or if any of its individual line items are refunded, then the order can't be fulfilled.

              All line items being fulfilled must have the same fulfillment service.


                Note
                If you are using this endpoint with a Partner development store or a trial store, then you can create no more than 5 new fulfillments per minute.

              About tracking urls
               If you're creating a fulfillment for a supported carrier, then you can send the tracking_company and tracking_numbers fields, and Shopify will generate the tracking_url for you. If you're creating a fulfillment for an unsupported carrier (not in the tracking_company list), then send the tracking_company, tracking_numbers, and tracking_urls fields.


                Note
                If you send an unsupported carrier without a tracking URL, then Shopify will still try to generate a valid tracking URL by using pattern matching on the tracking number. However, Shopify does not validate the tracking URL, so you should make sure that your tracking URL is correct for the order and fulfillment.
    *
    * This function performs a `POST` to the `/admin/api/2020-07/orders/{order_id}/fulfillments.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#create-2020-07
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202007_create_orders_param_order_fulfillments(
        &self,
        order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/fulfillments.json",
                crate::progenitor_support::encode_path(order_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves fulfillments associated with a fulfillment order.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/fulfillment_orders/{fulfillment_order_id}/fulfillments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#indexV2-2020-07
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_order_id: &str` -- The ID of the fulfillment order that is associated with the fulfillments.
     */
    pub async fn deprecated_202007_get_fulfillment_orders_param_order_fulfillment(
        &self,
        fulfillment_order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/fulfillment_orders/{}/fulfillments.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a count of fulfillments associated with a specific order.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/orders/{order_id}/fulfillments/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#count-2020-07
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `created_at_min: &str` -- Count fulfillments created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count fulfillments created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Count fulfillments last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count fulfillments last updated before date (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn deprecated_202007_get_orders_param_order_fulfillments_count(
        &self,
        order_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/fulfillments/count.json?{}",
                crate::progenitor_support::encode_path(order_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve a specific fulfillment.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/orders/{order_id}/fulfillments/{fulfillment_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#show-2020-07
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202007_get_orders_param_order_fulfillments_fulfillment(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/fulfillments/{}/json?{}",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Update information associated with a fulfillment.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/orders/{order_id}/fulfillments/{fulfillment_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#update-2020-07
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_orders_param_order_fulfillments_fulfillment(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/fulfillments/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Creates a fulfillment for one or many fulfillment orders. The fulfillment orders are associated with the same order and are assigned to the same location.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/fulfillments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#createV2-2020-07
     */
    pub async fn deprecated_202007_create_fulfillments(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-07/fulfillments.json", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Updates the tracking information for a fulfillment.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/fulfillments/{fulfillment_id}/update_tracking.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#update_tracking-2020-07
     *
     * **Parameters:**
     *
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_create_fulfillments_param_fulfillment_update_tracking(
        &self,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/fulfillments/{}/update_tracking.json",
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Mark a fulfillment as complete.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/orders/{order_id}/fulfillments/{fulfillment_id}/complete.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#complete-2020-07
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_create_orders_param_order_fulfillments_fulfillment_complete(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/fulfillments/{}/complete.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Mark a fulfillment as open.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/orders/{order_id}/fulfillments/{fulfillment_id}/open.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#open-2020-07
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_create_orders_param_order_fulfillments_fulfillment_open(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/fulfillments/{}/open.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Cancel a fulfillment for a specific order ID.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/orders/{order_id}/fulfillments/{fulfillment_id}/cancel.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#cancel-2020-07
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_create_orders_param_order_fulfillments_fulfillment_cancel(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/fulfillments/{}/cancel.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Cancels a fulfillment.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/fulfillments/{fulfillment_id}/cancel.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#cancelV2-2020-07
     *
     * **Parameters:**
     *
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_create_fulfillments_param_fulfillment_cancel(
        &self,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/fulfillments/{}/cancel.json",
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves fulfillments associated with an order. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/orders/{order_id}/fulfillments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#index-2020-10
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `created_at_max: &str` -- Show fulfillments created before date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_min: &str` -- Show fulfillments created after date (format: 2014-04-25T16:15:47-04:00).
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     * * `limit: &str` -- Limit the amount of results.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `updated_at_max: &str` -- Show fulfillments last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show fulfillments last updated after date (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn get_orders_param_order_fulfillment(
        &self,
        order_id: &str,
        created_at_max: &str,
        created_at_min: &str,
        fields: &str,
        limit: &str,
        since_id: &str,
        updated_at_max: &str,
        updated_at_min: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/fulfillments.json?{}",
                crate::progenitor_support::encode_path(order_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create a fulfillment for the specified order and line items.
              The fulfillment's status depends on the line items in the order:

              If the line items in the fulfillment use a manual or custom fulfillment service, then the status of the returned fulfillment will be set immediately.
              If the line items use an external fulfillment service, then they will be queued for fulfillment and the status will be set to pending until the external fulfillment service has been invoked.


              A fulfillment might then transition to open, which implies it is being processed by the service, before transitioning to success when the items have shipped.
              If you don't specify line item IDs, then all unfulfilled and partially fulfilled line items for the order will be fulfilled.
              However, if an order is refunded or if any of its individual line items are refunded, then the order can't be fulfilled.

              All line items being fulfilled must have the same fulfillment service.


                Note
                If you are using this endpoint with a Partner development store or a trial store, then you can create no more than 5 new fulfillments per minute.

              About tracking urls
               If you're creating a fulfillment for a supported carrier, then you can send the tracking_company and tracking_numbers fields, and Shopify will generate the tracking_url for you. If you're creating a fulfillment for an unsupported carrier (not in the tracking_company list), then send the tracking_company, tracking_numbers, and tracking_urls fields.


                Note
                If you send an unsupported carrier without a tracking URL, then Shopify will still try to generate a valid tracking URL by using pattern matching on the tracking number. However, Shopify does not validate the tracking URL, so you should make sure that your tracking URL is correct for the order and fulfillment.
    *
    * This function performs a `POST` to the `/admin/api/2020-10/orders/{order_id}/fulfillments.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#create-2020-10
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    */
    pub async fn create_orders_param_order_fulfillments(
        &self,
        order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/fulfillments.json",
                crate::progenitor_support::encode_path(order_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves fulfillments associated with a fulfillment order.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/fulfillment_orders/{fulfillment_order_id}/fulfillments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#indexV2-2020-10
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_order_id: &str` -- The ID of the fulfillment order that is associated with the fulfillments.
     */
    pub async fn get_fulfillment_orders_param_order_fulfillment(
        &self,
        fulfillment_order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/fulfillment_orders/{}/fulfillments.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a count of fulfillments associated with a specific order.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/orders/{order_id}/fulfillments/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#count-2020-10
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `created_at_min: &str` -- Count fulfillments created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count fulfillments created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Count fulfillments last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count fulfillments last updated before date (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn get_orders_param_order_fulfillments_count(
        &self,
        order_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/fulfillments/count.json?{}",
                crate::progenitor_support::encode_path(order_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve a specific fulfillment.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/orders/{order_id}/fulfillments/{fulfillment_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#show-2020-10
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Comma-separated list of fields to include in the response.
     */
    pub async fn get_orders_param_order_fulfillments_fulfillment(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/fulfillments/{}/json?{}",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Update information associated with a fulfillment.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/orders/{order_id}/fulfillments/{fulfillment_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#update-2020-10
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn update_orders_param_order_fulfillments_fulfillment(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/fulfillments/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Creates a fulfillment for one or many fulfillment orders. The fulfillment orders are associated with the same order and are assigned to the same location.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/fulfillments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#createV2-2020-10
     */
    pub async fn create_fulfillments(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-10/fulfillments.json", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Updates the tracking information for a fulfillment.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/fulfillments/{fulfillment_id}/update_tracking.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#update_tracking-2020-10
     *
     * **Parameters:**
     *
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn create_fulfillments_param_fulfillment_update_tracking(
        &self,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/fulfillments/{}/update_tracking.json",
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Mark a fulfillment as complete.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/orders/{order_id}/fulfillments/{fulfillment_id}/complete.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#complete-2020-10
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn create_orders_param_order_fulfillments_fulfillment_complete(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/fulfillments/{}/complete.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Mark a fulfillment as open.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/orders/{order_id}/fulfillments/{fulfillment_id}/open.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#open-2020-10
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn create_orders_param_order_fulfillments_fulfillment_open(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/fulfillments/{}/open.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Cancel a fulfillment for a specific order ID.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/orders/{order_id}/fulfillments/{fulfillment_id}/cancel.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#cancel-2020-10
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn create_orders_param_order_fulfillments_fulfillment_cancel(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/fulfillments/{}/cancel.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Cancels a fulfillment.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/fulfillments/{fulfillment_id}/cancel.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#cancelV2-2020-10
     *
     * **Parameters:**
     *
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn create_fulfillments_param_fulfillment_cancel(
        &self,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/fulfillments/{}/cancel.json",
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves fulfillments associated with an order. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/orders/{order_id}/fulfillments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#index-2021-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `created_at_max: &str` -- Show fulfillments created before date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_min: &str` -- Show fulfillments created after date (format: 2014-04-25T16:15:47-04:00).
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     * * `limit: &str` -- Limit the amount of results.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `updated_at_max: &str` -- Show fulfillments last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show fulfillments last updated after date (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn deprecated_202101_get_orders_param_order_fulfillment(
        &self,
        order_id: &str,
        created_at_max: &str,
        created_at_min: &str,
        fields: &str,
        limit: &str,
        since_id: &str,
        updated_at_max: &str,
        updated_at_min: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/fulfillments.json?{}",
                crate::progenitor_support::encode_path(order_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create a fulfillment for the specified order and line items.
              The fulfillment's status depends on the line items in the order:

              If the line items in the fulfillment use a manual or custom fulfillment service, then the status of the returned fulfillment will be set immediately.
              If the line items use an external fulfillment service, then they will be queued for fulfillment and the status will be set to pending until the external fulfillment service has been invoked.


              A fulfillment might then transition to open, which implies it is being processed by the service, before transitioning to success when the items have shipped.
              If you don't specify line item IDs, then all unfulfilled and partially fulfilled line items for the order will be fulfilled.
              However, if an order is refunded or if any of its individual line items are refunded, then the order can't be fulfilled.

              All line items being fulfilled must have the same fulfillment service.


                Note
                If you are using this endpoint with a Partner development store or a trial store, then you can create no more than 5 new fulfillments per minute.

              About tracking urls
               If you're creating a fulfillment for a supported carrier, then you can send the tracking_company and tracking_numbers fields, and Shopify will generate the tracking_url for you. If you're creating a fulfillment for an unsupported carrier (not in the tracking_company list), then send the tracking_company, tracking_numbers, and tracking_urls fields.


                Note
                If you send an unsupported carrier without a tracking URL, then Shopify will still try to generate a valid tracking URL by using pattern matching on the tracking number. However, Shopify does not validate the tracking URL, so you should make sure that your tracking URL is correct for the order and fulfillment.
    *
    * This function performs a `POST` to the `/admin/api/2021-01/orders/{order_id}/fulfillments.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#create-2021-01
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202101_create_orders_param_order_fulfillments(
        &self,
        order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/fulfillments.json",
                crate::progenitor_support::encode_path(order_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves fulfillments associated with a fulfillment order.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/fulfillment_orders/{fulfillment_order_id}/fulfillments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#indexV2-2021-01
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_order_id: &str` -- The ID of the fulfillment order that is associated with the fulfillments.
     */
    pub async fn deprecated_202101_get_fulfillment_orders_param_order_fulfillment(
        &self,
        fulfillment_order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/fulfillment_orders/{}/fulfillments.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a count of fulfillments associated with a specific order.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/orders/{order_id}/fulfillments/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#count-2021-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `created_at_min: &str` -- Count fulfillments created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count fulfillments created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Count fulfillments last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count fulfillments last updated before date (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn deprecated_202101_get_orders_param_order_fulfillments_count(
        &self,
        order_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/fulfillments/count.json?{}",
                crate::progenitor_support::encode_path(order_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve a specific fulfillment.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/orders/{order_id}/fulfillments/{fulfillment_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#show-2021-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202101_get_orders_param_order_fulfillments_fulfillment(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/fulfillments/{}/json?{}",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Update information associated with a fulfillment.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/orders/{order_id}/fulfillments/{fulfillment_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#update-2021-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_orders_param_order_fulfillments_fulfillment(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/fulfillments/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Creates a fulfillment for one or many fulfillment orders. The fulfillment orders are associated with the same order and are assigned to the same location.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/fulfillments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#createV2-2021-01
     */
    pub async fn deprecated_202101_create_fulfillments(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2021-01/fulfillments.json", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Updates the tracking information for a fulfillment.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/fulfillments/{fulfillment_id}/update_tracking.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#update_tracking-2021-01
     *
     * **Parameters:**
     *
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_create_fulfillments_param_fulfillment_update_tracking(
        &self,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/fulfillments/{}/update_tracking.json",
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Mark a fulfillment as complete.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/orders/{order_id}/fulfillments/{fulfillment_id}/complete.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#complete-2021-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_create_orders_param_order_fulfillments_fulfillment_complete(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/fulfillments/{}/complete.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Mark a fulfillment as open.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/orders/{order_id}/fulfillments/{fulfillment_id}/open.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#open-2021-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_create_orders_param_order_fulfillments_fulfillment_open(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/fulfillments/{}/open.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Cancel a fulfillment for a specific order ID.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/orders/{order_id}/fulfillments/{fulfillment_id}/cancel.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#cancel-2021-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_create_orders_param_order_fulfillments_fulfillment_cancel(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/fulfillments/{}/cancel.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Cancels a fulfillment.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/fulfillments/{fulfillment_id}/cancel.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#cancelV2-2021-01
     *
     * **Parameters:**
     *
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_create_fulfillments_param_fulfillment_cancel(
        &self,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/fulfillments/{}/cancel.json",
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves fulfillments associated with an order. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/orders/{order_id}/fulfillments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#index-unstable
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `created_at_max: &str` -- Show fulfillments created before date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_min: &str` -- Show fulfillments created after date (format: 2014-04-25T16:15:47-04:00).
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     * * `limit: &str` -- Limit the amount of results.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `updated_at_max: &str` -- Show fulfillments last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show fulfillments last updated after date (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn deprecated_unstable_get_orders_param_order_fulfillment(
        &self,
        order_id: &str,
        created_at_max: &str,
        created_at_min: &str,
        fields: &str,
        limit: &str,
        since_id: &str,
        updated_at_max: &str,
        updated_at_min: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/fulfillments.json?{}",
                crate::progenitor_support::encode_path(order_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Create a fulfillment for the specified order and line items.
              The fulfillment's status depends on the line items in the order:

              If the line items in the fulfillment use a manual or custom fulfillment service, then the status of the returned fulfillment will be set immediately.
              If the line items use an external fulfillment service, then they will be queued for fulfillment and the status will be set to pending until the external fulfillment service has been invoked.


              A fulfillment might then transition to open, which implies it is being processed by the service, before transitioning to success when the items have shipped.
              If you don't specify line item IDs, then all unfulfilled and partially fulfilled line items for the order will be fulfilled.
              However, if an order is refunded or if any of its individual line items are refunded, then the order can't be fulfilled.

              All line items being fulfilled must have the same fulfillment service.


                Note
                If you are using this endpoint with a Partner development store or a trial store, then you can create no more than 5 new fulfillments per minute.

              About tracking urls
               If you're creating a fulfillment for a supported carrier, then you can send the tracking_company and tracking_numbers fields, and Shopify will generate the tracking_url for you. If you're creating a fulfillment for an unsupported carrier (not in the tracking_company list), then send the tracking_company, tracking_numbers, and tracking_urls fields.


                Note
                If you send an unsupported carrier without a tracking URL, then Shopify will still try to generate a valid tracking URL by using pattern matching on the tracking number. However, Shopify does not validate the tracking URL, so you should make sure that your tracking URL is correct for the order and fulfillment.
    *
    * This function performs a `POST` to the `/admin/api/unstable/orders/{order_id}/fulfillments.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#create-unstable
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_unstable_create_orders_param_order_fulfillments(
        &self,
        order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/fulfillments.json",
                crate::progenitor_support::encode_path(order_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves fulfillments associated with a fulfillment order.
     *
     * This function performs a `GET` to the `/admin/api/unstable/fulfillment_orders/{fulfillment_order_id}/fulfillments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#indexV2-unstable
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_order_id: &str` -- The ID of the fulfillment order that is associated with the fulfillments.
     */
    pub async fn deprecated_unstable_get_fulfillment_orders_param_order_fulfillment(
        &self,
        fulfillment_order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/fulfillment_orders/{}/fulfillments.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a count of fulfillments associated with a specific order.
     *
     * This function performs a `GET` to the `/admin/api/unstable/orders/{order_id}/fulfillments/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#count-unstable
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `created_at_min: &str` -- Count fulfillments created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count fulfillments created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Count fulfillments last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count fulfillments last updated before date (format: 2014-04-25T16:15:47-04:00).
     */
    pub async fn deprecated_unstable_get_orders_param_order_fulfillments_count(
        &self,
        order_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/fulfillments/count.json?{}",
                crate::progenitor_support::encode_path(order_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve a specific fulfillment.
     *
     * This function performs a `GET` to the `/admin/api/unstable/orders/{order_id}/fulfillments/{fulfillment_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#show-unstable
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_unstable_get_orders_param_order_fulfillments_fulfillment(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/fulfillments/{}/json?{}",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Update information associated with a fulfillment.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/orders/{order_id}/fulfillments/{fulfillment_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#update-unstable
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_orders_param_order_fulfillments_fulfillment(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/fulfillments/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Creates a fulfillment for one or many fulfillment orders. The fulfillment orders are associated with the same order and are assigned to the same location.
     *
     * This function performs a `POST` to the `/admin/api/unstable/fulfillments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#createV2-unstable
     */
    pub async fn deprecated_unstable_create_fulfillments(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/unstable/fulfillments.json", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Updates the tracking information for a fulfillment.
     *
     * This function performs a `POST` to the `/admin/api/unstable/fulfillments/{fulfillment_id}/update_tracking.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#update_tracking-unstable
     *
     * **Parameters:**
     *
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_create_fulfillments_param_fulfillment_update_tracking(
        &self,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/fulfillments/{}/update_tracking.json",
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Mark a fulfillment as complete.
     *
     * This function performs a `POST` to the `/admin/api/unstable/orders/{order_id}/fulfillments/{fulfillment_id}/complete.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#complete-unstable
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_create_orders_param_order_fulfillments_fulfillment_complete(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/fulfillments/{}/complete.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Mark a fulfillment as open.
     *
     * This function performs a `POST` to the `/admin/api/unstable/orders/{order_id}/fulfillments/{fulfillment_id}/open.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#open-unstable
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_create_orders_param_order_fulfillments_fulfillment_open(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/fulfillments/{}/open.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Cancel a fulfillment for a specific order ID.
     *
     * This function performs a `POST` to the `/admin/api/unstable/orders/{order_id}/fulfillments/{fulfillment_id}/cancel.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#cancel-unstable
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_create_orders_param_order_fulfillments_fulfillment_cancel(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/fulfillments/{}/cancel.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Cancels a fulfillment.
     *
     * This function performs a `POST` to the `/admin/api/unstable/fulfillments/{fulfillment_id}/cancel.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillment#cancelV2-unstable
     *
     * **Parameters:**
     *
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_create_fulfillments_param_fulfillment_cancel(
        &self,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/fulfillments/{}/cancel.json",
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves a list of fulfillment events for a specific fulfillment.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/orders/{order_id}/fulfillments/{fulfillment_id}/events.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#index-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- The ID of the fulfillment that's associated with the fulfillment event.
     * * `order_id: &str` -- The ID of the order that's associated with the fulfillment event.
     */
    pub async fn deprecated_202001_get_orders_param_order_fulfillments_fulfillment_event(
        &self,
        order_id: &str,
        fulfillment_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/fulfillments/{}/events.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a fulfillment event.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/orders/{order_id}/fulfillments/{fulfillment_id}/events.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#create-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_orders_param_order_fulfillments_fulfillment_events(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/fulfillments/{}/events.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves a specific fulfillment event.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/orders/{order_id}/fulfillments/{fulfillment_id}/events/{event_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#show-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `event_id: &str` -- storefront_access_token_id.
     * * `event_id: &str` -- The ID of the fulfillment event.
     */
    pub async fn deprecated_202001_get_orders_param_order_fulfillments_fulfillment_events_event(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        event_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/fulfillments/{}/events/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
                crate::progenitor_support::encode_path(event_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Deletes a fulfillment event.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/orders/{order_id}/fulfillments/{fulfillment_id}/events/{event_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `event_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_orders_param_order_fulfillments_fulfillment_events_event(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        event_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/fulfillments/{}/events/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
                crate::progenitor_support::encode_path(event_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of fulfillment events for a specific fulfillment.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/orders/{order_id}/fulfillments/{fulfillment_id}/events.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#index-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- The ID of the fulfillment that's associated with the fulfillment event.
     * * `order_id: &str` -- The ID of the order that's associated with the fulfillment event.
     */
    pub async fn deprecated_202004_get_orders_param_order_fulfillments_fulfillment_event(
        &self,
        order_id: &str,
        fulfillment_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/fulfillments/{}/events.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a fulfillment event.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/orders/{order_id}/fulfillments/{fulfillment_id}/events.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#create-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_create_orders_param_order_fulfillments_fulfillment_events(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/fulfillments/{}/events.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves a specific fulfillment event.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/orders/{order_id}/fulfillments/{fulfillment_id}/events/{event_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#show-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `event_id: &str` -- storefront_access_token_id.
     * * `event_id: &str` -- The ID of the fulfillment event.
     */
    pub async fn deprecated_202004_get_orders_param_order_fulfillments_fulfillment_events_event(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        event_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/fulfillments/{}/events/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
                crate::progenitor_support::encode_path(event_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Deletes a fulfillment event.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/orders/{order_id}/fulfillments/{fulfillment_id}/events/{event_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `event_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_orders_param_order_fulfillments_fulfillment_events_event(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        event_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/fulfillments/{}/events/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
                crate::progenitor_support::encode_path(event_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of fulfillment events for a specific fulfillment.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/orders/{order_id}/fulfillments/{fulfillment_id}/events.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#index-2020-07
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- The ID of the fulfillment that's associated with the fulfillment event.
     * * `order_id: &str` -- The ID of the order that's associated with the fulfillment event.
     */
    pub async fn deprecated_202007_get_orders_param_order_fulfillments_fulfillment_event(
        &self,
        order_id: &str,
        fulfillment_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/fulfillments/{}/events.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a fulfillment event.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/orders/{order_id}/fulfillments/{fulfillment_id}/events.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#create-2020-07
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_create_orders_param_order_fulfillments_fulfillment_events(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/fulfillments/{}/events.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves a specific fulfillment event.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/orders/{order_id}/fulfillments/{fulfillment_id}/events/{event_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#show-2020-07
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `event_id: &str` -- storefront_access_token_id.
     * * `event_id: &str` -- The ID of the fulfillment event.
     */
    pub async fn deprecated_202007_get_orders_param_order_fulfillments_fulfillment_events_event(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        event_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/fulfillments/{}/events/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
                crate::progenitor_support::encode_path(event_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Deletes a fulfillment event.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/orders/{order_id}/fulfillments/{fulfillment_id}/events/{event_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `event_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_orders_param_order_fulfillments_fulfillment_events_event(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        event_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/fulfillments/{}/events/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
                crate::progenitor_support::encode_path(event_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of fulfillment events for a specific fulfillment.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/orders/{order_id}/fulfillments/{fulfillment_id}/events.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#index-2020-10
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- The ID of the fulfillment that's associated with the fulfillment event.
     * * `order_id: &str` -- The ID of the order that's associated with the fulfillment event.
     */
    pub async fn get_orders_param_order_fulfillments_fulfillment_event(
        &self,
        order_id: &str,
        fulfillment_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/fulfillments/{}/events.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a fulfillment event.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/orders/{order_id}/fulfillments/{fulfillment_id}/events.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#create-2020-10
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn create_orders_param_order_fulfillments_fulfillment_events(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/fulfillments/{}/events.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves a specific fulfillment event.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/orders/{order_id}/fulfillments/{fulfillment_id}/events/{event_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#show-2020-10
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `event_id: &str` -- storefront_access_token_id.
     * * `event_id: &str` -- The ID of the fulfillment event.
     */
    pub async fn get_orders_param_order_fulfillments_fulfillment_events_event(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        event_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/fulfillments/{}/events/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
                crate::progenitor_support::encode_path(event_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Deletes a fulfillment event.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/orders/{order_id}/fulfillments/{fulfillment_id}/events/{event_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `event_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_orders_param_order_fulfillments_fulfillment_events_event(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        event_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/fulfillments/{}/events/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
                crate::progenitor_support::encode_path(event_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of fulfillment events for a specific fulfillment.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/orders/{order_id}/fulfillments/{fulfillment_id}/events.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#index-2021-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- The ID of the fulfillment that's associated with the fulfillment event.
     * * `order_id: &str` -- The ID of the order that's associated with the fulfillment event.
     */
    pub async fn deprecated_202101_get_orders_param_order_fulfillments_fulfillment_event(
        &self,
        order_id: &str,
        fulfillment_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/fulfillments/{}/events.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a fulfillment event.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/orders/{order_id}/fulfillments/{fulfillment_id}/events.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#create-2021-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_create_orders_param_order_fulfillments_fulfillment_events(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/fulfillments/{}/events.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves a specific fulfillment event.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/orders/{order_id}/fulfillments/{fulfillment_id}/events/{event_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#show-2021-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `event_id: &str` -- storefront_access_token_id.
     * * `event_id: &str` -- The ID of the fulfillment event.
     */
    pub async fn deprecated_202101_get_orders_param_order_fulfillments_fulfillment_events_event(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        event_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/fulfillments/{}/events/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
                crate::progenitor_support::encode_path(event_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Deletes a fulfillment event.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/orders/{order_id}/fulfillments/{fulfillment_id}/events/{event_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `event_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_orders_param_order_fulfillments_fulfillment_events_event(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        event_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/fulfillments/{}/events/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
                crate::progenitor_support::encode_path(event_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of fulfillment events for a specific fulfillment.
     *
     * This function performs a `GET` to the `/admin/api/unstable/orders/{order_id}/fulfillments/{fulfillment_id}/events.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#index-unstable
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- The ID of the fulfillment that's associated with the fulfillment event.
     * * `order_id: &str` -- The ID of the order that's associated with the fulfillment event.
     */
    pub async fn deprecated_unstable_get_orders_param_order_fulfillments_fulfillment_event(
        &self,
        order_id: &str,
        fulfillment_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/fulfillments/{}/events.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a fulfillment event.
     *
     * This function performs a `POST` to the `/admin/api/unstable/orders/{order_id}/fulfillments/{fulfillment_id}/events.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#create-unstable
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_create_orders_param_order_fulfillments_fulfillment_events(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/fulfillments/{}/events.json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves a specific fulfillment event.
     *
     * This function performs a `GET` to the `/admin/api/unstable/orders/{order_id}/fulfillments/{fulfillment_id}/events/{event_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#show-unstable
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `event_id: &str` -- storefront_access_token_id.
     * * `event_id: &str` -- The ID of the fulfillment event.
     */
    pub async fn deprecated_unstable_get_orders_param_order_fulfillments_fulfillment_events_event(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        event_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/fulfillments/{}/events/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
                crate::progenitor_support::encode_path(event_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Deletes a fulfillment event.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/orders/{order_id}/fulfillments/{fulfillment_id}/events/{event_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentevent#destroy-unstable
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fulfillment_id: &str` -- storefront_access_token_id.
     * * `event_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_orders_param_order_fulfillments_fulfillment_events_event(
        &self,
        order_id: &str,
        fulfillment_id: &str,
        event_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/fulfillments/{}/events/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(fulfillment_id),
                crate::progenitor_support::encode_path(event_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of fulfillment orders for a specific order.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/orders/{order_id}/fulfillment_orders.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#index-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `order_id: &str` -- The ID of the order that is associated with the fulfillment orders.
     */
    pub async fn deprecated_202001_get_orders_param_order_fulfillment_shipping_and_fulfillment(
        &self,
        order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/fulfillment_orders.json",
                crate::progenitor_support::encode_path(order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a specific fulfillment order.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/fulfillment_orders/{fulfillment_order_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#show-2020-01
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_fulfillment_orders_param_order(
        &self,
        fulfillment_order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/fulfillment_orders/{}/json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Marks a fulfillment order as cancelled.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/fulfillment_orders/{fulfillment_order_id}/cancel.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#cancel-2020-01
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_fulfillment_orders_param_order_cancel(
        &self,
        fulfillment_order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/fulfillment_orders/{}/cancel.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Marks an in progress fulfillment order as incomplete, indicating the fulfillment service
            is unable to ship any remaining items and intends to close the fulfillment order.
    *
    * This function performs a `POST` to the `/admin/api/2020-01/fulfillment_orders/{fulfillment_order_id}/close.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#close-2020-01
    *
    * **Parameters:**
    *
    * * `fulfillment_order_id: &str` -- storefront_access_token_id.
    * * `message: &str` -- An optional reason for marking the fulfillment order as incomplete.
    */
    pub async fn deprecated_202001_create_fulfillment_orders_param_order_close(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/fulfillment_orders/{}/close.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Moves a fulfillment order from one merchant managed location to another merchant managed location.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/fulfillment_orders/{fulfillment_order_id}/move.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#move-2020-01
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `new_location_id: &str` -- The id of the location to which the fulfillment order will be moved.
     */
    pub async fn deprecated_202001_create_fulfillment_orders_param_order_move(
        &self,
        fulfillment_order_id: &str,
        new_location_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !new_location_id.is_empty() {
            query_args.push(("new_location_id".to_string(), new_location_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/fulfillment_orders/{}/move.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves a list of fulfillment orders for a specific order.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/orders/{order_id}/fulfillment_orders.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#index-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `order_id: &str` -- The ID of the order that is associated with the fulfillment orders.
     */
    pub async fn deprecated_202004_get_orders_param_order_fulfillment_shipping_and_fulfillment(
        &self,
        order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/fulfillment_orders.json",
                crate::progenitor_support::encode_path(order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a specific fulfillment order.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/fulfillment_orders/{fulfillment_order_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#show-2020-04
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_fulfillment_orders_param_order(
        &self,
        fulfillment_order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/fulfillment_orders/{}/json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Marks a fulfillment order as cancelled.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/fulfillment_orders/{fulfillment_order_id}/cancel.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#cancel-2020-04
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_create_fulfillment_orders_param_order_cancel(
        &self,
        fulfillment_order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/fulfillment_orders/{}/cancel.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Marks an in progress fulfillment order as incomplete, indicating the fulfillment service
            is unable to ship any remaining items and intends to close the fulfillment order.
    *
    * This function performs a `POST` to the `/admin/api/2020-04/fulfillment_orders/{fulfillment_order_id}/close.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#close-2020-04
    *
    * **Parameters:**
    *
    * * `fulfillment_order_id: &str` -- storefront_access_token_id.
    * * `message: &str` -- An optional reason for marking the fulfillment order as incomplete.
    */
    pub async fn deprecated_202004_create_fulfillment_orders_param_order_close(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/fulfillment_orders/{}/close.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Moves a fulfillment order from one merchant managed location to another merchant managed location.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/fulfillment_orders/{fulfillment_order_id}/move.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#move-2020-04
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `new_location_id: &str` -- The id of the location to which the fulfillment order will be moved.
     */
    pub async fn deprecated_202004_create_fulfillment_orders_param_order_move(
        &self,
        fulfillment_order_id: &str,
        new_location_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !new_location_id.is_empty() {
            query_args.push(("new_location_id".to_string(), new_location_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/fulfillment_orders/{}/move.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves a list of fulfillment orders for a specific order.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/orders/{order_id}/fulfillment_orders.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#index-2020-07
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `order_id: &str` -- The ID of the order that is associated with the fulfillment orders.
     */
    pub async fn deprecated_202007_get_orders_param_order_fulfillment_shipping_and_fulfillment(
        &self,
        order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/fulfillment_orders.json",
                crate::progenitor_support::encode_path(order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a specific fulfillment order.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/fulfillment_orders/{fulfillment_order_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#show-2020-07
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_fulfillment_orders_param_order(
        &self,
        fulfillment_order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/fulfillment_orders/{}/json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Marks a fulfillment order as cancelled.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/fulfillment_orders/{fulfillment_order_id}/cancel.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#cancel-2020-07
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_create_fulfillment_orders_param_order_cancel(
        &self,
        fulfillment_order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/fulfillment_orders/{}/cancel.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Marks an in progress fulfillment order as incomplete, indicating the fulfillment service
            is unable to ship any remaining items and intends to close the fulfillment order.
    *
    * This function performs a `POST` to the `/admin/api/2020-07/fulfillment_orders/{fulfillment_order_id}/close.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#close-2020-07
    *
    * **Parameters:**
    *
    * * `fulfillment_order_id: &str` -- storefront_access_token_id.
    * * `message: &str` -- An optional reason for marking the fulfillment order as incomplete.
    */
    pub async fn deprecated_202007_create_fulfillment_orders_param_order_close(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/fulfillment_orders/{}/close.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Moves a fulfillment order from one merchant managed location to another merchant managed location.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/fulfillment_orders/{fulfillment_order_id}/move.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#move-2020-07
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `new_location_id: &str` -- The id of the location to which the fulfillment order will be moved.
     */
    pub async fn deprecated_202007_create_fulfillment_orders_param_order_move(
        &self,
        fulfillment_order_id: &str,
        new_location_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !new_location_id.is_empty() {
            query_args.push(("new_location_id".to_string(), new_location_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/fulfillment_orders/{}/move.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves a list of fulfillment orders for a specific order.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/orders/{order_id}/fulfillment_orders.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#index-2020-10
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `order_id: &str` -- The ID of the order that is associated with the fulfillment orders.
     */
    pub async fn get_orders_param_order_fulfillment_shipping_and_fulfillment(
        &self,
        order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/fulfillment_orders.json",
                crate::progenitor_support::encode_path(order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a specific fulfillment order.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/fulfillment_orders/{fulfillment_order_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#show-2020-10
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     */
    pub async fn get_fulfillment_orders_param_order(
        &self,
        fulfillment_order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/fulfillment_orders/{}/json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Marks a fulfillment order as cancelled.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/fulfillment_orders/{fulfillment_order_id}/cancel.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#cancel-2020-10
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     */
    pub async fn create_fulfillment_orders_param_order_cancel(
        &self,
        fulfillment_order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/fulfillment_orders/{}/cancel.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Marks an in progress fulfillment order as incomplete, indicating the fulfillment service
            is unable to ship any remaining items and intends to close the fulfillment order.
    *
    * This function performs a `POST` to the `/admin/api/2020-10/fulfillment_orders/{fulfillment_order_id}/close.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#close-2020-10
    *
    * **Parameters:**
    *
    * * `fulfillment_order_id: &str` -- storefront_access_token_id.
    * * `message: &str` -- An optional reason for marking the fulfillment order as incomplete.
    */
    pub async fn create_fulfillment_orders_param_order_close(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/fulfillment_orders/{}/close.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Moves a fulfillment order from one merchant managed location to another merchant managed location.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/fulfillment_orders/{fulfillment_order_id}/move.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#move-2020-10
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `new_location_id: &str` -- The id of the location to which the fulfillment order will be moved.
     */
    pub async fn create_fulfillment_orders_param_order_move(
        &self,
        fulfillment_order_id: &str,
        new_location_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !new_location_id.is_empty() {
            query_args.push(("new_location_id".to_string(), new_location_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/fulfillment_orders/{}/move.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves a list of fulfillment orders for a specific order.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/orders/{order_id}/fulfillment_orders.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#index-2021-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `order_id: &str` -- The ID of the order that is associated with the fulfillment orders.
     */
    pub async fn deprecated_202101_get_orders_param_order_fulfillment_shipping_and_fulfillment(
        &self,
        order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/fulfillment_orders.json",
                crate::progenitor_support::encode_path(order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a specific fulfillment order.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/fulfillment_orders/{fulfillment_order_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#show-2021-01
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_fulfillment_orders_param_order(
        &self,
        fulfillment_order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/fulfillment_orders/{}/json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Marks a fulfillment order as cancelled.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/fulfillment_orders/{fulfillment_order_id}/cancel.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#cancel-2021-01
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_create_fulfillment_orders_param_order_cancel(
        &self,
        fulfillment_order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/fulfillment_orders/{}/cancel.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Marks an in progress fulfillment order as incomplete, indicating the fulfillment service
            is unable to ship any remaining items and intends to close the fulfillment order.
    *
    * This function performs a `POST` to the `/admin/api/2021-01/fulfillment_orders/{fulfillment_order_id}/close.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#close-2021-01
    *
    * **Parameters:**
    *
    * * `fulfillment_order_id: &str` -- storefront_access_token_id.
    * * `message: &str` -- An optional reason for marking the fulfillment order as incomplete.
    */
    pub async fn deprecated_202101_create_fulfillment_orders_param_order_close(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/fulfillment_orders/{}/close.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Moves a fulfillment order from one merchant managed location to another merchant managed location.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/fulfillment_orders/{fulfillment_order_id}/move.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#move-2021-01
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `new_location_id: &str` -- The id of the location to which the fulfillment order will be moved.
     */
    pub async fn deprecated_202101_create_fulfillment_orders_param_order_move(
        &self,
        fulfillment_order_id: &str,
        new_location_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !new_location_id.is_empty() {
            query_args.push(("new_location_id".to_string(), new_location_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/fulfillment_orders/{}/move.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Marks a scheduled fulfillment order as ready for fulfillment.
              This endpoint allows merchants to work on a scheduled fulfillment order before its expected fulfill_at datetime.
    *
    * This function performs a `POST` to the `/admin/api/2021-01/fulfillment_orders/{fulfillment_order_id}/open.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#open-2021-01
    *
    * **Parameters:**
    *
    * * `fulfillment_order_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202101_create_fulfillment_orders_param_order_open(
        &self,
        fulfillment_order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/fulfillment_orders/{}/open.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Updates the fulfill_at time of a scheduled fulfillment order.
              This endpoint is used to manage the time a scheduled fulfillment order will be marked as ready for fulfillment.
    *
    * This function performs a `POST` to the `/admin/api/2021-01/fulfillment_orders/{fulfillment_order_id}/reschedule.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#reschedule-2021-01
    *
    * **Parameters:**
    *
    * * `fulfillment_order_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202101_create_fulfillment_orders_param_order_reschedule(
        &self,
        fulfillment_order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/fulfillment_orders/{}/reschedule.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves a list of fulfillment orders for a specific order.
     *
     * This function performs a `GET` to the `/admin/api/unstable/orders/{order_id}/fulfillment_orders.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#index-unstable
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `order_id: &str` -- The ID of the order that is associated with the fulfillment orders.
     */
    pub async fn deprecated_unstable_get_orders_param_order_fulfillment_shipping_and_fulfillment(
        &self,
        order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/fulfillment_orders.json",
                crate::progenitor_support::encode_path(order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a specific fulfillment order.
     *
     * This function performs a `GET` to the `/admin/api/unstable/fulfillment_orders/{fulfillment_order_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#show-unstable
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_fulfillment_orders_param_order(
        &self,
        fulfillment_order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/fulfillment_orders/{}/json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Marks a fulfillment order as cancelled.
     *
     * This function performs a `POST` to the `/admin/api/unstable/fulfillment_orders/{fulfillment_order_id}/cancel.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#cancel-unstable
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_create_fulfillment_orders_param_order_cancel(
        &self,
        fulfillment_order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/fulfillment_orders/{}/cancel.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Releases the fulfillment order holds for a specific order. Fulfillment orders are created
              with an ON_HOLD status if the channel that created the order has a fulfillment hold policy.
    *
    * This function performs a `POST` to the `/admin/api/unstable/fulfillment_orders/release_hold.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#release_hold-unstable
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- The ID of the order that is associated to the fulfillment orders.
    */
    pub async fn deprecated_unstable_create_fulfillment_orders_release_hold(
        &self,
        order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !order_id.is_empty() {
            query_args.push(("order_id".to_string(), order_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/fulfillment_orders/release_hold.json?{}",
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Marks an in progress fulfillment order as incomplete, indicating the fulfillment service
            is unable to ship any remaining items and intends to close the fulfillment order.
    *
    * This function performs a `POST` to the `/admin/api/unstable/fulfillment_orders/{fulfillment_order_id}/close.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#close-unstable
    *
    * **Parameters:**
    *
    * * `fulfillment_order_id: &str` -- storefront_access_token_id.
    * * `message: &str` -- An optional reason for marking the fulfillment order as incomplete.
    */
    pub async fn deprecated_unstable_create_fulfillment_orders_param_order_close(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/fulfillment_orders/{}/close.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Moves a fulfillment order from one merchant managed location to another merchant managed location.
     *
     * This function performs a `POST` to the `/admin/api/unstable/fulfillment_orders/{fulfillment_order_id}/move.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#move-unstable
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `new_location_id: &str` -- The id of the location to which the fulfillment order will be moved.
     */
    pub async fn deprecated_unstable_create_fulfillment_orders_param_order_move(
        &self,
        fulfillment_order_id: &str,
        new_location_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !new_location_id.is_empty() {
            query_args.push(("new_location_id".to_string(), new_location_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/fulfillment_orders/{}/move.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Marks a scheduled fulfillment order as ready for fulfillment.
              This endpoint allows merchants to work on a scheduled fulfillment order before its expected fulfill_at datetime.
    *
    * This function performs a `POST` to the `/admin/api/unstable/fulfillment_orders/{fulfillment_order_id}/open.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#open-unstable
    *
    * **Parameters:**
    *
    * * `fulfillment_order_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_unstable_create_fulfillment_orders_param_order_open(
        &self,
        fulfillment_order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/fulfillment_orders/{}/open.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
    * Updates the fulfill_at time of a scheduled fulfillment order.
              This endpoint is used to manage the time a scheduled fulfillment order will be marked as ready for fulfillment.
    *
    * This function performs a `POST` to the `/admin/api/unstable/fulfillment_orders/{fulfillment_order_id}/reschedule.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentorder#reschedule-unstable
    *
    * **Parameters:**
    *
    * * `fulfillment_order_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_unstable_create_fulfillment_orders_param_order_reschedule(
        &self,
        fulfillment_order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/fulfillment_orders/{}/reschedule.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Sends a fulfillment request to the fulfillment service of a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/fulfillment_orders/{fulfillment_order_id}/fulfillment_request.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentrequest#create-2020-01
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional message for the fulfillment request.
     * * `fulfillment_order_line_items: &str` -- The fulfillment order line items to be requested for fulfillment. If left blank, all line items of the fulfillment order are requested for fulfillment.
     */
    pub async fn deprecated_202001_create_fulfillment_orders_param_order_request(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        fulfillment_order_line_items: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fulfillment_order_line_items.is_empty() {
            query_args.push((
                "fulfillment_order_line_items".to_string(),
                fulfillment_order_line_items.to_string(),
            ));
        }
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/fulfillment_orders/{}/fulfillment_request.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Accepts a fulfillment request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/fulfillment_orders/{fulfillment_order_id}/fulfillment_request/accept.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentrequest#accept-2020-01
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for accepting the fulfillment request.
     */
    pub async fn deprecated_202001_create_fulfillment_orders_param_order_request_accept(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/fulfillment_orders/{}/fulfillment_request/accept.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Rejects a fulfillment request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/fulfillment_orders/{fulfillment_order_id}/fulfillment_request/reject.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentrequest#reject-2020-01
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for rejecting the fulfillment request.
     */
    pub async fn deprecated_202001_create_fulfillment_orders_param_order_request_reject(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/fulfillment_orders/{}/fulfillment_request/reject.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Sends a fulfillment request to the fulfillment service of a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/fulfillment_orders/{fulfillment_order_id}/fulfillment_request.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentrequest#create-2020-04
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional message for the fulfillment request.
     * * `fulfillment_order_line_items: &str` -- The fulfillment order line items to be requested for fulfillment. If left blank, all line items of the fulfillment order are requested for fulfillment.
     */
    pub async fn deprecated_202004_create_fulfillment_orders_param_order_request(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        fulfillment_order_line_items: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fulfillment_order_line_items.is_empty() {
            query_args.push((
                "fulfillment_order_line_items".to_string(),
                fulfillment_order_line_items.to_string(),
            ));
        }
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/fulfillment_orders/{}/fulfillment_request.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Accepts a fulfillment request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/fulfillment_orders/{fulfillment_order_id}/fulfillment_request/accept.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentrequest#accept-2020-04
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for accepting the fulfillment request.
     */
    pub async fn deprecated_202004_create_fulfillment_orders_param_order_request_accept(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/fulfillment_orders/{}/fulfillment_request/accept.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Rejects a fulfillment request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/fulfillment_orders/{fulfillment_order_id}/fulfillment_request/reject.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentrequest#reject-2020-04
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for rejecting the fulfillment request.
     */
    pub async fn deprecated_202004_create_fulfillment_orders_param_order_request_reject(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/fulfillment_orders/{}/fulfillment_request/reject.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Sends a fulfillment request to the fulfillment service of a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/fulfillment_orders/{fulfillment_order_id}/fulfillment_request.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentrequest#create-2020-07
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional message for the fulfillment request.
     * * `fulfillment_order_line_items: &str` -- The fulfillment order line items to be requested for fulfillment. If left blank, all line items of the fulfillment order are requested for fulfillment.
     */
    pub async fn deprecated_202007_create_fulfillment_orders_param_order_request(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        fulfillment_order_line_items: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fulfillment_order_line_items.is_empty() {
            query_args.push((
                "fulfillment_order_line_items".to_string(),
                fulfillment_order_line_items.to_string(),
            ));
        }
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/fulfillment_orders/{}/fulfillment_request.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Accepts a fulfillment request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/fulfillment_orders/{fulfillment_order_id}/fulfillment_request/accept.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentrequest#accept-2020-07
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for accepting the fulfillment request.
     */
    pub async fn deprecated_202007_create_fulfillment_orders_param_order_request_accept(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/fulfillment_orders/{}/fulfillment_request/accept.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Rejects a fulfillment request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/fulfillment_orders/{fulfillment_order_id}/fulfillment_request/reject.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentrequest#reject-2020-07
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for rejecting the fulfillment request.
     */
    pub async fn deprecated_202007_create_fulfillment_orders_param_order_request_reject(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/fulfillment_orders/{}/fulfillment_request/reject.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Sends a fulfillment request to the fulfillment service of a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/fulfillment_orders/{fulfillment_order_id}/fulfillment_request.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentrequest#create-2020-10
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional message for the fulfillment request.
     * * `fulfillment_order_line_items: &str` -- The fulfillment order line items to be requested for fulfillment. If left blank, all line items of the fulfillment order are requested for fulfillment.
     */
    pub async fn create_fulfillment_orders_param_order_request(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        fulfillment_order_line_items: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fulfillment_order_line_items.is_empty() {
            query_args.push((
                "fulfillment_order_line_items".to_string(),
                fulfillment_order_line_items.to_string(),
            ));
        }
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/fulfillment_orders/{}/fulfillment_request.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Accepts a fulfillment request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/fulfillment_orders/{fulfillment_order_id}/fulfillment_request/accept.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentrequest#accept-2020-10
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for accepting the fulfillment request.
     */
    pub async fn create_fulfillment_orders_param_order_request_accept(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/fulfillment_orders/{}/fulfillment_request/accept.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Rejects a fulfillment request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/fulfillment_orders/{fulfillment_order_id}/fulfillment_request/reject.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentrequest#reject-2020-10
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for rejecting the fulfillment request.
     */
    pub async fn create_fulfillment_orders_param_order_request_reject(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/fulfillment_orders/{}/fulfillment_request/reject.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Sends a fulfillment request to the fulfillment service of a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/fulfillment_orders/{fulfillment_order_id}/fulfillment_request.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentrequest#create-2021-01
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional message for the fulfillment request.
     * * `fulfillment_order_line_items: &str` -- The fulfillment order line items to be requested for fulfillment. If left blank, all line items of the fulfillment order are requested for fulfillment.
     */
    pub async fn deprecated_202101_create_fulfillment_orders_param_order_request(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        fulfillment_order_line_items: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fulfillment_order_line_items.is_empty() {
            query_args.push((
                "fulfillment_order_line_items".to_string(),
                fulfillment_order_line_items.to_string(),
            ));
        }
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/fulfillment_orders/{}/fulfillment_request.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Accepts a fulfillment request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/fulfillment_orders/{fulfillment_order_id}/fulfillment_request/accept.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentrequest#accept-2021-01
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for accepting the fulfillment request.
     */
    pub async fn deprecated_202101_create_fulfillment_orders_param_order_request_accept(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/fulfillment_orders/{}/fulfillment_request/accept.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Rejects a fulfillment request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/fulfillment_orders/{fulfillment_order_id}/fulfillment_request/reject.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentrequest#reject-2021-01
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for rejecting the fulfillment request.
     */
    pub async fn deprecated_202101_create_fulfillment_orders_param_order_request_reject(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/fulfillment_orders/{}/fulfillment_request/reject.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Sends a fulfillment request to the fulfillment service of a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/unstable/fulfillment_orders/{fulfillment_order_id}/fulfillment_request.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentrequest#create-unstable
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional message for the fulfillment request.
     * * `fulfillment_order_line_items: &str` -- The fulfillment order line items to be requested for fulfillment. If left blank, all line items of the fulfillment order are requested for fulfillment.
     */
    pub async fn deprecated_unstable_create_fulfillment_orders_param_order_request(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        fulfillment_order_line_items: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fulfillment_order_line_items.is_empty() {
            query_args.push((
                "fulfillment_order_line_items".to_string(),
                fulfillment_order_line_items.to_string(),
            ));
        }
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/fulfillment_orders/{}/fulfillment_request.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Accepts a fulfillment request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/unstable/fulfillment_orders/{fulfillment_order_id}/fulfillment_request/accept.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentrequest#accept-unstable
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for accepting the fulfillment request.
     */
    pub async fn deprecated_unstable_create_fulfillment_orders_param_order_request_accept(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/fulfillment_orders/{}/fulfillment_request/accept.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Rejects a fulfillment request sent to a fulfillment service for a fulfillment order.
     *
     * This function performs a `POST` to the `/admin/api/unstable/fulfillment_orders/{fulfillment_order_id}/fulfillment_request/reject.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentrequest#reject-unstable
     *
     * **Parameters:**
     *
     * * `fulfillment_order_id: &str` -- storefront_access_token_id.
     * * `message: &str` -- An optional reason for rejecting the fulfillment request.
     */
    pub async fn deprecated_unstable_create_fulfillment_orders_param_order_request_reject(
        &self,
        fulfillment_order_id: &str,
        message: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !message.is_empty() {
            query_args.push(("message".to_string(), message.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/fulfillment_orders/{}/fulfillment_request/reject.json?{}",
                crate::progenitor_support::encode_path(fulfillment_order_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Get tracking numbers for orders.
     *
     * This function performs a `GET` to the `/fetch_tracking_numbers` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#fetch-tracking-numbers
     *
     * **Parameters:**
     *
     * * `order_names: &str` -- The fulfillment names we require tracking numbers for (i.e. #1001.1).
     * * `shop: &str` -- The shop's myshopify url.
     */
    pub async fn deprecated_unknown_version_get_fetch_tracking_number(
        &self,
        order_names: &str,
        shop: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !order_names.is_empty() {
            query_args.push(("order_names".to_string(), order_names.to_string()));
        }
        if !shop.is_empty() {
            query_args.push(("shop".to_string(), shop.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/fetch_tracking_numbers?{}", query_), None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get inventory levels.
     *
     * This function performs a `GET` to the `/fetch_stock` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#fetch-stock
     *
     * **Parameters:**
     *
     * * `max_retries: &str` -- The maximum amount of times Shopify will send the request for inventory levels.
     * * `shop: &str` -- The shop's myshopify url.
     * * `sku: &str` -- The SKU for the Product Variant we need stock levels for.
     * * `timestamp: &str` -- The Unix timestamp from when the inventory request was made.
     */
    pub async fn deprecated_unknown_version_get_fetch_stock(
        &self,
        max_retries: &str,
        shop: &str,
        sku: &str,
        timestamp: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !max_retries.is_empty() {
            query_args.push(("max_retries".to_string(), max_retries.to_string()));
        }
        if !shop.is_empty() {
            query_args.push(("shop".to_string(), shop.to_string()));
        }
        if !sku.is_empty() {
            query_args.push(("sku".to_string(), sku.to_string()));
        }
        if !timestamp.is_empty() {
            query_args.push(("timestamp".to_string(), timestamp.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/fetch_stock?{}", query_), None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `GET` to the `/admin/api/2020-01/fulfillment_services.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#index-2020-01
     *
     * **Parameters:**
     *
     * * `scope: &str` -- current_client: Returns fulfillment providers that have been created by the app sending the request (default)
     *                           all: Returns all the fulfillment providers.
     */
    pub async fn deprecated_202001_get_fulfillment_service(&self, scope: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !scope.is_empty() {
            query_args.push(("scope".to_string(), scope.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/fulfillment_services.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * To create a fulfillment service, you can also use a cURL request that uses that fulfillment_service.json payload:
              Copy  curl -X POST -d @fulfillment_service.json -H"Accept:application/json" -H"Content-Type:application/json" -H"X-Shopify-Access-Token:THE_TOKEN_GOES_HERE" https://AUTHORIZED_SHOP.myshopify.com/admin/fulfillment_services

              Where THE_TOKEN_GOES_HERE is replaced by the OAuth token given to you by Shopify and https://AUTHORIZED_SHOP.myshopify.com/admin/fulfillment_services is replaced by the authorized shop's URL.
    *
    * This function performs a `POST` to the `/admin/api/2020-01/fulfillment_services.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#create-2020-01
    */
    pub async fn deprecated_202001_create_fulfillment_services(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-01/fulfillment_services.json", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `GET` to the `/admin/api/2020-01/fulfillment_services/{fulfillment_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#show-2020-01
     *
     * **Parameters:**
     *
     * * `fulfillment_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_fulfillment_services_param_service(
        &self,
        fulfillment_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/fulfillment_services/{}/json",
                crate::progenitor_support::encode_path(fulfillment_service_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/fulfillment_services/{fulfillment_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#update-2020-01
     *
     * **Parameters:**
     *
     * * `fulfillment_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_fulfillment_services_param_service(
        &self,
        fulfillment_service_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/fulfillment_services/{}/json",
                crate::progenitor_support::encode_path(fulfillment_service_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/fulfillment_services/{fulfillment_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `fulfillment_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_fulfillment_services_param_service(
        &self,
        fulfillment_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/fulfillment_services/{}/json",
                crate::progenitor_support::encode_path(fulfillment_service_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `GET` to the `/admin/api/2020-04/fulfillment_services.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#index-2020-04
     *
     * **Parameters:**
     *
     * * `scope: &str` -- current_client: Returns fulfillment providers that have been created by the app sending the request (default)
     *                           all: Returns all the fulfillment providers.
     */
    pub async fn deprecated_202004_get_fulfillment_service(&self, scope: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !scope.is_empty() {
            query_args.push(("scope".to_string(), scope.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/fulfillment_services.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * To create a fulfillment service, you can also use a cURL request that uses that fulfillment_service.json payload:
              Copy  curl -X POST -d @fulfillment_service.json -H"Accept:application/json" -H"Content-Type:application/json" -H"X-Shopify-Access-Token:THE_TOKEN_GOES_HERE" https://AUTHORIZED_SHOP.myshopify.com/admin/fulfillment_services

              Where THE_TOKEN_GOES_HERE is replaced by the OAuth token given to you by Shopify and https://AUTHORIZED_SHOP.myshopify.com/admin/fulfillment_services is replaced by the authorized shop's URL.
    *
    * This function performs a `POST` to the `/admin/api/2020-04/fulfillment_services.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#create-2020-04
    */
    pub async fn deprecated_202004_create_fulfillment_services(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-04/fulfillment_services.json", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `GET` to the `/admin/api/2020-04/fulfillment_services/{fulfillment_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#show-2020-04
     *
     * **Parameters:**
     *
     * * `fulfillment_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_fulfillment_services_param_service(
        &self,
        fulfillment_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/fulfillment_services/{}/json",
                crate::progenitor_support::encode_path(fulfillment_service_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/fulfillment_services/{fulfillment_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#update-2020-04
     *
     * **Parameters:**
     *
     * * `fulfillment_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_fulfillment_services_param_service(
        &self,
        fulfillment_service_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/fulfillment_services/{}/json",
                crate::progenitor_support::encode_path(fulfillment_service_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/fulfillment_services/{fulfillment_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `fulfillment_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_fulfillment_services_param_service(
        &self,
        fulfillment_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/fulfillment_services/{}/json",
                crate::progenitor_support::encode_path(fulfillment_service_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `GET` to the `/admin/api/2020-07/fulfillment_services.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#index-2020-07
     *
     * **Parameters:**
     *
     * * `scope: &str` -- current_client: Returns fulfillment providers that have been created by the app sending the request (default)
     *                           all: Returns all the fulfillment providers.
     */
    pub async fn deprecated_202007_get_fulfillment_service(&self, scope: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !scope.is_empty() {
            query_args.push(("scope".to_string(), scope.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/fulfillment_services.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * To create a fulfillment service, you can also use a cURL request that uses that fulfillment_service.json payload:
              Copy  curl -X POST -d @fulfillment_service.json -H"Accept:application/json" -H"Content-Type:application/json" -H"X-Shopify-Access-Token:THE_TOKEN_GOES_HERE" https://AUTHORIZED_SHOP.myshopify.com/admin/fulfillment_services

              Where THE_TOKEN_GOES_HERE is replaced by the OAuth token given to you by Shopify and https://AUTHORIZED_SHOP.myshopify.com/admin/fulfillment_services is replaced by the authorized shop's URL.
    *
    * This function performs a `POST` to the `/admin/api/2020-07/fulfillment_services.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#create-2020-07
    */
    pub async fn deprecated_202007_create_fulfillment_services(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-07/fulfillment_services.json", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `GET` to the `/admin/api/2020-07/fulfillment_services/{fulfillment_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#show-2020-07
     *
     * **Parameters:**
     *
     * * `fulfillment_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_fulfillment_services_param_service(
        &self,
        fulfillment_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/fulfillment_services/{}/json",
                crate::progenitor_support::encode_path(fulfillment_service_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/fulfillment_services/{fulfillment_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#update-2020-07
     *
     * **Parameters:**
     *
     * * `fulfillment_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_fulfillment_services_param_service(
        &self,
        fulfillment_service_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/fulfillment_services/{}/json",
                crate::progenitor_support::encode_path(fulfillment_service_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/fulfillment_services/{fulfillment_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `fulfillment_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_fulfillment_services_param_service(
        &self,
        fulfillment_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/fulfillment_services/{}/json",
                crate::progenitor_support::encode_path(fulfillment_service_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `GET` to the `/admin/api/2020-10/fulfillment_services.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#index-2020-10
     *
     * **Parameters:**
     *
     * * `scope: &str` -- current_client: Returns fulfillment providers that have been created by the app sending the request (default)
     *                           all: Returns all the fulfillment providers.
     */
    pub async fn get_fulfillment_service(&self, scope: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !scope.is_empty() {
            query_args.push(("scope".to_string(), scope.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/fulfillment_services.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * To create a fulfillment service, you can also use a cURL request that uses that fulfillment_service.json payload:
              Copy  curl -X POST -d @fulfillment_service.json -H"Accept:application/json" -H"Content-Type:application/json" -H"X-Shopify-Access-Token:THE_TOKEN_GOES_HERE" https://AUTHORIZED_SHOP.myshopify.com/admin/fulfillment_services

              Where THE_TOKEN_GOES_HERE is replaced by the OAuth token given to you by Shopify and https://AUTHORIZED_SHOP.myshopify.com/admin/fulfillment_services is replaced by the authorized shop's URL.
    *
    * This function performs a `POST` to the `/admin/api/2020-10/fulfillment_services.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#create-2020-10
    */
    pub async fn create_fulfillment_services(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-10/fulfillment_services.json", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `GET` to the `/admin/api/2020-10/fulfillment_services/{fulfillment_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#show-2020-10
     *
     * **Parameters:**
     *
     * * `fulfillment_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn get_fulfillment_services_param_service(
        &self,
        fulfillment_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/fulfillment_services/{}/json",
                crate::progenitor_support::encode_path(fulfillment_service_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/fulfillment_services/{fulfillment_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#update-2020-10
     *
     * **Parameters:**
     *
     * * `fulfillment_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn update_fulfillment_services_param_service(
        &self,
        fulfillment_service_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/fulfillment_services/{}/json",
                crate::progenitor_support::encode_path(fulfillment_service_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/fulfillment_services/{fulfillment_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `fulfillment_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_fulfillment_services_param_service(
        &self,
        fulfillment_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/fulfillment_services/{}/json",
                crate::progenitor_support::encode_path(fulfillment_service_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `GET` to the `/admin/api/2021-01/fulfillment_services.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#index-2021-01
     *
     * **Parameters:**
     *
     * * `scope: &str` -- current_client: Returns fulfillment providers that have been created by the app sending the request (default)
     *                           all: Returns all the fulfillment providers.
     */
    pub async fn deprecated_202101_get_fulfillment_service(&self, scope: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !scope.is_empty() {
            query_args.push(("scope".to_string(), scope.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/fulfillment_services.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * To create a fulfillment service, you can also use a cURL request that uses that fulfillment_service.json payload:
              Copy  curl -X POST -d @fulfillment_service.json -H"Accept:application/json" -H"Content-Type:application/json" -H"X-Shopify-Access-Token:THE_TOKEN_GOES_HERE" https://AUTHORIZED_SHOP.myshopify.com/admin/fulfillment_services

              Where THE_TOKEN_GOES_HERE is replaced by the OAuth token given to you by Shopify and https://AUTHORIZED_SHOP.myshopify.com/admin/fulfillment_services is replaced by the authorized shop's URL.
    *
    * This function performs a `POST` to the `/admin/api/2021-01/fulfillment_services.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#create-2021-01
    */
    pub async fn deprecated_202101_create_fulfillment_services(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2021-01/fulfillment_services.json", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `GET` to the `/admin/api/2021-01/fulfillment_services/{fulfillment_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#show-2021-01
     *
     * **Parameters:**
     *
     * * `fulfillment_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_fulfillment_services_param_service(
        &self,
        fulfillment_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/fulfillment_services/{}/json",
                crate::progenitor_support::encode_path(fulfillment_service_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/fulfillment_services/{fulfillment_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#update-2021-01
     *
     * **Parameters:**
     *
     * * `fulfillment_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_fulfillment_services_param_service(
        &self,
        fulfillment_service_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/fulfillment_services/{}/json",
                crate::progenitor_support::encode_path(fulfillment_service_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/fulfillment_services/{fulfillment_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `fulfillment_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_fulfillment_services_param_service(
        &self,
        fulfillment_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/fulfillment_services/{}/json",
                crate::progenitor_support::encode_path(fulfillment_service_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `GET` to the `/admin/api/unstable/fulfillment_services.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#index-unstable
     *
     * **Parameters:**
     *
     * * `scope: &str` -- current_client: Returns fulfillment providers that have been created by the app sending the request (default)
     *                           all: Returns all the fulfillment providers.
     */
    pub async fn deprecated_unstable_get_fulfillment_service(
        &self,
        scope: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !scope.is_empty() {
            query_args.push(("scope".to_string(), scope.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/fulfillment_services.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * To create a fulfillment service, you can also use a cURL request that uses that fulfillment_service.json payload:
              Copy  curl -X POST -d @fulfillment_service.json -H"Accept:application/json" -H"Content-Type:application/json" -H"X-Shopify-Access-Token:THE_TOKEN_GOES_HERE" https://AUTHORIZED_SHOP.myshopify.com/admin/fulfillment_services

              Where THE_TOKEN_GOES_HERE is replaced by the OAuth token given to you by Shopify and https://AUTHORIZED_SHOP.myshopify.com/admin/fulfillment_services is replaced by the authorized shop's URL.
    *
    * This function performs a `POST` to the `/admin/api/unstable/fulfillment_services.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#create-unstable
    */
    pub async fn deprecated_unstable_create_fulfillment_services(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/unstable/fulfillment_services.json", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `GET` to the `/admin/api/unstable/fulfillment_services/{fulfillment_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#show-unstable
     *
     * **Parameters:**
     *
     * * `fulfillment_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_fulfillment_services_param_service(
        &self,
        fulfillment_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/fulfillment_services/{}/json",
                crate::progenitor_support::encode_path(fulfillment_service_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `PUT` to the `/admin/api/unstable/fulfillment_services/{fulfillment_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#update-unstable
     *
     * **Parameters:**
     *
     * * `fulfillment_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_fulfillment_services_param_service(
        &self,
        fulfillment_service_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/fulfillment_services/{}/json",
                crate::progenitor_support::encode_path(fulfillment_service_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * .
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/fulfillment_services/{fulfillment_service_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/fulfillmentservice#destroy-unstable
     *
     * **Parameters:**
     *
     * * `fulfillment_service_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_fulfillment_services_param_service(
        &self,
        fulfillment_service_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/fulfillment_services/{}/json",
                crate::progenitor_support::encode_path(fulfillment_service_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Retrieves a list of locations that a fulfillment order can potentially move to.
              The resulting list is sorted alphabetically in ascending order by location name.
    *
    * This function performs a `GET` to the `/admin/api/2020-01/fulfillment_orders/{fulfillment_order_id}/locations_for_move.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/locationsformove#index-2020-01
    *
    * **Parameters:**
    *
    * * `fulfillment_order_id: &str` -- storefront_access_token_id.
    * * `fulfillment_order_id: &str` -- The ID of the fulfillment order.
    */
    pub async fn deprecated_202001_get_fulfillment_orders_param_order_locations_for_move(
        &self,
        fulfillment_order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/fulfillment_orders/{}/locations_for_move.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Retrieves a list of locations that a fulfillment order can potentially move to.
              The resulting list is sorted alphabetically in ascending order by location name.
    *
    * This function performs a `GET` to the `/admin/api/2020-04/fulfillment_orders/{fulfillment_order_id}/locations_for_move.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/locationsformove#index-2020-04
    *
    * **Parameters:**
    *
    * * `fulfillment_order_id: &str` -- storefront_access_token_id.
    * * `fulfillment_order_id: &str` -- The ID of the fulfillment order.
    */
    pub async fn deprecated_202004_get_fulfillment_orders_param_order_locations_for_move(
        &self,
        fulfillment_order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/fulfillment_orders/{}/locations_for_move.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Retrieves a list of locations that a fulfillment order can potentially move to.
              The resulting list is sorted alphabetically in ascending order by location name.
    *
    * This function performs a `GET` to the `/admin/api/2020-07/fulfillment_orders/{fulfillment_order_id}/locations_for_move.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/locationsformove#index-2020-07
    *
    * **Parameters:**
    *
    * * `fulfillment_order_id: &str` -- storefront_access_token_id.
    * * `fulfillment_order_id: &str` -- The ID of the fulfillment order.
    */
    pub async fn deprecated_202007_get_fulfillment_orders_param_order_locations_for_move(
        &self,
        fulfillment_order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/fulfillment_orders/{}/locations_for_move.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Retrieves a list of locations that a fulfillment order can potentially move to.
              The resulting list is sorted alphabetically in ascending order by location name.
    *
    * This function performs a `GET` to the `/admin/api/2020-10/fulfillment_orders/{fulfillment_order_id}/locations_for_move.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/locationsformove#index-2020-10
    *
    * **Parameters:**
    *
    * * `fulfillment_order_id: &str` -- storefront_access_token_id.
    * * `fulfillment_order_id: &str` -- The ID of the fulfillment order.
    */
    pub async fn get_fulfillment_orders_param_order_locations_for_move(
        &self,
        fulfillment_order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/fulfillment_orders/{}/locations_for_move.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Retrieves a list of locations that a fulfillment order can potentially move to.
              The resulting list is sorted alphabetically in ascending order by location name.
    *
    * This function performs a `GET` to the `/admin/api/2021-01/fulfillment_orders/{fulfillment_order_id}/locations_for_move.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/locationsformove#index-2021-01
    *
    * **Parameters:**
    *
    * * `fulfillment_order_id: &str` -- storefront_access_token_id.
    * * `fulfillment_order_id: &str` -- The ID of the fulfillment order.
    */
    pub async fn deprecated_202101_get_fulfillment_orders_param_order_locations_for_move(
        &self,
        fulfillment_order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/fulfillment_orders/{}/locations_for_move.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Retrieves a list of locations that a fulfillment order can potentially move to.
              The resulting list is sorted alphabetically in ascending order by location name.
    *
    * This function performs a `GET` to the `/admin/api/unstable/fulfillment_orders/{fulfillment_order_id}/locations_for_move.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shipping-and-fulfillment/locationsformove#index-unstable
    *
    * **Parameters:**
    *
    * * `fulfillment_order_id: &str` -- storefront_access_token_id.
    * * `fulfillment_order_id: &str` -- The ID of the fulfillment order.
    */
    pub async fn deprecated_unstable_get_fulfillment_orders_param_order_locations_for_move(
        &self,
        fulfillment_order_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/fulfillment_orders/{}/locations_for_move.json",
                crate::progenitor_support::encode_path(fulfillment_order_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}
