use anyhow::Result;

use crate::Client;

pub struct TimeOffRequests {
    client: Client,
}

impl TimeOffRequests {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        TimeOffRequests {
            client,
        }
    }

    /**
* Get time off requests for a company.
*
* This function performs a `GET` to the `/v1/companies/{company_id}/time_off_requests` endpoint.
*
* Get all time off requests, past and present, for a company.
* 
* In order to reduce the number of time off requests returned in a single response, or to retrieve time off requests from a time period of interest, you may use the `start_date` and `end_date` parameters.
* 
* You may provide both or either parameters to scope the returned data. For example:
* 
* `?start_date='2019-01-01'`
* 
* Returns all time off requests where the request start date is equal to or after January 1, 2019.
* 
* `?end_date='2019-01-01'`
* 
* Returns all time off requests where the request end date is equal to or before January 1, 2019.
* 
* `?start_date='2019-05-01'&end_date='2019-08-31'`
* 
* Returns all time off requests where the request start date is equal to or after May 1, 2019 and the request end date is equal to or before August 31, 2019.
* 
*
* **Parameters:**
*
* * `start_date: &str` -- Filter time off requests where the request start date is equal to or after this parameter.
* * `end_date: &str` -- Filter time off requests where the request end date is equal to or after this parameter.
*/
pub async fn get_v_1_companies_company_id_time_off_requests(
&self,
start_date: &str, end_date: &str,
) -> Result<Vec<crate::types::TimeOffRequest>> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
if !end_date.is_empty() { query_args.push(format!("end_date={}", end_date)); }
if !start_date.is_empty() { query_args.push(format!("start_date={}", start_date)); }
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/v1/companies/{}/time_off_requests?{}",
crate::progenitor_support::encode_path(&company_id.to_string()),query);

self.client.get(&url).await
}

/**
* Get time off requests for a company.
*
* This function performs a `GET` to the `/v1/companies/{company_id}/time_off_requests` endpoint.
*
* As opposed to `get_v_1_companies_company_id_time_off_requests`, this function returns all the pages of the request at once.
*
* Get all time off requests, past and present, for a company.
* 
* In order to reduce the number of time off requests returned in a single response, or to retrieve time off requests from a time period of interest, you may use the `start_date` and `end_date` parameters.
* 
* You may provide both or either parameters to scope the returned data. For example:
* 
* `?start_date='2019-01-01'`
* 
* Returns all time off requests where the request start date is equal to or after January 1, 2019.
* 
* `?end_date='2019-01-01'`
* 
* Returns all time off requests where the request end date is equal to or before January 1, 2019.
* 
* `?start_date='2019-05-01'&end_date='2019-08-31'`
* 
* Returns all time off requests where the request start date is equal to or after May 1, 2019 and the request end date is equal to or before August 31, 2019.
* 
*/
pub async fn get_v_1_companies_company_id_time_off_requests(
&self,
start_date: &str, end_date: &str,
) -> Result<Vec<crate::types::TimeOffRequest>> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
if !end_date.is_empty() { query_args.push(format!("end_date={}", end_date)); }
if !start_date.is_empty() { query_args.push(format!("start_date={}", start_date)); }
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/v1/companies/{}/time_off_requests?{}",
crate::progenitor_support::encode_path(&company_id.to_string()),query);

self.client.get_all_pages(&url).await
}

/**
* Get a specific time off request.
*
* This function performs a `GET` to the `/v1/companies/{company_id}/time_off_requests/{time_off_request_id}` endpoint.
*
* Details of a single time off request
*/
pub async fn get_v_1_companies_company_id_time_off_requests_time_off_request_id(
&self,
) -> Result<crate::types::TimeOffRequest> {
let url =
format!("/v1/companies/{}/time_off_requests/{}",
crate::progenitor_support::encode_path(&company_id.to_string()),crate::progenitor_support::encode_path(&time_off_request_id.to_string()),);

self.client.get(&url).await
}


}