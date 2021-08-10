use anyhow::Result;

use crate::Client;

pub struct Compensations {
    client: Client,
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
    pub async fn get_v_1_compensations_compensation_id(
        &self,
    ) -> Result<crate::types::Compensation> {
        let url = format!(
            "/v1/compensations/{}",
            crate::progenitor_support::encode_path(&compensation_id.to_string()),
        );

        self.client.get(&url).await
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
    pub async fn put_v_1_compensations_compensation_id(
        &self,
        body: &crate::types::PutV1CompensationsCompensationIdRequest,
    ) -> Result<crate::types::Compensation> {
        let url = format!(
            "/v1/compensations/{}",
            crate::progenitor_support::encode_path(&compensation_id.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
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
    pub async fn get_v_1_jobs_job_id_compensations(
        &self,
    ) -> Result<Vec<crate::types::Compensation>> {
        let url = format!(
            "/v1/jobs/{}/compensations",
            crate::progenitor_support::encode_path(&job_id.to_string()),
        );

        self.client.get(&url).await
    }

    /**
     * Get compensations for a job.
     *
     * This function performs a `GET` to the `/v1/jobs/{job_id}/compensations` endpoint.
     *
     * As opposed to `get_v_1_jobs_job_id_compensations`, this function returns all the pages of the request at once.
     *
     * Compensations contain information on how much is paid out for a job. Jobs may have many compensations, but only one that is active. The current compensation is the one with the most recent `effective_date`.
     *
     * Note: Currently, jobs are arbitrarily limited to a single compensation as multiple compensations per job are not yet available in Gusto. The API is architected as if multiple compensations may exist, so integrations should integrate under the same assumption. The only exception is that creating a compensation with the same `job_id` as another will fail with a relevant error.
     *
     * Use the `flsa_status` to determine if an employee is elibgle for overtime.
     */
    pub async fn get_v_1_jobs_job_id_compensations(
        &self,
    ) -> Result<Vec<crate::types::Compensation>> {
        let url = format!(
            "/v1/jobs/{}/compensations",
            crate::progenitor_support::encode_path(&job_id.to_string()),
        );

        self.client.get_all_pages(&url).await
    }
}
