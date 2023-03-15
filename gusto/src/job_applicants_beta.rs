use crate::Client;
use crate::ClientResult;

pub struct JobApplicantsBeta {
    pub client: Client,
}

impl JobApplicantsBeta {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        JobApplicantsBeta { client }
    }

    /**
     * Get all job applicants for a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/job_applicants` endpoint.
     *
     * *This endpoint is in beta - we will be making breaking changes based on developer feedback.
     *
     * Returns all job applicants for a company.
     */
    pub async fn get_company_job_applicants(
        &self,
        company_id: &str,
    ) -> ClientResult<Vec<crate::types::JobApplicant>> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/job_applicants",
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
     * Get all job applicants for a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/job_applicants` endpoint.
     *
     * As opposed to `get_company_job_applicants`, this function returns all the pages of the request at once.
     *
     * *This endpoint is in beta - we will be making breaking changes based on developer feedback.
     *
     * Returns all job applicants for a company.
     */
    pub async fn get_all_company_job_applicants(
        &self,
        company_id: &str,
    ) -> ClientResult<Vec<crate::types::JobApplicant>> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/job_applicants",
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
     * Create a job applicant.
     *
     * This function performs a `POST` to the `/v1/companies/{company_id}/job_applicants` endpoint.
     *
     * *This endpoint is in beta - we will be making breaking changes based on developer feedback.
     *
     * Create a job applicant.
     */
    pub async fn post_company_job_applicant(
        &self,
        company_id: &str,
        body: &crate::types::PostCompanyJobApplicantsRequest,
    ) -> ClientResult<crate::types::JobApplicant> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/job_applicants",
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
     * Get a job applicant.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/job_applicants/{job_applicant_uuid}` endpoint.
     *
     * *This endpoint is in beta - we will be making breaking changes based on developer feedback.
     *
     * Returns a single job applicant.
     */
    pub async fn get_company_job_applicants_applicant(
        &self,
        company_id: &str,
        job_applicant_uuid: &str,
    ) -> ClientResult<crate::types::JobApplicant> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/job_applicants/{}",
                crate::progenitor_support::encode_path(company_id),
                crate::progenitor_support::encode_path(job_applicant_uuid),
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
     * Update a job applicant.
     *
     * This function performs a `PUT` to the `/v1/companies/{company_id}/job_applicants/{job_applicant_uuid}` endpoint.
     *
     * *This endpoint is in beta - we will be making breaking changes based on developer feedback.
     *
     * Update an existing job applicant (only allowed when the job applicant has not been imported).
     */
    pub async fn put_company_job_applicants_applicant(
        &self,
        company_id: &str,
        job_applicant_uuid: &str,
        body: &crate::types::PutCompanyJobApplicantRequest,
    ) -> ClientResult<crate::types::JobApplicant> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/job_applicants/{}",
                crate::progenitor_support::encode_path(company_id),
                crate::progenitor_support::encode_path(job_applicant_uuid),
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
     * Delete a job applicant.
     *
     * This function performs a `DELETE` to the `/v1/companies/{company_id}/job_applicants/{job_applicant_uuid}` endpoint.
     *
     * *This endpoint is in beta - we will be making breaking changes based on developer feedback.
     *
     * Permanently remove a job applicant by uuid (only allowed when the job applicant has not been imported).
     */
    pub async fn delete_company_job_applicants_applicant(
        &self,
        company_id: &str,
        job_applicant_uuid: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/job_applicants/{}",
                crate::progenitor_support::encode_path(company_id),
                crate::progenitor_support::encode_path(job_applicant_uuid),
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
