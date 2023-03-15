use crate::Client;
use crate::ClientResult;

pub struct Metafield {
    pub client: Client,
}

impl Metafield {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Metafield { client }
    }

    /**
     * Retrieves a list of metafields that belong to a Product Image resource.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/metafields.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#index-2020-01
     *
     * **Parameters:**
     *
     * * `metafield_owner_id: i64` -- recurring_application_charge[capped_amount].
     * * `metafield_owner_resource: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get(
        &self,
        metafield_owner_id: i64,
        metafield_owner_resource: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if metafield_owner_id > 0 {
            query_args.push((
                "metafield[owner_id]".to_string(),
                metafield_owner_id.to_string(),
            ));
        }
        if !metafield_owner_resource.is_empty() {
            query_args.push((
                "metafield[owner_resource]".to_string(),
                metafield_owner_resource.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/metafields.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a new metafield for a resource.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/metafields.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#create-2020-01
     */
    pub async fn deprecated_202001_create(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-01/metafields.json", None);
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
     * Retrieves a count of a resource's metafields.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/metafields/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#count-2020-01
     */
    pub async fn deprecated_202001_get_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-01/metafields/count.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a single metafield from a resource by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/metafields/{metafield_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#show-2020-01
     *
     * **Parameters:**
     *
     * * `metafield_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_param(
        &self,
        metafield_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/metafields/{}/json?{}",
                crate::progenitor_support::encode_path(metafield_id),
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
     * Updates a metafield.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/metafields/{metafield_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#update-2020-01
     *
     * **Parameters:**
     *
     * * `metafield_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_param(
        &self,
        metafield_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/metafields/{}/json",
                crate::progenitor_support::encode_path(metafield_id),
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
     * Deletes a metafield by its ID.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/metafields/{metafield_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `metafield_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_param(&self, metafield_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/metafields/{}/json",
                crate::progenitor_support::encode_path(metafield_id),
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
     * Retrieves a list of metafields that belong to a Product Image resource.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/metafields.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#index-2020-04
     *
     * **Parameters:**
     *
     * * `metafield_owner_id: i64` -- recurring_application_charge[capped_amount].
     * * `metafield_owner_resource: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get(
        &self,
        metafield_owner_id: i64,
        metafield_owner_resource: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if metafield_owner_id > 0 {
            query_args.push((
                "metafield[owner_id]".to_string(),
                metafield_owner_id.to_string(),
            ));
        }
        if !metafield_owner_resource.is_empty() {
            query_args.push((
                "metafield[owner_resource]".to_string(),
                metafield_owner_resource.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/metafields.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a new metafield for a resource.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/metafields.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#create-2020-04
     */
    pub async fn deprecated_202004_create(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-04/metafields.json", None);
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
     * Retrieves a count of a resource's metafields.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/metafields/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#count-2020-04
     */
    pub async fn deprecated_202004_get_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-04/metafields/count.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a single metafield from a resource by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/metafields/{metafield_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#show-2020-04
     *
     * **Parameters:**
     *
     * * `metafield_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_param(
        &self,
        metafield_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/metafields/{}/json?{}",
                crate::progenitor_support::encode_path(metafield_id),
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
     * Updates a metafield.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/metafields/{metafield_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#update-2020-04
     *
     * **Parameters:**
     *
     * * `metafield_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_param(
        &self,
        metafield_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/metafields/{}/json",
                crate::progenitor_support::encode_path(metafield_id),
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
     * Deletes a metafield by its ID.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/metafields/{metafield_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `metafield_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_param(&self, metafield_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/metafields/{}/json",
                crate::progenitor_support::encode_path(metafield_id),
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
     * Retrieves a list of metafields that belong to a Product Image resource.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/metafields.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#index-2020-07
     *
     * **Parameters:**
     *
     * * `metafield_owner_id: i64` -- recurring_application_charge[capped_amount].
     * * `metafield_owner_resource: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get(
        &self,
        metafield_owner_id: i64,
        metafield_owner_resource: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if metafield_owner_id > 0 {
            query_args.push((
                "metafield[owner_id]".to_string(),
                metafield_owner_id.to_string(),
            ));
        }
        if !metafield_owner_resource.is_empty() {
            query_args.push((
                "metafield[owner_resource]".to_string(),
                metafield_owner_resource.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/metafields.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a new metafield for a resource.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/metafields.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#create-2020-07
     */
    pub async fn deprecated_202007_create(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-07/metafields.json", None);
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
     * Retrieves a count of a resource's metafields.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/metafields/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#count-2020-07
     */
    pub async fn deprecated_202007_get_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-07/metafields/count.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a single metafield from a resource by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/metafields/{metafield_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#show-2020-07
     *
     * **Parameters:**
     *
     * * `metafield_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_param(
        &self,
        metafield_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/metafields/{}/json?{}",
                crate::progenitor_support::encode_path(metafield_id),
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
     * Updates a metafield.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/metafields/{metafield_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#update-2020-07
     *
     * **Parameters:**
     *
     * * `metafield_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_param(
        &self,
        metafield_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/metafields/{}/json",
                crate::progenitor_support::encode_path(metafield_id),
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
     * Deletes a metafield by its ID.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/metafields/{metafield_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `metafield_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_param(&self, metafield_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/metafields/{}/json",
                crate::progenitor_support::encode_path(metafield_id),
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
     * Retrieves a list of metafields that belong to a Product Image resource.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/metafields.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#index-2020-10
     *
     * **Parameters:**
     *
     * * `metafield_owner_id: i64` -- recurring_application_charge[capped_amount].
     * * `metafield_owner_resource: &str` -- storefront_access_token_id.
     */
    pub async fn get(
        &self,
        metafield_owner_id: i64,
        metafield_owner_resource: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if metafield_owner_id > 0 {
            query_args.push((
                "metafield[owner_id]".to_string(),
                metafield_owner_id.to_string(),
            ));
        }
        if !metafield_owner_resource.is_empty() {
            query_args.push((
                "metafield[owner_resource]".to_string(),
                metafield_owner_resource.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/metafields.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a new metafield for a resource.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/metafields.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#create-2020-10
     */
    pub async fn create(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-10/metafields.json", None);
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
     * Retrieves a count of a resource's metafields.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/metafields/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#count-2020-10
     */
    pub async fn get_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-10/metafields/count.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a single metafield from a resource by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/metafields/{metafield_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#show-2020-10
     *
     * **Parameters:**
     *
     * * `metafield_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_param(&self, metafield_id: &str, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/metafields/{}/json?{}",
                crate::progenitor_support::encode_path(metafield_id),
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
     * Updates a metafield.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/metafields/{metafield_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#update-2020-10
     *
     * **Parameters:**
     *
     * * `metafield_id: &str` -- storefront_access_token_id.
     */
    pub async fn update_param(
        &self,
        metafield_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/metafields/{}/json",
                crate::progenitor_support::encode_path(metafield_id),
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
     * Deletes a metafield by its ID.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/metafields/{metafield_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `metafield_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_param(&self, metafield_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/metafields/{}/json",
                crate::progenitor_support::encode_path(metafield_id),
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
     * Retrieves a list of metafields that belong to a Product Image resource.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/metafields.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#index-2021-01
     *
     * **Parameters:**
     *
     * * `metafield_owner_id: i64` -- recurring_application_charge[capped_amount].
     * * `metafield_owner_resource: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get(
        &self,
        metafield_owner_id: i64,
        metafield_owner_resource: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if metafield_owner_id > 0 {
            query_args.push((
                "metafield[owner_id]".to_string(),
                metafield_owner_id.to_string(),
            ));
        }
        if !metafield_owner_resource.is_empty() {
            query_args.push((
                "metafield[owner_resource]".to_string(),
                metafield_owner_resource.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/metafields.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a new metafield for a resource.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/metafields.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#create-2021-01
     */
    pub async fn deprecated_202101_create(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2021-01/metafields.json", None);
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
     * Retrieves a count of a resource's metafields.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/metafields/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#count-2021-01
     */
    pub async fn deprecated_202101_get_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2021-01/metafields/count.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a single metafield from a resource by its ID.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/metafields/{metafield_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#show-2021-01
     *
     * **Parameters:**
     *
     * * `metafield_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_param(
        &self,
        metafield_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/metafields/{}/json?{}",
                crate::progenitor_support::encode_path(metafield_id),
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
     * Updates a metafield.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/metafields/{metafield_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#update-2021-01
     *
     * **Parameters:**
     *
     * * `metafield_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_param(
        &self,
        metafield_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/metafields/{}/json",
                crate::progenitor_support::encode_path(metafield_id),
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
     * Deletes a metafield by its ID.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/metafields/{metafield_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `metafield_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_param(&self, metafield_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/metafields/{}/json",
                crate::progenitor_support::encode_path(metafield_id),
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
     * Retrieves a list of metafields that belong to a Product Image resource.
     *
     * This function performs a `GET` to the `/admin/api/unstable/metafields.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#index-unstable
     *
     * **Parameters:**
     *
     * * `metafield_owner_id: i64` -- recurring_application_charge[capped_amount].
     * * `metafield_owner_resource: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get(
        &self,
        metafield_owner_id: i64,
        metafield_owner_resource: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if metafield_owner_id > 0 {
            query_args.push((
                "metafield[owner_id]".to_string(),
                metafield_owner_id.to_string(),
            ));
        }
        if !metafield_owner_resource.is_empty() {
            query_args.push((
                "metafield[owner_resource]".to_string(),
                metafield_owner_resource.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/metafields.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a new metafield for a resource.
     *
     * This function performs a `POST` to the `/admin/api/unstable/metafields.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#create-unstable
     */
    pub async fn deprecated_unstable_create(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/unstable/metafields.json", None);
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
     * Retrieves a count of a resource's metafields.
     *
     * This function performs a `GET` to the `/admin/api/unstable/metafields/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#count-unstable
     */
    pub async fn deprecated_unstable_get_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/unstable/metafields/count.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a single metafield from a resource by its ID.
     *
     * This function performs a `GET` to the `/admin/api/unstable/metafields/{metafield_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#show-unstable
     *
     * **Parameters:**
     *
     * * `metafield_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_param(
        &self,
        metafield_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/metafields/{}/json?{}",
                crate::progenitor_support::encode_path(metafield_id),
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
     * Updates a metafield.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/metafields/{metafield_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#update-unstable
     *
     * **Parameters:**
     *
     * * `metafield_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_param(
        &self,
        metafield_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/metafields/{}/json",
                crate::progenitor_support::encode_path(metafield_id),
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
     * Deletes a metafield by its ID.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/metafields/{metafield_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/metafield#destroy-unstable
     *
     * **Parameters:**
     *
     * * `metafield_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_param(&self, metafield_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/metafields/{}/json",
                crate::progenitor_support::encode_path(metafield_id),
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
}
