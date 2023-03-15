use crate::Client;
use crate::ClientResult;

pub struct ContactsApiCustomFields {
    pub client: Client,
}

impl ContactsApiCustomFields {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ContactsApiCustomFields { client }
    }

    /**
     * Retrieve all custom fields.
     *
     * This function performs a `GET` to the `/contactdb/custom_fields` endpoint.
     *
     * **This endpoint allows you to retrieve all custom fields.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_contactdb_custom_fields(
        &self,
    ) -> ClientResult<crate::types::ListAllCustomFieldsResponse> {
        let url = self.client.url("/contactdb/custom_fields", None);
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
     * Create a Custom Field.
     *
     * This function performs a `POST` to the `/contactdb/custom_fields` endpoint.
     *
     * **This endpoint allows you to create a custom field.**
     *
     * **You can create up to 120 custom fields.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_contactdb_custom_field(
        &self,
        body: &crate::types::PostContactdbCustomFieldsRequest,
    ) -> ClientResult<crate::types::ContactdbCustomFieldWithAllOf> {
        let url = self.client.url("/contactdb/custom_fields", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieve a Custom Field.
     *
     * This function performs a `GET` to the `/contactdb/custom_fields/{custom_field_id}` endpoint.
     *
     * **This endpoint allows you to retrieve a custom field by ID.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_contactdb_custom_fields_field(
        &self,
        custom_field_id: i64,
    ) -> ClientResult<crate::types::ContactdbCustomFieldWithAllOf> {
        let url = self.client.url(
            &format!(
                "/contactdb/custom_fields/{}",
                crate::progenitor_support::encode_path(&custom_field_id.to_string()),
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
     * Delete a Custom Field.
     *
     * This function performs a `DELETE` to the `/contactdb/custom_fields/{custom_field_id}` endpoint.
     *
     * **This endpoint allows you to delete a custom field by ID.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_contactdb_custom_fields_field(
        &self,
        custom_field_id: i64,
    ) -> ClientResult<crate::types::GlobalErrorResponseSchema> {
        let url = self.client.url(
            &format!(
                "/contactdb/custom_fields/{}",
                crate::progenitor_support::encode_path(&custom_field_id.to_string()),
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
     * Retrieve reserved fields.
     *
     * This function performs a `GET` to the `/contactdb/reserved_fields` endpoint.
     *
     * **This endpoint allows you to list all fields that are reserved and can't be used for custom field names.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_contactdb_reserved_fields(
        &self,
    ) -> ClientResult<crate::types::GetContactdbReservedFieldsResponse> {
        let url = self.client.url("/contactdb/reserved_fields", None);
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
}
