use crate::Client;
use crate::ClientResult;

pub struct TemplateDocumentFields {
    pub client: Client,
}

impl TemplateDocumentFields {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TemplateDocumentFields { client }
    }

    /**
     * Gets the custom document fields for a an existing template document.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/templates/{templateId}/documents/{documentId}/fields` endpoint.
     *
     * This method retrieves the custom document fields for an existing template document.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn document_fields_get_template(
        &self,
        account_id: &str,
        document_id: &str,
        template_id: &str,
    ) -> ClientResult<crate::types::DocumentFieldsInformation> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/documents/{}/fields",
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
    /**
     * Updates existing custom document fields in an existing template document.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/templates/{templateId}/documents/{documentId}/fields` endpoint.
     *
     * Updates existing custom document fields in an existing template document.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn document_fields_put_template(
        &self,
        account_id: &str,
        document_id: &str,
        template_id: &str,
        body: &crate::types::DocumentFieldsInformation,
    ) -> ClientResult<crate::types::DocumentFieldsInformation> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/documents/{}/fields",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
                crate::progenitor_support::encode_path(document_id),
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
     * Creates custom document fields in an existing template document.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/templates/{templateId}/documents/{documentId}/fields` endpoint.
     *
     * Creates custom document fields in an existing template document.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn document_fields_post_template(
        &self,
        account_id: &str,
        document_id: &str,
        template_id: &str,
        body: &crate::types::DocumentFieldsInformation,
    ) -> ClientResult<crate::types::DocumentFieldsInformation> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/documents/{}/fields",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
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
    /**
     * Deletes custom document fields from an existing template document.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/templates/{templateId}/documents/{documentId}/fields` endpoint.
     *
     * Deletes custom document fields from an existing template document.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn document_fields_delete_template(
        &self,
        account_id: &str,
        document_id: &str,
        template_id: &str,
        body: &crate::types::DocumentFieldsInformation,
    ) -> ClientResult<crate::types::DocumentFieldsInformation> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/documents/{}/fields",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
                crate::progenitor_support::encode_path(document_id),
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
}
