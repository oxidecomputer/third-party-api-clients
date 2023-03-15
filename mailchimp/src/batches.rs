use crate::Client;
use crate::ClientResult;

pub struct Batches {
    pub client: Client,
}

impl Batches {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Batches { client }
    }

    /**
     * List batch requests.
     *
     * This function performs a `GET` to the `/batches` endpoint.
     *
     * Get a summary of batch requests that have been made.
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
    ) -> ClientResult<crate::types::BatchOperations> {
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
        let url = self.client.url(&format!("/batches?{}", query_), None);
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
     * Start batch operation.
     *
     * This function performs a `POST` to the `/batches` endpoint.
     *
     * Begin processing a batch operations request.
     */
    pub async fn post(
        &self,
        body: &crate::types::PostBatchesRequest,
    ) -> ClientResult<crate::types::Batch> {
        let url = self.client.url("/batches", None);
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
     * Get batch operation status.
     *
     * This function performs a `GET` to the `/batches/{batch_id}` endpoint.
     *
     * Get the status of a batch request.
     *
     * **Parameters:**
     *
     * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
     * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
     * * `batch_id: &str` -- The unique id for the batch operation.
     */
    pub async fn get_batches(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        batch_id: &str,
    ) -> ClientResult<crate::types::Batch> {
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
                "/batches/{}?{}",
                crate::progenitor_support::encode_path(batch_id),
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
     * Delete batch request.
     *
     * This function performs a `DELETE` to the `/batches/{batch_id}` endpoint.
     *
     * Stops a batch request from running. Since only one batch request is run at a time, this can be used to cancel a long running request. The results of any completed operations will not be available after this call.
     *
     * **Parameters:**
     *
     * * `batch_id: &str` -- The unique id for the batch operation.
     */
    pub async fn delete(&self, batch_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/batches/{}",
                crate::progenitor_support::encode_path(batch_id),
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
