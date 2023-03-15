use crate::Client;
use crate::ClientResult;

pub struct TemplateDocumentHtmlDefinitions {
    pub client: Client,
}

impl TemplateDocumentHtmlDefinitions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TemplateDocumentHtmlDefinitions { client }
    }

    /**
     * Gets the Original HTML Definition used to generate the Responsive HTML for a given document in a template.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/templates/{templateId}/documents/{documentId}/html_definitions` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn responsive_html_get_template_document_definition(
        &self,
        account_id: &str,
        document_id: &str,
        template_id: &str,
    ) -> ClientResult<crate::types::DocumentHtmlDefinitionOriginals> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/documents/{}/html_definitions",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
                crate::progenitor_support::encode_path(document_id),
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
