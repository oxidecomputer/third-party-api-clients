use crate::Client;
use crate::ClientResult;

pub struct ProfileMappings {
    pub client: Client,
}

impl ProfileMappings {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ProfileMappings { client }
    }

    /**
     * This function performs a `GET` to the `/api/v1/mappings` endpoint.
     *
     * Enumerates Profile Mappings in your organization with pagination.
     *
     * **Parameters:**
     *
     * * `after: &str`
     * * `limit: i64`
     * * `source_id: &str`
     * * `target_id: &str`
     */
    pub async fn list(
        &self,
        after: &str,
        limit: i64,
        source_id: &str,
        target_id: &str,
    ) -> ClientResult<Vec<crate::types::ProfileMapping>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !source_id.is_empty() {
            query_args.push(("sourceId".to_string(), source_id.to_string()));
        }
        if !target_id.is_empty() {
            query_args.push(("targetId".to_string(), target_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/api/v1/mappings?{}", query_), None);
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
     * This function performs a `GET` to the `/api/v1/mappings` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Enumerates Profile Mappings in your organization with pagination.
     */
    pub async fn list_all(
        &self,
        source_id: &str,
        target_id: &str,
    ) -> ClientResult<Vec<crate::types::ProfileMapping>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !source_id.is_empty() {
            query_args.push(("sourceId".to_string(), source_id.to_string()));
        }
        if !target_id.is_empty() {
            query_args.push(("targetId".to_string(), target_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/api/v1/mappings?{}", query_), None);
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
     * Get Profile Mapping.
     *
     * This function performs a `GET` to the `/api/v1/mappings/{mappingId}` endpoint.
     *
     * Fetches a single Profile Mapping referenced by its ID.
     *
     * **Parameters:**
     *
     * * `mapping_id: &str`
     */
    pub async fn get(&self, mapping_id: &str) -> ClientResult<crate::types::ProfileMapping> {
        let url = self.client.url(
            &format!(
                "/api/v1/mappings/{}",
                crate::progenitor_support::encode_path(mapping_id),
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
     * Update Profile Mapping.
     *
     * This function performs a `POST` to the `/api/v1/mappings/{mappingId}` endpoint.
     *
     * Updates an existing Profile Mapping by adding, updating, or removing one or many Property Mappings.
     *
     * **Parameters:**
     *
     * * `mapping_id: &str`
     */
    pub async fn update(
        &self,
        mapping_id: &str,
        body: &crate::types::ProfileMapping,
    ) -> ClientResult<crate::types::ProfileMapping> {
        let url = self.client.url(
            &format!(
                "/api/v1/mappings/{}",
                crate::progenitor_support::encode_path(mapping_id),
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
