use crate::Client;
use crate::ClientResult;

pub struct EnvelopeFormData {
    pub client: Client,
}

impl EnvelopeFormData {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EnvelopeFormData { client }
    }

    /**
     * Returns envelope form data for an existing envelope.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/form_data` endpoint.
     *
     * This method downloads the envelope and field data from any in-process, completed, or canceled envelope that you sent or that is shared with you. Recipients who are also full administrators on an account can view form data for any envelopes that another user on the account has sent to them.
     *
     * **Note**: To use this feature, the Sending Setting "Allow sender to download form data" must be enabled for the account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn form_data_get(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<crate::types::EnvelopeFormDataType> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/form_data",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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
