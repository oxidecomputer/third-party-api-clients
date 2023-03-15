use crate::Client;
use crate::ClientResult;

pub struct Domains {
    pub client: Client,
}

impl Domains {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Domains { client }
    }

    /**
     * List Domains.
     *
     * This function performs a `GET` to the `/api/v1/domains` endpoint.
     *
     * List all verified custom Domains for the org.
     */
    pub async fn list(&self) -> ClientResult<crate::types::DomainListResponse> {
        let url = self.client.url("/api/v1/domains", None);
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
     * Create Domain.
     *
     * This function performs a `POST` to the `/api/v1/domains` endpoint.
     *
     * Creates your domain.
     */
    pub async fn create(&self, body: &crate::types::Domain) -> ClientResult<crate::types::Domain> {
        let url = self.client.url("/api/v1/domains", None);
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
     * Get Domain.
     *
     * This function performs a `GET` to the `/api/v1/domains/{domainId}` endpoint.
     *
     * Fetches a Domain by `id`.
     *
     * **Parameters:**
     *
     * * `domain_id: &str`
     */
    pub async fn get(&self, domain_id: &str) -> ClientResult<crate::types::Domain> {
        let url = self.client.url(
            &format!(
                "/api/v1/domains/{}",
                crate::progenitor_support::encode_path(domain_id),
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
     * Delete Domain.
     *
     * This function performs a `DELETE` to the `/api/v1/domains/{domainId}` endpoint.
     *
     * Deletes a Domain by `id`.
     *
     * **Parameters:**
     *
     * * `domain_id: &str`
     */
    pub async fn delete(&self, domain_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/domains/{}",
                crate::progenitor_support::encode_path(domain_id),
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
     * Create Certificate.
     *
     * This function performs a `PUT` to the `/api/v1/domains/{domainId}/certificate` endpoint.
     *
     * Creates the Certificate for the Domain.
     *
     * **Parameters:**
     *
     * * `domain_id: &str`
     */
    pub async fn create_certificate(
        &self,
        domain_id: &str,
        body: &crate::types::DomainCertificate,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/domains/{}/certificate",
                crate::progenitor_support::encode_path(domain_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Verify Domain.
     *
     * This function performs a `POST` to the `/api/v1/domains/{domainId}/verify` endpoint.
     *
     * Verifies the Domain by `id`.
     *
     * **Parameters:**
     *
     * * `domain_id: &str`
     */
    pub async fn verify(&self, domain_id: &str) -> ClientResult<crate::types::Domain> {
        let url = self.client.url(
            &format!(
                "/api/v1/domains/{}/verify",
                crate::progenitor_support::encode_path(domain_id),
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
