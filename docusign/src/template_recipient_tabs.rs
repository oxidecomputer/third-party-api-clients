use anyhow::Result;

use crate::Client;

pub struct TemplateRecipientTabs {
    client: Client,
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
    ) -> Result<crate::types::Tabs> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !include_anchor_tab_locations.is_empty() {
            query_args.push(format!(
                "include_anchor_tab_locations={}",
                include_anchor_tab_locations
            ));
        }
        if !include_metadata.is_empty() {
            query_args.push(format!("include_metadata={}", include_metadata));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/v2.1/accounts/{}/templates/{}/recipients/{}/tabs?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&template_id.to_string()),
            crate::progenitor_support::encode_path(&recipient_id.to_string()),
            query
        );

        self.client.get(&url, None).await
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
    ) -> Result<crate::types::Tabs> {
        let url = format!(
            "/v2.1/accounts/{}/templates/{}/recipients/{}/tabs",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&template_id.to_string()),
            crate::progenitor_support::encode_path(&recipient_id.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
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
    ) -> Result<crate::types::Tabs> {
        let url = format!(
            "/v2.1/accounts/{}/templates/{}/recipients/{}/tabs",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&template_id.to_string()),
            crate::progenitor_support::encode_path(&recipient_id.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
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
    ) -> Result<crate::types::Tabs> {
        let url = format!(
            "/v2.1/accounts/{}/templates/{}/recipients/{}/tabs",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&template_id.to_string()),
            crate::progenitor_support::encode_path(&recipient_id.to_string()),
        );

        self.client
            .delete(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
