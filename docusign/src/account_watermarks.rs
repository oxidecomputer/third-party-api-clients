use anyhow::Result;

use crate::Client;

pub struct AccountWatermarks {
    pub client: Client,
}

impl AccountWatermarks {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AccountWatermarks { client }
    }

    /**
     * Get watermark information.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/watermark` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn watermark_get(&self, account_id: &str) -> Result<crate::types::Watermark> {
        let url = format!(
            "/v2.1/accounts/{}/watermark",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );
        let url = self.client.url(&url, None);
        self.client.get(&url, None).await
    }
    /**
     * Update watermark information.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/watermark` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn watermark_put(
        &self,
        account_id: &str,
        body: &crate::types::Watermark,
    ) -> Result<crate::types::Watermark> {
        let url = format!(
            "/v2.1/accounts/{}/watermark",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );
        let url = self.client.url(&url, None);
        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
    /**
     * Get watermark preview.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/watermark/preview` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn watermark_preview_put(
        &self,
        account_id: &str,
        body: &crate::types::Watermark,
    ) -> Result<crate::types::Watermark> {
        let url = format!(
            "/v2.1/accounts/{}/watermark/preview",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );
        let url = self.client.url(&url, None);
        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
