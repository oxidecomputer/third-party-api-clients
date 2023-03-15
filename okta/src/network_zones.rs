use crate::Client;
use crate::ClientResult;

pub struct NetworkZones {
    pub client: Client,
}

impl NetworkZones {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        NetworkZones { client }
    }

    /**
     * List Network Zones.
     *
     * This function performs a `GET` to the `/api/v1/zones` endpoint.
     *
     * Enumerates network zones added to your organization with pagination. A subset of zones can be returned that match a supported filter expression or query.
     *
     * **Parameters:**
     *
     * * `after: &str` -- Specifies the pagination cursor for the next page of network zones.
     * * `limit: i64` -- Specifies the number of results for a page.
     * * `filter: &str` -- Filters zones by usage or id expression.
     */
    pub async fn list(
        &self,
        after: &str,
        limit: i64,
        filter: &str,
    ) -> ClientResult<Vec<crate::types::NetworkZone>> {
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
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/api/v1/zones?{}", query_), None);
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
     * List Network Zones.
     *
     * This function performs a `GET` to the `/api/v1/zones` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Enumerates network zones added to your organization with pagination. A subset of zones can be returned that match a supported filter expression or query.
     */
    pub async fn list_all(&self, filter: &str) -> ClientResult<Vec<crate::types::NetworkZone>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/api/v1/zones?{}", query_), None);
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
     * Add Network Zone.
     *
     * This function performs a `POST` to the `/api/v1/zones` endpoint.
     *
     * Adds a new network zone to your Okta organization.
     */
    pub async fn create(
        &self,
        body: &crate::types::NetworkZone,
    ) -> ClientResult<crate::types::NetworkZone> {
        let url = self.client.url("/api/v1/zones", None);
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
     * Get Network Zone.
     *
     * This function performs a `GET` to the `/api/v1/zones/{zoneId}` endpoint.
     *
     * Fetches a network zone from your Okta organization by `id`.
     *
     * **Parameters:**
     *
     * * `zone_id: &str`
     */
    pub async fn get(&self, zone_id: &str) -> ClientResult<crate::types::NetworkZone> {
        let url = self.client.url(
            &format!(
                "/api/v1/zones/{}",
                crate::progenitor_support::encode_path(zone_id),
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
     * Update Network Zone.
     *
     * This function performs a `PUT` to the `/api/v1/zones/{zoneId}` endpoint.
     *
     * Updates a network zone in your organization.
     *
     * **Parameters:**
     *
     * * `zone_id: &str`
     */
    pub async fn update(
        &self,
        zone_id: &str,
        body: &crate::types::NetworkZone,
    ) -> ClientResult<crate::types::NetworkZone> {
        let url = self.client.url(
            &format!(
                "/api/v1/zones/{}",
                crate::progenitor_support::encode_path(zone_id),
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
     * Delete Network Zone.
     *
     * This function performs a `DELETE` to the `/api/v1/zones/{zoneId}` endpoint.
     *
     * Removes network zone.
     *
     * **Parameters:**
     *
     * * `zone_id: &str`
     */
    pub async fn delete(&self, zone_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/zones/{}",
                crate::progenitor_support::encode_path(zone_id),
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
     * Activate Network Zone.
     *
     * This function performs a `POST` to the `/api/v1/zones/{zoneId}/lifecycle/activate` endpoint.
     *
     * Activate Network Zone
     *
     * **Parameters:**
     *
     * * `zone_id: &str`
     */
    pub async fn activate(&self, zone_id: &str) -> ClientResult<crate::types::NetworkZone> {
        let url = self.client.url(
            &format!(
                "/api/v1/zones/{}/lifecycle/activate",
                crate::progenitor_support::encode_path(zone_id),
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
     * Deactivate Network Zone.
     *
     * This function performs a `POST` to the `/api/v1/zones/{zoneId}/lifecycle/deactivate` endpoint.
     *
     * Deactivates a network zone.
     *
     * **Parameters:**
     *
     * * `zone_id: &str`
     */
    pub async fn deactivate(&self, zone_id: &str) -> ClientResult<crate::types::NetworkZone> {
        let url = self.client.url(
            &format!(
                "/api/v1/zones/{}/lifecycle/deactivate",
                crate::progenitor_support::encode_path(zone_id),
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
