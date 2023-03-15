use crate::Client;
use crate::ClientResult;

pub struct Benefits {
    pub client: Client,
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
    pub async fn get_page(&self) -> ClientResult<Vec<crate::types::SupportedBenefit>> {
        let url = self.client.url("/v1/benefits", None);
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
     * Get all benefits supported by Gusto.
     *
     * This function performs a `GET` to the `/v1/benefits` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * Returns all benefits supported by Gusto.
     *
     * The benefit object in Gusto contains high level information about a particular benefit type and its tax considerations. When companies choose to offer a benefit, they are creating a Company Benefit object associated with a particular benefit.
     */
    pub async fn get_all(&self) -> ClientResult<Vec<crate::types::SupportedBenefit>> {
        let url = self.client.url("/v1/benefits", None);
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
     * Get a supported benefit by ID.
     *
     * This function performs a `GET` to the `/v1/benefits/{benefit_id}` endpoint.
     *
     * Returns a benefit supported by Gusto.
     *
     * The benefit object in Gusto contains high level information about a particular benefit type and its tax considerations. When companies choose to offer a benefit, they are creating a Company Benefit object associated with a particular benefit.
     */
    pub async fn get(&self, benefit_id: &str) -> ClientResult<crate::types::SupportedBenefit> {
        let url = self.client.url(
            &format!(
                "/v1/benefits/{}",
                crate::progenitor_support::encode_path(benefit_id),
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
     * Get benefits for a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/company_benefits` endpoint.
     *
     * Company benefits represent the benefits that a company is offering to employees. This ties together a particular supported benefit with the company-specific information for the offering of that benefit.
     *
     * Note that company benefits can be deactivated only when no employees are enrolled.
     */
    pub async fn get_company(
        &self,
        company_id: &str,
    ) -> ClientResult<Vec<crate::types::CompanyBenefit>> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/company_benefits",
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
     * Get benefits for a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/company_benefits` endpoint.
     *
     * As opposed to `get_company`, this function returns all the pages of the request at once.
     *
     * Company benefits represent the benefits that a company is offering to employees. This ties together a particular supported benefit with the company-specific information for the offering of that benefit.
     *
     * Note that company benefits can be deactivated only when no employees are enrolled.
     */
    pub async fn get_all_company(
        &self,
        company_id: &str,
    ) -> ClientResult<Vec<crate::types::CompanyBenefit>> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/company_benefits",
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
     * Create a company benefit.
     *
     * This function performs a `POST` to the `/v1/companies/{company_id}/company_benefits` endpoint.
     *
     * Company benefits represent the benefits that a company is offering to employees. This ties together a particular supported benefit with the company-specific information for the offering of that benefit.
     *
     * Note that company benefits can be deactivated only when no employees are enrolled.
     */
    pub async fn post_company(
        &self,
        company_id: &str,
        body: &crate::types::PostCompanyBenefitsRequest,
    ) -> ClientResult<crate::types::CompanyBenefit> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/company_benefits",
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
     * Get a company benefit.
     *
     * This function performs a `GET` to the `/v1/company_benefits/{company_benefit_id}` endpoint.
     *
     * Company benefits represent the benefits that a company is offering to employees. This ties together a particular supported benefit with the company-specific information for the offering of that benefit.
     *
     * Note that company benefits can be deactivated only when no employees are enrolled.
     */
    pub async fn get_company_benefits(
        &self,
        company_benefit_id: &str,
    ) -> ClientResult<crate::types::CompanyBenefit> {
        let url = self.client.url(
            &format!(
                "/v1/company_benefits/{}",
                crate::progenitor_support::encode_path(company_benefit_id),
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
     * Update a company benefit.
     *
     * This function performs a `PUT` to the `/v1/company_benefits/{company_benefit_id}` endpoint.
     *
     * Company benefits represent the benefits that a company is offering to employees. This ties together a particular supported benefit with the company-specific information for the offering of that benefit.
     *
     * Note that company benefits can be deactivated only when no employees are enrolled.
     */
    pub async fn put_company(
        &self,
        company_benefit_id: &str,
        body: &crate::types::PutCompanyBenefitRequest,
    ) -> ClientResult<crate::types::CompanyBenefit> {
        let url = self.client.url(
            &format!(
                "/v1/company_benefits/{}",
                crate::progenitor_support::encode_path(company_benefit_id),
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
     * Get an employee's benefits.
     *
     * This function performs a `GET` to the `/v1/employees/{employee_id}/employee_benefits` endpoint.
     *
     * Employee benefits represent an employee enrolled in a particular company benefit. It includes information specific to that employee’s enrollment.
     *
     * Returns an array of all employee benefits for this employee
     */
    pub async fn get_employee(
        &self,
        employee_id: &str,
    ) -> ClientResult<Vec<crate::types::EmployeeBenefit>> {
        let url = self.client.url(
            &format!(
                "/v1/employees/{}/employee_benefits",
                crate::progenitor_support::encode_path(employee_id),
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
     * Get an employee's benefits.
     *
     * This function performs a `GET` to the `/v1/employees/{employee_id}/employee_benefits` endpoint.
     *
     * As opposed to `get_employee`, this function returns all the pages of the request at once.
     *
     * Employee benefits represent an employee enrolled in a particular company benefit. It includes information specific to that employee’s enrollment.
     *
     * Returns an array of all employee benefits for this employee
     */
    pub async fn get_all_employee(
        &self,
        employee_id: &str,
    ) -> ClientResult<Vec<crate::types::EmployeeBenefit>> {
        let url = self.client.url(
            &format!(
                "/v1/employees/{}/employee_benefits",
                crate::progenitor_support::encode_path(employee_id),
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
     * Create an employee benefit.
     *
     * This function performs a `POST` to the `/v1/employees/{employee_id}/employee_benefits` endpoint.
     *
     * Employee benefits represent an employee enrolled in a particular company benefit. It includes information specific to that employee’s enrollment.
     */
    pub async fn post_employee(
        &self,
        employee_id: &str,
        body: &crate::types::PostEmployeeBenefitsRequest,
    ) -> ClientResult<crate::types::EmployeeBenefit> {
        let url = self.client.url(
            &format!(
                "/v1/employees/{}/employee_benefits",
                crate::progenitor_support::encode_path(employee_id),
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
     * Year-to-date Benefit Amounts from Different Company.
     *
     * This function performs a `POST` to the `/v1/employees/{employee_id}/ytd_benefit_amounts_from_different_company` endpoint.
     *
     * Year-to-date benefit amounts from a different company represents the amount of money added to an employees plan during a current year, made outside of the current contribution when they were employed at a different company.
     */
    pub async fn post_employee_ytd_amounts_from_different_company(
        &self,
        employee_id: &str,
        body: &crate::types::PostEmployeeYtdBenefitAmountsFromDifferentCompanyRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v1/employees/{}/ytd_benefit_amounts_from_different_company",
                crate::progenitor_support::encode_path(employee_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
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
    pub async fn get_employee_benefits(
        &self,
        employee_benefit_id: &str,
    ) -> ClientResult<crate::types::EmployeeBenefit> {
        let url = self.client.url(
            &format!(
                "/v1/employee_benefits/{}",
                crate::progenitor_support::encode_path(employee_benefit_id),
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
     * Update an employee benefit.
     *
     * This function performs a `PUT` to the `/v1/employee_benefits/{employee_benefit_id}` endpoint.
     *
     * Employee benefits represent an employee enrolled in a particular company benefit. It includes information specific to that employee’s enrollment.
     */
    pub async fn put_employee(
        &self,
        employee_benefit_id: &str,
        body: &crate::types::PutEmployeeBenefitRequest,
    ) -> ClientResult<crate::types::EmployeeBenefit> {
        let url = self.client.url(
            &format!(
                "/v1/employee_benefits/{}",
                crate::progenitor_support::encode_path(employee_benefit_id),
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
     * Delete an employee benefit.
     *
     * This function performs a `DELETE` to the `/v1/employee_benefits/{employee_benefit_id}` endpoint.
     *
     * Employee benefits represent an employee enrolled in a particular company benefit. It includes information specific to that employee’s enrollment.
     */
    pub async fn delete_employee(&self, employee_benefit_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v1/employee_benefits/{}",
                crate::progenitor_support::encode_path(employee_benefit_id),
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
