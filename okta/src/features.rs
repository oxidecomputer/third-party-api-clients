use crate::Client;
use crate::ClientResult;

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
    pub async fn list(&self) -> ClientResult<Vec<crate::types::Feature>> {
        let url = self.client.url("/api/v1/features", None);
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
     * This function performs a `GET` to the `/api/v1/features` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Success
     */
    pub async fn list_all(&self) -> ClientResult<Vec<crate::types::Feature>> {
        let url = self.client.url("/api/v1/features", None);
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
     * This function performs a `GET` to the `/api/v1/features/{featureId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `feature_id: &str`
     */
    pub async fn get(&self, feature_id: &str) -> ClientResult<crate::types::Feature> {
        let url = self.client.url(
            &format!(
                "/api/v1/features/{}",
                crate::progenitor_support::encode_path(feature_id),
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
     * This function performs a `GET` to the `/api/v1/features/{featureId}/dependencies` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `feature_id: &str`
     */
    pub async fn list_dependencies(
        &self,
        feature_id: &str,
    ) -> ClientResult<Vec<crate::types::Feature>> {
        let url = self.client.url(
            &format!(
                "/api/v1/features/{}/dependencies",
                crate::progenitor_support::encode_path(feature_id),
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
     * This function performs a `GET` to the `/api/v1/features/{featureId}/dependencies` endpoint.
     *
     * As opposed to `list_dependencies`, this function returns all the pages of the request at once.
     *
     * Success
     */
    pub async fn list_all_dependencies(
        &self,
        feature_id: &str,
    ) -> ClientResult<Vec<crate::types::Feature>> {
        let url = self.client.url(
            &format!(
                "/api/v1/features/{}/dependencies",
                crate::progenitor_support::encode_path(feature_id),
            ),
            None,
        );
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
     * This function performs a `GET` to the `/api/v1/features/{featureId}/dependents` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `feature_id: &str`
     */
    pub async fn list_dependents(
        &self,
        feature_id: &str,
    ) -> ClientResult<Vec<crate::types::Feature>> {
        let url = self.client.url(
            &format!(
                "/api/v1/features/{}/dependents",
                crate::progenitor_support::encode_path(feature_id),
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
     * This function performs a `GET` to the `/api/v1/features/{featureId}/dependents` endpoint.
     *
     * As opposed to `list_dependents`, this function returns all the pages of the request at once.
     *
     * Success
     */
    pub async fn list_all_dependents(
        &self,
        feature_id: &str,
    ) -> ClientResult<Vec<crate::types::Feature>> {
        let url = self.client.url(
            &format!(
                "/api/v1/features/{}/dependents",
                crate::progenitor_support::encode_path(feature_id),
            ),
            None,
        );
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
    ) -> ClientResult<crate::types::Feature> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !mode.is_empty() {
            query_args.push(("mode".to_string(), mode.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/api/v1/features/{}/{}?{}",
                crate::progenitor_support::encode_path(feature_id),
                crate::progenitor_support::encode_path(lifecycle),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}
