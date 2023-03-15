use crate::Client;
use crate::ClientResult;

pub struct UserSchemas {
    pub client: Client,
}

impl UserSchemas {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UserSchemas { client }
    }

    /**
     * Fetches the Schema for an App User.
     *
     * This function performs a `GET` to the `/api/v1/meta/schemas/apps/{appInstanceId}/default` endpoint.
     *
     * Fetches the Schema for an App User
     *
     * **Parameters:**
     *
     * * `app_instance_id: &str`
     */
    pub async fn get_application(
        &self,
        app_instance_id: &str,
    ) -> ClientResult<crate::types::UserSchema> {
        let url = self.client.url(
            &format!(
                "/api/v1/meta/schemas/apps/{}/default",
                crate::progenitor_support::encode_path(app_instance_id),
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
     * Partial updates on the User Profile properties of the Application User Schema.
     *
     * This function performs a `POST` to the `/api/v1/meta/schemas/apps/{appInstanceId}/default` endpoint.
     *
     * Partial updates on the User Profile properties of the Application User Schema.
     *
     * **Parameters:**
     *
     * * `app_instance_id: &str`
     */
    pub async fn update_application_user_profile(
        &self,
        app_instance_id: &str,
        body: &crate::types::UserSchema,
    ) -> ClientResult<crate::types::UserSchema> {
        let url = self.client.url(
            &format!(
                "/api/v1/meta/schemas/apps/{}/default",
                crate::progenitor_support::encode_path(app_instance_id),
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
     * Fetches the schema for a Schema Id.
     *
     * This function performs a `GET` to the `/api/v1/meta/schemas/user/{schemaId}` endpoint.
     *
     * Fetches the schema for a Schema Id.
     *
     * **Parameters:**
     *
     * * `schema_id: &str`
     */
    pub async fn get(&self, schema_id: &str) -> ClientResult<crate::types::UserSchema> {
        let url = self.client.url(
            &format!(
                "/api/v1/meta/schemas/user/{}",
                crate::progenitor_support::encode_path(schema_id),
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
     * This function performs a `POST` to the `/api/v1/meta/schemas/user/{schemaId}` endpoint.
     *
     * Partial updates on the User Profile properties of the user schema.
     *
     * **Parameters:**
     *
     * * `schema_id: &str`
     */
    pub async fn update_user_profile(
        &self,
        schema_id: &str,
        body: &crate::types::UserSchema,
    ) -> ClientResult<crate::types::UserSchema> {
        let url = self.client.url(
            &format!(
                "/api/v1/meta/schemas/user/{}",
                crate::progenitor_support::encode_path(schema_id),
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
}
