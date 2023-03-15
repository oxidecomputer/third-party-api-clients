use crate::Client;
use crate::ClientResult;

pub struct SalesChannels {
    pub client: Client,
}

impl SalesChannels {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SalesChannels { client }
    }

    /**
     * Creates a checkout.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/checkouts.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#create-2020-01
     */
    pub async fn deprecated_202001_create_checkouts(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-01/checkouts.json", None);
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
     * Creates a checkout.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/checkouts.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#create-2020-04
     */
    pub async fn deprecated_202004_create_checkouts(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-04/checkouts.json", None);
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
     * Creates a checkout.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/checkouts.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#create-2020-07
     */
    pub async fn deprecated_202007_create_checkouts(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-07/checkouts.json", None);
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
     * Creates a checkout.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/checkouts.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#create-2020-10
     */
    pub async fn create_checkouts(&self, body: &serde_json::Value) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2020-10/checkouts.json", None);
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
     * Creates a checkout.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/checkouts.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#create-2021-01
     */
    pub async fn deprecated_202101_create_checkouts(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/2021-01/checkouts.json", None);
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
     * Creates a checkout.
     *
     * This function performs a `POST` to the `/admin/api/unstable/checkouts.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#create-unstable
     */
    pub async fn deprecated_unstable_create_checkouts(
        &self,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url("/admin/api/unstable/checkouts.json", None);
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
     * Completes a checkout.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/checkouts/{token}/complete.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#complete-2020-01
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_checkouts_param_token_complete(
        &self,
        token: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/checkouts/{}/complete.json",
                crate::progenitor_support::encode_path(token),
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
     * Retrieves a checkout.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/checkouts/{token}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#show-2020-01
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_checkouts_param_token(
        &self,
        token: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/checkouts/{}/json",
                crate::progenitor_support::encode_path(token),
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
     * Modifies an existing checkout.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/checkouts/{token}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#update-2020-01
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_checkouts_param_token(
        &self,
        token: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/checkouts/{}/json",
                crate::progenitor_support::encode_path(token),
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
    * Retrieves a list of available shipping rates for the specified checkout. Implementers need to poll this endpoint until rates become available.
        Each shipping rate contains the checkout's new subtotal price, total tax, and total price in the event that this shipping rate is selected. This can be used to update the UI without performing further API requests.
        To apply a shipping rate, update the checkout's shipping line with the handle of the selected rate.
    *
    * This function performs a `GET` to the `/admin/api/2020-01/checkouts/{token}/shipping_rates.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#shipping_rates-2020-01
    *
    * **Parameters:**
    *
    * * `token: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202001_get_checkouts_param_token_shipping_rate(
        &self,
        token: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/checkouts/{}/shipping_rates.json",
                crate::progenitor_support::encode_path(token),
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
     * Completes a checkout.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/checkouts/{token}/complete.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#complete-2020-04
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_create_checkouts_param_token_complete(
        &self,
        token: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/checkouts/{}/complete.json",
                crate::progenitor_support::encode_path(token),
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
     * Retrieves a checkout.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/checkouts/{token}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#show-2020-04
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_checkouts_param_token(
        &self,
        token: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/checkouts/{}/json",
                crate::progenitor_support::encode_path(token),
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
     * Modifies an existing checkout.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/checkouts/{token}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#update-2020-04
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_checkouts_param_token(
        &self,
        token: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/checkouts/{}/json",
                crate::progenitor_support::encode_path(token),
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
    * Retrieves a list of available shipping rates for the specified checkout. Implementers need to poll this endpoint until rates become available.
        Each shipping rate contains the checkout's new subtotal price, total tax, and total price in the event that this shipping rate is selected. This can be used to update the UI without performing further API requests.
        To apply a shipping rate, update the checkout's shipping line with the handle of the selected rate.
    *
    * This function performs a `GET` to the `/admin/api/2020-04/checkouts/{token}/shipping_rates.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#shipping_rates-2020-04
    *
    * **Parameters:**
    *
    * * `token: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202004_get_checkouts_param_token_shipping_rate(
        &self,
        token: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/checkouts/{}/shipping_rates.json",
                crate::progenitor_support::encode_path(token),
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
     * Completes a checkout.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/checkouts/{token}/complete.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#complete-2020-07
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_create_checkouts_param_token_complete(
        &self,
        token: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/checkouts/{}/complete.json",
                crate::progenitor_support::encode_path(token),
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
     * Retrieves a checkout.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/checkouts/{token}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#show-2020-07
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_checkouts_param_token(
        &self,
        token: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/checkouts/{}/json",
                crate::progenitor_support::encode_path(token),
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
     * Modifies an existing checkout.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/checkouts/{token}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#update-2020-07
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_checkouts_param_token(
        &self,
        token: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/checkouts/{}/json",
                crate::progenitor_support::encode_path(token),
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
    * Retrieves a list of available shipping rates for the specified checkout. Implementers need to poll this endpoint until rates become available.
        Each shipping rate contains the checkout's new subtotal price, total tax, and total price in the event that this shipping rate is selected. This can be used to update the UI without performing further API requests.
        To apply a shipping rate, update the checkout's shipping line with the handle of the selected rate.
    *
    * This function performs a `GET` to the `/admin/api/2020-07/checkouts/{token}/shipping_rates.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#shipping_rates-2020-07
    *
    * **Parameters:**
    *
    * * `token: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202007_get_checkouts_param_token_shipping_rate(
        &self,
        token: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/checkouts/{}/shipping_rates.json",
                crate::progenitor_support::encode_path(token),
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
     * Completes a checkout.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/checkouts/{token}/complete.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#complete-2020-10
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn create_checkouts_param_token_complete(
        &self,
        token: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/checkouts/{}/complete.json",
                crate::progenitor_support::encode_path(token),
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
     * Retrieves a checkout.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/checkouts/{token}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#show-2020-10
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn get_checkouts_param_token(&self, token: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/checkouts/{}/json",
                crate::progenitor_support::encode_path(token),
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
     * Modifies an existing checkout.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/checkouts/{token}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#update-2020-10
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn update_checkouts_param_token(
        &self,
        token: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/checkouts/{}/json",
                crate::progenitor_support::encode_path(token),
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
    * Retrieves a list of available shipping rates for the specified checkout. Implementers need to poll this endpoint until rates become available.
        Each shipping rate contains the checkout's new subtotal price, total tax, and total price in the event that this shipping rate is selected. This can be used to update the UI without performing further API requests.
        To apply a shipping rate, update the checkout's shipping line with the handle of the selected rate.
    *
    * This function performs a `GET` to the `/admin/api/2020-10/checkouts/{token}/shipping_rates.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#shipping_rates-2020-10
    *
    * **Parameters:**
    *
    * * `token: &str` -- storefront_access_token_id.
    */
    pub async fn get_checkouts_param_token_shipping_rate(&self, token: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/checkouts/{}/shipping_rates.json",
                crate::progenitor_support::encode_path(token),
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
     * Completes a checkout.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/checkouts/{token}/complete.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#complete-2021-01
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_create_checkouts_param_token_complete(
        &self,
        token: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/checkouts/{}/complete.json",
                crate::progenitor_support::encode_path(token),
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
     * Retrieves a checkout.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/checkouts/{token}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#show-2021-01
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_checkouts_param_token(
        &self,
        token: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/checkouts/{}/json",
                crate::progenitor_support::encode_path(token),
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
     * Modifies an existing checkout.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/checkouts/{token}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#update-2021-01
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_checkouts_param_token(
        &self,
        token: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/checkouts/{}/json",
                crate::progenitor_support::encode_path(token),
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
    * Retrieves a list of available shipping rates for the specified checkout. Implementers need to poll this endpoint until rates become available.
        Each shipping rate contains the checkout's new subtotal price, total tax, and total price in the event that this shipping rate is selected. This can be used to update the UI without performing further API requests.
        To apply a shipping rate, update the checkout's shipping line with the handle of the selected rate.
    *
    * This function performs a `GET` to the `/admin/api/2021-01/checkouts/{token}/shipping_rates.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#shipping_rates-2021-01
    *
    * **Parameters:**
    *
    * * `token: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202101_get_checkouts_param_token_shipping_rate(
        &self,
        token: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/checkouts/{}/shipping_rates.json",
                crate::progenitor_support::encode_path(token),
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
     * Completes a checkout.
     *
     * This function performs a `POST` to the `/admin/api/unstable/checkouts/{token}/complete.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#complete-unstable
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_create_checkouts_param_token_complete(
        &self,
        token: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/checkouts/{}/complete.json",
                crate::progenitor_support::encode_path(token),
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
     * Retrieves a checkout.
     *
     * This function performs a `GET` to the `/admin/api/unstable/checkouts/{token}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#show-unstable
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_checkouts_param_token(
        &self,
        token: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/checkouts/{}/json",
                crate::progenitor_support::encode_path(token),
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
     * Modifies an existing checkout.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/checkouts/{token}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#update-unstable
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_checkouts_param_token(
        &self,
        token: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/checkouts/{}/json",
                crate::progenitor_support::encode_path(token),
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
    * Retrieves a list of available shipping rates for the specified checkout. Implementers need to poll this endpoint until rates become available.
        Each shipping rate contains the checkout's new subtotal price, total tax, and total price in the event that this shipping rate is selected. This can be used to update the UI without performing further API requests.
        To apply a shipping rate, update the checkout's shipping line with the handle of the selected rate.
    *
    * This function performs a `GET` to the `/admin/api/unstable/checkouts/{token}/shipping_rates.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/checkout#shipping_rates-unstable
    *
    * **Parameters:**
    *
    * * `token: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_unstable_get_checkouts_param_token_shipping_rate(
        &self,
        token: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/checkouts/{}/shipping_rates.json",
                crate::progenitor_support::encode_path(token),
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
     * Retrieve collection listings that are published to your app. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/collection_listings.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#index-2020-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- Amount of results
     *                     (default: 50, maximum: 1000).
     */
    pub async fn deprecated_202001_get_collection_listing(&self, limit: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/collection_listings.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve product_ids that are published to a collection_id.       Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/collection_listings/{collection_listing_id}/product_ids.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#product_ids-2020-01
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- Amount of results
     *                     (default: 50, maximum: 1000).
     */
    pub async fn deprecated_202001_get_collection_listings_param_listing_product_id(
        &self,
        collection_listing_id: &str,
        limit: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/collection_listings/{}/product_ids.json?{}",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Retrieve a specific collection listing that is published to your app.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/collection_listings/{collection_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#show-2020-01
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_collection_listings_param_listing(
        &self,
        collection_listing_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/collection_listings/{}/json",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Create a collection listing to publish a collection to your app.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/collection_listings/{collection_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#create-2020-01
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_collection_listings_param_listing(
        &self,
        collection_listing_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/collection_listings/{}/json",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Delete a collection listing to unpublish a collection from your app.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/collection_listings/{collection_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_collection_listings_param_listing(
        &self,
        collection_listing_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/collection_listings/{}/json",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Retrieve collection listings that are published to your app. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/collection_listings.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#index-2020-04
     *
     * **Parameters:**
     *
     * * `limit: &str` -- Amount of results
     *                     (default: 50, maximum: 1000).
     */
    pub async fn deprecated_202004_get_collection_listing(&self, limit: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/collection_listings.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve product_ids that are published to a collection_id.       Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/collection_listings/{collection_listing_id}/product_ids.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#product_ids-2020-04
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- Amount of results
     *                     (default: 50, maximum: 1000).
     */
    pub async fn deprecated_202004_get_collection_listings_param_listing_product_id(
        &self,
        collection_listing_id: &str,
        limit: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/collection_listings/{}/product_ids.json?{}",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Retrieve a specific collection listing that is published to your app.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/collection_listings/{collection_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#show-2020-04
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_collection_listings_param_listing(
        &self,
        collection_listing_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/collection_listings/{}/json",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Create a collection listing to publish a collection to your app.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/collection_listings/{collection_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#create-2020-04
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_collection_listings_param_listing(
        &self,
        collection_listing_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/collection_listings/{}/json",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Delete a collection listing to unpublish a collection from your app.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/collection_listings/{collection_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_collection_listings_param_listing(
        &self,
        collection_listing_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/collection_listings/{}/json",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Retrieve collection listings that are published to your app. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/collection_listings.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#index-2020-07
     *
     * **Parameters:**
     *
     * * `limit: &str` -- Amount of results
     *                     (default: 50, maximum: 1000).
     */
    pub async fn deprecated_202007_get_collection_listing(&self, limit: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/collection_listings.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve product_ids that are published to a collection_id.       Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/collection_listings/{collection_listing_id}/product_ids.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#product_ids-2020-07
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- Amount of results
     *                     (default: 50, maximum: 1000).
     */
    pub async fn deprecated_202007_get_collection_listings_param_listing_product_id(
        &self,
        collection_listing_id: &str,
        limit: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/collection_listings/{}/product_ids.json?{}",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Retrieve a specific collection listing that is published to your app.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/collection_listings/{collection_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#show-2020-07
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_collection_listings_param_listing(
        &self,
        collection_listing_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/collection_listings/{}/json",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Create a collection listing to publish a collection to your app.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/collection_listings/{collection_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#create-2020-07
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_collection_listings_param_listing(
        &self,
        collection_listing_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/collection_listings/{}/json",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Delete a collection listing to unpublish a collection from your app.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/collection_listings/{collection_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_collection_listings_param_listing(
        &self,
        collection_listing_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/collection_listings/{}/json",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Retrieve collection listings that are published to your app. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/collection_listings.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#index-2020-10
     *
     * **Parameters:**
     *
     * * `limit: &str` -- Amount of results
     *                     (default: 50, maximum: 1000).
     */
    pub async fn get_collection_listing(&self, limit: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/collection_listings.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve product_ids that are published to a collection_id.       Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/collection_listings/{collection_listing_id}/product_ids.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#product_ids-2020-10
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- Amount of results
     *                     (default: 50, maximum: 1000).
     */
    pub async fn get_collection_listings_param_listing_product_id(
        &self,
        collection_listing_id: &str,
        limit: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/collection_listings/{}/product_ids.json?{}",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Retrieve a specific collection listing that is published to your app.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/collection_listings/{collection_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#show-2020-10
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn get_collection_listings_param_listing(
        &self,
        collection_listing_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/collection_listings/{}/json",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Create a collection listing to publish a collection to your app.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/collection_listings/{collection_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#create-2020-10
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn update_collection_listings_param_listing(
        &self,
        collection_listing_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/collection_listings/{}/json",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Delete a collection listing to unpublish a collection from your app.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/collection_listings/{collection_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_collection_listings_param_listing(
        &self,
        collection_listing_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/collection_listings/{}/json",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Retrieve collection listings that are published to your app. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/collection_listings.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#index-2021-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- Amount of results
     *                     (default: 50, maximum: 1000).
     */
    pub async fn deprecated_202101_get_collection_listing(&self, limit: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/collection_listings.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve product_ids that are published to a collection_id.       Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/collection_listings/{collection_listing_id}/product_ids.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#product_ids-2021-01
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- Amount of results
     *                     (default: 50, maximum: 1000).
     */
    pub async fn deprecated_202101_get_collection_listings_param_listing_product_id(
        &self,
        collection_listing_id: &str,
        limit: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/collection_listings/{}/product_ids.json?{}",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Retrieve a specific collection listing that is published to your app.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/collection_listings/{collection_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#show-2021-01
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_collection_listings_param_listing(
        &self,
        collection_listing_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/collection_listings/{}/json",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Create a collection listing to publish a collection to your app.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/collection_listings/{collection_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#create-2021-01
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_collection_listings_param_listing(
        &self,
        collection_listing_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/collection_listings/{}/json",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Delete a collection listing to unpublish a collection from your app.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/collection_listings/{collection_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_collection_listings_param_listing(
        &self,
        collection_listing_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/collection_listings/{}/json",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Retrieve collection listings that are published to your app. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/collection_listings.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#index-unstable
     *
     * **Parameters:**
     *
     * * `limit: &str` -- Amount of results
     *                     (default: 50, maximum: 1000).
     */
    pub async fn deprecated_unstable_get_collection_listing(
        &self,
        limit: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/collection_listings.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve product_ids that are published to a collection_id.       Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/collection_listings/{collection_listing_id}/product_ids.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#product_ids-unstable
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     * * `limit: &str` -- Amount of results
     *                     (default: 50, maximum: 1000).
     */
    pub async fn deprecated_unstable_get_collection_listings_param_listing_product_id(
        &self,
        collection_listing_id: &str,
        limit: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/collection_listings/{}/product_ids.json?{}",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Retrieve a specific collection listing that is published to your app.
     *
     * This function performs a `GET` to the `/admin/api/unstable/collection_listings/{collection_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#show-unstable
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_collection_listings_param_listing(
        &self,
        collection_listing_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/collection_listings/{}/json",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Create a collection listing to publish a collection to your app.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/collection_listings/{collection_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#create-unstable
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_collection_listings_param_listing(
        &self,
        collection_listing_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/collection_listings/{}/json",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Delete a collection listing to unpublish a collection from your app.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/collection_listings/{collection_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/collectionlisting#destroy-unstable
     *
     * **Parameters:**
     *
     * * `collection_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_collection_listings_param_listing(
        &self,
        collection_listing_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/collection_listings/{}/json",
                crate::progenitor_support::encode_path(collection_listing_id),
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
     * Retrieves a list of payments on a particular checkout.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/checkouts/{token}/payments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#index-2020-01
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_checkouts_param_token_payment(
        &self,
        token: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/checkouts/{}/payments.json",
                crate::progenitor_support::encode_path(token),
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
     * Creates a payment on a checkout using the session ID returned by the card vault.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/checkouts/{token}/payments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#create_payment-2020-01
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     * * `amount_required: &str` -- The amount of the payment.
     * * `request_details_required: &str` -- The details of the request, including the following attributes:
     *                       
     *                           ip_address: The IP address of the customer.
     *                           accept_language: The language preferences of the customer, in the same format as a standard Accept-Language request header.
     *   
     *                           user_agent: The user agent string for the customer's device.
     * * `session_required: &str` -- A session ID provided by the card vault when creating a payment session.
     * * `unique_token_required: &str` -- A unique idempotency token generated by your app. This can be any value, but must be unique across all payment requests.
     */
    pub async fn deprecated_202001_create_checkouts_param_token_payments(
        &self,
        token: &str,
        amount_required: &str,
        request_details_required: &str,
        session_id_required: &str,
        unique_token_required: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !amount_required.is_empty() {
            query_args.push((
                "amount
                  required"
                    .to_string(),
                amount_required.to_string(),
            ));
        }
        if !request_details_required.is_empty() {
            query_args.push((
                "request_details
                  required"
                    .to_string(),
                request_details_required.to_string(),
            ));
        }
        if !session_id_required.is_empty() {
            query_args.push((
                "session_id
                  required"
                    .to_string(),
                session_id_required.to_string(),
            ));
        }
        if !unique_token_required.is_empty() {
            query_args.push((
                "unique_token
                  required"
                    .to_string(),
                unique_token_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/checkouts/{}/payments.json?{}",
                crate::progenitor_support::encode_path(token),
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
     * Retrieves the payment information for an existing payment.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/checkouts/{token}/payments/{payment_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#show-2020-01
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     * * `payment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_checkouts_param_token_payments_payment(
        &self,
        token: &str,
        payment_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/checkouts/{}/payments/{}/json",
                crate::progenitor_support::encode_path(token),
                crate::progenitor_support::encode_path(payment_id),
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
     * Counts the number of payments attempted on a checkout.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/checkouts/{token}/payments/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#count-2020-01
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_checkouts_param_token_payments_count(
        &self,
        token: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/checkouts/{}/payments/count.json",
                crate::progenitor_support::encode_path(token),
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
     * Retrieves a list of payments on a particular checkout.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/checkouts/{token}/payments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#index-2020-04
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_checkouts_param_token_payment(
        &self,
        token: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/checkouts/{}/payments.json",
                crate::progenitor_support::encode_path(token),
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
     * Creates a payment on a checkout using the session ID returned by the card vault.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/checkouts/{token}/payments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#create_payment-2020-04
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     * * `amount_required: &str` -- The amount of the payment.
     * * `request_details_required: &str` -- The details of the request, including the following attributes:
     *                       
     *                           ip_address: The IP address of the customer.
     *                           accept_language: The language preferences of the customer, in the same format as a standard Accept-Language request header.
     *   
     *                           user_agent: The user agent string for the customer's device.
     * * `session_required: &str` -- A session ID provided by the card vault when creating a payment session.
     * * `unique_token_required: &str` -- A unique idempotency token generated by your app. This can be any value, but must be unique across all payment requests.
     */
    pub async fn deprecated_202004_create_checkouts_param_token_payments(
        &self,
        token: &str,
        amount_required: &str,
        request_details_required: &str,
        session_id_required: &str,
        unique_token_required: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !amount_required.is_empty() {
            query_args.push((
                "amount
                  required"
                    .to_string(),
                amount_required.to_string(),
            ));
        }
        if !request_details_required.is_empty() {
            query_args.push((
                "request_details
                  required"
                    .to_string(),
                request_details_required.to_string(),
            ));
        }
        if !session_id_required.is_empty() {
            query_args.push((
                "session_id
                  required"
                    .to_string(),
                session_id_required.to_string(),
            ));
        }
        if !unique_token_required.is_empty() {
            query_args.push((
                "unique_token
                  required"
                    .to_string(),
                unique_token_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/checkouts/{}/payments.json?{}",
                crate::progenitor_support::encode_path(token),
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
     * Retrieves the payment information for an existing payment.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/checkouts/{token}/payments/{payment_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#show-2020-04
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     * * `payment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_checkouts_param_token_payments_payment(
        &self,
        token: &str,
        payment_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/checkouts/{}/payments/{}/json",
                crate::progenitor_support::encode_path(token),
                crate::progenitor_support::encode_path(payment_id),
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
     * Counts the number of payments attempted on a checkout.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/checkouts/{token}/payments/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#count-2020-04
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_checkouts_param_token_payments_count(
        &self,
        token: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/checkouts/{}/payments/count.json",
                crate::progenitor_support::encode_path(token),
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
     * Retrieves a list of payments on a particular checkout.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/checkouts/{token}/payments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#index-2020-07
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_checkouts_param_token_payment(
        &self,
        token: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/checkouts/{}/payments.json",
                crate::progenitor_support::encode_path(token),
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
     * Creates a payment on a checkout using the session ID returned by the card vault.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/checkouts/{token}/payments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#create_payment-2020-07
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     * * `amount_required: &str` -- The amount of the payment.
     * * `request_details_required: &str` -- The details of the request, including the following attributes:
     *                       
     *                           ip_address: The IP address of the customer.
     *                           accept_language: The language preferences of the customer, in the same format as a standard Accept-Language request header.
     *   
     *                           user_agent: The user agent string for the customer's device.
     * * `session_required: &str` -- A session ID provided by the card vault when creating a payment session.
     * * `unique_token_required: &str` -- A unique idempotency token generated by your app. This can be any value, but must be unique across all payment requests.
     */
    pub async fn deprecated_202007_create_checkouts_param_token_payments(
        &self,
        token: &str,
        amount_required: &str,
        request_details_required: &str,
        session_id_required: &str,
        unique_token_required: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !amount_required.is_empty() {
            query_args.push((
                "amount
                  required"
                    .to_string(),
                amount_required.to_string(),
            ));
        }
        if !request_details_required.is_empty() {
            query_args.push((
                "request_details
                  required"
                    .to_string(),
                request_details_required.to_string(),
            ));
        }
        if !session_id_required.is_empty() {
            query_args.push((
                "session_id
                  required"
                    .to_string(),
                session_id_required.to_string(),
            ));
        }
        if !unique_token_required.is_empty() {
            query_args.push((
                "unique_token
                  required"
                    .to_string(),
                unique_token_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/checkouts/{}/payments.json?{}",
                crate::progenitor_support::encode_path(token),
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
     * Retrieves the payment information for an existing payment.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/checkouts/{token}/payments/{payment_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#show-2020-07
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     * * `payment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_checkouts_param_token_payments_payment(
        &self,
        token: &str,
        payment_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/checkouts/{}/payments/{}/json",
                crate::progenitor_support::encode_path(token),
                crate::progenitor_support::encode_path(payment_id),
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
     * Counts the number of payments attempted on a checkout.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/checkouts/{token}/payments/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#count-2020-07
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_checkouts_param_token_payments_count(
        &self,
        token: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/checkouts/{}/payments/count.json",
                crate::progenitor_support::encode_path(token),
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
     * Retrieves a list of payments on a particular checkout.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/checkouts/{token}/payments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#index-2020-10
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn get_checkouts_param_token_payment(&self, token: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/checkouts/{}/payments.json",
                crate::progenitor_support::encode_path(token),
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
     * Creates a payment on a checkout using the session ID returned by the card vault.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/checkouts/{token}/payments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#create_payment-2020-10
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     * * `amount_required: &str` -- The amount of the payment.
     * * `request_details_required: &str` -- The details of the request, including the following attributes:
     *                       
     *                           ip_address: The IP address of the customer.
     *                           accept_language: The language preferences of the customer, in the same format as a standard Accept-Language request header.
     *   
     *                           user_agent: The user agent string for the customer's device.
     * * `session_required: &str` -- A session ID provided by the card vault when creating a payment session.
     * * `unique_token_required: &str` -- A unique idempotency token generated by your app. This can be any value, but must be unique across all payment requests.
     */
    pub async fn create_checkouts_param_token_payments(
        &self,
        token: &str,
        amount_required: &str,
        request_details_required: &str,
        session_id_required: &str,
        unique_token_required: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !amount_required.is_empty() {
            query_args.push((
                "amount
                  required"
                    .to_string(),
                amount_required.to_string(),
            ));
        }
        if !request_details_required.is_empty() {
            query_args.push((
                "request_details
                  required"
                    .to_string(),
                request_details_required.to_string(),
            ));
        }
        if !session_id_required.is_empty() {
            query_args.push((
                "session_id
                  required"
                    .to_string(),
                session_id_required.to_string(),
            ));
        }
        if !unique_token_required.is_empty() {
            query_args.push((
                "unique_token
                  required"
                    .to_string(),
                unique_token_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/checkouts/{}/payments.json?{}",
                crate::progenitor_support::encode_path(token),
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
     * Retrieves the payment information for an existing payment.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/checkouts/{token}/payments/{payment_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#show-2020-10
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     * * `payment_id: &str` -- storefront_access_token_id.
     */
    pub async fn get_checkouts_param_token_payments_payment(
        &self,
        token: &str,
        payment_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/checkouts/{}/payments/{}/json",
                crate::progenitor_support::encode_path(token),
                crate::progenitor_support::encode_path(payment_id),
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
     * Counts the number of payments attempted on a checkout.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/checkouts/{token}/payments/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#count-2020-10
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn get_checkouts_param_token_payments_count(&self, token: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/checkouts/{}/payments/count.json",
                crate::progenitor_support::encode_path(token),
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
     * Retrieves a list of payments on a particular checkout.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/checkouts/{token}/payments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#index-2021-01
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_checkouts_param_token_payment(
        &self,
        token: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/checkouts/{}/payments.json",
                crate::progenitor_support::encode_path(token),
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
     * Creates a payment on a checkout using the session ID returned by the card vault.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/checkouts/{token}/payments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#create_payment-2021-01
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     * * `amount_required: &str` -- The amount of the payment.
     * * `request_details_required: &str` -- The details of the request, including the following attributes:
     *                       
     *                           ip_address: The IP address of the customer.
     *                           accept_language: The language preferences of the customer, in the same format as a standard Accept-Language request header.
     *   
     *                           user_agent: The user agent string for the customer's device.
     * * `session_required: &str` -- A session ID provided by the card vault when creating a payment session.
     * * `unique_token_required: &str` -- A unique idempotency token generated by your app. This can be any value, but must be unique across all payment requests.
     */
    pub async fn deprecated_202101_create_checkouts_param_token_payments(
        &self,
        token: &str,
        amount_required: &str,
        request_details_required: &str,
        session_id_required: &str,
        unique_token_required: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !amount_required.is_empty() {
            query_args.push((
                "amount
                  required"
                    .to_string(),
                amount_required.to_string(),
            ));
        }
        if !request_details_required.is_empty() {
            query_args.push((
                "request_details
                  required"
                    .to_string(),
                request_details_required.to_string(),
            ));
        }
        if !session_id_required.is_empty() {
            query_args.push((
                "session_id
                  required"
                    .to_string(),
                session_id_required.to_string(),
            ));
        }
        if !unique_token_required.is_empty() {
            query_args.push((
                "unique_token
                  required"
                    .to_string(),
                unique_token_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/checkouts/{}/payments.json?{}",
                crate::progenitor_support::encode_path(token),
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
     * Retrieves the payment information for an existing payment.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/checkouts/{token}/payments/{payment_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#show-2021-01
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     * * `payment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_checkouts_param_token_payments_payment(
        &self,
        token: &str,
        payment_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/checkouts/{}/payments/{}/json",
                crate::progenitor_support::encode_path(token),
                crate::progenitor_support::encode_path(payment_id),
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
     * Counts the number of payments attempted on a checkout.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/checkouts/{token}/payments/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#count-2021-01
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_checkouts_param_token_payments_count(
        &self,
        token: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/checkouts/{}/payments/count.json",
                crate::progenitor_support::encode_path(token),
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
     * Retrieves a list of payments on a particular checkout.
     *
     * This function performs a `GET` to the `/admin/api/unstable/checkouts/{token}/payments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#index-unstable
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_checkouts_param_token_payment(
        &self,
        token: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/checkouts/{}/payments.json",
                crate::progenitor_support::encode_path(token),
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
     * Creates a payment on a checkout using the session ID returned by the card vault.
     *
     * This function performs a `POST` to the `/admin/api/unstable/checkouts/{token}/payments.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#create_payment-unstable
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     * * `amount_required: &str` -- The amount of the payment.
     * * `request_details_required: &str` -- The details of the request, including the following attributes:
     *                       
     *                           ip_address: The IP address of the customer.
     *                           accept_language: The language preferences of the customer, in the same format as a standard Accept-Language request header.
     *   
     *                           user_agent: The user agent string for the customer's device.
     * * `session_required: &str` -- A session ID provided by the card vault when creating a payment session.
     * * `unique_token_required: &str` -- A unique idempotency token generated by your app. This can be any value, but must be unique across all payment requests.
     */
    pub async fn deprecated_unstable_create_checkouts_param_token_payments(
        &self,
        token: &str,
        amount_required: &str,
        request_details_required: &str,
        session_id_required: &str,
        unique_token_required: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !amount_required.is_empty() {
            query_args.push((
                "amount
                  required"
                    .to_string(),
                amount_required.to_string(),
            ));
        }
        if !request_details_required.is_empty() {
            query_args.push((
                "request_details
                  required"
                    .to_string(),
                request_details_required.to_string(),
            ));
        }
        if !session_id_required.is_empty() {
            query_args.push((
                "session_id
                  required"
                    .to_string(),
                session_id_required.to_string(),
            ));
        }
        if !unique_token_required.is_empty() {
            query_args.push((
                "unique_token
                  required"
                    .to_string(),
                unique_token_required.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/checkouts/{}/payments.json?{}",
                crate::progenitor_support::encode_path(token),
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
     * Retrieves the payment information for an existing payment.
     *
     * This function performs a `GET` to the `/admin/api/unstable/checkouts/{token}/payments/{payment_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#show-unstable
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     * * `payment_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_checkouts_param_token_payments_payment(
        &self,
        token: &str,
        payment_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/checkouts/{}/payments/{}/json",
                crate::progenitor_support::encode_path(token),
                crate::progenitor_support::encode_path(payment_id),
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
     * Counts the number of payments attempted on a checkout.
     *
     * This function performs a `GET` to the `/admin/api/unstable/checkouts/{token}/payments/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/payment#count-unstable
     *
     * **Parameters:**
     *
     * * `token: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_checkouts_param_token_payments_count(
        &self,
        token: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/checkouts/{}/payments/count.json",
                crate::progenitor_support::encode_path(token),
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
     * Retrieve product listings that are published to your app. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/product_listings.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/productlisting#index-2020-01
     *
     * **Parameters:**
     *
     * * `product_ids: &str` -- A comma-separated list of product ids.
     * * `limit: &str` -- Amount of results
     *                     (default: 50, maximum: 1000).
     * * `collection_id: &str` -- Filter by products belonging to a particular collection.
     * * `updated_at_min: &str` -- Filter by products last updated after a certain date and time (formatted in ISO 8601).
     * * `handle: &str` -- Filter by product handle.
     */
    pub async fn deprecated_202001_get_product_listing(
        &self,
        product_ids: &str,
        limit: &str,
        collection_id: &str,
        updated_at_min: &str,
        handle: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !collection_id.is_empty() {
            query_args.push(("collection_id".to_string(), collection_id.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !product_ids.is_empty() {
            query_args.push(("product_ids".to_string(), product_ids.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/product_listings.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve product_ids that are published to your app. Maximum 1,000 results per page.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/product_listings/product_ids.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/productlisting#product_ids-2020-01
     *
     * **Parameters:**
     *
     * * `limit: &str` -- Amount of results
     *                     (default: 50, maximum: 1000).
     */
    pub async fn deprecated_202001_get_product_listings_id(&self, limit: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/product_listings/product_ids.json?{}",
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
     * Retrieve a count of products that are published to your app.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/product_listings/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/productlisting#count-2020-01
     */
    pub async fn deprecated_202001_get_product_listings_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-01/product_listings/count.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve a specific product listing that is published to your app.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/product_listings/{product_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/productlisting#show-2020-01
     *
     * **Parameters:**
     *
     * * `product_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_product_listings_param_listing(
        &self,
        product_listing_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/product_listings/{}/json",
                crate::progenitor_support::encode_path(product_listing_id),
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
     * Create a product listing to publish a product to your app.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/product_listings/{product_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/productlisting#create-2020-01
     *
     * **Parameters:**
     *
     * * `product_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_product_listings_param_listing(
        &self,
        product_listing_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/product_listings/{}/json",
                crate::progenitor_support::encode_path(product_listing_id),
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
     * Delete a product listing to unpublish a product from your app.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/product_listings/{product_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/productlisting#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `product_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_product_listings_param_listing(
        &self,
        product_listing_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/product_listings/{}/json",
                crate::progenitor_support::encode_path(product_listing_id),
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
     * Retrieve product listings that are published to your app. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/product_listings.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/productlisting#index-2020-04
     *
     * **Parameters:**
     *
     * * `product_ids: &str` -- A comma-separated list of product ids.
     * * `limit: &str` -- Amount of results
     *                     (default: 50, maximum: 1000).
     * * `collection_id: &str` -- Filter by products belonging to a particular collection.
     * * `updated_at_min: &str` -- Filter by products last updated after a certain date and time (formatted in ISO 8601).
     * * `handle: &str` -- Filter by product handle.
     */
    pub async fn deprecated_202004_get_product_listing(
        &self,
        product_ids: &str,
        limit: &str,
        collection_id: &str,
        updated_at_min: &str,
        handle: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !collection_id.is_empty() {
            query_args.push(("collection_id".to_string(), collection_id.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !product_ids.is_empty() {
            query_args.push(("product_ids".to_string(), product_ids.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/product_listings.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve product_ids that are published to your app. Maximum 1,000 results per page.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/product_listings/product_ids.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/productlisting#product_ids-2020-04
     *
     * **Parameters:**
     *
     * * `limit: &str` -- Amount of results
     *                     (default: 50, maximum: 1000).
     */
    pub async fn deprecated_202004_get_product_listings_id(&self, limit: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/product_listings/product_ids.json?{}",
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
     * Retrieve a count of products that are published to your app.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/product_listings/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/productlisting#count-2020-04
     */
    pub async fn deprecated_202004_get_product_listings_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-04/product_listings/count.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve a specific product listing that is published to your app.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/product_listings/{product_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/productlisting#show-2020-04
     *
     * **Parameters:**
     *
     * * `product_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_product_listings_param_listing(
        &self,
        product_listing_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/product_listings/{}/json",
                crate::progenitor_support::encode_path(product_listing_id),
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
     * Create a product listing to publish a product to your app.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/product_listings/{product_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/productlisting#create-2020-04
     *
     * **Parameters:**
     *
     * * `product_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_product_listings_param_listing(
        &self,
        product_listing_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/product_listings/{}/json",
                crate::progenitor_support::encode_path(product_listing_id),
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
     * Delete a product listing to unpublish a product from your app.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/product_listings/{product_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/productlisting#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `product_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_product_listings_param_listing(
        &self,
        product_listing_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/product_listings/{}/json",
                crate::progenitor_support::encode_path(product_listing_id),
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
     * Retrieve product listings that are published to your app. Note: As of version 2019-07, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/product_listings.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/productlisting#index-2020-07
     *
     * **Parameters:**
     *
     * * `product_ids: &str` -- A comma-separated list of product ids.
     * * `limit: &str` -- Amount of results
     *                     (default: 50, maximum: 1000).
     * * `collection_id: &str` -- Filter by products belonging to a particular collection.
     * * `updated_at_min: &str` -- Filter by products last updated after a certain date and time (formatted in ISO 8601).
     * * `handle: &str` -- Filter by product handle.
     */
    pub async fn deprecated_202007_get_product_listing(
        &self,
        product_ids: &str,
        limit: &str,
        collection_id: &str,
        updated_at_min: &str,
        handle: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !collection_id.is_empty() {
            query_args.push(("collection_id".to_string(), collection_id.to_string()));
        }
        if !handle.is_empty() {
            query_args.push(("handle".to_string(), handle.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !product_ids.is_empty() {
            query_args.push(("product_ids".to_string(), product_ids.to_string()));
        }
        if !updated_at_min.is_empty() {
            query_args.push(("updated_at_min".to_string(), updated_at_min.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/product_listings.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve product_ids that are published to your app. Maximum 1,000 results per page.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/product_listings/product_ids.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/productlisting#product_ids-2020-07
     *
     * **Parameters:**
     *
     * * `limit: &str` -- Amount of results
     *                     (default: 50, maximum: 1000).
     */
    pub async fn deprecated_202007_get_product_listings_id(&self, limit: &str) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/product_listings/product_ids.json?{}",
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
     * Retrieve a count of products that are published to your app.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/product_listings/count.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/productlisting#count-2020-07
     */
    pub async fn deprecated_202007_get_product_listings_count(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-07/product_listings/count.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve a specific product listing that is published to your app.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/product_listings/{product_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/productlisting#show-2020-07
     *
     * **Parameters:**
     *
     * * `product_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_product_listings_param_listing(
        &self,
        product_listing_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/product_listings/{}/json",
                crate::progenitor_support::encode_path(product_listing_id),
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
     * Create a product listing to publish a product to your app.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/product_listings/{product_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/productlisting#create-2020-07
     *
     * **Parameters:**
     *
     * * `product_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_product_listings_param_listing(
        &self,
        product_listing_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/product_listings/{}/json",
                crate::progenitor_support::encode_path(product_listing_id),
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
     * Delete a product listing to unpublish a product from your app.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/product_listings/{product_listing_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/sales-channels/productlisting#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `product_listing_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_product_listings_param_listing(
        &self,
        product_listing_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/product_listings/{}/json",
                crate::progenitor_support::encode_path(product_listing_id),
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
