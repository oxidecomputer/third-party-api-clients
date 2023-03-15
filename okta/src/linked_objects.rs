use crate::Client;
use crate::ClientResult;

pub struct LinkedObjects {
    pub client: Client,
}

impl LinkedObjects {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        LinkedObjects { client }
    }

    /**
     * This function performs a `GET` to the `/api/v1/meta/schemas/user/linkedObjects` endpoint.
     *
     * Success
     */
    pub async fn list_definitions(&self) -> ClientResult<Vec<crate::types::LinkedObject>> {
        let url = self
            .client
            .url("/api/v1/meta/schemas/user/linkedObjects", None);
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
     * This function performs a `GET` to the `/api/v1/meta/schemas/user/linkedObjects` endpoint.
     *
     * As opposed to `list_definitions`, this function returns all the pages of the request at once.
     *
     * Success
     */
    pub async fn list_all_definitions(&self) -> ClientResult<Vec<crate::types::LinkedObject>> {
        let url = self
            .client
            .url("/api/v1/meta/schemas/user/linkedObjects", None);
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/api/v1/meta/schemas/user/linkedObjects` endpoint.
     *
     * Success
     */
    pub async fn add_definition(
        &self,
        body: &crate::types::LinkedObject,
    ) -> ClientResult<crate::types::LinkedObject> {
        let url = self
            .client
            .url("/api/v1/meta/schemas/user/linkedObjects", None);
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
    ) -> ClientResult<crate::types::LinkedObject> {
        let url = self.client.url(
            &format!(
                "/api/v1/meta/schemas/user/linkedObjects/{}",
                crate::progenitor_support::encode_path(linked_object_name),
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
     * This function performs a `DELETE` to the `/api/v1/meta/schemas/user/linkedObjects/{linkedObjectName}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `linked_object_name: &str`
     */
    pub async fn delete_definition(&self, linked_object_name: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/meta/schemas/user/linkedObjects/{}",
                crate::progenitor_support::encode_path(linked_object_name),
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
