use anyhow::Result;

use crate::Client;

pub struct AdminsBeta {
    client: Client,
}

impl AdminsBeta {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        AdminsBeta {
            client,
        }
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
) -> Result<Vec<crate::types::Admin>> {
let url =
format!("/v1/companies/{}/admins",
crate::progenitor_support::encode_path(&company_id.to_string()),);

self.client.get(&url, None).await
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
) -> Result<Vec<crate::types::Admin>> {
let url =
format!("/v1/companies/{}/admins",
crate::progenitor_support::encode_path(&company_id.to_string()),);

self.client.get_all_pages(&url, None).await
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
body: &crate::types::PostCompanyAdminsRequest
) -> Result<crate::types::Admin> {
let url =
format!("/v1/companies/{}/admins",
crate::progenitor_support::encode_path(&company_id.to_string()),);

self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}


}