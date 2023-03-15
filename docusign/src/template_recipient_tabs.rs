use crate::Client;
use crate::ClientResult;

pub struct TemplateRecipientTabs {
    pub client: Client,
}

impl TemplateRecipientTabs {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TemplateRecipientTabs { client }
    }

    /**
     * Gets the tabs information for a signer or sign-in-person recipient in a template.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/templates/{templateId}/recipients/{recipientId}/tabs` endpoint.
     *
     * Gets the tabs information for a signer or sign-in-person recipient in a template.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `recipient_id: &str` -- A local reference that senders use to map recipients to other objects, such as specific document tabs. Within an envelope, each `recipientId` must be unique, but there is no uniqueness requirement across envelopes. For example, many envelopes assign the first recipient a `recipientId` of `1`.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include_anchor_tab_locations: &str` -- When set to **true**, all tabs with anchor tab properties are included in the response. The default value is **false**.
     * * `include_metadata: &str` -- When set to **true**, the response includes metadata indicating which properties are editable.
     */
    pub async fn recipients_get(
        &self,
        account_id: &str,
        recipient_id: &str,
        template_id: &str,
        include_anchor_tab_locations: &str,
        include_metadata: &str,
    ) -> ClientResult<crate::types::Tabs> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include_anchor_tab_locations.is_empty() {
            query_args.push((
                "include_anchor_tab_locations".to_string(),
                include_anchor_tab_locations.to_string(),
            ));
        }
        if !include_metadata.is_empty() {
            query_args.push(("include_metadata".to_string(), include_metadata.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/recipients/{}/tabs?{}",
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
     * Updates the tabs for a recipient.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/templates/{templateId}/recipients/{recipientId}/tabs` endpoint.
     *
     * Updates one or more tabs for a recipient in a template.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `recipient_id: &str` -- A local reference that senders use to map recipients to other objects, such as specific document tabs. Within an envelope, each `recipientId` must be unique, but there is no uniqueness requirement across envelopes. For example, many envelopes assign the first recipient a `recipientId` of `1`.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn recipients_put(
        &self,
        account_id: &str,
        recipient_id: &str,
        template_id: &str,
        body: &crate::types::TemplateTabs,
    ) -> ClientResult<crate::types::Tabs> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/recipients/{}/tabs",
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
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Adds tabs for a recipient.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/templates/{templateId}/recipients/{recipientId}/tabs` endpoint.
     *
     * Adds one or more tabs for a recipient.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `recipient_id: &str` -- A local reference that senders use to map recipients to other objects, such as specific document tabs. Within an envelope, each `recipientId` must be unique, but there is no uniqueness requirement across envelopes. For example, many envelopes assign the first recipient a `recipientId` of `1`.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn recipients_post(
        &self,
        account_id: &str,
        recipient_id: &str,
        template_id: &str,
        body: &crate::types::TemplateTabs,
    ) -> ClientResult<crate::types::Tabs> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/recipients/{}/tabs",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
                crate::progenitor_support::encode_path(recipient_id),
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
     * Deletes the tabs associated with a recipient in a template.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/templates/{templateId}/recipients/{recipientId}/tabs` endpoint.
     *
     * Deletes one or more tabs associated with a recipient in a template.
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
        body: &crate::types::TemplateTabs,
    ) -> ClientResult<crate::types::Tabs> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/recipients/{}/tabs",
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
}
