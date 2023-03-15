use crate::Client;
use crate::ClientResult;

pub struct EnvelopeTemplates {
    pub client: Client,
}

impl EnvelopeTemplates {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EnvelopeTemplates { client }
    }

    /**
     * Gets the templates associated with a document in an existing envelope.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents/{documentId}/templates` endpoint.
     *
     * Retrieves the templates associated with a document in the specified envelope.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include: &str` -- A comma-separated list that limits the results.
     *   Valid values are:
     *   
     *   * `applied`
     *   * `matched`
     *   .
     */
    pub async fn templates_get_document(
        &self,
        account_id: &str,
        document_id: &str,
        envelope_id: &str,
        include: &str,
    ) -> ClientResult<crate::types::TemplateInformation> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include.is_empty() {
            query_args.push(("include".to_string(), include.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/documents/{}/templates?{}",
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
     * Adds templates to a document in an  envelope.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents/{documentId}/templates` endpoint.
     *
     * Adds templates to a document in the specified envelope.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `preserve_template_recipient: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn templates_post_document(
        &self,
        account_id: &str,
        document_id: &str,
        envelope_id: &str,
        preserve_template_recipient: &str,
        body: &crate::types::DocumentTemplateList,
    ) -> ClientResult<crate::types::DocumentTemplateList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !preserve_template_recipient.is_empty() {
            query_args.push((
                "preserve_template_recipient".to_string(),
                preserve_template_recipient.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/documents/{}/templates?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                crate::progenitor_support::encode_path(document_id),
                query_
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
    /**
     * Deletes a template from a document in an existing envelope.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents/{documentId}/templates/{templateId}` endpoint.
     *
     * Deletes the specified template from a document in an existing envelope.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn templates_delete_document(
        &self,
        account_id: &str,
        document_id: &str,
        envelope_id: &str,
        template_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/documents/{}/templates/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                crate::progenitor_support::encode_path(document_id),
                crate::progenitor_support::encode_path(template_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get List of Templates used in an Envelope.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/templates` endpoint.
     *
     * This returns a list of the server-side templates, their name and ID, used in an envelope.
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include: &str` -- The possible value is `matching_applied`, which returns template matching information for the template.
     */
    pub async fn templates_get_envelope(
        &self,
        account_id: &str,
        envelope_id: &str,
        include: &str,
    ) -> ClientResult<crate::types::TemplateInformation> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include.is_empty() {
            query_args.push(("include".to_string(), include.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/templates?{}",
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
     * Adds templates to an envelope.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/templates` endpoint.
     *
     * Adds templates to the specified envelope.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `preserve_template_recipient: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn templates_post_envelope(
        &self,
        account_id: &str,
        envelope_id: &str,
        preserve_template_recipient: &str,
        body: &crate::types::DocumentTemplateList,
    ) -> ClientResult<crate::types::DocumentTemplateList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !preserve_template_recipient.is_empty() {
            query_args.push((
                "preserve_template_recipient".to_string(),
                preserve_template_recipient.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/templates?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                query_
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
