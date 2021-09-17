use anyhow::Result;

use crate::Client;

pub struct LinkedObject {
    pub client: Client,
}

impl LinkedObject {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        LinkedObject { client }
    }

    /**
     * This function performs a `GET` to the `/api/v1/meta/schemas/user/linkedObjects` endpoint.
     *
     * Success
     */
    pub async fn list_definitions(&self) -> Result<Vec<crate::types::LinkedObject>> {
        let url = "/api/v1/meta/schemas/user/linkedObjects".to_string();
        self.client.get(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/api/v1/meta/schemas/user/linkedObjects` endpoint.
     *
     * As opposed to `list_definitions`, this function returns all the pages of the request at once.
     *
     * Success
     */
    pub async fn list_all_definitions(&self) -> Result<Vec<crate::types::LinkedObject>> {
        let url = "/api/v1/meta/schemas/user/linkedObjects".to_string();
        self.client.get_all_pages(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/api/v1/meta/schemas/user/linkedObjects` endpoint.
     *
     * Success
     */
    pub async fn add_definition(
        &self,
        body: &crate::types::LinkedObject,
    ) -> Result<crate::types::LinkedObject> {
        let url = "/api/v1/meta/schemas/user/linkedObjects".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * This function performs a `GET` to the `/api/v1/meta/schemas/user/linkedObjects/{linkedObjectName}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `linked_object_name: &str`
     */
    pub async fn get_definition(
        &self,
        linked_object_name: &str,
    ) -> Result<crate::types::LinkedObject> {
        let url = format!(
            "/api/v1/meta/schemas/user/linkedObjects/{}",
            crate::progenitor_support::encode_path(&linked_object_name.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `DELETE` to the `/api/v1/meta/schemas/user/linkedObjects/{linkedObjectName}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `linked_object_name: &str`
     */
    pub async fn delete_definition(&self, linked_object_name: &str) -> Result<()> {
        let url = format!(
            "/api/v1/meta/schemas/user/linkedObjects/{}",
            crate::progenitor_support::encode_path(&linked_object_name.to_string()),
        );

        self.client.delete(&url, None).await
    }
}
