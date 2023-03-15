use crate::Client;
use crate::ClientResult;

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
    ) -> ClientResult<crate::types::Employee> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include.is_empty() {
            query_args.push(("include".to_string(), include.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v1/employees/{}?{}",
                crate::progenitor_support::encode_path(employee_id_or_uuid),
                query_
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
    ) -> ClientResult<crate::types::Employee> {
        let url = self.client.url(
            &format!(
                "/v1/employees/{}",
                crate::progenitor_support::encode_path(employee_id_or_uuid),
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
    ) -> ClientResult<Vec<crate::types::Employee>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include.is_empty() {
            query_args.push(("include".to_string(), include.join(" ")));
        }
        if !page.to_string().is_empty() {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if !per.to_string().is_empty() {
            query_args.push(("per".to_string(), per.to_string()));
        }
        if terminated {
            query_args.push(("terminated".to_string(), terminated.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/employees?{}",
                crate::progenitor_support::encode_path(company_id_or_uuid),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
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
    ) -> ClientResult<Vec<crate::types::Employee>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include.is_empty() {
            query_args.push(("include".to_string(), include.join(" ")));
        }
        if terminated {
            query_args.push(("terminated".to_string(), terminated.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/employees?{}",
                crate::progenitor_support::encode_path(company_id_or_uuid),
                query_
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
     * Create an employee.
     *
     * This function performs a `POST` to the `/v1/companies/{company_id_or_uuid}/employees` endpoint.
     *
     * Create an employee.
     */
    pub async fn post(
        &self,
        company_id_or_uuid: &str,
        body: &crate::types::PostEmployeesRequest,
    ) -> ClientResult<crate::types::Employee> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/employees",
                crate::progenitor_support::encode_path(company_id_or_uuid),
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
     * Get an employee's home address.
     *
     * This function performs a `GET` to the `/v1/employees/{employee_id}/home_address` endpoint.
     *
     * The home address of an employee is used to determine certain tax information about them. Addresses are geocoded on create and update to ensure validity.
     */
    pub async fn get_home_address(
        &self,
        employee_id: &str,
    ) -> ClientResult<crate::types::Location> {
        let url = self.client.url(
            &format!(
                "/v1/employees/{}/home_address",
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
     * Update an employee's home address.
     *
     * This function performs a `PUT` to the `/v1/employees/{employee_id}/home_address` endpoint.
     *
     * The home address of an employee is used to determine certain tax information about them. Addresses are geocoded on create and update to ensure validity.
     */
    pub async fn put_home_address(
        &self,
        employee_id: &str,
        body: &crate::types::PutEmployeeHomeAddressRequest,
    ) -> ClientResult<crate::types::Location> {
        let url = self.client.url(
            &format!(
                "/v1/employees/{}/home_address",
                crate::progenitor_support::encode_path(employee_id),
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
}
