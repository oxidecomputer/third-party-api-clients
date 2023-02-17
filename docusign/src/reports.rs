use anyhow::Result;

use crate::Client;

pub struct Reports {
    pub client: Client,
}

impl Reports {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Reports { client }
    }

    /**
    * Gets the descriptors for all of
    an account's active reports (for listings).
    *
    * This function performs a `GET` to the `/v2.1/accounts/{accountId}/reports` endpoint.
    *
    *
    *
    * **Parameters:**
    *
    * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    */
    pub async fn product_get_list(
        &self,
        account_id: &str,
    ) -> Result<crate::types::ReportInProductList> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/reports",
                crate::progenitor_support::encode_path(account_id),
            ),
            None,
        );
        self.client.get(&url, None, None).await
    }
    /**
     * Creates a customized report.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/reports` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn product_post_create(
        &self,
        account_id: &str,
        body: &crate::types::ReportInProductRunRequest,
    ) -> Result<crate::types::ReportInProductSaveResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/reports",
                crate::progenitor_support::encode_path(account_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                None,
            )
            .await
    }
    /**
     * Returns the result set from running the specified report.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/reports/report_results` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn product_put_run_results(
        &self,
        account_id: &str,
        body: &crate::types::ReportInProductRunRequest,
    ) -> Result<crate::types::ReportInProductRunResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/reports/report_results",
                crate::progenitor_support::encode_path(account_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                None,
            )
            .await
    }
    /**
     * Returns the specified report as a CSV string.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/reports/report_results_csv` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn product_put_results_csv(
        &self,
        account_id: &str,
        body: &crate::types::ReportInProductCsvRunRequest,
    ) -> Result<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/reports/report_results_csv",
                crate::progenitor_support::encode_path(account_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                Some("application/json"),
            )
            .await
    }
    /**
     * Gets the specified report.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/reports/{id}` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn product_get(
        &self,
        account_id: &str,
        id: &str,
    ) -> Result<crate::types::ReportInProductGet> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/reports/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(id),
            ),
            None,
        );
        self.client.get(&url, None, None).await
    }
    /**
     * Saves a customized report.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/reports/{id}` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn product_put_save(
        &self,
        account_id: &str,
        id: &str,
        body: &crate::types::ReportInProductRunRequest,
    ) -> Result<crate::types::ReportInProductSaveResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/reports/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                None,
            )
            .await
    }
    /**
     * Removes a customized report.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/reports/{id}` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn product_delete(
        &self,
        account_id: &str,
        id: &str,
    ) -> Result<crate::types::ReportInProductSaveResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/reports/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(id),
            ),
            None,
        );
        self.client.delete(&url, None, None).await
    }
}
