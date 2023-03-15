use crate::Client;
use crate::ClientResult;

pub struct EnvelopeDocuments {
    pub client: Client,
}

impl EnvelopeDocuments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EnvelopeDocuments { client }
    }

    /**
     * Gets a list of envelope documents.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents` endpoint.
     *
     * Retrieves a list of documents associated with the specified envelope.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `documents_by_userid: &str` -- When set to **true**, allows recipients to get documents by their user id. For example, if a user is included in two different routing orders with different visibilities, using this parameter returns all of the documents from both routing orders.
     * * `include_document_size: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include_metadata: &str` -- When set to **true**, the response includes metadata that indicates which properties the sender can edit.
     * * `include_tabs: &str` -- When set to **true**, information about the tabs associated with the documents are included in the response.
     * * `recipient_id: &str` -- Allows the sender to retrieve the documents as one of the recipients that they control. The `documents_by_userid` parameter must be set to **false** for this to work.
     * * `shared_user_id: &str` -- The ID of a shared user that you want to impersonate in order to retrieve their view of the list of documents. This parameter is used in the context of a shared inbox (i.e., when you share envelopes from one user to another through the RADmin console).
     */
    pub async fn documents_get(
        &self,
        account_id: &str,
        envelope_id: &str,
        documents_by_userid: &str,
        include_document_size: &str,
        include_metadata: &str,
        include_tabs: &str,
        recipient_id: &str,
        shared_user_id: &str,
    ) -> ClientResult<crate::types::EnvelopeDocumentsResult> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !documents_by_userid.is_empty() {
            query_args.push((
                "documents_by_userid".to_string(),
                documents_by_userid.to_string(),
            ));
        }
        if !include_document_size.is_empty() {
            query_args.push((
                "include_document_size".to_string(),
                include_document_size.to_string(),
            ));
        }
        if !include_metadata.is_empty() {
            query_args.push(("include_metadata".to_string(), include_metadata.to_string()));
        }
        if !include_tabs.is_empty() {
            query_args.push(("include_tabs".to_string(), include_tabs.to_string()));
        }
        if !recipient_id.is_empty() {
            query_args.push(("recipient_id".to_string(), recipient_id.to_string()));
        }
        if !shared_user_id.is_empty() {
            query_args.push(("shared_user_id".to_string(), shared_user_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/documents?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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
     * Adds one or more documents to an existing envelope document.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents` endpoint.
     *
     * Adds one or more documents to an existing envelope document.
     * <p>**Note**: When adding or modifying documents for an in-process envelope, DocuSign recommends locking the envelope prior to making any changes.
     *
     * If the file name of a document contains unicode characters, you need to include a `Content-Disposition` header. Example:
     *
     *
     * **Header**: `Content-Disposition`
     *
     *
     * **Value**: `file; filename=\"name\";fileExtension=ext;documentId=1`
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn documents_put(
        &self,
        account_id: &str,
        envelope_id: &str,
        body: &crate::types::EnvelopeDefinition,
    ) -> ClientResult<crate::types::EnvelopeDocumentsResult> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/documents",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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
     * Deletes documents from a draft envelope.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents` endpoint.
     *
     * Deletes one or more documents from an existing envelope that has not yet been completed.
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
     * The envelope status must be one of:
     *
     * - `created`
     * - `sent`
     * - `delivered`
     *
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn documents_delete(
        &self,
        account_id: &str,
        envelope_id: &str,
        body: &crate::types::EnvelopeDefinition,
    ) -> ClientResult<crate::types::EnvelopeDocumentsResult> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/documents",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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
     * Gets a document from an envelope.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents/{documentId}` endpoint.
     *
     * Retrieves the specified document from the envelope. If the account has the Highlight Data Changes feature enabled, there is an option to request that any changes in the envelope be highlighted.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- This parameter takes the following special keywords:
     *   
     *   - `combined`: Retrieves a PDF file that contains the combined content of all of the documents. If the account option **Attach certification of completion to envelope** is on, then the Certificate of Completion is also included in the PDF file. You set this account option in the Admin tool on the **Signing Settings** screen, or by setting the `attachCompletedEnvelope` property in the `accountSettings` object to **true**.
     *   - `archive`: Retrieves a ZIP archive that contains all of the PDF documents and the Certificate of Completion.
     *   .
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `certificate: &str` -- When set to **false**, the envelope signing certificate is removed from the download.
     * * `documents_by_userid: &str` -- When set to **true**, allows recipients to get documents by their user id. For example, if a user is included in two different routing orders with different visibilities, using this parameter returns all of the documents from both routing orders.
     * * `encoding: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `encrypt: &str` -- When set to **true**, the PDF bytes returned in the response are encrypted for all the key managers configured on your DocuSign account. You can decrypt the documents by using the Key Manager DecryptDocument API method. For more information about Key Manager, see the DocuSign Security Appliance Installation Guide that your organization received from DocuSign.
     * * `language: &str` -- Specifies the language for the Certificate of Completion in the response. The supported languages are: Chinese Simplified (zh_CN), Chinese Traditional (zh_TW), Dutch (nl), English US (en), French (fr), German (de), Italian (it), Japanese (ja), Korean (ko), Portuguese (pt), Portuguese (Brazil) (pt_BR), Russian (ru), Spanish (es). .
     * * `recipient_id: &str` -- Allows the sender to retrieve the documents as one of the recipients that they control. The `documents_by_userid` parameter must be set to **false** for this functionality to work.
     * * `shared_user_id: &str` -- The ID of a shared user that you want to impersonate in order to retrieve their view of the list of documents. This parameter is used in the context of a shared inbox (i.e., when you share envelopes from one user to another through the RADmin console).
     * * `show_changes: &str` -- When set to **true**, any changed fields for the returned PDF are highlighted in yellow and optional signatures or initials outlined in red. .
     * * `watermark: &str` -- When set to **true**, the account has the watermark feature enabled, and the envelope is not complete, then the watermark for the account is added to the PDF documents. This option can remove the watermark. .
     */
    pub async fn documents_get_document(
        &self,
        account_id: &str,
        document_id: &str,
        envelope_id: &str,
        certificate: &str,
        documents_by_userid: &str,
        encoding: &str,
        encrypt: &str,
        language: &str,
        recipient_id: &str,
        shared_user_id: &str,
        show_changes: &str,
        watermark: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !certificate.is_empty() {
            query_args.push(("certificate".to_string(), certificate.to_string()));
        }
        if !documents_by_userid.is_empty() {
            query_args.push((
                "documents_by_userid".to_string(),
                documents_by_userid.to_string(),
            ));
        }
        if !encoding.is_empty() {
            query_args.push(("encoding".to_string(), encoding.to_string()));
        }
        if !encrypt.is_empty() {
            query_args.push(("encrypt".to_string(), encrypt.to_string()));
        }
        if !language.is_empty() {
            query_args.push(("language".to_string(), language.to_string()));
        }
        if !recipient_id.is_empty() {
            query_args.push(("recipient_id".to_string(), recipient_id.to_string()));
        }
        if !shared_user_id.is_empty() {
            query_args.push(("shared_user_id".to_string(), shared_user_id.to_string()));
        }
        if !show_changes.is_empty() {
            query_args.push(("show_changes".to_string(), show_changes.to_string()));
        }
        if !watermark.is_empty() {
            query_args.push(("watermark".to_string(), watermark.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/documents/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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
     * Adds a document to an existing draft envelope.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents/{documentId}` endpoint.
     *
     * Adds a document to an existing draft envelope. The bytes of the document make up the body of the request.
     *
     *
     *
     * **Note**: When adding or modifying documents for an in-process envelope, DocuSign recommends locking the envelope prior to making any changes.
     *
     *
     *
     *
     * If the file name of the document contains unicode characters, you need to include a `Content-Disposition` header. Example:
     *
     *
     * **Header**: `Content-Disposition`
     *
     *
     * **Value**: `file; filename=\"name\";fileExtension=ext;documentId=1`
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn documents_put_document(
        &self,
        account_id: &str,
        document_id: &str,
        envelope_id: &str,
    ) -> ClientResult<crate::types::EnvelopeDocument> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/documents/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                crate::progenitor_support::encode_path(document_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}
