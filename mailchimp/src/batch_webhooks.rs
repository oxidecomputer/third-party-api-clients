use crate::Client;
use crate::ClientResult;

pub struct BatchWebhooks {
    pub client: Client,
}

impl BatchWebhooks {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        BatchWebhooks { client }
    }

    /**
     * List batch webhooks.
     *
     * This function performs a `GET` to the `/batch-webhooks` endpoint.
     *
     * Get all webhooks that have been configured for batches.
     *
     * **Parameters:**
     *
     * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
     * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
     * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
     * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
     */
    pub async fn get(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
    ) -> ClientResult<crate::Response<crate::types::BatchWebhooks>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/batch-webhooks?{}", query_), None);
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
     * Add batch webhook.
     *
     * This function performs a `POST` to the `/batch-webhooks` endpoint.
     *
     * Configure a webhook that will fire whenever any batch request completes processing.
     */
    pub async fn post(
        &self,
        body: &crate::types::BatchWebhook,
    ) -> ClientResult<crate::Response<crate::types::Webhooks>> {
        let url = self.client.url("/batch-webhooks", None);
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
     * Get batch webhook info.
     *
     * This function performs a `GET` to the `/batch-webhooks/{batch_webhook_id}` endpoint.
     *
     * Get information about a specific batch webhook.
     *
     * **Parameters:**
     *
     * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
     * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
     * * `batch_webhook_id: &str` -- The unique id for the batch webhook.
     */
    pub async fn get_batch_webhooks(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        batch_webhook_id: &str,
    ) -> ClientResult<crate::Response<crate::types::Webhooks>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/batch-webhooks/{}?{}",
                crate::progenitor_support::encode_path(batch_webhook_id),
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
     * Delete batch webhook.
     *
     * This function performs a `DELETE` to the `/batch-webhooks/{batch_webhook_id}` endpoint.
     *
     * Remove a batch webhook. Webhooks will no longer be sent to the given URL.
     *
     * **Parameters:**
     *
     * * `batch_webhook_id: &str` -- The unique id for the batch webhook.
     */
    pub async fn delete(&self, batch_webhook_id: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/batch-webhooks/{}",
                crate::progenitor_support::encode_path(batch_webhook_id),
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
     * Update batch webhook.
     *
     * This function performs a `PATCH` to the `/batch-webhooks/{batch_webhook_id}` endpoint.
     *
     * Update a webhook that will fire whenever any batch request completes processing.
     *
     * **Parameters:**
     *
     * * `batch_webhook_id: &str` -- The unique id for the batch webhook.
     */
    pub async fn patch(
        &self,
        batch_webhook_id: &str,
        body: &crate::types::BatchWebhook,
    ) -> ClientResult<crate::Response<crate::types::Webhooks>> {
        let url = self.client.url(
            &format!(
                "/batch-webhooks/{}",
                crate::progenitor_support::encode_path(batch_webhook_id),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
}
