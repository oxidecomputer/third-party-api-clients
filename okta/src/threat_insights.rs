use anyhow::Result;

use crate::Client;

pub struct ThreatInsights {
    pub client: Client,
}

impl ThreatInsights {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ThreatInsights { client }
    }

    /**
     * This function performs a `GET` to the `/api/v1/threats/configuration` endpoint.
     *
     * Gets current ThreatInsight configuration
     */
    pub async fn get_current_configuration(
        &self,
    ) -> Result<crate::types::ThreatInsightConfiguration> {
        let url = self.client.url("/api/v1/threats/configuration", None);
        self.client.get(&url, None, None).await
    }
    /**
     * This function performs a `POST` to the `/api/v1/threats/configuration` endpoint.
     *
     * Updates ThreatInsight configuration
     */
    pub async fn update_configuration(
        &self,
        body: &crate::types::ThreatInsightConfiguration,
    ) -> Result<crate::types::ThreatInsightConfiguration> {
        let url = self.client.url("/api/v1/threats/configuration", None);
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                Some("application/json"),
            )
            .await
    }
}
