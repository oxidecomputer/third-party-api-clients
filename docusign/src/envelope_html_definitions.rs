use crate::Client;
use crate::ClientResult;

pub struct EnvelopeHtmlDefinitions {
    pub client: Client,
}

impl EnvelopeHtmlDefinitions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EnvelopeHtmlDefinitions { client }
    }

    /**
     * Gets the Original HTML Definition used to generate the Responsive HTML for the envelope.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/html_definitions` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn responsive_html_get_envelope_definition(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<crate::types::DocumentHtmlDefinitionOriginals> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/html_definitions",
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
