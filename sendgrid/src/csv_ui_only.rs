use crate::Client;
use crate::ClientResult;

pub struct CsvUiOnly {
    pub client: Client,
}

impl CsvUiOnly {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CsvUiOnly { client }
    }

    /**
     * Request CSV.
     *
     * This function performs a `POST` to the `/messages/download` endpoint.
     *
     * This is BETA functionality. You may not have access, and we reserve the right to change functionality without notice.
     *
     * This request will kick off a backend process to generate a CSV file. Once generated, the worker will then send an email for the user download the file. The link will expire in 3 days.
     *
     * The CSV fill contain the last 1 million messages. This endpoint will be rate limited to 1 request every 12 hours (rate limit may change).
     *
     * This endpoint is similar to the GET Single Message endpoint - the only difference is that /download is added to indicate that this is a CSV download requests but the same query is used to determine what the CSV should contain.
     *
     * **Parameters:**
     *
     * * `query: &str` -- Uses a SQL like syntax to indicate which messages to include in the CSV.
     * * `authorization: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_messages_download(
        &self,
        query: &str,
    ) -> ClientResult<crate::types::PostMessagesDownloadResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !query.is_empty() {
            query_args.push(("query".to_string(), query.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/messages/download?{}", query_), None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Download CSV.
     *
     * This function performs a `GET` to the `/messages/download/{download_uuid}` endpoint.
     *
     * **This endpoint will return a presigned URL that can be used to download the CSV that was requested from the "Request a CSV" endpoint.**
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_messages_download(
        &self,
        download_uuid: &str,
    ) -> ClientResult<crate::types::GetMessagesDownloadResponse> {
        let url = self.client.url(
            &format!(
                "/messages/download/{}",
                crate::progenitor_support::encode_path(download_uuid),
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
