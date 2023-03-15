use crate::Client;
use crate::ClientResult;

pub struct Schemas {
    pub client: Client,
}

impl Schemas {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Schemas { client }
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customerId}/schemas` endpoint.
     *
     * Retrieves all schemas for a customer.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- Immutable ID of the Google Workspace account.
     */
    pub async fn list(&self, customer_id: &str) -> ClientResult<crate::types::Schemas> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/schemas",
                crate::progenitor_support::encode_path(customer_id),
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
     * This function performs a `POST` to the `/admin/directory/v1/customer/{customerId}/schemas` endpoint.
     *
     * Creates a schema.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- Immutable ID of the Google Workspace account.
     */
    pub async fn insert(
        &self,
        customer_id: &str,
        body: &crate::types::Schema,
    ) -> ClientResult<crate::types::Schema> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/schemas",
                crate::progenitor_support::encode_path(customer_id),
            ),
            None,
        );
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
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customerId}/schemas/{schemaKey}` endpoint.
     *
     * Retrieves a schema.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- Immutable ID of the Google Workspace account.
     * * `schema_key: &str` -- Name or immutable ID of the schema.
     */
    pub async fn get(
        &self,
        customer_id: &str,
        schema_key: &str,
    ) -> ClientResult<crate::types::Schema> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/schemas/{}",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(schema_key),
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
     * This function performs a `PUT` to the `/admin/directory/v1/customer/{customerId}/schemas/{schemaKey}` endpoint.
     *
     * Updates a schema.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- Immutable ID of the Google Workspace account.
     * * `schema_key: &str` -- Name or immutable ID of the schema.
     */
    pub async fn update(
        &self,
        customer_id: &str,
        schema_key: &str,
        body: &crate::types::Schema,
    ) -> ClientResult<crate::types::Schema> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/schemas/{}",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(schema_key),
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
     * This function performs a `DELETE` to the `/admin/directory/v1/customer/{customerId}/schemas/{schemaKey}` endpoint.
     *
     * Deletes a schema.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- Immutable ID of the Google Workspace account.
     * * `schema_key: &str` -- Name or immutable ID of the schema.
     */
    pub async fn delete(&self, customer_id: &str, schema_key: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/schemas/{}",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(schema_key),
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
     * This function performs a `PATCH` to the `/admin/directory/v1/customer/{customerId}/schemas/{schemaKey}` endpoint.
     *
     * Patches a schema.
     *
     * **Parameters:**
     *
     * * `customer_id: &str` -- Immutable ID of the Google Workspace account.
     * * `schema_key: &str` -- Name or immutable ID of the schema.
     */
    pub async fn patch(
        &self,
        customer_id: &str,
        schema_key: &str,
        body: &crate::types::Schema,
    ) -> ClientResult<crate::types::Schema> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/customer/{}/schemas/{}",
                crate::progenitor_support::encode_path(customer_id),
                crate::progenitor_support::encode_path(schema_key),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
}
