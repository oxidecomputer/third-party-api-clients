use crate::Client;
use crate::ClientResult;

pub struct TrustedOrigins {
    pub client: Client,
}

impl TrustedOrigins {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TrustedOrigins { client }
    }

    /**
     * This function performs a `GET` to the `/api/v1/trustedOrigins` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `q: &str`
     * * `filter: &str`
     * * `after: &str`
     * * `limit: i64`
     */
    pub async fn list_origins(
        &self,
        q: &str,
        filter: &str,
        after: &str,
        limit: i64,
    ) -> ClientResult<Vec<crate::types::TrustedOrigin>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/api/v1/trustedOrigins?{}", query_), None);
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
     * This function performs a `GET` to the `/api/v1/trustedOrigins` endpoint.
     *
     * As opposed to `list_origins`, this function returns all the pages of the request at once.
     *
     * Success
     */
    pub async fn list_all_origins(
        &self,
        q: &str,
        filter: &str,
    ) -> ClientResult<Vec<crate::types::TrustedOrigin>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if !q.is_empty() {
            query_args.push(("q".to_string(), q.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/api/v1/trustedOrigins?{}", query_), None);
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
     * This function performs a `POST` to the `/api/v1/trustedOrigins` endpoint.
     *
     * Success
     */
    pub async fn create_origin(
        &self,
        body: &crate::types::TrustedOrigin,
    ) -> ClientResult<crate::types::TrustedOrigin> {
        let url = self.client.url("/api/v1/trustedOrigins", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/api/v1/trustedOrigins/{trustedOriginId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `trusted_origin_id: &str`
     */
    pub async fn get_origin(
        &self,
        trusted_origin_id: &str,
    ) -> ClientResult<crate::types::TrustedOrigin> {
        let url = self.client.url(
            &format!(
                "/api/v1/trustedOrigins/{}",
                crate::progenitor_support::encode_path(trusted_origin_id),
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
     * This function performs a `PUT` to the `/api/v1/trustedOrigins/{trustedOriginId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `trusted_origin_id: &str`
     */
    pub async fn update_origin(
        &self,
        trusted_origin_id: &str,
        body: &crate::types::TrustedOrigin,
    ) -> ClientResult<crate::types::TrustedOrigin> {
        let url = self.client.url(
            &format!(
                "/api/v1/trustedOrigins/{}",
                crate::progenitor_support::encode_path(trusted_origin_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * This function performs a `DELETE` to the `/api/v1/trustedOrigins/{trustedOriginId}` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `trusted_origin_id: &str`
     */
    pub async fn delete_origin(&self, trusted_origin_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/trustedOrigins/{}",
                crate::progenitor_support::encode_path(trusted_origin_id),
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
     * This function performs a `POST` to the `/api/v1/trustedOrigins/{trustedOriginId}/lifecycle/activate` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `trusted_origin_id: &str`
     */
    pub async fn activate_origin(
        &self,
        trusted_origin_id: &str,
    ) -> ClientResult<crate::types::TrustedOrigin> {
        let url = self.client.url(
            &format!(
                "/api/v1/trustedOrigins/{}/lifecycle/activate",
                crate::progenitor_support::encode_path(trusted_origin_id),
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
    /**
     * This function performs a `POST` to the `/api/v1/trustedOrigins/{trustedOriginId}/lifecycle/deactivate` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `trusted_origin_id: &str`
     */
    pub async fn deactivate_origin(
        &self,
        trusted_origin_id: &str,
    ) -> ClientResult<crate::types::TrustedOrigin> {
        let url = self.client.url(
            &format!(
                "/api/v1/trustedOrigins/{}/lifecycle/deactivate",
                crate::progenitor_support::encode_path(trusted_origin_id),
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
