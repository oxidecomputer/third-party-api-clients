use crate::Client;
use crate::ClientResult;

pub struct Customers {
    pub client: Client,
}

impl Customers {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Customers { client }
    }

    /**
     * Retrieves a list of customers. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/customers.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#index-2020-01
     *
     * **Parameters:**
     *
     * * `ids: &str` -- Restrict results to customers specified by a comma-separated list of IDs.
     * * `since_id: &str` -- Restrict results to those after the specified ID.
     * * `created_at_min: &str` -- Show customers created after a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show customers created before a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show customers last updated after a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show customers last updated before a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get(
        &self,
        ids: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        limit: &str,
        fields: &str,
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
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
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
            &format!("/admin/api/2020-01/customers.json?{}", query_),
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
     * Creates a customer.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/customers.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#create-2020-01
     */
    pub async fn deprecated_202001_create(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-01/customers.json", None);
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
     * Searches for customers that match a supplied query. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/customers/search.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#search-2020-01
     *
     * **Parameters:**
     *
     * * `order: &str` -- Set the field and direction by which to order results.
     *                     (default: last_order_date DESC).
     * * `query: &str` -- Text to search for in the shop's customer data.
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_search(
        &self,
        order: &str,
        query: &str,
        limit: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/customers/search.json?{}", query_),
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
     * Retrieves a single customer.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/customers/{customer_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#show-2020-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_param(
        &self,
        customer_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/customers/{}/json?{}",
                crate::progenitor_support::encode_path(customer_id),
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
     * Updates a customer.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/customers/{customer_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#update-2020-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_param(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/customers/{}/json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Deletes a customer. A customer can't be deleted if they have existing orders.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/customers/{customer_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_param(&self, customer_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/customers/{}/json",
                crate::progenitor_support::encode_path(customer_id),
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
    * Generate an account activation URL for a customer whose account is not yet enabled. This is useful when you've imported a large number of customers and want to send them activation emails all at once. Using this approach, you'll need to generate and send the activation emails yourself.
                The account activation URL generated by this endpoint is for one-time use and will expire after 30 days. If you make a new POST request to this endpoint, then a new URL will be generated. The new URL will be again valid for 30 days, but the previous URL will no longer be valid.
    *
    * This function performs a `POST` to the `/admin/api/2020-01/customers/{customer_id}/account_activation_url.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#account_activation_url-2020-01
    *
    * **Parameters:**
    *
    * * `customer_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202001_create_param_account_activation_url(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/customers/{}/account_activation_url.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Sends an account invite to a customer.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/customers/{customer_id}/send_invite.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#send_invite-2020-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_param_send_invite(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/customers/{}/send_invite.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Retrieves a count of all customers.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/customers/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#count-2020-01
     */
    pub async fn deprecated_202001_get_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-01/customers/count.json", None);
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
     * Retrieves all orders belonging to a customer. The query string parameters that are available to the  Order resource are also available to this endpoint.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/customers/{customer_id}/orders.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#orders-2020-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_param_order(&self, customer_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/customers/{}/orders.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Retrieves a list of customers. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/customers.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#index-2020-04
     *
     * **Parameters:**
     *
     * * `ids: &str` -- Restrict results to customers specified by a comma-separated list of IDs.
     * * `since_id: &str` -- Restrict results to those after the specified ID.
     * * `created_at_min: &str` -- Show customers created after a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show customers created before a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show customers last updated after a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show customers last updated before a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get(
        &self,
        ids: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        limit: &str,
        fields: &str,
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
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
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
            &format!("/admin/api/2020-04/customers.json?{}", query_),
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
     * Creates a customer.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/customers.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#create-2020-04
     */
    pub async fn deprecated_202004_create(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-04/customers.json", None);
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
     * Searches for customers that match a supplied query. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/customers/search.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#search-2020-04
     *
     * **Parameters:**
     *
     * * `order: &str` -- Set the field and direction by which to order results.
     *                     (default: last_order_date DESC).
     * * `query: &str` -- Text to search for in the shop's customer data.
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_search(
        &self,
        order: &str,
        query: &str,
        limit: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/customers/search.json?{}", query_),
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
     * Retrieves a single customer.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/customers/{customer_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#show-2020-04
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_param(
        &self,
        customer_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/customers/{}/json?{}",
                crate::progenitor_support::encode_path(customer_id),
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
     * Updates a customer.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/customers/{customer_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#update-2020-04
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_param(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/customers/{}/json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Deletes a customer. A customer can't be deleted if they have existing orders.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/customers/{customer_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_param(&self, customer_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/customers/{}/json",
                crate::progenitor_support::encode_path(customer_id),
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
    * Generate an account activation URL for a customer whose account is not yet enabled. This is useful when you've imported a large number of customers and want to send them activation emails all at once. Using this approach, you'll need to generate and send the activation emails yourself.
                The account activation URL generated by this endpoint is for one-time use and will expire after 30 days. If you make a new POST request to this endpoint, then a new URL will be generated. The new URL will be again valid for 30 days, but the previous URL will no longer be valid.
    *
    * This function performs a `POST` to the `/admin/api/2020-04/customers/{customer_id}/account_activation_url.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#account_activation_url-2020-04
    *
    * **Parameters:**
    *
    * * `customer_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202004_create_param_account_activation_url(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/customers/{}/account_activation_url.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Sends an account invite to a customer.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/customers/{customer_id}/send_invite.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#send_invite-2020-04
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_create_param_send_invite(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/customers/{}/send_invite.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Retrieves a count of all customers.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/customers/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#count-2020-04
     */
    pub async fn deprecated_202004_get_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-04/customers/count.json", None);
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
     * Retrieves all orders belonging to a customer. The query string parameters that are available to the  Order resource are also available to this endpoint.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/customers/{customer_id}/orders.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#orders-2020-04
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_param_order(&self, customer_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/customers/{}/orders.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Retrieves a list of customers. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/customers.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#index-2020-07
     *
     * **Parameters:**
     *
     * * `ids: &str` -- Restrict results to customers specified by a comma-separated list of IDs.
     * * `since_id: &str` -- Restrict results to those after the specified ID.
     * * `created_at_min: &str` -- Show customers created after a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show customers created before a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show customers last updated after a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show customers last updated before a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get(
        &self,
        ids: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        limit: &str,
        fields: &str,
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
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
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
            &format!("/admin/api/2020-07/customers.json?{}", query_),
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
     * Creates a customer.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/customers.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#create-2020-07
     */
    pub async fn deprecated_202007_create(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-07/customers.json", None);
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
     * Searches for customers that match a supplied query. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/customers/search.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#search-2020-07
     *
     * **Parameters:**
     *
     * * `order: &str` -- Set the field and direction by which to order results.
     *                     (default: last_order_date DESC).
     * * `query: &str` -- Text to search for in the shop's customer data.
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_search(
        &self,
        order: &str,
        query: &str,
        limit: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/customers/search.json?{}", query_),
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
     * Retrieves a single customer.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/customers/{customer_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#show-2020-07
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_param(
        &self,
        customer_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/customers/{}/json?{}",
                crate::progenitor_support::encode_path(customer_id),
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
     * Updates a customer.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/customers/{customer_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#update-2020-07
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_param(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/customers/{}/json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Deletes a customer. A customer can't be deleted if they have existing orders.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/customers/{customer_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_param(&self, customer_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/customers/{}/json",
                crate::progenitor_support::encode_path(customer_id),
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
    * Generate an account activation URL for a customer whose account is not yet enabled. This is useful when you've imported a large number of customers and want to send them activation emails all at once. Using this approach, you'll need to generate and send the activation emails yourself.
                The account activation URL generated by this endpoint is for one-time use and will expire after 30 days. If you make a new POST request to this endpoint, then a new URL will be generated. The new URL will be again valid for 30 days, but the previous URL will no longer be valid.
    *
    * This function performs a `POST` to the `/admin/api/2020-07/customers/{customer_id}/account_activation_url.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#account_activation_url-2020-07
    *
    * **Parameters:**
    *
    * * `customer_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202007_create_param_account_activation_url(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/customers/{}/account_activation_url.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Sends an account invite to a customer.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/customers/{customer_id}/send_invite.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#send_invite-2020-07
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_create_param_send_invite(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/customers/{}/send_invite.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Retrieves a count of all customers.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/customers/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#count-2020-07
     */
    pub async fn deprecated_202007_get_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-07/customers/count.json", None);
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
     * Retrieves all orders belonging to a customer. The query string parameters that are available to the  Order resource are also available to this endpoint.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/customers/{customer_id}/orders.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#orders-2020-07
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_param_order(&self, customer_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/customers/{}/orders.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Retrieves a list of customers. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/customers.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#index-2020-10
     *
     * **Parameters:**
     *
     * * `ids: &str` -- Restrict results to customers specified by a comma-separated list of IDs.
     * * `since_id: &str` -- Restrict results to those after the specified ID.
     * * `created_at_min: &str` -- Show customers created after a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show customers created before a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show customers last updated after a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show customers last updated before a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get(
        &self,
        ids: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        limit: &str,
        fields: &str,
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
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
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
            &format!("/admin/api/2020-10/customers.json?{}", query_),
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
     * Creates a customer.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/customers.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#create-2020-10
     */
    pub async fn create(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-10/customers.json", None);
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
     * Searches for customers that match a supplied query. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/customers/search.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#search-2020-10
     *
     * **Parameters:**
     *
     * * `order: &str` -- Set the field and direction by which to order results.
     *                     (default: last_order_date DESC).
     * * `query: &str` -- Text to search for in the shop's customer data.
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_search(
        &self,
        order: &str,
        query: &str,
        limit: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/customers/search.json?{}", query_),
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
     * Retrieves a single customer.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/customers/{customer_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#show-2020-10
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_param(&self, customer_id: &str, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/customers/{}/json?{}",
                crate::progenitor_support::encode_path(customer_id),
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
     * Updates a customer.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/customers/{customer_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#update-2020-10
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn update_param(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/customers/{}/json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Deletes a customer. A customer can't be deleted if they have existing orders.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/customers/{customer_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_param(&self, customer_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/customers/{}/json",
                crate::progenitor_support::encode_path(customer_id),
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
    * Generate an account activation URL for a customer whose account is not yet enabled. This is useful when you've imported a large number of customers and want to send them activation emails all at once. Using this approach, you'll need to generate and send the activation emails yourself.
                The account activation URL generated by this endpoint is for one-time use and will expire after 30 days. If you make a new POST request to this endpoint, then a new URL will be generated. The new URL will be again valid for 30 days, but the previous URL will no longer be valid.
    *
    * This function performs a `POST` to the `/admin/api/2020-10/customers/{customer_id}/account_activation_url.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#account_activation_url-2020-10
    *
    * **Parameters:**
    *
    * * `customer_id: &str` -- storefront_access_token_id.
    */
    pub async fn create_param_account_activation_url(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/customers/{}/account_activation_url.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Sends an account invite to a customer.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/customers/{customer_id}/send_invite.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#send_invite-2020-10
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn create_param_send_invite(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/customers/{}/send_invite.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Retrieves a count of all customers.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/customers/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#count-2020-10
     */
    pub async fn get_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-10/customers/count.json", None);
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
     * Retrieves all orders belonging to a customer. The query string parameters that are available to the  Order resource are also available to this endpoint.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/customers/{customer_id}/orders.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#orders-2020-10
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn get_param_order(&self, customer_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/customers/{}/orders.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Retrieves a list of customers. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/customers.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#index-2021-01
     *
     * **Parameters:**
     *
     * * `ids: &str` -- Restrict results to customers specified by a comma-separated list of IDs.
     * * `since_id: &str` -- Restrict results to those after the specified ID.
     * * `created_at_min: &str` -- Show customers created after a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show customers created before a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show customers last updated after a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show customers last updated before a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get(
        &self,
        ids: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        limit: &str,
        fields: &str,
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
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
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
            &format!("/admin/api/2021-01/customers.json?{}", query_),
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
     * Creates a customer.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/customers.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#create-2021-01
     */
    pub async fn deprecated_202101_create(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2021-01/customers.json", None);
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
     * Searches for customers that match a supplied query. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/customers/search.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#search-2021-01
     *
     * **Parameters:**
     *
     * * `order: &str` -- Set the field and direction by which to order results.
     *                     (default: last_order_date DESC).
     * * `query: &str` -- Text to search for in the shop's customer data.
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_search(
        &self,
        order: &str,
        query: &str,
        limit: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/customers/search.json?{}", query_),
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
     * Retrieves a single customer.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/customers/{customer_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#show-2021-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_param(
        &self,
        customer_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/customers/{}/json?{}",
                crate::progenitor_support::encode_path(customer_id),
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
     * Updates a customer.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/customers/{customer_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#update-2021-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_param(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/customers/{}/json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Deletes a customer. A customer can't be deleted if they have existing orders.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/customers/{customer_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_param(&self, customer_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/customers/{}/json",
                crate::progenitor_support::encode_path(customer_id),
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
    * Generate an account activation URL for a customer whose account is not yet enabled. This is useful when you've imported a large number of customers and want to send them activation emails all at once. Using this approach, you'll need to generate and send the activation emails yourself.
                The account activation URL generated by this endpoint is for one-time use and will expire after 30 days. If you make a new POST request to this endpoint, then a new URL will be generated. The new URL will be again valid for 30 days, but the previous URL will no longer be valid.
    *
    * This function performs a `POST` to the `/admin/api/2021-01/customers/{customer_id}/account_activation_url.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#account_activation_url-2021-01
    *
    * **Parameters:**
    *
    * * `customer_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202101_create_param_account_activation_url(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/customers/{}/account_activation_url.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Sends an account invite to a customer.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/customers/{customer_id}/send_invite.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#send_invite-2021-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_create_param_send_invite(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/customers/{}/send_invite.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Retrieves a count of all customers.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/customers/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#count-2021-01
     */
    pub async fn deprecated_202101_get_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2021-01/customers/count.json", None);
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
     * Retrieves all orders belonging to a customer. The query string parameters that are available to the  Order resource are also available to this endpoint.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/customers/{customer_id}/orders.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#orders-2021-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_param_order(&self, customer_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/customers/{}/orders.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Retrieves a list of customers. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/customers.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#index-unstable
     *
     * **Parameters:**
     *
     * * `ids: &str` -- Restrict results to customers specified by a comma-separated list of IDs.
     * * `since_id: &str` -- Restrict results to those after the specified ID.
     * * `created_at_min: &str` -- Show customers created after a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show customers created before a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show customers last updated after a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show customers last updated before a specified date.(format: 2014-04-25T16:15:47-04:00).
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get(
        &self,
        ids: &str,
        since_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        limit: &str,
        fields: &str,
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
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
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
            &format!("/admin/api/unstable/customers.json?{}", query_),
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
     * Creates a customer.
     *
     * This function performs a `POST` to the `/admin/api/unstable/customers.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#create-unstable
     */
    pub async fn deprecated_unstable_create(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/unstable/customers.json", None);
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
     * Searches for customers that match a supplied query. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/customers/search.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#search-unstable
     *
     * **Parameters:**
     *
     * * `order: &str` -- Set the field and direction by which to order results.
     *                     (default: last_order_date DESC).
     * * `query: &str` -- Text to search for in the shop's customer data.
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_search(
        &self,
        order: &str,
        query: &str,
        limit: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/customers/search.json?{}", query_),
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
     * Retrieves a single customer.
     *
     * This function performs a `GET` to the `/admin/api/unstable/customers/{customer_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#show-unstable
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_param(
        &self,
        customer_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/customers/{}/json?{}",
                crate::progenitor_support::encode_path(customer_id),
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
     * Updates a customer.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/customers/{customer_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#update-unstable
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_param(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/customers/{}/json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Deletes a customer. A customer can't be deleted if they have existing orders.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/customers/{customer_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#destroy-unstable
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_param(&self, customer_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/customers/{}/json",
                crate::progenitor_support::encode_path(customer_id),
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
    * Generate an account activation URL for a customer whose account is not yet enabled. This is useful when you've imported a large number of customers and want to send them activation emails all at once. Using this approach, you'll need to generate and send the activation emails yourself.
                The account activation URL generated by this endpoint is for one-time use and will expire after 30 days. If you make a new POST request to this endpoint, then a new URL will be generated. The new URL will be again valid for 30 days, but the previous URL will no longer be valid.
    *
    * This function performs a `POST` to the `/admin/api/unstable/customers/{customer_id}/account_activation_url.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#account_activation_url-unstable
    *
    * **Parameters:**
    *
    * * `customer_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_unstable_create_param_account_activation_url(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/customers/{}/account_activation_url.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Sends an account invite to a customer.
     *
     * This function performs a `POST` to the `/admin/api/unstable/customers/{customer_id}/send_invite.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#send_invite-unstable
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_create_param_send_invite(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/customers/{}/send_invite.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Retrieves a count of all customers.
     *
     * This function performs a `GET` to the `/admin/api/unstable/customers/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#count-unstable
     */
    pub async fn deprecated_unstable_get_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/unstable/customers/count.json", None);
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
     * Retrieves all orders belonging to a customer. The query string parameters that are available to the  Order resource are also available to this endpoint.
     *
     * This function performs a `GET` to the `/admin/api/unstable/customers/{customer_id}/orders.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer#orders-unstable
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_param_order(&self, customer_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/customers/{}/orders.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Retrieves a list of addresses for a customer. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/customers/{customer_id}/addresses.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#index-2020-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_param_addresse(
        &self,
        customer_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/customers/{}/addresses.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Creates a new address for a customer.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/customers/{customer_id}/addresses.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#create-2020-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_param_addresses(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/customers/{}/addresses.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Retrieves details a single customer address.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/customers/{customer_id}/addresses/{address_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#show-2020-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_param_addresses_address(
        &self,
        customer_id: &str,
        address_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/customers/{}/addresses/{}/json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
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
     * Updates an existing customer address.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/customers/{customer_id}/addresses/{address_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#update-2020-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_param_addresses_address(
        &self,
        customer_id: &str,
        address_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/customers/{}/addresses/{}/json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
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
     * Removes an address from a customer’s address list.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/customers/{customer_id}/addresses/{address_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_param_addresses_address(
        &self,
        customer_id: &str,
        address_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/customers/{}/addresses/{}/json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
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
     * Performs bulk operations for multiple customer addresses.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/customers/{customer_id}/addresses/set.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#set-2020-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_ids: i64` -- recurring_application_charge[capped_amount].
     * * `operation: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_param_addresses_set(
        &self,
        customer_id: &str,
        address_ids: i64,
        operation: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if address_ids > 0 {
            query_args.push(("address_ids[]".to_string(), address_ids.to_string()));
        }
        if !operation.is_empty() {
            query_args.push(("operation".to_string(), operation.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/customers/{}/addresses/set.json?{}",
                crate::progenitor_support::encode_path(customer_id),
                query_
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Sets the default address for a customer.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/customers/{customer_id}/addresses/{address_id}/default.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#default-2020-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_param_addresses_address_default(
        &self,
        customer_id: &str,
        address_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/customers/{}/addresses/{}/default.json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of addresses for a customer. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/customers/{customer_id}/addresses.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#index-2020-04
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_param_addresse(
        &self,
        customer_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/customers/{}/addresses.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Creates a new address for a customer.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/customers/{customer_id}/addresses.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#create-2020-04
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_create_param_addresses(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/customers/{}/addresses.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Retrieves details a single customer address.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/customers/{customer_id}/addresses/{address_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#show-2020-04
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_param_addresses_address(
        &self,
        customer_id: &str,
        address_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/customers/{}/addresses/{}/json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
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
     * Updates an existing customer address.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/customers/{customer_id}/addresses/{address_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#update-2020-04
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_param_addresses_address(
        &self,
        customer_id: &str,
        address_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/customers/{}/addresses/{}/json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
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
     * Removes an address from a customer’s address list.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/customers/{customer_id}/addresses/{address_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_param_addresses_address(
        &self,
        customer_id: &str,
        address_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/customers/{}/addresses/{}/json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
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
     * Performs bulk operations for multiple customer addresses.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/customers/{customer_id}/addresses/set.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#set-2020-04
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_ids: i64` -- recurring_application_charge[capped_amount].
     * * `operation: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_param_addresses_set(
        &self,
        customer_id: &str,
        address_ids: i64,
        operation: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if address_ids > 0 {
            query_args.push(("address_ids[]".to_string(), address_ids.to_string()));
        }
        if !operation.is_empty() {
            query_args.push(("operation".to_string(), operation.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/customers/{}/addresses/set.json?{}",
                crate::progenitor_support::encode_path(customer_id),
                query_
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Sets the default address for a customer.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/customers/{customer_id}/addresses/{address_id}/default.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#default-2020-04
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_param_addresses_address_default(
        &self,
        customer_id: &str,
        address_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/customers/{}/addresses/{}/default.json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of addresses for a customer. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/customers/{customer_id}/addresses.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#index-2020-07
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_param_addresse(
        &self,
        customer_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/customers/{}/addresses.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Creates a new address for a customer.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/customers/{customer_id}/addresses.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#create-2020-07
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_create_param_addresses(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/customers/{}/addresses.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Retrieves details a single customer address.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/customers/{customer_id}/addresses/{address_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#show-2020-07
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_param_addresses_address(
        &self,
        customer_id: &str,
        address_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/customers/{}/addresses/{}/json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
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
     * Updates an existing customer address.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/customers/{customer_id}/addresses/{address_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#update-2020-07
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_param_addresses_address(
        &self,
        customer_id: &str,
        address_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/customers/{}/addresses/{}/json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
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
     * Removes an address from a customer’s address list.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/customers/{customer_id}/addresses/{address_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_param_addresses_address(
        &self,
        customer_id: &str,
        address_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/customers/{}/addresses/{}/json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
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
     * Performs bulk operations for multiple customer addresses.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/customers/{customer_id}/addresses/set.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#set-2020-07
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_ids: i64` -- recurring_application_charge[capped_amount].
     * * `operation: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_param_addresses_set(
        &self,
        customer_id: &str,
        address_ids: i64,
        operation: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if address_ids > 0 {
            query_args.push(("address_ids[]".to_string(), address_ids.to_string()));
        }
        if !operation.is_empty() {
            query_args.push(("operation".to_string(), operation.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/customers/{}/addresses/set.json?{}",
                crate::progenitor_support::encode_path(customer_id),
                query_
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Sets the default address for a customer.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/customers/{customer_id}/addresses/{address_id}/default.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#default-2020-07
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_param_addresses_address_default(
        &self,
        customer_id: &str,
        address_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/customers/{}/addresses/{}/default.json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of addresses for a customer. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/customers/{customer_id}/addresses.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#index-2020-10
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn get_param_addresse(&self, customer_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/customers/{}/addresses.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Creates a new address for a customer.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/customers/{customer_id}/addresses.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#create-2020-10
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn create_param_addresses(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/customers/{}/addresses.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Retrieves details a single customer address.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/customers/{customer_id}/addresses/{address_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#show-2020-10
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn get_param_addresses_address(
        &self,
        customer_id: &str,
        address_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/customers/{}/addresses/{}/json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
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
     * Updates an existing customer address.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/customers/{customer_id}/addresses/{address_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#update-2020-10
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn update_param_addresses_address(
        &self,
        customer_id: &str,
        address_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/customers/{}/addresses/{}/json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
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
     * Removes an address from a customer’s address list.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/customers/{customer_id}/addresses/{address_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_param_addresses_address(
        &self,
        customer_id: &str,
        address_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/customers/{}/addresses/{}/json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
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
     * Performs bulk operations for multiple customer addresses.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/customers/{customer_id}/addresses/set.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#set-2020-10
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_ids: i64` -- recurring_application_charge[capped_amount].
     * * `operation: &str` -- storefront_access_token_id.
     */
    pub async fn update_param_addresses_set(
        &self,
        customer_id: &str,
        address_ids: i64,
        operation: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if address_ids > 0 {
            query_args.push(("address_ids[]".to_string(), address_ids.to_string()));
        }
        if !operation.is_empty() {
            query_args.push(("operation".to_string(), operation.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/customers/{}/addresses/set.json?{}",
                crate::progenitor_support::encode_path(customer_id),
                query_
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Sets the default address for a customer.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/customers/{customer_id}/addresses/{address_id}/default.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#default-2020-10
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn update_param_addresses_address_default(
        &self,
        customer_id: &str,
        address_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/customers/{}/addresses/{}/default.json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of addresses for a customer. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/customers/{customer_id}/addresses.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#index-2021-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_param_addresse(
        &self,
        customer_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/customers/{}/addresses.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Creates a new address for a customer.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/customers/{customer_id}/addresses.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#create-2021-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_create_param_addresses(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/customers/{}/addresses.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Retrieves details a single customer address.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/customers/{customer_id}/addresses/{address_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#show-2021-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_param_addresses_address(
        &self,
        customer_id: &str,
        address_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/customers/{}/addresses/{}/json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
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
     * Updates an existing customer address.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/customers/{customer_id}/addresses/{address_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#update-2021-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_param_addresses_address(
        &self,
        customer_id: &str,
        address_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/customers/{}/addresses/{}/json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
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
     * Removes an address from a customer’s address list.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/customers/{customer_id}/addresses/{address_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_param_addresses_address(
        &self,
        customer_id: &str,
        address_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/customers/{}/addresses/{}/json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
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
     * Performs bulk operations for multiple customer addresses.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/customers/{customer_id}/addresses/set.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#set-2021-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_ids: i64` -- recurring_application_charge[capped_amount].
     * * `operation: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_param_addresses_set(
        &self,
        customer_id: &str,
        address_ids: i64,
        operation: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if address_ids > 0 {
            query_args.push(("address_ids[]".to_string(), address_ids.to_string()));
        }
        if !operation.is_empty() {
            query_args.push(("operation".to_string(), operation.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/customers/{}/addresses/set.json?{}",
                crate::progenitor_support::encode_path(customer_id),
                query_
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Sets the default address for a customer.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/customers/{customer_id}/addresses/{address_id}/default.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#default-2021-01
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_param_addresses_address_default(
        &self,
        customer_id: &str,
        address_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/customers/{}/addresses/{}/default.json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of addresses for a customer. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/customers/{customer_id}/addresses.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#index-unstable
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_param_addresse(
        &self,
        customer_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/customers/{}/addresses.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Creates a new address for a customer.
     *
     * This function performs a `POST` to the `/admin/api/unstable/customers/{customer_id}/addresses.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#create-unstable
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_create_param_addresses(
        &self,
        customer_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/customers/{}/addresses.json",
                crate::progenitor_support::encode_path(customer_id),
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
     * Retrieves details a single customer address.
     *
     * This function performs a `GET` to the `/admin/api/unstable/customers/{customer_id}/addresses/{address_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#show-unstable
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_param_addresses_address(
        &self,
        customer_id: &str,
        address_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/customers/{}/addresses/{}/json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
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
     * Updates an existing customer address.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/customers/{customer_id}/addresses/{address_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#update-unstable
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_param_addresses_address(
        &self,
        customer_id: &str,
        address_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/customers/{}/addresses/{}/json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
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
     * Removes an address from a customer’s address list.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/customers/{customer_id}/addresses/{address_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#destroy-unstable
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_param_addresses_address(
        &self,
        customer_id: &str,
        address_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/customers/{}/addresses/{}/json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
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
     * Performs bulk operations for multiple customer addresses.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/customers/{customer_id}/addresses/set.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#set-unstable
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_ids: i64` -- recurring_application_charge[capped_amount].
     * * `operation: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_param_addresses_set(
        &self,
        customer_id: &str,
        address_ids: i64,
        operation: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if address_ids > 0 {
            query_args.push(("address_ids[]".to_string(), address_ids.to_string()));
        }
        if !operation.is_empty() {
            query_args.push(("operation".to_string(), operation.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/customers/{}/addresses/set.json?{}",
                crate::progenitor_support::encode_path(customer_id),
                query_
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Sets the default address for a customer.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/customers/{customer_id}/addresses/{address_id}/default.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customer-address#default-unstable
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- storefront_access_token_id.
     * * `address_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_param_addresses_address_default(
        &self,
        customer_id: &str,
        address_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/customers/{}/addresses/{}/default.json",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(address_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of customer saved searches. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/customer_saved_searches.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#index-2020-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_saved_searche(
        &self,
        limit: &str,
        since_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/customer_saved_searches.json?{}", query_),
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
     * Creates a customer saved search.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/customer_saved_searches.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#create-2020-01
     */
    pub async fn deprecated_202001_create_saved_searches(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-01/customer_saved_searches.json", None);
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
     * Retrieves a count of all customer saved searches.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/customer_saved_searches/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#count-2020-01
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     */
    pub async fn deprecated_202001_get_saved_searches_count(
        &self,
        since_id: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/customer_saved_searches/count.json?{}",
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
     * Retrieves a single customer saved search.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/customer_saved_searches/{customer_saved_search_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#show-2020-01
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_saved_searches_param_search(
        &self,
        customer_saved_search_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/customer_saved_searches/{}/json?{}",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Updates a customer saved search.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/customer_saved_searches/{customer_saved_search_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#update-2020-01
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_saved_searches_param_search(
        &self,
        customer_saved_search_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/customer_saved_searches/{}/json",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Deletes a customer saved search.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/customer_saved_searches/{customer_saved_search_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_saved_searches_param_search(
        &self,
        customer_saved_search_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/customer_saved_searches/{}/json",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Retrieves all customers returned by a customer saved search.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/customer_saved_searches/{customer_saved_search_id}/customers.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#other-2020-01
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     * * `order: &str` -- Set the field and direction by which to order results.
     *                     (default: last_order_date DESC).
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_saved_searches_param_search_customers(
        &self,
        customer_saved_search_id: &str,
        order: &str,
        limit: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/customer_saved_searches/{}/customers.json?{}",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Retrieves a list of customer saved searches. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/customer_saved_searches.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#index-2020-04
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_saved_searche(
        &self,
        limit: &str,
        since_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/customer_saved_searches.json?{}", query_),
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
     * Creates a customer saved search.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/customer_saved_searches.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#create-2020-04
     */
    pub async fn deprecated_202004_create_saved_searches(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-04/customer_saved_searches.json", None);
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
     * Retrieves a count of all customer saved searches.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/customer_saved_searches/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#count-2020-04
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     */
    pub async fn deprecated_202004_get_saved_searches_count(
        &self,
        since_id: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/customer_saved_searches/count.json?{}",
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
     * Retrieves a single customer saved search.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/customer_saved_searches/{customer_saved_search_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#show-2020-04
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_saved_searches_param_search(
        &self,
        customer_saved_search_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/customer_saved_searches/{}/json?{}",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Updates a customer saved search.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/customer_saved_searches/{customer_saved_search_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#update-2020-04
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_saved_searches_param_search(
        &self,
        customer_saved_search_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/customer_saved_searches/{}/json",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Deletes a customer saved search.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/customer_saved_searches/{customer_saved_search_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_saved_searches_param_search(
        &self,
        customer_saved_search_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/customer_saved_searches/{}/json",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Retrieves all customers returned by a customer saved search.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/customer_saved_searches/{customer_saved_search_id}/customers.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#other-2020-04
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     * * `order: &str` -- Set the field and direction by which to order results.
     *                     (default: last_order_date DESC).
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_saved_searches_param_search_customers(
        &self,
        customer_saved_search_id: &str,
        order: &str,
        limit: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/customer_saved_searches/{}/customers.json?{}",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Retrieves a list of customer saved searches. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/customer_saved_searches.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#index-2020-07
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_saved_searche(
        &self,
        limit: &str,
        since_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/customer_saved_searches.json?{}", query_),
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
     * Creates a customer saved search.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/customer_saved_searches.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#create-2020-07
     */
    pub async fn deprecated_202007_create_saved_searches(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-07/customer_saved_searches.json", None);
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
     * Retrieves a count of all customer saved searches.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/customer_saved_searches/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#count-2020-07
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     */
    pub async fn deprecated_202007_get_saved_searches_count(
        &self,
        since_id: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/customer_saved_searches/count.json?{}",
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
     * Retrieves a single customer saved search.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/customer_saved_searches/{customer_saved_search_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#show-2020-07
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_saved_searches_param_search(
        &self,
        customer_saved_search_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/customer_saved_searches/{}/json?{}",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Updates a customer saved search.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/customer_saved_searches/{customer_saved_search_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#update-2020-07
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_saved_searches_param_search(
        &self,
        customer_saved_search_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/customer_saved_searches/{}/json",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Deletes a customer saved search.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/customer_saved_searches/{customer_saved_search_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_saved_searches_param_search(
        &self,
        customer_saved_search_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/customer_saved_searches/{}/json",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Retrieves all customers returned by a customer saved search.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/customer_saved_searches/{customer_saved_search_id}/customers.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#other-2020-07
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     * * `order: &str` -- Set the field and direction by which to order results.
     *                     (default: last_order_date DESC).
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_saved_searches_param_search_customers(
        &self,
        customer_saved_search_id: &str,
        order: &str,
        limit: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/customer_saved_searches/{}/customers.json?{}",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Retrieves a list of customer saved searches. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/customer_saved_searches.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#index-2020-10
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_saved_searche(
        &self,
        limit: &str,
        since_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/customer_saved_searches.json?{}", query_),
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
     * Creates a customer saved search.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/customer_saved_searches.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#create-2020-10
     */
    pub async fn create_saved_searches(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-10/customer_saved_searches.json", None);
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
     * Retrieves a count of all customer saved searches.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/customer_saved_searches/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#count-2020-10
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     */
    pub async fn get_saved_searches_count(&self, since_id: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/customer_saved_searches/count.json?{}",
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
     * Retrieves a single customer saved search.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/customer_saved_searches/{customer_saved_search_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#show-2020-10
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_saved_searches_param_search(
        &self,
        customer_saved_search_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/customer_saved_searches/{}/json?{}",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Updates a customer saved search.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/customer_saved_searches/{customer_saved_search_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#update-2020-10
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     */
    pub async fn update_saved_searches_param_search(
        &self,
        customer_saved_search_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/customer_saved_searches/{}/json",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Deletes a customer saved search.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/customer_saved_searches/{customer_saved_search_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_saved_searches_param_search(
        &self,
        customer_saved_search_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/customer_saved_searches/{}/json",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Retrieves all customers returned by a customer saved search.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/customer_saved_searches/{customer_saved_search_id}/customers.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#other-2020-10
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     * * `order: &str` -- Set the field and direction by which to order results.
     *                     (default: last_order_date DESC).
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_saved_searches_param_search_customers(
        &self,
        customer_saved_search_id: &str,
        order: &str,
        limit: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/customer_saved_searches/{}/customers.json?{}",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Retrieves a list of customer saved searches. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/customer_saved_searches.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#index-2021-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_saved_searche(
        &self,
        limit: &str,
        since_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/customer_saved_searches.json?{}", query_),
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
     * Creates a customer saved search.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/customer_saved_searches.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#create-2021-01
     */
    pub async fn deprecated_202101_create_saved_searches(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2021-01/customer_saved_searches.json", None);
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
     * Retrieves a count of all customer saved searches.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/customer_saved_searches/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#count-2021-01
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     */
    pub async fn deprecated_202101_get_saved_searches_count(
        &self,
        since_id: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/customer_saved_searches/count.json?{}",
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
     * Retrieves a single customer saved search.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/customer_saved_searches/{customer_saved_search_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#show-2021-01
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_saved_searches_param_search(
        &self,
        customer_saved_search_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/customer_saved_searches/{}/json?{}",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Updates a customer saved search.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/customer_saved_searches/{customer_saved_search_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#update-2021-01
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_saved_searches_param_search(
        &self,
        customer_saved_search_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/customer_saved_searches/{}/json",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Deletes a customer saved search.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/customer_saved_searches/{customer_saved_search_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_saved_searches_param_search(
        &self,
        customer_saved_search_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/customer_saved_searches/{}/json",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Retrieves all customers returned by a customer saved search.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/customer_saved_searches/{customer_saved_search_id}/customers.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#other-2021-01
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     * * `order: &str` -- Set the field and direction by which to order results.
     *                     (default: last_order_date DESC).
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_saved_searches_param_search_customers(
        &self,
        customer_saved_search_id: &str,
        order: &str,
        limit: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/customer_saved_searches/{}/customers.json?{}",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Retrieves a list of customer saved searches. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/customer_saved_searches.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#index-unstable
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_saved_searche(
        &self,
        limit: &str,
        since_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/customer_saved_searches.json?{}",
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
     * Creates a customer saved search.
     *
     * This function performs a `POST` to the `/admin/api/unstable/customer_saved_searches.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#create-unstable
     */
    pub async fn deprecated_unstable_create_saved_searches(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/unstable/customer_saved_searches.json", None);
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
     * Retrieves a count of all customer saved searches.
     *
     * This function performs a `GET` to the `/admin/api/unstable/customer_saved_searches/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#count-unstable
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     */
    pub async fn deprecated_unstable_get_saved_searches_count(
        &self,
        since_id: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/customer_saved_searches/count.json?{}",
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
     * Retrieves a single customer saved search.
     *
     * This function performs a `GET` to the `/admin/api/unstable/customer_saved_searches/{customer_saved_search_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#show-unstable
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_saved_searches_param_search(
        &self,
        customer_saved_search_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/customer_saved_searches/{}/json?{}",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Updates a customer saved search.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/customer_saved_searches/{customer_saved_search_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#update-unstable
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_saved_searches_param_search(
        &self,
        customer_saved_search_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/customer_saved_searches/{}/json",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Deletes a customer saved search.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/customer_saved_searches/{customer_saved_search_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#destroy-unstable
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_saved_searches_param_search(
        &self,
        customer_saved_search_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/customer_saved_searches/{}/json",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
     * Retrieves all customers returned by a customer saved search.
     *
     * This function performs a `GET` to the `/admin/api/unstable/customer_saved_searches/{customer_saved_search_id}/customers.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/customers/customersavedsearch#other-unstable
     *
     * **Parameters:**
     *
     * * `customer_saved_search_id: &str` -- storefront_access_token_id.
     * * `order: &str` -- Set the field and direction by which to order results.
     *                     (default: last_order_date DESC).
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_saved_searches_param_search_customers(
        &self,
        customer_saved_search_id: &str,
        order: &str,
        limit: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/customer_saved_searches/{}/customers.json?{}",
                crate::progenitor_support::encode_path(customer_saved_search_id),
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
}
