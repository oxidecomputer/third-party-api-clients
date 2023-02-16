use anyhow::Result;

use crate::Client;

pub struct Sessions {
    pub client: Client,
}

impl Sessions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Sessions { client }
    }

    /**
     * Create Session with Session Token.
     *
     * This function performs a `POST` to the `/api/v1/sessions` endpoint.
     *
     * Creates a new session for a user with a valid session token. Use this API if, for example, you want to set the session cookie yourself instead of allowing Okta to set it, or want to hold the session ID in order to delete a session via the API instead of visiting the logout URL.
     */
    pub async fn create(
        &self,
        body: &crate::types::CreateSessionRequest,
    ) -> Result<crate::types::Session> {
        let url = "/api/v1/sessions".to_string();
        let url = self.client.url(&url, None);
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                Some("application/json"),
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/api/v1/sessions/{sessionId}` endpoint.
     *
     * Get details about a session.
     *
     * **Parameters:**
     *
     * * `session_id: &str`
     */
    pub async fn get(&self, session_id: &str) -> Result<crate::types::Session> {
        let url = format!(
            "/api/v1/sessions/{}",
            crate::progenitor_support::encode_path(session_id),
        );
        let url = self.client.url(&url, None);
        self.client.get(&url, None, None).await
    }
    /**
     * Close Session.
     *
     * This function performs a `DELETE` to the `/api/v1/sessions/{sessionId}` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `session_id: &str`
     */
    pub async fn end(&self, session_id: &str) -> Result<()> {
        let url = format!(
            "/api/v1/sessions/{}",
            crate::progenitor_support::encode_path(session_id),
        );
        let url = self.client.url(&url, None);
        self.client.delete(&url, None, None).await
    }
    /**
     * Refresh Session.
     *
     * This function performs a `POST` to the `/api/v1/sessions/{sessionId}/lifecycle/refresh` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `session_id: &str`
     */
    pub async fn refresh(&self, session_id: &str) -> Result<crate::types::Session> {
        let url = format!(
            "/api/v1/sessions/{}/lifecycle/refresh",
            crate::progenitor_support::encode_path(session_id),
        );
        let url = self.client.url(&url, None);
        self.client.post(&url, None, None).await
    }
}
