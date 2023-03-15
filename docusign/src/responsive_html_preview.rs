use crate::Client;
use crate::ClientResult;

pub struct ResponsiveHtmlPreview {
    pub client: Client,
}

impl ResponsiveHtmlPreview {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ResponsiveHtmlPreview { client }
    }

    /**
     * Creates a preview of the responsive versions of all of the documents in an envelope.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/responsive_html_preview` endpoint.
     *
     * Creates a preview of the
     * [responsive](https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/responsive/),
     * HTML versions of all of the documents in an
     * envelope. This method enables you to preview the
     * PDF document conversions to responsive HTML across
     * device types prior to sending.
     *
     * The request body is a `documentHtmlDefinition`
     * object, which holds the responsive signing
     * parameters that define how to generate the HTML
     * version of the documents.
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn responsive_html_post_preview(
        &self,
        account_id: &str,
        envelope_id: &str,
        body: &crate::types::DocumentHtmlDefinition,
    ) -> ClientResult<crate::types::DocumentHtmlDefinitions> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/responsive_html_preview",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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
}
