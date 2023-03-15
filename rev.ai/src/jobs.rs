use crate::Client;
use crate::ClientResult;

pub struct Jobs {
    pub client: Client,
}

impl Jobs {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Jobs { client }
    }

    /**
     * Get List of Jobs.
     *
     * This function performs a `GET` to the `/jobs` endpoint.
     *
     * Gets a list of transcription jobs submitted within the last 30 days in reverse chronological order up to the provided `limit` number of jobs per call. **Note:** Jobs older than 30 days will not be listed. Pagination is supported via passing the last job `id` from a previous call into `starting_after`.
     *
     * **Parameters:**
     *
     * * `limit: i64` -- Limits the number of jobs returned, default is 100, max is 1000.
     * * `starting_after: &str` -- If specified, returns transcription jobs submitted before the job with this id, exclusive (job with this id is not included).
     */
    pub async fn get_list_of(
        &self,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::JobAllOf>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/jobs?{}", query_), None);
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
     * Get List of Jobs.
     *
     * This function performs a `GET` to the `/jobs` endpoint.
     *
     * As opposed to `get_list_of`, this function returns all the pages of the request at once.
     *
     * Gets a list of transcription jobs submitted within the last 30 days in reverse chronological order up to the provided `limit` number of jobs per call. **Note:** Jobs older than 30 days will not be listed. Pagination is supported via passing the last job `id` from a previous call into `starting_after`.
     */
    pub async fn get_all_list_all_of(
        &self,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::JobAllOf>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/jobs?{}", query_), None);
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Submit Transcription Job.
     *
     * This function performs a `POST` to the `/jobs` endpoint.
     *
     * Starts an asynchronous job to transcribe speech-to-text for a media file. Media files can be specified in two ways, either by including a public url to the media in the transcription job `options` or by uploading a local file as part of a multipart/form request.
     */
    pub async fn submit_transcription(
        &self,
        body: &crate::types::SubmitJobMediaUrlOptionsAllOf,
    ) -> ClientResult<crate::types::JobAllOf> {
        let url = self.client.url("/jobs", None);
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
     * Get Job By Id.
     *
     * This function performs a `GET` to the `/jobs/{id}` endpoint.
     *
     * Returns information about a transcription job
     */
    pub async fn get(&self, id: &str) -> ClientResult<crate::types::JobAllOf> {
        let url = self.client.url(
            &format!("/jobs/{}", crate::progenitor_support::encode_path(id),),
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
     * Delete Job by Id.
     *
     * This function performs a `DELETE` to the `/jobs/{id}` endpoint.
     *
     * Deletes a transcription job. All data related to the job, such as input media and transcript, will be permanently deleted. A job can only be deleted once it's completed (either with success or failure).
     */
    pub async fn delete(&self, id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!("/jobs/{}", crate::progenitor_support::encode_path(id),),
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
