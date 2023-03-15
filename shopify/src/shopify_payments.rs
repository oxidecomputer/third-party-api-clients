use crate::Client;
use crate::ClientResult;

pub struct ShopifyPayments {
    pub client: Client,
}

impl ShopifyPayments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ShopifyPayments { client }
    }

    /**
     * Retrieves the account's current balance.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/shopify_payments/balance.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/balance#show-2020-01
     */
    pub async fn deprecated_202001_get_balance(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-01/shopify_payments/balance.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves the account's current balance.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/shopify_payments/balance.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/balance#show-2020-04
     */
    pub async fn deprecated_202004_get_balance(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-04/shopify_payments/balance.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves the account's current balance.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/shopify_payments/balance.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/balance#show-2020-07
     */
    pub async fn deprecated_202007_get_balance(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-07/shopify_payments/balance.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves the account's current balance.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/shopify_payments/balance.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/balance#show-2020-10
     */
    pub async fn get_balance(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2020-10/shopify_payments/balance.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves the account's current balance.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/shopify_payments/balance.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/balance#show-2021-01
     */
    pub async fn deprecated_202101_get_balance(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2021-01/shopify_payments/balance.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves the account's current balance.
     *
     * This function performs a `GET` to the `/admin/api/unstable/shopify_payments/balance.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/balance#show-unstable
     */
    pub async fn deprecated_unstable_get_balance(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/unstable/shopify_payments/balance.json", None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Retrieve all disputes ordered by initiated_at date and time (ISO 8601 format), with the most recent being first.
              Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
    *
    * This function performs a `GET` to the `/admin/api/2020-01/shopify_payments/disputes.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/dispute#index-2020-01
    *
    * **Parameters:**
    *
    * * `since_id: &str` -- Return only disputes after the specified ID.
    * * `last_id: &str` -- Return only disputes before the specified ID.
    * * `status: &str` -- Return only disputes with the specified status.
    * * `initiated_at: &str` -- Return only disputes with the specified initiated_at date (ISO 8601 format).
    */
    pub async fn deprecated_202001_get_dispute(
        &self,
        since_id: &str,
        last_id: &str,
        status: &str,
        initiated_at: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !initiated_at.is_empty() {
            query_args.push(("initiated_at".to_string(), initiated_at.to_string()));
        }
        if !last_id.is_empty() {
            query_args.push(("last_id".to_string(), last_id.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/shopify_payments/disputes.json?{}",
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
     * Retrieves a single dispute by ID.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/shopify_payments/disputes/{dispute_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/dispute#show-2020-01
     *
     * **Parameters:**
     *
     * * `dispute_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_disputes_param_dispute(
        &self,
        dispute_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/shopify_payments/disputes/{}/json",
                crate::progenitor_support::encode_path(dispute_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Retrieve all disputes ordered by initiated_at date and time (ISO 8601 format), with the most recent being first.
              Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
    *
    * This function performs a `GET` to the `/admin/api/2020-04/shopify_payments/disputes.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/dispute#index-2020-04
    *
    * **Parameters:**
    *
    * * `since_id: &str` -- Return only disputes after the specified ID.
    * * `last_id: &str` -- Return only disputes before the specified ID.
    * * `status: &str` -- Return only disputes with the specified status.
    * * `initiated_at: &str` -- Return only disputes with the specified initiated_at date (ISO 8601 format).
    */
    pub async fn deprecated_202004_get_dispute(
        &self,
        since_id: &str,
        last_id: &str,
        status: &str,
        initiated_at: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !initiated_at.is_empty() {
            query_args.push(("initiated_at".to_string(), initiated_at.to_string()));
        }
        if !last_id.is_empty() {
            query_args.push(("last_id".to_string(), last_id.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/shopify_payments/disputes.json?{}",
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
    * Retrieves a list of all payouts ordered by payout date, with the most recent being first.
              Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
    *
    * This function performs a `GET` to the `/admin/api/2020-01/shopify_payments/payouts.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/payout#index-2020-01
    *
    * **Parameters:**
    *
    * * `since_id: &str` -- Filter the response to payouts made after the specified ID.
    * * `last_id: &str` -- Filter the response to payouts made before the specified ID.
    * * `date_min: &str` -- Filter the response to payouts made inclusively after the specified date.
    * * `date_max: &str` -- Filter the response to payouts made inclusively before the specified date.
    * * `date: &str` -- Filter the response to payouts made on the specified date.
    * * `status: &str` -- Filter the response to payouts made with the specified status.
    */
    pub async fn deprecated_202001_get_payout(
        &self,
        since_id: &str,
        last_id: &str,
        date_min: &str,
        date_max: &str,
        date: &str,
        status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !date.is_empty() {
            query_args.push(("date".to_string(), date.to_string()));
        }
        if !date_max.is_empty() {
            query_args.push(("date_max".to_string(), date_max.to_string()));
        }
        if !date_min.is_empty() {
            query_args.push(("date_min".to_string(), date_min.to_string()));
        }
        if !last_id.is_empty() {
            query_args.push(("last_id".to_string(), last_id.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/shopify_payments/payouts.json?{}",
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
     * Retrieves a single payout by id.
     *
     * This function performs a `GET` to the `/admin/api/2020-01/shopify_payments/payouts/{payout_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/payout#show-2020-01
     *
     * **Parameters:**
     *
     * * `payout_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202001_get_payouts_param_payout(
        &self,
        payout_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/shopify_payments/payouts/{}/json",
                crate::progenitor_support::encode_path(payout_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Retrieves a list of all payouts ordered by payout date, with the most recent being first.
              Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
    *
    * This function performs a `GET` to the `/admin/api/2020-04/shopify_payments/payouts.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/payout#index-2020-04
    *
    * **Parameters:**
    *
    * * `since_id: &str` -- Filter the response to payouts made after the specified ID.
    * * `last_id: &str` -- Filter the response to payouts made before the specified ID.
    * * `date_min: &str` -- Filter the response to payouts made inclusively after the specified date.
    * * `date_max: &str` -- Filter the response to payouts made inclusively before the specified date.
    * * `date: &str` -- Filter the response to payouts made on the specified date.
    * * `status: &str` -- Filter the response to payouts made with the specified status.
    */
    pub async fn deprecated_202004_get_payout(
        &self,
        since_id: &str,
        last_id: &str,
        date_min: &str,
        date_max: &str,
        date: &str,
        status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !date.is_empty() {
            query_args.push(("date".to_string(), date.to_string()));
        }
        if !date_max.is_empty() {
            query_args.push(("date_max".to_string(), date_max.to_string()));
        }
        if !date_min.is_empty() {
            query_args.push(("date_min".to_string(), date_min.to_string()));
        }
        if !last_id.is_empty() {
            query_args.push(("last_id".to_string(), last_id.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/shopify_payments/payouts.json?{}",
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
     * Retrieves a single payout by id.
     *
     * This function performs a `GET` to the `/admin/api/2020-04/shopify_payments/payouts/{payout_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/payout#show-2020-04
     *
     * **Parameters:**
     *
     * * `payout_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202004_get_payouts_param_payout(
        &self,
        payout_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-04/shopify_payments/payouts/{}/json",
                crate::progenitor_support::encode_path(payout_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Retrieves a list of all payouts ordered by payout date, with the most recent being first.
              Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
    *
    * This function performs a `GET` to the `/admin/api/2020-07/shopify_payments/payouts.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/payout#index-2020-07
    *
    * **Parameters:**
    *
    * * `since_id: &str` -- Filter the response to payouts made after the specified ID.
    * * `last_id: &str` -- Filter the response to payouts made before the specified ID.
    * * `date_min: &str` -- Filter the response to payouts made inclusively after the specified date.
    * * `date_max: &str` -- Filter the response to payouts made inclusively before the specified date.
    * * `date: &str` -- Filter the response to payouts made on the specified date.
    * * `status: &str` -- Filter the response to payouts made with the specified status.
    */
    pub async fn deprecated_202007_get_payout(
        &self,
        since_id: &str,
        last_id: &str,
        date_min: &str,
        date_max: &str,
        date: &str,
        status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !date.is_empty() {
            query_args.push(("date".to_string(), date.to_string()));
        }
        if !date_max.is_empty() {
            query_args.push(("date_max".to_string(), date_max.to_string()));
        }
        if !date_min.is_empty() {
            query_args.push(("date_min".to_string(), date_min.to_string()));
        }
        if !last_id.is_empty() {
            query_args.push(("last_id".to_string(), last_id.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/shopify_payments/payouts.json?{}",
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
     * Retrieves a single payout by id.
     *
     * This function performs a `GET` to the `/admin/api/2020-07/shopify_payments/payouts/{payout_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/payout#show-2020-07
     *
     * **Parameters:**
     *
     * * `payout_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202007_get_payouts_param_payout(
        &self,
        payout_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-07/shopify_payments/payouts/{}/json",
                crate::progenitor_support::encode_path(payout_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Retrieves a list of all payouts ordered by payout date, with the most recent being first.
              Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
    *
    * This function performs a `GET` to the `/admin/api/2020-10/shopify_payments/payouts.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/payout#index-2020-10
    *
    * **Parameters:**
    *
    * * `since_id: &str` -- Filter the response to payouts made after the specified ID.
    * * `last_id: &str` -- Filter the response to payouts made before the specified ID.
    * * `date_min: &str` -- Filter the response to payouts made inclusively after the specified date.
    * * `date_max: &str` -- Filter the response to payouts made inclusively before the specified date.
    * * `date: &str` -- Filter the response to payouts made on the specified date.
    * * `status: &str` -- Filter the response to payouts made with the specified status.
    */
    pub async fn get_payout(
        &self,
        since_id: &str,
        last_id: &str,
        date_min: &str,
        date_max: &str,
        date: &str,
        status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !date.is_empty() {
            query_args.push(("date".to_string(), date.to_string()));
        }
        if !date_max.is_empty() {
            query_args.push(("date_max".to_string(), date_max.to_string()));
        }
        if !date_min.is_empty() {
            query_args.push(("date_min".to_string(), date_min.to_string()));
        }
        if !last_id.is_empty() {
            query_args.push(("last_id".to_string(), last_id.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/shopify_payments/payouts.json?{}",
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
     * Retrieves a single payout by id.
     *
     * This function performs a `GET` to the `/admin/api/2020-10/shopify_payments/payouts/{payout_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/payout#show-2020-10
     *
     * **Parameters:**
     *
     * * `payout_id: &str` -- storefront_access_token_id.
     */
    pub async fn get_payouts_param_payout(&self, payout_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2020-10/shopify_payments/payouts/{}/json",
                crate::progenitor_support::encode_path(payout_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Retrieves a list of all payouts ordered by payout date, with the most recent being first.
              Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
    *
    * This function performs a `GET` to the `/admin/api/2021-01/shopify_payments/payouts.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/payout#index-2021-01
    *
    * **Parameters:**
    *
    * * `since_id: &str` -- Filter the response to payouts made after the specified ID.
    * * `last_id: &str` -- Filter the response to payouts made before the specified ID.
    * * `date_min: &str` -- Filter the response to payouts made inclusively after the specified date.
    * * `date_max: &str` -- Filter the response to payouts made inclusively before the specified date.
    * * `date: &str` -- Filter the response to payouts made on the specified date.
    * * `status: &str` -- Filter the response to payouts made with the specified status.
    */
    pub async fn deprecated_202101_get_payout(
        &self,
        since_id: &str,
        last_id: &str,
        date_min: &str,
        date_max: &str,
        date: &str,
        status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !date.is_empty() {
            query_args.push(("date".to_string(), date.to_string()));
        }
        if !date_max.is_empty() {
            query_args.push(("date_max".to_string(), date_max.to_string()));
        }
        if !date_min.is_empty() {
            query_args.push(("date_min".to_string(), date_min.to_string()));
        }
        if !last_id.is_empty() {
            query_args.push(("last_id".to_string(), last_id.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/shopify_payments/payouts.json?{}",
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
     * Retrieves a single payout by id.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/shopify_payments/payouts/{payout_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/payout#show-2021-01
     *
     * **Parameters:**
     *
     * * `payout_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_202101_get_payouts_param_payout(
        &self,
        payout_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/2021-01/shopify_payments/payouts/{}/json",
                crate::progenitor_support::encode_path(payout_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Retrieves a list of all payouts ordered by payout date, with the most recent being first.
              Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
    *
    * This function performs a `GET` to the `/admin/api/unstable/shopify_payments/payouts.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/payout#index-unstable
    *
    * **Parameters:**
    *
    * * `since_id: &str` -- Filter the response to payouts made after the specified ID.
    * * `last_id: &str` -- Filter the response to payouts made before the specified ID.
    * * `date_min: &str` -- Filter the response to payouts made inclusively after the specified date.
    * * `date_max: &str` -- Filter the response to payouts made inclusively before the specified date.
    * * `date: &str` -- Filter the response to payouts made on the specified date.
    * * `status: &str` -- Filter the response to payouts made with the specified status.
    */
    pub async fn deprecated_unstable_get_payout(
        &self,
        since_id: &str,
        last_id: &str,
        date_min: &str,
        date_max: &str,
        date: &str,
        status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !date.is_empty() {
            query_args.push(("date".to_string(), date.to_string()));
        }
        if !date_max.is_empty() {
            query_args.push(("date_max".to_string(), date_max.to_string()));
        }
        if !date_min.is_empty() {
            query_args.push(("date_min".to_string(), date_min.to_string()));
        }
        if !last_id.is_empty() {
            query_args.push(("last_id".to_string(), last_id.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/shopify_payments/payouts.json?{}",
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
     * Retrieves a single payout by id.
     *
     * This function performs a `GET` to the `/admin/api/unstable/shopify_payments/payouts/{payout_id}.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/payout#show-unstable
     *
     * **Parameters:**
     *
     * * `payout_id: &str` -- storefront_access_token_id.
     */
    pub async fn deprecated_unstable_get_payouts_param_payout(
        &self,
        payout_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/api/unstable/shopify_payments/payouts/{}/json",
                crate::progenitor_support::encode_path(payout_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
    * Retrieves a list of all balance transactions ordered by processing
    time, with the most recent being first.
    Note: As of version 2019-10, this endpoint implements pagination by using links that are provided in the response header. Sending the page parameter will return an error. To learn more, see Making requests to paginated REST Admin API endpoints.
    *
    * This function performs a `GET` to the `/admin/api/2020-01/shopify_payments/balance/transactions.json` endpoint.
    *
    * https://shopify.dev/docs/admin-api/rest/reference/shopify_payments/transaction#index-2020-01
    *
    * **Parameters:**
    *
    * * `since_id: &str` -- Filter response to transactions exclusively after the specified ID.
    * * `last_id: &str` -- Filter response to transactions exclusively before the specified ID.
    * * `test: &str` -- Filter response to transactions placed in test mode.
    * * `payout_id: &str` -- Filter response to transactions paid out in the specified payout.
    * * `payout_status: &str` -- Filter response to transactions with the specified payout status.
    */
    pub async fn deprecated_202001_get_balance_transaction(
        &self,
        since_id: &str,
        last_id: &str,
        test: &str,
        payout_id: &str,
        payout_status: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !last_id.is_empty() {
            query_args.push(("last_id".to_string(), last_id.to_string()));
        }
        if !payout_id.is_empty() {
            query_args.push(("payout_id".to_string(), payout_id.to_string()));
        }
        if !payout_status.is_empty() {
            query_args.push(("payout_status".to_string(), payout_status.to_string()));
        }
        if !since_id.is_empty() {
            query_args.push(("since_id".to_string(), since_id.to_string()));
        }
        if !test.is_empty() {
            query_args.push(("test".to_string(), test.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin/api/2020-01/shopify_payments/balance/transactions.json?{}",
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
