use crate::Client;
use crate::ClientResult;

pub struct Transcript {
    pub client: Client,
}

impl Transcript {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Transcript { client }
    }

    /**
     * Get Transcript By Id.
     *
     * This function performs a `GET` to the `/jobs/{id}/transcript` endpoint.
     *
     * Returns the transcript for a completed transcription job. Transcript can be returned as either JSON or plaintext format. Transcript output format can be specified in the `Accept` header. Returns JSON by default.
     * ***
     * Note: For streaming jobs, transient failure of our storage during a live session may prevent the final hypothesis elements from saving properly, resulting in an incomplete transcript. This is rare, but not impossible. To guarantee 100% completeness, we recommend capturing all final hypothesis when you receive them on the client.
     *
     *
     * **Parameters:**
     *
     * * `accept: crate::types::AcceptTranscript` -- MIME type specifying the transcription output format.
     */
    pub async fn get(
        &self,
        id: &str,
        accept: crate::types::AcceptTranscript,
    ) -> ClientResult<String> {
        let url = self.client.url(
            &format!(
                "/jobs/{}/transcript",
                crate::progenitor_support::encode_path(id),
            ),
            None,
        );
        self.client
            .request_with_accept_mime(reqwest::Method::GET, &url, &accept.to_string())
            .await
    }
}
