use crate::Client;
use crate::ClientResult;

pub struct StoreProperties {
    pub client: Client,
}

impl StoreProperties {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        StoreProperties { client }
    }

    /**
     * Retrieves a list of countries.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/countries.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#index-2020-01
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_countrie(
        &self,
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
            &format!("/admin/api/2020-01/countries.json?{}", query_),
            None,
        );
        self.client
            .get(
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
      As of version 2020-10, the tax field is deprecated.

    Creates a country.
    *
    * This function performs a `POST` to the `/admin/api/2020-01/countries.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#create-2020-01
    */
    pub async fn deprecated_202001_create_countries(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-01/countries.json", None);
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
     * Retrieves a count of countries.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/countries/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#count-2020-01
     */
    pub async fn deprecated_202001_get_countries_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-01/countries/count.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a specific county.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/countries/{country_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#show-2020-01
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_countries_param_country(
        &self,
        country_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/countries/{}/json?{}",
                crate::progenitor_support::encode_path(country_id),
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
      As of version 2020-10, the tax field is deprecated.

    Updates an existing country.
    *
    * This function performs a `PUT` to the `/admin/api/2020-01/countries/{country_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#update-2020-01
    *
    * **Parameters:**
    *
    * * `country_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202001_update_countries_param_country(
        &self,
        country_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/countries/{}/json",
                crate::progenitor_support::encode_path(country_id),
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
     * Deletes a country.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/countries/{country_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_countries_param_country(
        &self,
        country_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/countries/{}/json",
                crate::progenitor_support::encode_path(country_id),
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
     * Retrieves a list of countries.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/countries.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#index-2020-04
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_countrie(
        &self,
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
            &format!("/admin/api/2020-04/countries.json?{}", query_),
            None,
        );
        self.client
            .get(
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
      As of version 2020-10, the tax field is deprecated.

    Creates a country.
    *
    * This function performs a `POST` to the `/admin/api/2020-04/countries.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#create-2020-04
    */
    pub async fn deprecated_202004_create_countries(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-04/countries.json", None);
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
     * Retrieves a count of countries.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/countries/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#count-2020-04
     */
    pub async fn deprecated_202004_get_countries_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-04/countries/count.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a specific county.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/countries/{country_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#show-2020-04
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_countries_param_country(
        &self,
        country_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/countries/{}/json?{}",
                crate::progenitor_support::encode_path(country_id),
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
      As of version 2020-10, the tax field is deprecated.

    Updates an existing country.
    *
    * This function performs a `PUT` to the `/admin/api/2020-04/countries/{country_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#update-2020-04
    *
    * **Parameters:**
    *
    * * `country_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202004_update_countries_param_country(
        &self,
        country_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/countries/{}/json",
                crate::progenitor_support::encode_path(country_id),
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
     * Deletes a country.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/countries/{country_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_countries_param_country(
        &self,
        country_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/countries/{}/json",
                crate::progenitor_support::encode_path(country_id),
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
     * Retrieves a list of countries.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/countries.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#index-2020-07
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_countrie(
        &self,
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
            &format!("/admin/api/2020-07/countries.json?{}", query_),
            None,
        );
        self.client
            .get(
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
      As of version 2020-10, the tax field is deprecated.

    Creates a country.
    *
    * This function performs a `POST` to the `/admin/api/2020-07/countries.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#create-2020-07
    */
    pub async fn deprecated_202007_create_countries(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-07/countries.json", None);
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
     * Retrieves a count of countries.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/countries/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#count-2020-07
     */
    pub async fn deprecated_202007_get_countries_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-07/countries/count.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a specific county.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/countries/{country_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#show-2020-07
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_countries_param_country(
        &self,
        country_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/countries/{}/json?{}",
                crate::progenitor_support::encode_path(country_id),
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
      As of version 2020-10, the tax field is deprecated.

    Updates an existing country.
    *
    * This function performs a `PUT` to the `/admin/api/2020-07/countries/{country_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#update-2020-07
    *
    * **Parameters:**
    *
    * * `country_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202007_update_countries_param_country(
        &self,
        country_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/countries/{}/json",
                crate::progenitor_support::encode_path(country_id),
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
     * Deletes a country.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/countries/{country_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_countries_param_country(
        &self,
        country_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/countries/{}/json",
                crate::progenitor_support::encode_path(country_id),
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
     * Retrieves a list of countries.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/countries.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#index-2020-10
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_countrie(&self, since_id: &str, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/countries.json?{}", query_),
            None,
        );
        self.client
            .get(
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
      As of version 2020-10, the tax field is deprecated.

    Creates a country.
    *
    * This function performs a `POST` to the `/admin/api/2020-10/countries.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#create-2020-10
    */
    pub async fn create_countries(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-10/countries.json", None);
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
     * Retrieves a count of countries.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/countries/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#count-2020-10
     */
    pub async fn get_countries_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-10/countries/count.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a specific county.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/countries/{country_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#show-2020-10
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_countries_param_country(
        &self,
        country_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/countries/{}/json?{}",
                crate::progenitor_support::encode_path(country_id),
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
      As of version 2020-10, the tax field is deprecated.

    Updates an existing country.
    *
    * This function performs a `PUT` to the `/admin/api/2020-10/countries/{country_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#update-2020-10
    *
    * **Parameters:**
    *
    * * `country_id: &str` -- storefront_access_token_id.
    */
    pub async fn update_countries_param_country(
        &self,
        country_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/countries/{}/json",
                crate::progenitor_support::encode_path(country_id),
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
     * Deletes a country.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/countries/{country_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_countries_param_country(&self, country_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/countries/{}/json",
                crate::progenitor_support::encode_path(country_id),
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
     * Retrieves a list of countries.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/countries.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#index-2021-01
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_countrie(
        &self,
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
            &format!("/admin/api/2021-01/countries.json?{}", query_),
            None,
        );
        self.client
            .get(
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
      As of version 2020-10, the tax field is deprecated.

    Creates a country.
    *
    * This function performs a `POST` to the `/admin/api/2021-01/countries.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#create-2021-01
    */
    pub async fn deprecated_202101_create_countries(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2021-01/countries.json", None);
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
     * Retrieves a count of countries.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/countries/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#count-2021-01
     */
    pub async fn deprecated_202101_get_countries_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2021-01/countries/count.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a specific county.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/countries/{country_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#show-2021-01
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_countries_param_country(
        &self,
        country_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/countries/{}/json?{}",
                crate::progenitor_support::encode_path(country_id),
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
      As of version 2020-10, the tax field is deprecated.

    Updates an existing country.
    *
    * This function performs a `PUT` to the `/admin/api/2021-01/countries/{country_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#update-2021-01
    *
    * **Parameters:**
    *
    * * `country_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202101_update_countries_param_country(
        &self,
        country_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/countries/{}/json",
                crate::progenitor_support::encode_path(country_id),
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
     * Deletes a country.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/countries/{country_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_countries_param_country(
        &self,
        country_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/countries/{}/json",
                crate::progenitor_support::encode_path(country_id),
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
     * Retrieves a list of countries.
     *
     * This function performs a `GET` to the `/admin/api/unstable/countries.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#index-unstable
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_countrie(
        &self,
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
            &format!("/admin/api/unstable/countries.json?{}", query_),
            None,
        );
        self.client
            .get(
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
      As of version 2020-10, the tax field is deprecated.

    Creates a country.
    *
    * This function performs a `POST` to the `/admin/api/unstable/countries.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#create-unstable
    */
    pub async fn deprecated_unstable_create_countries(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/unstable/countries.json", None);
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
     * Retrieves a count of countries.
     *
     * This function performs a `GET` to the `/admin/api/unstable/countries/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#count-unstable
     */
    pub async fn deprecated_unstable_get_countries_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/unstable/countries/count.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a specific county.
     *
     * This function performs a `GET` to the `/admin/api/unstable/countries/{country_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#show-unstable
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_countries_param_country(
        &self,
        country_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/countries/{}/json?{}",
                crate::progenitor_support::encode_path(country_id),
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
      As of version 2020-10, the tax field is deprecated.

    Updates an existing country.
    *
    * This function performs a `PUT` to the `/admin/api/unstable/countries/{country_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#update-unstable
    *
    * **Parameters:**
    *
    * * `country_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_unstable_update_countries_param_country(
        &self,
        country_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/countries/{}/json",
                crate::progenitor_support::encode_path(country_id),
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
     * Deletes a country.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/countries/{country_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/country#destroy-unstable
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_countries_param_country(
        &self,
        country_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/countries/{}/json",
                crate::progenitor_support::encode_path(country_id),
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
     * Retrieves a list of currencies enabled on a shop.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/currencies.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/currency#index-2020-01
     */
    pub async fn deprecated_202001_get_currencie(&self) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-01/currencies.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of currencies enabled on a shop.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/currencies.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/currency#index-2020-04
     */
    pub async fn deprecated_202004_get_currencie(&self) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-04/currencies.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of currencies enabled on a shop.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/currencies.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/currency#index-2020-07
     */
    pub async fn deprecated_202007_get_currencie(&self) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-07/currencies.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of currencies enabled on a shop.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/currencies.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/currency#index-2020-10
     */
    pub async fn get_currencie(&self) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-10/currencies.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of currencies enabled on a shop.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/currencies.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/currency#index-2021-01
     */
    pub async fn deprecated_202101_get_currencie(&self) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2021-01/currencies.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of currencies enabled on a shop.
     *
     * This function performs a `GET` to the `/admin/api/unstable/currencies.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/currency#index-unstable
     */
    pub async fn deprecated_unstable_get_currencie(&self) -> ClientResult<()> {
        let url = self.client.url("/admin/api/unstable/currencies.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of the shop's policies.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/policies.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/policy#index-2020-01
     */
    pub async fn deprecated_202001_get_policie(&self) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-01/policies.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of the shop's policies.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/policies.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/policy#index-2020-04
     */
    pub async fn deprecated_202004_get_policie(&self) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-04/policies.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of the shop's policies.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/policies.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/policy#index-2020-07
     */
    pub async fn deprecated_202007_get_policie(&self) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-07/policies.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of the shop's policies.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/policies.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/policy#index-2020-10
     */
    pub async fn get_policie(&self) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-10/policies.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of the shop's policies.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/policies.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/policy#index-2021-01
     */
    pub async fn deprecated_202101_get_policie(&self) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2021-01/policies.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of the shop's policies.
     *
     * This function performs a `GET` to the `/admin/api/unstable/policies.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/policy#index-unstable
     */
    pub async fn deprecated_unstable_get_policie(&self) -> ClientResult<()> {
        let url = self.client.url("/admin/api/unstable/policies.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves a list of provinces.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/countries/{country_id}/provinces.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#index-2020-01
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of fields names.
     */
    pub async fn deprecated_202001_get_countries_param_country_province(
        &self,
        country_id: &str,
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
                "/admin/api/2020-01/countries/{}/provinces.json?{}",
                crate::progenitor_support::encode_path(country_id),
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
     * Retrieves a count of provinces for a country.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/countries/{country_id}/provinces/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#count-2020-01
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_countries_param_country_provinces_count(
        &self,
        country_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/countries/{}/provinces/count.json",
                crate::progenitor_support::encode_path(country_id),
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
     * Retrieves a single province for a country.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/countries/{country_id}/provinces/{province_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#show-2020-01
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     * * `province_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202001_get_countries_param_country_provinces_province(
        &self,
        country_id: &str,
        province_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/countries/{}/provinces/{}/json?{}",
                crate::progenitor_support::encode_path(country_id),
                crate::progenitor_support::encode_path(province_id),
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
      As of version 2020-10, the tax field is deprecated.

    Updates an existing province for a country.
    *
    * This function performs a `PUT` to the `/admin/api/2020-01/countries/{country_id}/provinces/{province_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#update-2020-01
    *
    * **Parameters:**
    *
    * * `country_id: &str` -- storefront_access_token_id.
    * * `province_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202001_update_countries_param_country_provinces_province(
        &self,
        country_id: &str,
        province_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/countries/{}/provinces/{}/json",
                crate::progenitor_support::encode_path(country_id),
                crate::progenitor_support::encode_path(province_id),
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
     * Retrieves a list of provinces.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/countries/{country_id}/provinces.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#index-2020-04
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of fields names.
     */
    pub async fn deprecated_202004_get_countries_param_country_province(
        &self,
        country_id: &str,
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
                "/admin/api/2020-04/countries/{}/provinces.json?{}",
                crate::progenitor_support::encode_path(country_id),
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
     * Retrieves a count of provinces for a country.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/countries/{country_id}/provinces/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#count-2020-04
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_countries_param_country_provinces_count(
        &self,
        country_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/countries/{}/provinces/count.json",
                crate::progenitor_support::encode_path(country_id),
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
     * Retrieves a single province for a country.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/countries/{country_id}/provinces/{province_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#show-2020-04
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     * * `province_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202004_get_countries_param_country_provinces_province(
        &self,
        country_id: &str,
        province_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/countries/{}/provinces/{}/json?{}",
                crate::progenitor_support::encode_path(country_id),
                crate::progenitor_support::encode_path(province_id),
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
      As of version 2020-10, the tax field is deprecated.

    Updates an existing province for a country.
    *
    * This function performs a `PUT` to the `/admin/api/2020-04/countries/{country_id}/provinces/{province_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#update-2020-04
    *
    * **Parameters:**
    *
    * * `country_id: &str` -- storefront_access_token_id.
    * * `province_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202004_update_countries_param_country_provinces_province(
        &self,
        country_id: &str,
        province_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/countries/{}/provinces/{}/json",
                crate::progenitor_support::encode_path(country_id),
                crate::progenitor_support::encode_path(province_id),
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
     * Retrieves a list of provinces.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/countries/{country_id}/provinces.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#index-2020-07
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of fields names.
     */
    pub async fn deprecated_202007_get_countries_param_country_province(
        &self,
        country_id: &str,
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
                "/admin/api/2020-07/countries/{}/provinces.json?{}",
                crate::progenitor_support::encode_path(country_id),
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
     * Retrieves a count of provinces for a country.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/countries/{country_id}/provinces/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#count-2020-07
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_countries_param_country_provinces_count(
        &self,
        country_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/countries/{}/provinces/count.json",
                crate::progenitor_support::encode_path(country_id),
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
     * Retrieves a single province for a country.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/countries/{country_id}/provinces/{province_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#show-2020-07
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     * * `province_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202007_get_countries_param_country_provinces_province(
        &self,
        country_id: &str,
        province_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/countries/{}/provinces/{}/json?{}",
                crate::progenitor_support::encode_path(country_id),
                crate::progenitor_support::encode_path(province_id),
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
      As of version 2020-10, the tax field is deprecated.

    Updates an existing province for a country.
    *
    * This function performs a `PUT` to the `/admin/api/2020-07/countries/{country_id}/provinces/{province_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#update-2020-07
    *
    * **Parameters:**
    *
    * * `country_id: &str` -- storefront_access_token_id.
    * * `province_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202007_update_countries_param_country_provinces_province(
        &self,
        country_id: &str,
        province_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/countries/{}/provinces/{}/json",
                crate::progenitor_support::encode_path(country_id),
                crate::progenitor_support::encode_path(province_id),
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
     * Retrieves a list of provinces.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/countries/{country_id}/provinces.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#index-2020-10
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of fields names.
     */
    pub async fn get_countries_param_country_province(
        &self,
        country_id: &str,
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
                "/admin/api/2020-10/countries/{}/provinces.json?{}",
                crate::progenitor_support::encode_path(country_id),
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
     * Retrieves a count of provinces for a country.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/countries/{country_id}/provinces/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#count-2020-10
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     */
    pub async fn get_countries_param_country_provinces_count(
        &self,
        country_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/countries/{}/provinces/count.json",
                crate::progenitor_support::encode_path(country_id),
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
     * Retrieves a single province for a country.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/countries/{country_id}/provinces/{province_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#show-2020-10
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     * * `province_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn get_countries_param_country_provinces_province(
        &self,
        country_id: &str,
        province_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/countries/{}/provinces/{}/json?{}",
                crate::progenitor_support::encode_path(country_id),
                crate::progenitor_support::encode_path(province_id),
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
      As of version 2020-10, the tax field is deprecated.

    Updates an existing province for a country.
    *
    * This function performs a `PUT` to the `/admin/api/2020-10/countries/{country_id}/provinces/{province_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#update-2020-10
    *
    * **Parameters:**
    *
    * * `country_id: &str` -- storefront_access_token_id.
    * * `province_id: &str` -- storefront_access_token_id.
    */
    pub async fn update_countries_param_country_provinces_province(
        &self,
        country_id: &str,
        province_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/countries/{}/provinces/{}/json",
                crate::progenitor_support::encode_path(country_id),
                crate::progenitor_support::encode_path(province_id),
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
     * Retrieves a list of provinces.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/countries/{country_id}/provinces.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#index-2021-01
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of fields names.
     */
    pub async fn deprecated_202101_get_countries_param_country_province(
        &self,
        country_id: &str,
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
                "/admin/api/2021-01/countries/{}/provinces.json?{}",
                crate::progenitor_support::encode_path(country_id),
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
     * Retrieves a count of provinces for a country.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/countries/{country_id}/provinces/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#count-2021-01
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_countries_param_country_provinces_count(
        &self,
        country_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/countries/{}/provinces/count.json",
                crate::progenitor_support::encode_path(country_id),
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
     * Retrieves a single province for a country.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/countries/{country_id}/provinces/{province_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#show-2021-01
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     * * `province_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_202101_get_countries_param_country_provinces_province(
        &self,
        country_id: &str,
        province_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/countries/{}/provinces/{}/json?{}",
                crate::progenitor_support::encode_path(country_id),
                crate::progenitor_support::encode_path(province_id),
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
      As of version 2020-10, the tax field is deprecated.

    Updates an existing province for a country.
    *
    * This function performs a `PUT` to the `/admin/api/2021-01/countries/{country_id}/provinces/{province_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#update-2021-01
    *
    * **Parameters:**
    *
    * * `country_id: &str` -- storefront_access_token_id.
    * * `province_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202101_update_countries_param_country_provinces_province(
        &self,
        country_id: &str,
        province_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/countries/{}/provinces/{}/json",
                crate::progenitor_support::encode_path(country_id),
                crate::progenitor_support::encode_path(province_id),
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
     * Retrieves a list of provinces.
     *
     * This function performs a `GET` to the `/admin/api/unstable/countries/{country_id}/provinces.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#index-unstable
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of fields names.
     */
    pub async fn deprecated_unstable_get_countries_param_country_province(
        &self,
        country_id: &str,
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
                "/admin/api/unstable/countries/{}/provinces.json?{}",
                crate::progenitor_support::encode_path(country_id),
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
     * Retrieves a count of provinces for a country.
     *
     * This function performs a `GET` to the `/admin/api/unstable/countries/{country_id}/provinces/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#count-unstable
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_countries_param_country_provinces_count(
        &self,
        country_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/countries/{}/provinces/count.json",
                crate::progenitor_support::encode_path(country_id),
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
     * Retrieves a single province for a country.
     *
     * This function performs a `GET` to the `/admin/api/unstable/countries/{country_id}/provinces/{province_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#show-unstable
     *
     * **Parameters:**
     *
     * * `country_id: &str` -- storefront_access_token_id.
     * * `province_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- Show only certain fields, specified by a comma-separated list of field names.
     */
    pub async fn deprecated_unstable_get_countries_param_country_provinces_province(
        &self,
        country_id: &str,
        province_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/countries/{}/provinces/{}/json?{}",
                crate::progenitor_support::encode_path(country_id),
                crate::progenitor_support::encode_path(province_id),
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
      As of version 2020-10, the tax field is deprecated.

    Updates an existing province for a country.
    *
    * This function performs a `PUT` to the `/admin/api/unstable/countries/{country_id}/provinces/{province_id}.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/store-properties/province#update-unstable
    *
    * **Parameters:**
    *
    * * `country_id: &str` -- storefront_access_token_id.
    * * `province_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_unstable_update_countries_param_country_provinces_province(
        &self,
        country_id: &str,
        province_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/countries/{}/provinces/{}/json",
                crate::progenitor_support::encode_path(country_id),
                crate::progenitor_support::encode_path(province_id),
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
     * Get a list of all shipping zones.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/shipping_zones.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/shippingzone#index-2020-01
     *
     * **Parameters:**
     *
     * * `fields: &str` -- comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202001_get_shipping_zone(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/shipping_zones.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get a list of all shipping zones.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/shipping_zones.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/shippingzone#index-2020-04
     *
     * **Parameters:**
     *
     * * `fields: &str` -- comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202004_get_shipping_zone(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/shipping_zones.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get a list of all shipping zones.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/shipping_zones.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/shippingzone#index-2020-07
     *
     * **Parameters:**
     *
     * * `fields: &str` -- comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202007_get_shipping_zone(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/shipping_zones.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get a list of all shipping zones.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/shipping_zones.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/shippingzone#index-2020-10
     *
     * **Parameters:**
     *
     * * `fields: &str` -- comma-separated list of fields to include in the response.
     */
    pub async fn get_shipping_zone(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/shipping_zones.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get a list of all shipping zones.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/shipping_zones.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/shippingzone#index-2021-01
     *
     * **Parameters:**
     *
     * * `fields: &str` -- comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202101_get_shipping_zone(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/shipping_zones.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get a list of all shipping zones.
     *
     * This function performs a `GET` to the `/admin/api/unstable/shipping_zones.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/shippingzone#index-unstable
     *
     * **Parameters:**
     *
     * * `fields: &str` -- comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_unstable_get_shipping_zone(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/shipping_zones.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves the shop's configuration.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/shop.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/shop#show-2020-01
     *
     * **Parameters:**
     *
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202001_get_shop(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-01/shop.json?{}", query_), None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves the shop's configuration.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/shop.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/shop#show-2020-04
     *
     * **Parameters:**
     *
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202004_get_shop(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-04/shop.json?{}", query_), None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves the shop's configuration.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/shop.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/shop#show-2020-07
     *
     * **Parameters:**
     *
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202007_get_shop(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-07/shop.json?{}", query_), None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves the shop's configuration.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/shop.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/shop#show-2020-10
     *
     * **Parameters:**
     *
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn get_shop(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2020-10/shop.json?{}", query_), None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves the shop's configuration.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/shop.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/shop#show-2021-01
     *
     * **Parameters:**
     *
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202101_get_shop(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/2021-01/shop.json?{}", query_), None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves the shop's configuration.
     *
     * This function performs a `GET` to the `/admin/api/unstable/shop.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/store-properties/shop#show-unstable
     *
     * **Parameters:**
     *
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_unstable_get_shop(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/admin/api/unstable/shop.json?{}", query_), None);
        self.client
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
