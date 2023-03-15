use crate::Client;
use crate::ClientResult;

pub struct ChunkedUploads {
    pub client: Client,
}

impl ChunkedUploads {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ChunkedUploads { client }
    }

    /**
     * Initiate a new chunked upload.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/chunked_uploads` endpoint.
     *
     * This method initiates a new chunked upload with the first part of the content.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn post(
        &self,
        account_id: &str,
        body: &crate::types::ChunkedUploadRequest,
    ) -> ClientResult<crate::types::ChunkedUploadResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/chunked_uploads",
                crate::progenitor_support::encode_path(account_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Retrieves metadata about a chunked upload.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/chunked_uploads/{chunkedUploadId}` endpoint.
     *
     * Returns the details (but not the content) about a chunked upload.
     *
     * **Note**: You cannot obtain details about a chunked upload that has expired, been deleted, or consumed by other actions.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `chunked_upload_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include: &str` -- (Optional) This parameter enables you to include additional attribute data in the response. The valid value for this method is `checksum`, which returns an SHA256 checksum of the content of the chunked upload in the response. You can use compare this checksum against your own checksum of the original content to verify that there are no missing parts before you attempt to commit the chunked upload.
     */
    pub async fn get_upload(
        &self,
        account_id: &str,
        chunked_upload_id: &str,
        include: &str,
    ) -> ClientResult<crate::types::ChunkedUploadResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include.is_empty() {
            query_args.push(("include".to_string(), include.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/chunked_uploads/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(chunked_upload_id),
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
     * Commit a chunked upload.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/chunked_uploads/{chunkedUploadId}` endpoint.
     *
     * This method checks the integrity of a chunked upload and then commits it. When this request is successful, the chunked upload is then ready to be referenced in other API calls.
     *
     * If the request is unsuccessful, ensure that you have uploaded all of the parts by using the Update method.
     *
     * **Note**: After you commit a chunked upload, it no longer accepts additional parts.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `chunked_upload_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `action: &str` -- (Required) You must use this query parameter with the value `commit`, which affirms the request to validate and prepare the chunked upload for use with other API calls.
     */
    pub async fn put(
        &self,
        account_id: &str,
        chunked_upload_id: &str,
        action: &str,
    ) -> ClientResult<crate::types::ChunkedUploadResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !action.is_empty() {
            query_args.push(("action".to_string(), action.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/chunked_uploads/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(chunked_upload_id),
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
     * Deletes a chunked upload.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/chunked_uploads/{chunkedUploadId}` endpoint.
     *
     * Deletes a chunked upload that has been committed but not yet consumed.
     *
     * This method cannot be used to delete the following types of chunked uploads, which the system deletes automatically:
     *
     *
     * - Chunked uploads that have been consumed by use in another API call.
     * - Expired chunked uploads.
     *
     * **Note**: If you are aware of a chunked upload that can be discarded, the best practice is to explicitly delete it. If you wait for the system to automatically delete it after it expires, the chunked upload will continue to count against your quota.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `chunked_upload_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn delete_upload(
        &self,
        account_id: &str,
        chunked_upload_id: &str,
    ) -> ClientResult<crate::types::ChunkedUploadResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/chunked_uploads/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(chunked_upload_id),
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
     * Add a chunk to an existing chunked upload.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/chunked_uploads/{chunkedUploadId}/{chunkedUploadPartSeq}` endpoint.
     *
     * Adds a chunk or part to an existing chunked upload. After you use the Create method to initiate a new chunked upload and upload the first part,
     * use this method to upload subsequent parts.
     *
     * For simplicity, we recommend that you upload the parts in their sequential order ( 1,2, 3, 4, etc.). The Create method adds the first part and assigns it the `sequence` value `0`. As a result, we recommend that you start with a `sequence` value of `1` when you use this method, and continue uploading parts contiguously until you have uploaded the entirety of the original content to DocuSign.
     *
     * Example:
     *
     *
     * ```
     * PUT /v2.1/accounts/{accountId}/chunked_uploads/{chunkedUploadId}/1
     * PUT /v2.1/accounts/{accountId}/chunked_uploads/{chunkedUploadId}/2
     * PUT /v2.1/accounts/{accountId}/chunked_uploads/{chunkedUploadId}/3
     * ```
     *
     * **Note**: You cannot replace a part that DocuSign has already received, or add parts to a chunked upload that is already successfully committed.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `chunked_upload_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `chunked_upload_part_seq: &str` -- The sequence or order of the part in the chunked upload. By default, the sequence of the first part that is uploaded as part of the Create request is `0`.
     *   
     *   **Note**: You can add parts out of order. However, the chunked upload must consist of a contiguous series of one or more parts before you can successfully commit it.
     */
    pub async fn put_upload_part(
        &self,
        account_id: &str,
        chunked_upload_id: &str,
        chunked_upload_part_seq: &str,
        body: &crate::types::ChunkedUploadRequest,
    ) -> ClientResult<crate::types::ChunkedUploadResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/chunked_uploads/{}/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(chunked_upload_id),
                crate::progenitor_support::encode_path(chunked_upload_part_seq),
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
