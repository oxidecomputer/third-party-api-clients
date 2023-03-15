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
     * Retrieves a count of checkouts from the past 90 days.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/checkouts/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/abandoned-checkouts#count-2020-01
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `created_at_min: &str` -- Count checkouts created after the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count checkouts created before the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Count checkouts last updated after the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count checkouts last updated before the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `status: &str` -- Count checkouts with a given status.
     *                     (default: open)
     *                       
     *                           open: Count only open abandoned checkouts.
     *                           closed: Count only closed abandoned checkouts.
     */
    pub async fn deprecated_202001_get_checkouts_count(
        &self,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/checkouts/count.json?{}", query_),
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
     * Retrieves a count of checkouts from the past 90 days.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/checkouts/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/abandoned-checkouts#count-2020-04
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `created_at_min: &str` -- Count checkouts created after the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count checkouts created before the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Count checkouts last updated after the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count checkouts last updated before the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `status: &str` -- Count checkouts with a given status.
     *                     (default: open)
     *                       
     *                           open: Count only open abandoned checkouts.
     *                           closed: Count only closed abandoned checkouts.
     */
    pub async fn deprecated_202004_get_checkouts_count(
        &self,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/checkouts/count.json?{}", query_),
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
     * Retrieves a count of checkouts from the past 90 days.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/checkouts/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/abandoned-checkouts#count-2020-07
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `created_at_min: &str` -- Count checkouts created after the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count checkouts created before the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Count checkouts last updated after the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count checkouts last updated before the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `status: &str` -- Count checkouts with a given status.
     *                     (default: open)
     *                       
     *                           open: Count only open abandoned checkouts.
     *                           closed: Count only closed abandoned checkouts.
     */
    pub async fn deprecated_202007_get_checkouts_count(
        &self,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/checkouts/count.json?{}", query_),
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
     * Retrieves a count of checkouts from the past 90 days.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/checkouts/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/abandoned-checkouts#count-2020-10
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `created_at_min: &str` -- Count checkouts created after the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count checkouts created before the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Count checkouts last updated after the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count checkouts last updated before the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `status: &str` -- Count checkouts with a given status.
     *                     (default: open)
     *                       
     *                           open: Count only open abandoned checkouts.
     *                           closed: Count only closed abandoned checkouts.
     */
    pub async fn get_checkouts_count(
        &self,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/checkouts/count.json?{}", query_),
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
     * Retrieves a count of checkouts from the past 90 days.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/checkouts/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/abandoned-checkouts#count-2021-01
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `created_at_min: &str` -- Count checkouts created after the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count checkouts created before the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Count checkouts last updated after the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count checkouts last updated before the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `status: &str` -- Count checkouts with a given status.
     *                     (default: open)
     *                       
     *                           open: Count only open abandoned checkouts.
     *                           closed: Count only closed abandoned checkouts.
     */
    pub async fn deprecated_202101_get_checkouts_count(
        &self,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/checkouts/count.json?{}", query_),
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
     * Retrieves a count of checkouts from the past 90 days.
     *
     * This function performs a `GET` to the `/admin/api/unstable/checkouts/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/abandoned-checkouts#count-unstable
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `created_at_min: &str` -- Count checkouts created after the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count checkouts created before the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Count checkouts last updated after the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count checkouts last updated before the specified date. (format: 2014-04-25T16:15:47-04:00).
     * * `status: &str` -- Count checkouts with a given status.
     *                     (default: open)
     *                       
     *                           open: Count only open abandoned checkouts.
     *                           closed: Count only closed abandoned checkouts.
     */
    pub async fn deprecated_unstable_get_checkouts_count(
        &self,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/checkouts/count.json?{}", query_),
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
     * Retrieves a list of orders. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/orders.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order#index-2020-01
     *
     * **Parameters:**
     *
     * * `ids: &str` -- Retrieve only orders specified by a comma-separated list of order IDs.
     * * `limit: &str` -- The maximum number of results to show on a page.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Show orders after the specified ID.
     * * `created_at_min: &str` -- Show orders created at or after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show orders created at or before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show orders last updated at or after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show orders last updated at or before date (format: 2014-04-25T16:15:47-04:00).
     * * `processed_at_min: &str` -- Show orders imported at or after date (format: 2014-04-25T16:15:47-04:00).
     * * `processed_at_max: &str` -- Show orders imported at or before date (format: 2014-04-25T16:15:47-04:00).
     * * `attribution_app_id: &str` -- Show orders attributed to a certain app, specified by the app ID. Set as current to show orders for the app currently consuming the API.
     * * `status: &str` -- Filter orders by their status.
     *                     (default: open)
     *                       
     *                           open: Show only open orders.
     *                           closed: Show only closed orders.
     *                           cancelled: Show only canceled orders.
     *                           any: Show orders of any status, including archived orders.
     * * `financial_status: &str` -- Filter orders by their financial status.
     *                     (default: any)
     *                       
     *                           authorized: Show only authorized orders
     *                           pending: Show only pending orders
     *                           paid: Show only paid orders
     *                           partially_paid: Show only partially paid orders
     *                           refunded: Show only refunded orders
     *                           voided: Show only voided orders
     *                           partially_refunded: Show only partially refunded orders
     *                           any: Show orders of any financial status.
     *                           unpaid: Show authorized and partially paid orders.
     * * `fulfillment_status: &str` -- Filter orders by their fulfillment status.
     *                     (default: any)
     *                       
     *                           shipped: Show orders that have been shipped. Returns orders with fulfillment_status of fulfilled.
     *                           partial: Show partially shipped orders.
     *                           unshipped: Show orders that have not yet been shipped. Returns orders with fulfillment_status of null.
     *                           any: Show orders of any fulfillment status.
     *                           unfulfilled: Returns orders with fulfillment_status of null or partial.
     * * `fields: &str` -- Retrieve only certain fields, specified by a comma-separated list of fields names.
     */
    pub async fn deprecated_202001_get(
        &self,
        ids: &str,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        processed_at_min: &str,
        processed_at_max: &str,
        attribution_app_id: &str,
        status: &str,
        financial_status: &str,
        fulfillment_status: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !attribution_app_id.is_empty() {
            query_args.push((
                "attribution_app_id".to_string(),
                attribution_app_id.to_string(),
            ));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !financial_status.is_empty() {
            query_args.push(("financial_status".to_string(), financial_status.to_string()));
        }
        if !fulfillment_status.is_empty() {
            query_args.push((
                "fulfillment_status".to_string(),
                fulfillment_status.to_string(),
            ));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !processed_at_max.is_empty() {
            query_args.push(("processed_at_max".to_string(), processed_at_max.to_string()));
        }
        if !processed_at_min.is_empty() {
            query_args.push(("processed_at_min".to_string(), processed_at_min.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-01/orders.json?{}", query_), None);
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
    * Creates an order. By default, product inventory is not claimed.
              When you create an order, you can include the following option parameters in the body of the request:

                inventory_behaviour: The behaviour to use when updating inventory. (default: bypass)

                    bypass: Do not claim inventory.
                    decrement_ignoring_policy: Ignore the product's inventory policy and claim inventory.
                    decrement_obeying_policy: Follow the product's inventory policy and claim inventory, if possible.


                send_receipt: Whether to send an order confirmation to the customer.


                  Note
                  If you're working on a private app and order confirmations are still being sent to the customer when send_receipt is set to false, then you need to disable the Storefront API from the private app's page in the Shopify admin.


                send_fulfillment_receipt: Whether to send a shipping confirmation to the customer.


                Note
                If you are including shipping_address or billing_address, make sure to pass both
                  first_name and last_name. Otherwise both these addresses will be ignored.
                If you're using this endpoint with a trial or Partner development store, then you can create no more than 5 new orders per minute.
    *
    * This function performs a `POST` to the `/admin/api/2020-01/orders.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/order#create-2020-01
    */
    pub async fn deprecated_202001_create(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-01/orders.json", None);
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
     * Retrieves a specific order.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/orders/{order_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order#show-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Retrieve only certain fields, specified by a comma-separated list of fields names.
     */
    pub async fn deprecated_202001_get_param(
        &self,
        order_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/json?{}",
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
     * Updates an order.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/orders/{order_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order#update-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_param(
        &self,
        order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/json",
                crate::progenitor_support::encode_path(order_id),
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
     * Deletes an order. Orders that interact with an online gateway can't be deleted.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/orders/{order_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_param(&self, order_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/json",
                crate::progenitor_support::encode_path(order_id),
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
     * Retrieves an order count.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/orders/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order#count-2020-01
     *
     * **Parameters:**
     *
     * * `created_at_min: &str` -- Count orders created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count orders created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Count orders last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count orders last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `status: &str` -- Count orders of a given status.
     *                     (default: open)
     *                       
     *                           open: Count open orders.
     *                           closed: Count closed orders.
     *                           any: Count orders of any status.
     * * `financial_status: &str` -- Count orders of a given financial status.
     *                     (default: any)
     *                       
     *                           authorized: Count authorized orders.
     *                           pending: Count pending orders.
     *                           paid: Count paid orders.
     *                           refunded: Count refunded orders.
     *                           voided: Count voided orders.
     *                           any: Count orders of any financial status.
     * * `fulfillment_status: &str` -- Filter orders by their fulfillment status.
     *                     (default: any)
     *                       
     *                           shipped: Show orders that have been shipped. Returns orders with fulfillment_status of fulfilled.
     *                           partial: Show partially shipped orders.
     *                           unshipped: Show orders that have not yet been shipped. Returns orders with fulfillment_status of null.
     *                           any: Show orders of any fulfillment status.
     *                           unfulfilled: Returns orders with fulfillment_status of null or partial.
     */
    pub async fn deprecated_202001_get_count(
        &self,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        status: &str,
        financial_status: &str,
        fulfillment_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !financial_status.is_empty() {
            query_args.push(("financial_status".to_string(), financial_status.to_string()));
        }
        if !fulfillment_status.is_empty() {
            query_args.push((
                "fulfillment_status".to_string(),
                fulfillment_status.to_string(),
            ));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/orders/count.json?{}", query_),
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
     * Closes an order.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/orders/{order_id}/close.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order#close-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_param_close(
        &self,
        order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/close.json",
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
     * Re-opens a closed order.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/orders/{order_id}/open.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order#open-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_param_open(
        &self,
        order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/open.json",
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
    * Caution
      For multi-currency orders, the currency property is required whenever the amount property is provided. For more information, see Migrating to support multiple currencies.

    Cancels an order. Orders that have a fulfillment object can't be canceled.
    *
    * This function performs a `POST` to the `/admin/api/2020-01/orders/{order_id}/cancel.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/order#cancel-2020-01
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `amount: &str` -- The amount to refund. If set, Shopify attempts to void or refund the payment, depending on its status. Shopify refunds through a manual gateway in cases where the original transaction was not made in Shopify. Refunds through a manual gateway are recorded as a refund on Shopify, but the customer is not refunded.
    * * `currency: &str` -- The currency of the refund that's issued when the order is canceled. Required for multi-currency orders whenever the amount property is provided.
    * * `restock_deprecated: &str` -- Whether to restock refunded items back to your store's inventory.
    *                     (default: false).
    * * `reason: &str` -- The reason for the order cancellation. Valid values: customer, inventory, fraud, declined, and other.)
    *                     (default: other).
    * * `email: &str` -- Whether to send an email to the customer notifying them of the cancellation.
    *                     (default: false).
    * * `refund: &str` -- The refund transactions to perform. Required for some more complex refund situations. For more information, see the Refund API.
    */
    pub async fn deprecated_202001_create_param_cancel(
        &self,
        order_id: &str,
        amount: &str,
        currency: &str,
        restock_deprecated: &str,
        reason: &str,
        email: &str,
        refund: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !amount.is_empty() {
            query_args.push(("amount".to_string(), amount.to_string()));
        }
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        if !email.is_empty() {
            query_args.push(("email".to_string(), email.to_string()));
        }
        if !reason.is_empty() {
            query_args.push(("reason".to_string(), reason.to_string()));
        }
        if !refund.is_empty() {
            query_args.push(("refund".to_string(), refund.to_string()));
        }
        if !restock_deprecated.is_empty() {
            query_args.push((
                "restock
                  deprecated"
                    .to_string(),
                restock_deprecated.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/cancel.json?{}",
                crate::progenitor_support::encode_path(order_id),
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
     * Retrieves a list of orders. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/orders.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order#index-2020-04
     *
     * **Parameters:**
     *
     * * `ids: &str` -- Retrieve only orders specified by a comma-separated list of order IDs.
     * * `limit: &str` -- The maximum number of results to show on a page.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Show orders after the specified ID.
     * * `created_at_min: &str` -- Show orders created at or after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show orders created at or before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show orders last updated at or after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show orders last updated at or before date (format: 2014-04-25T16:15:47-04:00).
     * * `processed_at_min: &str` -- Show orders imported at or after date (format: 2014-04-25T16:15:47-04:00).
     * * `processed_at_max: &str` -- Show orders imported at or before date (format: 2014-04-25T16:15:47-04:00).
     * * `attribution_app_id: &str` -- Show orders attributed to a certain app, specified by the app ID. Set as current to show orders for the app currently consuming the API.
     * * `status: &str` -- Filter orders by their status.
     *                     (default: open)
     *                       
     *                           open: Show only open orders.
     *                           closed: Show only closed orders.
     *                           cancelled: Show only canceled orders.
     *                           any: Show orders of any status, including archived orders.
     * * `financial_status: &str` -- Filter orders by their financial status.
     *                     (default: any)
     *                       
     *                           authorized: Show only authorized orders
     *                           pending: Show only pending orders
     *                           paid: Show only paid orders
     *                           partially_paid: Show only partially paid orders
     *                           refunded: Show only refunded orders
     *                           voided: Show only voided orders
     *                           partially_refunded: Show only partially refunded orders
     *                           any: Show orders of any financial status.
     *                           unpaid: Show authorized and partially paid orders.
     * * `fulfillment_status: &str` -- Filter orders by their fulfillment status.
     *                     (default: any)
     *                       
     *                           shipped: Show orders that have been shipped. Returns orders with fulfillment_status of fulfilled.
     *                           partial: Show partially shipped orders.
     *                           unshipped: Show orders that have not yet been shipped. Returns orders with fulfillment_status of null.
     *                           any: Show orders of any fulfillment status.
     *                           unfulfilled: Returns orders with fulfillment_status of null or partial.
     * * `fields: &str` -- Retrieve only certain fields, specified by a comma-separated list of fields names.
     */
    pub async fn deprecated_202004_get(
        &self,
        ids: &str,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        processed_at_min: &str,
        processed_at_max: &str,
        attribution_app_id: &str,
        status: &str,
        financial_status: &str,
        fulfillment_status: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !attribution_app_id.is_empty() {
            query_args.push((
                "attribution_app_id".to_string(),
                attribution_app_id.to_string(),
            ));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !financial_status.is_empty() {
            query_args.push(("financial_status".to_string(), financial_status.to_string()));
        }
        if !fulfillment_status.is_empty() {
            query_args.push((
                "fulfillment_status".to_string(),
                fulfillment_status.to_string(),
            ));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !processed_at_max.is_empty() {
            query_args.push(("processed_at_max".to_string(), processed_at_max.to_string()));
        }
        if !processed_at_min.is_empty() {
            query_args.push(("processed_at_min".to_string(), processed_at_min.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-04/orders.json?{}", query_), None);
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
    * Creates an order. By default, product inventory is not claimed.
              When you create an order, you can include the following option parameters in the body of the request:

                inventory_behaviour: The behaviour to use when updating inventory. (default: bypass)

                    bypass: Do not claim inventory.
                    decrement_ignoring_policy: Ignore the product's inventory policy and claim inventory.
                    decrement_obeying_policy: Follow the product's inventory policy and claim inventory, if possible.


                send_receipt: Whether to send an order confirmation to the customer.


                  Note
                  If you're working on a private app and order confirmations are still being sent to the customer when send_receipt is set to false, then you need to disable the Storefront API from the private app's page in the Shopify admin.


                send_fulfillment_receipt: Whether to send a shipping confirmation to the customer.


                Note
                If you are including shipping_address or billing_address, make sure to pass both
                  first_name and last_name. Otherwise both these addresses will be ignored.
                If you're using this endpoint with a trial or Partner development store, then you can create no more than 5 new orders per minute.
    *
    * This function performs a `POST` to the `/admin/api/2020-04/orders.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/order#create-2020-04
    */
    pub async fn deprecated_202004_create(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-04/orders.json", None);
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
     * Retrieves a specific order.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/orders/{order_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order#show-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Retrieve only certain fields, specified by a comma-separated list of fields names.
     */
    pub async fn deprecated_202004_get_param(
        &self,
        order_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/json?{}",
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
     * Updates an order.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/orders/{order_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order#update-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_param(
        &self,
        order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/json",
                crate::progenitor_support::encode_path(order_id),
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
     * Deletes an order. Orders that interact with an online gateway can't be deleted.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/orders/{order_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_param(&self, order_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/json",
                crate::progenitor_support::encode_path(order_id),
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
     * Retrieves an order count.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/orders/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order#count-2020-04
     *
     * **Parameters:**
     *
     * * `created_at_min: &str` -- Count orders created after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Count orders created before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Count orders last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count orders last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `status: &str` -- Count orders of a given status.
     *                     (default: open)
     *                       
     *                           open: Count open orders.
     *                           closed: Count closed orders.
     *                           any: Count orders of any status.
     * * `financial_status: &str` -- Count orders of a given financial status.
     *                     (default: any)
     *                       
     *                           authorized: Count authorized orders.
     *                           pending: Count pending orders.
     *                           paid: Count paid orders.
     *                           refunded: Count refunded orders.
     *                           voided: Count voided orders.
     *                           any: Count orders of any financial status.
     * * `fulfillment_status: &str` -- Filter orders by their fulfillment status.
     *                     (default: any)
     *                       
     *                           shipped: Show orders that have been shipped. Returns orders with fulfillment_status of fulfilled.
     *                           partial: Show partially shipped orders.
     *                           unshipped: Show orders that have not yet been shipped. Returns orders with fulfillment_status of null.
     *                           any: Show orders of any fulfillment status.
     *                           unfulfilled: Returns orders with fulfillment_status of null or partial.
     */
    pub async fn deprecated_202004_get_count(
        &self,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        status: &str,
        financial_status: &str,
        fulfillment_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !financial_status.is_empty() {
            query_args.push(("financial_status".to_string(), financial_status.to_string()));
        }
        if !fulfillment_status.is_empty() {
            query_args.push((
                "fulfillment_status".to_string(),
                fulfillment_status.to_string(),
            ));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/orders/count.json?{}", query_),
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
     * Closes an order.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/orders/{order_id}/close.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order#close-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_create_param_close(
        &self,
        order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/close.json",
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
     * Re-opens a closed order.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/orders/{order_id}/open.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order#open-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_create_param_open(
        &self,
        order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/open.json",
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
    * Caution
      For multi-currency orders, the currency property is required whenever the amount property is provided. For more information, see Migrating to support multiple currencies.

    Cancels an order. Orders that have a fulfillment object can't be canceled.
    *
    * This function performs a `POST` to the `/admin/api/2020-04/orders/{order_id}/cancel.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/order#cancel-2020-04
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `amount: &str` -- The amount to refund. If set, Shopify attempts to void or refund the payment, depending on its status. Shopify refunds through a manual gateway in cases where the original transaction was not made in Shopify. Refunds through a manual gateway are recorded as a refund on Shopify, but the customer is not refunded.
    * * `currency: &str` -- The currency of the refund that's issued when the order is canceled. Required for multi-currency orders whenever the amount property is provided.
    * * `restock_deprecated: &str` -- Whether to restock refunded items back to your store's inventory.
    *                     (default: false).
    * * `reason: &str` -- The reason for the order cancellation. Valid values: customer, inventory, fraud, declined, and other.)
    *                     (default: other).
    * * `email: &str` -- Whether to send an email to the customer notifying them of the cancellation.
    *                     (default: false).
    * * `refund: &str` -- The refund transactions to perform. Required for some more complex refund situations. For more information, see the Refund API.
    */
    pub async fn deprecated_202004_create_param_cancel(
        &self,
        order_id: &str,
        amount: &str,
        currency: &str,
        restock_deprecated: &str,
        reason: &str,
        email: &str,
        refund: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !amount.is_empty() {
            query_args.push(("amount".to_string(), amount.to_string()));
        }
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        if !email.is_empty() {
            query_args.push(("email".to_string(), email.to_string()));
        }
        if !reason.is_empty() {
            query_args.push(("reason".to_string(), reason.to_string()));
        }
        if !refund.is_empty() {
            query_args.push(("refund".to_string(), refund.to_string()));
        }
        if !restock_deprecated.is_empty() {
            query_args.push((
                "restock
                  deprecated"
                    .to_string(),
                restock_deprecated.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/cancel.json?{}",
                crate::progenitor_support::encode_path(order_id),
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
     * Retrieves a list of orders. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/orders.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order#index-2020-07
     *
     * **Parameters:**
     *
     * * `ids: &str` -- Retrieve only orders specified by a comma-separated list of order IDs.
     * * `limit: &str` -- The maximum number of results to show on a page.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Show orders after the specified ID.
     * * `created_at_min: &str` -- Show orders created at or after date (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show orders created at or before date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show orders last updated at or after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show orders last updated at or before date (format: 2014-04-25T16:15:47-04:00).
     * * `processed_at_min: &str` -- Show orders imported at or after date (format: 2014-04-25T16:15:47-04:00).
     * * `processed_at_max: &str` -- Show orders imported at or before date (format: 2014-04-25T16:15:47-04:00).
     * * `attribution_app_id: &str` -- Show orders attributed to a certain app, specified by the app ID. Set as current to show orders for the app currently consuming the API.
     * * `status: &str` -- Filter orders by their status.
     *                     (default: open)
     *                       
     *                           open: Show only open orders.
     *                           closed: Show only closed orders.
     *                           cancelled: Show only canceled orders.
     *                           any: Show orders of any status, including archived orders.
     * * `financial_status: &str` -- Filter orders by their financial status.
     *                     (default: any)
     *                       
     *                           authorized: Show only authorized orders
     *                           pending: Show only pending orders
     *                           paid: Show only paid orders
     *                           partially_paid: Show only partially paid orders
     *                           refunded: Show only refunded orders
     *                           voided: Show only voided orders
     *                           partially_refunded: Show only partially refunded orders
     *                           any: Show orders of any financial status.
     *                           unpaid: Show authorized and partially paid orders.
     * * `fulfillment_status: &str` -- Filter orders by their fulfillment status.
     *                     (default: any)
     *                       
     *                           shipped: Show orders that have been shipped. Returns orders with fulfillment_status of fulfilled.
     *                           partial: Show partially shipped orders.
     *                           unshipped: Show orders that have not yet been shipped. Returns orders with fulfillment_status of null.
     *                           any: Show orders of any fulfillment status.
     *                           unfulfilled: Returns orders with fulfillment_status of null or partial.
     * * `fields: &str` -- Retrieve only certain fields, specified by a comma-separated list of fields names.
     */
    pub async fn deprecated_202007_get(
        &self,
        ids: &str,
        limit: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        processed_at_min: &str,
        processed_at_max: &str,
        attribution_app_id: &str,
        status: &str,
        financial_status: &str,
        fulfillment_status: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !attribution_app_id.is_empty() {
            query_args.push((
                "attribution_app_id".to_string(),
                attribution_app_id.to_string(),
            ));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !financial_status.is_empty() {
            query_args.push(("financial_status".to_string(), financial_status.to_string()));
        }
        if !fulfillment_status.is_empty() {
            query_args.push((
                "fulfillment_status".to_string(),
                fulfillment_status.to_string(),
            ));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !processed_at_max.is_empty() {
            query_args.push(("processed_at_max".to_string(), processed_at_max.to_string()));
        }
        if !processed_at_min.is_empty() {
            query_args.push(("processed_at_min".to_string(), processed_at_min.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-07/orders.json?{}", query_), None);
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
     * Retrieves a list of all order risks for an order. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/orders/{order_id}/risks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#index-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_param_risk(&self, order_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/risks.json",
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
     * Creates an order risk for an order.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/orders/{order_id}/risks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#create-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_param_risks(
        &self,
        order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/risks.json",
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
     * Retrieves a single order risk by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/orders/{order_id}/risks/{risk_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#show-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `risk_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_param_risks_risk(
        &self,
        order_id: &str,
        risk_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/risks/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(risk_id),
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
    * Updates an order risk


                Note
                You cannot modify an order risk that was created by another application.
    *
    * This function performs a `PUT` to the `/admin/api/2020-01/orders/{order_id}/risks/{risk_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#update-2020-01
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `risk_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202001_update_param_risks_risk(
        &self,
        order_id: &str,
        risk_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/risks/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(risk_id),
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
    * Deletes an order risk for an order


                Note
                You cannot delete an order risk that was created by another application.
    *
    * This function performs a `DELETE` to the `/admin/api/2020-01/orders/{order_id}/risks/{risk_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#destroy-2020-01
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `risk_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202001_delete_param_risks_risk(
        &self,
        order_id: &str,
        risk_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/risks/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(risk_id),
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
     * Retrieves a list of all order risks for an order. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/orders/{order_id}/risks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#index-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_param_risk(&self, order_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/risks.json",
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
     * Creates an order risk for an order.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/orders/{order_id}/risks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#create-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_create_param_risks(
        &self,
        order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/risks.json",
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
     * Retrieves a single order risk by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/orders/{order_id}/risks/{risk_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#show-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `risk_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_param_risks_risk(
        &self,
        order_id: &str,
        risk_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/risks/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(risk_id),
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
    * Updates an order risk


                Note
                You cannot modify an order risk that was created by another application.
    *
    * This function performs a `PUT` to the `/admin/api/2020-04/orders/{order_id}/risks/{risk_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#update-2020-04
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `risk_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202004_update_param_risks_risk(
        &self,
        order_id: &str,
        risk_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/risks/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(risk_id),
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
    * Deletes an order risk for an order


                Note
                You cannot delete an order risk that was created by another application.
    *
    * This function performs a `DELETE` to the `/admin/api/2020-04/orders/{order_id}/risks/{risk_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#destroy-2020-04
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `risk_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202004_delete_param_risks_risk(
        &self,
        order_id: &str,
        risk_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/risks/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(risk_id),
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
     * Retrieves a list of all order risks for an order. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/orders/{order_id}/risks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#index-2020-07
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_param_risk(&self, order_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/risks.json",
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
     * Creates an order risk for an order.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/orders/{order_id}/risks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#create-2020-07
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_create_param_risks(
        &self,
        order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/risks.json",
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
     * Retrieves a single order risk by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/orders/{order_id}/risks/{risk_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#show-2020-07
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `risk_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_param_risks_risk(
        &self,
        order_id: &str,
        risk_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/risks/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(risk_id),
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
    * Updates an order risk


                Note
                You cannot modify an order risk that was created by another application.
    *
    * This function performs a `PUT` to the `/admin/api/2020-07/orders/{order_id}/risks/{risk_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#update-2020-07
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `risk_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202007_update_param_risks_risk(
        &self,
        order_id: &str,
        risk_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/risks/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(risk_id),
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
    * Deletes an order risk for an order


                Note
                You cannot delete an order risk that was created by another application.
    *
    * This function performs a `DELETE` to the `/admin/api/2020-07/orders/{order_id}/risks/{risk_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#destroy-2020-07
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `risk_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202007_delete_param_risks_risk(
        &self,
        order_id: &str,
        risk_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/risks/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(risk_id),
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
     * Retrieves a list of all order risks for an order. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/orders/{order_id}/risks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#index-2020-10
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     */
    pub async fn get_param_risk(&self, order_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/risks.json",
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
     * Creates an order risk for an order.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/orders/{order_id}/risks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#create-2020-10
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     */
    pub async fn create_param_risks(
        &self,
        order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/risks.json",
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
     * Retrieves a single order risk by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/orders/{order_id}/risks/{risk_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#show-2020-10
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `risk_id: &str` -- storefront_access_token_id.
     */
    pub async fn get_param_risks_risk(&self, order_id: &str, risk_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/risks/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(risk_id),
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
    * Updates an order risk


                Note
                You cannot modify an order risk that was created by another application.
    *
    * This function performs a `PUT` to the `/admin/api/2020-10/orders/{order_id}/risks/{risk_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#update-2020-10
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `risk_id: &str` -- storefront_access_token_id.
    */
    pub async fn update_param_risks_risk(
        &self,
        order_id: &str,
        risk_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/risks/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(risk_id),
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
    * Deletes an order risk for an order


                Note
                You cannot delete an order risk that was created by another application.
    *
    * This function performs a `DELETE` to the `/admin/api/2020-10/orders/{order_id}/risks/{risk_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#destroy-2020-10
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `risk_id: &str` -- storefront_access_token_id.
    */
    pub async fn delete_param_risks_risk(&self, order_id: &str, risk_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/risks/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(risk_id),
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
     * Retrieves a list of all order risks for an order. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/orders/{order_id}/risks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#index-2021-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_param_risk(&self, order_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/risks.json",
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
     * Creates an order risk for an order.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/orders/{order_id}/risks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#create-2021-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_create_param_risks(
        &self,
        order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/risks.json",
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
     * Retrieves a single order risk by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/orders/{order_id}/risks/{risk_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#show-2021-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `risk_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_param_risks_risk(
        &self,
        order_id: &str,
        risk_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/risks/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(risk_id),
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
    * Updates an order risk


                Note
                You cannot modify an order risk that was created by another application.
    *
    * This function performs a `PUT` to the `/admin/api/2021-01/orders/{order_id}/risks/{risk_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#update-2021-01
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `risk_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202101_update_param_risks_risk(
        &self,
        order_id: &str,
        risk_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/risks/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(risk_id),
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
    * Deletes an order risk for an order


                Note
                You cannot delete an order risk that was created by another application.
    *
    * This function performs a `DELETE` to the `/admin/api/2021-01/orders/{order_id}/risks/{risk_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#destroy-2021-01
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `risk_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202101_delete_param_risks_risk(
        &self,
        order_id: &str,
        risk_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/risks/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(risk_id),
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
     * Retrieves a list of all order risks for an order. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/orders/{order_id}/risks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#index-unstable
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_param_risk(&self, order_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/risks.json",
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
     * Creates an order risk for an order.
     *
     * This function performs a `POST` to the `/admin/api/unstable/orders/{order_id}/risks.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#create-unstable
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_create_param_risks(
        &self,
        order_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/risks.json",
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
     * Retrieves a single order risk by its ID.
     *
     * This function performs a `GET` to the `/admin/api/unstable/orders/{order_id}/risks/{risk_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#show-unstable
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `risk_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_param_risks_risk(
        &self,
        order_id: &str,
        risk_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/risks/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(risk_id),
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
    * Updates an order risk


                Note
                You cannot modify an order risk that was created by another application.
    *
    * This function performs a `PUT` to the `/admin/api/unstable/orders/{order_id}/risks/{risk_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#update-unstable
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `risk_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_unstable_update_param_risks_risk(
        &self,
        order_id: &str,
        risk_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/risks/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(risk_id),
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
    * Deletes an order risk for an order


                Note
                You cannot delete an order risk that was created by another application.
    *
    * This function performs a `DELETE` to the `/admin/api/unstable/orders/{order_id}/risks/{risk_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/order-risk#destroy-unstable
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `risk_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_unstable_delete_param_risks_risk(
        &self,
        order_id: &str,
        risk_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/risks/{}/json",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(risk_id),
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
     * Retrieves a list of refunds for an order. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/orders/{order_id}/refunds.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#index-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     * * `in_shop_currency: &str` -- Show amounts in the shop currency for the underlying transaction.
     *                     (default: false).
     */
    pub async fn deprecated_202001_get_param_refund(
        &self,
        order_id: &str,
        limit: &str,
        fields: &str,
        in_shop_currency: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !in_shop_currency.is_empty() {
            query_args.push(("in_shop_currency".to_string(), in_shop_currency.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/refunds.json?{}",
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
    * Caution
                For multi-currency orders, the currency property is required whenever the amount property is provided. For more information, see Migrating to support multiple currencies.

              Creates a refund. Use the calculate endpoint to produce the transactions to submit.


                Note
                When you use this endpoint with a Partner development store or a trial store, you can create only five refunds per minute.
    *
    * This function performs a `POST` to the `/admin/api/2020-01/orders/{order_id}/refunds.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#create-2020-01
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `restock_deprecated: &str` -- Whether to add the line items back to the store inventory. Use restock_type for refund line items instead.
    * * `notify: &str` -- Whether to send a refund notification to the customer.
    * * `note: &str` -- An optional note attached to a refund.
    * * `discrepancy_reason: &str` -- An optional comment that explains a discrepancy between calculated and actual refund amounts. Used to populate the reason property of the resulting order adjustment object attached to the refund. Valid values: restock, damage, customer, and other.
    * * `shipping: &str` -- Specify how much shipping to refund. It has the following properties:
    *
    *                           full_refund: Whether to refund all remaining shipping.
    *                           amount: Set a specific amount to refund for shipping. Takes precedence over full_refund.
    * * `refund_line_items: &str` -- A list of line item IDs, quantities to refund, and restock instructions. Each entry has the following properties:
    *
    *                           line_item_id: The ID of a line item to refund.
    *                           quantity: The quantity to refund.
    *                           restock_type:           How this refund line item affects inventory levels. Valid values:
    *
    *               no_restock: Refunding these items won't affect inventory.
    *               cancel: The items have not yet been fulfilled.
    *               The canceled quantity will be added back to the available count.
    *               The number of fulfillable units for this line item will decrease.
    *               return: The items were already delivered but will be returned to the merchant.
    *               The returned quantity will be added back to the available count. The number of fulfillable units for this
    *               line item will remain unchanged.
    *
    *
    *                           location_id:           The ID of the location where the items should be
    *             restocked. This is required when the value of restock_type is return or cancel.
    *             If the item is not already stocked at the location, then
    *             the item is connected to the location. An error is returned when the item is connected to
    *             a
    *             fulfillment service location and a different location is provided.
    * * `transactions: &str` -- A list of transactions
    *             to process as refunds.
    * * `currency: &str` -- The three-letter code (ISO 4217 format) for the currency used for the refund.
    */
    pub async fn deprecated_202001_create_param_refunds(
        &self,
        order_id: &str,
        restock_deprecated: &str,
        notify: &str,
        note: &str,
        discrepancy_reason: &str,
        shipping: &str,
        refund_line_items: &str,
        transactions: &str,
        currency: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        if !discrepancy_reason.is_empty() {
            query_args.push((
                "discrepancy_reason".to_string(),
                discrepancy_reason.to_string(),
            ));
        }
        if !note.is_empty() {
            query_args.push(("note".to_string(), note.to_string()));
        }
        if !notify.is_empty() {
            query_args.push(("notify".to_string(), notify.to_string()));
        }
        if !refund_line_items.is_empty() {
            query_args.push((
                "refund_line_items".to_string(),
                refund_line_items.to_string(),
            ));
        }
        if !restock_deprecated.is_empty() {
            query_args.push((
                "restock
                  deprecated"
                    .to_string(),
                restock_deprecated.to_string(),
            ));
        }
        if !shipping.is_empty() {
            query_args.push(("shipping".to_string(), shipping.to_string()));
        }
        if !transactions.is_empty() {
            query_args.push(("transactions".to_string(), transactions.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/refunds.json?{}",
                crate::progenitor_support::encode_path(order_id),
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
     * Retrieves a specific refund.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/orders/{order_id}/refunds/{refund_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#show-2020-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `refund_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     * * `in_shop_currency: &str` -- Show amounts in the shop currency for the underlying transaction.
     *                     (default: false).
     */
    pub async fn deprecated_202001_get_param_refunds_refund(
        &self,
        order_id: &str,
        refund_id: &str,
        fields: &str,
        in_shop_currency: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !in_shop_currency.is_empty() {
            query_args.push(("in_shop_currency".to_string(), in_shop_currency.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/refunds/{}/json?{}",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(refund_id),
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
    * Caution
              For multi-currency orders, the currency property is required whenever the amount property is provided. For more information, see Migrating to support multiple currencies.

              Calculates refund transactions based on line items and shipping. When you want to create a refund,
              you should first use the calculate endpoint to generate accurate refund transactions. Specify the line items
              that are being refunded, their quantity and restock instructions, and whether you intend to refund
              shipping costs. If the restock instructions can't be met—for example, because you try to return more items than have been
              fulfilled—then the endpoint returns modified restock instructions. You can then use the response in the body of the request to create the actual refund.
              The response includes a transactions object with "kind": "suggested_refund",
              which must to be changed to "kind" : "refund" for the refund to be accepted.
    *
    * This function performs a `POST` to the `/admin/api/2020-01/orders/{order_id}/refunds/calculate.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#calculate-2020-01
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `shipping: &str` -- Specify how much shipping to refund. It has the following properties:
    *
    *                           full_refund: Whether to refund all remaining shipping.
    *                           amount: Set a specific amount to refund for shipping. Takes precedence over full_refund.
    * * `refund_line_items: &str` -- A list of line item IDs, quantities to refund, and restock instructions. Each entry has the following properties:
    *
    *                           line_item_id: The ID of a line item to refund.
    *                           quantity: The quantity to refund.
    *                           restock_type:           How this refund line item affects inventory levels. Valid values:
    *
    *               no_restock: Refunding these items won't affect inventory.
    *               cancel: The items have not yet been fulfilled. The canceled quantity will be added
    *               back to the available count. The number of fulfillable units for this line item will decrease.
    *               return: The items were already delivered but will be returned to the merchant.
    *               The returned quantity will be added back to the available count.
    *               The number of fulfillable units for this line item will remain unchanged.
    *
    *
    *                           location_id:           The ID of the location
    *             where the items should be restocked. If location_id is not provided and the value of
    *             restock_type is return or cancel, then the endpoint returns a suitable
    *             location ID.
    *
    *                           already_stocked:           Whether the item is already stocked at
    *             the location. If this is false, then creating the refund will connect the item to the location and start
    *             stocking it there.
    * * `currency: &str` -- The three-letter code (ISO 4217 format) for the
    *             currency used for the refund. Note: Required whenever the shipping amount property is provided.
    */
    pub async fn deprecated_202001_create_param_refunds_calculate(
        &self,
        order_id: &str,
        shipping: &str,
        refund_line_items: &str,
        currency: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        if !refund_line_items.is_empty() {
            query_args.push((
                "refund_line_items".to_string(),
                refund_line_items.to_string(),
            ));
        }
        if !shipping.is_empty() {
            query_args.push(("shipping".to_string(), shipping.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/orders/{}/refunds/calculate.json?{}",
                crate::progenitor_support::encode_path(order_id),
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
     * Retrieves a list of refunds for an order. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/orders/{order_id}/refunds.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#index-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     * * `in_shop_currency: &str` -- Show amounts in the shop currency for the underlying transaction.
     *                     (default: false).
     */
    pub async fn deprecated_202004_get_param_refund(
        &self,
        order_id: &str,
        limit: &str,
        fields: &str,
        in_shop_currency: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !in_shop_currency.is_empty() {
            query_args.push(("in_shop_currency".to_string(), in_shop_currency.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/refunds.json?{}",
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
    * Caution
                For multi-currency orders, the currency property is required whenever the amount property is provided. For more information, see Migrating to support multiple currencies.

              Creates a refund. Use the calculate endpoint to produce the transactions to submit.


                Note
                When you use this endpoint with a Partner development store or a trial store, you can create only five refunds per minute.
    *
    * This function performs a `POST` to the `/admin/api/2020-04/orders/{order_id}/refunds.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#create-2020-04
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `restock_deprecated: &str` -- Whether to add the line items back to the store inventory. Use restock_type for refund line items instead.
    * * `notify: &str` -- Whether to send a refund notification to the customer.
    * * `note: &str` -- An optional note attached to a refund.
    * * `discrepancy_reason: &str` -- An optional comment that explains a discrepancy between calculated and actual refund amounts. Used to populate the reason property of the resulting order adjustment object attached to the refund. Valid values: restock, damage, customer, and other.
    * * `shipping: &str` -- Specify how much shipping to refund. It has the following properties:
    *
    *                           full_refund: Whether to refund all remaining shipping.
    *                           amount: Set a specific amount to refund for shipping. Takes precedence over full_refund.
    * * `refund_line_items: &str` -- A list of line item IDs, quantities to refund, and restock instructions. Each entry has the following properties:
    *
    *                           line_item_id: The ID of a line item to refund.
    *                           quantity: The quantity to refund.
    *                           restock_type:           How this refund line item affects inventory levels. Valid values:
    *
    *               no_restock: Refunding these items won't affect inventory.
    *               cancel: The items have not yet been fulfilled.
    *               The canceled quantity will be added back to the available count.
    *               The number of fulfillable units for this line item will decrease.
    *               return: The items were already delivered but will be returned to the merchant.
    *               The returned quantity will be added back to the available count. The number of fulfillable units for this
    *               line item will remain unchanged.
    *
    *
    *                           location_id:           The ID of the location where the items should be
    *             restocked. This is required when the value of restock_type is return or cancel.
    *             If the item is not already stocked at the location, then
    *             the item is connected to the location. An error is returned when the item is connected to
    *             a
    *             fulfillment service location and a different location is provided.
    * * `transactions: &str` -- A list of transactions
    *             to process as refunds.
    * * `currency: &str` -- The three-letter code (ISO 4217 format) for the currency used for the refund.
    */
    pub async fn deprecated_202004_create_param_refunds(
        &self,
        order_id: &str,
        restock_deprecated: &str,
        notify: &str,
        note: &str,
        discrepancy_reason: &str,
        shipping: &str,
        refund_line_items: &str,
        transactions: &str,
        currency: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        if !discrepancy_reason.is_empty() {
            query_args.push((
                "discrepancy_reason".to_string(),
                discrepancy_reason.to_string(),
            ));
        }
        if !note.is_empty() {
            query_args.push(("note".to_string(), note.to_string()));
        }
        if !notify.is_empty() {
            query_args.push(("notify".to_string(), notify.to_string()));
        }
        if !refund_line_items.is_empty() {
            query_args.push((
                "refund_line_items".to_string(),
                refund_line_items.to_string(),
            ));
        }
        if !restock_deprecated.is_empty() {
            query_args.push((
                "restock
                  deprecated"
                    .to_string(),
                restock_deprecated.to_string(),
            ));
        }
        if !shipping.is_empty() {
            query_args.push(("shipping".to_string(), shipping.to_string()));
        }
        if !transactions.is_empty() {
            query_args.push(("transactions".to_string(), transactions.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/refunds.json?{}",
                crate::progenitor_support::encode_path(order_id),
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
     * Retrieves a specific refund.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/orders/{order_id}/refunds/{refund_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#show-2020-04
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `refund_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     * * `in_shop_currency: &str` -- Show amounts in the shop currency for the underlying transaction.
     *                     (default: false).
     */
    pub async fn deprecated_202004_get_param_refunds_refund(
        &self,
        order_id: &str,
        refund_id: &str,
        fields: &str,
        in_shop_currency: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !in_shop_currency.is_empty() {
            query_args.push(("in_shop_currency".to_string(), in_shop_currency.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/refunds/{}/json?{}",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(refund_id),
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
    * Caution
              For multi-currency orders, the currency property is required whenever the amount property is provided. For more information, see Migrating to support multiple currencies.

              Calculates refund transactions based on line items and shipping. When you want to create a refund,
              you should first use the calculate endpoint to generate accurate refund transactions. Specify the line items
              that are being refunded, their quantity and restock instructions, and whether you intend to refund
              shipping costs. If the restock instructions can't be met—for example, because you try to return more items than have been
              fulfilled—then the endpoint returns modified restock instructions. You can then use the response in the body of the request to create the actual refund.
              The response includes a transactions object with "kind": "suggested_refund",
              which must to be changed to "kind" : "refund" for the refund to be accepted.
    *
    * This function performs a `POST` to the `/admin/api/2020-04/orders/{order_id}/refunds/calculate.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#calculate-2020-04
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `shipping: &str` -- Specify how much shipping to refund. It has the following properties:
    *
    *                           full_refund: Whether to refund all remaining shipping.
    *                           amount: Set a specific amount to refund for shipping. Takes precedence over full_refund.
    * * `refund_line_items: &str` -- A list of line item IDs, quantities to refund, and restock instructions. Each entry has the following properties:
    *
    *                           line_item_id: The ID of a line item to refund.
    *                           quantity: The quantity to refund.
    *                           restock_type:           How this refund line item affects inventory levels. Valid values:
    *
    *               no_restock: Refunding these items won't affect inventory.
    *               cancel: The items have not yet been fulfilled. The canceled quantity will be added
    *               back to the available count. The number of fulfillable units for this line item will decrease.
    *               return: The items were already delivered but will be returned to the merchant.
    *               The returned quantity will be added back to the available count.
    *               The number of fulfillable units for this line item will remain unchanged.
    *
    *
    *                           location_id:           The ID of the location
    *             where the items should be restocked. If location_id is not provided and the value of
    *             restock_type is return or cancel, then the endpoint returns a suitable
    *             location ID.
    *
    *                           already_stocked:           Whether the item is already stocked at
    *             the location. If this is false, then creating the refund will connect the item to the location and start
    *             stocking it there.
    * * `currency: &str` -- The three-letter code (ISO 4217 format) for the
    *             currency used for the refund. Note: Required whenever the shipping amount property is provided.
    */
    pub async fn deprecated_202004_create_param_refunds_calculate(
        &self,
        order_id: &str,
        shipping: &str,
        refund_line_items: &str,
        currency: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        if !refund_line_items.is_empty() {
            query_args.push((
                "refund_line_items".to_string(),
                refund_line_items.to_string(),
            ));
        }
        if !shipping.is_empty() {
            query_args.push(("shipping".to_string(), shipping.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/orders/{}/refunds/calculate.json?{}",
                crate::progenitor_support::encode_path(order_id),
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
     * Retrieves a list of refunds for an order. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/orders/{order_id}/refunds.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#index-2020-07
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     * * `in_shop_currency: &str` -- Show amounts in the shop currency for the underlying transaction.
     *                     (default: false).
     */
    pub async fn deprecated_202007_get_param_refund(
        &self,
        order_id: &str,
        limit: &str,
        fields: &str,
        in_shop_currency: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !in_shop_currency.is_empty() {
            query_args.push(("in_shop_currency".to_string(), in_shop_currency.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/refunds.json?{}",
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
    * Caution
                For multi-currency orders, the currency property is required whenever the amount property is provided. For more information, see Migrating to support multiple currencies.

              Creates a refund. Use the calculate endpoint to produce the transactions to submit.


                Note
                When you use this endpoint with a Partner development store or a trial store, you can create only five refunds per minute.
    *
    * This function performs a `POST` to the `/admin/api/2020-07/orders/{order_id}/refunds.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#create-2020-07
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `restock_deprecated: &str` -- Whether to add the line items back to the store inventory. Use restock_type for refund line items instead.
    * * `notify: &str` -- Whether to send a refund notification to the customer.
    * * `note: &str` -- An optional note attached to a refund.
    * * `discrepancy_reason: &str` -- An optional comment that explains a discrepancy between calculated and actual refund amounts. Used to populate the reason property of the resulting order adjustment object attached to the refund. Valid values: restock, damage, customer, and other.
    * * `shipping: &str` -- Specify how much shipping to refund. It has the following properties:
    *
    *                           full_refund: Whether to refund all remaining shipping.
    *                           amount: Set a specific amount to refund for shipping. Takes precedence over full_refund.
    * * `refund_line_items: &str` -- A list of line item IDs, quantities to refund, and restock instructions. Each entry has the following properties:
    *
    *                           line_item_id: The ID of a line item to refund.
    *                           quantity: The quantity to refund.
    *                           restock_type:           How this refund line item affects inventory levels. Valid values:
    *
    *               no_restock: Refunding these items won't affect inventory.
    *               cancel: The items have not yet been fulfilled.
    *               The canceled quantity will be added back to the available count.
    *               The number of fulfillable units for this line item will decrease.
    *               return: The items were already delivered but will be returned to the merchant.
    *               The returned quantity will be added back to the available count. The number of fulfillable units for this
    *               line item will remain unchanged.
    *
    *
    *                           location_id:           The ID of the location where the items should be
    *             restocked. This is required when the value of restock_type is return or cancel.
    *             If the item is not already stocked at the location, then
    *             the item is connected to the location. An error is returned when the item is connected to
    *             a
    *             fulfillment service location and a different location is provided.
    * * `transactions: &str` -- A list of transactions
    *             to process as refunds.
    * * `currency: &str` -- The three-letter code (ISO 4217 format) for the currency used for the refund.
    */
    pub async fn deprecated_202007_create_param_refunds(
        &self,
        order_id: &str,
        restock_deprecated: &str,
        notify: &str,
        note: &str,
        discrepancy_reason: &str,
        shipping: &str,
        refund_line_items: &str,
        transactions: &str,
        currency: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        if !discrepancy_reason.is_empty() {
            query_args.push((
                "discrepancy_reason".to_string(),
                discrepancy_reason.to_string(),
            ));
        }
        if !note.is_empty() {
            query_args.push(("note".to_string(), note.to_string()));
        }
        if !notify.is_empty() {
            query_args.push(("notify".to_string(), notify.to_string()));
        }
        if !refund_line_items.is_empty() {
            query_args.push((
                "refund_line_items".to_string(),
                refund_line_items.to_string(),
            ));
        }
        if !restock_deprecated.is_empty() {
            query_args.push((
                "restock
                  deprecated"
                    .to_string(),
                restock_deprecated.to_string(),
            ));
        }
        if !shipping.is_empty() {
            query_args.push(("shipping".to_string(), shipping.to_string()));
        }
        if !transactions.is_empty() {
            query_args.push(("transactions".to_string(), transactions.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/refunds.json?{}",
                crate::progenitor_support::encode_path(order_id),
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
     * Retrieves a specific refund.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/orders/{order_id}/refunds/{refund_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#show-2020-07
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `refund_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     * * `in_shop_currency: &str` -- Show amounts in the shop currency for the underlying transaction.
     *                     (default: false).
     */
    pub async fn deprecated_202007_get_param_refunds_refund(
        &self,
        order_id: &str,
        refund_id: &str,
        fields: &str,
        in_shop_currency: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !in_shop_currency.is_empty() {
            query_args.push(("in_shop_currency".to_string(), in_shop_currency.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/refunds/{}/json?{}",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(refund_id),
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
    * Caution
              For multi-currency orders, the currency property is required whenever the amount property is provided. For more information, see Migrating to support multiple currencies.

              Calculates refund transactions based on line items and shipping. When you want to create a refund,
              you should first use the calculate endpoint to generate accurate refund transactions. Specify the line items
              that are being refunded, their quantity and restock instructions, and whether you intend to refund
              shipping costs. If the restock instructions can't be met—for example, because you try to return more items than have been
              fulfilled—then the endpoint returns modified restock instructions. You can then use the response in the body of the request to create the actual refund.
              The response includes a transactions object with "kind": "suggested_refund",
              which must to be changed to "kind" : "refund" for the refund to be accepted.
    *
    * This function performs a `POST` to the `/admin/api/2020-07/orders/{order_id}/refunds/calculate.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#calculate-2020-07
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `shipping: &str` -- Specify how much shipping to refund. It has the following properties:
    *
    *                           full_refund: Whether to refund all remaining shipping.
    *                           amount: Set a specific amount to refund for shipping. Takes precedence over full_refund.
    * * `refund_line_items: &str` -- A list of line item IDs, quantities to refund, and restock instructions. Each entry has the following properties:
    *
    *                           line_item_id: The ID of a line item to refund.
    *                           quantity: The quantity to refund.
    *                           restock_type:           How this refund line item affects inventory levels. Valid values:
    *
    *               no_restock: Refunding these items won't affect inventory.
    *               cancel: The items have not yet been fulfilled. The canceled quantity will be added
    *               back to the available count. The number of fulfillable units for this line item will decrease.
    *               return: The items were already delivered but will be returned to the merchant.
    *               The returned quantity will be added back to the available count.
    *               The number of fulfillable units for this line item will remain unchanged.
    *
    *
    *                           location_id:           The ID of the location
    *             where the items should be restocked. If location_id is not provided and the value of
    *             restock_type is return or cancel, then the endpoint returns a suitable
    *             location ID.
    *
    *                           already_stocked:           Whether the item is already stocked at
    *             the location. If this is false, then creating the refund will connect the item to the location and start
    *             stocking it there.
    * * `currency: &str` -- The three-letter code (ISO 4217 format) for the
    *             currency used for the refund. Note: Required whenever the shipping amount property is provided.
    */
    pub async fn deprecated_202007_create_param_refunds_calculate(
        &self,
        order_id: &str,
        shipping: &str,
        refund_line_items: &str,
        currency: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        if !refund_line_items.is_empty() {
            query_args.push((
                "refund_line_items".to_string(),
                refund_line_items.to_string(),
            ));
        }
        if !shipping.is_empty() {
            query_args.push(("shipping".to_string(), shipping.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/orders/{}/refunds/calculate.json?{}",
                crate::progenitor_support::encode_path(order_id),
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
     * Retrieves a list of refunds for an order. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/orders/{order_id}/refunds.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#index-2020-10
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     * * `in_shop_currency: &str` -- Show amounts in the shop currency for the underlying transaction.
     *                     (default: false).
     */
    pub async fn get_param_refund(
        &self,
        order_id: &str,
        limit: &str,
        fields: &str,
        in_shop_currency: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !in_shop_currency.is_empty() {
            query_args.push(("in_shop_currency".to_string(), in_shop_currency.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/refunds.json?{}",
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
    * Caution
                For multi-currency orders, the currency property is required whenever the amount property is provided. For more information, see Migrating to support multiple currencies.

              Creates a refund. Use the calculate endpoint to produce the transactions to submit.


                Note
                When you use this endpoint with a Partner development store or a trial store, you can create only five refunds per minute.
    *
    * This function performs a `POST` to the `/admin/api/2020-10/orders/{order_id}/refunds.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#create-2020-10
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `restock_deprecated: &str` -- Whether to add the line items back to the store inventory. Use restock_type for refund line items instead.
    * * `notify: &str` -- Whether to send a refund notification to the customer.
    * * `note: &str` -- An optional note attached to a refund.
    * * `discrepancy_reason: &str` -- An optional comment that explains a discrepancy between calculated and actual refund amounts. Used to populate the reason property of the resulting order adjustment object attached to the refund. Valid values: restock, damage, customer, and other.
    * * `shipping: &str` -- Specify how much shipping to refund. It has the following properties:
    *
    *                           full_refund: Whether to refund all remaining shipping.
    *                           amount: Set a specific amount to refund for shipping. Takes precedence over full_refund.
    * * `refund_line_items: &str` -- A list of line item IDs, quantities to refund, and restock instructions. Each entry has the following properties:
    *
    *                           line_item_id: The ID of a line item to refund.
    *                           quantity: The quantity to refund.
    *                           restock_type:           How this refund line item affects inventory levels. Valid values:
    *
    *               no_restock: Refunding these items won't affect inventory.
    *               cancel: The items have not yet been fulfilled.
    *               The canceled quantity will be added back to the available count.
    *               The number of fulfillable units for this line item will decrease.
    *               return: The items were already delivered but will be returned to the merchant.
    *               The returned quantity will be added back to the available count. The number of fulfillable units for this
    *               line item will remain unchanged.
    *
    *
    *                           location_id:           The ID of the location where the items should be
    *             restocked. This is required when the value of restock_type is return or cancel.
    *             If the item is not already stocked at the location, then
    *             the item is connected to the location. An error is returned when the item is connected to
    *             a
    *             fulfillment service location and a different location is provided.
    * * `transactions: &str` -- A list of transactions
    *             to process as refunds.
    * * `currency: &str` -- The three-letter code (ISO 4217 format) for the currency used for the refund.
    */
    pub async fn create_param_refunds(
        &self,
        order_id: &str,
        restock_deprecated: &str,
        notify: &str,
        note: &str,
        discrepancy_reason: &str,
        shipping: &str,
        refund_line_items: &str,
        transactions: &str,
        currency: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        if !discrepancy_reason.is_empty() {
            query_args.push((
                "discrepancy_reason".to_string(),
                discrepancy_reason.to_string(),
            ));
        }
        if !note.is_empty() {
            query_args.push(("note".to_string(), note.to_string()));
        }
        if !notify.is_empty() {
            query_args.push(("notify".to_string(), notify.to_string()));
        }
        if !refund_line_items.is_empty() {
            query_args.push((
                "refund_line_items".to_string(),
                refund_line_items.to_string(),
            ));
        }
        if !restock_deprecated.is_empty() {
            query_args.push((
                "restock
                  deprecated"
                    .to_string(),
                restock_deprecated.to_string(),
            ));
        }
        if !shipping.is_empty() {
            query_args.push(("shipping".to_string(), shipping.to_string()));
        }
        if !transactions.is_empty() {
            query_args.push(("transactions".to_string(), transactions.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/refunds.json?{}",
                crate::progenitor_support::encode_path(order_id),
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
     * Retrieves a specific refund.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/orders/{order_id}/refunds/{refund_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#show-2020-10
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `refund_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     * * `in_shop_currency: &str` -- Show amounts in the shop currency for the underlying transaction.
     *                     (default: false).
     */
    pub async fn get_param_refunds_refund(
        &self,
        order_id: &str,
        refund_id: &str,
        fields: &str,
        in_shop_currency: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !in_shop_currency.is_empty() {
            query_args.push(("in_shop_currency".to_string(), in_shop_currency.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/refunds/{}/json?{}",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(refund_id),
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
    * Caution
              For multi-currency orders, the currency property is required whenever the amount property is provided. For more information, see Migrating to support multiple currencies.

              Calculates refund transactions based on line items and shipping. When you want to create a refund,
              you should first use the calculate endpoint to generate accurate refund transactions. Specify the line items
              that are being refunded, their quantity and restock instructions, and whether you intend to refund
              shipping costs. If the restock instructions can't be met—for example, because you try to return more items than have been
              fulfilled—then the endpoint returns modified restock instructions. You can then use the response in the body of the request to create the actual refund.
              The response includes a transactions object with "kind": "suggested_refund",
              which must to be changed to "kind" : "refund" for the refund to be accepted.
    *
    * This function performs a `POST` to the `/admin/api/2020-10/orders/{order_id}/refunds/calculate.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#calculate-2020-10
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `shipping: &str` -- Specify how much shipping to refund. It has the following properties:
    *
    *                           full_refund: Whether to refund all remaining shipping.
    *                           amount: Set a specific amount to refund for shipping. Takes precedence over full_refund.
    * * `refund_line_items: &str` -- A list of line item IDs, quantities to refund, and restock instructions. Each entry has the following properties:
    *
    *                           line_item_id: The ID of a line item to refund.
    *                           quantity: The quantity to refund.
    *                           restock_type:           How this refund line item affects inventory levels. Valid values:
    *
    *               no_restock: Refunding these items won't affect inventory.
    *               cancel: The items have not yet been fulfilled. The canceled quantity will be added
    *               back to the available count. The number of fulfillable units for this line item will decrease.
    *               return: The items were already delivered but will be returned to the merchant.
    *               The returned quantity will be added back to the available count.
    *               The number of fulfillable units for this line item will remain unchanged.
    *
    *
    *                           location_id:           The ID of the location
    *             where the items should be restocked. If location_id is not provided and the value of
    *             restock_type is return or cancel, then the endpoint returns a suitable
    *             location ID.
    *
    *                           already_stocked:           Whether the item is already stocked at
    *             the location. If this is false, then creating the refund will connect the item to the location and start
    *             stocking it there.
    * * `currency: &str` -- The three-letter code (ISO 4217 format) for the
    *             currency used for the refund. Note: Required whenever the shipping amount property is provided.
    */
    pub async fn create_param_refunds_calculate(
        &self,
        order_id: &str,
        shipping: &str,
        refund_line_items: &str,
        currency: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        if !refund_line_items.is_empty() {
            query_args.push((
                "refund_line_items".to_string(),
                refund_line_items.to_string(),
            ));
        }
        if !shipping.is_empty() {
            query_args.push(("shipping".to_string(), shipping.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/orders/{}/refunds/calculate.json?{}",
                crate::progenitor_support::encode_path(order_id),
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
     * Retrieves a list of refunds for an order. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/orders/{order_id}/refunds.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#index-2021-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     * * `in_shop_currency: &str` -- Show amounts in the shop currency for the underlying transaction.
     *                     (default: false).
     */
    pub async fn deprecated_202101_get_param_refund(
        &self,
        order_id: &str,
        limit: &str,
        fields: &str,
        in_shop_currency: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !in_shop_currency.is_empty() {
            query_args.push(("in_shop_currency".to_string(), in_shop_currency.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/refunds.json?{}",
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
    * Caution
                For multi-currency orders, the currency property is required whenever the amount property is provided. For more information, see Migrating to support multiple currencies.

              Creates a refund. Use the calculate endpoint to produce the transactions to submit.


                Note
                When you use this endpoint with a Partner development store or a trial store, you can create only five refunds per minute.
    *
    * This function performs a `POST` to the `/admin/api/2021-01/orders/{order_id}/refunds.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#create-2021-01
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `restock_deprecated: &str` -- Whether to add the line items back to the store inventory. Use restock_type for refund line items instead.
    * * `notify: &str` -- Whether to send a refund notification to the customer.
    * * `note: &str` -- An optional note attached to a refund.
    * * `discrepancy_reason: &str` -- An optional comment that explains a discrepancy between calculated and actual refund amounts. Used to populate the reason property of the resulting order adjustment object attached to the refund. Valid values: restock, damage, customer, and other.
    * * `shipping: &str` -- Specify how much shipping to refund. It has the following properties:
    *
    *                           full_refund: Whether to refund all remaining shipping.
    *                           amount: Set a specific amount to refund for shipping. Takes precedence over full_refund.
    * * `refund_line_items: &str` -- A list of line item IDs, quantities to refund, and restock instructions. Each entry has the following properties:
    *
    *                           line_item_id: The ID of a line item to refund.
    *                           quantity: The quantity to refund.
    *                           restock_type:           How this refund line item affects inventory levels. Valid values:
    *
    *               no_restock: Refunding these items won't affect inventory.
    *               cancel: The items have not yet been fulfilled.
    *               The canceled quantity will be added back to the available count.
    *               The number of fulfillable units for this line item will decrease.
    *               return: The items were already delivered but will be returned to the merchant.
    *               The returned quantity will be added back to the available count. The number of fulfillable units for this
    *               line item will remain unchanged.
    *
    *
    *                           location_id:           The ID of the location where the items should be
    *             restocked. This is required when the value of restock_type is return or cancel.
    *             If the item is not already stocked at the location, then
    *             the item is connected to the location. An error is returned when the item is connected to
    *             a
    *             fulfillment service location and a different location is provided.
    * * `transactions: &str` -- A list of transactions
    *             to process as refunds.
    * * `currency: &str` -- The three-letter code (ISO 4217 format) for the currency used for the refund.
    */
    pub async fn deprecated_202101_create_param_refunds(
        &self,
        order_id: &str,
        restock_deprecated: &str,
        notify: &str,
        note: &str,
        discrepancy_reason: &str,
        shipping: &str,
        refund_line_items: &str,
        transactions: &str,
        currency: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        if !discrepancy_reason.is_empty() {
            query_args.push((
                "discrepancy_reason".to_string(),
                discrepancy_reason.to_string(),
            ));
        }
        if !note.is_empty() {
            query_args.push(("note".to_string(), note.to_string()));
        }
        if !notify.is_empty() {
            query_args.push(("notify".to_string(), notify.to_string()));
        }
        if !refund_line_items.is_empty() {
            query_args.push((
                "refund_line_items".to_string(),
                refund_line_items.to_string(),
            ));
        }
        if !restock_deprecated.is_empty() {
            query_args.push((
                "restock
                  deprecated"
                    .to_string(),
                restock_deprecated.to_string(),
            ));
        }
        if !shipping.is_empty() {
            query_args.push(("shipping".to_string(), shipping.to_string()));
        }
        if !transactions.is_empty() {
            query_args.push(("transactions".to_string(), transactions.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/refunds.json?{}",
                crate::progenitor_support::encode_path(order_id),
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
     * Retrieves a specific refund.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/orders/{order_id}/refunds/{refund_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#show-2021-01
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `refund_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     * * `in_shop_currency: &str` -- Show amounts in the shop currency for the underlying transaction.
     *                     (default: false).
     */
    pub async fn deprecated_202101_get_param_refunds_refund(
        &self,
        order_id: &str,
        refund_id: &str,
        fields: &str,
        in_shop_currency: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !in_shop_currency.is_empty() {
            query_args.push(("in_shop_currency".to_string(), in_shop_currency.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/refunds/{}/json?{}",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(refund_id),
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
    * Caution
              For multi-currency orders, the currency property is required whenever the amount property is provided. For more information, see Migrating to support multiple currencies.

              Calculates refund transactions based on line items and shipping. When you want to create a refund,
              you should first use the calculate endpoint to generate accurate refund transactions. Specify the line items
              that are being refunded, their quantity and restock instructions, and whether you intend to refund
              shipping costs. If the restock instructions can't be met—for example, because you try to return more items than have been
              fulfilled—then the endpoint returns modified restock instructions. You can then use the response in the body of the request to create the actual refund.
              The response includes a transactions object with "kind": "suggested_refund",
              which must to be changed to "kind" : "refund" for the refund to be accepted.
    *
    * This function performs a `POST` to the `/admin/api/2021-01/orders/{order_id}/refunds/calculate.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#calculate-2021-01
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `shipping: &str` -- Specify how much shipping to refund. It has the following properties:
    *
    *                           full_refund: Whether to refund all remaining shipping.
    *                           amount: Set a specific amount to refund for shipping. Takes precedence over full_refund.
    * * `refund_line_items: &str` -- A list of line item IDs, quantities to refund, and restock instructions. Each entry has the following properties:
    *
    *                           line_item_id: The ID of a line item to refund.
    *                           quantity: The quantity to refund.
    *                           restock_type:           How this refund line item affects inventory levels. Valid values:
    *
    *               no_restock: Refunding these items won't affect inventory.
    *               cancel: The items have not yet been fulfilled. The canceled quantity will be added
    *               back to the available count. The number of fulfillable units for this line item will decrease.
    *               return: The items were already delivered but will be returned to the merchant.
    *               The returned quantity will be added back to the available count.
    *               The number of fulfillable units for this line item will remain unchanged.
    *
    *
    *                           location_id:           The ID of the location
    *             where the items should be restocked. If location_id is not provided and the value of
    *             restock_type is return or cancel, then the endpoint returns a suitable
    *             location ID.
    *
    *                           already_stocked:           Whether the item is already stocked at
    *             the location. If this is false, then creating the refund will connect the item to the location and start
    *             stocking it there.
    * * `currency: &str` -- The three-letter code (ISO 4217 format) for the
    *             currency used for the refund. Note: Required whenever the shipping amount property is provided.
    */
    pub async fn deprecated_202101_create_param_refunds_calculate(
        &self,
        order_id: &str,
        shipping: &str,
        refund_line_items: &str,
        currency: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        if !refund_line_items.is_empty() {
            query_args.push((
                "refund_line_items".to_string(),
                refund_line_items.to_string(),
            ));
        }
        if !shipping.is_empty() {
            query_args.push(("shipping".to_string(), shipping.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/orders/{}/refunds/calculate.json?{}",
                crate::progenitor_support::encode_path(order_id),
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
     * Retrieves a list of refunds for an order. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/orders/{order_id}/refunds.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#index-unstable
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     * * `in_shop_currency: &str` -- Show amounts in the shop currency for the underlying transaction.
     *                     (default: false).
     */
    pub async fn deprecated_unstable_get_param_refund(
        &self,
        order_id: &str,
        limit: &str,
        fields: &str,
        in_shop_currency: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !in_shop_currency.is_empty() {
            query_args.push(("in_shop_currency".to_string(), in_shop_currency.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/refunds.json?{}",
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
    * Caution
                For multi-currency orders, the currency property is required whenever the amount property is provided. For more information, see Migrating to support multiple currencies.

              Creates a refund. Use the calculate endpoint to produce the transactions to submit.


                Note
                When you use this endpoint with a Partner development store or a trial store, you can create only five refunds per minute.
    *
    * This function performs a `POST` to the `/admin/api/unstable/orders/{order_id}/refunds.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#create-unstable
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `restock_deprecated: &str` -- Whether to add the line items back to the store inventory. Use restock_type for refund line items instead.
    * * `notify: &str` -- Whether to send a refund notification to the customer.
    * * `note: &str` -- An optional note attached to a refund.
    * * `discrepancy_reason: &str` -- An optional comment that explains a discrepancy between calculated and actual refund amounts. Used to populate the reason property of the resulting order adjustment object attached to the refund. Valid values: restock, damage, customer, and other.
    * * `shipping: &str` -- Specify how much shipping to refund. It has the following properties:
    *
    *                           full_refund: Whether to refund all remaining shipping.
    *                           amount: Set a specific amount to refund for shipping. Takes precedence over full_refund.
    * * `refund_line_items: &str` -- A list of line item IDs, quantities to refund, and restock instructions. Each entry has the following properties:
    *
    *                           line_item_id: The ID of a line item to refund.
    *                           quantity: The quantity to refund.
    *                           restock_type:           How this refund line item affects inventory levels. Valid values:
    *
    *               no_restock: Refunding these items won't affect inventory.
    *               cancel: The items have not yet been fulfilled.
    *               The canceled quantity will be added back to the available count.
    *               The number of fulfillable units for this line item will decrease.
    *               return: The items were already delivered but will be returned to the merchant.
    *               The returned quantity will be added back to the available count. The number of fulfillable units for this
    *               line item will remain unchanged.
    *
    *
    *                           location_id:           The ID of the location where the items should be
    *             restocked. This is required when the value of restock_type is return or cancel.
    *             If the item is not already stocked at the location, then
    *             the item is connected to the location. An error is returned when the item is connected to
    *             a
    *             fulfillment service location and a different location is provided.
    * * `transactions: &str` -- A list of transactions
    *             to process as refunds.
    * * `currency: &str` -- The three-letter code (ISO 4217 format) for the currency used for the refund.
    */
    pub async fn deprecated_unstable_create_param_refunds(
        &self,
        order_id: &str,
        restock_deprecated: &str,
        notify: &str,
        note: &str,
        discrepancy_reason: &str,
        shipping: &str,
        refund_line_items: &str,
        transactions: &str,
        currency: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        if !discrepancy_reason.is_empty() {
            query_args.push((
                "discrepancy_reason".to_string(),
                discrepancy_reason.to_string(),
            ));
        }
        if !note.is_empty() {
            query_args.push(("note".to_string(), note.to_string()));
        }
        if !notify.is_empty() {
            query_args.push(("notify".to_string(), notify.to_string()));
        }
        if !refund_line_items.is_empty() {
            query_args.push((
                "refund_line_items".to_string(),
                refund_line_items.to_string(),
            ));
        }
        if !restock_deprecated.is_empty() {
            query_args.push((
                "restock
                  deprecated"
                    .to_string(),
                restock_deprecated.to_string(),
            ));
        }
        if !shipping.is_empty() {
            query_args.push(("shipping".to_string(), shipping.to_string()));
        }
        if !transactions.is_empty() {
            query_args.push(("transactions".to_string(), transactions.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/refunds.json?{}",
                crate::progenitor_support::encode_path(order_id),
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
     * Retrieves a specific refund.
     *
     * This function performs a `GET` to the `/admin/api/unstable/orders/{order_id}/refunds/{refund_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#show-unstable
     *
     * **Parameters:**
     *
     * * `order_id: &str` -- storefront_access_token_id.
     * * `refund_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     * * `in_shop_currency: &str` -- Show amounts in the shop currency for the underlying transaction.
     *                     (default: false).
     */
    pub async fn deprecated_unstable_get_param_refunds_refund(
        &self,
        order_id: &str,
        refund_id: &str,
        fields: &str,
        in_shop_currency: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !in_shop_currency.is_empty() {
            query_args.push(("in_shop_currency".to_string(), in_shop_currency.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/refunds/{}/json?{}",
                crate::progenitor_support::encode_path(order_id),
                crate::progenitor_support::encode_path(refund_id),
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
    * Caution
              For multi-currency orders, the currency property is required whenever the amount property is provided. For more information, see Migrating to support multiple currencies.

              Calculates refund transactions based on line items and shipping. When you want to create a refund,
              you should first use the calculate endpoint to generate accurate refund transactions. Specify the line items
              that are being refunded, their quantity and restock instructions, and whether you intend to refund
              shipping costs. If the restock instructions can't be met—for example, because you try to return more items than have been
              fulfilled—then the endpoint returns modified restock instructions. You can then use the response in the body of the request to create the actual refund.
              The response includes a transactions object with "kind": "suggested_refund",
              which must to be changed to "kind" : "refund" for the refund to be accepted.
    *
    * This function performs a `POST` to the `/admin/api/unstable/orders/{order_id}/refunds/calculate.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/orders/refund#calculate-unstable
    *
    * **Parameters:**
    *
    * * `order_id: &str` -- storefront_access_token_id.
    * * `shipping: &str` -- Specify how much shipping to refund. It has the following properties:
    *
    *                           full_refund: Whether to refund all remaining shipping.
    *                           amount: Set a specific amount to refund for shipping. Takes precedence over full_refund.
    * * `refund_line_items: &str` -- A list of line item IDs, quantities to refund, and restock instructions. Each entry has the following properties:
    *
    *                           line_item_id: The ID of a line item to refund.
    *                           quantity: The quantity to refund.
    *                           restock_type:           How this refund line item affects inventory levels. Valid values:
    *
    *               no_restock: Refunding these items won't affect inventory.
    *               cancel: The items have not yet been fulfilled. The canceled quantity will be added
    *               back to the available count. The number of fulfillable units for this line item will decrease.
    *               return: The items were already delivered but will be returned to the merchant.
    *               The returned quantity will be added back to the available count.
    *               The number of fulfillable units for this line item will remain unchanged.
    *
    *
    *                           location_id:           The ID of the location
    *             where the items should be restocked. If location_id is not provided and the value of
    *             restock_type is return or cancel, then the endpoint returns a suitable
    *             location ID.
    *
    *                           already_stocked:           Whether the item is already stocked at
    *             the location. If this is false, then creating the refund will connect the item to the location and start
    *             stocking it there.
    * * `currency: &str` -- The three-letter code (ISO 4217 format) for the
    *             currency used for the refund. Note: Required whenever the shipping amount property is provided.
    */
    pub async fn deprecated_unstable_create_param_refunds_calculate(
        &self,
        order_id: &str,
        shipping: &str,
        refund_line_items: &str,
        currency: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !currency.is_empty() {
            query_args.push(("currency".to_string(), currency.to_string()));
        }
        if !refund_line_items.is_empty() {
            query_args.push((
                "refund_line_items".to_string(),
                refund_line_items.to_string(),
            ));
        }
        if !shipping.is_empty() {
            query_args.push(("shipping".to_string(), shipping.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/orders/{}/refunds/calculate.json?{}",
                crate::progenitor_support::encode_path(order_id),
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
}
