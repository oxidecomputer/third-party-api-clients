use anyhow::Result;

use crate::Client;

pub struct EnvelopeDocumentFields {
    client: Client,
}

impl EnvelopeDocumentFields {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EnvelopeDocumentFields { client }
    }

    /**
     * Gets the custom document fields from an  existing envelope document.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents/{documentId}/fields` endpoint.
     *
     * Retrieves the custom document field information from an existing envelope document.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn document_fields_get(
        &self,
        account_id: &str,
        document_id: &str,
        envelope_id: &str,
    ) -> Result<crate::types::EnvelopeDocumentFields> {
        let url = format!(
            "/v2.1/accounts/{}/envelopes/{}/documents/{}/fields",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&envelope_id.to_string()),
            crate::progenitor_support::encode_path(&document_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Updates existing custom document fields in an existing envelope document.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents/{documentId}/fields` endpoint.
     *
     * Updates existing custom document fields in an existing envelope document.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn document_fields_put(
        &self,
        account_id: &str,
        document_id: &str,
        envelope_id: &str,
        body: &crate::types::EnvelopeDocumentFields,
    ) -> Result<crate::types::EnvelopeDocumentFields> {
        let url = format!(
            "/v2.1/accounts/{}/envelopes/{}/documents/{}/fields",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&envelope_id.to_string()),
            crate::progenitor_support::encode_path(&document_id.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Creates custom document fields in an existing envelope document.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents/{documentId}/fields` endpoint.
     *
     * Creates custom document fields in an existing envelope document.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn document_fields_post(
        &self,
        account_id: &str,
        document_id: &str,
        envelope_id: &str,
        body: &crate::types::EnvelopeDocumentFields,
    ) -> Result<crate::types::EnvelopeDocumentFields> {
        let url = format!(
            "/v2.1/accounts/{}/envelopes/{}/documents/{}/fields",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&envelope_id.to_string()),
            crate::progenitor_support::encode_path(&document_id.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Deletes custom document fields from an existing envelope document.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents/{documentId}/fields` endpoint.
     *
     * Deletes custom document fields from an existing envelope document.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn document_fields_delete(
        &self,
        account_id: &str,
        document_id: &str,
        envelope_id: &str,
        body: &crate::types::EnvelopeDocumentFields,
    ) -> Result<crate::types::EnvelopeDocumentFields> {
        let url = format!(
            "/v2.1/accounts/{}/envelopes/{}/documents/{}/fields",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&envelope_id.to_string()),
            crate::progenitor_support::encode_path(&document_id.to_string()),
        );

        self.client
            .delete(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
