use crate::Client;
use crate::ClientResult;

pub struct TemplateBulkRecipients {
    pub client: Client,
}

impl TemplateBulkRecipients {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TemplateBulkRecipients { client }
    }

    /**
     * Gets the bulk recipient file from a template.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/templates/{templateId}/recipients/{recipientId}/bulk_recipients` endpoint.
     *
     * Retrieves the bulk recipient file information from a template that has a bulk recipient.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `recipient_id: &str` -- A local reference that senders use to map recipients to other objects, such as specific document tabs. Within an envelope, each `recipientId` must be unique, but there is no uniqueness requirement across envelopes. For example, many envelopes assign the first recipient a `recipientId` of `1`.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include_tabs: &str` -- When set to **true**, the tab information associated with the recipient is included in the response. If you do not specify this parameter, the effect is the default behavior (**false**).
     * * `start_position: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn recipients_get_template_bulk(
        &self,
        account_id: &str,
        recipient_id: &str,
        template_id: &str,
        include_tabs: &str,
        start_position: &str,
    ) -> ClientResult<crate::types::BulkRecipientsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include_tabs.is_empty() {
            query_args.push(("include_tabs".to_string(), include_tabs.to_string()));
        }
        if !start_position.is_empty() {
            query_args.push(("start_position".to_string(), start_position.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/recipients/{}/bulk_recipients?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
                crate::progenitor_support::encode_path(recipient_id),
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
     * Adds or replaces the bulk recipients list in a template.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/templates/{templateId}/recipients/{recipientId}/bulk_recipients` endpoint.
     *
     * Updates the bulk recipients in a template using a file upload. The Content-Type supported for uploading a bulk recipient file is CSV (text/csv).
     *
     * The REST API does not support modifying individual rows or values in the bulk recipients file. It only allows the entire file to be added or replaced with a new file.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `recipient_id: &str` -- A local reference that senders use to map recipients to other objects, such as specific document tabs. Within an envelope, each `recipientId` must be unique, but there is no uniqueness requirement across envelopes. For example, many envelopes assign the first recipient a `recipientId` of `1`.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn recipients_put_template_bulk(
        &self,
        account_id: &str,
        recipient_id: &str,
        template_id: &str,
        body: &crate::types::BulkRecipientsRequest,
    ) -> ClientResult<crate::types::BulkRecipientsSummaryResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/recipients/{}/bulk_recipients",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
                crate::progenitor_support::encode_path(recipient_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Deletes the bulk recipient list on a template.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/templates/{templateId}/recipients/{recipientId}/bulk_recipients` endpoint.
     *
     * Deletes the bulk recipient list on a template.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `recipient_id: &str` -- A local reference that senders use to map recipients to other objects, such as specific document tabs. Within an envelope, each `recipientId` must be unique, but there is no uniqueness requirement across envelopes. For example, many envelopes assign the first recipient a `recipientId` of `1`.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn recipients_delete_template_bulk_file(
        &self,
        account_id: &str,
        recipient_id: &str,
        template_id: &str,
    ) -> ClientResult<crate::types::BulkRecipientsUpdateResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/recipients/{}/bulk_recipients",
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
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}
