use crate::Client;
use crate::ClientResult;

pub struct Billing {
    pub client: Client,
}

impl Billing {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Billing { client }
    }

    /**
     * Retrieves a list of application charges.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#index-2020-01
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202001_get_application_charge(
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
            &format!("/admin/api/2020-01/application_charges.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates an application charge.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#create-2020-01
     */
    pub async fn deprecated_202001_create_application_charges(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-01/application_charges.json", None);
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
     * Retrieves an application charge.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/application_charges/{application_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#show-2020-01
     *
     * **Parameters:**
     *
     * * `application_charge_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202001_get_application_charges_param_charge(
        &self,
        application_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/application_charges/{}/json?{}",
                crate::progenitor_support::encode_path(application_charge_id),
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
      This endpoint is no longer required and is deprecated as of
      API version 2021-01.

    "Activates an accepted application charge. One-time charges are now immediately activated
    when approved by a merchant.
    *
    * This function performs a `POST` to the `/admin/api/2020-01/application_charges/{application_charge_id}/activate.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#activate-2020-01
    *
    * **Parameters:**
    *
    * * `application_charge_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202001_create_application_charges_param_charge_activate(
        &self,
        application_charge_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/application_charges/{}/activate.json",
                crate::progenitor_support::encode_path(application_charge_id),
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
     * Retrieves a list of application charges.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#index-2020-04
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202004_get_application_charge(
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
            &format!("/admin/api/2020-04/application_charges.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates an application charge.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#create-2020-04
     */
    pub async fn deprecated_202004_create_application_charges(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-04/application_charges.json", None);
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
     * Retrieves an application charge.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/application_charges/{application_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#show-2020-04
     *
     * **Parameters:**
     *
     * * `application_charge_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202004_get_application_charges_param_charge(
        &self,
        application_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/application_charges/{}/json?{}",
                crate::progenitor_support::encode_path(application_charge_id),
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
      This endpoint is no longer required and is deprecated as of
      API version 2021-01.

    "Activates an accepted application charge. One-time charges are now immediately activated
    when approved by a merchant.
    *
    * This function performs a `POST` to the `/admin/api/2020-04/application_charges/{application_charge_id}/activate.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#activate-2020-04
    *
    * **Parameters:**
    *
    * * `application_charge_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202004_create_application_charges_param_charge_activate(
        &self,
        application_charge_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/application_charges/{}/activate.json",
                crate::progenitor_support::encode_path(application_charge_id),
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
     * Retrieves a list of application charges.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#index-2020-07
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202007_get_application_charge(
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
            &format!("/admin/api/2020-07/application_charges.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates an application charge.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#create-2020-07
     */
    pub async fn deprecated_202007_create_application_charges(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-07/application_charges.json", None);
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
     * Retrieves an application charge.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/application_charges/{application_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#show-2020-07
     *
     * **Parameters:**
     *
     * * `application_charge_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202007_get_application_charges_param_charge(
        &self,
        application_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/application_charges/{}/json?{}",
                crate::progenitor_support::encode_path(application_charge_id),
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
      This endpoint is no longer required and is deprecated as of
      API version 2021-01.

    "Activates an accepted application charge. One-time charges are now immediately activated
    when approved by a merchant.
    *
    * This function performs a `POST` to the `/admin/api/2020-07/application_charges/{application_charge_id}/activate.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#activate-2020-07
    *
    * **Parameters:**
    *
    * * `application_charge_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202007_create_application_charges_param_charge_activate(
        &self,
        application_charge_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/application_charges/{}/activate.json",
                crate::progenitor_support::encode_path(application_charge_id),
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
     * Retrieves a list of application charges.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#index-2020-10
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn get_application_charge(&self, since_id: &str, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/application_charges.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates an application charge.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#create-2020-10
     */
    pub async fn create_application_charges(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-10/application_charges.json", None);
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
     * Retrieves an application charge.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/application_charges/{application_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#show-2020-10
     *
     * **Parameters:**
     *
     * * `application_charge_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn get_application_charges_param_charge(
        &self,
        application_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/application_charges/{}/json?{}",
                crate::progenitor_support::encode_path(application_charge_id),
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
      This endpoint is no longer required and is deprecated as of
      API version 2021-01.

    "Activates an accepted application charge. One-time charges are now immediately activated
    when approved by a merchant.
    *
    * This function performs a `POST` to the `/admin/api/2020-10/application_charges/{application_charge_id}/activate.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#activate-2020-10
    *
    * **Parameters:**
    *
    * * `application_charge_id: &str` -- storefront_access_token_id.
    */
    pub async fn create_application_charges_param_charge_activate(
        &self,
        application_charge_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/application_charges/{}/activate.json",
                crate::progenitor_support::encode_path(application_charge_id),
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
     * Retrieves a list of application charges.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#index-2021-01
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202101_get_application_charge(
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
            &format!("/admin/api/2021-01/application_charges.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates an application charge.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#create-2021-01
     */
    pub async fn deprecated_202101_create_application_charges(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2021-01/application_charges.json", None);
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
     * Retrieves an application charge.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/application_charges/{application_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#show-2021-01
     *
     * **Parameters:**
     *
     * * `application_charge_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202101_get_application_charges_param_charge(
        &self,
        application_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/application_charges/{}/json?{}",
                crate::progenitor_support::encode_path(application_charge_id),
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
     * Retrieves a list of application charges.
     *
     * This function performs a `GET` to the `/admin/api/unstable/application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#index-unstable
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_unstable_get_application_charge(
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
            &format!("/admin/api/unstable/application_charges.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates an application charge.
     *
     * This function performs a `POST` to the `/admin/api/unstable/application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#create-unstable
     */
    pub async fn deprecated_unstable_create_application_charges(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/unstable/application_charges.json", None);
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
     * Retrieves an application charge.
     *
     * This function performs a `GET` to the `/admin/api/unstable/application_charges/{application_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcharge#show-unstable
     *
     * **Parameters:**
     *
     * * `application_charge_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_unstable_get_application_charges_param_charge(
        &self,
        application_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/application_charges/{}/json?{}",
                crate::progenitor_support::encode_path(application_charge_id),
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
     * Retrieves all application credits.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/application_credits.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcredit#index-2020-01
     *
     * **Parameters:**
     *
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202001_get_application_credit(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/application_credits.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates an application credit.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/application_credits.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcredit#create-2020-01
     */
    pub async fn deprecated_202001_create_application_credits(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-01/application_credits.json", None);
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
     * Retrieves a single application credit.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/application_credits/{application_credit_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcredit#show-2020-01
     *
     * **Parameters:**
     *
     * * `application_credit_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202001_get_application_credits_param_credit(
        &self,
        application_credit_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/application_credits/{}/json?{}",
                crate::progenitor_support::encode_path(application_credit_id),
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
     * Retrieves all application credits.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/application_credits.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcredit#index-2020-04
     *
     * **Parameters:**
     *
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202004_get_application_credit(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/application_credits.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates an application credit.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/application_credits.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcredit#create-2020-04
     */
    pub async fn deprecated_202004_create_application_credits(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-04/application_credits.json", None);
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
     * Retrieves a single application credit.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/application_credits/{application_credit_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcredit#show-2020-04
     *
     * **Parameters:**
     *
     * * `application_credit_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202004_get_application_credits_param_credit(
        &self,
        application_credit_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/application_credits/{}/json?{}",
                crate::progenitor_support::encode_path(application_credit_id),
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
     * Retrieves all application credits.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/application_credits.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcredit#index-2020-07
     *
     * **Parameters:**
     *
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202007_get_application_credit(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/application_credits.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates an application credit.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/application_credits.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcredit#create-2020-07
     */
    pub async fn deprecated_202007_create_application_credits(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-07/application_credits.json", None);
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
     * Retrieves a single application credit.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/application_credits/{application_credit_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcredit#show-2020-07
     *
     * **Parameters:**
     *
     * * `application_credit_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202007_get_application_credits_param_credit(
        &self,
        application_credit_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/application_credits/{}/json?{}",
                crate::progenitor_support::encode_path(application_credit_id),
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
     * Retrieves all application credits.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/application_credits.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcredit#index-2020-10
     *
     * **Parameters:**
     *
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn get_application_credit(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/application_credits.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates an application credit.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/application_credits.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcredit#create-2020-10
     */
    pub async fn create_application_credits(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-10/application_credits.json", None);
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
     * Retrieves a single application credit.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/application_credits/{application_credit_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcredit#show-2020-10
     *
     * **Parameters:**
     *
     * * `application_credit_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn get_application_credits_param_credit(
        &self,
        application_credit_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/application_credits/{}/json?{}",
                crate::progenitor_support::encode_path(application_credit_id),
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
     * Retrieves all application credits.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/application_credits.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcredit#index-2021-01
     *
     * **Parameters:**
     *
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202101_get_application_credit(&self, fields: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/application_credits.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates an application credit.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/application_credits.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcredit#create-2021-01
     */
    pub async fn deprecated_202101_create_application_credits(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2021-01/application_credits.json", None);
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
     * Retrieves a single application credit.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/application_credits/{application_credit_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcredit#show-2021-01
     *
     * **Parameters:**
     *
     * * `application_credit_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202101_get_application_credits_param_credit(
        &self,
        application_credit_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/application_credits/{}/json?{}",
                crate::progenitor_support::encode_path(application_credit_id),
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
     * Retrieves all application credits.
     *
     * This function performs a `GET` to the `/admin/api/unstable/application_credits.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcredit#index-unstable
     *
     * **Parameters:**
     *
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_unstable_get_application_credit(
        &self,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/application_credits.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates an application credit.
     *
     * This function performs a `POST` to the `/admin/api/unstable/application_credits.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcredit#create-unstable
     */
    pub async fn deprecated_unstable_create_application_credits(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/unstable/application_credits.json", None);
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
     * Retrieves a single application credit.
     *
     * This function performs a `GET` to the `/admin/api/unstable/application_credits/{application_credit_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/applicationcredit#show-unstable
     *
     * **Parameters:**
     *
     * * `application_credit_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_unstable_get_application_credits_param_credit(
        &self,
        application_credit_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/application_credits/{}/json?{}",
                crate::progenitor_support::encode_path(application_credit_id),
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
     * Retrieves a list of recurring application charges.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/recurring_application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#index-2020-01
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202001_get_recurring_application_charge(
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
            &format!(
                "/admin/api/2020-01/recurring_application_charges.json?{}",
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
     * Creates a recurring application charge.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/recurring_application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#create-2020-01
     */
    pub async fn deprecated_202001_create_recurring_application_charges(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            "/admin/api/2020-01/recurring_application_charges.json",
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
     * Retrieves a single charge.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/recurring_application_charges/{recurring_application_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#show-2020-01
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202001_get_recurring_application_charges_param_charge(
        &self,
        recurring_application_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/recurring_application_charges/{}/json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Cancels a recurring application charge.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/recurring_application_charges/{recurring_application_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     */
    pub async fn deprecated_202001_delete_recurring_application_charges_param_charge(
        &self,
        recurring_application_charge_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/recurring_application_charges/{}/json",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
    * Caution
      This endpoint is no longer required and is deprecated as of
      API version 2021-01.

    Activates a previously accepted recurring application charge. Recurring charges are now
    immediately activated when approved by a merchant.
    *
    * This function performs a `POST` to the `/admin/api/2020-01/recurring_application_charges/{recurring_application_charge_id}/activate.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#activate-2020-01
    *
    * **Parameters:**
    *
    * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
    */
    pub async fn deprecated_202001_create_recurring_application_charges_param_charge_activate(
        &self,
        recurring_application_charge_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/recurring_application_charges/{}/activate.json",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Updates the capped amount of an active recurring application charge.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/recurring_application_charges/{recurring_application_charge_id}/customize.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#customize-2020-01
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `recurring_application_charge_capped_amount: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202001_update_recurring_application_charges_param_charge_customize(
        &self,
        recurring_application_charge_id: &str,
        recurring_application_charge_capped_amount: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if recurring_application_charge_capped_amount > 0 {
            query_args.push((
                "recurring_application_charge[capped_amount]".to_string(),
                recurring_application_charge_capped_amount.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/recurring_application_charges/{}/customize.json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Retrieves a list of recurring application charges.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/recurring_application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#index-2020-04
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202004_get_recurring_application_charge(
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
            &format!(
                "/admin/api/2020-04/recurring_application_charges.json?{}",
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
     * Creates a recurring application charge.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/recurring_application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#create-2020-04
     */
    pub async fn deprecated_202004_create_recurring_application_charges(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            "/admin/api/2020-04/recurring_application_charges.json",
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
     * Retrieves a single charge.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/recurring_application_charges/{recurring_application_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#show-2020-04
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202004_get_recurring_application_charges_param_charge(
        &self,
        recurring_application_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/recurring_application_charges/{}/json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Cancels a recurring application charge.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/recurring_application_charges/{recurring_application_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     */
    pub async fn deprecated_202004_delete_recurring_application_charges_param_charge(
        &self,
        recurring_application_charge_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/recurring_application_charges/{}/json",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
    * Caution
      This endpoint is no longer required and is deprecated as of
      API version 2021-01.

    Activates a previously accepted recurring application charge. Recurring charges are now
    immediately activated when approved by a merchant.
    *
    * This function performs a `POST` to the `/admin/api/2020-04/recurring_application_charges/{recurring_application_charge_id}/activate.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#activate-2020-04
    *
    * **Parameters:**
    *
    * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
    */
    pub async fn deprecated_202004_create_recurring_application_charges_param_charge_activate(
        &self,
        recurring_application_charge_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/recurring_application_charges/{}/activate.json",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Updates the capped amount of an active recurring application charge.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/recurring_application_charges/{recurring_application_charge_id}/customize.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#customize-2020-04
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `recurring_application_charge_capped_amount: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202004_update_recurring_application_charges_param_charge_customize(
        &self,
        recurring_application_charge_id: &str,
        recurring_application_charge_capped_amount: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if recurring_application_charge_capped_amount > 0 {
            query_args.push((
                "recurring_application_charge[capped_amount]".to_string(),
                recurring_application_charge_capped_amount.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/recurring_application_charges/{}/customize.json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Retrieves a list of recurring application charges.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/recurring_application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#index-2020-07
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202007_get_recurring_application_charge(
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
            &format!(
                "/admin/api/2020-07/recurring_application_charges.json?{}",
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
     * Creates a recurring application charge.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/recurring_application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#create-2020-07
     */
    pub async fn deprecated_202007_create_recurring_application_charges(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            "/admin/api/2020-07/recurring_application_charges.json",
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
     * Retrieves a single charge.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/recurring_application_charges/{recurring_application_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#show-2020-07
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202007_get_recurring_application_charges_param_charge(
        &self,
        recurring_application_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/recurring_application_charges/{}/json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Cancels a recurring application charge.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/recurring_application_charges/{recurring_application_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     */
    pub async fn deprecated_202007_delete_recurring_application_charges_param_charge(
        &self,
        recurring_application_charge_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/recurring_application_charges/{}/json",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
    * Caution
      This endpoint is no longer required and is deprecated as of
      API version 2021-01.

    Activates a previously accepted recurring application charge. Recurring charges are now
    immediately activated when approved by a merchant.
    *
    * This function performs a `POST` to the `/admin/api/2020-07/recurring_application_charges/{recurring_application_charge_id}/activate.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#activate-2020-07
    *
    * **Parameters:**
    *
    * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
    */
    pub async fn deprecated_202007_create_recurring_application_charges_param_charge_activate(
        &self,
        recurring_application_charge_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/recurring_application_charges/{}/activate.json",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Updates the capped amount of an active recurring application charge.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/recurring_application_charges/{recurring_application_charge_id}/customize.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#customize-2020-07
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `recurring_application_charge_capped_amount: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202007_update_recurring_application_charges_param_charge_customize(
        &self,
        recurring_application_charge_id: &str,
        recurring_application_charge_capped_amount: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if recurring_application_charge_capped_amount > 0 {
            query_args.push((
                "recurring_application_charge[capped_amount]".to_string(),
                recurring_application_charge_capped_amount.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/recurring_application_charges/{}/customize.json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Retrieves a list of recurring application charges.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/recurring_application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#index-2020-10
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn get_recurring_application_charge(
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
            &format!(
                "/admin/api/2020-10/recurring_application_charges.json?{}",
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
     * Creates a recurring application charge.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/recurring_application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#create-2020-10
     */
    pub async fn create_recurring_application_charges(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            "/admin/api/2020-10/recurring_application_charges.json",
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
     * Retrieves a single charge.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/recurring_application_charges/{recurring_application_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#show-2020-10
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn get_recurring_application_charges_param_charge(
        &self,
        recurring_application_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/recurring_application_charges/{}/json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Cancels a recurring application charge.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/recurring_application_charges/{recurring_application_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     */
    pub async fn delete_recurring_application_charges_param_charge(
        &self,
        recurring_application_charge_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/recurring_application_charges/{}/json",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
    * Caution
      This endpoint is no longer required and is deprecated as of
      API version 2021-01.

    Activates a previously accepted recurring application charge. Recurring charges are now
    immediately activated when approved by a merchant.
    *
    * This function performs a `POST` to the `/admin/api/2020-10/recurring_application_charges/{recurring_application_charge_id}/activate.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#activate-2020-10
    *
    * **Parameters:**
    *
    * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
    */
    pub async fn create_recurring_application_charges_param_charge_activate(
        &self,
        recurring_application_charge_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/recurring_application_charges/{}/activate.json",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Updates the capped amount of an active recurring application charge.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/recurring_application_charges/{recurring_application_charge_id}/customize.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#customize-2020-10
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `recurring_application_charge_capped_amount: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn update_recurring_application_charges_param_charge_customize(
        &self,
        recurring_application_charge_id: &str,
        recurring_application_charge_capped_amount: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if recurring_application_charge_capped_amount > 0 {
            query_args.push((
                "recurring_application_charge[capped_amount]".to_string(),
                recurring_application_charge_capped_amount.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/recurring_application_charges/{}/customize.json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Retrieves a list of recurring application charges.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/recurring_application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#index-2021-01
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202101_get_recurring_application_charge(
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
            &format!(
                "/admin/api/2021-01/recurring_application_charges.json?{}",
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
     * Creates a recurring application charge.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/recurring_application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#create-2021-01
     */
    pub async fn deprecated_202101_create_recurring_application_charges(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            "/admin/api/2021-01/recurring_application_charges.json",
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
     * Retrieves a single charge.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/recurring_application_charges/{recurring_application_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#show-2021-01
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202101_get_recurring_application_charges_param_charge(
        &self,
        recurring_application_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/recurring_application_charges/{}/json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Cancels a recurring application charge.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/recurring_application_charges/{recurring_application_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     */
    pub async fn deprecated_202101_delete_recurring_application_charges_param_charge(
        &self,
        recurring_application_charge_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/recurring_application_charges/{}/json",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Updates the capped amount of an active recurring application charge.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/recurring_application_charges/{recurring_application_charge_id}/customize.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#customize-2021-01
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `recurring_application_charge_capped_amount: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_202101_update_recurring_application_charges_param_charge_customize(
        &self,
        recurring_application_charge_id: &str,
        recurring_application_charge_capped_amount: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if recurring_application_charge_capped_amount > 0 {
            query_args.push((
                "recurring_application_charge[capped_amount]".to_string(),
                recurring_application_charge_capped_amount.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/recurring_application_charges/{}/customize.json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Retrieves a list of recurring application charges.
     *
     * This function performs a `GET` to the `/admin/api/unstable/recurring_application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#index-unstable
     *
     * **Parameters:**
     *
     * * `since_id: &str` -- Restrict results to after the specified ID.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_unstable_get_recurring_application_charge(
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
            &format!(
                "/admin/api/unstable/recurring_application_charges.json?{}",
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
     * Creates a recurring application charge.
     *
     * This function performs a `POST` to the `/admin/api/unstable/recurring_application_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#create-unstable
     */
    pub async fn deprecated_unstable_create_recurring_application_charges(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            "/admin/api/unstable/recurring_application_charges.json",
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
     * Retrieves a single charge.
     *
     * This function performs a `GET` to the `/admin/api/unstable/recurring_application_charges/{recurring_application_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#show-unstable
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_unstable_get_recurring_application_charges_param_charge(
        &self,
        recurring_application_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/recurring_application_charges/{}/json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Cancels a recurring application charge.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/recurring_application_charges/{recurring_application_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#destroy-unstable
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     */
    pub async fn deprecated_unstable_delete_recurring_application_charges_param_charge(
        &self,
        recurring_application_charge_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/recurring_application_charges/{}/json",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Updates the capped amount of an active recurring application charge.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/recurring_application_charges/{recurring_application_charge_id}/customize.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/recurringapplicationcharge#customize-unstable
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `recurring_application_charge_capped_amount: i64` -- recurring_application_charge[capped_amount].
     */
    pub async fn deprecated_unstable_update_recurring_application_charges_param_charge_customize(
        &self,
        recurring_application_charge_id: &str,
        recurring_application_charge_capped_amount: i64,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if recurring_application_charge_capped_amount > 0 {
            query_args.push((
                "recurring_application_charge[capped_amount]".to_string(),
                recurring_application_charge_capped_amount.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/recurring_application_charges/{}/customize.json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Retrieves a list of usage charges.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/recurring_application_charges/{recurring_application_charge_id}/usage_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/usagecharge#index-2020-01
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202001_get_recurring_application_charges_param_charge_usage(
        &self,
        recurring_application_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/recurring_application_charges/{}/usage_charges.json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Creates a usage charge.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/recurring_application_charges/{recurring_application_charge_id}/usage_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/usagecharge#create-2020-01
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     */
    pub async fn deprecated_202001_create_recurring_application_charges_param_charge_usage(
        &self,
        recurring_application_charge_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/recurring_application_charges/{}/usage_charges.json",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Retrieves a single charge.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/recurring_application_charges/{recurring_application_charge_id}/usage_charges/{usage_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/usagecharge#show-2020-01
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `usage_charge_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202001_get_recurring_application_charges_param_charge_usage_billing(
        &self,
        recurring_application_charge_id: &str,
        usage_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/recurring_application_charges/{}/usage_charges/{}/json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
                crate::progenitor_support::encode_path(usage_charge_id),
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
     * Retrieves a list of usage charges.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/recurring_application_charges/{recurring_application_charge_id}/usage_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/usagecharge#index-2020-04
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202004_get_recurring_application_charges_param_charge_usage(
        &self,
        recurring_application_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/recurring_application_charges/{}/usage_charges.json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Creates a usage charge.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/recurring_application_charges/{recurring_application_charge_id}/usage_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/usagecharge#create-2020-04
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     */
    pub async fn deprecated_202004_create_recurring_application_charges_param_charge_usage(
        &self,
        recurring_application_charge_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/recurring_application_charges/{}/usage_charges.json",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Retrieves a single charge.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/recurring_application_charges/{recurring_application_charge_id}/usage_charges/{usage_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/usagecharge#show-2020-04
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `usage_charge_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202004_get_recurring_application_charges_param_charge_usage_billing(
        &self,
        recurring_application_charge_id: &str,
        usage_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/recurring_application_charges/{}/usage_charges/{}/json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
                crate::progenitor_support::encode_path(usage_charge_id),
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
     * Retrieves a list of usage charges.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/recurring_application_charges/{recurring_application_charge_id}/usage_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/usagecharge#index-2020-07
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202007_get_recurring_application_charges_param_charge_usage(
        &self,
        recurring_application_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/recurring_application_charges/{}/usage_charges.json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Creates a usage charge.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/recurring_application_charges/{recurring_application_charge_id}/usage_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/usagecharge#create-2020-07
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     */
    pub async fn deprecated_202007_create_recurring_application_charges_param_charge_usage(
        &self,
        recurring_application_charge_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/recurring_application_charges/{}/usage_charges.json",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Retrieves a single charge.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/recurring_application_charges/{recurring_application_charge_id}/usage_charges/{usage_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/usagecharge#show-2020-07
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `usage_charge_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202007_get_recurring_application_charges_param_charge_usage_billing(
        &self,
        recurring_application_charge_id: &str,
        usage_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/recurring_application_charges/{}/usage_charges/{}/json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
                crate::progenitor_support::encode_path(usage_charge_id),
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
     * Retrieves a list of usage charges.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/recurring_application_charges/{recurring_application_charge_id}/usage_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/usagecharge#index-2020-10
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn get_recurring_application_charges_param_charge_usage(
        &self,
        recurring_application_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/recurring_application_charges/{}/usage_charges.json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Creates a usage charge.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/recurring_application_charges/{recurring_application_charge_id}/usage_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/usagecharge#create-2020-10
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     */
    pub async fn create_recurring_application_charges_param_charge_usage(
        &self,
        recurring_application_charge_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/recurring_application_charges/{}/usage_charges.json",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Retrieves a single charge.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/recurring_application_charges/{recurring_application_charge_id}/usage_charges/{usage_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/usagecharge#show-2020-10
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `usage_charge_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn get_recurring_application_charges_param_charge_usage_billing(
        &self,
        recurring_application_charge_id: &str,
        usage_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/recurring_application_charges/{}/usage_charges/{}/json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
                crate::progenitor_support::encode_path(usage_charge_id),
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
     * Retrieves a list of usage charges.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/recurring_application_charges/{recurring_application_charge_id}/usage_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/usagecharge#index-2021-01
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202101_get_recurring_application_charges_param_charge_usage(
        &self,
        recurring_application_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/recurring_application_charges/{}/usage_charges.json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Creates a usage charge.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/recurring_application_charges/{recurring_application_charge_id}/usage_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/usagecharge#create-2021-01
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     */
    pub async fn deprecated_202101_create_recurring_application_charges_param_charge_usage(
        &self,
        recurring_application_charge_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/recurring_application_charges/{}/usage_charges.json",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Retrieves a single charge.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/recurring_application_charges/{recurring_application_charge_id}/usage_charges/{usage_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/usagecharge#show-2021-01
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `usage_charge_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_202101_get_recurring_application_charges_param_charge_usage_billing(
        &self,
        recurring_application_charge_id: &str,
        usage_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/recurring_application_charges/{}/usage_charges/{}/json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
                crate::progenitor_support::encode_path(usage_charge_id),
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
     * Retrieves a list of usage charges.
     *
     * This function performs a `GET` to the `/admin/api/unstable/recurring_application_charges/{recurring_application_charge_id}/usage_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/usagecharge#index-unstable
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_unstable_get_recurring_application_charges_param_charge_usage(
        &self,
        recurring_application_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/recurring_application_charges/{}/usage_charges.json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Creates a usage charge.
     *
     * This function performs a `POST` to the `/admin/api/unstable/recurring_application_charges/{recurring_application_charge_id}/usage_charges.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/usagecharge#create-unstable
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     */
    pub async fn deprecated_unstable_create_recurring_application_charges_param_charge_usage(
        &self,
        recurring_application_charge_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/recurring_application_charges/{}/usage_charges.json",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
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
     * Retrieves a single charge.
     *
     * This function performs a `GET` to the `/admin/api/unstable/recurring_application_charges/{recurring_application_charge_id}/usage_charges/{usage_charge_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/billing/usagecharge#show-unstable
     *
     * **Parameters:**
     *
     * * `recurring_application_charge_id: &str` -- recurring_application_charge_id.
     * * `usage_charge_id: &str` -- storefront_access_token_id.
     * * `fields: &str` -- A comma-separated list of fields to include in the response.
     */
    pub async fn deprecated_unstable_get_recurring_application_charges_param_charge_usage_billing(
        &self,
        recurring_application_charge_id: &str,
        usage_charge_id: &str,
        fields: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/recurring_application_charges/{}/usage_charges/{}/json?{}",
                crate::progenitor_support::encode_path(recurring_application_charge_id),
                crate::progenitor_support::encode_path(usage_charge_id),
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
