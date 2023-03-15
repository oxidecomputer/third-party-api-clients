use crate::Client;
use crate::ClientResult;

pub struct Compensations {
    pub client: Client,
}

impl Compensations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Compensations { client }
    }

    /**
     * Get a compensation.
     *
     * This function performs a `GET` to the `/v1/compensations/{compensation_id}` endpoint.
     *
     * Compensations contain information on how much is paid out for a job. Jobs may have many compensations, but only one that is active. The current compensation is the one with the most recent `effective_date`.
     *
     * Note: Currently, jobs are arbitrarily limited to a single compensation as multiple compensations per job are not yet available in Gusto. The API is architected as if multiple compensations may exist, so integrations should integrate under the same assumption. The only exception is that creating a compensation with the same `job_id` as another will fail with a relevant error.
     *
     */
    pub async fn get(&self, compensation_id: &str) -> ClientResult<crate::types::Compensation> {
        let url = self.client.url(
            &format!(
                "/v1/compensations/{}",
                crate::progenitor_support::encode_path(compensation_id),
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
     * Update a compensation.
     *
     * This function performs a `PUT` to the `/v1/compensations/{compensation_id}` endpoint.
     *
     * Compensations contain information on how much is paid out for a job. Jobs may have many compensations, but only one that is active. The current compensation is the one with the most recent `effective_date`.
     *
     * Note: Currently, jobs are arbitrarily limited to a single compensation as multiple compensations per job are not yet available in Gusto. The API is architected as if multiple compensations may exist, so integrations should integrate under the same assumption. The only exception is that creating a compensation with the same `job_id` as another will fail with a relevant error
     */
    pub async fn put(
        &self,
        compensation_id: &str,
        body: &crate::types::PutCompensationRequest,
    ) -> ClientResult<crate::types::Compensation> {
        let url = self.client.url(
            &format!(
                "/v1/compensations/{}",
                crate::progenitor_support::encode_path(compensation_id),
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
     * Get compensations for a job.
     *
     * This function performs a `GET` to the `/v1/jobs/{job_id}/compensations` endpoint.
     *
     * Compensations contain information on how much is paid out for a job. Jobs may have many compensations, but only one that is active. The current compensation is the one with the most recent `effective_date`.
     *
     * Note: Currently, jobs are arbitrarily limited to a single compensation as multiple compensations per job are not yet available in Gusto. The API is architected as if multiple compensations may exist, so integrations should integrate under the same assumption. The only exception is that creating a compensation with the same `job_id` as another will fail with a relevant error.
     *
     * Use the `flsa_status` to determine if an employee is elibgle for overtime.
     */
    pub async fn get_job(&self, job_id: &str) -> ClientResult<Vec<crate::types::Compensation>> {
        let url = self.client.url(
            &format!(
                "/v1/jobs/{}/compensations",
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
     * Get compensations for a job.
     *
     * This function performs a `GET` to the `/v1/jobs/{job_id}/compensations` endpoint.
     *
     * As opposed to `get_job`, this function returns all the pages of the request at once.
     *
     * Compensations contain information on how much is paid out for a job. Jobs may have many compensations, but only one that is active. The current compensation is the one with the most recent `effective_date`.
     *
     * Note: Currently, jobs are arbitrarily limited to a single compensation as multiple compensations per job are not yet available in Gusto. The API is architected as if multiple compensations may exist, so integrations should integrate under the same assumption. The only exception is that creating a compensation with the same `job_id` as another will fail with a relevant error.
     *
     * Use the `flsa_status` to determine if an employee is elibgle for overtime.
     */
    pub async fn get_all_job(&self, job_id: &str) -> ClientResult<Vec<crate::types::Compensation>> {
        let url = self.client.url(
            &format!(
                "/v1/jobs/{}/compensations",
                crate::progenitor_support::encode_path(job_id),
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
}
