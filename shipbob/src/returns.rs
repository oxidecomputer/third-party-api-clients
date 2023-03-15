use crate::Client;
use crate::ClientResult;

pub struct Returns {
    pub client: Client,
}

impl Returns {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Returns { client }
    }

    /**
     * Get Return Order.
     *
     * This function performs a `GET` to the `/return/{id}` endpoint.
     *
     * **Parameters:**
     *
     * * `id: i64` -- Unique id of the channel.
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn get(&self, id: i64) -> ClientResult<crate::types::ReturnOrder> {
        let url = self.client.url(
            &format!(
                "/return/{}",
                crate::progenitor_support::encode_path(&id.to_string()),
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
     * Modify Return Order.
     *
     * This function performs a `PUT` to the `/return/{id}` endpoint.
     *
     * **Parameters:**
     *
     * * `channel_id: i64` -- Unique id of the channel.
     * * `id: i64` -- Unique id of the channel.
     */
    pub async fn put(
        &self,
        id: i64,
        body: &crate::types::ReturnsCreateReturn,
    ) -> ClientResult<crate::types::ReturnOrder> {
        let url = self.client.url(
            &format!(
                "/return/{}",
                crate::progenitor_support::encode_path(&id.to_string()),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json-patch+json".to_string()),
                },
            )
            .await
    }
    /**
     * Get Return Orders.
     *
     * This function performs a `GET` to the `/return` endpoint.
     *
     * **Parameters:**
     *
     * * `page: i64` -- Page of return orders to get.
     * * `limit: i64` -- Amount of return orders per page to request.
     * * `sort_order: crate::types::SortOrder` -- Order to sort results in.
     * * `start_date: chrono::DateTime<chrono::Utc>` -- Start date to filter orders inserted later than.
     * * `end_date: chrono::DateTime<chrono::Utc>` -- End date to filter orders inserted earlier than.
     * * `i_ds: &[String]` -- Comma separated list of return orders ids to filter by.
     * * `reference_ids: &[String]` -- Comma separated list of reference ids to filter by.
     * * `status: &[String]` -- Comma separated list of statuses to filter by.
     * * `fulfillment_center_ids: &[String]` -- Comma separated list of destination fulfillment center IDs to filter by.
     * * `tracking_numbers: &[String]` -- Comma separated list of tracking numbers to filter by.
     * * `original_shipment_ids: &[String]` -- Comma separated list of original shipment IDs to filter by.
     * * `inventory_ids: &[String]` -- Comma separated list of inventory IDs contained in return to filter by.
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn get_page(
        &self,
        page: i64,
        limit: i64,
        sort_order: crate::types::SortOrder,
        start_date: Option<chrono::DateTime<chrono::Utc>>,
        end_date: Option<chrono::DateTime<chrono::Utc>>,
        ids: &[String],
        reference_ids: &[String],
        status: &[String],
        fulfillment_center_ids: &[String],
        tracking_numbers: &[String],
        original_shipment_ids: &[String],
        inventory_ids: &[String],
    ) -> ClientResult<Vec<crate::types::ReturnOrder>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if let Some(date) = end_date {
            query_args.push(("EndDate".to_string(), date.to_rfc3339()));
        }
        if !fulfillment_center_ids.is_empty() {
            query_args.push((
                "FulfillmentCenterIds".to_string(),
                fulfillment_center_ids.join(" "),
            ));
        }
        if !ids.is_empty() {
            query_args.push(("IDs".to_string(), ids.join(" ")));
        }
        if !inventory_ids.is_empty() {
            query_args.push(("InventoryIds".to_string(), inventory_ids.join(" ")));
        }
        if limit > 0 {
            query_args.push(("Limit".to_string(), limit.to_string()));
        }
        if !original_shipment_ids.is_empty() {
            query_args.push((
                "OriginalShipmentIds".to_string(),
                original_shipment_ids.join(" "),
            ));
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
        if !status.is_empty() {
            query_args.push(("Status".to_string(), status.join(" ")));
        }
        if !tracking_numbers.is_empty() {
            query_args.push(("TrackingNumbers".to_string(), tracking_numbers.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/return?{}", query_), None);
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
     * Get Return Orders.
     *
     * This function performs a `GET` to the `/return` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     */
    pub async fn get_all(
        &self,
        sort_order: crate::types::SortOrder,
        start_date: Option<chrono::DateTime<chrono::Utc>>,
        end_date: Option<chrono::DateTime<chrono::Utc>>,
        ids: &[String],
        reference_ids: &[String],
        status: &[String],
        fulfillment_center_ids: &[String],
        tracking_numbers: &[String],
        original_shipment_ids: &[String],
        inventory_ids: &[String],
    ) -> ClientResult<Vec<crate::types::ReturnOrder>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if let Some(date) = end_date {
            query_args.push(("EndDate".to_string(), date.to_rfc3339()));
        }
        if !fulfillment_center_ids.is_empty() {
            query_args.push((
                "FulfillmentCenterIds".to_string(),
                fulfillment_center_ids.join(" "),
            ));
        }
        if !ids.is_empty() {
            query_args.push(("IDs".to_string(), ids.join(" ")));
        }
        if !inventory_ids.is_empty() {
            query_args.push(("InventoryIds".to_string(), inventory_ids.join(" ")));
        }
        if !original_shipment_ids.is_empty() {
            query_args.push((
                "OriginalShipmentIds".to_string(),
                original_shipment_ids.join(" "),
            ));
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
        if !status.is_empty() {
            query_args.push(("Status".to_string(), status.join(" ")));
        }
        if !tracking_numbers.is_empty() {
            query_args.push(("TrackingNumbers".to_string(), tracking_numbers.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/return?{}", query_), None);
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
     * Create Return Order.
     *
     * This function performs a `POST` to the `/return` endpoint.
     *
     * **Parameters:**
     *
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn post(
        &self,
        body: &crate::types::ReturnsCreateReturn,
    ) -> ClientResult<crate::types::ReturnOrder> {
        let url = self.client.url("/return", None);
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
     * Cancel Return Order.
     *
     * This function performs a `POST` to the `/return/{id}/cancel` endpoint.
     *
     * **Parameters:**
     *
     * * `id: i64` -- Unique id of the channel.
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn post_cancel(&self, id: i64) -> ClientResult<crate::types::ReturnOrder> {
        let url = self.client.url(
            &format!(
                "/return/{}/cancel",
                crate::progenitor_support::encode_path(&id.to_string()),
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
     * Get One Return's status history.
     *
     * This function performs a `GET` to the `/return/{id}/statushistory` endpoint.
     *
     * **Parameters:**
     *
     * * `id: i64` -- Unique id of the channel.
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn get_status_history(
        &self,
        id: i64,
    ) -> ClientResult<crate::types::ReturnOrderStatusHistory> {
        let url = self.client.url(
            &format!(
                "/return/{}/statushistory",
                crate::progenitor_support::encode_path(&id.to_string()),
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
