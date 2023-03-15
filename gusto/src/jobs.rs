use crate::Client;
use crate::ClientResult;

pub struct Jobs {
    pub client: Client,
}

impl Jobs {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Jobs { client }
    }

    /**
     * Get a job.
     *
     * This function performs a `GET` to the `/v1/jobs/{job_id}` endpoint.
     *
     * Get a job.
     */
    pub async fn get(&self, job_id: &str) -> ClientResult<crate::types::Job> {
        let url = self.client.url(
            &format!(
                "/v1/jobs/{}",
                crate::progenitor_support::encode_path(job_id),
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
     * Update a job.
     *
     * This function performs a `PUT` to the `/v1/jobs/{job_id}` endpoint.
     *
     * Update a job.
     */
    pub async fn put(
        &self,
        job_id: &str,
        body: &crate::types::PutJobRequest,
    ) -> ClientResult<crate::types::Job> {
        let url = self.client.url(
            &format!(
                "/v1/jobs/{}",
                crate::progenitor_support::encode_path(job_id),
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
     * Delete an individual job.
     *
     * This function performs a `DELETE` to the `/v1/jobs/{job_id}` endpoint.
     *
     * Deletes a specific job that an employee holds.
     */
    pub async fn delete(&self, job_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v1/jobs/{}",
                crate::progenitor_support::encode_path(job_id),
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
    /**
     * Get jobs for an employee.
     *
     * This function performs a `GET` to the `/v1/employees/{employee_id}/jobs` endpoint.
     *
     * Get all of the jobs that an employee holds.
     */
    pub async fn get_employee(&self, employee_id: &str) -> ClientResult<Vec<crate::types::Job>> {
        let url = self.client.url(
            &format!(
                "/v1/employees/{}/jobs",
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
     * Get jobs for an employee.
     *
     * This function performs a `GET` to the `/v1/employees/{employee_id}/jobs` endpoint.
     *
     * As opposed to `get_employee`, this function returns all the pages of the request at once.
     *
     * Get all of the jobs that an employee holds.
     */
    pub async fn get_all_employee(
        &self,
        employee_id: &str,
    ) -> ClientResult<Vec<crate::types::Job>> {
        let url = self.client.url(
            &format!(
                "/v1/employees/{}/jobs",
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
     * Create a job.
     *
     * This function performs a `POST` to the `/v1/employees/{employee_id}/jobs` endpoint.
     *
     * Create a job.
     */
    pub async fn post(
        &self,
        employee_id: &str,
        body: &crate::types::PostJobRequest,
    ) -> ClientResult<crate::types::Job> {
        let url = self.client.url(
            &format!(
                "/v1/employees/{}/jobs",
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
     * Create a compensation.
     *
     * This function performs a `POST` to the `/v1/jobs/{job_id}/compensations` endpoint.
     *
     * Compensations contain information on how much is paid out for a job. Jobs may have many compensations, but only one that is active. The current compensation is the one with the most recent `effective_date`.
     *
     * Note: Currently, jobs are arbitrarily limited to a single compensation as multiple compensations per job are not yet available in Gusto. The API is architected as if multiple compensations may exist, so integrations should integrate under the same assumption. The only exception is that creating a compensation with the same `job_id` as another will fail with a relevant error
     */
    pub async fn post_compensation(
        &self,
        job_id: &str,
        body: &crate::types::PostJobCompensationsRequest,
    ) -> ClientResult<crate::types::Compensation> {
        let url = self.client.url(
            &format!(
                "/v1/jobs/{}/compensations",
                crate::progenitor_support::encode_path(job_id),
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
