use crate::Client;
use crate::ClientResult;

pub struct Discounts {
    pub client: Client,
}

impl Discounts {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Discounts { client }
    }

    /**
     * Retrieve a list of discount codes. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/price_rules/{price_rule_id}/discount_codes.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#index-2020-01
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_price_rules_param_rule_code(
        &self,
        price_rule_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/price_rules/{}/discount_codes.json",
                crate::progenitor_support::encode_path(price_rule_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a discount code.
     *
     * This function performs a `POST` to the `/admin/api/2020-01/price_rules/{price_rule_id}/discount_codes.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#create-2020-01
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_create_price_rules_param_rule_codes(
        &self,
        price_rule_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/price_rules/{}/discount_codes.json",
                crate::progenitor_support::encode_path(price_rule_id),
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
     * Retrieves a single discount code.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/price_rules/{price_rule_id}/discount_codes/{discount_code_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#show-2020-01
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `discount_code_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_price_rules_param_rule_codes_code(
        &self,
        price_rule_id: &str,
        discount_code_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/price_rules/{}/discount_codes/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(discount_code_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Updates an existing discount code.
     *
     * This function performs a `PUT` to the `/admin/api/2020-01/price_rules/{price_rule_id}/discount_codes/{discount_code_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#update-2020-01
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `discount_code_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_update_price_rules_param_rule_codes_code(
        &self,
        price_rule_id: &str,
        discount_code_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/price_rules/{}/discount_codes/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(discount_code_id),
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
     * Deletes a discount code.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-01/price_rules/{price_rule_id}/discount_codes/{discount_code_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#destroy-2020-01
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `discount_code_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_delete_price_rules_param_rule_codes_code(
        &self,
        price_rule_id: &str,
        discount_code_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/price_rules/{}/discount_codes/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(discount_code_id),
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
    * Retrieves the location of a discount code.
              The discount code's location is returned in the location header, not in the DiscountCode object itself.
                Depending on your HTTP client, the location of the discount code might follow the location header automatically.
    *
    * This function performs a `GET` to the `/admin/api/2020-01/discount_codes/lookup.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#lookup-2020-01
    *
    * **Parameters:**
    *
    * * `code: i64` -- recurring_application_charge[capped_amount].
    */
    pub async fn deprecated_202001_get_codes_lookup(&self, code: i64) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if code > 0 {
            query_args.push(("code".to_string(), code.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-01/discount_codes/lookup.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Creates a discount code creation job.
              The batch endpoint can be used to asynchronously create up to 100 discount codes in a single request. It
              enqueues and returns a discount_code_creation object that can be monitored for completion.
              Response fields that are specific to the batch endpoint include:

                status: The state of the discount code creation job. Possible values are:

                    queued: The job is acknowledged, but not started.
                    running: The job is in process.
                    completed: The job has finished.

                codes_count: The number of discount codes to create.
                imported_count: The number of discount codes created successfully.
                failed_count: The number of discount codes that were not created successfully. Unsuccessful attempts will retry up to three times.
                logs: A report that specifies when no discount codes were created because the provided data was invalid. Example responses:

                    "Price rule target selection can't be blank"
                    "Price rule allocation method can't be blank".
    *
    * This function performs a `POST` to the `/admin/api/2020-01/price_rules/{price_rule_id}/batch.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#batch_create-2020-01
    *
    * **Parameters:**
    *
    * * `price_rule_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202001_create_price_rules_param_rule_batch(
        &self,
        price_rule_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/price_rules/{}/batch.json",
                crate::progenitor_support::encode_path(price_rule_id),
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
     * Retrieves a discount code creation job.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/price_rules/{price_rule_id}/batch/{batch_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#batch_show-2020-01
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `batch_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_price_rules_param_rule_batch(
        &self,
        price_rule_id: &str,
        batch_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/price_rules/{}/batch/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(batch_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Retrieves a list of discount codes for a discount code creation job.
              Discount codes that have been successfully created include a populated id field. Discount codes that
              encountered errors during the creation process include a populated errors field.
    *
    * This function performs a `GET` to the `/admin/api/2020-01/price_rules/{price_rule_id}/batch/{batch_id}/discount_codes.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#batch_discount_codes_index-2020-01
    *
    * **Parameters:**
    *
    * * `price_rule_id: &str` -- storefront_access_token_id.
    * * `batch_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202001_get_price_rules_param_rule_batch_code(
        &self,
        price_rule_id: &str,
        batch_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/price_rules/{}/batch/{}/discount_codes.json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(batch_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve a list of discount codes. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/price_rules/{price_rule_id}/discount_codes.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#index-2020-04
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_price_rules_param_rule_code(
        &self,
        price_rule_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/price_rules/{}/discount_codes.json",
                crate::progenitor_support::encode_path(price_rule_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a discount code.
     *
     * This function performs a `POST` to the `/admin/api/2020-04/price_rules/{price_rule_id}/discount_codes.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#create-2020-04
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_create_price_rules_param_rule_codes(
        &self,
        price_rule_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/price_rules/{}/discount_codes.json",
                crate::progenitor_support::encode_path(price_rule_id),
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
     * Retrieves a single discount code.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/price_rules/{price_rule_id}/discount_codes/{discount_code_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#show-2020-04
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `discount_code_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_price_rules_param_rule_codes_code(
        &self,
        price_rule_id: &str,
        discount_code_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/price_rules/{}/discount_codes/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(discount_code_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Updates an existing discount code.
     *
     * This function performs a `PUT` to the `/admin/api/2020-04/price_rules/{price_rule_id}/discount_codes/{discount_code_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#update-2020-04
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `discount_code_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_update_price_rules_param_rule_codes_code(
        &self,
        price_rule_id: &str,
        discount_code_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/price_rules/{}/discount_codes/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(discount_code_id),
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
     * Deletes a discount code.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-04/price_rules/{price_rule_id}/discount_codes/{discount_code_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#destroy-2020-04
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `discount_code_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_delete_price_rules_param_rule_codes_code(
        &self,
        price_rule_id: &str,
        discount_code_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/price_rules/{}/discount_codes/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(discount_code_id),
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
    * Retrieves the location of a discount code.
              The discount code's location is returned in the location header, not in the DiscountCode object itself.
                Depending on your HTTP client, the location of the discount code might follow the location header automatically.
    *
    * This function performs a `GET` to the `/admin/api/2020-04/discount_codes/lookup.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#lookup-2020-04
    *
    * **Parameters:**
    *
    * * `code: i64` -- recurring_application_charge[capped_amount].
    */
    pub async fn deprecated_202004_get_codes_lookup(&self, code: i64) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if code > 0 {
            query_args.push(("code".to_string(), code.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-04/discount_codes/lookup.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Creates a discount code creation job.
              The batch endpoint can be used to asynchronously create up to 100 discount codes in a single request. It
              enqueues and returns a discount_code_creation object that can be monitored for completion.
              Response fields that are specific to the batch endpoint include:

                status: The state of the discount code creation job. Possible values are:

                    queued: The job is acknowledged, but not started.
                    running: The job is in process.
                    completed: The job has finished.

                codes_count: The number of discount codes to create.
                imported_count: The number of discount codes created successfully.
                failed_count: The number of discount codes that were not created successfully. Unsuccessful attempts will retry up to three times.
                logs: A report that specifies when no discount codes were created because the provided data was invalid. Example responses:

                    "Price rule target selection can't be blank"
                    "Price rule allocation method can't be blank".
    *
    * This function performs a `POST` to the `/admin/api/2020-04/price_rules/{price_rule_id}/batch.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#batch_create-2020-04
    *
    * **Parameters:**
    *
    * * `price_rule_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202004_create_price_rules_param_rule_batch(
        &self,
        price_rule_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/price_rules/{}/batch.json",
                crate::progenitor_support::encode_path(price_rule_id),
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
     * Retrieves a discount code creation job.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/price_rules/{price_rule_id}/batch/{batch_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#batch_show-2020-04
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `batch_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_price_rules_param_rule_batch(
        &self,
        price_rule_id: &str,
        batch_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/price_rules/{}/batch/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(batch_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Retrieves a list of discount codes for a discount code creation job.
              Discount codes that have been successfully created include a populated id field. Discount codes that
              encountered errors during the creation process include a populated errors field.
    *
    * This function performs a `GET` to the `/admin/api/2020-04/price_rules/{price_rule_id}/batch/{batch_id}/discount_codes.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#batch_discount_codes_index-2020-04
    *
    * **Parameters:**
    *
    * * `price_rule_id: &str` -- storefront_access_token_id.
    * * `batch_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202004_get_price_rules_param_rule_batch_code(
        &self,
        price_rule_id: &str,
        batch_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/price_rules/{}/batch/{}/discount_codes.json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(batch_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve a list of discount codes. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/price_rules/{price_rule_id}/discount_codes.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#index-2020-07
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_price_rules_param_rule_code(
        &self,
        price_rule_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/price_rules/{}/discount_codes.json",
                crate::progenitor_support::encode_path(price_rule_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a discount code.
     *
     * This function performs a `POST` to the `/admin/api/2020-07/price_rules/{price_rule_id}/discount_codes.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#create-2020-07
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_create_price_rules_param_rule_codes(
        &self,
        price_rule_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/price_rules/{}/discount_codes.json",
                crate::progenitor_support::encode_path(price_rule_id),
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
     * Retrieves a single discount code.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/price_rules/{price_rule_id}/discount_codes/{discount_code_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#show-2020-07
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `discount_code_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_price_rules_param_rule_codes_code(
        &self,
        price_rule_id: &str,
        discount_code_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/price_rules/{}/discount_codes/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(discount_code_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Updates an existing discount code.
     *
     * This function performs a `PUT` to the `/admin/api/2020-07/price_rules/{price_rule_id}/discount_codes/{discount_code_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#update-2020-07
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `discount_code_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_update_price_rules_param_rule_codes_code(
        &self,
        price_rule_id: &str,
        discount_code_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/price_rules/{}/discount_codes/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(discount_code_id),
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
     * Deletes a discount code.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-07/price_rules/{price_rule_id}/discount_codes/{discount_code_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#destroy-2020-07
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `discount_code_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_delete_price_rules_param_rule_codes_code(
        &self,
        price_rule_id: &str,
        discount_code_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/price_rules/{}/discount_codes/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(discount_code_id),
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
    * Retrieves the location of a discount code.
              The discount code's location is returned in the location header, not in the DiscountCode object itself.
                Depending on your HTTP client, the location of the discount code might follow the location header automatically.
    *
    * This function performs a `GET` to the `/admin/api/2020-07/discount_codes/lookup.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#lookup-2020-07
    *
    * **Parameters:**
    *
    * * `code: i64` -- recurring_application_charge[capped_amount].
    */
    pub async fn deprecated_202007_get_codes_lookup(&self, code: i64) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if code > 0 {
            query_args.push(("code".to_string(), code.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-07/discount_codes/lookup.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Creates a discount code creation job.
              The batch endpoint can be used to asynchronously create up to 100 discount codes in a single request. It
              enqueues and returns a discount_code_creation object that can be monitored for completion.
              Response fields that are specific to the batch endpoint include:

                status: The state of the discount code creation job. Possible values are:

                    queued: The job is acknowledged, but not started.
                    running: The job is in process.
                    completed: The job has finished.

                codes_count: The number of discount codes to create.
                imported_count: The number of discount codes created successfully.
                failed_count: The number of discount codes that were not created successfully. Unsuccessful attempts will retry up to three times.
                logs: A report that specifies when no discount codes were created because the provided data was invalid. Example responses:

                    "Price rule target selection can't be blank"
                    "Price rule allocation method can't be blank".
    *
    * This function performs a `POST` to the `/admin/api/2020-07/price_rules/{price_rule_id}/batch.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#batch_create-2020-07
    *
    * **Parameters:**
    *
    * * `price_rule_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202007_create_price_rules_param_rule_batch(
        &self,
        price_rule_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/price_rules/{}/batch.json",
                crate::progenitor_support::encode_path(price_rule_id),
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
     * Retrieves a discount code creation job.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/price_rules/{price_rule_id}/batch/{batch_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#batch_show-2020-07
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `batch_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_price_rules_param_rule_batch(
        &self,
        price_rule_id: &str,
        batch_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/price_rules/{}/batch/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(batch_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Retrieves a list of discount codes for a discount code creation job.
              Discount codes that have been successfully created include a populated id field. Discount codes that
              encountered errors during the creation process include a populated errors field.
    *
    * This function performs a `GET` to the `/admin/api/2020-07/price_rules/{price_rule_id}/batch/{batch_id}/discount_codes.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#batch_discount_codes_index-2020-07
    *
    * **Parameters:**
    *
    * * `price_rule_id: &str` -- storefront_access_token_id.
    * * `batch_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202007_get_price_rules_param_rule_batch_code(
        &self,
        price_rule_id: &str,
        batch_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/price_rules/{}/batch/{}/discount_codes.json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(batch_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve a list of discount codes. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/price_rules/{price_rule_id}/discount_codes.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#index-2020-10
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     */
    pub async fn get_price_rules_param_rule_code(&self, price_rule_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/price_rules/{}/discount_codes.json",
                crate::progenitor_support::encode_path(price_rule_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a discount code.
     *
     * This function performs a `POST` to the `/admin/api/2020-10/price_rules/{price_rule_id}/discount_codes.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#create-2020-10
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     */
    pub async fn create_price_rules_param_rule_codes(
        &self,
        price_rule_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/price_rules/{}/discount_codes.json",
                crate::progenitor_support::encode_path(price_rule_id),
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
     * Retrieves a single discount code.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/price_rules/{price_rule_id}/discount_codes/{discount_code_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#show-2020-10
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `discount_code_id: &str` -- storefront_access_token_id.
     */
    pub async fn get_price_rules_param_rule_codes_code(
        &self,
        price_rule_id: &str,
        discount_code_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/price_rules/{}/discount_codes/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(discount_code_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Updates an existing discount code.
     *
     * This function performs a `PUT` to the `/admin/api/2020-10/price_rules/{price_rule_id}/discount_codes/{discount_code_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#update-2020-10
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `discount_code_id: &str` -- storefront_access_token_id.
     */
    pub async fn update_price_rules_param_rule_codes_code(
        &self,
        price_rule_id: &str,
        discount_code_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/price_rules/{}/discount_codes/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(discount_code_id),
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
     * Deletes a discount code.
     *
     * This function performs a `DELETE` to the `/admin/api/2020-10/price_rules/{price_rule_id}/discount_codes/{discount_code_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#destroy-2020-10
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `discount_code_id: &str` -- storefront_access_token_id.
     */
    pub async fn delete_price_rules_param_rule_codes_code(
        &self,
        price_rule_id: &str,
        discount_code_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/price_rules/{}/discount_codes/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(discount_code_id),
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
    * Retrieves the location of a discount code.
              The discount code's location is returned in the location header, not in the DiscountCode object itself.
                Depending on your HTTP client, the location of the discount code might follow the location header automatically.
    *
    * This function performs a `GET` to the `/admin/api/2020-10/discount_codes/lookup.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#lookup-2020-10
    *
    * **Parameters:**
    *
    * * `code: i64` -- recurring_application_charge[capped_amount].
    */
    pub async fn get_codes_lookup(&self, code: i64) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if code > 0 {
            query_args.push(("code".to_string(), code.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2020-10/discount_codes/lookup.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Creates a discount code creation job.
              The batch endpoint can be used to asynchronously create up to 100 discount codes in a single request. It
              enqueues and returns a discount_code_creation object that can be monitored for completion.
              Response fields that are specific to the batch endpoint include:

                status: The state of the discount code creation job. Possible values are:

                    queued: The job is acknowledged, but not started.
                    running: The job is in process.
                    completed: The job has finished.

                codes_count: The number of discount codes to create.
                imported_count: The number of discount codes created successfully.
                failed_count: The number of discount codes that were not created successfully. Unsuccessful attempts will retry up to three times.
                logs: A report that specifies when no discount codes were created because the provided data was invalid. Example responses:

                    "Price rule target selection can't be blank"
                    "Price rule allocation method can't be blank".
    *
    * This function performs a `POST` to the `/admin/api/2020-10/price_rules/{price_rule_id}/batch.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#batch_create-2020-10
    *
    * **Parameters:**
    *
    * * `price_rule_id: &str` -- storefront_access_token_id.
    */
    pub async fn create_price_rules_param_rule_batch(
        &self,
        price_rule_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/price_rules/{}/batch.json",
                crate::progenitor_support::encode_path(price_rule_id),
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
     * Retrieves a discount code creation job.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/price_rules/{price_rule_id}/batch/{batch_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#batch_show-2020-10
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `batch_id: &str` -- storefront_access_token_id.
     */
    pub async fn get_price_rules_param_rule_batch(
        &self,
        price_rule_id: &str,
        batch_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/price_rules/{}/batch/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(batch_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Retrieves a list of discount codes for a discount code creation job.
              Discount codes that have been successfully created include a populated id field. Discount codes that
              encountered errors during the creation process include a populated errors field.
    *
    * This function performs a `GET` to the `/admin/api/2020-10/price_rules/{price_rule_id}/batch/{batch_id}/discount_codes.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#batch_discount_codes_index-2020-10
    *
    * **Parameters:**
    *
    * * `price_rule_id: &str` -- storefront_access_token_id.
    * * `batch_id: &str` -- storefront_access_token_id.
    */
    pub async fn get_price_rules_param_rule_batch_code(
        &self,
        price_rule_id: &str,
        batch_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/price_rules/{}/batch/{}/discount_codes.json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(batch_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve a list of discount codes. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/price_rules/{price_rule_id}/discount_codes.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#index-2021-01
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_price_rules_param_rule_code(
        &self,
        price_rule_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/price_rules/{}/discount_codes.json",
                crate::progenitor_support::encode_path(price_rule_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a discount code.
     *
     * This function performs a `POST` to the `/admin/api/2021-01/price_rules/{price_rule_id}/discount_codes.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#create-2021-01
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_create_price_rules_param_rule_codes(
        &self,
        price_rule_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/price_rules/{}/discount_codes.json",
                crate::progenitor_support::encode_path(price_rule_id),
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
     * Retrieves a single discount code.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/price_rules/{price_rule_id}/discount_codes/{discount_code_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#show-2021-01
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `discount_code_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_price_rules_param_rule_codes_code(
        &self,
        price_rule_id: &str,
        discount_code_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/price_rules/{}/discount_codes/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(discount_code_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Updates an existing discount code.
     *
     * This function performs a `PUT` to the `/admin/api/2021-01/price_rules/{price_rule_id}/discount_codes/{discount_code_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#update-2021-01
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `discount_code_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_update_price_rules_param_rule_codes_code(
        &self,
        price_rule_id: &str,
        discount_code_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/price_rules/{}/discount_codes/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(discount_code_id),
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
     * Deletes a discount code.
     *
     * This function performs a `DELETE` to the `/admin/api/2021-01/price_rules/{price_rule_id}/discount_codes/{discount_code_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#destroy-2021-01
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `discount_code_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_delete_price_rules_param_rule_codes_code(
        &self,
        price_rule_id: &str,
        discount_code_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/price_rules/{}/discount_codes/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(discount_code_id),
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
    * Retrieves the location of a discount code.
              The discount code's location is returned in the location header, not in the DiscountCode object itself.
                Depending on your HTTP client, the location of the discount code might follow the location header automatically.
    *
    * This function performs a `GET` to the `/admin/api/2021-01/discount_codes/lookup.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#lookup-2021-01
    *
    * **Parameters:**
    *
    * * `code: i64` -- recurring_application_charge[capped_amount].
    */
    pub async fn deprecated_202101_get_codes_lookup(&self, code: i64) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if code > 0 {
            query_args.push(("code".to_string(), code.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/2021-01/discount_codes/lookup.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Creates a discount code creation job.
              The batch endpoint can be used to asynchronously create up to 100 discount codes in a single request. It
              enqueues and returns a discount_code_creation object that can be monitored for completion.
              Response fields that are specific to the batch endpoint include:

                status: The state of the discount code creation job. Possible values are:

                    queued: The job is acknowledged, but not started.
                    running: The job is in process.
                    completed: The job has finished.

                codes_count: The number of discount codes to create.
                imported_count: The number of discount codes created successfully.
                failed_count: The number of discount codes that were not created successfully. Unsuccessful attempts will retry up to three times.
                logs: A report that specifies when no discount codes were created because the provided data was invalid. Example responses:

                    "Price rule target selection can't be blank"
                    "Price rule allocation method can't be blank".
    *
    * This function performs a `POST` to the `/admin/api/2021-01/price_rules/{price_rule_id}/batch.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#batch_create-2021-01
    *
    * **Parameters:**
    *
    * * `price_rule_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202101_create_price_rules_param_rule_batch(
        &self,
        price_rule_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/price_rules/{}/batch.json",
                crate::progenitor_support::encode_path(price_rule_id),
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
     * Retrieves a discount code creation job.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/price_rules/{price_rule_id}/batch/{batch_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#batch_show-2021-01
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `batch_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_price_rules_param_rule_batch(
        &self,
        price_rule_id: &str,
        batch_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/price_rules/{}/batch/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(batch_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Retrieves a list of discount codes for a discount code creation job.
              Discount codes that have been successfully created include a populated id field. Discount codes that
              encountered errors during the creation process include a populated errors field.
    *
    * This function performs a `GET` to the `/admin/api/2021-01/price_rules/{price_rule_id}/batch/{batch_id}/discount_codes.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#batch_discount_codes_index-2021-01
    *
    * **Parameters:**
    *
    * * `price_rule_id: &str` -- storefront_access_token_id.
    * * `batch_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_202101_get_price_rules_param_rule_batch_code(
        &self,
        price_rule_id: &str,
        batch_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/price_rules/{}/batch/{}/discount_codes.json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(batch_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieve a list of discount codes. Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
     *
     * This function performs a `GET` to the `/admin/api/unstable/price_rules/{price_rule_id}/discount_codes.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#index-unstable
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_price_rules_param_rule_code(
        &self,
        price_rule_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/price_rules/{}/discount_codes.json",
                crate::progenitor_support::encode_path(price_rule_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a discount code.
     *
     * This function performs a `POST` to the `/admin/api/unstable/price_rules/{price_rule_id}/discount_codes.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#create-unstable
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_create_price_rules_param_rule_codes(
        &self,
        price_rule_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/price_rules/{}/discount_codes.json",
                crate::progenitor_support::encode_path(price_rule_id),
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
     * Retrieves a single discount code.
     *
     * This function performs a `GET` to the `/admin/api/unstable/price_rules/{price_rule_id}/discount_codes/{discount_code_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#show-unstable
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `discount_code_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_price_rules_param_rule_codes_code(
        &self,
        price_rule_id: &str,
        discount_code_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/price_rules/{}/discount_codes/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(discount_code_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Updates an existing discount code.
     *
     * This function performs a `PUT` to the `/admin/api/unstable/price_rules/{price_rule_id}/discount_codes/{discount_code_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#update-unstable
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `discount_code_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_update_price_rules_param_rule_codes_code(
        &self,
        price_rule_id: &str,
        discount_code_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/price_rules/{}/discount_codes/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(discount_code_id),
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
     * Deletes a discount code.
     *
     * This function performs a `DELETE` to the `/admin/api/unstable/price_rules/{price_rule_id}/discount_codes/{discount_code_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#destroy-unstable
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `discount_code_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_delete_price_rules_param_rule_codes_code(
        &self,
        price_rule_id: &str,
        discount_code_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/price_rules/{}/discount_codes/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(discount_code_id),
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
    * Retrieves the location of a discount code.
              The discount code's location is returned in the location header, not in the DiscountCode object itself.
                Depending on your HTTP client, the location of the discount code might follow the location header automatically.
    *
    * This function performs a `GET` to the `/admin/api/unstable/discount_codes/lookup.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#lookup-unstable
    *
    * **Parameters:**
    *
    * * `code: i64` -- recurring_application_charge[capped_amount].
    */
    pub async fn deprecated_unstable_get_codes_lookup(&self, code: i64) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if code > 0 {
            query_args.push(("code".to_string(), code.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin/api/unstable/discount_codes/lookup.json?{}", query_),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Creates a discount code creation job.
              The batch endpoint can be used to asynchronously create up to 100 discount codes in a single request. It
              enqueues and returns a discount_code_creation object that can be monitored for completion.
              Response fields that are specific to the batch endpoint include:

                status: The state of the discount code creation job. Possible values are:

                    queued: The job is acknowledged, but not started.
                    running: The job is in process.
                    completed: The job has finished.

                codes_count: The number of discount codes to create.
                imported_count: The number of discount codes created successfully.
                failed_count: The number of discount codes that were not created successfully. Unsuccessful attempts will retry up to three times.
                logs: A report that specifies when no discount codes were created because the provided data was invalid. Example responses:

                    "Price rule target selection can't be blank"
                    "Price rule allocation method can't be blank".
    *
    * This function performs a `POST` to the `/admin/api/unstable/price_rules/{price_rule_id}/batch.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#batch_create-unstable
    *
    * **Parameters:**
    *
    * * `price_rule_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_unstable_create_price_rules_param_rule_batch(
        &self,
        price_rule_id: &str,
        body: &serde_json::Value,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/price_rules/{}/batch.json",
                crate::progenitor_support::encode_path(price_rule_id),
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
     * Retrieves a discount code creation job.
     *
     * This function performs a `GET` to the `/admin/api/unstable/price_rules/{price_rule_id}/batch/{batch_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#batch_show-unstable
     *
     * **Parameters:**
     *
     * * `price_rule_id: &str` -- storefront_access_token_id.
     * * `batch_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_price_rules_param_rule_batch(
        &self,
        price_rule_id: &str,
        batch_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/price_rules/{}/batch/{}/json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(batch_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Retrieves a list of discount codes for a discount code creation job.
              Discount codes that have been successfully created include a populated id field. Discount codes that
              encountered errors during the creation process include a populated errors field.
    *
    * This function performs a `GET` to the `/admin/api/unstable/price_rules/{price_rule_id}/batch/{batch_id}/discount_codes.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/discounts/discountcode#batch_discount_codes_index-unstable
    *
    * **Parameters:**
    *
    * * `price_rule_id: &str` -- storefront_access_token_id.
    * * `batch_id: &str` -- storefront_access_token_id.
    */
    pub async fn deprecated_unstable_get_price_rules_param_rule_batch_code(
        &self,
        price_rule_id: &str,
        batch_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/price_rules/{}/batch/{}/discount_codes.json",
                crate::progenitor_support::encode_path(price_rule_id),
                crate::progenitor_support::encode_path(batch_id),
            ),
            None,
        );
        self.client
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
