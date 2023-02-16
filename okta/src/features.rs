use anyhow::Result;

use crate::Client;

pub struct Features {
    pub client: Client,
}

impl Features {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Features { client }
    }

    /**
     * This function performs a `GET` to the `/api/v1/features` endpoint.
     *
     * Success
     */
    pub async fn list(&self) -> Result<Vec<crate::types::Feature>> {
        let url = "/api/v1/features".to_string();
        let url = self.client.url(&url, None);
        self.client.get(&url, None).await
    }
    /**
     * This function performs a `GET` to the `/api/v1/features` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Success
     */
    pub async fn list_all(&self) -> Result<Vec<crate::types::Feature>> {
        let url = "/api/v1/features".to_string();
        self.client.get_all_pages(&url, None).await
    }
    /**
     * This function performs a `GET` to the `/api/v1/features/{featureId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `feature_id: &str`
     */
    pub async fn get(&self, feature_id: &str) -> Result<crate::types::Feature> {
        let url = format!(
            "/api/v1/features/{}",
            crate::progenitor_support::encode_path(&feature_id.to_string()),
        );
        let url = self.client.url(&url, None);
        self.client.get(&url, None).await
    }
    /**
     * This function performs a `GET` to the `/api/v1/features/{featureId}/dependencies` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `feature_id: &str`
     */
    pub async fn list_dependencies(&self, feature_id: &str) -> Result<Vec<crate::types::Feature>> {
        let url = format!(
            "/api/v1/features/{}/dependencies",
            crate::progenitor_support::encode_path(&feature_id.to_string()),
        );
        let url = self.client.url(&url, None);
        self.client.get(&url, None).await
    }
    /**
     * This function performs a `GET` to the `/api/v1/features/{featureId}/dependencies` endpoint.
     *
     * As opposed to `list_dependencies`, this function returns all the pages of the request at once.
     *
     * Success
     */
    pub async fn list_all_dependencies(
        &self,
        feature_id: &str,
    ) -> Result<Vec<crate::types::Feature>> {
        let url = format!(
            "/api/v1/features/{}/dependencies",
            crate::progenitor_support::encode_path(&feature_id.to_string()),
        );
        self.client.get_all_pages(&url, None).await
    }
    /**
     * This function performs a `GET` to the `/api/v1/features/{featureId}/dependents` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `feature_id: &str`
     */
    pub async fn list_dependents(&self, feature_id: &str) -> Result<Vec<crate::types::Feature>> {
        let url = format!(
            "/api/v1/features/{}/dependents",
            crate::progenitor_support::encode_path(&feature_id.to_string()),
        );
        let url = self.client.url(&url, None);
        self.client.get(&url, None).await
    }
    /**
     * This function performs a `GET` to the `/api/v1/features/{featureId}/dependents` endpoint.
     *
     * As opposed to `list_dependents`, this function returns all the pages of the request at once.
     *
     * Success
     */
    pub async fn list_all_dependents(
        &self,
        feature_id: &str,
    ) -> Result<Vec<crate::types::Feature>> {
        let url = format!(
            "/api/v1/features/{}/dependents",
            crate::progenitor_support::encode_path(&feature_id.to_string()),
        );
        self.client.get_all_pages(&url, None).await
    }
    /**
     * This function performs a `POST` to the `/api/v1/features/{featureId}/{lifecycle}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `feature_id: &str`
     * * `lifecycle: &str`
     * * `mode: &str`
     */
    pub async fn update_lifecycle(
        &self,
        feature_id: &str,
        lifecycle: &str,
        mode: &str,
    ) -> Result<crate::types::Feature> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !mode.is_empty() {
            query_args.push(("mode".to_string(), mode.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/api/v1/features/{}/{}?{}",
            crate::progenitor_support::encode_path(&feature_id.to_string()),
            crate::progenitor_support::encode_path(&lifecycle.to_string()),
            query_
        );
        let url = self.client.url(&url, None);
        self.client.post(&url, None).await
    }
}
