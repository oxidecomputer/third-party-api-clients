use anyhow::Result;

use crate::Client;

pub struct Ecommerce {
    pub client: Client,
}

impl Ecommerce {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Ecommerce { client }
    }

    /**
    * List account orders.
    *
    * This function performs a `GET` to the `/ecommerce/orders` endpoint.
    *
    * Get information about an account's orders.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `campaign_id: &str` -- Restrict results to orders with a specific `campaign_id` value.
    * * `outreach_id: &str` -- Restrict results to orders with a specific `outreach_id` value.
    * * `customer_id: &str` -- Restrict results to orders made by a specific customer.
    * * `has_outreach: bool` -- Restrict results to orders that have an outreach attached. For example, an email campaign or Facebook ad.
    */
    pub async fn get_order(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        campaign_id: &str,
        outreach_id: &str,
        customer_id: &str,
        has_outreach: bool,
    ) -> Result<crate::types::OrdersData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !campaign_id.is_empty() {
            query_args.push(("campaign_id".to_string(), campaign_id.to_string()));
        }
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !customer_id.is_empty() {
            query_args.push(("customer_id".to_string(), customer_id.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if has_outreach {
            query_args.push(("has_outreach".to_string(), has_outreach.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !outreach_id.is_empty() {
            query_args.push(("outreach_id".to_string(), outreach_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/ecommerce/orders?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * List stores.
    *
    * This function performs a `GET` to the `/ecommerce/stores` endpoint.
    *
    * Get information about all stores in the account.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    */
    pub async fn get_store(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
    ) -> Result<crate::types::ECommerceStores> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/ecommerce/stores?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * Add store.
    *
    * This function performs a `POST` to the `/ecommerce/stores` endpoint.
    *
    * Add a new store to your Mailchimp account.
    */
    pub async fn post_store(
        &self,
        body: &crate::types::ECommerceStore,
    ) -> Result<crate::types::Stores> {
        let url = "/ecommerce/stores".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get store info.
    *
    * This function performs a `GET` to the `/ecommerce/stores/{store_id}` endpoint.
    *
    * Get information about a specific store.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `store_id: &str` -- The name of the folder.
    */
    pub async fn get_store_ecommerce(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        store_id: &str,
    ) -> Result<crate::types::Stores> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ecommerce/stores/{}?{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete store.
    *
    * This function performs a `DELETE` to the `/ecommerce/stores/{store_id}` endpoint.
    *
    * Delete a store. Deleting a store will also delete any associated subresources, including Customers, Orders, Products, and Carts.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    */
    pub async fn delete_stores(&self, store_id: &str) -> Result<()> {
        let url = format!(
            "/ecommerce/stores/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update store.
    *
    * This function performs a `PATCH` to the `/ecommerce/stores/{store_id}` endpoint.
    *
    * Update a store.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    */
    pub async fn patch_stores(
        &self,
        store_id: &str,
        body: &crate::types::ECommerceStoreData,
    ) -> Result<crate::types::Stores> {
        let url = format!(
            "/ecommerce/stores/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List carts.
    *
    * This function performs a `GET` to the `/ecommerce/stores/{store_id}/carts` endpoint.
    *
    * Get information about a store's carts.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `store_id: &str` -- The name of the folder.
    */
    pub async fn get_stores_cart(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        store_id: &str,
    ) -> Result<crate::types::CartsData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ecommerce/stores/{}/carts?{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add cart.
    *
    * This function performs a `POST` to the `/ecommerce/stores/{store_id}/carts` endpoint.
    *
    * Add a new cart to a store.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    */
    pub async fn post_stores_cart(
        &self,
        store_id: &str,
        body: &crate::types::ECommerceCart,
    ) -> Result<crate::types::Carts> {
        let url = format!(
            "/ecommerce/stores/{}/carts",
            crate::progenitor_support::encode_path(&store_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get cart info.
    *
    * This function performs a `GET` to the `/ecommerce/stores/{store_id}/carts/{cart_id}` endpoint.
    *
    * Get information about a specific cart.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `store_id: &str` -- The name of the folder.
    * * `cart_id: &str` -- The name of the folder.
    */
    pub async fn get_stores_cart_ecommerce(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        store_id: &str,
        cart_id: &str,
    ) -> Result<crate::types::Carts> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ecommerce/stores/{}/carts/{}?{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&cart_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete cart.
    *
    * This function performs a `DELETE` to the `/ecommerce/stores/{store_id}/carts/{cart_id}` endpoint.
    *
    * Delete a cart.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `cart_id: &str` -- The name of the folder.
    */
    pub async fn delete_stores_carts(&self, store_id: &str, cart_id: &str) -> Result<()> {
        let url = format!(
            "/ecommerce/stores/{}/carts/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&cart_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update cart.
    *
    * This function performs a `PATCH` to the `/ecommerce/stores/{store_id}/carts/{cart_id}` endpoint.
    *
    * Update a specific cart.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `cart_id: &str` -- The name of the folder.
    */
    pub async fn patch_stores_carts(
        &self,
        store_id: &str,
        cart_id: &str,
        body: &crate::types::ECommerceCartData,
    ) -> Result<crate::types::Carts> {
        let url = format!(
            "/ecommerce/stores/{}/carts/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&cart_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List cart line items.
    *
    * This function performs a `GET` to the `/ecommerce/stores/{store_id}/carts/{cart_id}/lines` endpoint.
    *
    * Get information about a cart's line items.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `store_id: &str` -- The name of the folder.
    * * `cart_id: &str` -- The name of the folder.
    */
    pub async fn get_stores_carts_line(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        store_id: &str,
        cart_id: &str,
    ) -> Result<crate::types::CartLines> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ecommerce/stores/{}/carts/{}/lines?{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&cart_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add cart line item.
    *
    * This function performs a `POST` to the `/ecommerce/stores/{store_id}/carts/{cart_id}/lines` endpoint.
    *
    * Add a new line item to an existing cart.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `cart_id: &str` -- The name of the folder.
    */
    pub async fn post_stores_carts_line(
        &self,
        store_id: &str,
        cart_id: &str,
        body: &crate::types::ECommerceCartLineItemData,
    ) -> Result<crate::types::ECommerceCartLineItem> {
        let url = format!(
            "/ecommerce/stores/{}/carts/{}/lines",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&cart_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get cart line item.
    *
    * This function performs a `GET` to the `/ecommerce/stores/{store_id}/carts/{cart_id}/lines/{line_id}` endpoint.
    *
    * Get information about a specific cart line item.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `store_id: &str` -- The name of the folder.
    * * `cart_id: &str` -- The name of the folder.
    * * `line_id: &str` -- The id for the line item of a cart.
    */
    pub async fn get_stores_carts_line_ecommerce(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        store_id: &str,
        cart_id: &str,
        line_id: &str,
    ) -> Result<crate::types::ECommerceCartLineItem> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ecommerce/stores/{}/carts/{}/lines/{}?{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&cart_id.to_string()),
            crate::progenitor_support::encode_path(&line_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete cart line item.
    *
    * This function performs a `DELETE` to the `/ecommerce/stores/{store_id}/carts/{cart_id}/lines/{line_id}` endpoint.
    *
    * Delete a specific cart line item.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `cart_id: &str` -- The name of the folder.
    * * `line_id: &str` -- The id for the line item of a cart.
    */
    pub async fn delete_stores_carts_lines(
        &self,
        store_id: &str,
        cart_id: &str,
        line_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/ecommerce/stores/{}/carts/{}/lines/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&cart_id.to_string()),
            crate::progenitor_support::encode_path(&line_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update cart line item.
    *
    * This function performs a `PATCH` to the `/ecommerce/stores/{store_id}/carts/{cart_id}/lines/{line_id}` endpoint.
    *
    * Update a specific cart line item.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `cart_id: &str` -- The name of the folder.
    * * `line_id: &str` -- The id for the line item of a cart.
    */
    pub async fn patch_stores_carts_lines(
        &self,
        store_id: &str,
        cart_id: &str,
        line_id: &str,
        body: &crate::types::ECommerceCartLineItemDataType,
    ) -> Result<crate::types::ECommerceCartLineItem> {
        let url = format!(
            "/ecommerce/stores/{}/carts/{}/lines/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&cart_id.to_string()),
            crate::progenitor_support::encode_path(&line_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List customers.
    *
    * This function performs a `GET` to the `/ecommerce/stores/{store_id}/customers` endpoint.
    *
    * Get information about a store's customers.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `store_id: &str` -- The name of the folder.
    * * `email_address: &str` -- Restrict the response to customers with the email address.
    */
    pub async fn get_stores_customer(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        store_id: &str,
        email_address: &str,
    ) -> Result<crate::types::Customers> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !email_address.is_empty() {
            query_args.push(("email_address".to_string(), email_address.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ecommerce/stores/{}/customers?{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add customer.
    *
    * This function performs a `POST` to the `/ecommerce/stores/{store_id}/customers` endpoint.
    *
    * Add a new customer to a store.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    */
    pub async fn post_stores_customer(
        &self,
        store_id: &str,
        body: &crate::types::ECommerceCustomerData,
    ) -> Result<crate::types::Customer> {
        let url = format!(
            "/ecommerce/stores/{}/customers",
            crate::progenitor_support::encode_path(&store_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get customer info.
    *
    * This function performs a `GET` to the `/ecommerce/stores/{store_id}/customers/{customer_id}` endpoint.
    *
    * Get information about a specific customer.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `store_id: &str` -- The name of the folder.
    * * `customer_id: &str` -- The id for the customer of a store.
    */
    pub async fn get_stores_customer_ecommerce(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        store_id: &str,
        customer_id: &str,
    ) -> Result<crate::types::Customer> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ecommerce/stores/{}/customers/{}?{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&customer_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add or update customer.
    *
    * This function performs a `PUT` to the `/ecommerce/stores/{store_id}/customers/{customer_id}` endpoint.
    *
    * Add or update a customer.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `customer_id: &str` -- The id for the customer of a store.
    */
    pub async fn put_stores_customers(
        &self,
        store_id: &str,
        customer_id: &str,
        body: &crate::types::ECommerceCustomerDataType,
    ) -> Result<crate::types::Customer> {
        let url = format!(
            "/ecommerce/stores/{}/customers/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&customer_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Delete customer.
    *
    * This function performs a `DELETE` to the `/ecommerce/stores/{store_id}/customers/{customer_id}` endpoint.
    *
    * Delete a customer from a store.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `customer_id: &str` -- The id for the customer of a store.
    */
    pub async fn delete_stores_customers(&self, store_id: &str, customer_id: &str) -> Result<()> {
        let url = format!(
            "/ecommerce/stores/{}/customers/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&customer_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update customer.
    *
    * This function performs a `PATCH` to the `/ecommerce/stores/{store_id}/customers/{customer_id}` endpoint.
    *
    * Update a customer.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `customer_id: &str` -- The id for the customer of a store.
    */
    pub async fn patch_stores_customers(
        &self,
        store_id: &str,
        customer_id: &str,
        body: &crate::types::ECommerceCartCustomer,
    ) -> Result<crate::types::Customer> {
        let url = format!(
            "/ecommerce/stores/{}/customers/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&customer_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List promo rules.
    *
    * This function performs a `GET` to the `/ecommerce/stores/{store_id}/promo-rules` endpoint.
    *
    * Get information about a store's promo rules.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `store_id: &str` -- The name of the folder.
    */
    pub async fn get_stores_promorule(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        store_id: &str,
    ) -> Result<crate::types::PromoRulesData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ecommerce/stores/{}/promo-rules?{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add promo rule.
    *
    * This function performs a `POST` to the `/ecommerce/stores/{store_id}/promo-rules` endpoint.
    *
    * Add a new promo rule to a store.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    */
    pub async fn post_stores_promorule(
        &self,
        store_id: &str,
        body: &crate::types::ECommercePromoRule,
    ) -> Result<crate::types::PromoRules> {
        let url = format!(
            "/ecommerce/stores/{}/promo-rules",
            crate::progenitor_support::encode_path(&store_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get promo rule.
    *
    * This function performs a `GET` to the `/ecommerce/stores/{store_id}/promo-rules/{promo_rule_id}` endpoint.
    *
    * Get information about a specific promo rule.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `store_id: &str` -- The name of the folder.
    * * `promo_rule_id: &str` -- The id for the promo rule of a store.
    */
    pub async fn get_stores_promorule_ecommerce(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        store_id: &str,
        promo_rule_id: &str,
    ) -> Result<crate::types::PromoRules> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ecommerce/stores/{}/promo-rules/{}?{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&promo_rule_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete promo rule.
    *
    * This function performs a `DELETE` to the `/ecommerce/stores/{store_id}/promo-rules/{promo_rule_id}` endpoint.
    *
    * Delete a promo rule from a store.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `promo_rule_id: &str` -- The id for the promo rule of a store.
    */
    pub async fn delete_stores_promorules(
        &self,
        store_id: &str,
        promo_rule_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/ecommerce/stores/{}/promo-rules/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&promo_rule_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update promo rule.
    *
    * This function performs a `PATCH` to the `/ecommerce/stores/{store_id}/promo-rules/{promo_rule_id}` endpoint.
    *
    * Update a promo rule.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `promo_rule_id: &str` -- The id for the promo rule of a store.
    */
    pub async fn patch_stores_promorules(
        &self,
        store_id: &str,
        promo_rule_id: &str,
        body: &crate::types::ECommercePromoRuleData,
    ) -> Result<crate::types::PromoRules> {
        let url = format!(
            "/ecommerce/stores/{}/promo-rules/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&promo_rule_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List promo codes.
    *
    * This function performs a `GET` to the `/ecommerce/stores/{store_id}/promo-rules/{promo_rule_id}/promo-codes` endpoint.
    *
    * Get information about a store's promo codes.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `promo_rule_id: &str` -- The id for the promo rule of a store.
    * * `store_id: &str` -- The name of the folder.
    */
    pub async fn get_stores_promocode(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        promo_rule_id: &str,
        store_id: &str,
    ) -> Result<crate::types::PromoCodesData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ecommerce/stores/{}/promo-rules/{}/promo-codes?{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&promo_rule_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add promo code.
    *
    * This function performs a `POST` to the `/ecommerce/stores/{store_id}/promo-rules/{promo_rule_id}/promo-codes` endpoint.
    *
    * Add a new promo code to a store.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `promo_rule_id: &str` -- The id for the promo rule of a store.
    */
    pub async fn post_stores_promocode(
        &self,
        store_id: &str,
        promo_rule_id: &str,
        body: &crate::types::ECommercePromoCode,
    ) -> Result<crate::types::PromoCodes> {
        let url = format!(
            "/ecommerce/stores/{}/promo-rules/{}/promo-codes",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&promo_rule_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get promo code.
    *
    * This function performs a `GET` to the `/ecommerce/stores/{store_id}/promo-rules/{promo_rule_id}/promo-codes/{promo_code_id}` endpoint.
    *
    * Get information about a specific promo code.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `store_id: &str` -- The name of the folder.
    * * `promo_rule_id: &str` -- The id for the promo rule of a store.
    * * `promo_code_id: &str` -- The id for the promo code of a store.
    */
    pub async fn get_stores_promocode_ecommerce(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        store_id: &str,
        promo_rule_id: &str,
        promo_code_id: &str,
    ) -> Result<crate::types::PromoCodes> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ecommerce/stores/{}/promo-rules/{}/promo-codes/{}?{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&promo_rule_id.to_string()),
            crate::progenitor_support::encode_path(&promo_code_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete promo code.
    *
    * This function performs a `DELETE` to the `/ecommerce/stores/{store_id}/promo-rules/{promo_rule_id}/promo-codes/{promo_code_id}` endpoint.
    *
    * Delete a promo code from a store.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `promo_rule_id: &str` -- The id for the promo rule of a store.
    * * `promo_code_id: &str` -- The id for the promo code of a store.
    */
    pub async fn delete_stores_promocodes(
        &self,
        store_id: &str,
        promo_rule_id: &str,
        promo_code_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/ecommerce/stores/{}/promo-rules/{}/promo-codes/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&promo_rule_id.to_string()),
            crate::progenitor_support::encode_path(&promo_code_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update promo code.
    *
    * This function performs a `PATCH` to the `/ecommerce/stores/{store_id}/promo-rules/{promo_rule_id}/promo-codes/{promo_code_id}` endpoint.
    *
    * Update a promo code.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `promo_rule_id: &str` -- The id for the promo rule of a store.
    * * `promo_code_id: &str` -- The id for the promo code of a store.
    */
    pub async fn patch_stores_promocodes(
        &self,
        store_id: &str,
        promo_rule_id: &str,
        promo_code_id: &str,
        body: &crate::types::ECommercePromoCodeData,
    ) -> Result<crate::types::PromoCodes> {
        let url = format!(
            "/ecommerce/stores/{}/promo-rules/{}/promo-codes/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&promo_rule_id.to_string()),
            crate::progenitor_support::encode_path(&promo_code_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List orders.
    *
    * This function performs a `GET` to the `/ecommerce/stores/{store_id}/orders` endpoint.
    *
    * Get information about a store's orders.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `store_id: &str` -- The name of the folder.
    * * `customer_id: &str` -- Restrict results to orders made by a specific customer.
    * * `has_outreach: bool` -- Restrict results to orders that have an outreach attached. For example, an email campaign or Facebook ad.
    * * `campaign_id: &str` -- Restrict results to orders with a specific `campaign_id` value.
    * * `outreach_id: &str` -- Restrict results to orders with a specific `outreach_id` value.
    */
    pub async fn get_stores_order(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        store_id: &str,
        customer_id: &str,
        has_outreach: bool,
        campaign_id: &str,
        outreach_id: &str,
    ) -> Result<crate::types::OrdersDataType> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !campaign_id.is_empty() {
            query_args.push(("campaign_id".to_string(), campaign_id.to_string()));
        }
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !customer_id.is_empty() {
            query_args.push(("customer_id".to_string(), customer_id.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if has_outreach {
            query_args.push(("has_outreach".to_string(), has_outreach.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !outreach_id.is_empty() {
            query_args.push(("outreach_id".to_string(), outreach_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ecommerce/stores/{}/orders?{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add order.
    *
    * This function performs a `POST` to the `/ecommerce/stores/{store_id}/orders` endpoint.
    *
    * Add a new order to a store.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    */
    pub async fn post_stores_order(
        &self,
        store_id: &str,
        body: &crate::types::ECommerceOrder,
    ) -> Result<crate::types::Orders> {
        let url = format!(
            "/ecommerce/stores/{}/orders",
            crate::progenitor_support::encode_path(&store_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get order info.
    *
    * This function performs a `GET` to the `/ecommerce/stores/{store_id}/orders/{order_id}` endpoint.
    *
    * Get information about a specific order.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `store_id: &str` -- The name of the folder.
    * * `order_id: &str` -- The id for the order in a store.
    */
    pub async fn get_stores_order_ecommerce(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        store_id: &str,
        order_id: &str,
    ) -> Result<crate::types::Orders> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ecommerce/stores/{}/orders/{}?{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&order_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete order.
    *
    * This function performs a `DELETE` to the `/ecommerce/stores/{store_id}/orders/{order_id}` endpoint.
    *
    * Delete an order.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `order_id: &str` -- The id for the order in a store.
    */
    pub async fn delete_stores_orders(&self, store_id: &str, order_id: &str) -> Result<()> {
        let url = format!(
            "/ecommerce/stores/{}/orders/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&order_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update order.
    *
    * This function performs a `PATCH` to the `/ecommerce/stores/{store_id}/orders/{order_id}` endpoint.
    *
    * Update a specific order.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `order_id: &str` -- The id for the order in a store.
    */
    pub async fn patch_stores_orders(
        &self,
        store_id: &str,
        order_id: &str,
        body: &crate::types::ECommerceOrderData,
    ) -> Result<crate::types::Orders> {
        let url = format!(
            "/ecommerce/stores/{}/orders/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&order_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List order line items.
    *
    * This function performs a `GET` to the `/ecommerce/stores/{store_id}/orders/{order_id}/lines` endpoint.
    *
    * Get information about an order's line items.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `store_id: &str` -- The name of the folder.
    * * `order_id: &str` -- The id for the order in a store.
    */
    pub async fn get_stores_orders_line(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        store_id: &str,
        order_id: &str,
    ) -> Result<crate::types::OrderLines> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ecommerce/stores/{}/orders/{}/lines?{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&order_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add order line item.
    *
    * This function performs a `POST` to the `/ecommerce/stores/{store_id}/orders/{order_id}/lines` endpoint.
    *
    * Add a new line item to an existing order.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `order_id: &str` -- The id for the order in a store.
    */
    pub async fn post_stores_orders_line(
        &self,
        store_id: &str,
        order_id: &str,
        body: &crate::types::ECommerceOrderLineItem,
    ) -> Result<crate::types::Lines> {
        let url = format!(
            "/ecommerce/stores/{}/orders/{}/lines",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&order_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get order line item.
    *
    * This function performs a `GET` to the `/ecommerce/stores/{store_id}/orders/{order_id}/lines/{line_id}` endpoint.
    *
    * Get information about a specific order line item.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `store_id: &str` -- The name of the folder.
    * * `order_id: &str` -- The id for the order in a store.
    * * `line_id: &str` -- The id for the line item of an order.
    */
    pub async fn get_stores_orders_line_ecommerce(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        store_id: &str,
        order_id: &str,
        line_id: &str,
    ) -> Result<crate::types::Lines> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ecommerce/stores/{}/orders/{}/lines/{}?{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&order_id.to_string()),
            crate::progenitor_support::encode_path(&line_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete order line item.
    *
    * This function performs a `DELETE` to the `/ecommerce/stores/{store_id}/orders/{order_id}/lines/{line_id}` endpoint.
    *
    * Delete a specific order line item.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `order_id: &str` -- The id for the order in a store.
    * * `line_id: &str` -- The id for the line item of an order.
    */
    pub async fn delete_stores_orders_lines(
        &self,
        store_id: &str,
        order_id: &str,
        line_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/ecommerce/stores/{}/orders/{}/lines/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&order_id.to_string()),
            crate::progenitor_support::encode_path(&line_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update order line item.
    *
    * This function performs a `PATCH` to the `/ecommerce/stores/{store_id}/orders/{order_id}/lines/{line_id}` endpoint.
    *
    * Update a specific order line item.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `order_id: &str` -- The id for the order in a store.
    * * `line_id: &str` -- The id for the line item of an order.
    */
    pub async fn patch_stores_orders_lines(
        &self,
        store_id: &str,
        order_id: &str,
        line_id: &str,
        body: &crate::types::ECommerceOrderLineItemData,
    ) -> Result<crate::types::Lines> {
        let url = format!(
            "/ecommerce/stores/{}/orders/{}/lines/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&order_id.to_string()),
            crate::progenitor_support::encode_path(&line_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List product.
    *
    * This function performs a `GET` to the `/ecommerce/stores/{store_id}/products` endpoint.
    *
    * Get information about a store's products.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `store_id: &str` -- The name of the folder.
    */
    pub async fn get_stores_product(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        store_id: &str,
    ) -> Result<crate::types::ProductsData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ecommerce/stores/{}/products?{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add product.
    *
    * This function performs a `POST` to the `/ecommerce/stores/{store_id}/products` endpoint.
    *
    * Add a new product to a store.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    */
    pub async fn post_stores_product(
        &self,
        store_id: &str,
        body: &crate::types::ECommerceProductData,
    ) -> Result<crate::types::ECommerceProduct> {
        let url = format!(
            "/ecommerce/stores/{}/products",
            crate::progenitor_support::encode_path(&store_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get product info.
    *
    * This function performs a `GET` to the `/ecommerce/stores/{store_id}/products/{product_id}` endpoint.
    *
    * Get information about a specific product.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `store_id: &str` -- The name of the folder.
    * * `product_id: &str` -- The id for the product of a store.
    */
    pub async fn get_stores_product_ecommerce(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        store_id: &str,
        product_id: &str,
    ) -> Result<crate::types::ECommerceProduct> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ecommerce/stores/{}/products/{}?{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&product_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete product.
    *
    * This function performs a `DELETE` to the `/ecommerce/stores/{store_id}/products/{product_id}` endpoint.
    *
    * Delete a product.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `product_id: &str` -- The id for the product of a store.
    */
    pub async fn delete_stores_products(&self, store_id: &str, product_id: &str) -> Result<()> {
        let url = format!(
            "/ecommerce/stores/{}/products/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&product_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update product.
    *
    * This function performs a `PATCH` to the `/ecommerce/stores/{store_id}/products/{product_id}` endpoint.
    *
    * Update a specific product.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `product_id: &str` -- The id for the product of a store.
    */
    pub async fn patch_stores_products(
        &self,
        store_id: &str,
        product_id: &str,
        body: &crate::types::ECommerceProductDataType,
    ) -> Result<crate::types::ECommerceProduct> {
        let url = format!(
            "/ecommerce/stores/{}/products/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&product_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List product variants.
    *
    * This function performs a `GET` to the `/ecommerce/stores/{store_id}/products/{product_id}/variants` endpoint.
    *
    * Get information about a product's variants.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `store_id: &str` -- The name of the folder.
    * * `product_id: &str` -- The id for the product of a store.
    */
    pub async fn get_stores_products_variant(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        store_id: &str,
        product_id: &str,
    ) -> Result<crate::types::EcommerceProductVariants> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ecommerce/stores/{}/products/{}/variants?{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&product_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add product variant.
    *
    * This function performs a `POST` to the `/ecommerce/stores/{store_id}/products/{product_id}/variants` endpoint.
    *
    * Add a new variant to the product.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `product_id: &str` -- The id for the product of a store.
    */
    pub async fn post_stores_products_variant(
        &self,
        store_id: &str,
        product_id: &str,
        body: &crate::types::ECommerceProductVariant,
    ) -> Result<crate::types::Variants> {
        let url = format!(
            "/ecommerce/stores/{}/products/{}/variants",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&product_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get product variant info.
    *
    * This function performs a `GET` to the `/ecommerce/stores/{store_id}/products/{product_id}/variants/{variant_id}` endpoint.
    *
    * Get information about a specific product variant.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `store_id: &str` -- The name of the folder.
    * * `product_id: &str` -- The id for the product of a store.
    * * `variant_id: &str` -- The id for the product variant.
    */
    pub async fn get_stores_products_variant_ecommerce(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        store_id: &str,
        product_id: &str,
        variant_id: &str,
    ) -> Result<crate::types::Variants> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ecommerce/stores/{}/products/{}/variants/{}?{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&product_id.to_string()),
            crate::progenitor_support::encode_path(&variant_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add or update product variant.
    *
    * This function performs a `PUT` to the `/ecommerce/stores/{store_id}/products/{product_id}/variants/{variant_id}` endpoint.
    *
    * Add or update a product variant.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `product_id: &str` -- The id for the product of a store.
    * * `variant_id: &str` -- The id for the product variant.
    */
    pub async fn put_stores_products_variants(
        &self,
        store_id: &str,
        product_id: &str,
        variant_id: &str,
        body: &crate::types::ECommerceProductVariant,
    ) -> Result<crate::types::Variants> {
        let url = format!(
            "/ecommerce/stores/{}/products/{}/variants/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&product_id.to_string()),
            crate::progenitor_support::encode_path(&variant_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Delete product variant.
    *
    * This function performs a `DELETE` to the `/ecommerce/stores/{store_id}/products/{product_id}/variants/{variant_id}` endpoint.
    *
    * Delete a product variant.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `product_id: &str` -- The id for the product of a store.
    * * `variant_id: &str` -- The id for the product variant.
    */
    pub async fn delete_stores_products_variants(
        &self,
        store_id: &str,
        product_id: &str,
        variant_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/ecommerce/stores/{}/products/{}/variants/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&product_id.to_string()),
            crate::progenitor_support::encode_path(&variant_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update product variant.
    *
    * This function performs a `PATCH` to the `/ecommerce/stores/{store_id}/products/{product_id}/variants/{variant_id}` endpoint.
    *
    * Update a product variant.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `product_id: &str` -- The id for the product of a store.
    * * `variant_id: &str` -- The id for the product variant.
    */
    pub async fn patch_stores_products_variants(
        &self,
        store_id: &str,
        product_id: &str,
        variant_id: &str,
        body: &crate::types::ECommerceProductVariantData,
    ) -> Result<crate::types::Variants> {
        let url = format!(
            "/ecommerce/stores/{}/products/{}/variants/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&product_id.to_string()),
            crate::progenitor_support::encode_path(&variant_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List product images.
    *
    * This function performs a `GET` to the `/ecommerce/stores/{store_id}/products/{product_id}/images` endpoint.
    *
    * Get information about a product's images.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `store_id: &str` -- The name of the folder.
    * * `product_id: &str` -- The id for the product of a store.
    */
    pub async fn get_stores_products_image(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        store_id: &str,
        product_id: &str,
    ) -> Result<crate::types::EcommerceProductImages> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ecommerce/stores/{}/products/{}/images?{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&product_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add product image.
    *
    * This function performs a `POST` to the `/ecommerce/stores/{store_id}/products/{product_id}/images` endpoint.
    *
    * Add a new image to the product.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `product_id: &str` -- The id for the product of a store.
    */
    pub async fn post_stores_products_image(
        &self,
        store_id: &str,
        product_id: &str,
        body: &crate::types::ECommerceProductImage,
    ) -> Result<crate::types::Images> {
        let url = format!(
            "/ecommerce/stores/{}/products/{}/images",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&product_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get product image info.
    *
    * This function performs a `GET` to the `/ecommerce/stores/{store_id}/products/{product_id}/images/{image_id}` endpoint.
    *
    * Get information about a specific product image.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `store_id: &str` -- The name of the folder.
    * * `product_id: &str` -- The id for the product of a store.
    * * `image_id: &str` -- The id for the product image.
    */
    pub async fn get_stores_products_image_ecommerce(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        store_id: &str,
        product_id: &str,
        image_id: &str,
    ) -> Result<crate::types::Images> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/ecommerce/stores/{}/products/{}/images/{}?{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&product_id.to_string()),
            crate::progenitor_support::encode_path(&image_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete product image.
    *
    * This function performs a `DELETE` to the `/ecommerce/stores/{store_id}/products/{product_id}/images/{image_id}` endpoint.
    *
    * Delete a product image.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `product_id: &str` -- The id for the product of a store.
    * * `image_id: &str` -- The id for the product image.
    */
    pub async fn delete_stores_products_images(
        &self,
        store_id: &str,
        product_id: &str,
        image_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/ecommerce/stores/{}/products/{}/images/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&product_id.to_string()),
            crate::progenitor_support::encode_path(&image_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update product image.
    *
    * This function performs a `PATCH` to the `/ecommerce/stores/{store_id}/products/{product_id}/images/{image_id}` endpoint.
    *
    * Update a product image.
    *
    * **Parameters:**
    *
    * * `store_id: &str` -- The name of the folder.
    * * `product_id: &str` -- The id for the product of a store.
    * * `image_id: &str` -- The id for the product image.
    */
    pub async fn patch_stores_products_images(
        &self,
        store_id: &str,
        product_id: &str,
        image_id: &str,
        body: &crate::types::ECommerceProductImageData,
    ) -> Result<crate::types::Images> {
        let url = format!(
            "/ecommerce/stores/{}/products/{}/images/{}",
            crate::progenitor_support::encode_path(&store_id.to_string()),
            crate::progenitor_support::encode_path(&product_id.to_string()),
            crate::progenitor_support::encode_path(&image_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
