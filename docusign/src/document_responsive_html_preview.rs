use crate::Client;
use crate::ClientResult;

pub struct DocumentResponsiveHtmlPreview {
    pub client: Client,
}

impl DocumentResponsiveHtmlPreview {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        DocumentResponsiveHtmlPreview { client }
    }

    /**
     * Creates a preview of the responsive version of a document.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents/{documentId}/responsive_html_preview` endpoint.
     *
     * Creates a preview of the
     * [responsive](https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/responsive/)
     * HTML version of a specific document.
     * This method enables you to preview a PDF document
     * conversion to responsive HTML across device types prior to sending.
     *
     * The request body is a `documentHtmlDefinition` object, which holds the responsive signing parameters that define how to generate the HTML version of the signing document.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn responsive_html_post_document_preview(
        &self,
        account_id: &str,
        document_id: &str,
        envelope_id: &str,
        body: &crate::types::DocumentHtmlDefinition,
    ) -> ClientResult<crate::types::DocumentHtmlDefinitions> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/documents/{}/responsive_html_preview",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                crate::progenitor_support::encode_path(document_id),
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
