use crate::Client;
use crate::ClientResult;

pub struct ApplianceInfo {
    pub client: Client,
}

impl ApplianceInfo {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ApplianceInfo { client }
    }

    /**
     * Returns document pages for Display Appliance.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/display_appliance_info/dynamicsystemsettings` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_get_dynamic_system_setting(
        &self,
        account_id: &str,
    ) -> ClientResult<crate::types::ApplianceInfo> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/display_appliance_info/dynamicsystemsettings",
                crate::progenitor_support::encode_path(account_id),
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
     * Returns whether a template was encrypted by Display Appliance.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/display_appliance_info/templateInfo` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_get_template(
        &self,
        account_id: &str,
    ) -> ClientResult<crate::types::ApplianceInfo> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/display_appliance_info/templateInfo",
                crate::progenitor_support::encode_path(account_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Returns envelope and recipient information for Display Appliance.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_get(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<crate::types::ApplianceInfo> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info",
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
    /**
     * Returns envelope account information for Display Appliance.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info/account_info` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_get_account(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<crate::types::DisplayApplianceAccount> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info/account_info",
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
    /**
     * Return custom fields information for Display Appliance.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info/custom_fields` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_get_custom_field(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<crate::types::ApplianceInfo> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info/custom_fields",
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
    /**
     * Deletes custom fields information for Display Appliance.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info/custom_fields/delete` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_delete_custom_fields(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info/custom_fields/delete",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Gets date signed information for Display Appliance.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info/date_signed` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_get_date_signed(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<crate::types::ApplianceInfo> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info/date_signed",
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
    /**
     * Updates document information for Display Applianc.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info/document/{documentId}` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_put_document(
        &self,
        account_id: &str,
        document_id: &str,
        envelope_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info/document/{}",
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
    /**
     * Deletes document information for Display Appliance.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info/document/{documentId}` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_delete_document(
        &self,
        account_id: &str,
        document_id: &str,
        envelope_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info/document/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                crate::progenitor_support::encode_path(document_id),
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
     * Return document pages for Display Appliance.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info/document_page_list` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_get_document_page(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<crate::types::ApplianceInfo> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info/document_page_list",
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
    /**
     * Returns images for Display Appliance.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info/image` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_get_image(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<crate::types::ApplianceInfo> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info/image",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Returns locale policy information for Display Appliance.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info/localepolicy/{userId}` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     */
    pub async fn envelope_get_locale_policy(
        &self,
        account_id: &str,
        envelope_id: &str,
        user_id: &str,
    ) -> ClientResult<crate::types::ApplianceInfo> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info/localepolicy/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                crate::progenitor_support::encode_path(user_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Updates page information for Display Appliance.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info/page_info` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_put_page(&self, account_id: &str, envelope_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info/page_info",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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
    /**
     * Creates page information for Display Appliance.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info/page_info` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_post_page(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info/page_info",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Deletes page information for Display Appliance.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info/page_info/delete` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_delete_page(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info/page_info/delete",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Sets latest PDF for Display Appliance.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info/pdf` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_put_pdf(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<crate::types::ApplianceInfo> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info/pdf",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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
    /**
     * Return PDF for Display Appliance.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info/pdf/{pdfId}` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `pdf_id: &str` -- **Deprecated**.
     *   
     *   The `pdfId` property in the consumer_disclosure PUT request is deprecated. For security reasons going forward, any value provided in the request packet must be ignored.
     */
    pub async fn envelope_get_pdf(
        &self,
        account_id: &str,
        envelope_id: &str,
        pdf_id: &str,
    ) -> ClientResult<crate::types::ApplianceInfo> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info/pdf/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                crate::progenitor_support::encode_path(pdf_id),
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
     * Returns PDF blobs for Display Appliance.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info/pdf_blobs` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_get_pdf_blob(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<crate::types::ApplianceInfo> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info/pdf_blobs",
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
    /**
     * Updates PDF blobs for Display Appliance.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info/pdf_blobs` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_put_pdf_blob(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info/pdf_blobs",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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
    /**
     * Adds PDF blobs for Display Appliance.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info/pdf_blobs` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_post_pdf_blob(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<crate::types::ApplianceInfo> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info/pdf_blobs",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Updates RecipientDeniedDocumentCopy for Display Appliance.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info/recipient_denied_copy` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_put_recipient_denied_document_copy(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info/recipient_denied_copy",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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
    /**
     * Deletes RecipientDeniedDocumentCopy for Display Appliance.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info/recipient_denied_copy` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_delete_recipient_denied_document_copy(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info/recipient_denied_copy",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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
     * Returns signer attachment information for Display Appliance.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info/signer_attachment_info` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_get_signer_attachment(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<crate::types::ApplianceInfo> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info/signer_attachment_info",
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
    /**
     * Deletes signer attachment information for Display Appliance.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/display_appliance_info/signer_attachment_info` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_delete_signer_attachment(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/display_appliance_info/signer_attachment_info",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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
     * Uploads Kazmon error for Display Appliance.
     *
     * This function performs a `POST` to the `/v2.1/display_appliance_info/error` endpoint.
     *
     *
     */
    pub async fn envelope_post_error(&self) -> ClientResult<()> {
        let url = self.client.url("/v2.1/display_appliance_info/error", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Returns signing URL for Display Appliance.
     *
     * This function performs a `POST` to the `/v2.1/display_appliance_info/redeem` endpoint.
     *
     *
     */
    pub async fn envelope_post_redeem(&self) -> ClientResult<crate::types::ApplianceInfo> {
        let url = self.client.url("/v2.1/display_appliance_info/redeem", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}
