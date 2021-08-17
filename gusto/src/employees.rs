use anyhow::Result;

use crate::Client;

pub struct Employees {
    pub client: Client,
}

impl Employees {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Employees { client }
    }

    /**
     * Get an employee.
     *
     * This function performs a `GET` to the `/v1/employees/{employee_id_or_uuid}` endpoint.
     *
     * Get an employee.
     *
     * **Parameters:**
     *
     * * `include: &[String]` -- Include the requested attribute(s) in each employee response.
     */
    pub async fn get(
        &self,
        employee_id_or_uuid: &str,
        include: &[String],
    ) -> Result<crate::types::Employee> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !include.is_empty() {
            query_args.push(format!("include={}", include.join(" ")));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/v1/employees/{}?{}",
            crate::progenitor_support::encode_path(&employee_id_or_uuid.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Update an employee.
     *
     * This function performs a `PUT` to the `/v1/employees/{employee_id_or_uuid}` endpoint.
     *
     * Update an employee.
     */
    pub async fn put(
        &self,
        employee_id_or_uuid: &str,
        body: &crate::types::PutEmployeesRequest,
    ) -> Result<crate::types::Employee> {
        let url = format!(
            "/v1/employees/{}",
            crate::progenitor_support::encode_path(&employee_id_or_uuid.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Get employees of a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id_or_uuid}/employees` endpoint.
     *
     * Get all of the employees, onboarding, active and terminated, for a given company.
     *
     * **Parameters:**
     *
     * * `terminated: bool` -- Filters employees by the provided boolean.
     * * `page: f64` -- The page that is requested. When unspecified, will load all employees.
     * * `per: f64` -- Number of employees per page. When unspecified, will default to 25.
     * * `include: &[String]` -- Include the requested attribute(s) in each employee response.
     */
    pub async fn get_company(
        &self,
        company_id_or_uuid: &str,
        terminated: bool,
        page: f64,
        per: f64,
        include: &[String],
    ) -> Result<Vec<crate::types::Employee>> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !include.is_empty() {
            query_args.push(format!("include={}", include.join(" ")));
        }
        if !page.to_string().is_empty() {
            query_args.push(format!("page={}", page.to_string()));
        }
        if !per.to_string().is_empty() {
            query_args.push(format!("per={}", per.to_string()));
        }
        if terminated {
            query_args.push(format!("terminated={}", terminated));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/v1/companies/{}/employees?{}",
            crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get employees of a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id_or_uuid}/employees` endpoint.
     *
     * As opposed to `get_company`, this function returns all the pages of the request at once.
     *
     * Get all of the employees, onboarding, active and terminated, for a given company.
     */
    pub async fn get_all_company(
        &self,
        company_id_or_uuid: &str,
        terminated: bool,
        include: &[String],
    ) -> Result<Vec<crate::types::Employee>> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !include.is_empty() {
            query_args.push(format!("include={}", include.join(" ")));
        }
        if terminated {
            query_args.push(format!("terminated={}", terminated));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/v1/companies/{}/employees?{}",
            crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),
            query_
        );

        self.client.get_all_pages(&url, None).await
    }

    /**
     * Create an employee.
     *
     * This function performs a `POST` to the `/v1/companies/{company_id_or_uuid}/employees` endpoint.
     *
     * Create an employee.
     */
    pub async fn post_employee(
        &self,
        company_id_or_uuid: &str,
        body: &crate::types::PostEmployeesRequest,
    ) -> Result<crate::types::Employee> {
        let url = format!(
            "/v1/companies/{}/employees",
            crate::progenitor_support::encode_path(&company_id_or_uuid.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Get an employee's home address.
     *
     * This function performs a `GET` to the `/v1/employees/{employee_id}/home_address` endpoint.
     *
     * The home address of an employee is used to determine certain tax information about them. Addresses are geocoded on create and update to ensure validity.
     */
    pub async fn get_employee_home_addre(
        &self,
        employee_id: &str,
    ) -> Result<crate::types::Location> {
        let url = format!(
            "/v1/employees/{}/home_address",
            crate::progenitor_support::encode_path(&employee_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update an employee's home address.
     *
     * This function performs a `PUT` to the `/v1/employees/{employee_id}/home_address` endpoint.
     *
     * The home address of an employee is used to determine certain tax information about them. Addresses are geocoded on create and update to ensure validity.
     */
    pub async fn put_employee_home_address(
        &self,
        employee_id: &str,
        body: &crate::types::PutEmployeeHomeAddressRequest,
    ) -> Result<crate::types::Location> {
        let url = format!(
            "/v1/employees/{}/home_address",
            crate::progenitor_support::encode_path(&employee_id.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
