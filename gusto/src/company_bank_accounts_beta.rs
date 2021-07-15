use anyhow::Result;

use crate::Client;

pub struct CompanyBankAccountsBeta {
    client: Client,
}

impl CompanyBankAccountsBeta {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        CompanyBankAccountsBeta {
            client,
        }
    }

    /**
* Get all company bank accounts.
*
* This function performs a `GET` to the `/v1/companies/{company_id_or_uuid}/bank_accounts` endpoint.
*
* *This endpoint is in beta. Please contact developer-gws@gusto.com if you’d like to have more information and use it for production. Note, this may require you to enter a different agreement with Gusto.
* 
* Returns all company bank accounts
*/
pub async fn get_v_1_companies_company_id_bank_accounts(
&self,
) -> Result<Vec<crate::types::CompanyBankAccount>> {
let url =
format!("/v1/companies/{}/bank_accounts",
crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),);

self.client.get(&url).await
}

/**
* Get all company bank accounts.
*
* This function performs a `GET` to the `/v1/companies/{company_id_or_uuid}/bank_accounts` endpoint.
*
* As opposed to `get_v_1_companies_company_id_bank_accounts`, this function returns all the pages of the request at once.
*
* *This endpoint is in beta. Please contact developer-gws@gusto.com if you’d like to have more information and use it for production. Note, this may require you to enter a different agreement with Gusto.
* 
* Returns all company bank accounts
*/
pub async fn get_v_1_companies_company_id_bank_accounts(
&self,
) -> Result<Vec<crate::types::CompanyBankAccount>> {
let url =
format!("/v1/companies/{}/bank_accounts",
crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),);

self.client.get_all_pages(&url).await
}

/**
* Create a company bank account.
*
* This function performs a `POST` to the `/v1/companies/{company_id_or_uuid}/bank_accounts` endpoint.
*
* *This endpoint is in beta. Please contact developer-gws@gusto.com if you’d like to have more information and use it for production. Note, this may require you to enter a different agreement with Gusto.
* 
* Create a company bank account. The new bank account will replace an existing bank account as the default company funding method.
*/
pub async fn post_v_1_companies_company_id_bank_accounts(
&self,
body: &crate::types::PostV1CompaniesCompanyIdBankAccountsRequest
) -> Result<crate::types::CompanyBankAccount> {
let url =
format!("/v1/companies/{}/bank_accounts",
crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),);

self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Verify a company bank account.
*
* This function performs a `PUT` to the `/v1/companies/{company_id_or_uuid}/bank_accounts/{bank_account_uuid}/verify` endpoint.
*
* This endpoint is in beta. Please contact developer-gws@gusto.com if you’d like to have more information and use it for production. Note, this may require you to enter a different agreement with Gusto.
* 
* Verify a company bank account by confirming the two micro-deposits sent to the bank account. Note that the order of the two deposits specified in request parameters does not matter.
*/
pub async fn put_v_1_companies_company_id_bank_accounts_verify(
&self,
body: &crate::types::PutV1CompaniesCompanyIdBankAccountsVerifyRequest
) -> Result<crate::types::CompanyBankAccount> {
let url =
format!("/v1/companies/{}/bank_accounts/{}/verify",
crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),crate::progenitor_support::encode_path(&bank_account_uuid.to_string()),);

self.client.put(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}


}