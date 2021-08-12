use anyhow::Result;

use crate::Client;

pub struct Jobs {
    client: Client,
}

impl Jobs {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        Jobs {
            client,
        }
    }

    /**
* Get a job.
*
* This function performs a `GET` to the `/v1/jobs/{job_id}` endpoint.
*
* Get a job.
*/
pub async fn get_job(
&self,
job_id: &str,
) -> Result<crate::types::Job> {
let url =
format!("/v1/jobs/{}",
crate::progenitor_support::encode_path(&job_id.to_string()),);

self.client.get(&url, None).await
}

/**
* Update a job.
*
* This function performs a `PUT` to the `/v1/jobs/{job_id}` endpoint.
*
* Update a job.
*/
pub async fn put_job(
&self,
job_id: &str,
body: &crate::types::PutV1JobsJobIdRequest
) -> Result<crate::types::Job> {
let url =
format!("/v1/jobs/{}",
crate::progenitor_support::encode_path(&job_id.to_string()),);

self.client.put(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Delete an individual job.
*
* This function performs a `DELETE` to the `/v1/jobs/{job_id}` endpoint.
*
* Deletes a specific job that an employee holds.
*/
pub async fn delete_job(
&self,
job_id: &str,
) -> Result<()> {
let url =
format!("/v1/jobs/{}",
crate::progenitor_support::encode_path(&job_id.to_string()),);

self.client.delete(&url, None).await
}

/**
* Get jobs for an employee.
*
* This function performs a `GET` to the `/v1/employees/{employee_id}/jobs` endpoint.
*
* Get all of the jobs that an employee holds.
*/
pub async fn get_employee(
&self,
employee_id: &str,
) -> Result<Vec<crate::types::Job>> {
let url =
format!("/v1/employees/{}/jobs",
crate::progenitor_support::encode_path(&employee_id.to_string()),);

self.client.get(&url, None).await
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
) -> Result<Vec<crate::types::Job>> {
let url =
format!("/v1/employees/{}/jobs",
crate::progenitor_support::encode_path(&employee_id.to_string()),);

self.client.get_all_pages(&url, None).await
}

/**
* Create a job.
*
* This function performs a `POST` to the `/v1/employees/{employee_id}/jobs` endpoint.
*
* Create a job.
*/
pub async fn post_job(
&self,
employee_id: &str,
body: &crate::types::PostV1JobsJobIdRequest
) -> Result<crate::types::Job> {
let url =
format!("/v1/employees/{}/jobs",
crate::progenitor_support::encode_path(&employee_id.to_string()),);

self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
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
pub async fn post_job_compensation(
&self,
job_id: &str,
body: &crate::types::PostV1JobsJobIdCompensationsRequest
) -> Result<crate::types::Compensation> {
let url =
format!("/v1/jobs/{}/compensations",
crate::progenitor_support::encode_path(&job_id.to_string()),);

self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}


}