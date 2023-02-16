use anyhow::Result;

use crate::Client;

pub struct Garnishments {
    pub client: Client,
}

impl Garnishments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Garnishments { client }
    }

    /**
     * Get garnishments for an employee.
     *
     * This function performs a `GET` to the `/v1/employees/{employee_id}/garnishments` endpoint.
     *
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    pub async fn get_employee(&self, employee_id: &str) -> Result<Vec<crate::types::Garnishment>> {
        let url = format!(
            "/v1/employees/{}/garnishments",
            crate::progenitor_support::encode_path(employee_id),
        );
        let url = self.client.url(&url, None);
        self.client.get(&url, None).await
    }
    /**
     * Get garnishments for an employee.
     *
     * This function performs a `GET` to the `/v1/employees/{employee_id}/garnishments` endpoint.
     *
     * As opposed to `get_employee`, this function returns all the pages of the request at once.
     *
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    pub async fn get_all_employee(
        &self,
        employee_id: &str,
    ) -> Result<Vec<crate::types::Garnishment>> {
        let url = format!(
            "/v1/employees/{}/garnishments",
            crate::progenitor_support::encode_path(employee_id),
        );
        self.client.get_all_pages(&url, None).await
    }
    /**
     * Create a garnishment.
     *
     * This function performs a `POST` to the `/v1/employees/{employee_id}/garnishments` endpoint.
     *
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    pub async fn post_employee(
        &self,
        employee_id: &str,
        body: &crate::types::PostEmployeeGarnishmentsRequest,
    ) -> Result<crate::types::Garnishment> {
        let url = format!(
            "/v1/employees/{}/garnishments",
            crate::progenitor_support::encode_path(employee_id),
        );
        let url = self.client.url(&url, None);
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
    /**
     * Get a garnishment.
     *
     * This function performs a `GET` to the `/v1/garnishments/{garnishment_id}` endpoint.
     *
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    pub async fn get(&self, garnishment_id: &str) -> Result<crate::types::Garnishment> {
        let url = format!(
            "/v1/garnishments/{}",
            crate::progenitor_support::encode_path(garnishment_id),
        );
        let url = self.client.url(&url, None);
        self.client.get(&url, None).await
    }
    /**
     * Update a garnishment.
     *
     * This function performs a `PUT` to the `/v1/garnishments/{garnishment_id}` endpoint.
     *
     * Garnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.
     */
    pub async fn put(
        &self,
        garnishment_id: &str,
        body: &crate::types::PutGarnishmentRequest,
    ) -> Result<crate::types::Garnishment> {
        let url = format!(
            "/v1/garnishments/{}",
            crate::progenitor_support::encode_path(garnishment_id),
        );
        let url = self.client.url(&url, None);
        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
