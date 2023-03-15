use crate::Client;
use crate::ClientResult;

pub struct TimeOffRequests {
    pub client: Client,
}

impl TimeOffRequests {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TimeOffRequests { client }
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
    pub async fn get_company(
        &self,
        company_id: &str,
        start_date: &str,
        end_date: &str,
    ) -> ClientResult<Vec<crate::types::TimeOffRequest>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !end_date.is_empty() {
            query_args.push(("end_date".to_string(), end_date.to_string()));
        }
        if !start_date.is_empty() {
            query_args.push(("start_date".to_string(), start_date.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/time_off_requests?{}",
                crate::progenitor_support::encode_path(company_id),
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
     * Get time off requests for a company.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/time_off_requests` endpoint.
     *
     * As opposed to `get_company`, this function returns all the pages of the request at once.
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
    pub async fn get_all_company(
        &self,
        company_id: &str,
        start_date: &str,
        end_date: &str,
    ) -> ClientResult<Vec<crate::types::TimeOffRequest>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !end_date.is_empty() {
            query_args.push(("end_date".to_string(), end_date.to_string()));
        }
        if !start_date.is_empty() {
            query_args.push(("start_date".to_string(), start_date.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/time_off_requests?{}",
                crate::progenitor_support::encode_path(company_id),
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
     * Get a specific time off request.
     *
     * This function performs a `GET` to the `/v1/companies/{company_id}/time_off_requests/{time_off_request_id}` endpoint.
     *
     * Details of a single time off request
     */
    pub async fn get_company_request(
        &self,
        company_id: &str,
        time_off_request_id: &str,
    ) -> ClientResult<crate::types::TimeOffRequest> {
        let url = self.client.url(
            &format!(
                "/v1/companies/{}/time_off_requests/{}",
                crate::progenitor_support::encode_path(company_id),
                crate::progenitor_support::encode_path(time_off_request_id),
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
}
