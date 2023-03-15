use crate::Client;
use crate::ClientResult;

pub struct TemplateCustomFields {
    pub client: Client,
}

impl TemplateCustomFields {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TemplateCustomFields { client }
    }

    /**
     * Gets the custom document fields from a template.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/templates/{templateId}/custom_fields` endpoint.
     *
     * Retrieves the custom document field information from an existing template.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn custom_fields_get_template(
        &self,
        account_id: &str,
        template_id: &str,
    ) -> ClientResult<crate::types::CustomFields> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/custom_fields",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
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
     * Updates envelope custom fields in a template.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/templates/{templateId}/custom_fields` endpoint.
     *
     * Updates the custom fields in a template.
     *
     * Each custom field used in a template must have a unique name.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn custom_fields_put_template(
        &self,
        account_id: &str,
        template_id: &str,
        body: &crate::types::TemplateCustomFieldsData,
    ) -> ClientResult<crate::types::CustomFields> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/custom_fields",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
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
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/templates/{templateId}/custom_fields` endpoint.
     *
     * Creates custom document fields in an existing template document.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn custom_fields_post_template(
        &self,
        account_id: &str,
        template_id: &str,
        body: &crate::types::TemplateCustomFieldsData,
    ) -> ClientResult<crate::types::CustomFields> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/custom_fields",
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
    /**
     * Deletes envelope custom fields in a template.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/templates/{templateId}/custom_fields` endpoint.
     *
     * Deletes envelope custom fields in a template.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn custom_fields_delete_template(
        &self,
        account_id: &str,
        template_id: &str,
        body: &crate::types::TemplateCustomFieldsData,
    ) -> ClientResult<crate::types::CustomFields> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/custom_fields",
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
}
