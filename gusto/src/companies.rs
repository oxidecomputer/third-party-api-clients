use anyhow::Result;

use crate::Client;

pub struct Companies {
    client: Client,
}

impl Companies {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        Companies {
            client,
        }
    }

    /**
* Get a company.
*
* This function performs a `GET` to the `/v1/companies/{company_id_or_uuid}` endpoint.
*
* Get a company.
*/
pub async fn get(
&self,
company_id_or_uuid: &str,
) -> Result<crate::types::Company> {
let url =
format!("/v1/companies/{}",
crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),);

self.client.get(&url, None).await
}

/**
* Create a partner managed company (Beta).
*
* This function performs a `POST` to the `/v1/partner_managed_companies` endpoint.
*
* This endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.
* 
* ### Overview
* 
* The partner managed company API provides a way to create a Gusto company that you can manage. This endpoint behaves similarly to [creating a company](../~1v1~1provision/post) in that it does the following:
* 
* * Creates a new company in Gusto.
* * Creates a new user in Gusto.
* * Makes the new user the primary payroll administrator of the new company.
* * Sends a welcome email to the new user.
* 
* Additionally, on successful creation of the company, this API will do the following:
* * Creates a link between the partner and the company.
* * Creates access tokens and refresh tokens that can be used immediately.
* 
* In the response, you will receive the access token, the refresh token, and the uuid of the created company.
* 
* ### Authentication
* 
* Due to the nature of this endpoint, Gusto will provide partners with an API token and will permit partners to use API Token Authentication instead of OAuth to provision Gusto accounts. The API token is included in the authorization HTTP header with the Token scheme, e.g.:
* 
* ```
* Content-Type: application/json
* Authorization: Token bbb286ff1a4fe6b84742b0d49b8d0d65bd0208d27d3d50333591df71
* ```
*/
pub async fn post_partner_managed_companie(
&self,
body: &crate::types::PostV1PartnerManagedCompaniesRequest
) -> Result<crate::types::PostV1PartnerManagedCompaniesResponse> {
let url =
"/v1/partner_managed_companies".to_string();
self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Create a company.
*
* This function performs a `POST` to the `/v1/provision` endpoint.
*
* ### Overview
* 
* The company provisioning API provides a way to create a Gusto company as part of your integration. When you successfully call the API, the API does the following:
* 
* * Creates a new company in Gusto.
* * Creates a new user in Gusto.
* * Makes the new user the primary payroll administrator of the new company.
* * Sends a welcome email to the new user.
* 
* In the response, you will receive an account claim URL. Redirect the user to this URL to complete their account setup inside of Gusto
* 
* ### Authentication
* 
* Due to the nature of this endpoint, Gusto will provide partners with an API token and will permit partners to use API Token Authentication instead of OAuth to provision Gusto accounts. The API token is included in the authorization HTTP header with the Token scheme, e.g.:
* 
* ```
* Content-Type: application/json
* Authorization: Token bbb286ff1a4fe6b84742b0d49b8d0d65bd0208d27d3d50333591df71
* ```
*/
pub async fn post_provision(
&self,
body: &crate::types::PostV1ProvisionRequest
) -> Result<crate::types::PostV1ProvisionResponse> {
let url =
"/v1/provision".to_string();
self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}


}