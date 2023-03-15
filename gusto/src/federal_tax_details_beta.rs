
use crate::ClientResult;
use crate::Client;



pub struct FederalTaxDetailsBeta {
    pub client: Client,
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
*/pub async fn get_company_or_federal_tax_details(&self,company_id_or_uuid: &str,) -> ClientResult<crate::types::GetCompanyFederalTaxDetailsResponse> {let url = self.client.url(
&format!("/v1/companies/{}/federal_tax_details",
crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),), None);
self.client.get(&url, crate::Message { body: None, content_type: None } ).await}/**
* Update Federal Tax Details.
*
* This function performs a `PUT` to the `/v1/companies/{company_id_or_uuid}/federal_tax_details` endpoint.
*
* This endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.
* 
* Updates attributes relevant for a company's federal taxes. This information is required is to onboard a company for use with Gusto Embedded Payroll.
*/pub async fn put_company_or_federal_tax_details(&self,company_id_or_uuid: &str,body: &crate::types::PutCompanyFederalTaxDetailsRequest) -> ClientResult<crate::types::GetCompanyFederalTaxDetailsResponse> {let url = self.client.url(
&format!("/v1/companies/{}/federal_tax_details",
crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),), None);
self.client.put(&url, crate::Message { body: Some(reqwest::Body::from(serde_json::to_vec(body)?)), content_type: Some("application/json".to_string()) } ).await}
}