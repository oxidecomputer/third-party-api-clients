use anyhow::Result;

use crate::Client;

pub struct Benefits {
    client: Client,
}

impl Benefits {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Benefits { client }
    }

    /**
     * Get all benefits supported by Gusto.
     *
     * This function performs a `GET` to the `/v1/benefits` endpoint.
     *
     * Returns all benefits supported by Gusto.
     *
     * The benefit object in Gusto contains high level information about a particular benefit type and its tax considerations. When companies choose to offer a benefit, they are creating a Company Benefit object associated with a particular benefit.
     */
    pub async fn get_v_1_benefits(&self) -> Result<Vec<crate::types::SupportedBenefit>> {
        let url = "/v1/benefits".to_string();
        self.client.get(&url).await
    }

    /**
     * Get all benefits supported by Gusto.
     *
     * This function performs a `GET` to the `/v1/benefits` endpoint.
     *
     * As opposed to `get_v_1_benefits`, this function returns all the pages of the request at once.
     *
     * Returns all benefits supported by Gusto.
     *
     * The benefit object in Gusto contains high level information about a particular benefit type and its tax considerations. When companies choose to offer a benefit, they are creating a Company Benefit object associated with a particular benefit.
     */
    pub async fn get_v_1_benefits(&self) -> Result<Vec<crate::types::SupportedBenefit>> {
        let url = "/v1/benefits".to_string();
        self.client.get_all_pages(&url).await
    }

    /**
     * Get a supported benefit by ID.
     *
     * This function performs a `GET` to the `/v1/benefits/{benefit_id}` endpoint.
     *
     * Returns a benefit supported by Gusto.
     *
     * The benefit object in Gusto contains high level information about a particular benefit type and its tax considerations. When companies choose to offer a benefit, they are creating a Company Benefit object associated with a particular benefit.
     */
    pub async fn get_v_1_benefits_benefit_id(&self) -> Result<crate::types::SupportedBenefit> {
        let url = format!(
            "/v1/benefits/{}",
            crate::progenitor_support::encode_path(&benefit_id.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Get benefits for a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/company_benefits` endpoint.
     *
     * Company benefits represent the benefits that a company is offering to employees. This ties together a particular supported benefit with the company-specific information for the offering of that benefit.
     *
     * Note that company benefits can be deactivated only when no employees are enrolled.
     */
    pub async fn get_v_1_companies_company_id_company_benefits(
        &self,
    ) -> Result<Vec<crate::types::CompanyBenefit>> {
        let url = format!(
            "/v1/companies/{}/company_benefits",
            crate::progenitor_support::encode_path(&company_id.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Get benefits for a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/company_benefits` endpoint.
     *
     * As opposed to `get_v_1_companies_company_id_company_benefits`, this function returns all the pages of the request at once.
     *
     * Company benefits represent the benefits that a company is offering to employees. This ties together a particular supported benefit with the company-specific information for the offering of that benefit.
     *
     * Note that company benefits can be deactivated only when no employees are enrolled.
     */
    pub async fn get_v_1_companies_company_id_company_benefits(
        &self,
    ) -> Result<Vec<crate::types::CompanyBenefit>> {
        let url = format!(
            "/v1/companies/{}/company_benefits",
            crate::progenitor_support::encode_path(&company_id.to_string()),
        );

        self.client.get_all_pages(&url).await
    }

    /**
     * Create a company benefit.
     *
     * This function performs a `POST` to the `/v1/companies/{company_id}/company_benefits` endpoint.
     *
     * Company benefits represent the benefits that a company is offering to employees. This ties together a particular supported benefit with the company-specific information for the offering of that benefit.
     *
     * Note that company benefits can be deactivated only when no employees are enrolled.
     */
    pub async fn post_v_1_companies_company_id_company_benefits(
        &self,
        body: &crate::types::PostV1CompaniesCompanyIdBenefitsRequest,
    ) -> Result<crate::types::CompanyBenefit> {
        let url = format!(
            "/v1/companies/{}/company_benefits",
            crate::progenitor_support::encode_path(&company_id.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Get a company benefit.
     *
     * This function performs a `GET` to the `/v1/company_benefits/{company_benefit_id}` endpoint.
     *
     * Company benefits represent the benefits that a company is offering to employees. This ties together a particular supported benefit with the company-specific information for the offering of that benefit.
     *
     * Note that company benefits can be deactivated only when no employees are enrolled.
     */
    pub async fn get_v_1_company_benefits_company_benefit_id(
        &self,
    ) -> Result<crate::types::CompanyBenefit> {
        let url = format!(
            "/v1/company_benefits/{}",
            crate::progenitor_support::encode_path(&company_benefit_id.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Update a company benefit.
     *
     * This function performs a `PUT` to the `/v1/company_benefits/{company_benefit_id}` endpoint.
     *
     * Company benefits represent the benefits that a company is offering to employees. This ties together a particular supported benefit with the company-specific information for the offering of that benefit.
     *
     * Note that company benefits can be deactivated only when no employees are enrolled.
     */
    pub async fn put_v_1_company_benefits_company_benefit_id(
        &self,
        body: &crate::types::PutV1CompanyBenefitsBenefitIdRequest,
    ) -> Result<crate::types::CompanyBenefit> {
        let url = format!(
            "/v1/company_benefits/{}",
            crate::progenitor_support::encode_path(&company_benefit_id.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Get an employee's benefits.
     *
     * This function performs a `GET` to the `/v1/employees/{employee_id}/employee_benefits` endpoint.
     *
     * Employee benefits represent an employee enrolled in a particular company benefit. It includes information specific to that employee’s enrollment.
     *
     * Returns an array of all employee benefits for this employee
     */
    pub async fn get_v_1_employees_employee_id_employee_benefits(
        &self,
    ) -> Result<Vec<crate::types::EmployeeBenefit>> {
        let url = format!(
            "/v1/employees/{}/employee_benefits",
            crate::progenitor_support::encode_path(&employee_id.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Get an employee's benefits.
     *
     * This function performs a `GET` to the `/v1/employees/{employee_id}/employee_benefits` endpoint.
     *
     * As opposed to `get_v_1_employees_employee_id_employee_benefits`, this function returns all the pages of the request at once.
     *
     * Employee benefits represent an employee enrolled in a particular company benefit. It includes information specific to that employee’s enrollment.
     *
     * Returns an array of all employee benefits for this employee
     */
    pub async fn get_v_1_employees_employee_id_employee_benefits(
        &self,
    ) -> Result<Vec<crate::types::EmployeeBenefit>> {
        let url = format!(
            "/v1/employees/{}/employee_benefits",
            crate::progenitor_support::encode_path(&employee_id.to_string()),
        );

        self.client.get_all_pages(&url).await
    }

    /**
     * Create an employee benefit.
     *
     * This function performs a `POST` to the `/v1/employees/{employee_id}/employee_benefits` endpoint.
     *
     * Employee benefits represent an employee enrolled in a particular company benefit. It includes information specific to that employee’s enrollment.
     */
    pub async fn post_v_1_employees_employee_id_employee_benefits(
        &self,
        body: &crate::types::PostV1EmployeesEmployeeIdBenefitsRequest,
    ) -> Result<crate::types::EmployeeBenefit> {
        let url = format!(
            "/v1/employees/{}/employee_benefits",
            crate::progenitor_support::encode_path(&employee_id.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Year-to-date Benefit Amounts from Different Company.
     *
     * This function performs a `POST` to the `/v1/employees/{employee_id}/ytd_benefit_amounts_from_different_company` endpoint.
     *
     * Year-to-date benefit amounts from a different company represents the amount of money added to an employees plan during a current year, made outside of the current contribution when they were employed at a different company.
     */
    pub async fn post_employee_ytd_benefit_amounts_from_different_company(
        &self,
        body: &crate::types::PostEmployeeYtdBenefitAmountsFromDifferentCompanyRequest,
    ) -> Result<()> {
        let url = format!(
            "/v1/employees/{}/ytd_benefit_amounts_from_different_company",
            crate::progenitor_support::encode_path(&employee_id.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Get an employee benefit.
     *
     * This function performs a `GET` to the `/v1/employee_benefits/{employee_benefit_id}` endpoint.
     *
     * Employee benefits represent an employee enrolled in a particular company benefit. It includes information specific to that employee’s enrollment.
     */
    pub async fn get_v_1_employee_benefits_employee_benefit_id(
        &self,
    ) -> Result<crate::types::EmployeeBenefit> {
        let url = format!(
            "/v1/employee_benefits/{}",
            crate::progenitor_support::encode_path(&employee_benefit_id.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Update an employee benefit.
     *
     * This function performs a `PUT` to the `/v1/employee_benefits/{employee_benefit_id}` endpoint.
     *
     * Employee benefits represent an employee enrolled in a particular company benefit. It includes information specific to that employee’s enrollment.
     */
    pub async fn put_v_1_employee_benefits_employee_benefit_id(
        &self,
        body: &crate::types::PutV1EmployeeBenefitsBenefitIdRequest,
    ) -> Result<crate::types::EmployeeBenefit> {
        let url = format!(
            "/v1/employee_benefits/{}",
            crate::progenitor_support::encode_path(&employee_benefit_id.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Delete an employee benefit.
     *
     * This function performs a `DELETE` to the `/v1/employee_benefits/{employee_benefit_id}` endpoint.
     *
     * Employee benefits represent an employee enrolled in a particular company benefit. It includes information specific to that employee’s enrollment.
     */
    pub async fn delete_v_1_employee_benefits_employee_benefit_id(&self) -> Result<()> {
        let url = format!(
            "/v1/employee_benefits/{}",
            crate::progenitor_support::encode_path(&employee_benefit_id.to_string()),
        );

        self.client.delete(&url, None).await
    }
}
