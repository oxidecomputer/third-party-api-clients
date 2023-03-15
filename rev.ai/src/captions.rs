use crate::Client;
use crate::ClientResult;

pub struct Captions {
    pub client: Client,
}

impl Captions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Captions { client }
    }

    /**
     * Get Captions.
     *
     * This function performs a `GET` to the `/jobs/{id}/captions` endpoint.
     *
     * Returns the caption output for a transcription job. We currently support SubRip (SRT) and Web Video Text Tracks (VTT) output.
     * Caption output format can be specified in the `Accept` header. Returns SRT by default.
     * ***
     * Note: For streaming jobs, transient failure of our storage during a live session may prevent the final hypothesis elements from saving properly, resulting in an incomplete caption file. This is rare, but not impossible.
     *
     *
     * **Parameters:**
     *
     * * `accept: crate::types::Accept` -- MIME type specifying the caption output format.
     * * `speaker_channel: i64` -- Identifies which channel of the job output to caption. Default is `null` which works only for jobs with no `speaker_channels_count` provided during job submission.
     */
    pub async fn get(
        &self,
        id: &str,
        accept: crate::types::Accept,
        speaker_channel: i64,
    ) -> ClientResult<String> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if speaker_channel > 0 {
            query_args.push(("speaker_channel".to_string(), speaker_channel.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/jobs/{}/captions?{}",
                crate::progenitor_support::encode_path(id),
                query_
            ),
            None,
        );
        self.client
            .request_with_accept_mime(reqwest::Method::GET, &url, &accept.to_string())
            .await
    }
}
