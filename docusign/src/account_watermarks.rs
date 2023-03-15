use crate::Client;
use crate::ClientResult;

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
    pub async fn watermark_get(&self, account_id: &str) -> ClientResult<crate::types::Watermark> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/watermark",
                crate::progenitor_support::encode_path(account_id),
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
    ) -> ClientResult<crate::types::Watermark> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/watermark",
                crate::progenitor_support::encode_path(account_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
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
    ) -> ClientResult<crate::types::Watermark> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/watermark/preview",
                crate::progenitor_support::encode_path(account_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
}
