use crate::Client;
use crate::ClientResult;

pub struct TemplateRecipients {
    pub client: Client,
}

impl TemplateRecipients {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TemplateRecipients { client }
    }

    /**
     * Gets recipient information from a template.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/templates/{templateId}/recipients` endpoint.
     *
     * Retrieves the information for all recipients in the specified template.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include_anchor_tab_locations: &str` --  When set to **true** and `include_tabs` is set to **true**, all tabs with anchor tab properties are included in the response. .
     * * `include_extended: &str` --  When set to **true**, the extended properties are included in the response. .
     * * `include_tabs: &str` -- When set to **true**, the tab information associated with the recipient is included in the response.
     */
    pub async fn recipients_get_template(
        &self,
        account_id: &str,
        template_id: &str,
        include_anchor_tab_locations: &str,
        include_extended: &str,
        include_tabs: &str,
    ) -> ClientResult<crate::types::Recipients> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include_anchor_tab_locations.is_empty() {
            query_args.push((
                "include_anchor_tab_locations".to_string(),
                include_anchor_tab_locations.to_string(),
            ));
        }
        if !include_extended.is_empty() {
            query_args.push(("include_extended".to_string(), include_extended.to_string()));
        }
        if !include_tabs.is_empty() {
            query_args.push(("include_tabs".to_string(), include_tabs.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/recipients?{}",
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
     * Updates recipients in a template.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/templates/{templateId}/recipients` endpoint.
     *
     * Updates recipients in a template.
     *
     * You can edit the following properties: `email`, `userName`, `routingOrder`, `faxNumber`, `deliveryMethod`, `accessCode`, and `requireIdLookup`.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `resend_envelope: &str` -- When set to **true**, resends the envelope to the recipients that you specify in the request body. You use this parameter to resend the envelope to a recipient who deleted the original email notification.
     *   
     *   **Note**: Correcting an envelope is a different process. DocuSign always resends an envelope when you correct it, regardless of the value that you enter here.
     */
    pub async fn recipients_put_template(
        &self,
        account_id: &str,
        template_id: &str,
        resend_envelope: &str,
        body: &crate::types::TemplateRecipientsData,
    ) -> ClientResult<crate::types::RecipientsUpdateSummary> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !resend_envelope.is_empty() {
            query_args.push(("resend_envelope".to_string(), resend_envelope.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/recipients?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
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
    /**
     * Adds tabs for a recipient.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/templates/{templateId}/recipients` endpoint.
     *
     * Adds one or more recipients to a template.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `resend_envelope: &str` -- When set to **true**, resends the envelope to the recipients that you specify in the request body. You use this parameter to resend the envelope to a recipient who deleted the original email notification.
     *   
     *   **Note**: Correcting an envelope is a different process. DocuSign always resends an envelope when you correct it, regardless of the value that you enter here.
     */
    pub async fn recipients_post_template(
        &self,
        account_id: &str,
        template_id: &str,
        resend_envelope: &str,
        body: &crate::types::TemplateRecipientsData,
    ) -> ClientResult<crate::types::Recipients> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !resend_envelope.is_empty() {
            query_args.push(("resend_envelope".to_string(), resend_envelope.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/recipients?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
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
     * Deletes recipients from a template.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/templates/{templateId}/recipients` endpoint.
     *
     * Deletes one or more recipients from a template. Recipients to be deleted are listed in the request, with the `recipientId` being used as the key for deleting recipients.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn recipients_delete_template(
        &self,
        account_id: &str,
        template_id: &str,
        body: &crate::types::TemplateRecipientsData,
    ) -> ClientResult<crate::types::Recipients> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/recipients",
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
     * Deletes the specified recipient file from a template.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/templates/{templateId}/recipients/{recipientId}` endpoint.
     *
     * Deletes the specified recipient file from the specified template.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `recipient_id: &str` -- A local reference that senders use to map recipients to other objects, such as specific document tabs. Within an envelope, each `recipientId` must be unique, but there is no uniqueness requirement across envelopes. For example, many envelopes assign the first recipient a `recipientId` of `1`.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn recipients_delete(
        &self,
        account_id: &str,
        recipient_id: &str,
        template_id: &str,
        body: &crate::types::TemplateRecipientsData,
    ) -> ClientResult<crate::types::Recipients> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/recipients/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
                crate::progenitor_support::encode_path(recipient_id),
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
     * Creates a template recipient preview.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/templates/{templateId}/views/recipient_preview` endpoint.
     *
     * This method returns a URL for a template recipient preview  in the DocuSign UI that you can embed in your application. You use this method to enable the sender to preview the recipients' experience.
     *
     * For more information, see [Preview and Send](https://support.docusign.com/en/guides/ndse-user-guide-send-your-documents).
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn views_post_preview(
        &self,
        account_id: &str,
        template_id: &str,
        body: &crate::types::RecipientPreviewRequest,
    ) -> ClientResult<crate::types::ViewUrl> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/views/recipient_preview",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
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
