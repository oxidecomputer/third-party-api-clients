use anyhow::Result;

use crate::Client;

pub struct Domains {
    pub client: Client,
}

impl Domains {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Domains { client }
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/domains` endpoint.
     *
     * Lists the domains of the customer.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- Immutable ID of the Google Workspace account.
     */
    pub async fn list(&self, customer: &str) -> Result<crate::types::Domains2> {
        let url = format!(
            "/admin/directory/v1/customer/{}/domains",
            crate::progenitor_support::encode_path(customer),
        );
        let url = self.client.url(&url, None);
        self.client.get(&url, None, None).await
    }
    /**
     * This function performs a `POST` to the `/admin/directory/v1/customer/{customer}/domains` endpoint.
     *
     * Inserts a domain of the customer.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- Immutable ID of the Google Workspace account.
     */
    pub async fn insert(
        &self,
        customer: &str,
        body: &crate::types::Domains,
    ) -> Result<crate::types::Domains> {
        let url = format!(
            "/admin/directory/v1/customer/{}/domains",
            crate::progenitor_support::encode_path(customer),
        );
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
     * This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/domains/{domainName}` endpoint.
     *
     * Retrieves a domain of the customer.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- Immutable ID of the Google Workspace account.
     * * `domain_name: &str` -- Name of domain to be retrieved.
     */
    pub async fn get(&self, customer: &str, domain_name: &str) -> Result<crate::types::Domains> {
        let url = format!(
            "/admin/directory/v1/customer/{}/domains/{}",
            crate::progenitor_support::encode_path(customer),
            crate::progenitor_support::encode_path(domain_name),
        );
        let url = self.client.url(&url, None);
        self.client.get(&url, None, None).await
    }
    /**
     * This function performs a `DELETE` to the `/admin/directory/v1/customer/{customer}/domains/{domainName}` endpoint.
     *
     * Deletes a domain of the customer.
     *
     * **Parameters:**
     *
     * * `customer: &str` -- Immutable ID of the Google Workspace account.
     * * `domain_name: &str` -- Name of domain to be deleted.
     */
    pub async fn delete(&self, customer: &str, domain_name: &str) -> Result<()> {
        let url = format!(
            "/admin/directory/v1/customer/{}/domains/{}",
            crate::progenitor_support::encode_path(customer),
            crate::progenitor_support::encode_path(domain_name),
        );
        let url = self.client.url(&url, None);
        self.client.delete(&url, None, None).await
    }
}
