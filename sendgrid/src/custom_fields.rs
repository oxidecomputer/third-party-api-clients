use anyhow::Result;

use crate::Client;

pub struct CustomFields {
    pub client: Client,
}

impl CustomFields {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CustomFields { client }
    }

    /**
    * Get All Field Definitions.
    *
    * This function performs a `GET` to the `/marketing/field_definitions` endpoint.
    *
    * **This endpoint retrieves all defined Custom Fields and Reserved Fields.**
    */
    pub async fn get_mc_field_definitions(
        &self,
    ) -> Result<crate::types::GetMcFieldDefinitionsResponse> {
        let url = "/marketing/field_definitions".to_string();
        self.client.get(&url, None).await
    }

    /**
    * Create Custom Field Definition.
    *
    * This function performs a `POST` to the `/marketing/field_definitions` endpoint.
    *
    * **This endpoint creates a new custom field definition.**
    *
    * Custom field definitions are created with the given `name` and `field_type`. Although field names are stored in a case-sensitive manner, all field names must be case-insensitively unique. This means you may create a field named `CamelCase` or `camelcase`, but not both. Additionally, a Custom Field name cannot collide with any Reserved Field names. You should save the returned `id` value in order to update or delete the field at a later date. You can have up to 120 custom fields.
    *
    * The custom field name should be created using only alphanumeric characters (A-Z and 0-9) and underscores (\_). Custom fields can only begin with letters  A-Z or underscores (_). The field type can be date, text, or number fields. The field type is important for creating segments from your contact database.
    *
    * **Note: Creating a custom field that begins with a number will cause issues with sending in Marketing Campaigns.**
    */
    pub async fn post_mc_field_definition(
        &self,
        body: &crate::types::PostMcFieldDefinitionsRequest,
    ) -> Result<crate::types::PostMcFieldDefinitionsResponseAllOf> {
        let url = "/marketing/field_definitions".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Delete Custom Field Definition.
    *
    * This function performs a `DELETE` to the `/marketing/field_definitions/{custom_field_id}` endpoint.
    *
    * **This endpoint deletes a defined Custom Field.**
    *
    * You cand delete only Custom Fields; Reserved Fields cannot be deleted.
    */
    pub async fn delete_mc_field_definitions_custom(&self, custom_field_id: &str) -> Result<()> {
        let url = format!(
            "/marketing/field_definitions/{}",
            crate::progenitor_support::encode_path(&custom_field_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update Custom Field Definition.
    *
    * This function performs a `PATCH` to the `/marketing/field_definitions/{custom_field_id}` endpoint.
    *
    * **This endopoint allows you to update a defined Custom Field.**
    *
    * Only your Custom fields can be modified; Reserved Fields cannot be updated.
    */
    pub async fn patch_mc_field_definitions_custom(
        &self,
        custom_field_id: &str,
        body: &crate::types::IpPool,
    ) -> Result<crate::types::PostMcFieldDefinitionsResponseAllOf> {
        let url = format!(
            "/marketing/field_definitions/{}",
            crate::progenitor_support::encode_path(&custom_field_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
