use crate::Client;
use crate::ClientResult;

pub struct TemplateDocuments {
    pub client: Client,
}

impl TemplateDocuments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TemplateDocuments { client }
    }

    /**
     * Gets a list of documents associated with a template.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/templates/{templateId}/documents` endpoint.
     *
     * Retrieves a list of documents associated with the specified template.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include_tabs: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn documents_get_template(
        &self,
        account_id: &str,
        template_id: &str,
        include_tabs: &str,
    ) -> ClientResult<crate::types::TemplateDocumentsResult> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include_tabs.is_empty() {
            query_args.push(("include_tabs".to_string(), include_tabs.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/documents?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
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
     * Adds documents to a template document.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/templates/{templateId}/documents` endpoint.
     *
     * Adds one or more documents to an existing template document.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn documents_put_template(
        &self,
        account_id: &str,
        template_id: &str,
        body: &crate::types::EnvelopeDefinition,
    ) -> ClientResult<crate::types::TemplateDocumentsResult> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/documents",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
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
    /**
     * Deletes documents from a template.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/templates/{templateId}/documents` endpoint.
     *
     * This method deletes one or more documents from an existing template.
     *
     * To delete a document, use only the relevant parts of the [`envelopeDefinition`](#envelopeDefinition).
     * For example, this request body specifies that you want to delete the document whose `documentId` is "1".
     *
     *
     * ```text
     * {
     *   "documents": [
     *     {
     *       "documentId": "1"
     *     }
     *   ]
     * }
     * ```
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn documents_delete_template(
        &self,
        account_id: &str,
        template_id: &str,
        body: &crate::types::EnvelopeDefinition,
    ) -> ClientResult<crate::types::TemplateDocumentsResult> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/documents",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Gets PDF documents from a template.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/templates/{templateId}/documents/{documentId}` endpoint.
     *
     * This method retrieves one or more PDF documents from the template that you specify.
     *
     * You can specify the ID of the document to retrieve, or pass in the value `combined` to retrieve all documents in the template as a single PDF file.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `encrypt: &str` -- When set to **true**, the PDF bytes returned in the response are encrypted for all the key managers configured on your DocuSign account. You can decrypt the documents by using the Key Manager DecryptDocument API method. For more information about Key Manager, see the DocuSign Security Appliance Installation Guide that your organization received from DocuSign.
     * * `show_changes: &str` -- When set to **true**, any document fields that a recipient changed are highlighted in yellow in the returned PDF document, and optional signatures or initials are outlined in red.
     */
    pub async fn documents_get(
        &self,
        account_id: &str,
        document_id: &str,
        template_id: &str,
        encrypt: &str,
        show_changes: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !encrypt.is_empty() {
            query_args.push(("encrypt".to_string(), encrypt.to_string()));
        }
        if !show_changes.is_empty() {
            query_args.push(("show_changes".to_string(), show_changes.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/documents/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
                crate::progenitor_support::encode_path(document_id),
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
     * Updates a template document.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/templates/{templateId}/documents/{documentId}` endpoint.
     *
     * This methods updates an existing template document.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `is_envelope_definition: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn documents_put(
        &self,
        account_id: &str,
        document_id: &str,
        template_id: &str,
        is_envelope_definition: &str,
        body: &crate::types::EnvelopeDefinition,
    ) -> ClientResult<crate::types::EnvelopeDocument> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !is_envelope_definition.is_empty() {
            query_args.push((
                "is_envelope_definition".to_string(),
                is_envelope_definition.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/documents/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
                crate::progenitor_support::encode_path(document_id),
                query_
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
