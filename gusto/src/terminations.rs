use crate::Client;
use crate::ClientResult;

pub struct Terminations {
    pub client: Client,
}

impl Terminations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Terminations { client }
    }

    /**
     * Get terminations for an employee.
     *
     * This function performs a `GET` to the `/v1/employees/{employee_id}/terminations` endpoint.
     *
     * Terminations are created whenever an employee is scheduled to leave the company. The only things required are an effective date (their last day of work) and whether they should receive their wages in a one-off termination payroll or with the rest of the company.
     *
     * Note that some states require employees to receive their final wages within 24 hours (unless they consent otherwise,) in which case running a one-off payroll may be the only option.
     */
    pub async fn get_employee(
        &self,
        employee_id: &str,
    ) -> ClientResult<Vec<crate::types::Termination>> {
        let url = self.client.url(
            &format!(
                "/v1/employees/{}/terminations",
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
     * Get terminations for an employee.
     *
     * This function performs a `GET` to the `/v1/employees/{employee_id}/terminations` endpoint.
     *
     * As opposed to `get_employee`, this function returns all the pages of the request at once.
     *
     * Terminations are created whenever an employee is scheduled to leave the company. The only things required are an effective date (their last day of work) and whether they should receive their wages in a one-off termination payroll or with the rest of the company.
     *
     * Note that some states require employees to receive their final wages within 24 hours (unless they consent otherwise,) in which case running a one-off payroll may be the only option.
     */
    pub async fn get_all_employee(
        &self,
        employee_id: &str,
    ) -> ClientResult<Vec<crate::types::Termination>> {
        let url = self.client.url(
            &format!(
                "/v1/employees/{}/terminations",
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
     * Create an employee termination.
     *
     * This function performs a `POST` to the `/v1/employees/{employee_id}/terminations` endpoint.
     *
     * Terminations are created whenever an employee is scheduled to leave the company. The only things required are an effective date (their last day of work) and whether they should receive their wages in a one-off termination payroll or with the rest of the company.
     *
     * Note that some states require employees to receive their final wages within 24 hours (unless they consent otherwise,) in which case running a one-off payroll may be the only option.
     */
    pub async fn post_employee(
        &self,
        employee_id: &str,
        body: &crate::types::PostEmployeeTerminationsRequest,
    ) -> ClientResult<crate::types::Termination> {
        let url = self.client.url(
            &format!(
                "/v1/employees/{}/terminations",
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
}
