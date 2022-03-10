use anyhow::Result;

use crate::Client;

pub struct Contractors {
    pub client: Client,
}

impl Contractors {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Contractors { client }
    }

    /**
     * Get a contractor.
     *
     * This function performs a `GET` to the `/v1/contractors/{contractor_id_or_uuid}` endpoint.
     *
     * Get a contractor.
     */
    pub async fn get(&self, contractor_id_or_uuid: &str) -> Result<crate::types::Contractor> {
        let url = format!(
            "/v1/contractors/{}",
            crate::progenitor_support::encode_path(contractor_id_or_uuid),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update a contractor.
     *
     * This function performs a `PUT` to the `/v1/contractors/{contractor_id_or_uuid}` endpoint.
     *
     * Update a contractor.
     */
    pub async fn put(
        &self,
        contractor_id_or_uuid: &str,
        body: &crate::types::PutComntractorRequest,
    ) -> Result<crate::types::Contractor> {
        let url = format!(
            "/v1/contractors/{}",
            crate::progenitor_support::encode_path(contractor_id_or_uuid),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get contractors of a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id_or_uuid}/contractors` endpoint.
     *
     * Get all contractors, active and inactive, individual and business, for a company.
     */
    pub async fn get_company(
        &self,
        company_id_or_uuid: &str,
    ) -> Result<Vec<crate::types::Contractor>> {
        let url = format!(
            "/v1/companies/{}/contractors",
            crate::progenitor_support::encode_path(company_id_or_uuid),
        );

        self.client.get(&url, None).await
    }

    /**
     * Get contractors of a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id_or_uuid}/contractors` endpoint.
     *
     * As opposed to `get_company`, this function returns all the pages of the request at once.
     *
     * Get all contractors, active and inactive, individual and business, for a company.
     */
    pub async fn get_all_company(
        &self,
        company_id_or_uuid: &str,
    ) -> Result<Vec<crate::types::Contractor>> {
        let url = format!(
            "/v1/companies/{}/contractors",
            crate::progenitor_support::encode_path(company_id_or_uuid),
        );

        self.client.get_all_pages(&url, None).await
    }

    /**
     * Create a contractor.
     *
     * This function performs a `POST` to the `/v1/companies/{company_id_or_uuid}/contractors` endpoint.
     *
     * Create an individual or business contractor.
     */
    pub async fn post_company(
        &self,
        company_id_or_uuid: &str,
        body: &crate::types::PostCompanyContractorsRequest,
    ) -> Result<crate::types::Contractor> {
        let url = format!(
            "/v1/companies/{}/contractors",
            crate::progenitor_support::encode_path(company_id_or_uuid),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
