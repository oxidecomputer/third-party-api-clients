use anyhow::Result;

use crate::Client;

pub struct Reports {
    pub client: Client,
}

impl Reports {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        Reports {
            client,
        }
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
let url =
format!("/v2.1/accounts/{}/reports",
crate::progenitor_support::encode_path(&account_id.to_string()),);

self.client.get(&url, None).await
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
body: &crate::types::ReportInProductRunRequest
) -> Result<crate::types::ReportInProductSaveResponse> {
let url =
format!("/v2.1/accounts/{}/reports",
crate::progenitor_support::encode_path(&account_id.to_string()),);

self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?))).await
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
body: &crate::types::ReportInProductRunRequest
) -> Result<crate::types::ReportInProductRunResponse> {
let url =
format!("/v2.1/accounts/{}/reports/report_results",
crate::progenitor_support::encode_path(&account_id.to_string()),);

self.client.put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?))).await
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
body: &crate::types::ReportInProductCsvRunRequest
) -> Result<()> {
let url =
format!("/v2.1/accounts/{}/reports/report_results_csv",
crate::progenitor_support::encode_path(&account_id.to_string()),);

self.client.put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?))).await
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
account_id: &str, id: &str,
) -> Result<crate::types::ReportInProductGet> {
let url =
format!("/v2.1/accounts/{}/reports/{}",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&id.to_string()),);

self.client.get(&url, None).await
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
account_id: &str, id: &str,
body: &crate::types::ReportInProductRunRequest
) -> Result<crate::types::ReportInProductSaveResponse> {
let url =
format!("/v2.1/accounts/{}/reports/{}",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&id.to_string()),);

self.client.put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?))).await
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
account_id: &str, id: &str,
) -> Result<crate::types::ReportInProductSaveResponse> {
let url =
format!("/v2.1/accounts/{}/reports/{}",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&id.to_string()),);

self.client.delete(&url, None).await
}


}