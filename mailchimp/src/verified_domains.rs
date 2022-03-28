use anyhow::Result;

use crate::Client;

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
    pub async fn get(&self, domain_name: &str) -> Result<crate::types::VerifiedDomains> {
        let url = format!(
            "/verified-domains/{}",
            crate::progenitor_support::encode_path(&domain_name.to_string()),
        );

        self.client.get(&url, None).await
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
    pub async fn delete(&self, domain_name: &str) -> Result<()> {
        let url = format!(
            "/verified-domains/{}",
            crate::progenitor_support::encode_path(&domain_name.to_string()),
        );

        self.client.delete(&url, None).await
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
    ) -> Result<crate::types::VerifiedDomains> {
        let url = format!(
            "/verified-domains/{}/actions/verify",
            crate::progenitor_support::encode_path(&domain_name.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List sending domains.
    *
    * This function performs a `GET` to the `/verified-domains` endpoint.
    *
    * Get all of the sending domains on the account.
    */
    pub async fn get_verified_domains(&self) -> Result<crate::types::VerifiedDomainsData> {
        let url = "/verified-domains".to_string();
        self.client.get(&url, None).await
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
    ) -> Result<crate::types::VerifiedDomains> {
        let url = "/verified-domains".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
