use crate::Client;
use crate::ClientResult;

pub struct EarningType {
    pub client: Client,
}

impl EarningType {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EarningType { client }
    }

    /**
     * Get all earning types for a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/earning_types` endpoint.
     *
     * A payroll item in Gusto is associated to an earning type to name the type of earning described by the payroll item.
     *
     * #### Default Earning Type
     * Certain earning types are special because they have tax considerations. Those earning types are mostly the same for every company depending on its legal structure (LLC, Corporation, etc.)
     *
     * #### Custom Earning Type
     * Custom earning types are all the other earning types added specifically for a company.
     */
    pub async fn get_company(
        &self,
        company_id: &str,
    ) -> ClientResult<crate::types::EarningTypeListResponse> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/earning_types",
                crate::progenitor_support::encode_path(company_id),
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
     * Create a custom earning type.
     *
     * This function performs a `POST` to the `/v1/companies/{company_id}/earning_types` endpoint.
     *
     * Create a custom earning type.
     *
     * If an inactive earning type exists with the same name, this will reactivate it instead of creating a new one.
     */
    pub async fn post_company(
        &self,
        company_id: &str,
        body: &crate::types::PostCompanyEarningTypesRequest,
    ) -> ClientResult<crate::types::EarningType> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/earning_types",
                crate::progenitor_support::encode_path(company_id),
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
     * Update an earning type.
     *
     * This function performs a `PUT` to the `/v1/companies/{company_id}/earning_types/{earning_type_uuid}` endpoint.
     *
     * Update an earning type.
     */
    pub async fn put_company_type(
        &self,
        company_id: &str,
        earning_type_uuid: &str,
        body: &crate::types::PutCompanyEarningTypeRequest,
    ) -> ClientResult<crate::types::EarningType> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/earning_types/{}",
                crate::progenitor_support::encode_path(company_id),
                crate::progenitor_support::encode_path(earning_type_uuid),
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
     * Deactivate an earning type.
     *
     * This function performs a `DELETE` to the `/v1/companies/{company_id}/earning_types/{earning_type_uuid}` endpoint.
     *
     * Deactivate an earning type.
     */
    pub async fn delete_company_type(
        &self,
        company_id: &str,
        earning_type_uuid: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/earning_types/{}",
                crate::progenitor_support::encode_path(company_id),
                crate::progenitor_support::encode_path(earning_type_uuid),
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
}
