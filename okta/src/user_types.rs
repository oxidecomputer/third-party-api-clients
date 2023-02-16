use anyhow::Result;

use crate::Client;

pub struct UserTypes {
    pub client: Client,
}

impl UserTypes {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UserTypes { client }
    }

    /**
     * This function performs a `GET` to the `/api/v1/meta/types/user` endpoint.
     *
     * Fetches all User Types in your org
     */
    pub async fn list(&self) -> Result<Vec<crate::types::UserType>> {
        let url = "/api/v1/meta/types/user".to_string();
        let url = self.client.url(&url, None);
        self.client.get(&url, None, None).await
    }
    /**
     * This function performs a `GET` to the `/api/v1/meta/types/user` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Fetches all User Types in your org
     */
    pub async fn list_all(&self) -> Result<Vec<crate::types::UserType>> {
        let url = "/api/v1/meta/types/user".to_string();
        self.client.get_all_pages(&url, None).await
    }
    /**
     * This function performs a `POST` to the `/api/v1/meta/types/user` endpoint.
     *
     * Creates a new User Type. A default User Type is automatically created along with your org, and you may add another 9 User Types for a maximum of 10.
     */
    pub async fn create(&self, body: &crate::types::UserType) -> Result<crate::types::UserType> {
        let url = "/api/v1/meta/types/user".to_string();
        let url = self.client.url(&url, None);
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                None,
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/api/v1/meta/types/user/{typeId}` endpoint.
     *
     * Fetches a User Type by ID. The special identifier `default` may be used to fetch the default User Type.
     *
     * **Parameters:**
     *
     * * `type_id: &str`
     */
    pub async fn get(&self, type_id: &str) -> Result<crate::types::UserType> {
        let url = format!(
            "/api/v1/meta/types/user/{}",
            crate::progenitor_support::encode_path(type_id),
        );
        let url = self.client.url(&url, None);
        self.client.get(&url, None, None).await
    }
    /**
     * This function performs a `PUT` to the `/api/v1/meta/types/user/{typeId}` endpoint.
     *
     * Replace an existing User Type
     *
     * **Parameters:**
     *
     * * `type_id: &str`
     */
    pub async fn replace(
        &self,
        type_id: &str,
        body: &crate::types::UserType,
    ) -> Result<crate::types::UserType> {
        let url = format!(
            "/api/v1/meta/types/user/{}",
            crate::progenitor_support::encode_path(type_id),
        );
        let url = self.client.url(&url, None);
        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                None,
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/api/v1/meta/types/user/{typeId}` endpoint.
     *
     * Updates an existing User Type
     *
     * **Parameters:**
     *
     * * `type_id: &str`
     */
    pub async fn update(
        &self,
        type_id: &str,
        body: &crate::types::UserType,
    ) -> Result<crate::types::UserType> {
        let url = format!(
            "/api/v1/meta/types/user/{}",
            crate::progenitor_support::encode_path(type_id),
        );
        let url = self.client.url(&url, None);
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                None,
            )
            .await
    }
    /**
     * This function performs a `DELETE` to the `/api/v1/meta/types/user/{typeId}` endpoint.
     *
     * Deletes a User Type permanently. This operation is not permitted for the default type, nor for any User Type that has existing users
     *
     * **Parameters:**
     *
     * * `type_id: &str`
     */
    pub async fn delete(&self, type_id: &str) -> Result<()> {
        let url = format!(
            "/api/v1/meta/types/user/{}",
            crate::progenitor_support::encode_path(type_id),
        );
        let url = self.client.url(&url, None);
        self.client.delete(&url, None, None).await
    }
}
