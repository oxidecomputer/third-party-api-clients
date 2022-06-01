use anyhow::Result;

use crate::Client;

pub struct TemplateDocumentVisibility {
    pub client: Client,
}

impl TemplateDocumentVisibility {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TemplateDocumentVisibility { client }
    }

    /**
    * Updates document visibility for template recipients.
    *
    * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/templates/{templateId}/recipients/document_visibility` endpoint.
    *
    * This method updates document visibility for one or more template recipients based on the `recipientId` and `visible` values that you include in the request body.
    *
    *
    * **Note**: A document cannot be hidden from a recipient if the recipient has tabs assigned to them on the document. Carbon Copy, Certified Delivery (Needs to Sign), Editor, and Agent recipients can always see all documents.
    *
    * **Parameters:**
    *
    * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    */
    pub async fn recipients_put(
        &self,
        account_id: &str,
        template_id: &str,
        body: &crate::types::TemplateDocumentVisibilityList,
    ) -> Result<crate::types::TemplateDocumentVisibilityList> {
        let url = format!(
            "/v2.1/accounts/{}/templates/{}/recipients/document_visibility",
            crate::progenitor_support::encode_path(account_id),
            crate::progenitor_support::encode_path(template_id),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Returns document visibility for a template recipient.
    *
    * This function performs a `GET` to the `/v2.1/accounts/{accountId}/templates/{templateId}/recipients/{recipientId}/document_visibility` endpoint.
    *
    * This method returns information about document visibility for a template recipient.
    *
    * **Parameters:**
    *
    * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    * * `recipient_id: &str` -- A local reference that senders use to map recipients to other objects, such as specific document tabs. Within an envelope, each `recipientId` must be unique, but there is no uniqueness requirement across envelopes. For example, many envelopes assign the first recipient a `recipientId` of `1`.
    * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    */
    pub async fn recipients_get_template_recipient_document_visibility(
        &self,
        account_id: &str,
        recipient_id: &str,
        template_id: &str,
    ) -> Result<crate::types::DocumentVisibilityList> {
        let url = format!(
            "/v2.1/accounts/{}/templates/{}/recipients/{}/document_visibility",
            crate::progenitor_support::encode_path(account_id),
            crate::progenitor_support::encode_path(template_id),
            crate::progenitor_support::encode_path(recipient_id),
        );

        self.client.get(&url, None).await
    }

    /**
    * Updates document visibility for a template recipient.
    *
    * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/templates/{templateId}/recipients/{recipientId}/document_visibility` endpoint.
    *
    * This method updates the document visibility for a template recipient.
    *
    * **Note**: A document cannot be hidden from a recipient if the recipient has tabs assigned to them on the document. Carbon Copy, Certified Delivery (Needs to Sign), Editor, and Agent recipients can always see all documents.
    *
    * **Parameters:**
    *
    * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    * * `recipient_id: &str` -- A local reference that senders use to map recipients to other objects, such as specific document tabs. Within an envelope, each `recipientId` must be unique, but there is no uniqueness requirement across envelopes. For example, many envelopes assign the first recipient a `recipientId` of `1`.
    * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    */
    pub async fn recipients_put_template_recipient_document_visibility(
        &self,
        account_id: &str,
        recipient_id: &str,
        template_id: &str,
        body: &crate::types::TemplateDocumentVisibilityList,
    ) -> Result<crate::types::TemplateDocumentVisibilityList> {
        let url = format!(
            "/v2.1/accounts/{}/templates/{}/recipients/{}/document_visibility",
            crate::progenitor_support::encode_path(account_id),
            crate::progenitor_support::encode_path(template_id),
            crate::progenitor_support::encode_path(recipient_id),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
