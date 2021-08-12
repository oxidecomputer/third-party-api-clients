use anyhow::Result;

use crate::Client;

pub struct FederalTaxDetailsBeta {
    client: Client,
}

impl FederalTaxDetailsBeta {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        FederalTaxDetailsBeta {
            client,
        }
    }

    /**
* Get Federal Tax Details.
*
* This function performs a `GET` to the `/v1/companies/{company_id_or_uuid}/federal_tax_details` endpoint.
*
* This endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.
* 
* Fetches attributes relevant for a company's federal taxes.
*/
pub async fn get_company_federal_tax_details(
&self,
company_id_or_uuid: &str,
) -> Result<crate::types::GetV1CompaniesCompanyIdUuidFederalTaxDetailsResponse> {
let url =
format!("/v1/companies/{}/federal_tax_details",
crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),);

self.client.get(&url, None).await
}

/**
* Update Federal Tax Details.
*
* This function performs a `PUT` to the `/v1/companies/{company_id_or_uuid}/federal_tax_details` endpoint.
*
* This endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.
* 
* Updates attributes relevant for a company's federal taxes. This information is required is to onboard a company for use with Gusto Embedded Payroll.
*/
pub async fn put_company_federal_tax_details(
&self,
company_id_or_uuid: &str,
body: &crate::types::PutV1CompaniesCompanyIdUuidFederalTaxDetailsRequest
) -> Result<crate::types::GetV1CompaniesCompanyIdUuidFederalTaxDetailsResponse> {
let url =
format!("/v1/companies/{}/federal_tax_details",
crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),);

self.client.put(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}


}