use anyhow::Result;

use crate::Client;

pub struct AdminsBeta {
    client: Client,
}

impl AdminsBeta {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AdminsBeta { client }
    }

    /**
     * Get all the admins at a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/admins` endpoint.
     *
     * *This endpoint is in beta. Please contact developer-gws@gusto.com if you’d like to have more information and use it for production. Note, this may require you to enter a different agreement with Gusto.
     *
     * Returns a list of all the admins at a company
     */
    pub async fn get_v_1_companies_company_id_admins(&self) -> Result<Vec<crate::types::Admin>> {
        let url = format!(
            "/v1/companies/{}/admins",
            crate::progenitor_support::encode_path(&company_id.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Get all the admins at a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/admins` endpoint.
     *
     * As opposed to `get_v_1_companies_company_id_admins`, this function returns all the pages of the request at once.
     *
     * *This endpoint is in beta. Please contact developer-gws@gusto.com if you’d like to have more information and use it for production. Note, this may require you to enter a different agreement with Gusto.
     *
     * Returns a list of all the admins at a company
     */
    pub async fn get_v_1_companies_company_id_admins(&self) -> Result<Vec<crate::types::Admin>> {
        let url = format!(
            "/v1/companies/{}/admins",
            crate::progenitor_support::encode_path(&company_id.to_string()),
        );

        self.client.get_all_pages(&url).await
    }

    /**
     * Create an admin for the company.
     *
     * This function performs a `POST` to the `/v1/companies/{company_id}/admins` endpoint.
     *
     * *This endpoint is in beta. Please contact developer-gws@gusto.com if you’d like to have more information and use it for production. Note, this may require you to enter a different agreement with Gusto.
     *
     * Creates a new admin for a company. If the email matches an existing user, this will create an admin account for the current user. Otherwise, this will create a new user.
     */
    pub async fn post_v_1_companies_company_id_admins(
        &self,
        body: &crate::types::PostV1CompaniesCompanyIdAdminsRequest,
    ) -> Result<crate::types::Admin> {
        let url = format!(
            "/v1/companies/{}/admins",
            crate::progenitor_support::encode_path(&company_id.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
