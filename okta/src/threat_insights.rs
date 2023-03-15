use crate::Client;
use crate::ClientResult;

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
    ) -> ClientResult<crate::types::ThreatInsightConfiguration> {
        let url = self.client.url("/api/v1/threats/configuration", None);
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
     * This function performs a `POST` to the `/api/v1/threats/configuration` endpoint.
     *
     * Updates ThreatInsight configuration
     */
    pub async fn update_configuration(
        &self,
        body: &crate::types::ThreatInsightConfiguration,
    ) -> ClientResult<crate::types::ThreatInsightConfiguration> {
        let url = self.client.url("/api/v1/threats/configuration", None);
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
