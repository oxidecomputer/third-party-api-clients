use crate::Client;
use crate::ClientResult;

pub struct VerifiedDomains {
    pub client: Client,
}

impl VerifiedDomains {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        VerifiedDomains { client }
    }

    /**
     * Get domain info.
     *
     * This function performs a `GET` to the `/verified-domains/{domain_name}` endpoint.
     *
     * Get the details for a single domain on the account.
     *
     * **Parameters:**
     *
     * * `domain_name: &str` -- The name of the folder.
     */
    pub async fn get(&self, domain_name: &str) -> ClientResult<crate::types::VerifiedDomains> {
        let url = self.client.url(
            &format!(
                "/verified-domains/{}",
                crate::progenitor_support::encode_path(domain_name),
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
     * Delete domain.
     *
     * This function performs a `DELETE` to the `/verified-domains/{domain_name}` endpoint.
     *
     * Delete a verified domain from the account.
     *
     * **Parameters:**
     *
     * * `domain_name: &str` -- The name of the folder.
     */
    pub async fn delete(&self, domain_name: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/verified-domains/{}",
                crate::progenitor_support::encode_path(domain_name),
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
     * Verify domain.
     *
     * This function performs a `POST` to the `/verified-domains/{domain_name}/actions/verify` endpoint.
     *
     * Verify a domain for sending.
     *
     * **Parameters:**
     *
     * * `domain_name: &str` -- The name of the folder.
     */
    pub async fn verify_domain(
        &self,
        domain_name: &str,
        body: &crate::types::VerifyADomainSending,
    ) -> ClientResult<crate::types::VerifiedDomains> {
        let url = self.client.url(
            &format!(
                "/verified-domains/{}/actions/verify",
                crate::progenitor_support::encode_path(domain_name),
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
    /**
     * List sending domains.
     *
     * This function performs a `GET` to the `/verified-domains` endpoint.
     *
     * Get all of the sending domains on the account.
     */
    pub async fn get_verified_domains(&self) -> ClientResult<crate::types::VerifiedDomainsData> {
        let url = self.client.url("/verified-domains", None);
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
     * Add domain to account.
     *
     * This function performs a `POST` to the `/verified-domains` endpoint.
     *
     * Add a domain to the account.
     */
    pub async fn create(
        &self,
        body: &crate::types::VerifiedDomainsDataType,
    ) -> ClientResult<crate::types::VerifiedDomains> {
        let url = self.client.url("/verified-domains", None);
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
