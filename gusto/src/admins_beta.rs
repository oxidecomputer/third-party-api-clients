use crate::Client;
use crate::ClientResult;

pub struct AdminsBeta {
    pub client: Client,
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
     * This endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.
     *
     * Returns a list of all the admins at a company
     */
    pub async fn get_company_admins(
        &self,
        company_id: &str,
    ) -> ClientResult<Vec<crate::types::Admin>> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/admins",
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
     * Get all the admins at a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/admins` endpoint.
     *
     * As opposed to `get_company_admins`, this function returns all the pages of the request at once.
     *
     * This endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.
     *
     * Returns a list of all the admins at a company
     */
    pub async fn get_all_company_admins(
        &self,
        company_id: &str,
    ) -> ClientResult<Vec<crate::types::Admin>> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/admins",
                crate::progenitor_support::encode_path(company_id),
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
     * Create an admin for the company.
     *
     * This function performs a `POST` to the `/v1/companies/{company_id}/admins` endpoint.
     *
     * This endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.
     *
     * Creates a new admin for a company. If the email matches an existing user, this will create an admin account for the current user. Otherwise, this will create a new user.
     */
    pub async fn post_company_admin(
        &self,
        company_id: &str,
        body: &crate::types::PostCompanyAdminsRequest,
    ) -> ClientResult<crate::types::Admin> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/admins",
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
}
