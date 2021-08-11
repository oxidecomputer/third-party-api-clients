use anyhow::Result;

use crate::Client;

pub struct Contractors {
    client: Client,
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
    pub async fn get_contractors_contractor_id(
        &self,
        contractor_id_or_uuid: &str,
    ) -> Result<crate::types::Contractor> {
        let url = format!(
            "/v1/contractors/{}",
            crate::progenitor_support::encode_path(&contractor_id_or_uuid.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Update a contractor.
     *
     * This function performs a `PUT` to the `/v1/contractors/{contractor_id_or_uuid}` endpoint.
     *
     * Update a contractor.
     */
    pub async fn put_contractors_contractor_id(
        &self,
        contractor_id_or_uuid: &str,
        body: &crate::types::PutContractorsContractorIdRequest,
    ) -> Result<crate::types::Contractor> {
        let url = format!(
            "/v1/contractors/{}",
            crate::progenitor_support::encode_path(&contractor_id_or_uuid.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Get contractors of a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id_or_uuid}/contractors` endpoint.
     *
     * Get all contractors, active and inactive, individual and business, for a company.
     */
    pub async fn get_company_contractors(
        &self,
        company_id_or_uuid: &str,
    ) -> Result<Vec<crate::types::Contractor>> {
        let url = format!(
            "/v1/companies/{}/contractors",
            crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Get contractors of a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id_or_uuid}/contractors` endpoint.
     *
     * As opposed to `get_company_contractors`, this function returns all the pages of the request at once.
     *
     * Get all contractors, active and inactive, individual and business, for a company.
     */
    pub async fn get_all_company_contractors(
        &self,
        company_id_or_uuid: &str,
    ) -> Result<Vec<crate::types::Contractor>> {
        let url = format!(
            "/v1/companies/{}/contractors",
            crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),
        );

        self.client.get_all_pages(&url).await
    }

    /**
     * Create a contractor.
     *
     * This function performs a `POST` to the `/v1/companies/{company_id_or_uuid}/contractors` endpoint.
     *
     * Create an individual or business contractor.
     */
    pub async fn post_company_contractors(
        &self,
        company_id_or_uuid: &str,
        body: &crate::types::PostCompanyContractorsRequest,
    ) -> Result<crate::types::Contractor> {
        let url = format!(
            "/v1/companies/{}/contractors",
            crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
