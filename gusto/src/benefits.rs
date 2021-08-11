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
    pub async fn get_benefits(&self) -> Result<Vec<crate::types::SupportedBenefit>> {
        let url = "/v1/benefits".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Get all benefits supported by Gusto.
     *
     * This function performs a `GET` to the `/v1/benefits` endpoint.
     *
     * As opposed to `get_benefits`, this function returns all the pages of the request at once.
     *
     * Returns all benefits supported by Gusto.
     *
     * The benefit object in Gusto contains high level information about a particular benefit type and its tax considerations. When companies choose to offer a benefit, they are creating a Company Benefit object associated with a particular benefit.
     */
    pub async fn get_all_benefits(&self) -> Result<Vec<crate::types::SupportedBenefit>> {
        let url = "/v1/benefits".to_string();
        self.client.get_all_pages(&url, None).await
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
    pub async fn get_benefits_benefit_id(
        &self,
        benefit_id: &str,
    ) -> Result<crate::types::SupportedBenefit> {
        let url = format!(
            "/v1/benefits/{}",
            crate::progenitor_support::encode_path(&benefit_id.to_string()),
        );

        self.client.get(&url, None).await
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
    pub async fn get_company_company_benefits(
        &self,
        company_id: &str,
    ) -> Result<Vec<crate::types::CompanyBenefit>> {
        let url = format!(
            "/v1/companies/{}/company_benefits",
            crate::progenitor_support::encode_path(&company_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Get benefits for a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/company_benefits` endpoint.
     *
     * As opposed to `get_company_company_benefits`, this function returns all the pages of the request at once.
     *
     * Company benefits represent the benefits that a company is offering to employees. This ties together a particular supported benefit with the company-specific information for the offering of that benefit.
     *
     * Note that company benefits can be deactivated only when no employees are enrolled.
     */
    pub async fn get_all_company_company_benefits(
        &self,
        company_id: &str,
    ) -> Result<Vec<crate::types::CompanyBenefit>> {
        let url = format!(
            "/v1/companies/{}/company_benefits",
            crate::progenitor_support::encode_path(&company_id.to_string()),
        );

        self.client.get_all_pages(&url, None).await
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
    pub async fn post_company_company_benefits(
        &self,
        company_id: &str,
        body: &crate::types::PostCompanyBenefitsRequest,
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
    pub async fn get_company_benefits_company_benefit_id(
        &self,
        company_benefit_id: &str,
    ) -> Result<crate::types::CompanyBenefit> {
        let url = format!(
            "/v1/company_benefits/{}",
            crate::progenitor_support::encode_path(&company_benefit_id.to_string()),
        );

        self.client.get(&url, None).await
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
    pub async fn put_company_benefits_company_benefit_id(
        &self,
        company_benefit_id: &str,
        body: &crate::types::PutCompanyBenefitsBenefitIdRequest,
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
    pub async fn get_employees_employee_id_employee_benefits(
        &self,
        employee_id: &str,
    ) -> Result<Vec<crate::types::EmployeeBenefit>> {
        let url = format!(
            "/v1/employees/{}/employee_benefits",
            crate::progenitor_support::encode_path(&employee_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Get an employee's benefits.
     *
     * This function performs a `GET` to the `/v1/employees/{employee_id}/employee_benefits` endpoint.
     *
     * As opposed to `get_employees_employee_id_employee_benefits`, this function returns all the pages of the request at once.
     *
     * Employee benefits represent an employee enrolled in a particular company benefit. It includes information specific to that employee’s enrollment.
     *
     * Returns an array of all employee benefits for this employee
     */
    pub async fn get_all_employees_employee_id_employee_benefits(
        &self,
        employee_id: &str,
    ) -> Result<Vec<crate::types::EmployeeBenefit>> {
        let url = format!(
            "/v1/employees/{}/employee_benefits",
            crate::progenitor_support::encode_path(&employee_id.to_string()),
        );

        self.client.get_all_pages(&url, None).await
    }

    /**
     * Create an employee benefit.
     *
     * This function performs a `POST` to the `/v1/employees/{employee_id}/employee_benefits` endpoint.
     *
     * Employee benefits represent an employee enrolled in a particular company benefit. It includes information specific to that employee’s enrollment.
     */
    pub async fn post_employees_employee_id_employee_benefits(
        &self,
        employee_id: &str,
        body: &crate::types::PostEmployeesEmployeeIdBenefitsRequest,
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
        employee_id: &str,
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
    pub async fn get_employee_benefits_employee_benefit_id(
        &self,
        employee_benefit_id: &str,
    ) -> Result<crate::types::EmployeeBenefit> {
        let url = format!(
            "/v1/employee_benefits/{}",
            crate::progenitor_support::encode_path(&employee_benefit_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update an employee benefit.
     *
     * This function performs a `PUT` to the `/v1/employee_benefits/{employee_benefit_id}` endpoint.
     *
     * Employee benefits represent an employee enrolled in a particular company benefit. It includes information specific to that employee’s enrollment.
     */
    pub async fn put_employee_benefits_employee_benefit_id(
        &self,
        employee_benefit_id: &str,
        body: &crate::types::PutEmployeeBenefitsBenefitIdRequest,
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
    pub async fn delete_employee_benefits_employee_benefit_id(
        &self,
        employee_benefit_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/v1/employee_benefits/{}",
            crate::progenitor_support::encode_path(&employee_benefit_id.to_string()),
        );

        self.client.delete(&url, None).await
    }
}
