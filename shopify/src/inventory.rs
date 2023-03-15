use crate::Client;
use crate::ClientResult;

pub struct Inventory {
    pub client: Client,
}

impl Inventory {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Inventory { client }
    }

    /**
     * Retrieves a list of inventory items. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/inventory_items.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventoryitem#index-2020-01
     *
     * **Parameters:**
     *
     * * `ids_required: &str` -- Show only inventory items specified by a comma-separated list of IDs.
     *                     (maximum: 100).
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `ids: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202001_get_item(
        &self,
        ids_required: &str,
        limit: &str,
        ids: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if ids > 0 {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !ids_required.is_empty() {
            query_args.push((
                "ids
                  required"
                    .to_string(),
                ids_required.to_string(),
            ));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/inventory_items.json?{}", query_),
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
     * Retrieves a single inventory item by ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/inventory_items/{inventory_item_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventoryitem#show-2020-01
     *
     * **Parameters:**
     *
     * * `inventory_item_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_items_param_item(
        &self,
        inventory_item_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/inventory_items/{}/json",
                crate::progenitor_support::encode_path(inventory_item_id),
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
     * Updates an existing inventory item.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/inventory_items/{inventory_item_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventoryitem#update-2020-01
     *
     * **Parameters:**
     *
     * * `inventory_item_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_items_param_item(
        &self,
        inventory_item_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/inventory_items/{}/json",
                crate::progenitor_support::encode_path(inventory_item_id),
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
     * Retrieves a list of inventory items. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/inventory_items.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventoryitem#index-2020-04
     *
     * **Parameters:**
     *
     * * `ids_required: &str` -- Show only inventory items specified by a comma-separated list of IDs.
     *                     (maximum: 100).
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `ids: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202004_get_item(
        &self,
        ids_required: &str,
        limit: &str,
        ids: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if ids > 0 {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !ids_required.is_empty() {
            query_args.push((
                "ids
                  required"
                    .to_string(),
                ids_required.to_string(),
            ));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/inventory_items.json?{}", query_),
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
     * Retrieves a single inventory item by ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/inventory_items/{inventory_item_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventoryitem#show-2020-04
     *
     * **Parameters:**
     *
     * * `inventory_item_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_items_param_item(
        &self,
        inventory_item_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/inventory_items/{}/json",
                crate::progenitor_support::encode_path(inventory_item_id),
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
     * Updates an existing inventory item.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/inventory_items/{inventory_item_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventoryitem#update-2020-04
     *
     * **Parameters:**
     *
     * * `inventory_item_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_items_param_item(
        &self,
        inventory_item_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/inventory_items/{}/json",
                crate::progenitor_support::encode_path(inventory_item_id),
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
     * Retrieves a list of inventory items. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/inventory_items.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventoryitem#index-2020-07
     *
     * **Parameters:**
     *
     * * `ids_required: &str` -- Show only inventory items specified by a comma-separated list of IDs.
     *                     (maximum: 100).
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `ids: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202007_get_item(
        &self,
        ids_required: &str,
        limit: &str,
        ids: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if ids > 0 {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !ids_required.is_empty() {
            query_args.push((
                "ids
                  required"
                    .to_string(),
                ids_required.to_string(),
            ));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/inventory_items.json?{}", query_),
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
     * Retrieves a single inventory item by ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/inventory_items/{inventory_item_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventoryitem#show-2020-07
     *
     * **Parameters:**
     *
     * * `inventory_item_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_items_param_item(
        &self,
        inventory_item_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/inventory_items/{}/json",
                crate::progenitor_support::encode_path(inventory_item_id),
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
     * Updates an existing inventory item.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/inventory_items/{inventory_item_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventoryitem#update-2020-07
     *
     * **Parameters:**
     *
     * * `inventory_item_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_items_param_item(
        &self,
        inventory_item_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/inventory_items/{}/json",
                crate::progenitor_support::encode_path(inventory_item_id),
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
     * Retrieves a list of inventory items. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/inventory_items.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventoryitem#index-2020-10
     *
     * **Parameters:**
     *
     * * `ids_required: &str` -- Show only inventory items specified by a comma-separated list of IDs.
     *                     (maximum: 100).
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `ids: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn get_item(&self, ids_required: &str, limit: &str, ids: i64) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if ids > 0 {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !ids_required.is_empty() {
            query_args.push((
                "ids
                  required"
                    .to_string(),
                ids_required.to_string(),
            ));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/inventory_items.json?{}", query_),
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
     * Retrieves a single inventory item by ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/inventory_items/{inventory_item_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventoryitem#show-2020-10
     *
     * **Parameters:**
     *
     * * `inventory_item_id: &str` -- storefront_access_token_id.
     */
    pub async fn get_items_param_item(&self, inventory_item_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/inventory_items/{}/json",
                crate::progenitor_support::encode_path(inventory_item_id),
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
     * Updates an existing inventory item.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/inventory_items/{inventory_item_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventoryitem#update-2020-10
     *
     * **Parameters:**
     *
     * * `inventory_item_id: &str` -- storefront_access_token_id.
     */
    pub async fn update_items_param_item(
        &self,
        inventory_item_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/inventory_items/{}/json",
                crate::progenitor_support::encode_path(inventory_item_id),
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
     * Retrieves a list of inventory items. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/inventory_items.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventoryitem#index-2021-01
     *
     * **Parameters:**
     *
     * * `ids_required: &str` -- Show only inventory items specified by a comma-separated list of IDs.
     *                     (maximum: 100).
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `ids: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202101_get_item(
        &self,
        ids_required: &str,
        limit: &str,
        ids: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if ids > 0 {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !ids_required.is_empty() {
            query_args.push((
                "ids
                  required"
                    .to_string(),
                ids_required.to_string(),
            ));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/inventory_items.json?{}", query_),
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
     * Retrieves a single inventory item by ID.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/inventory_items/{inventory_item_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventoryitem#show-2021-01
     *
     * **Parameters:**
     *
     * * `inventory_item_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_items_param_item(
        &self,
        inventory_item_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/inventory_items/{}/json",
                crate::progenitor_support::encode_path(inventory_item_id),
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
     * Updates an existing inventory item.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/inventory_items/{inventory_item_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventoryitem#update-2021-01
     *
     * **Parameters:**
     *
     * * `inventory_item_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_items_param_item(
        &self,
        inventory_item_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/inventory_items/{}/json",
                crate::progenitor_support::encode_path(inventory_item_id),
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
     * Retrieves a list of inventory items. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/inventory_items.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventoryitem#index-unstable
     *
     * **Parameters:**
     *
     * * `ids_required: &str` -- Show only inventory items specified by a comma-separated list of IDs.
     *                     (maximum: 100).
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `ids: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_unstable_get_item(
        &self,
        ids_required: &str,
        limit: &str,
        ids: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if ids > 0 {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !ids_required.is_empty() {
            query_args.push((
                "ids
                  required"
                    .to_string(),
                ids_required.to_string(),
            ));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/inventory_items.json?{}", query_),
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
     * Retrieves a single inventory item by ID.
     *
     * This function performs a `GET` to the `/admin/api/unstable/inventory_items/{inventory_item_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventoryitem#show-unstable
     *
     * **Parameters:**
     *
     * * `inventory_item_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_items_param_item(
        &self,
        inventory_item_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/inventory_items/{}/json",
                crate::progenitor_support::encode_path(inventory_item_id),
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
     * Updates an existing inventory item.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/inventory_items/{inventory_item_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventoryitem#update-unstable
     *
     * **Parameters:**
     *
     * * `inventory_item_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_items_param_item(
        &self,
        inventory_item_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/inventory_items/{}/json",
                crate::progenitor_support::encode_path(inventory_item_id),
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
    * Retrieves a list of inventory levels.
              You must include inventory_item_ids, location_ids, or both as filter parameters.
              Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
    *
    * This function performs a `GET` to the `/admin/api/2020-01/inventory_levels.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#index-2020-01
    *
    * **Parameters:**
    *
    * * `inventory_item_ids: &str` -- A comma-separated list of inventory item IDs.
    *                     (maximum: 50).
    * * `location_ids: &str` -- A comma-separated list of location IDs. To find the ID of a location, use the Location resource.
    *                     (maximum: 50).
    * * `limit: &str` -- The maximum number of results to show.
    *                     (default: 50, maximum: 250).
    * * `updated_at_min: &str` -- Show inventory levels updated at or after date (format: 2019-03-19T01:21:44-04:00).
    */
    pub async fn deprecated_202001_get_level(
        &self,
        inventory_item_ids: &str,
        location_ids: &str,
        limit: &str,
        updated_at_min: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !inventory_item_ids.is_empty() {
            query_args.push((
                "inventory_item_ids".to_string(),
                inventory_item_ids.to_string(),
            ));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !location_ids.is_empty() {
            query_args.push(("location_ids".to_string(), location_ids.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/inventory_levels.json?{}", query_),
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
    * Deletes an inventory level of an inventory item at a location.
              Deleting an inventory level for an inventory item removes that item from the specified location.
              Every inventory item must have at least one inventory level. To move inventory to another location,
              first connect the inventory item to another location, and then delete the previous inventory level.
    *
    * This function performs a `DELETE` to the `/admin/api/2020-01/inventory_levels.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#destroy-2020-01
    *
    * **Parameters:**
    *
    * * `inventory_item_required: &str` -- The ID for the inventory item.
    * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
    * * `inventory_item_id: i64` -- recurring_application_charge[capped_amount].
    * * `location_id: i64` -- recurring_application_charge[capped_amount].
    */
    pub async fn deprecated_202001_delete_levels(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        inventory_item_id: i64,
        location_id: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if inventory_item_id > 0 {
            query_args.push((
                "inventory_item_id".to_string(),
                inventory_item_id.to_string(),
            ));
        }
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if location_id > 0 {
            query_args.push(("location_id".to_string(), location_id.to_string()));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/inventory_levels.json?{}", query_),
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
     * Adjusts the inventory level of an inventory item at a single location.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/inventory_levels/adjust.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#adjust-2020-01
     *
     * **Parameters:**
     *
     * * `inventory_item_required: &str` -- The ID of the inventory item.
     * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
     * * `available_adjustment_required: &str` -- The amount to adjust the available inventory quantity. Send negative values to subtract from the current available quantity. For example, "available_adjustment": 2 increases the current available quantity by 2, and "available_adjustment": -3decreases the current available quantity by 3.
     */
    pub async fn deprecated_202001_create_levels_adjust(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        available_adjustment_required: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !available_adjustment_required.is_empty() {
            query_args.push((
                "available_adjustment
                  required"
                    .to_string(),
                available_adjustment_required.to_string(),
            ));
        }
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/inventory_levels/adjust.json?{}", query_),
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
    * Connects an inventory item to a location by creating an inventory level at that location.
            When connecting inventory items to locations, it's important to understand the rules around
            fulfillment service locations.
    *
    * This function performs a `POST` to the `/admin/api/2020-01/inventory_levels/connect.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#connect-2020-01
    *
    * **Parameters:**
    *
    * * `inventory_item_required: &str` -- The ID of the inventory item.
    * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
    * * `relocate_if_necessary: &str` -- Whether inventory for any previously connected locations will be relocated. This property is ignored when no fulfillment service location is involved. For more information, see Inventory levels and fulfillment service locations.
    *                     (default: false).
    */
    pub async fn deprecated_202001_create_levels_connect(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        relocate_if_necessary: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        if !relocate_if_necessary.is_empty() {
            query_args.push((
                "relocate_if_necessary".to_string(),
                relocate_if_necessary.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/inventory_levels/connect.json?{}",
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
    * Sets the inventory level for an inventory item at a location.
              If the specified location is not connected, it will be automatically connected first.
              When connecting inventory items to locations, it's important to understand the rules around
              fulfillment service locations.
    *
    * This function performs a `POST` to the `/admin/api/2020-01/inventory_levels/set.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#set-2020-01
    *
    * **Parameters:**
    *
    * * `inventory_item_required: &str` -- The ID for the inventory item.
    * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
    * * `available_required: &str` -- Sets the available inventory quantity.
    * * `disconnect_if_necessary: &str` -- Whether inventory for any previously connected locations will be set to 0 and the locations disconnected. This property is ignored when no fulfillment service  is involved. For more information, see Inventory levels and fulfillment service locations.
    *                     (default: false).
    */
    pub async fn deprecated_202001_create_levels_set(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        available_required: &str,
        disconnect_if_necessary: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !available_required.is_empty() {
            query_args.push((
                "available
                  required"
                    .to_string(),
                available_required.to_string(),
            ));
        }
        if !disconnect_if_necessary.is_empty() {
            query_args.push((
                "disconnect_if_necessary".to_string(),
                disconnect_if_necessary.to_string(),
            ));
        }
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/inventory_levels/set.json?{}", query_),
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
    * Retrieves a list of inventory levels.
              You must include inventory_item_ids, location_ids, or both as filter parameters.
              Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
    *
    * This function performs a `GET` to the `/admin/api/2020-04/inventory_levels.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#index-2020-04
    *
    * **Parameters:**
    *
    * * `inventory_item_ids: &str` -- A comma-separated list of inventory item IDs.
    *                     (maximum: 50).
    * * `location_ids: &str` -- A comma-separated list of location IDs. To find the ID of a location, use the Location resource.
    *                     (maximum: 50).
    * * `limit: &str` -- The maximum number of results to show.
    *                     (default: 50, maximum: 250).
    * * `updated_at_min: &str` -- Show inventory levels updated at or after date (format: 2019-03-19T01:21:44-04:00).
    */
    pub async fn deprecated_202004_get_level(
        &self,
        inventory_item_ids: &str,
        location_ids: &str,
        limit: &str,
        updated_at_min: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !inventory_item_ids.is_empty() {
            query_args.push((
                "inventory_item_ids".to_string(),
                inventory_item_ids.to_string(),
            ));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !location_ids.is_empty() {
            query_args.push(("location_ids".to_string(), location_ids.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/inventory_levels.json?{}", query_),
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
    * Deletes an inventory level of an inventory item at a location.
              Deleting an inventory level for an inventory item removes that item from the specified location.
              Every inventory item must have at least one inventory level. To move inventory to another location,
              first connect the inventory item to another location, and then delete the previous inventory level.
    *
    * This function performs a `DELETE` to the `/admin/api/2020-04/inventory_levels.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#destroy-2020-04
    *
    * **Parameters:**
    *
    * * `inventory_item_required: &str` -- The ID for the inventory item.
    * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
    * * `inventory_item_id: i64` -- recurring_application_charge[capped_amount].
    * * `location_id: i64` -- recurring_application_charge[capped_amount].
    */
    pub async fn deprecated_202004_delete_levels(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        inventory_item_id: i64,
        location_id: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if inventory_item_id > 0 {
            query_args.push((
                "inventory_item_id".to_string(),
                inventory_item_id.to_string(),
            ));
        }
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if location_id > 0 {
            query_args.push(("location_id".to_string(), location_id.to_string()));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/inventory_levels.json?{}", query_),
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
     * Adjusts the inventory level of an inventory item at a single location.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/inventory_levels/adjust.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#adjust-2020-04
     *
     * **Parameters:**
     *
     * * `inventory_item_required: &str` -- The ID of the inventory item.
     * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
     * * `available_adjustment_required: &str` -- The amount to adjust the available inventory quantity. Send negative values to subtract from the current available quantity. For example, "available_adjustment": 2 increases the current available quantity by 2, and "available_adjustment": -3decreases the current available quantity by 3.
     */
    pub async fn deprecated_202004_create_levels_adjust(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        available_adjustment_required: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !available_adjustment_required.is_empty() {
            query_args.push((
                "available_adjustment
                  required"
                    .to_string(),
                available_adjustment_required.to_string(),
            ));
        }
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/inventory_levels/adjust.json?{}", query_),
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
    * Connects an inventory item to a location by creating an inventory level at that location.
            When connecting inventory items to locations, it's important to understand the rules around
            fulfillment service locations.
    *
    * This function performs a `POST` to the `/admin/api/2020-04/inventory_levels/connect.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#connect-2020-04
    *
    * **Parameters:**
    *
    * * `inventory_item_required: &str` -- The ID of the inventory item.
    * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
    * * `relocate_if_necessary: &str` -- Whether inventory for any previously connected locations will be relocated. This property is ignored when no fulfillment service location is involved. For more information, see Inventory levels and fulfillment service locations.
    *                     (default: false).
    */
    pub async fn deprecated_202004_create_levels_connect(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        relocate_if_necessary: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        if !relocate_if_necessary.is_empty() {
            query_args.push((
                "relocate_if_necessary".to_string(),
                relocate_if_necessary.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/inventory_levels/connect.json?{}",
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
    * Sets the inventory level for an inventory item at a location.
              If the specified location is not connected, it will be automatically connected first.
              When connecting inventory items to locations, it's important to understand the rules around
              fulfillment service locations.
    *
    * This function performs a `POST` to the `/admin/api/2020-04/inventory_levels/set.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#set-2020-04
    *
    * **Parameters:**
    *
    * * `inventory_item_required: &str` -- The ID for the inventory item.
    * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
    * * `available_required: &str` -- Sets the available inventory quantity.
    * * `disconnect_if_necessary: &str` -- Whether inventory for any previously connected locations will be set to 0 and the locations disconnected. This property is ignored when no fulfillment service  is involved. For more information, see Inventory levels and fulfillment service locations.
    *                     (default: false).
    */
    pub async fn deprecated_202004_create_levels_set(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        available_required: &str,
        disconnect_if_necessary: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !available_required.is_empty() {
            query_args.push((
                "available
                  required"
                    .to_string(),
                available_required.to_string(),
            ));
        }
        if !disconnect_if_necessary.is_empty() {
            query_args.push((
                "disconnect_if_necessary".to_string(),
                disconnect_if_necessary.to_string(),
            ));
        }
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/inventory_levels/set.json?{}", query_),
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
    * Retrieves a list of inventory levels.
              You must include inventory_item_ids, location_ids, or both as filter parameters.
              Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
    *
    * This function performs a `GET` to the `/admin/api/2020-07/inventory_levels.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#index-2020-07
    *
    * **Parameters:**
    *
    * * `inventory_item_ids: &str` -- A comma-separated list of inventory item IDs.
    *                     (maximum: 50).
    * * `location_ids: &str` -- A comma-separated list of location IDs. To find the ID of a location, use the Location resource.
    *                     (maximum: 50).
    * * `limit: &str` -- The maximum number of results to show.
    *                     (default: 50, maximum: 250).
    * * `updated_at_min: &str` -- Show inventory levels updated at or after date (format: 2019-03-19T01:21:44-04:00).
    */
    pub async fn deprecated_202007_get_level(
        &self,
        inventory_item_ids: &str,
        location_ids: &str,
        limit: &str,
        updated_at_min: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !inventory_item_ids.is_empty() {
            query_args.push((
                "inventory_item_ids".to_string(),
                inventory_item_ids.to_string(),
            ));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !location_ids.is_empty() {
            query_args.push(("location_ids".to_string(), location_ids.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/inventory_levels.json?{}", query_),
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
    * Deletes an inventory level of an inventory item at a location.
              Deleting an inventory level for an inventory item removes that item from the specified location.
              Every inventory item must have at least one inventory level. To move inventory to another location,
              first connect the inventory item to another location, and then delete the previous inventory level.
    *
    * This function performs a `DELETE` to the `/admin/api/2020-07/inventory_levels.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#destroy-2020-07
    *
    * **Parameters:**
    *
    * * `inventory_item_required: &str` -- The ID for the inventory item.
    * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
    * * `inventory_item_id: i64` -- recurring_application_charge[capped_amount].
    * * `location_id: i64` -- recurring_application_charge[capped_amount].
    */
    pub async fn deprecated_202007_delete_levels(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        inventory_item_id: i64,
        location_id: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if inventory_item_id > 0 {
            query_args.push((
                "inventory_item_id".to_string(),
                inventory_item_id.to_string(),
            ));
        }
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if location_id > 0 {
            query_args.push(("location_id".to_string(), location_id.to_string()));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/inventory_levels.json?{}", query_),
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
     * Adjusts the inventory level of an inventory item at a single location.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/inventory_levels/adjust.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#adjust-2020-07
     *
     * **Parameters:**
     *
     * * `inventory_item_required: &str` -- The ID of the inventory item.
     * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
     * * `available_adjustment_required: &str` -- The amount to adjust the available inventory quantity. Send negative values to subtract from the current available quantity. For example, "available_adjustment": 2 increases the current available quantity by 2, and "available_adjustment": -3decreases the current available quantity by 3.
     */
    pub async fn deprecated_202007_create_levels_adjust(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        available_adjustment_required: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !available_adjustment_required.is_empty() {
            query_args.push((
                "available_adjustment
                  required"
                    .to_string(),
                available_adjustment_required.to_string(),
            ));
        }
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/inventory_levels/adjust.json?{}", query_),
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
    * Connects an inventory item to a location by creating an inventory level at that location.
            When connecting inventory items to locations, it's important to understand the rules around
            fulfillment service locations.
    *
    * This function performs a `POST` to the `/admin/api/2020-07/inventory_levels/connect.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#connect-2020-07
    *
    * **Parameters:**
    *
    * * `inventory_item_required: &str` -- The ID of the inventory item.
    * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
    * * `relocate_if_necessary: &str` -- Whether inventory for any previously connected locations will be relocated. This property is ignored when no fulfillment service location is involved. For more information, see Inventory levels and fulfillment service locations.
    *                     (default: false).
    */
    pub async fn deprecated_202007_create_levels_connect(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        relocate_if_necessary: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        if !relocate_if_necessary.is_empty() {
            query_args.push((
                "relocate_if_necessary".to_string(),
                relocate_if_necessary.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/inventory_levels/connect.json?{}",
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
    * Sets the inventory level for an inventory item at a location.
              If the specified location is not connected, it will be automatically connected first.
              When connecting inventory items to locations, it's important to understand the rules around
              fulfillment service locations.
    *
    * This function performs a `POST` to the `/admin/api/2020-07/inventory_levels/set.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#set-2020-07
    *
    * **Parameters:**
    *
    * * `inventory_item_required: &str` -- The ID for the inventory item.
    * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
    * * `available_required: &str` -- Sets the available inventory quantity.
    * * `disconnect_if_necessary: &str` -- Whether inventory for any previously connected locations will be set to 0 and the locations disconnected. This property is ignored when no fulfillment service  is involved. For more information, see Inventory levels and fulfillment service locations.
    *                     (default: false).
    */
    pub async fn deprecated_202007_create_levels_set(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        available_required: &str,
        disconnect_if_necessary: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !available_required.is_empty() {
            query_args.push((
                "available
                  required"
                    .to_string(),
                available_required.to_string(),
            ));
        }
        if !disconnect_if_necessary.is_empty() {
            query_args.push((
                "disconnect_if_necessary".to_string(),
                disconnect_if_necessary.to_string(),
            ));
        }
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/inventory_levels/set.json?{}", query_),
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
    * Retrieves a list of inventory levels.
              You must include inventory_item_ids, location_ids, or both as filter parameters.
              Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
    *
    * This function performs a `GET` to the `/admin/api/2020-10/inventory_levels.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#index-2020-10
    *
    * **Parameters:**
    *
    * * `inventory_item_ids: &str` -- A comma-separated list of inventory item IDs.
    *                     (maximum: 50).
    * * `location_ids: &str` -- A comma-separated list of location IDs. To find the ID of a location, use the Location resource.
    *                     (maximum: 50).
    * * `limit: &str` -- The maximum number of results to show.
    *                     (default: 50, maximum: 250).
    * * `updated_at_min: &str` -- Show inventory levels updated at or after date (format: 2019-03-19T01:21:44-04:00).
    */
    pub async fn get_level(
        &self,
        inventory_item_ids: &str,
        location_ids: &str,
        limit: &str,
        updated_at_min: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !inventory_item_ids.is_empty() {
            query_args.push((
                "inventory_item_ids".to_string(),
                inventory_item_ids.to_string(),
            ));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !location_ids.is_empty() {
            query_args.push(("location_ids".to_string(), location_ids.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/inventory_levels.json?{}", query_),
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
    * Deletes an inventory level of an inventory item at a location.
              Deleting an inventory level for an inventory item removes that item from the specified location.
              Every inventory item must have at least one inventory level. To move inventory to another location,
              first connect the inventory item to another location, and then delete the previous inventory level.
    *
    * This function performs a `DELETE` to the `/admin/api/2020-10/inventory_levels.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#destroy-2020-10
    *
    * **Parameters:**
    *
    * * `inventory_item_required: &str` -- The ID for the inventory item.
    * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
    * * `inventory_item_id: i64` -- recurring_application_charge[capped_amount].
    * * `location_id: i64` -- recurring_application_charge[capped_amount].
    */
    pub async fn delete_levels(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        inventory_item_id: i64,
        location_id: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if inventory_item_id > 0 {
            query_args.push((
                "inventory_item_id".to_string(),
                inventory_item_id.to_string(),
            ));
        }
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if location_id > 0 {
            query_args.push(("location_id".to_string(), location_id.to_string()));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/inventory_levels.json?{}", query_),
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
     * Adjusts the inventory level of an inventory item at a single location.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/inventory_levels/adjust.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#adjust-2020-10
     *
     * **Parameters:**
     *
     * * `inventory_item_required: &str` -- The ID of the inventory item.
     * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
     * * `available_adjustment_required: &str` -- The amount to adjust the available inventory quantity. Send negative values to subtract from the current available quantity. For example, "available_adjustment": 2 increases the current available quantity by 2, and "available_adjustment": -3decreases the current available quantity by 3.
     */
    pub async fn create_levels_adjust(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        available_adjustment_required: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !available_adjustment_required.is_empty() {
            query_args.push((
                "available_adjustment
                  required"
                    .to_string(),
                available_adjustment_required.to_string(),
            ));
        }
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/inventory_levels/adjust.json?{}", query_),
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
    * Connects an inventory item to a location by creating an inventory level at that location.
            When connecting inventory items to locations, it's important to understand the rules around
            fulfillment service locations.
    *
    * This function performs a `POST` to the `/admin/api/2020-10/inventory_levels/connect.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#connect-2020-10
    *
    * **Parameters:**
    *
    * * `inventory_item_required: &str` -- The ID of the inventory item.
    * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
    * * `relocate_if_necessary: &str` -- Whether inventory for any previously connected locations will be relocated. This property is ignored when no fulfillment service location is involved. For more information, see Inventory levels and fulfillment service locations.
    *                     (default: false).
    */
    pub async fn create_levels_connect(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        relocate_if_necessary: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        if !relocate_if_necessary.is_empty() {
            query_args.push((
                "relocate_if_necessary".to_string(),
                relocate_if_necessary.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/inventory_levels/connect.json?{}",
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
    * Sets the inventory level for an inventory item at a location.
              If the specified location is not connected, it will be automatically connected first.
              When connecting inventory items to locations, it's important to understand the rules around
              fulfillment service locations.
    *
    * This function performs a `POST` to the `/admin/api/2020-10/inventory_levels/set.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#set-2020-10
    *
    * **Parameters:**
    *
    * * `inventory_item_required: &str` -- The ID for the inventory item.
    * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
    * * `available_required: &str` -- Sets the available inventory quantity.
    * * `disconnect_if_necessary: &str` -- Whether inventory for any previously connected locations will be set to 0 and the locations disconnected. This property is ignored when no fulfillment service  is involved. For more information, see Inventory levels and fulfillment service locations.
    *                     (default: false).
    */
    pub async fn create_levels_set(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        available_required: &str,
        disconnect_if_necessary: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !available_required.is_empty() {
            query_args.push((
                "available
                  required"
                    .to_string(),
                available_required.to_string(),
            ));
        }
        if !disconnect_if_necessary.is_empty() {
            query_args.push((
                "disconnect_if_necessary".to_string(),
                disconnect_if_necessary.to_string(),
            ));
        }
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/inventory_levels/set.json?{}", query_),
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
    * Retrieves a list of inventory levels.
              You must include inventory_item_ids, location_ids, or both as filter parameters.
              Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
    *
    * This function performs a `GET` to the `/admin/api/2021-01/inventory_levels.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#index-2021-01
    *
    * **Parameters:**
    *
    * * `inventory_item_ids: &str` -- A comma-separated list of inventory item IDs.
    *                     (maximum: 50).
    * * `location_ids: &str` -- A comma-separated list of location IDs. To find the ID of a location, use the Location resource.
    *                     (maximum: 50).
    * * `limit: &str` -- The maximum number of results to show.
    *                     (default: 50, maximum: 250).
    * * `updated_at_min: &str` -- Show inventory levels updated at or after date (format: 2019-03-19T01:21:44-04:00).
    */
    pub async fn deprecated_202101_get_level(
        &self,
        inventory_item_ids: &str,
        location_ids: &str,
        limit: &str,
        updated_at_min: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !inventory_item_ids.is_empty() {
            query_args.push((
                "inventory_item_ids".to_string(),
                inventory_item_ids.to_string(),
            ));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !location_ids.is_empty() {
            query_args.push(("location_ids".to_string(), location_ids.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/inventory_levels.json?{}", query_),
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
    * Deletes an inventory level of an inventory item at a location.
              Deleting an inventory level for an inventory item removes that item from the specified location.
              Every inventory item must have at least one inventory level. To move inventory to another location,
              first connect the inventory item to another location, and then delete the previous inventory level.
    *
    * This function performs a `DELETE` to the `/admin/api/2021-01/inventory_levels.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#destroy-2021-01
    *
    * **Parameters:**
    *
    * * `inventory_item_required: &str` -- The ID for the inventory item.
    * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
    * * `inventory_item_id: i64` -- recurring_application_charge[capped_amount].
    * * `location_id: i64` -- recurring_application_charge[capped_amount].
    */
    pub async fn deprecated_202101_delete_levels(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        inventory_item_id: i64,
        location_id: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if inventory_item_id > 0 {
            query_args.push((
                "inventory_item_id".to_string(),
                inventory_item_id.to_string(),
            ));
        }
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if location_id > 0 {
            query_args.push(("location_id".to_string(), location_id.to_string()));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/inventory_levels.json?{}", query_),
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
     * Adjusts the inventory level of an inventory item at a single location.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/inventory_levels/adjust.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#adjust-2021-01
     *
     * **Parameters:**
     *
     * * `inventory_item_required: &str` -- The ID of the inventory item.
     * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
     * * `available_adjustment_required: &str` -- The amount to adjust the available inventory quantity. Send negative values to subtract from the current available quantity. For example, "available_adjustment": 2 increases the current available quantity by 2, and "available_adjustment": -3decreases the current available quantity by 3.
     */
    pub async fn deprecated_202101_create_levels_adjust(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        available_adjustment_required: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !available_adjustment_required.is_empty() {
            query_args.push((
                "available_adjustment
                  required"
                    .to_string(),
                available_adjustment_required.to_string(),
            ));
        }
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/inventory_levels/adjust.json?{}", query_),
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
    * Connects an inventory item to a location by creating an inventory level at that location.
            When connecting inventory items to locations, it's important to understand the rules around
            fulfillment service locations.
    *
    * This function performs a `POST` to the `/admin/api/2021-01/inventory_levels/connect.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#connect-2021-01
    *
    * **Parameters:**
    *
    * * `inventory_item_required: &str` -- The ID of the inventory item.
    * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
    * * `relocate_if_necessary: &str` -- Whether inventory for any previously connected locations will be relocated. This property is ignored when no fulfillment service location is involved. For more information, see Inventory levels and fulfillment service locations.
    *                     (default: false).
    */
    pub async fn deprecated_202101_create_levels_connect(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        relocate_if_necessary: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        if !relocate_if_necessary.is_empty() {
            query_args.push((
                "relocate_if_necessary".to_string(),
                relocate_if_necessary.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/inventory_levels/connect.json?{}",
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
    * Sets the inventory level for an inventory item at a location.
              If the specified location is not connected, it will be automatically connected first.
              When connecting inventory items to locations, it's important to understand the rules around
              fulfillment service locations.
    *
    * This function performs a `POST` to the `/admin/api/2021-01/inventory_levels/set.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#set-2021-01
    *
    * **Parameters:**
    *
    * * `inventory_item_required: &str` -- The ID for the inventory item.
    * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
    * * `available_required: &str` -- Sets the available inventory quantity.
    * * `disconnect_if_necessary: &str` -- Whether inventory for any previously connected locations will be set to 0 and the locations disconnected. This property is ignored when no fulfillment service  is involved. For more information, see Inventory levels and fulfillment service locations.
    *                     (default: false).
    */
    pub async fn deprecated_202101_create_levels_set(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        available_required: &str,
        disconnect_if_necessary: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !available_required.is_empty() {
            query_args.push((
                "available
                  required"
                    .to_string(),
                available_required.to_string(),
            ));
        }
        if !disconnect_if_necessary.is_empty() {
            query_args.push((
                "disconnect_if_necessary".to_string(),
                disconnect_if_necessary.to_string(),
            ));
        }
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/inventory_levels/set.json?{}", query_),
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
    * Retrieves a list of inventory levels.
              You must include inventory_item_ids, location_ids, or both as filter parameters.
              Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
    *
    * This function performs a `GET` to the `/admin/api/unstable/inventory_levels.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#index-unstable
    *
    * **Parameters:**
    *
    * * `inventory_item_ids: &str` -- A comma-separated list of inventory item IDs.
    *                     (maximum: 50).
    * * `location_ids: &str` -- A comma-separated list of location IDs. To find the ID of a location, use the Location resource.
    *                     (maximum: 50).
    * * `limit: &str` -- The maximum number of results to show.
    *                     (default: 50, maximum: 250).
    * * `updated_at_min: &str` -- Show inventory levels updated at or after date (format: 2019-03-19T01:21:44-04:00).
    */
    pub async fn deprecated_unstable_get_level(
        &self,
        inventory_item_ids: &str,
        location_ids: &str,
        limit: &str,
        updated_at_min: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !inventory_item_ids.is_empty() {
            query_args.push((
                "inventory_item_ids".to_string(),
                inventory_item_ids.to_string(),
            ));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !location_ids.is_empty() {
            query_args.push(("location_ids".to_string(), location_ids.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/inventory_levels.json?{}", query_),
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
    * Deletes an inventory level of an inventory item at a location.
              Deleting an inventory level for an inventory item removes that item from the specified location.
              Every inventory item must have at least one inventory level. To move inventory to another location,
              first connect the inventory item to another location, and then delete the previous inventory level.
    *
    * This function performs a `DELETE` to the `/admin/api/unstable/inventory_levels.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#destroy-unstable
    *
    * **Parameters:**
    *
    * * `inventory_item_required: &str` -- The ID for the inventory item.
    * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
    * * `inventory_item_id: i64` -- recurring_application_charge[capped_amount].
    * * `location_id: i64` -- recurring_application_charge[capped_amount].
    */
    pub async fn deprecated_unstable_delete_levels(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        inventory_item_id: i64,
        location_id: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if inventory_item_id > 0 {
            query_args.push((
                "inventory_item_id".to_string(),
                inventory_item_id.to_string(),
            ));
        }
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if location_id > 0 {
            query_args.push(("location_id".to_string(), location_id.to_string()));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/inventory_levels.json?{}", query_),
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
     * Adjusts the inventory level of an inventory item at a single location.
     *
     * This function performs a `POST` to the `/admin/api/unstable/inventory_levels/adjust.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#adjust-unstable
     *
     * **Parameters:**
     *
     * * `inventory_item_required: &str` -- The ID of the inventory item.
     * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
     * * `available_adjustment_required: &str` -- The amount to adjust the available inventory quantity. Send negative values to subtract from the current available quantity. For example, "available_adjustment": 2 increases the current available quantity by 2, and "available_adjustment": -3decreases the current available quantity by 3.
     */
    pub async fn deprecated_unstable_create_levels_adjust(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        available_adjustment_required: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !available_adjustment_required.is_empty() {
            query_args.push((
                "available_adjustment
                  required"
                    .to_string(),
                available_adjustment_required.to_string(),
            ));
        }
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/inventory_levels/adjust.json?{}",
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
    * Connects an inventory item to a location by creating an inventory level at that location.
            When connecting inventory items to locations, it's important to understand the rules around
            fulfillment service locations.
    *
    * This function performs a `POST` to the `/admin/api/unstable/inventory_levels/connect.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#connect-unstable
    *
    * **Parameters:**
    *
    * * `inventory_item_required: &str` -- The ID of the inventory item.
    * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
    * * `relocate_if_necessary: &str` -- Whether inventory for any previously connected locations will be relocated. This property is ignored when no fulfillment service location is involved. For more information, see Inventory levels and fulfillment service locations.
    *                     (default: false).
    */
    pub async fn deprecated_unstable_create_levels_connect(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        relocate_if_necessary: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        if !relocate_if_necessary.is_empty() {
            query_args.push((
                "relocate_if_necessary".to_string(),
                relocate_if_necessary.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/inventory_levels/connect.json?{}",
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
    * Sets the inventory level for an inventory item at a location.
              If the specified location is not connected, it will be automatically connected first.
              When connecting inventory items to locations, it's important to understand the rules around
              fulfillment service locations.
    *
    * This function performs a `POST` to the `/admin/api/unstable/inventory_levels/set.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/inventory/inventorylevel#set-unstable
    *
    * **Parameters:**
    *
    * * `inventory_item_required: &str` -- The ID for the inventory item.
    * * `location_required: &str` -- The ID of the location that the inventory level belongs to. To find the ID of the location, use the Location resource.
    * * `available_required: &str` -- Sets the available inventory quantity.
    * * `disconnect_if_necessary: &str` -- Whether inventory for any previously connected locations will be set to 0 and the locations disconnected. This property is ignored when no fulfillment service  is involved. For more information, see Inventory levels and fulfillment service locations.
    *                     (default: false).
    */
    pub async fn deprecated_unstable_create_levels_set(
        &self,
        inventory_item_id_required: &str,
        location_id_required: &str,
        available_required: &str,
        disconnect_if_necessary: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !available_required.is_empty() {
            query_args.push((
                "available
                  required"
                    .to_string(),
                available_required.to_string(),
            ));
        }
        if !disconnect_if_necessary.is_empty() {
            query_args.push((
                "disconnect_if_necessary".to_string(),
                disconnect_if_necessary.to_string(),
            ));
        }
        if !inventory_item_id_required.is_empty() {
            query_args.push((
                "inventory_item_id
                  required"
                    .to_string(),
                inventory_item_id_required.to_string(),
            ));
        }
        if !location_id_required.is_empty() {
            query_args.push((
                "location_id
                  required"
                    .to_string(),
                location_id_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/inventory_levels/set.json?{}", query_),
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
     * Retrieves a list of locations.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/locations.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#index-2020-01
     */
    pub async fn deprecated_202001_get_location(&self) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-01/locations.json", None);
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
     * Retrieves a single location by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/locations/{location_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#show-2020-01
     *
     * **Parameters:**
     *
     * * `location_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_locations_param_location(
        &self,
        location_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/locations/{}/json",
                crate::progenitor_support::encode_path(location_id),
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
     * Retrieves a count of locations.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/locations/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#count-2020-01
     */
    pub async fn deprecated_202001_get_locations_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-01/locations/count.json", None);
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
     * Retrieves a list of inventory levels for a location. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/locations/{location_id}/inventory_levels.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#inventory_levels-2020-01
     *
     * **Parameters:**
     *
     * * `location_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_locations_param_location_level(
        &self,
        location_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/locations/{}/inventory_levels.json",
                crate::progenitor_support::encode_path(location_id),
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
     * Retrieves a list of locations.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/locations.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#index-2020-04
     */
    pub async fn deprecated_202004_get_location(&self) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-04/locations.json", None);
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
     * Retrieves a single location by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/locations/{location_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#show-2020-04
     *
     * **Parameters:**
     *
     * * `location_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_locations_param_location(
        &self,
        location_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/locations/{}/json",
                crate::progenitor_support::encode_path(location_id),
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
     * Retrieves a count of locations.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/locations/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#count-2020-04
     */
    pub async fn deprecated_202004_get_locations_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-04/locations/count.json", None);
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
     * Retrieves a list of inventory levels for a location. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/locations/{location_id}/inventory_levels.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#inventory_levels-2020-04
     *
     * **Parameters:**
     *
     * * `location_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_locations_param_location_level(
        &self,
        location_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/locations/{}/inventory_levels.json",
                crate::progenitor_support::encode_path(location_id),
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
     * Retrieves a list of locations.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/locations.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#index-2020-07
     */
    pub async fn deprecated_202007_get_location(&self) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-07/locations.json", None);
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
     * Retrieves a single location by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/locations/{location_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#show-2020-07
     *
     * **Parameters:**
     *
     * * `location_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_locations_param_location(
        &self,
        location_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/locations/{}/json",
                crate::progenitor_support::encode_path(location_id),
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
     * Retrieves a count of locations.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/locations/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#count-2020-07
     */
    pub async fn deprecated_202007_get_locations_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-07/locations/count.json", None);
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
     * Retrieves a list of inventory levels for a location. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/locations/{location_id}/inventory_levels.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#inventory_levels-2020-07
     *
     * **Parameters:**
     *
     * * `location_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_locations_param_location_level(
        &self,
        location_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/locations/{}/inventory_levels.json",
                crate::progenitor_support::encode_path(location_id),
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
     * Retrieves a list of locations.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/locations.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#index-2020-10
     */
    pub async fn get_location(&self) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-10/locations.json", None);
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
     * Retrieves a single location by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/locations/{location_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#show-2020-10
     *
     * **Parameters:**
     *
     * * `location_id: &str` -- storefront_access_token_id.
     */
    pub async fn get_locations_param_location(&self, location_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/locations/{}/json",
                crate::progenitor_support::encode_path(location_id),
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
     * Retrieves a count of locations.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/locations/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#count-2020-10
     */
    pub async fn get_locations_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-10/locations/count.json", None);
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
     * Retrieves a list of inventory levels for a location. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/locations/{location_id}/inventory_levels.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#inventory_levels-2020-10
     *
     * **Parameters:**
     *
     * * `location_id: &str` -- storefront_access_token_id.
     */
    pub async fn get_locations_param_location_level(&self, location_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/locations/{}/inventory_levels.json",
                crate::progenitor_support::encode_path(location_id),
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
     * Retrieves a list of locations.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/locations.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#index-2021-01
     */
    pub async fn deprecated_202101_get_location(&self) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2021-01/locations.json", None);
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
     * Retrieves a single location by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/locations/{location_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#show-2021-01
     *
     * **Parameters:**
     *
     * * `location_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_locations_param_location(
        &self,
        location_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/locations/{}/json",
                crate::progenitor_support::encode_path(location_id),
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
     * Retrieves a count of locations.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/locations/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#count-2021-01
     */
    pub async fn deprecated_202101_get_locations_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2021-01/locations/count.json", None);
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
     * Retrieves a list of inventory levels for a location. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/locations/{location_id}/inventory_levels.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#inventory_levels-2021-01
     *
     * **Parameters:**
     *
     * * `location_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_locations_param_location_level(
        &self,
        location_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/locations/{}/inventory_levels.json",
                crate::progenitor_support::encode_path(location_id),
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
     * Retrieves a list of locations.
     *
     * This function performs a `GET` to the `/admin/api/unstable/locations.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#index-unstable
     */
    pub async fn deprecated_unstable_get_location(&self) -> ClientResult<()> {
        let url = self.client.url("/admin/api/unstable/locations.json", None);
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
     * Retrieves a single location by its ID.
     *
     * This function performs a `GET` to the `/admin/api/unstable/locations/{location_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#show-unstable
     *
     * **Parameters:**
     *
     * * `location_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_locations_param_location(
        &self,
        location_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/locations/{}/json",
                crate::progenitor_support::encode_path(location_id),
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
     * Retrieves a count of locations.
     *
     * This function performs a `GET` to the `/admin/api/unstable/locations/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#count-unstable
     */
    pub async fn deprecated_unstable_get_locations_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/unstable/locations/count.json", None);
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
     * Retrieves a list of inventory levels for a location. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/locations/{location_id}/inventory_levels.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/inventory/location#inventory_levels-unstable
     *
     * **Parameters:**
     *
     * * `location_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_locations_param_location_level(
        &self,
        location_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/locations/{}/inventory_levels.json",
                crate::progenitor_support::encode_path(location_id),
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
