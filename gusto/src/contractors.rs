use crate::Client;
use crate::ClientResult;

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
    pub async fn get(&self, contractor_id_or_uuid: &str) -> ClientResult<crate::types::Contractor> {
        let url = self.client.url(
            &format!(
                "/v1/contractors/{}",
                crate::progenitor_support::encode_path(contractor_id_or_uuid),
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
    ) -> ClientResult<crate::types::Contractor> {
        let url = self.client.url(
            &format!(
                "/v1/contractors/{}",
                crate::progenitor_support::encode_path(contractor_id_or_uuid),
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
     * Get contractors of a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id_or_uuid}/contractors` endpoint.
     *
     * Get all contractors, active and inactive, individual and business, for a company.
     */
    pub async fn get_company(
        &self,
        company_id_or_uuid: &str,
    ) -> ClientResult<Vec<crate::types::Contractor>> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/contractors",
                crate::progenitor_support::encode_path(company_id_or_uuid),
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
    ) -> ClientResult<Vec<crate::types::Contractor>> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/contractors",
                crate::progenitor_support::encode_path(company_id_or_uuid),
            ),
            None,
        );
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
    ) -> ClientResult<crate::types::Contractor> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/contractors",
                crate::progenitor_support::encode_path(company_id_or_uuid),
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
}
