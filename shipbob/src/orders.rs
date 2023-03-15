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
     * Estimate Fulfillment Cost For Order.
     *
     * This function performs a `POST` to the `/order/estimate` endpoint.
     *
     * This endpoint will provide, where possible, an estimate of pricing and fulfillment center assignment of a potential standard (direct to consumer) order.
     * Keep in mind that there are ways for the merchant to change FC assignment or product configuration after order creation that could invalidate this estimate.
     * Estimates cannot be returned for items that are unknown, out of stock, or too large for fulfillment using standard box sizes.
     * Additional services such as high-pick fees, shipping insurance, auto-splitting or auto-adding items to orders, and signature required are not included in this estimate.
     *
     * **Parameters:**
     *
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn post_estimate(
        &self,
        body: &crate::types::OrdersEstimateFulfillmentRequestModel,
    ) -> ClientResult<crate::types::OrdersEstimate> {
        let url = self.client.url("/order/estimate", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json-patch+json".to_string()),
                },
            )
            .await
    }
    /**
     * Get Order.
     *
     * This function performs a `GET` to the `/order/{orderId}` endpoint.
     *
     * **Parameters:**
     *
     * * `order_id: i64` -- Unique id of the channel.
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn get(&self, order_id: i64) -> ClientResult<crate::types::Order> {
        let url = self.client.url(
            &format!(
                "/order/{}",
                crate::progenitor_support::encode_path(&order_id.to_string()),
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
     * Get Orders.
     *
     * This function performs a `GET` to the `/order` endpoint.
     *
     * All parameters are AND filters
     *
     * **Parameters:**
     *
     * * `page: i64` -- Unique id of the channel.
     * * `limit: i64` -- Amount of orders per page to request.
     * * `i_ds: &[String]` -- Shipment IDs to cancel.
     * * `reference_ids: &[String]` -- Array of permissions granted for the channel.
     * * `start_date: chrono::DateTime<chrono::Utc>` -- Start date to filter orders inserted later than.
     * * `end_date: chrono::DateTime<chrono::Utc>` -- End date to filter orders inserted earlier than.
     * * `sort_order: crate::types::SortOrder` -- Order to sort results in.
     * * `has_tracking: bool` -- Has any portion of this order been assigned a tracking number.
     * * `last_update_start_date: chrono::DateTime<chrono::Utc>` -- Start date to filter orders updated later than.
     * * `last_update_end_date: chrono::DateTime<chrono::Utc>` -- End date to filter orders updated later than.
     * * `is_tracking_uploaded: bool` -- Filter orders that their tracking information was fully uploaded.
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn get_page(
        &self,
        page: i64,
        limit: i64,
        ids: &[String],
        reference_ids: &[String],
        start_date: Option<chrono::DateTime<chrono::Utc>>,
        end_date: Option<chrono::DateTime<chrono::Utc>>,
        sort_order: crate::types::SortOrder,
        has_tracking: bool,
        last_update_start_date: Option<chrono::DateTime<chrono::Utc>>,
        last_update_end_date: Option<chrono::DateTime<chrono::Utc>>,
        is_tracking_uploaded: bool,
    ) -> ClientResult<Vec<crate::types::Order>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if let Some(date) = end_date {
            query_args.push(("EndDate".to_string(), date.to_rfc3339()));
        }
        if has_tracking {
            query_args.push(("HasTracking".to_string(), has_tracking.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("IDs".to_string(), ids.join(" ")));
        }
        if is_tracking_uploaded {
            query_args.push((
                "IsTrackingUploaded".to_string(),
                is_tracking_uploaded.to_string(),
            ));
        }
        if let Some(date) = last_update_end_date {
            query_args.push(("LastUpdateEndDate".to_string(), date.to_rfc3339()));
        }
        if let Some(date) = last_update_start_date {
            query_args.push(("LastUpdateStartDate".to_string(), date.to_rfc3339()));
        }
        if limit > 0 {
            query_args.push(("Limit".to_string(), limit.to_string()));
        }
        if page > 0 {
            query_args.push(("Page".to_string(), page.to_string()));
        }
        if !reference_ids.is_empty() {
            query_args.push(("ReferenceIds".to_string(), reference_ids.join(" ")));
        }
        if !sort_order.to_string().is_empty() {
            query_args.push(("SortOrder".to_string(), sort_order.to_string()));
        }
        if let Some(date) = start_date {
            query_args.push(("StartDate".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/order?{}", query_), None);
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
     * Get Orders.
     *
     * This function performs a `GET` to the `/order` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * All parameters are AND filters
     */
    pub async fn get_all(
        &self,
        ids: &[String],
        reference_ids: &[String],
        start_date: Option<chrono::DateTime<chrono::Utc>>,
        end_date: Option<chrono::DateTime<chrono::Utc>>,
        sort_order: crate::types::SortOrder,
        has_tracking: bool,
        last_update_start_date: Option<chrono::DateTime<chrono::Utc>>,
        last_update_end_date: Option<chrono::DateTime<chrono::Utc>>,
        is_tracking_uploaded: bool,
    ) -> ClientResult<Vec<crate::types::Order>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if let Some(date) = end_date {
            query_args.push(("EndDate".to_string(), date.to_rfc3339()));
        }
        if has_tracking {
            query_args.push(("HasTracking".to_string(), has_tracking.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("IDs".to_string(), ids.join(" ")));
        }
        if is_tracking_uploaded {
            query_args.push((
                "IsTrackingUploaded".to_string(),
                is_tracking_uploaded.to_string(),
            ));
        }
        if let Some(date) = last_update_end_date {
            query_args.push(("LastUpdateEndDate".to_string(), date.to_rfc3339()));
        }
        if let Some(date) = last_update_start_date {
            query_args.push(("LastUpdateStartDate".to_string(), date.to_rfc3339()));
        }
        if !reference_ids.is_empty() {
            query_args.push(("ReferenceIds".to_string(), reference_ids.join(" ")));
        }
        if !sort_order.to_string().is_empty() {
            query_args.push(("SortOrder".to_string(), sort_order.to_string()));
        }
        if let Some(date) = start_date {
            query_args.push(("StartDate".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/order?{}", query_), None);
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Create Order.
     *
     * This function performs a `POST` to the `/order` endpoint.
     *
     * **Parameters:**
     *
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn post(
        &self,
        body: &crate::types::OrdersCreateOrderModel,
    ) -> ClientResult<crate::types::Order> {
        let url = self.client.url("/order", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json-patch+json".to_string()),
                },
            )
            .await
    }
    /**
     * Cancel single Order by Order ID.
     *
     * This function performs a `POST` to the `/order/{orderId}/cancel` endpoint.
     *
     * **Parameters:**
     *
     * * `order_id: i64` -- Unique id of the channel.
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn post_cancel(
        &self,
        order_id: i64,
    ) -> ClientResult<crate::types::OrdersCanceledOrder> {
        let url = self.client.url(
            &format!(
                "/order/{}/cancel",
                crate::progenitor_support::encode_path(&order_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get Order Store Json.
     *
     * This function performs a `GET` to the `/order/{orderId}/storeOrderJson` endpoint.
     *
     * **Parameters:**
     *
     * * `order_id: i64` -- The order ID to Get the JSON Stored.
     */
    pub async fn get_store_json(&self, order_id: i64) -> ClientResult<String> {
        let url = self.client.url(
            &format!(
                "/order/{}/storeOrderJson",
                crate::progenitor_support::encode_path(&order_id.to_string()),
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
     * Save the Store Order Json.
     *
     * This function performs a `POST` to the `/order/{orderId}/storeOrderJson` endpoint.
     *
     * **Parameters:**
     *
     * * `order_id: i64` -- Unique id of the channel.
     */
    pub async fn post_store_json(
        &self,
        order_id: i64,
        body: &crate::types::OrdersAddStoreOrderJsonModel,
    ) -> ClientResult<String> {
        let url = self.client.url(
            &format!(
                "/order/{}/storeOrderJson",
                crate::progenitor_support::encode_path(&order_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json-patch+json".to_string()),
                },
            )
            .await
    }
    /**
     * Get one Shipment by Order Id and Shipment Id.
     *
     * This function performs a `GET` to the `/order/{orderId}/shipment/{shipmentId}` endpoint.
     *
     * **Parameters:**
     *
     * * `order_id: i64` -- The order id to get the shipment for.
     * * `shipment_id: i64` -- Unique id of the channel.
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn get_shipment(
        &self,
        order_id: i64,
        shipment_id: i64,
    ) -> ClientResult<crate::types::OrdersShipment> {
        let url = self.client.url(
            &format!(
                "/order/{}/shipment/{}",
                crate::progenitor_support::encode_path(&order_id.to_string()),
                crate::progenitor_support::encode_path(&shipment_id.to_string()),
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
     * Cancel one Shipment by Order Id and Shipment Id.
     *
     * This function performs a `POST` to the `/order/{orderId}/shipment/{shipmentId}/cancel` endpoint.
     *
     * **Parameters:**
     *
     * * `shipment_id: i64` -- Unique id of the channel.
     * * `channel_id: i64` -- Unique id of the channel.
     * * `order_id: &str` -- Name of the channel.
     */
    pub async fn post_shipment_cancel(
        &self,
        shipment_id: i64,
        order_id: &str,
    ) -> ClientResult<crate::types::OrdersShipment> {
        let url = self.client.url(
            &format!(
                "/order/{}/shipment/{}/cancel",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(&shipment_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get one Shipment's status timeline by Order Id and Shipment Id.
     *
     * This function performs a `GET` to the `/order/{orderId}/shipment/{shipmentId}/timeline` endpoint.
     *
     * **Parameters:**
     *
     * * `order_id: i64` -- The order id to get the shipment for.
     * * `shipment_id: i64` -- Unique id of the channel.
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn get_shipment_timeline(
        &self,
        order_id: i64,
        shipment_id: i64,
    ) -> ClientResult<Vec<crate::types::OrdersShipmentLog>> {
        let url = self.client.url(
            &format!(
                "/order/{}/shipment/{}/timeline",
                crate::progenitor_support::encode_path(&order_id.to_string()),
                crate::progenitor_support::encode_path(&shipment_id.to_string()),
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
     * Get one Shipment's status timeline by Order Id and Shipment Id.
     *
     * This function performs a `GET` to the `/order/{orderId}/shipment/{shipmentId}/timeline` endpoint.
     *
     * As opposed to `get_shipment_timeline`, this function returns all the pages of the request at once.
     */
    pub async fn get_all_shipment_timeline(
        &self,
        order_id: i64,
        shipment_id: i64,
    ) -> ClientResult<Vec<crate::types::OrdersShipmentLog>> {
        let url = self.client.url(
            &format!(
                "/order/{}/shipment/{}/timeline",
                crate::progenitor_support::encode_path(&order_id.to_string()),
                crate::progenitor_support::encode_path(&shipment_id.to_string()),
            ),
            None,
        );
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get all Shipments for Order.
     *
     * This function performs a `GET` to the `/order/{orderId}/shipment` endpoint.
     *
     * **Parameters:**
     *
     * * `order_id: i64` -- The order id to get shipments for.
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn get_shipments(
        &self,
        order_id: i64,
    ) -> ClientResult<Vec<crate::types::OrdersShipment>> {
        let url = self.client.url(
            &format!(
                "/order/{}/shipment",
                crate::progenitor_support::encode_path(&order_id.to_string()),
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
     * Get all Shipments for Order.
     *
     * This function performs a `GET` to the `/order/{orderId}/shipment` endpoint.
     *
     * As opposed to `get_shipments`, this function returns all the pages of the request at once.
     */
    pub async fn get_all_shipments(
        &self,
        order_id: i64,
    ) -> ClientResult<Vec<crate::types::OrdersShipment>> {
        let url = self.client.url(
            &format!(
                "/order/{}/shipment",
                crate::progenitor_support::encode_path(&order_id.to_string()),
            ),
            None,
        );
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get logs for one Shipment by Order Id and Shipment Id.
     *
     * This function performs a `GET` to the `/order/{orderId}/shipment/{shipmentId}/logs` endpoint.
     *
     * **Parameters:**
     *
     * * `order_id: i64` -- The order id to get the shipment for.
     * * `shipment_id: i64` -- Unique id of the channel.
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn get_shipment_logs(
        &self,
        order_id: i64,
        shipment_id: i64,
    ) -> ClientResult<Vec<crate::types::OrdersShipmentLog>> {
        let url = self.client.url(
            &format!(
                "/order/{}/shipment/{}/logs",
                crate::progenitor_support::encode_path(&order_id.to_string()),
                crate::progenitor_support::encode_path(&shipment_id.to_string()),
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
     * Get logs for one Shipment by Order Id and Shipment Id.
     *
     * This function performs a `GET` to the `/order/{orderId}/shipment/{shipmentId}/logs` endpoint.
     *
     * As opposed to `get_shipment_logs`, this function returns all the pages of the request at once.
     */
    pub async fn get_all_shipment_logs(
        &self,
        order_id: i64,
        shipment_id: i64,
    ) -> ClientResult<Vec<crate::types::OrdersShipmentLog>> {
        let url = self.client.url(
            &format!(
                "/order/{}/shipment/{}/logs",
                crate::progenitor_support::encode_path(&order_id.to_string()),
                crate::progenitor_support::encode_path(&shipment_id.to_string()),
            ),
            None,
        );
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get one Shipment by Shipment Id.
     *
     * This function performs a `GET` to the `/shipment/{shipmentId}` endpoint.
     *
     * **Parameters:**
     *
     * * `shipment_id: i64` -- Unique id of the channel.
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn get_shipment_orders(
        &self,
        shipment_id: i64,
    ) -> ClientResult<crate::types::OrdersShipment> {
        let url = self.client.url(
            &format!(
                "/shipment/{}",
                crate::progenitor_support::encode_path(&shipment_id.to_string()),
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
     * Cancel one Shipment by Shipment Id.
     *
     * This function performs a `POST` to the `/shipment/{shipmentId}/cancel` endpoint.
     *
     * **Parameters:**
     *
     * * `shipment_id: i64` -- Unique id of the channel.
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn post_shipment_cancel_orders(
        &self,
        shipment_id: i64,
    ) -> ClientResult<crate::types::OrdersShipment> {
        let url = self.client.url(
            &format!(
                "/shipment/{}/cancel",
                crate::progenitor_support::encode_path(&shipment_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Cancel multiple Shipments by Shipment Id.
     *
     * This function performs a `POST` to the `/shipment/cancelbulk` endpoint.
     *
     * **Parameters:**
     *
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn post_shipment_cancel_bulk(
        &self,
        body: &crate::types::OrdersCancelShipmentsModel,
    ) -> ClientResult<crate::types::OrdersCanceledShipments> {
        let url = self.client.url("/shipment/cancelbulk", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json-patch+json".to_string()),
                },
            )
            .await
    }
    /**
     * Get one Shipment's status timeline by Shipment Id.
     *
     * This function performs a `GET` to the `/shipment/{shipmentId}/timeline` endpoint.
     *
     * **Parameters:**
     *
     * * `shipment_id: i64` -- Unique id of the channel.
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn get_shipment_timeline_orders(
        &self,
        shipment_id: i64,
    ) -> ClientResult<Vec<crate::types::OrdersShipmentLog>> {
        let url = self.client.url(
            &format!(
                "/shipment/{}/timeline",
                crate::progenitor_support::encode_path(&shipment_id.to_string()),
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
     * Get one Shipment's status timeline by Shipment Id.
     *
     * This function performs a `GET` to the `/shipment/{shipmentId}/timeline` endpoint.
     *
     * As opposed to `get_shipment_timeline`, this function returns all the pages of the request at once.
     */
    pub async fn get_all_shipment_timeline_all(
        &self,
        shipment_id: i64,
    ) -> ClientResult<Vec<crate::types::OrdersShipmentLog>> {
        let url = self.client.url(
            &format!(
                "/shipment/{}/timeline",
                crate::progenitor_support::encode_path(&shipment_id.to_string()),
            ),
            None,
        );
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get logs for one Shipment by Shipment Id.
     *
     * This function performs a `GET` to the `/shipment/{shipmentId}/logs` endpoint.
     *
     * **Parameters:**
     *
     * * `shipment_id: i64` -- Unique id of the channel.
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn get_shipment_logs_orders(
        &self,
        shipment_id: i64,
    ) -> ClientResult<Vec<crate::types::OrdersShipmentLog>> {
        let url = self.client.url(
            &format!(
                "/shipment/{}/logs",
                crate::progenitor_support::encode_path(&shipment_id.to_string()),
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
     * Get logs for one Shipment by Shipment Id.
     *
     * This function performs a `GET` to the `/shipment/{shipmentId}/logs` endpoint.
     *
     * As opposed to `get_shipment_logs`, this function returns all the pages of the request at once.
     */
    pub async fn get_all_shipment_logs_all(
        &self,
        shipment_id: i64,
    ) -> ClientResult<Vec<crate::types::OrdersShipmentLog>> {
        let url = self.client.url(
            &format!(
                "/shipment/{}/logs",
                crate::progenitor_support::encode_path(&shipment_id.to_string()),
            ),
            None,
        );
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get shipping methods.
     *
     * This function performs a `GET` to the `/shippingmethod` endpoint.
     *
     * Get all merchants shipping methods
     *
     * **Parameters:**
     *
     * * `page: i64` -- Unique id of the channel.
     * * `limit: i64` -- Amount of records per page to request.
     */
    pub async fn get_shipping_method(
        &self,
        page: i64,
        limit: i64,
    ) -> ClientResult<Vec<crate::types::OrdersShipMethodDetail>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if limit > 0 {
            query_args.push(("Limit".to_string(), limit.to_string()));
        }
        if page > 0 {
            query_args.push(("Page".to_string(), page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/shippingmethod?{}", query_), None);
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
     * Get shipping methods.
     *
     * This function performs a `GET` to the `/shippingmethod` endpoint.
     *
     * As opposed to `get_shipping_method`, this function returns all the pages of the request at once.
     *
     * Get all merchants shipping methods
     */
    pub async fn get_all_shipping_method(
        &self,
    ) -> ClientResult<Vec<crate::types::OrdersShipMethodDetail>> {
        let url = self.client.url("/shippingmethod", None);
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}
