use anyhow::Result;

use crate::Client;

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
    ) -> Result<crate::types::CustomFields> {
        let url = format!(
            "/v2.1/accounts/{}/templates/{}/custom_fields",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&template_id.to_string()),
        );

        self.client.get(&url, None).await
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
        body: &crate::types::CustomFieldsEnvelope,
    ) -> Result<crate::types::CustomFields> {
        let url = format!(
            "/v2.1/accounts/{}/templates/{}/custom_fields",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&template_id.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
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
        body: &crate::types::CustomFieldsEnvelope,
    ) -> Result<crate::types::CustomFields> {
        let url = format!(
            "/v2.1/accounts/{}/templates/{}/custom_fields",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&template_id.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
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
        body: &crate::types::CustomFieldsEnvelope,
    ) -> Result<crate::types::CustomFields> {
        let url = format!(
            "/v2.1/accounts/{}/templates/{}/custom_fields",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&template_id.to_string()),
        );

        self.client
            .delete(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
