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
     * Retrieves a list of collects. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/collects.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#index-2020-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_collect(
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
            &format!("/admin/api/2020-01/collects.json?{}", query_),
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
     * Adds a product to a custom collection.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/collects.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#create-2020-01
     */
    pub async fn deprecated_202001_create_collects(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-01/collects.json", None);
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
     * Retrieves a specific collect by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/collects/{collect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#show-2020-01
     *
     * **Parameters:**
     *
     * * `collect_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_collects_param_collect(
        &self,
        collect_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/collects/{}/json?{}",
                crate::progenitor_support::encode_path(collect_id),
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
     * Removes a product from a collection.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/collects/{collect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `collect_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_collects_param_collect(
        &self,
        collect_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/collects/{}/json",
                crate::progenitor_support::encode_path(collect_id),
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
     * Retrieves a count of collects.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/collects/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#count-2020-01
     *
     * **Parameters:**
     *
     * * `collection_id: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202001_get_collects_count(
        &self,
        collection_id: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if collection_id > 0 {
            query_args.push(("collection_id".to_string(), collection_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/collects/count.json?{}", query_),
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
     * Retrieves a list of collects. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/collects.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#index-2020-04
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_collect(
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
            &format!("/admin/api/2020-04/collects.json?{}", query_),
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
     * Adds a product to a custom collection.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/collects.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#create-2020-04
     */
    pub async fn deprecated_202004_create_collects(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-04/collects.json", None);
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
     * Retrieves a specific collect by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/collects/{collect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#show-2020-04
     *
     * **Parameters:**
     *
     * * `collect_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_collects_param_collect(
        &self,
        collect_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/collects/{}/json?{}",
                crate::progenitor_support::encode_path(collect_id),
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
     * Removes a product from a collection.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/collects/{collect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `collect_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_collects_param_collect(
        &self,
        collect_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/collects/{}/json",
                crate::progenitor_support::encode_path(collect_id),
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
     * Retrieves a count of collects.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/collects/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#count-2020-04
     *
     * **Parameters:**
     *
     * * `collection_id: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202004_get_collects_count(
        &self,
        collection_id: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if collection_id > 0 {
            query_args.push(("collection_id".to_string(), collection_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/collects/count.json?{}", query_),
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
     * Retrieves a list of collects. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/collects.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#index-2020-07
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_collect(
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
            &format!("/admin/api/2020-07/collects.json?{}", query_),
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
     * Adds a product to a custom collection.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/collects.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#create-2020-07
     */
    pub async fn deprecated_202007_create_collects(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-07/collects.json", None);
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
     * Retrieves a specific collect by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/collects/{collect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#show-2020-07
     *
     * **Parameters:**
     *
     * * `collect_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_collects_param_collect(
        &self,
        collect_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/collects/{}/json?{}",
                crate::progenitor_support::encode_path(collect_id),
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
     * Removes a product from a collection.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/collects/{collect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `collect_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_collects_param_collect(
        &self,
        collect_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/collects/{}/json",
                crate::progenitor_support::encode_path(collect_id),
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
     * Retrieves a count of collects.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/collects/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#count-2020-07
     *
     * **Parameters:**
     *
     * * `collection_id: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202007_get_collects_count(
        &self,
        collection_id: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if collection_id > 0 {
            query_args.push(("collection_id".to_string(), collection_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/collects/count.json?{}", query_),
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
     * Retrieves a list of collects. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/collects.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#index-2020-10
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_collect(&self, limit: &str, since_id: &str, fields: &str) -> ClientResult<()> {
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
            &format!("/admin/api/2020-10/collects.json?{}", query_),
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
     * Adds a product to a custom collection.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/collects.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#create-2020-10
     */
    pub async fn create_collects(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-10/collects.json", None);
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
     * Retrieves a specific collect by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/collects/{collect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#show-2020-10
     *
     * **Parameters:**
     *
     * * `collect_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_collects_param_collect(
        &self,
        collect_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/collects/{}/json?{}",
                crate::progenitor_support::encode_path(collect_id),
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
     * Removes a product from a collection.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/collects/{collect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `collect_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_collects_param_collect(&self, collect_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/collects/{}/json",
                crate::progenitor_support::encode_path(collect_id),
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
     * Retrieves a count of collects.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/collects/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#count-2020-10
     *
     * **Parameters:**
     *
     * * `collection_id: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn get_collects_count(&self, collection_id: i64) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if collection_id > 0 {
            query_args.push(("collection_id".to_string(), collection_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/collects/count.json?{}", query_),
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
     * Retrieves a list of collects. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/collects.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#index-2021-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_collect(
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
            &format!("/admin/api/2021-01/collects.json?{}", query_),
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
     * Adds a product to a custom collection.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/collects.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#create-2021-01
     */
    pub async fn deprecated_202101_create_collects(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2021-01/collects.json", None);
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
     * Retrieves a specific collect by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/collects/{collect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#show-2021-01
     *
     * **Parameters:**
     *
     * * `collect_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_collects_param_collect(
        &self,
        collect_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/collects/{}/json?{}",
                crate::progenitor_support::encode_path(collect_id),
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
     * Removes a product from a collection.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/collects/{collect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `collect_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_collects_param_collect(
        &self,
        collect_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/collects/{}/json",
                crate::progenitor_support::encode_path(collect_id),
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
     * Retrieves a count of collects.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/collects/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#count-2021-01
     *
     * **Parameters:**
     *
     * * `collection_id: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202101_get_collects_count(
        &self,
        collection_id: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if collection_id > 0 {
            query_args.push(("collection_id".to_string(), collection_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/collects/count.json?{}", query_),
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
     * Retrieves a list of collects. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/collects.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#index-unstable
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to show.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_collect(
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
            &format!("/admin/api/unstable/collects.json?{}", query_),
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
     * Adds a product to a custom collection.
     *
     * This function performs a `POST` to the `/admin/api/unstable/collects.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#create-unstable
     */
    pub async fn deprecated_unstable_create_collects(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/unstable/collects.json", None);
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
     * Retrieves a specific collect by its ID.
     *
     * This function performs a `GET` to the `/admin/api/unstable/collects/{collect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#show-unstable
     *
     * **Parameters:**
     *
     * * `collect_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_collects_param_collect(
        &self,
        collect_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/collects/{}/json?{}",
                crate::progenitor_support::encode_path(collect_id),
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
     * Removes a product from a collection.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/collects/{collect_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#destroy-unstable
     *
     * **Parameters:**
     *
     * * `collect_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_collects_param_collect(
        &self,
        collect_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/collects/{}/json",
                crate::progenitor_support::encode_path(collect_id),
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
     * Retrieves a count of collects.
     *
     * This function performs a `GET` to the `/admin/api/unstable/collects/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collect#count-unstable
     *
     * **Parameters:**
     *
     * * `collection_id: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_unstable_get_collects_count(
        &self,
        collection_id: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if collection_id > 0 {
            query_args.push(("collection_id".to_string(), collection_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/collects/count.json?{}", query_),
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
     * Retrieves a single collection.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/collections/{collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collection#show-2020-01
     *
     * **Parameters:**
     *
     * * `collection_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_collections_param_collection(
        &self,
        collection_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/collections/{}/json?{}",
                crate::progenitor_support::encode_path(collection_id),
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
     * Retrieve a list of products belonging to a collection. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.. The products returned are sorted by the collection's sort order.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/collections/{collection_id}/products.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collection#products-2020-01
     *
     * **Parameters:**
     *
     * * `collection_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- The number of products to retrieve.
     *                     (default: 50, maximum: 250).
     */
    pub async fn deprecated_202001_get_collections_param_collection_products(
        &self,
        collection_id: &str,
        limit: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/collections/{}/products.json?{}",
                crate::progenitor_support::encode_path(collection_id),
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
     * Retrieves a single collection.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/collections/{collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collection#show-2020-04
     *
     * **Parameters:**
     *
     * * `collection_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_collections_param_collection(
        &self,
        collection_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/collections/{}/json?{}",
                crate::progenitor_support::encode_path(collection_id),
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
     * Retrieve a list of products belonging to a collection. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.. The products returned are sorted by the collection's sort order.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/collections/{collection_id}/products.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collection#products-2020-04
     *
     * **Parameters:**
     *
     * * `collection_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- The number of products to retrieve.
     *                     (default: 50, maximum: 250).
     */
    pub async fn deprecated_202004_get_collections_param_collection_products(
        &self,
        collection_id: &str,
        limit: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/collections/{}/products.json?{}",
                crate::progenitor_support::encode_path(collection_id),
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
     * Retrieves a single collection.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/collections/{collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collection#show-2020-07
     *
     * **Parameters:**
     *
     * * `collection_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_collections_param_collection(
        &self,
        collection_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/collections/{}/json?{}",
                crate::progenitor_support::encode_path(collection_id),
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
     * Retrieve a list of products belonging to a collection. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.. The products returned are sorted by the collection's sort order.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/collections/{collection_id}/products.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collection#products-2020-07
     *
     * **Parameters:**
     *
     * * `collection_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- The number of products to retrieve.
     *                     (default: 50, maximum: 250).
     */
    pub async fn deprecated_202007_get_collections_param_collection_products(
        &self,
        collection_id: &str,
        limit: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/collections/{}/products.json?{}",
                crate::progenitor_support::encode_path(collection_id),
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
     * Retrieves a single collection.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/collections/{collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collection#show-2020-10
     *
     * **Parameters:**
     *
     * * `collection_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_collections_param_collection(
        &self,
        collection_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/collections/{}/json?{}",
                crate::progenitor_support::encode_path(collection_id),
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
     * Retrieve a list of products belonging to a collection. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.. The products returned are sorted by the collection's sort order.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/collections/{collection_id}/products.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collection#products-2020-10
     *
     * **Parameters:**
     *
     * * `collection_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- The number of products to retrieve.
     *                     (default: 50, maximum: 250).
     */
    pub async fn get_collections_param_collection_products(
        &self,
        collection_id: &str,
        limit: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/collections/{}/products.json?{}",
                crate::progenitor_support::encode_path(collection_id),
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
     * Retrieves a single collection.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/collections/{collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collection#show-2021-01
     *
     * **Parameters:**
     *
     * * `collection_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_collections_param_collection(
        &self,
        collection_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/collections/{}/json?{}",
                crate::progenitor_support::encode_path(collection_id),
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
     * Retrieve a list of products belonging to a collection. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.. The products returned are sorted by the collection's sort order.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/collections/{collection_id}/products.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collection#products-2021-01
     *
     * **Parameters:**
     *
     * * `collection_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- The number of products to retrieve.
     *                     (default: 50, maximum: 250).
     */
    pub async fn deprecated_202101_get_collections_param_collection_products(
        &self,
        collection_id: &str,
        limit: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/collections/{}/products.json?{}",
                crate::progenitor_support::encode_path(collection_id),
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
     * Retrieves a single collection.
     *
     * This function performs a `GET` to the `/admin/api/unstable/collections/{collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collection#show-unstable
     *
     * **Parameters:**
     *
     * * `collection_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_collections_param_collection(
        &self,
        collection_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/collections/{}/json?{}",
                crate::progenitor_support::encode_path(collection_id),
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
     * Retrieve a list of products belonging to a collection. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.. The products returned are sorted by the collection's sort order.
     *
     * This function performs a `GET` to the `/admin/api/unstable/collections/{collection_id}/products.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/collection#products-unstable
     *
     * **Parameters:**
     *
     * * `collection_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- The number of products to retrieve.
     *                     (default: 50, maximum: 250).
     */
    pub async fn deprecated_unstable_get_collections_param_collection_products(
        &self,
        collection_id: &str,
        limit: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/collections/{}/products.json?{}",
                crate::progenitor_support::encode_path(collection_id),
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
     * Retrieves a list of custom collections. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/custom_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#index-2020-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `ids: &str` -- Show only collections specified by a comma-separated list of IDs.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `title: &str` -- Show custom collections with a given title.
     * * `product_id: &str` -- Show custom collections that include a given product.
     * * `handle: &str` -- Filter by custom collection handle.
     * * `updated_at_min: &str` -- Show custom collections last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show custom collections last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show custom collections published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show custom collections published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Show custom collectsion with a given published status.
     *                     (default: any)
     *                       
     *                           published: Show only published custom collections.
     *                           unpublished: Show only unpublished custom collections.
     *                           any: Show custom collections of any published status.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_custom_collection(
        &self,
        limit: &str,
        ids: &str,
        since_id: &str,
        title: &str,
        product_id: &str,
        handle: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/custom_collections.json?{}", query_),
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
     * Creates a custom collection.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/custom_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#create-2020-01
     */
    pub async fn deprecated_202001_create_custom_collections(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-01/custom_collections.json", None);
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
     * Retrieves a count of custom collections.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/custom_collections/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#count-2020-01
     *
     * **Parameters:**
     *
     * * `title: &str` -- Count custom collections with given title.
     * * `product_id: &str` -- Count custom collections that include a given product.
     * * `updated_at_min: &str` -- Count custom collections last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count custom collections last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Count custom collections published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Count custom collections published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Count custom collections with a given published status.
     *                     (default: any)
     *                       
     *                           published: Count only published custom collections.
     *                           unpublished: Count only unpublished custom collections.
     *                           any: Count custom collections of any published status.
     */
    pub async fn deprecated_202001_get_custom_collections_count(
        &self,
        title: &str,
        product_id: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
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
                "/admin/api/2020-01/custom_collections/count.json?{}",
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
     * Retrieves a single custom collection.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/custom_collections/{custom_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#show-2020-01
     *
     * **Parameters:**
     *
     * * `custom_collection_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_custom_collections_param_collection(
        &self,
        custom_collection_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/custom_collections/{}/json?{}",
                crate::progenitor_support::encode_path(custom_collection_id),
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
     * Updates an existing custom collection.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/custom_collections/{custom_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#update-2020-01
     *
     * **Parameters:**
     *
     * * `custom_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_custom_collections_param_collection(
        &self,
        custom_collection_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/custom_collections/{}/json",
                crate::progenitor_support::encode_path(custom_collection_id),
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
     * Deletes a custom collection.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/custom_collections/{custom_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `custom_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_custom_collections_param_collection(
        &self,
        custom_collection_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/custom_collections/{}/json",
                crate::progenitor_support::encode_path(custom_collection_id),
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
     * Retrieves a list of custom collections. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/custom_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#index-2020-04
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `ids: &str` -- Show only collections specified by a comma-separated list of IDs.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `title: &str` -- Show custom collections with a given title.
     * * `product_id: &str` -- Show custom collections that include a given product.
     * * `handle: &str` -- Filter by custom collection handle.
     * * `updated_at_min: &str` -- Show custom collections last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show custom collections last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show custom collections published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show custom collections published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Show custom collectsion with a given published status.
     *                     (default: any)
     *                       
     *                           published: Show only published custom collections.
     *                           unpublished: Show only unpublished custom collections.
     *                           any: Show custom collections of any published status.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_custom_collection(
        &self,
        limit: &str,
        ids: &str,
        since_id: &str,
        title: &str,
        product_id: &str,
        handle: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/custom_collections.json?{}", query_),
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
     * Creates a custom collection.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/custom_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#create-2020-04
     */
    pub async fn deprecated_202004_create_custom_collections(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-04/custom_collections.json", None);
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
     * Retrieves a count of custom collections.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/custom_collections/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#count-2020-04
     *
     * **Parameters:**
     *
     * * `title: &str` -- Count custom collections with given title.
     * * `product_id: &str` -- Count custom collections that include a given product.
     * * `updated_at_min: &str` -- Count custom collections last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count custom collections last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Count custom collections published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Count custom collections published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Count custom collections with a given published status.
     *                     (default: any)
     *                       
     *                           published: Count only published custom collections.
     *                           unpublished: Count only unpublished custom collections.
     *                           any: Count custom collections of any published status.
     */
    pub async fn deprecated_202004_get_custom_collections_count(
        &self,
        title: &str,
        product_id: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
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
                "/admin/api/2020-04/custom_collections/count.json?{}",
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
     * Retrieves a single custom collection.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/custom_collections/{custom_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#show-2020-04
     *
     * **Parameters:**
     *
     * * `custom_collection_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_custom_collections_param_collection(
        &self,
        custom_collection_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/custom_collections/{}/json?{}",
                crate::progenitor_support::encode_path(custom_collection_id),
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
     * Updates an existing custom collection.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/custom_collections/{custom_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#update-2020-04
     *
     * **Parameters:**
     *
     * * `custom_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_custom_collections_param_collection(
        &self,
        custom_collection_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/custom_collections/{}/json",
                crate::progenitor_support::encode_path(custom_collection_id),
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
     * Deletes a custom collection.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/custom_collections/{custom_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `custom_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_custom_collections_param_collection(
        &self,
        custom_collection_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/custom_collections/{}/json",
                crate::progenitor_support::encode_path(custom_collection_id),
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
     * Retrieves a list of custom collections. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/custom_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#index-2020-07
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `ids: &str` -- Show only collections specified by a comma-separated list of IDs.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `title: &str` -- Show custom collections with a given title.
     * * `product_id: &str` -- Show custom collections that include a given product.
     * * `handle: &str` -- Filter by custom collection handle.
     * * `updated_at_min: &str` -- Show custom collections last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show custom collections last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show custom collections published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show custom collections published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Show custom collectsion with a given published status.
     *                     (default: any)
     *                       
     *                           published: Show only published custom collections.
     *                           unpublished: Show only unpublished custom collections.
     *                           any: Show custom collections of any published status.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_custom_collection(
        &self,
        limit: &str,
        ids: &str,
        since_id: &str,
        title: &str,
        product_id: &str,
        handle: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/custom_collections.json?{}", query_),
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
     * Creates a custom collection.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/custom_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#create-2020-07
     */
    pub async fn deprecated_202007_create_custom_collections(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-07/custom_collections.json", None);
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
     * Retrieves a count of custom collections.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/custom_collections/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#count-2020-07
     *
     * **Parameters:**
     *
     * * `title: &str` -- Count custom collections with given title.
     * * `product_id: &str` -- Count custom collections that include a given product.
     * * `updated_at_min: &str` -- Count custom collections last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count custom collections last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Count custom collections published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Count custom collections published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Count custom collections with a given published status.
     *                     (default: any)
     *                       
     *                           published: Count only published custom collections.
     *                           unpublished: Count only unpublished custom collections.
     *                           any: Count custom collections of any published status.
     */
    pub async fn deprecated_202007_get_custom_collections_count(
        &self,
        title: &str,
        product_id: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
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
                "/admin/api/2020-07/custom_collections/count.json?{}",
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
     * Retrieves a single custom collection.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/custom_collections/{custom_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#show-2020-07
     *
     * **Parameters:**
     *
     * * `custom_collection_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_custom_collections_param_collection(
        &self,
        custom_collection_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/custom_collections/{}/json?{}",
                crate::progenitor_support::encode_path(custom_collection_id),
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
     * Updates an existing custom collection.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/custom_collections/{custom_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#update-2020-07
     *
     * **Parameters:**
     *
     * * `custom_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_custom_collections_param_collection(
        &self,
        custom_collection_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/custom_collections/{}/json",
                crate::progenitor_support::encode_path(custom_collection_id),
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
     * Deletes a custom collection.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/custom_collections/{custom_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `custom_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_custom_collections_param_collection(
        &self,
        custom_collection_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/custom_collections/{}/json",
                crate::progenitor_support::encode_path(custom_collection_id),
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
     * Retrieves a list of custom collections. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/custom_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#index-2020-10
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `ids: &str` -- Show only collections specified by a comma-separated list of IDs.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `title: &str` -- Show custom collections with a given title.
     * * `product_id: &str` -- Show custom collections that include a given product.
     * * `handle: &str` -- Filter by custom collection handle.
     * * `updated_at_min: &str` -- Show custom collections last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show custom collections last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show custom collections published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show custom collections published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Show custom collectsion with a given published status.
     *                     (default: any)
     *                       
     *                           published: Show only published custom collections.
     *                           unpublished: Show only unpublished custom collections.
     *                           any: Show custom collections of any published status.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_custom_collection(
        &self,
        limit: &str,
        ids: &str,
        since_id: &str,
        title: &str,
        product_id: &str,
        handle: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/custom_collections.json?{}", query_),
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
     * Creates a custom collection.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/custom_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#create-2020-10
     */
    pub async fn create_custom_collections(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-10/custom_collections.json", None);
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
     * Retrieves a count of custom collections.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/custom_collections/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#count-2020-10
     *
     * **Parameters:**
     *
     * * `title: &str` -- Count custom collections with given title.
     * * `product_id: &str` -- Count custom collections that include a given product.
     * * `updated_at_min: &str` -- Count custom collections last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count custom collections last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Count custom collections published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Count custom collections published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Count custom collections with a given published status.
     *                     (default: any)
     *                       
     *                           published: Count only published custom collections.
     *                           unpublished: Count only unpublished custom collections.
     *                           any: Count custom collections of any published status.
     */
    pub async fn get_custom_collections_count(
        &self,
        title: &str,
        product_id: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
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
                "/admin/api/2020-10/custom_collections/count.json?{}",
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
     * Retrieves a single custom collection.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/custom_collections/{custom_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#show-2020-10
     *
     * **Parameters:**
     *
     * * `custom_collection_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_custom_collections_param_collection(
        &self,
        custom_collection_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/custom_collections/{}/json?{}",
                crate::progenitor_support::encode_path(custom_collection_id),
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
     * Updates an existing custom collection.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/custom_collections/{custom_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#update-2020-10
     *
     * **Parameters:**
     *
     * * `custom_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn update_custom_collections_param_collection(
        &self,
        custom_collection_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/custom_collections/{}/json",
                crate::progenitor_support::encode_path(custom_collection_id),
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
     * Deletes a custom collection.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/custom_collections/{custom_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `custom_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_custom_collections_param_collection(
        &self,
        custom_collection_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/custom_collections/{}/json",
                crate::progenitor_support::encode_path(custom_collection_id),
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
     * Retrieves a list of custom collections. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/custom_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#index-2021-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `ids: &str` -- Show only collections specified by a comma-separated list of IDs.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `title: &str` -- Show custom collections with a given title.
     * * `product_id: &str` -- Show custom collections that include a given product.
     * * `handle: &str` -- Filter by custom collection handle.
     * * `updated_at_min: &str` -- Show custom collections last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show custom collections last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show custom collections published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show custom collections published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Show custom collectsion with a given published status.
     *                     (default: any)
     *                       
     *                           published: Show only published custom collections.
     *                           unpublished: Show only unpublished custom collections.
     *                           any: Show custom collections of any published status.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_custom_collection(
        &self,
        limit: &str,
        ids: &str,
        since_id: &str,
        title: &str,
        product_id: &str,
        handle: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/custom_collections.json?{}", query_),
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
     * Creates a custom collection.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/custom_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#create-2021-01
     */
    pub async fn deprecated_202101_create_custom_collections(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2021-01/custom_collections.json", None);
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
     * Retrieves a count of custom collections.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/custom_collections/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#count-2021-01
     *
     * **Parameters:**
     *
     * * `title: &str` -- Count custom collections with given title.
     * * `product_id: &str` -- Count custom collections that include a given product.
     * * `updated_at_min: &str` -- Count custom collections last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count custom collections last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Count custom collections published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Count custom collections published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Count custom collections with a given published status.
     *                     (default: any)
     *                       
     *                           published: Count only published custom collections.
     *                           unpublished: Count only unpublished custom collections.
     *                           any: Count custom collections of any published status.
     */
    pub async fn deprecated_202101_get_custom_collections_count(
        &self,
        title: &str,
        product_id: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
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
                "/admin/api/2021-01/custom_collections/count.json?{}",
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
     * Retrieves a single custom collection.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/custom_collections/{custom_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#show-2021-01
     *
     * **Parameters:**
     *
     * * `custom_collection_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_custom_collections_param_collection(
        &self,
        custom_collection_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/custom_collections/{}/json?{}",
                crate::progenitor_support::encode_path(custom_collection_id),
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
     * Updates an existing custom collection.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/custom_collections/{custom_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#update-2021-01
     *
     * **Parameters:**
     *
     * * `custom_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_custom_collections_param_collection(
        &self,
        custom_collection_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/custom_collections/{}/json",
                crate::progenitor_support::encode_path(custom_collection_id),
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
     * Deletes a custom collection.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/custom_collections/{custom_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `custom_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_custom_collections_param_collection(
        &self,
        custom_collection_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/custom_collections/{}/json",
                crate::progenitor_support::encode_path(custom_collection_id),
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
     * Retrieves a list of custom collections. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/custom_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#index-unstable
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The maximum number of results to retrieve.
     *                     (default: 50, maximum: 250).
     * * `ids: &str` -- Show only collections specified by a comma-separated list of IDs.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `title: &str` -- Show custom collections with a given title.
     * * `product_id: &str` -- Show custom collections that include a given product.
     * * `handle: &str` -- Filter by custom collection handle.
     * * `updated_at_min: &str` -- Show custom collections last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show custom collections last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show custom collections published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show custom collections published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Show custom collectsion with a given published status.
     *                     (default: any)
     *                       
     *                           published: Show only published custom collections.
     *                           unpublished: Show only unpublished custom collections.
     *                           any: Show custom collections of any published status.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_custom_collection(
        &self,
        limit: &str,
        ids: &str,
        since_id: &str,
        title: &str,
        product_id: &str,
        handle: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/custom_collections.json?{}", query_),
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
     * Creates a custom collection.
     *
     * This function performs a `POST` to the `/admin/api/unstable/custom_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#create-unstable
     */
    pub async fn deprecated_unstable_create_custom_collections(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/unstable/custom_collections.json", None);
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
     * Retrieves a count of custom collections.
     *
     * This function performs a `GET` to the `/admin/api/unstable/custom_collections/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#count-unstable
     *
     * **Parameters:**
     *
     * * `title: &str` -- Count custom collections with given title.
     * * `product_id: &str` -- Count custom collections that include a given product.
     * * `updated_at_min: &str` -- Count custom collections last updated after date (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Count custom collections last updated before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Count custom collections published after date (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Count custom collections published before date (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Count custom collections with a given published status.
     *                     (default: any)
     *                       
     *                           published: Count only published custom collections.
     *                           unpublished: Count only unpublished custom collections.
     *                           any: Count custom collections of any published status.
     */
    pub async fn deprecated_unstable_get_custom_collections_count(
        &self,
        title: &str,
        product_id: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
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
                "/admin/api/unstable/custom_collections/count.json?{}",
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
     * Retrieves a single custom collection.
     *
     * This function performs a `GET` to the `/admin/api/unstable/custom_collections/{custom_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#show-unstable
     *
     * **Parameters:**
     *
     * * `custom_collection_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_custom_collections_param_collection(
        &self,
        custom_collection_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/custom_collections/{}/json?{}",
                crate::progenitor_support::encode_path(custom_collection_id),
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
     * Updates an existing custom collection.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/custom_collections/{custom_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#update-unstable
     *
     * **Parameters:**
     *
     * * `custom_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_custom_collections_param_collection(
        &self,
        custom_collection_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/custom_collections/{}/json",
                crate::progenitor_support::encode_path(custom_collection_id),
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
     * Deletes a custom collection.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/custom_collections/{custom_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/customcollection#destroy-unstable
     *
     * **Parameters:**
     *
     * * `custom_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_custom_collections_param_collection(
        &self,
        custom_collection_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/custom_collections/{}/json",
                crate::progenitor_support::encode_path(custom_collection_id),
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
     * Retrieves a list of products. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/products.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#index-2020-01
     *
     * **Parameters:**
     *
     * * `ids: &str` -- Return only products specified by a comma-separated list of product IDs.
     * * `limit: &str` -- Return up to this many results per page.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `title: &str` -- Filter results by product title.
     * * `vendor: &str` -- Filter results by product vendor.
     * * `handle: &str` -- Filter results by product handle.
     * * `product_type: &str` -- Filter results by product type.
     * * `status: &str` -- Return products by their status.
     *                     (default: active)
     *                       
     *                           active: Show only active products.
     *                           archived: Show only archived products.
     *                           draft: Show only draft products.
     * * `collection_id: &str` -- Filter results by product collection ID.
     * * `created_at_min: &str` -- Show products created after date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show products created before date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show products last updated after date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show products last updated before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show products published after date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show products published before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Return products by their published status
     *                     (default: any)
     *                       
     *                           published: Show only published products.
     *                           unpublished: Show only unpublished products.
     *                           any: Show all products.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     * * `presentment_currencies: &str` -- Return presentment prices in only certain currencies, specified by a comma-separated list of ISO 4217 currency codes.
     */
    pub async fn deprecated_202001_get(
        &self,
        ids: &str,
        limit: &str,
        since_id: &str,
        title: &str,
        vendor: &str,
        handle: &str,
        product_type: &str,
        status: &str,
        collection_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        fields: &str,
        presentment_currencies: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !collection_id.is_empty() {
            query_args.push(("collection_id".to_string(), collection_id.to_string()));
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
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !presentment_currencies.is_empty() {
            query_args.push((
                "presentment_currencies".to_string(),
                presentment_currencies.to_string(),
            ));
        }
        if !product_type.is_empty() {
            query_args.push(("product_type".to_string(), product_type.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        if !vendor.is_empty() {
            query_args.push(("vendor".to_string(), vendor.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/products.json?{}", query_),
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
    * Creates a new product.
              If you want to set the product's SEO information, then you can use the following properties:

                metafields_global_title_tag: The name of the product used for SEO purposes. Generally added to the <meta name='title'> tag.
                metafields_global_description_tag: A description of the product used for SEO purposes. Generally added to the <meta name='description'> tag.
    *
    * This function performs a `POST` to the `/admin/api/2020-01/products.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/products/product#create-2020-01
    */
    pub async fn deprecated_202001_create(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-01/products.json", None);
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
     * Retrieves a count of products.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/products/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#count-2020-01
     *
     * **Parameters:**
     *
     * * `vendor: &str` -- Filter results by product vendor.
     * * `product_type: &str` -- Filter results by product type.
     * * `collection_id: &str` -- Filter results by collection ID.
     * * `created_at_min: &str` -- Show products created after date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show products created before date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show products last updated after date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show products last updated before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show products published after date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show products published before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Return products by their published status
     *                     (default: any)
     *                       
     *                           published: Show only published products.
     *                           unpublished: Show only unpublished products.
     *                           any: Show all products.
     */
    pub async fn deprecated_202001_get_count(
        &self,
        vendor: &str,
        product_type: &str,
        collection_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !collection_id.is_empty() {
            query_args.push(("collection_id".to_string(), collection_id.to_string()));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !product_type.is_empty() {
            query_args.push(("product_type".to_string(), product_type.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        if !vendor.is_empty() {
            query_args.push(("vendor".to_string(), vendor.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/products/count.json?{}", query_),
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
     * Retrieves a single product.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/products/{product_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#show-2020-01
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202001_get_param(
        &self,
        product_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/products/{}/json?{}",
                crate::progenitor_support::encode_path(product_id),
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
    * Updates a product and its variants and images.
              If you want to update the product's SEO information, then you can use the following properties:

                metafields_global_title_tag: The name of the product used for SEO purposes. Generally added to the <meta name='title'> tag.
                metafields_global_description_tag: A description of the product used for SEO purposes. Generally added to the <meta name='description'> tag.
    *
    * This function performs a `PUT` to the `/admin/api/2020-01/products/{product_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/products/product#update-2020-01
    *
    * **Parameters:**
    *
    * * `product_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202001_update_param(
        &self,
        product_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/products/{}/json",
                crate::progenitor_support::encode_path(product_id),
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
     * Deletes a product.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/products/{product_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_param(&self, product_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/products/{}/json",
                crate::progenitor_support::encode_path(product_id),
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
     * Retrieves a list of products. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/products.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#index-2020-04
     *
     * **Parameters:**
     *
     * * `ids: &str` -- Return only products specified by a comma-separated list of product IDs.
     * * `limit: &str` -- Return up to this many results per page.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `title: &str` -- Filter results by product title.
     * * `vendor: &str` -- Filter results by product vendor.
     * * `handle: &str` -- Filter results by product handle.
     * * `product_type: &str` -- Filter results by product type.
     * * `status: &str` -- Return products by their status.
     *                     (default: active)
     *                       
     *                           active: Show only active products.
     *                           archived: Show only archived products.
     *                           draft: Show only draft products.
     * * `collection_id: &str` -- Filter results by product collection ID.
     * * `created_at_min: &str` -- Show products created after date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show products created before date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show products last updated after date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show products last updated before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show products published after date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show products published before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Return products by their published status
     *                     (default: any)
     *                       
     *                           published: Show only published products.
     *                           unpublished: Show only unpublished products.
     *                           any: Show all products.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     * * `presentment_currencies: &str` -- Return presentment prices in only certain currencies, specified by a comma-separated list of ISO 4217 currency codes.
     */
    pub async fn deprecated_202004_get(
        &self,
        ids: &str,
        limit: &str,
        since_id: &str,
        title: &str,
        vendor: &str,
        handle: &str,
        product_type: &str,
        status: &str,
        collection_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        fields: &str,
        presentment_currencies: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !collection_id.is_empty() {
            query_args.push(("collection_id".to_string(), collection_id.to_string()));
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
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !presentment_currencies.is_empty() {
            query_args.push((
                "presentment_currencies".to_string(),
                presentment_currencies.to_string(),
            ));
        }
        if !product_type.is_empty() {
            query_args.push(("product_type".to_string(), product_type.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        if !vendor.is_empty() {
            query_args.push(("vendor".to_string(), vendor.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/products.json?{}", query_),
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
    * Creates a new product.
              If you want to set the product's SEO information, then you can use the following properties:

                metafields_global_title_tag: The name of the product used for SEO purposes. Generally added to the <meta name='title'> tag.
                metafields_global_description_tag: A description of the product used for SEO purposes. Generally added to the <meta name='description'> tag.
    *
    * This function performs a `POST` to the `/admin/api/2020-04/products.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/products/product#create-2020-04
    */
    pub async fn deprecated_202004_create(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-04/products.json", None);
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
     * Retrieves a count of products.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/products/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#count-2020-04
     *
     * **Parameters:**
     *
     * * `vendor: &str` -- Filter results by product vendor.
     * * `product_type: &str` -- Filter results by product type.
     * * `collection_id: &str` -- Filter results by collection ID.
     * * `created_at_min: &str` -- Show products created after date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show products created before date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show products last updated after date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show products last updated before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show products published after date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show products published before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Return products by their published status
     *                     (default: any)
     *                       
     *                           published: Show only published products.
     *                           unpublished: Show only unpublished products.
     *                           any: Show all products.
     */
    pub async fn deprecated_202004_get_count(
        &self,
        vendor: &str,
        product_type: &str,
        collection_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !collection_id.is_empty() {
            query_args.push(("collection_id".to_string(), collection_id.to_string()));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !product_type.is_empty() {
            query_args.push(("product_type".to_string(), product_type.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        if !vendor.is_empty() {
            query_args.push(("vendor".to_string(), vendor.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/products/count.json?{}", query_),
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
     * Retrieves a single product.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/products/{product_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#show-2020-04
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202004_get_param(
        &self,
        product_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/products/{}/json?{}",
                crate::progenitor_support::encode_path(product_id),
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
    * Updates a product and its variants and images.
              If you want to update the product's SEO information, then you can use the following properties:

                metafields_global_title_tag: The name of the product used for SEO purposes. Generally added to the <meta name='title'> tag.
                metafields_global_description_tag: A description of the product used for SEO purposes. Generally added to the <meta name='description'> tag.
    *
    * This function performs a `PUT` to the `/admin/api/2020-04/products/{product_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/products/product#update-2020-04
    *
    * **Parameters:**
    *
    * * `product_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202004_update_param(
        &self,
        product_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/products/{}/json",
                crate::progenitor_support::encode_path(product_id),
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
     * Deletes a product.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/products/{product_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_param(&self, product_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/products/{}/json",
                crate::progenitor_support::encode_path(product_id),
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
     * Retrieves a list of products. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/products.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#index-2020-07
     *
     * **Parameters:**
     *
     * * `ids: &str` -- Return only products specified by a comma-separated list of product IDs.
     * * `limit: &str` -- Return up to this many results per page.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `title: &str` -- Filter results by product title.
     * * `vendor: &str` -- Filter results by product vendor.
     * * `handle: &str` -- Filter results by product handle.
     * * `product_type: &str` -- Filter results by product type.
     * * `status: &str` -- Return products by their status.
     *                     (default: active)
     *                       
     *                           active: Show only active products.
     *                           archived: Show only archived products.
     *                           draft: Show only draft products.
     * * `collection_id: &str` -- Filter results by product collection ID.
     * * `created_at_min: &str` -- Show products created after date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show products created before date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show products last updated after date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show products last updated before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show products published after date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show products published before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Return products by their published status
     *                     (default: any)
     *                       
     *                           published: Show only published products.
     *                           unpublished: Show only unpublished products.
     *                           any: Show all products.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     * * `presentment_currencies: &str` -- Return presentment prices in only certain currencies, specified by a comma-separated list of ISO 4217 currency codes.
     */
    pub async fn deprecated_202007_get(
        &self,
        ids: &str,
        limit: &str,
        since_id: &str,
        title: &str,
        vendor: &str,
        handle: &str,
        product_type: &str,
        status: &str,
        collection_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        fields: &str,
        presentment_currencies: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !collection_id.is_empty() {
            query_args.push(("collection_id".to_string(), collection_id.to_string()));
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
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !presentment_currencies.is_empty() {
            query_args.push((
                "presentment_currencies".to_string(),
                presentment_currencies.to_string(),
            ));
        }
        if !product_type.is_empty() {
            query_args.push(("product_type".to_string(), product_type.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        if !vendor.is_empty() {
            query_args.push(("vendor".to_string(), vendor.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/products.json?{}", query_),
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
    * Creates a new product.
              If you want to set the product's SEO information, then you can use the following properties:

                metafields_global_title_tag: The name of the product used for SEO purposes. Generally added to the <meta name='title'> tag.
                metafields_global_description_tag: A description of the product used for SEO purposes. Generally added to the <meta name='description'> tag.
    *
    * This function performs a `POST` to the `/admin/api/2020-07/products.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/products/product#create-2020-07
    */
    pub async fn deprecated_202007_create(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-07/products.json", None);
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
     * Retrieves a count of products.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/products/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#count-2020-07
     *
     * **Parameters:**
     *
     * * `vendor: &str` -- Filter results by product vendor.
     * * `product_type: &str` -- Filter results by product type.
     * * `collection_id: &str` -- Filter results by collection ID.
     * * `created_at_min: &str` -- Show products created after date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show products created before date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show products last updated after date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show products last updated before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show products published after date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show products published before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Return products by their published status
     *                     (default: any)
     *                       
     *                           published: Show only published products.
     *                           unpublished: Show only unpublished products.
     *                           any: Show all products.
     */
    pub async fn deprecated_202007_get_count(
        &self,
        vendor: &str,
        product_type: &str,
        collection_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !collection_id.is_empty() {
            query_args.push(("collection_id".to_string(), collection_id.to_string()));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !product_type.is_empty() {
            query_args.push(("product_type".to_string(), product_type.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        if !vendor.is_empty() {
            query_args.push(("vendor".to_string(), vendor.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/products/count.json?{}", query_),
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
     * Retrieves a single product.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/products/{product_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#show-2020-07
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202007_get_param(
        &self,
        product_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/products/{}/json?{}",
                crate::progenitor_support::encode_path(product_id),
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
    * Updates a product and its variants and images.
              If you want to update the product's SEO information, then you can use the following properties:

                metafields_global_title_tag: The name of the product used for SEO purposes. Generally added to the <meta name='title'> tag.
                metafields_global_description_tag: A description of the product used for SEO purposes. Generally added to the <meta name='description'> tag.
    *
    * This function performs a `PUT` to the `/admin/api/2020-07/products/{product_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/products/product#update-2020-07
    *
    * **Parameters:**
    *
    * * `product_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202007_update_param(
        &self,
        product_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/products/{}/json",
                crate::progenitor_support::encode_path(product_id),
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
     * Deletes a product.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/products/{product_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_param(&self, product_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/products/{}/json",
                crate::progenitor_support::encode_path(product_id),
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
     * Retrieves a list of products. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/products.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#index-2020-10
     *
     * **Parameters:**
     *
     * * `ids: &str` -- Return only products specified by a comma-separated list of product IDs.
     * * `limit: &str` -- Return up to this many results per page.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `title: &str` -- Filter results by product title.
     * * `vendor: &str` -- Filter results by product vendor.
     * * `handle: &str` -- Filter results by product handle.
     * * `product_type: &str` -- Filter results by product type.
     * * `status: &str` -- Return products by their status.
     *                     (default: active)
     *                       
     *                           active: Show only active products.
     *                           archived: Show only archived products.
     *                           draft: Show only draft products.
     * * `collection_id: &str` -- Filter results by product collection ID.
     * * `created_at_min: &str` -- Show products created after date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show products created before date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show products last updated after date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show products last updated before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show products published after date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show products published before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Return products by their published status
     *                     (default: any)
     *                       
     *                           published: Show only published products.
     *                           unpublished: Show only unpublished products.
     *                           any: Show all products.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     * * `presentment_currencies: &str` -- Return presentment prices in only certain currencies, specified by a comma-separated list of ISO 4217 currency codes.
     */
    pub async fn get(
        &self,
        ids: &str,
        limit: &str,
        since_id: &str,
        title: &str,
        vendor: &str,
        handle: &str,
        product_type: &str,
        status: &str,
        collection_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        fields: &str,
        presentment_currencies: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !collection_id.is_empty() {
            query_args.push(("collection_id".to_string(), collection_id.to_string()));
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
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !presentment_currencies.is_empty() {
            query_args.push((
                "presentment_currencies".to_string(),
                presentment_currencies.to_string(),
            ));
        }
        if !product_type.is_empty() {
            query_args.push(("product_type".to_string(), product_type.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        if !vendor.is_empty() {
            query_args.push(("vendor".to_string(), vendor.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/products.json?{}", query_),
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
    * Creates a new product.
              If you want to set the product's SEO information, then you can use the following properties:

                metafields_global_title_tag: The name of the product used for SEO purposes. Generally added to the <meta name='title'> tag.
                metafields_global_description_tag: A description of the product used for SEO purposes. Generally added to the <meta name='description'> tag.
    *
    * This function performs a `POST` to the `/admin/api/2020-10/products.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/products/product#create-2020-10
    */
    pub async fn create(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-10/products.json", None);
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
     * Retrieves a count of products.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/products/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#count-2020-10
     *
     * **Parameters:**
     *
     * * `vendor: &str` -- Filter results by product vendor.
     * * `product_type: &str` -- Filter results by product type.
     * * `collection_id: &str` -- Filter results by collection ID.
     * * `created_at_min: &str` -- Show products created after date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show products created before date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show products last updated after date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show products last updated before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show products published after date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show products published before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Return products by their published status
     *                     (default: any)
     *                       
     *                           published: Show only published products.
     *                           unpublished: Show only unpublished products.
     *                           any: Show all products.
     */
    pub async fn get_count(
        &self,
        vendor: &str,
        product_type: &str,
        collection_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !collection_id.is_empty() {
            query_args.push(("collection_id".to_string(), collection_id.to_string()));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !product_type.is_empty() {
            query_args.push(("product_type".to_string(), product_type.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        if !vendor.is_empty() {
            query_args.push(("vendor".to_string(), vendor.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/products/count.json?{}", query_),
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
     * Retrieves a single product.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/products/{product_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#show-2020-10
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn get_param(&self, product_id: &str, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/products/{}/json?{}",
                crate::progenitor_support::encode_path(product_id),
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
    * Updates a product and its variants and images.
              If you want to update the product's SEO information, then you can use the following properties:

                metafields_global_title_tag: The name of the product used for SEO purposes. Generally added to the <meta name='title'> tag.
                metafields_global_description_tag: A description of the product used for SEO purposes. Generally added to the <meta name='description'> tag.
    *
    * This function performs a `PUT` to the `/admin/api/2020-10/products/{product_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/products/product#update-2020-10
    *
    * **Parameters:**
    *
    * * `product_id: &str` -- storefront_access_token_id.
    */
    pub async fn update_param(
        &self,
        product_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/products/{}/json",
                crate::progenitor_support::encode_path(product_id),
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
     * Deletes a product.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/products/{product_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_param(&self, product_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/products/{}/json",
                crate::progenitor_support::encode_path(product_id),
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
     * Retrieves a list of products. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/products.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#index-2021-01
     *
     * **Parameters:**
     *
     * * `ids: &str` -- Return only products specified by a comma-separated list of product IDs.
     * * `limit: &str` -- Return up to this many results per page.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `title: &str` -- Filter results by product title.
     * * `vendor: &str` -- Filter results by product vendor.
     * * `handle: &str` -- Filter results by product handle.
     * * `product_type: &str` -- Filter results by product type.
     * * `status: &str` -- Return products by their status.
     *                     (default: active)
     *                       
     *                           active: Show only active products.
     *                           archived: Show only archived products.
     *                           draft: Show only draft products.
     * * `collection_id: &str` -- Filter results by product collection ID.
     * * `created_at_min: &str` -- Show products created after date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show products created before date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show products last updated after date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show products last updated before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show products published after date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show products published before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Return products by their published status
     *                     (default: any)
     *                       
     *                           published: Show only published products.
     *                           unpublished: Show only unpublished products.
     *                           any: Show all products.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     * * `presentment_currencies: &str` -- Return presentment prices in only certain currencies, specified by a comma-separated list of ISO 4217 currency codes.
     */
    pub async fn deprecated_202101_get(
        &self,
        ids: &str,
        limit: &str,
        since_id: &str,
        title: &str,
        vendor: &str,
        handle: &str,
        product_type: &str,
        status: &str,
        collection_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        fields: &str,
        presentment_currencies: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !collection_id.is_empty() {
            query_args.push(("collection_id".to_string(), collection_id.to_string()));
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
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !presentment_currencies.is_empty() {
            query_args.push((
                "presentment_currencies".to_string(),
                presentment_currencies.to_string(),
            ));
        }
        if !product_type.is_empty() {
            query_args.push(("product_type".to_string(), product_type.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        if !vendor.is_empty() {
            query_args.push(("vendor".to_string(), vendor.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/products.json?{}", query_),
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
    * Creates a new product.
              If you want to set the product's SEO information, then you can use the following properties:

                metafields_global_title_tag: The name of the product used for SEO purposes. Generally added to the <meta name='title'> tag.
                metafields_global_description_tag: A description of the product used for SEO purposes. Generally added to the <meta name='description'> tag.
    *
    * This function performs a `POST` to the `/admin/api/2021-01/products.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/products/product#create-2021-01
    */
    pub async fn deprecated_202101_create(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2021-01/products.json", None);
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
     * Retrieves a count of products.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/products/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#count-2021-01
     *
     * **Parameters:**
     *
     * * `vendor: &str` -- Filter results by product vendor.
     * * `product_type: &str` -- Filter results by product type.
     * * `collection_id: &str` -- Filter results by collection ID.
     * * `created_at_min: &str` -- Show products created after date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show products created before date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show products last updated after date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show products last updated before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show products published after date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show products published before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Return products by their published status
     *                     (default: any)
     *                       
     *                           published: Show only published products.
     *                           unpublished: Show only unpublished products.
     *                           any: Show all products.
     */
    pub async fn deprecated_202101_get_count(
        &self,
        vendor: &str,
        product_type: &str,
        collection_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !collection_id.is_empty() {
            query_args.push(("collection_id".to_string(), collection_id.to_string()));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !product_type.is_empty() {
            query_args.push(("product_type".to_string(), product_type.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        if !vendor.is_empty() {
            query_args.push(("vendor".to_string(), vendor.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/products/count.json?{}", query_),
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
     * Retrieves a single product.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/products/{product_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#show-2021-01
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202101_get_param(
        &self,
        product_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/products/{}/json?{}",
                crate::progenitor_support::encode_path(product_id),
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
    * Updates a product and its variants and images.
              If you want to update the product's SEO information, then you can use the following properties:

                metafields_global_title_tag: The name of the product used for SEO purposes. Generally added to the <meta name='title'> tag.
                metafields_global_description_tag: A description of the product used for SEO purposes. Generally added to the <meta name='description'> tag.
    *
    * This function performs a `PUT` to the `/admin/api/2021-01/products/{product_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/products/product#update-2021-01
    *
    * **Parameters:**
    *
    * * `product_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202101_update_param(
        &self,
        product_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/products/{}/json",
                crate::progenitor_support::encode_path(product_id),
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
     * Deletes a product.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/products/{product_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_param(&self, product_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/products/{}/json",
                crate::progenitor_support::encode_path(product_id),
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
     * Retrieves a list of products. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/products.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#index-unstable
     *
     * **Parameters:**
     *
     * * `ids: &str` -- Return only products specified by a comma-separated list of product IDs.
     * * `limit: &str` -- Return up to this many results per page.
     *                     (default: 50, maximum: 250).
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `title: &str` -- Filter results by product title.
     * * `vendor: &str` -- Filter results by product vendor.
     * * `handle: &str` -- Filter results by product handle.
     * * `product_type: &str` -- Filter results by product type.
     * * `status: &str` -- Return products by their status.
     *                     (default: active)
     *                       
     *                           active: Show only active products.
     *                           archived: Show only archived products.
     *                           draft: Show only draft products.
     * * `collection_id: &str` -- Filter results by product collection ID.
     * * `created_at_min: &str` -- Show products created after date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show products created before date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show products last updated after date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show products last updated before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show products published after date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show products published before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Return products by their published status
     *                     (default: any)
     *                       
     *                           published: Show only published products.
     *                           unpublished: Show only unpublished products.
     *                           any: Show all products.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     * * `presentment_currencies: &str` -- Return presentment prices in only certain currencies, specified by a comma-separated list of ISO 4217 currency codes.
     */
    pub async fn deprecated_unstable_get(
        &self,
        ids: &str,
        limit: &str,
        since_id: &str,
        title: &str,
        vendor: &str,
        handle: &str,
        product_type: &str,
        status: &str,
        collection_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        fields: &str,
        presentment_currencies: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !collection_id.is_empty() {
            query_args.push(("collection_id".to_string(), collection_id.to_string()));
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
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !presentment_currencies.is_empty() {
            query_args.push((
                "presentment_currencies".to_string(),
                presentment_currencies.to_string(),
            ));
        }
        if !product_type.is_empty() {
            query_args.push(("product_type".to_string(), product_type.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        if !vendor.is_empty() {
            query_args.push(("vendor".to_string(), vendor.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/products.json?{}", query_),
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
    * Creates a new product.
              If you want to set the product's SEO information, then you can use the following properties:

                metafields_global_title_tag: The name of the product used for SEO purposes. Generally added to the <meta name='title'> tag.
                metafields_global_description_tag: A description of the product used for SEO purposes. Generally added to the <meta name='description'> tag.
    *
    * This function performs a `POST` to the `/admin/api/unstable/products.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/products/product#create-unstable
    */
    pub async fn deprecated_unstable_create(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/unstable/products.json", None);
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
     * Retrieves a count of products.
     *
     * This function performs a `GET` to the `/admin/api/unstable/products/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#count-unstable
     *
     * **Parameters:**
     *
     * * `vendor: &str` -- Filter results by product vendor.
     * * `product_type: &str` -- Filter results by product type.
     * * `collection_id: &str` -- Filter results by collection ID.
     * * `created_at_min: &str` -- Show products created after date. (format: 2014-04-25T16:15:47-04:00).
     * * `created_at_max: &str` -- Show products created before date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_min: &str` -- Show products last updated after date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show products last updated before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show products published after date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show products published before date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Return products by their published status
     *                     (default: any)
     *                       
     *                           published: Show only published products.
     *                           unpublished: Show only unpublished products.
     *                           any: Show all products.
     */
    pub async fn deprecated_unstable_get_count(
        &self,
        vendor: &str,
        product_type: &str,
        collection_id: &str,
        created_at_min: &str,
        created_at_max: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !collection_id.is_empty() {
            query_args.push(("collection_id".to_string(), collection_id.to_string()));
        }
        if !created_at_max.is_empty() {
            query_args.push(("created_at_max".to_string(), created_at_max.to_string()));
        }
        if !created_at_min.is_empty() {
            query_args.push(("created_at_min".to_string(), created_at_min.to_string()));
        }
        if !product_type.is_empty() {
            query_args.push(("product_type".to_string(), product_type.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        if !vendor.is_empty() {
            query_args.push(("vendor".to_string(), vendor.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/products/count.json?{}", query_),
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
     * Retrieves a single product.
     *
     * This function performs a `GET` to the `/admin/api/unstable/products/{product_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#show-unstable
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_unstable_get_param(
        &self,
        product_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/products/{}/json?{}",
                crate::progenitor_support::encode_path(product_id),
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
    * Updates a product and its variants and images.
              If you want to update the product's SEO information, then you can use the following properties:

                metafields_global_title_tag: The name of the product used for SEO purposes. Generally added to the <meta name='title'> tag.
                metafields_global_description_tag: A description of the product used for SEO purposes. Generally added to the <meta name='description'> tag.
    *
    * This function performs a `PUT` to the `/admin/api/unstable/products/{product_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/products/product#update-unstable
    *
    * **Parameters:**
    *
    * * `product_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_unstable_update_param(
        &self,
        product_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/products/{}/json",
                crate::progenitor_support::encode_path(product_id),
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
     * Deletes a product.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/products/{product_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product#destroy-unstable
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_param(&self, product_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/products/{}/json",
                crate::progenitor_support::encode_path(product_id),
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
     * Get all product images.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/products/{product_id}/images.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#index-2020-01
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202001_get_param_image(
        &self,
        product_id: &str,
        since_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/products/{}/images.json?{}",
                crate::progenitor_support::encode_path(product_id),
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
     * Create a new product image.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/products/{product_id}/images.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#create-2020-01
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_param_images(
        &self,
        product_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/products/{}/images.json",
                crate::progenitor_support::encode_path(product_id),
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
     * Get a count of all product images.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/products/{product_id}/images/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#count-2020-01
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     */
    pub async fn deprecated_202001_get_param_images_count(
        &self,
        product_id: &str,
        since_id: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/products/{}/images/count.json?{}",
                crate::progenitor_support::encode_path(product_id),
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
     * Get a single product image by id.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/products/{product_id}/images/{image_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#show-2020-01
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `image_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202001_get_param_images_image(
        &self,
        product_id: &str,
        image_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/products/{}/images/{}/json?{}",
                crate::progenitor_support::encode_path(product_id),
                crate::progenitor_support::encode_path(image_id),
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
     * Modify an existing product image.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/products/{product_id}/images/{image_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#update-2020-01
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `image_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_param_images_image(
        &self,
        product_id: &str,
        image_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/products/{}/images/{}/json",
                crate::progenitor_support::encode_path(product_id),
                crate::progenitor_support::encode_path(image_id),
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
     * This function performs a `DELETE` to the `/admin/api/2020-01/products/{product_id}/images/{image_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `image_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_param_images_image(
        &self,
        product_id: &str,
        image_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/products/{}/images/{}/json",
                crate::progenitor_support::encode_path(product_id),
                crate::progenitor_support::encode_path(image_id),
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
     * Get all product images.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/products/{product_id}/images.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#index-2020-04
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202004_get_param_image(
        &self,
        product_id: &str,
        since_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/products/{}/images.json?{}",
                crate::progenitor_support::encode_path(product_id),
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
     * Create a new product image.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/products/{product_id}/images.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#create-2020-04
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_create_param_images(
        &self,
        product_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/products/{}/images.json",
                crate::progenitor_support::encode_path(product_id),
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
     * Get a count of all product images.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/products/{product_id}/images/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#count-2020-04
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     */
    pub async fn deprecated_202004_get_param_images_count(
        &self,
        product_id: &str,
        since_id: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/products/{}/images/count.json?{}",
                crate::progenitor_support::encode_path(product_id),
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
     * Get a single product image by id.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/products/{product_id}/images/{image_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#show-2020-04
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `image_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202004_get_param_images_image(
        &self,
        product_id: &str,
        image_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/products/{}/images/{}/json?{}",
                crate::progenitor_support::encode_path(product_id),
                crate::progenitor_support::encode_path(image_id),
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
     * Modify an existing product image.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/products/{product_id}/images/{image_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#update-2020-04
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `image_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_param_images_image(
        &self,
        product_id: &str,
        image_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/products/{}/images/{}/json",
                crate::progenitor_support::encode_path(product_id),
                crate::progenitor_support::encode_path(image_id),
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
     * This function performs a `DELETE` to the `/admin/api/2020-04/products/{product_id}/images/{image_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `image_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_param_images_image(
        &self,
        product_id: &str,
        image_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/products/{}/images/{}/json",
                crate::progenitor_support::encode_path(product_id),
                crate::progenitor_support::encode_path(image_id),
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
     * Get all product images.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/products/{product_id}/images.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#index-2020-07
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202007_get_param_image(
        &self,
        product_id: &str,
        since_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/products/{}/images.json?{}",
                crate::progenitor_support::encode_path(product_id),
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
     * Create a new product image.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/products/{product_id}/images.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#create-2020-07
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_create_param_images(
        &self,
        product_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/products/{}/images.json",
                crate::progenitor_support::encode_path(product_id),
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
     * Get a count of all product images.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/products/{product_id}/images/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#count-2020-07
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     */
    pub async fn deprecated_202007_get_param_images_count(
        &self,
        product_id: &str,
        since_id: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/products/{}/images/count.json?{}",
                crate::progenitor_support::encode_path(product_id),
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
     * Get a single product image by id.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/products/{product_id}/images/{image_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#show-2020-07
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `image_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202007_get_param_images_image(
        &self,
        product_id: &str,
        image_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/products/{}/images/{}/json?{}",
                crate::progenitor_support::encode_path(product_id),
                crate::progenitor_support::encode_path(image_id),
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
     * Modify an existing product image.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/products/{product_id}/images/{image_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#update-2020-07
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `image_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_param_images_image(
        &self,
        product_id: &str,
        image_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/products/{}/images/{}/json",
                crate::progenitor_support::encode_path(product_id),
                crate::progenitor_support::encode_path(image_id),
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
     * This function performs a `DELETE` to the `/admin/api/2020-07/products/{product_id}/images/{image_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `image_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_param_images_image(
        &self,
        product_id: &str,
        image_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/products/{}/images/{}/json",
                crate::progenitor_support::encode_path(product_id),
                crate::progenitor_support::encode_path(image_id),
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
     * Get all product images.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/products/{product_id}/images.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#index-2020-10
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- comma-separated list of fields to include in the response.
     */
    pub async fn get_param_image(
        &self,
        product_id: &str,
        since_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/products/{}/images.json?{}",
                crate::progenitor_support::encode_path(product_id),
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
     * Create a new product image.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/products/{product_id}/images.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#create-2020-10
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     */
    pub async fn create_param_images(
        &self,
        product_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/products/{}/images.json",
                crate::progenitor_support::encode_path(product_id),
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
     * Get a count of all product images.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/products/{product_id}/images/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#count-2020-10
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     */
    pub async fn get_param_images_count(
        &self,
        product_id: &str,
        since_id: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/products/{}/images/count.json?{}",
                crate::progenitor_support::encode_path(product_id),
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
     * Get a single product image by id.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/products/{product_id}/images/{image_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#show-2020-10
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `image_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- comma-separated list of fields to include in the response.
     */
    pub async fn get_param_images_image(
        &self,
        product_id: &str,
        image_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/products/{}/images/{}/json?{}",
                crate::progenitor_support::encode_path(product_id),
                crate::progenitor_support::encode_path(image_id),
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
     * Modify an existing product image.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/products/{product_id}/images/{image_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#update-2020-10
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `image_id: &str` -- storefront_access_token_id.
     */
    pub async fn update_param_images_image(
        &self,
        product_id: &str,
        image_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/products/{}/images/{}/json",
                crate::progenitor_support::encode_path(product_id),
                crate::progenitor_support::encode_path(image_id),
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
     * This function performs a `DELETE` to the `/admin/api/2020-10/products/{product_id}/images/{image_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `image_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_param_images_image(
        &self,
        product_id: &str,
        image_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/products/{}/images/{}/json",
                crate::progenitor_support::encode_path(product_id),
                crate::progenitor_support::encode_path(image_id),
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
     * Get all product images.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/products/{product_id}/images.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#index-2021-01
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202101_get_param_image(
        &self,
        product_id: &str,
        since_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/products/{}/images.json?{}",
                crate::progenitor_support::encode_path(product_id),
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
     * Create a new product image.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/products/{product_id}/images.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#create-2021-01
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_create_param_images(
        &self,
        product_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/products/{}/images.json",
                crate::progenitor_support::encode_path(product_id),
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
     * Get a count of all product images.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/products/{product_id}/images/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#count-2021-01
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     */
    pub async fn deprecated_202101_get_param_images_count(
        &self,
        product_id: &str,
        since_id: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/products/{}/images/count.json?{}",
                crate::progenitor_support::encode_path(product_id),
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
     * Get a single product image by id.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/products/{product_id}/images/{image_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#show-2021-01
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `image_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202101_get_param_images_image(
        &self,
        product_id: &str,
        image_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/products/{}/images/{}/json?{}",
                crate::progenitor_support::encode_path(product_id),
                crate::progenitor_support::encode_path(image_id),
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
     * Modify an existing product image.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/products/{product_id}/images/{image_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#update-2021-01
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `image_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_param_images_image(
        &self,
        product_id: &str,
        image_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/products/{}/images/{}/json",
                crate::progenitor_support::encode_path(product_id),
                crate::progenitor_support::encode_path(image_id),
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
     * This function performs a `DELETE` to the `/admin/api/2021-01/products/{product_id}/images/{image_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `image_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_param_images_image(
        &self,
        product_id: &str,
        image_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/products/{}/images/{}/json",
                crate::progenitor_support::encode_path(product_id),
                crate::progenitor_support::encode_path(image_id),
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
     * Get all product images.
     *
     * This function performs a `GET` to the `/admin/api/unstable/products/{product_id}/images.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#index-unstable
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_unstable_get_param_image(
        &self,
        product_id: &str,
        since_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/products/{}/images.json?{}",
                crate::progenitor_support::encode_path(product_id),
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
     * Create a new product image.
     *
     * This function performs a `POST` to the `/admin/api/unstable/products/{product_id}/images.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#create-unstable
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_create_param_images(
        &self,
        product_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/products/{}/images.json",
                crate::progenitor_support::encode_path(product_id),
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
     * Get a count of all product images.
     *
     * This function performs a `GET` to the `/admin/api/unstable/products/{product_id}/images/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#count-unstable
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     */
    pub async fn deprecated_unstable_get_param_images_count(
        &self,
        product_id: &str,
        since_id: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/products/{}/images/count.json?{}",
                crate::progenitor_support::encode_path(product_id),
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
     * Get a single product image by id.
     *
     * This function performs a `GET` to the `/admin/api/unstable/products/{product_id}/images/{image_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#show-unstable
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `image_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_unstable_get_param_images_image(
        &self,
        product_id: &str,
        image_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/products/{}/images/{}/json?{}",
                crate::progenitor_support::encode_path(product_id),
                crate::progenitor_support::encode_path(image_id),
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
     * Modify an existing product image.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/products/{product_id}/images/{image_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#update-unstable
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `image_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_param_images_image(
        &self,
        product_id: &str,
        image_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/products/{}/images/{}/json",
                crate::progenitor_support::encode_path(product_id),
                crate::progenitor_support::encode_path(image_id),
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
     * This function performs a `DELETE` to the `/admin/api/unstable/products/{product_id}/images/{image_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/product-image#destroy-unstable
     *
     * **Parameters:**
     *
     * * `product_id: &str` -- storefront_access_token_id.
     * * `image_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_param_images_image(
        &self,
        product_id: &str,
        image_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/products/{}/images/{}/json",
                crate::progenitor_support::encode_path(product_id),
                crate::progenitor_support::encode_path(image_id),
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
     * Retrieves a list of smart collections. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/smart_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#index-2020-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The number of results to show.
     *                     (default: 50, maximum: 250).
     * * `ids: &str` -- Show only the smart collections specified by a comma-separated list of IDs.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `title: &str` -- Show smart collections with the specified title.
     * * `product_id: &str` -- Show smart collections that includes the specified product.
     * * `handle: &str` -- Filter results by smart collection handle.
     * * `updated_at_min: &str` -- Show smart collections last updated after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show smart collections last updated before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show smart collections published after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show smart collections published before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Filter results based on the published status of smart collections.
     *                     (default: any)
     *                       
     *                           published: Show only published smart collections.
     *                           unpublished: Show only unpublished smart collections.
     *                           any: Show all smart collections.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_smart_collection(
        &self,
        limit: &str,
        ids: &str,
        since_id: &str,
        title: &str,
        product_id: &str,
        handle: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/smart_collections.json?{}", query_),
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
     * Creates a new smart collection using the specified rules.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/smart_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#create-2020-01
     */
    pub async fn deprecated_202001_create_smart_collections(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-01/smart_collections.json", None);
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
     * Retrieves a count of smart collections.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/smart_collections/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#count-2020-01
     *
     * **Parameters:**
     *
     * * `title: &str` -- Show smart collections with the specified title.
     * * `product_id: &str` -- Show smart collections that include the specified product.
     * * `updated_at_min: &str` -- Show smart collections last updated after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show smart collections last updated before this date.  (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show smart collections published after this date.  (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show smart collections published before this date.  (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Filter results based on the published status of smart collections.
     *                     (default: any)
     *                       
     *                           published: Show only published smart collections.
     *                           unpublished: Show only unpublished smart collections.
     *                           any: Show all smart collections.
     */
    pub async fn deprecated_202001_get_smart_collections_count(
        &self,
        title: &str,
        product_id: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/smart_collections/count.json?{}", query_),
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
     * Retrieves a single smart collection.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/smart_collections/{smart_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#show-2020-01
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_smart_collections_param_collection(
        &self,
        smart_collection_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/smart_collections/{}/json?{}",
                crate::progenitor_support::encode_path(smart_collection_id),
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
     * Updates an existing smart collection.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/smart_collections/{smart_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#update-2020-01
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_smart_collections_param_collection(
        &self,
        smart_collection_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/smart_collections/{}/json",
                crate::progenitor_support::encode_path(smart_collection_id),
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
     * Removes a smart collection.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/smart_collections/{smart_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_smart_collections_param_collection(
        &self,
        smart_collection_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/smart_collections/{}/json",
                crate::progenitor_support::encode_path(smart_collection_id),
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
     * Updates the ordering type of products in a smart collection.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/smart_collections/{smart_collection_id}/order.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#order-2020-01
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     * * `products: &str` -- An array of product IDs, in the order that you want them to appear at the top of the collection. When products is specified but empty, any previously sorted products are cleared.
     * * `sort_order: &str` -- The type of sorting to apply. Valid values are listed in the Properties section above.
     *                     (default: (current value)).
     * * `products: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202001_update_smart_collections_param_collection_order(
        &self,
        smart_collection_id: &str,
        products: &str,
        sort_order: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !products.is_empty() {
            query_args.push(("products".to_string(), products.to_string()));
        }
        if !sort_order.is_empty() {
            query_args.push(("sort_order".to_string(), sort_order.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/smart_collections/{}/order.json?{}",
                crate::progenitor_support::encode_path(smart_collection_id),
                query_
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
     * Retrieves a list of smart collections. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/smart_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#index-2020-04
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The number of results to show.
     *                     (default: 50, maximum: 250).
     * * `ids: &str` -- Show only the smart collections specified by a comma-separated list of IDs.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `title: &str` -- Show smart collections with the specified title.
     * * `product_id: &str` -- Show smart collections that includes the specified product.
     * * `handle: &str` -- Filter results by smart collection handle.
     * * `updated_at_min: &str` -- Show smart collections last updated after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show smart collections last updated before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show smart collections published after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show smart collections published before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Filter results based on the published status of smart collections.
     *                     (default: any)
     *                       
     *                           published: Show only published smart collections.
     *                           unpublished: Show only unpublished smart collections.
     *                           any: Show all smart collections.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_smart_collection(
        &self,
        limit: &str,
        ids: &str,
        since_id: &str,
        title: &str,
        product_id: &str,
        handle: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/smart_collections.json?{}", query_),
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
     * Creates a new smart collection using the specified rules.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/smart_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#create-2020-04
     */
    pub async fn deprecated_202004_create_smart_collections(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-04/smart_collections.json", None);
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
     * Retrieves a count of smart collections.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/smart_collections/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#count-2020-04
     *
     * **Parameters:**
     *
     * * `title: &str` -- Show smart collections with the specified title.
     * * `product_id: &str` -- Show smart collections that include the specified product.
     * * `updated_at_min: &str` -- Show smart collections last updated after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show smart collections last updated before this date.  (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show smart collections published after this date.  (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show smart collections published before this date.  (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Filter results based on the published status of smart collections.
     *                     (default: any)
     *                       
     *                           published: Show only published smart collections.
     *                           unpublished: Show only unpublished smart collections.
     *                           any: Show all smart collections.
     */
    pub async fn deprecated_202004_get_smart_collections_count(
        &self,
        title: &str,
        product_id: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/smart_collections/count.json?{}", query_),
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
     * Retrieves a single smart collection.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/smart_collections/{smart_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#show-2020-04
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_smart_collections_param_collection(
        &self,
        smart_collection_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/smart_collections/{}/json?{}",
                crate::progenitor_support::encode_path(smart_collection_id),
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
     * Updates an existing smart collection.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/smart_collections/{smart_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#update-2020-04
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_smart_collections_param_collection(
        &self,
        smart_collection_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/smart_collections/{}/json",
                crate::progenitor_support::encode_path(smart_collection_id),
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
     * Removes a smart collection.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/smart_collections/{smart_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_smart_collections_param_collection(
        &self,
        smart_collection_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/smart_collections/{}/json",
                crate::progenitor_support::encode_path(smart_collection_id),
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
     * Updates the ordering type of products in a smart collection.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/smart_collections/{smart_collection_id}/order.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#order-2020-04
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     * * `products: &str` -- An array of product IDs, in the order that you want them to appear at the top of the collection. When products is specified but empty, any previously sorted products are cleared.
     * * `sort_order: &str` -- The type of sorting to apply. Valid values are listed in the Properties section above.
     *                     (default: (current value)).
     * * `products: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202004_update_smart_collections_param_collection_order(
        &self,
        smart_collection_id: &str,
        products: &str,
        sort_order: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !products.is_empty() {
            query_args.push(("products".to_string(), products.to_string()));
        }
        if !sort_order.is_empty() {
            query_args.push(("sort_order".to_string(), sort_order.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/smart_collections/{}/order.json?{}",
                crate::progenitor_support::encode_path(smart_collection_id),
                query_
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
     * Retrieves a list of smart collections. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/smart_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#index-2020-07
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The number of results to show.
     *                     (default: 50, maximum: 250).
     * * `ids: &str` -- Show only the smart collections specified by a comma-separated list of IDs.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `title: &str` -- Show smart collections with the specified title.
     * * `product_id: &str` -- Show smart collections that includes the specified product.
     * * `handle: &str` -- Filter results by smart collection handle.
     * * `updated_at_min: &str` -- Show smart collections last updated after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show smart collections last updated before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show smart collections published after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show smart collections published before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Filter results based on the published status of smart collections.
     *                     (default: any)
     *                       
     *                           published: Show only published smart collections.
     *                           unpublished: Show only unpublished smart collections.
     *                           any: Show all smart collections.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_smart_collection(
        &self,
        limit: &str,
        ids: &str,
        since_id: &str,
        title: &str,
        product_id: &str,
        handle: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/smart_collections.json?{}", query_),
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
     * Creates a new smart collection using the specified rules.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/smart_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#create-2020-07
     */
    pub async fn deprecated_202007_create_smart_collections(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-07/smart_collections.json", None);
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
     * Retrieves a count of smart collections.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/smart_collections/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#count-2020-07
     *
     * **Parameters:**
     *
     * * `title: &str` -- Show smart collections with the specified title.
     * * `product_id: &str` -- Show smart collections that include the specified product.
     * * `updated_at_min: &str` -- Show smart collections last updated after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show smart collections last updated before this date.  (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show smart collections published after this date.  (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show smart collections published before this date.  (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Filter results based on the published status of smart collections.
     *                     (default: any)
     *                       
     *                           published: Show only published smart collections.
     *                           unpublished: Show only unpublished smart collections.
     *                           any: Show all smart collections.
     */
    pub async fn deprecated_202007_get_smart_collections_count(
        &self,
        title: &str,
        product_id: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/smart_collections/count.json?{}", query_),
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
     * Retrieves a single smart collection.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/smart_collections/{smart_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#show-2020-07
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_smart_collections_param_collection(
        &self,
        smart_collection_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/smart_collections/{}/json?{}",
                crate::progenitor_support::encode_path(smart_collection_id),
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
     * Updates an existing smart collection.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/smart_collections/{smart_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#update-2020-07
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_smart_collections_param_collection(
        &self,
        smart_collection_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/smart_collections/{}/json",
                crate::progenitor_support::encode_path(smart_collection_id),
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
     * Removes a smart collection.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/smart_collections/{smart_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_smart_collections_param_collection(
        &self,
        smart_collection_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/smart_collections/{}/json",
                crate::progenitor_support::encode_path(smart_collection_id),
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
     * Updates the ordering type of products in a smart collection.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/smart_collections/{smart_collection_id}/order.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#order-2020-07
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     * * `products: &str` -- An array of product IDs, in the order that you want them to appear at the top of the collection. When products is specified but empty, any previously sorted products are cleared.
     * * `sort_order: &str` -- The type of sorting to apply. Valid values are listed in the Properties section above.
     *                     (default: (current value)).
     * * `products: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202007_update_smart_collections_param_collection_order(
        &self,
        smart_collection_id: &str,
        products: &str,
        sort_order: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !products.is_empty() {
            query_args.push(("products".to_string(), products.to_string()));
        }
        if !sort_order.is_empty() {
            query_args.push(("sort_order".to_string(), sort_order.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/smart_collections/{}/order.json?{}",
                crate::progenitor_support::encode_path(smart_collection_id),
                query_
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
     * Retrieves a list of smart collections. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/smart_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#index-2020-10
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The number of results to show.
     *                     (default: 50, maximum: 250).
     * * `ids: &str` -- Show only the smart collections specified by a comma-separated list of IDs.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `title: &str` -- Show smart collections with the specified title.
     * * `product_id: &str` -- Show smart collections that includes the specified product.
     * * `handle: &str` -- Filter results by smart collection handle.
     * * `updated_at_min: &str` -- Show smart collections last updated after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show smart collections last updated before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show smart collections published after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show smart collections published before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Filter results based on the published status of smart collections.
     *                     (default: any)
     *                       
     *                           published: Show only published smart collections.
     *                           unpublished: Show only unpublished smart collections.
     *                           any: Show all smart collections.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_smart_collection(
        &self,
        limit: &str,
        ids: &str,
        since_id: &str,
        title: &str,
        product_id: &str,
        handle: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/smart_collections.json?{}", query_),
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
     * Creates a new smart collection using the specified rules.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/smart_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#create-2020-10
     */
    pub async fn create_smart_collections(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-10/smart_collections.json", None);
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
     * Retrieves a count of smart collections.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/smart_collections/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#count-2020-10
     *
     * **Parameters:**
     *
     * * `title: &str` -- Show smart collections with the specified title.
     * * `product_id: &str` -- Show smart collections that include the specified product.
     * * `updated_at_min: &str` -- Show smart collections last updated after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show smart collections last updated before this date.  (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show smart collections published after this date.  (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show smart collections published before this date.  (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Filter results based on the published status of smart collections.
     *                     (default: any)
     *                       
     *                           published: Show only published smart collections.
     *                           unpublished: Show only unpublished smart collections.
     *                           any: Show all smart collections.
     */
    pub async fn get_smart_collections_count(
        &self,
        title: &str,
        product_id: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/smart_collections/count.json?{}", query_),
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
     * Retrieves a single smart collection.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/smart_collections/{smart_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#show-2020-10
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_smart_collections_param_collection(
        &self,
        smart_collection_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/smart_collections/{}/json?{}",
                crate::progenitor_support::encode_path(smart_collection_id),
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
     * Updates an existing smart collection.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/smart_collections/{smart_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#update-2020-10
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn update_smart_collections_param_collection(
        &self,
        smart_collection_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/smart_collections/{}/json",
                crate::progenitor_support::encode_path(smart_collection_id),
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
     * Removes a smart collection.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/smart_collections/{smart_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_smart_collections_param_collection(
        &self,
        smart_collection_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/smart_collections/{}/json",
                crate::progenitor_support::encode_path(smart_collection_id),
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
     * Updates the ordering type of products in a smart collection.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/smart_collections/{smart_collection_id}/order.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#order-2020-10
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     * * `products: &str` -- An array of product IDs, in the order that you want them to appear at the top of the collection. When products is specified but empty, any previously sorted products are cleared.
     * * `sort_order: &str` -- The type of sorting to apply. Valid values are listed in the Properties section above.
     *                     (default: (current value)).
     * * `products: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn update_smart_collections_param_collection_order(
        &self,
        smart_collection_id: &str,
        products: &str,
        sort_order: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !products.is_empty() {
            query_args.push(("products".to_string(), products.to_string()));
        }
        if !sort_order.is_empty() {
            query_args.push(("sort_order".to_string(), sort_order.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/smart_collections/{}/order.json?{}",
                crate::progenitor_support::encode_path(smart_collection_id),
                query_
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
     * Retrieves a list of smart collections. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/smart_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#index-2021-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The number of results to show.
     *                     (default: 50, maximum: 250).
     * * `ids: &str` -- Show only the smart collections specified by a comma-separated list of IDs.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `title: &str` -- Show smart collections with the specified title.
     * * `product_id: &str` -- Show smart collections that includes the specified product.
     * * `handle: &str` -- Filter results by smart collection handle.
     * * `updated_at_min: &str` -- Show smart collections last updated after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show smart collections last updated before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show smart collections published after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show smart collections published before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Filter results based on the published status of smart collections.
     *                     (default: any)
     *                       
     *                           published: Show only published smart collections.
     *                           unpublished: Show only unpublished smart collections.
     *                           any: Show all smart collections.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_smart_collection(
        &self,
        limit: &str,
        ids: &str,
        since_id: &str,
        title: &str,
        product_id: &str,
        handle: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/smart_collections.json?{}", query_),
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
     * Creates a new smart collection using the specified rules.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/smart_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#create-2021-01
     */
    pub async fn deprecated_202101_create_smart_collections(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2021-01/smart_collections.json", None);
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
     * Retrieves a count of smart collections.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/smart_collections/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#count-2021-01
     *
     * **Parameters:**
     *
     * * `title: &str` -- Show smart collections with the specified title.
     * * `product_id: &str` -- Show smart collections that include the specified product.
     * * `updated_at_min: &str` -- Show smart collections last updated after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show smart collections last updated before this date.  (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show smart collections published after this date.  (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show smart collections published before this date.  (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Filter results based on the published status of smart collections.
     *                     (default: any)
     *                       
     *                           published: Show only published smart collections.
     *                           unpublished: Show only unpublished smart collections.
     *                           any: Show all smart collections.
     */
    pub async fn deprecated_202101_get_smart_collections_count(
        &self,
        title: &str,
        product_id: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/smart_collections/count.json?{}", query_),
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
     * Retrieves a single smart collection.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/smart_collections/{smart_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#show-2021-01
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_smart_collections_param_collection(
        &self,
        smart_collection_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/smart_collections/{}/json?{}",
                crate::progenitor_support::encode_path(smart_collection_id),
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
     * Updates an existing smart collection.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/smart_collections/{smart_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#update-2021-01
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_smart_collections_param_collection(
        &self,
        smart_collection_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/smart_collections/{}/json",
                crate::progenitor_support::encode_path(smart_collection_id),
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
     * Removes a smart collection.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/smart_collections/{smart_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_smart_collections_param_collection(
        &self,
        smart_collection_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/smart_collections/{}/json",
                crate::progenitor_support::encode_path(smart_collection_id),
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
     * Updates the ordering type of products in a smart collection.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/smart_collections/{smart_collection_id}/order.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#order-2021-01
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     * * `products: &str` -- An array of product IDs, in the order that you want them to appear at the top of the collection. When products is specified but empty, any previously sorted products are cleared.
     * * `sort_order: &str` -- The type of sorting to apply. Valid values are listed in the Properties section above.
     *                     (default: (current value)).
     * * `products: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202101_update_smart_collections_param_collection_order(
        &self,
        smart_collection_id: &str,
        products: &str,
        sort_order: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !products.is_empty() {
            query_args.push(("products".to_string(), products.to_string()));
        }
        if !sort_order.is_empty() {
            query_args.push(("sort_order".to_string(), sort_order.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/smart_collections/{}/order.json?{}",
                crate::progenitor_support::encode_path(smart_collection_id),
                query_
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
     * Retrieves a list of smart collections. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/smart_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#index-unstable
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The number of results to show.
     *                     (default: 50, maximum: 250).
     * * `ids: &str` -- Show only the smart collections specified by a comma-separated list of IDs.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `title: &str` -- Show smart collections with the specified title.
     * * `product_id: &str` -- Show smart collections that includes the specified product.
     * * `handle: &str` -- Filter results by smart collection handle.
     * * `updated_at_min: &str` -- Show smart collections last updated after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show smart collections last updated before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show smart collections published after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show smart collections published before this date. (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Filter results based on the published status of smart collections.
     *                     (default: any)
     *                       
     *                           published: Show only published smart collections.
     *                           unpublished: Show only unpublished smart collections.
     *                           any: Show all smart collections.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_smart_collection(
        &self,
        limit: &str,
        ids: &str,
        since_id: &str,
        title: &str,
        product_id: &str,
        handle: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
        }
        if !updated_at_max.is_empty() {
            query_args.push(("updated_at_max".to_string(), updated_at_max.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/smart_collections.json?{}", query_),
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
     * Creates a new smart collection using the specified rules.
     *
     * This function performs a `POST` to the `/admin/api/unstable/smart_collections.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#create-unstable
     */
    pub async fn deprecated_unstable_create_smart_collections(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/unstable/smart_collections.json", None);
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
     * Retrieves a count of smart collections.
     *
     * This function performs a `GET` to the `/admin/api/unstable/smart_collections/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#count-unstable
     *
     * **Parameters:**
     *
     * * `title: &str` -- Show smart collections with the specified title.
     * * `product_id: &str` -- Show smart collections that include the specified product.
     * * `updated_at_min: &str` -- Show smart collections last updated after this date. (format: 2014-04-25T16:15:47-04:00).
     * * `updated_at_max: &str` -- Show smart collections last updated before this date.  (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_min: &str` -- Show smart collections published after this date.  (format: 2014-04-25T16:15:47-04:00).
     * * `published_at_max: &str` -- Show smart collections published before this date.  (format: 2014-04-25T16:15:47-04:00).
     * * `published_status: &str` -- Filter results based on the published status of smart collections.
     *                     (default: any)
     *                       
     *                           published: Show only published smart collections.
     *                           unpublished: Show only unpublished smart collections.
     *                           any: Show all smart collections.
     */
    pub async fn deprecated_unstable_get_smart_collections_count(
        &self,
        title: &str,
        product_id: &str,
        updated_at_min: &str,
        updated_at_max: &str,
        published_at_min: &str,
        published_at_max: &str,
        published_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !product_id.is_empty() {
            query_args.push(("product_id".to_string(), product_id.to_string()));
        }
        if !published_at_max.is_empty() {
            query_args.push(("published_at_max".to_string(), published_at_max.to_string()));
        }
        if !published_at_min.is_empty() {
            query_args.push(("published_at_min".to_string(), published_at_min.to_string()));
        }
        if !published_status.is_empty() {
            query_args.push(("published_status".to_string(), published_status.to_string()));
        }
        if !title.is_empty() {
            query_args.push(("title".to_string(), title.to_string()));
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
                "/admin/api/unstable/smart_collections/count.json?{}",
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
     * Retrieves a single smart collection.
     *
     * This function performs a `GET` to the `/admin/api/unstable/smart_collections/{smart_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#show-unstable
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_smart_collections_param_collection(
        &self,
        smart_collection_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/smart_collections/{}/json?{}",
                crate::progenitor_support::encode_path(smart_collection_id),
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
     * Updates an existing smart collection.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/smart_collections/{smart_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#update-unstable
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_smart_collections_param_collection(
        &self,
        smart_collection_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/smart_collections/{}/json",
                crate::progenitor_support::encode_path(smart_collection_id),
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
     * Removes a smart collection.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/smart_collections/{smart_collection_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#destroy-unstable
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_smart_collections_param_collection(
        &self,
        smart_collection_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/smart_collections/{}/json",
                crate::progenitor_support::encode_path(smart_collection_id),
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
     * Updates the ordering type of products in a smart collection.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/smart_collections/{smart_collection_id}/order.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/products/smartcollection#order-unstable
     *
     * **Parameters:**
     *
     * * `smart_collection_id: &str` -- storefront_access_token_id.
     * * `products: &str` -- An array of product IDs, in the order that you want them to appear at the top of the collection. When products is specified but empty, any previously sorted products are cleared.
     * * `sort_order: &str` -- The type of sorting to apply. Valid values are listed in the Properties section above.
     *                     (default: (current value)).
     * * `products: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_unstable_update_smart_collections_param_collection_order(
        &self,
        smart_collection_id: &str,
        products: &str,
        sort_order: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !products.is_empty() {
            query_args.push(("products".to_string(), products.to_string()));
        }
        if !sort_order.is_empty() {
            query_args.push(("sort_order".to_string(), sort_order.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/smart_collections/{}/order.json?{}",
                crate::progenitor_support::encode_path(smart_collection_id),
                query_
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
}
