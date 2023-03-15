use crate::Client;
use crate::ClientResult;

pub struct Products {
    pub client: Client,
}

impl Products {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Products { client }
    }

    /**
     * Get multiple products.
     *
     * This function performs a `GET` to the `/product` endpoint.
     *
     * **Parameters:**
     *
     * * `page: i64` -- Unique id of the channel.
     * * `limit: i64` -- Amount of products per page to request.
     * * `i_ds: &[String]` -- Comma separated list of product ids to filter by.
     * * `reference_ids: &[String]` -- Comma separated list of reference ids to filter by.
     * * `search: &str` -- Search is available for 2 fields of the inventory record related to the product: Inventory ID and Name -
     *   1. Expected behavior for search by Inventory ID is exact match
     *   2. Expected behavior for search by Inventory Name is partial match, i.e. does not have to be start of word,
     *   but must be consecutive characters. This is not case sensitive.
     * * `active_status: crate::types::ProductActiveStatus` -- Status filter for products:
     *   - Any: Include both active and inactive
     *   - Active: Filter products that are Active
     *   - Inactive: Filter products that are Inactive.
     * * `bundle_status: crate::types::ProductBundleStatus` -- Bundle filter for products:
     *   - Any: Don't filter and consider products that are bundles or not bundles
     *   - Bundle: Filter by products that are bundles
     *   - NotBundle: Filter by products that are not bundles.
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn get_page(
        &self,
        page: i64,
        limit: i64,
        ids: &[String],
        reference_ids: &[String],
        search: &str,
        active_status: crate::types::ProductActiveStatus,
        bundle_status: crate::types::ProductBundleStatus,
    ) -> ClientResult<Vec<crate::types::Product>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !active_status.to_string().is_empty() {
            query_args.push(("ActiveStatus".to_string(), active_status.to_string()));
        }
        if !bundle_status.to_string().is_empty() {
            query_args.push(("BundleStatus".to_string(), bundle_status.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("IDs".to_string(), ids.join(" ")));
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
        if !search.is_empty() {
            query_args.push(("Search".to_string(), search.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/product?{}", query_), None);
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
     * Get multiple products.
     *
     * This function performs a `GET` to the `/product` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     */
    pub async fn get_all(
        &self,
        ids: &[String],
        reference_ids: &[String],
        search: &str,
        active_status: crate::types::ProductActiveStatus,
        bundle_status: crate::types::ProductBundleStatus,
    ) -> ClientResult<Vec<crate::types::Product>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !active_status.to_string().is_empty() {
            query_args.push(("ActiveStatus".to_string(), active_status.to_string()));
        }
        if !bundle_status.to_string().is_empty() {
            query_args.push(("BundleStatus".to_string(), bundle_status.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("IDs".to_string(), ids.join(" ")));
        }
        if !reference_ids.is_empty() {
            query_args.push(("ReferenceIds".to_string(), reference_ids.join(" ")));
        }
        if !search.is_empty() {
            query_args.push(("Search".to_string(), search.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/product?{}", query_), None);
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
     * Add a single product to the store.
     *
     * This function performs a `POST` to the `/product` endpoint.
     *
     * **Parameters:**
     *
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn post(
        &self,
        body: &crate::types::ProductsCreateProductModel,
    ) -> ClientResult<Vec<crate::types::Product>> {
        let url = self.client.url("/product", None);
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
     * Get a single product.
     *
     * This function performs a `GET` to the `/product/{productId}` endpoint.
     *
     * **Parameters:**
     *
     * * `product_id: i64` -- Unique identifier of the product.
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn get(&self, product_id: i64) -> ClientResult<crate::types::Product> {
        let url = self.client.url(
            &format!(
                "/product/{}",
                crate::progenitor_support::encode_path(&product_id.to_string()),
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
     * Modify a single product.
     *
     * This function performs a `PUT` to the `/product/{productId}` endpoint.
     *
     * **Parameters:**
     *
     * * `product_id: i64` -- Unique identifier of the product to modify.
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn put(
        &self,
        product_id: i64,
        body: &crate::types::ProductsUpdateProductModel,
    ) -> ClientResult<Vec<crate::types::Product>> {
        let url = self.client.url(
            &format!(
                "/product/{}",
                crate::progenitor_support::encode_path(&product_id.to_string()),
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
     * Add multiple products to the store.
     *
     * This function performs a `POST` to the `/product/batch` endpoint.
     *
     * **Parameters:**
     *
     * * `channel_id: i64` -- Unique id of the channel.
     */
    pub async fn post_batch(
        &self,
        body: &[crate::types::ProductsCreateProductModel],
    ) -> ClientResult<Vec<crate::types::Product>> {
        let url = self.client.url("/product/batch", None);
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
}
