use crate::Client;
use crate::ClientResult;

pub struct Reporting {
    pub client: Client,
}

impl Reporting {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Reporting { client }
    }

    /**
     * This function performs a `GET` to the `/v1/reporting/report_runs` endpoint.
     *
     * <p>Returns a list of Report Runs, with the most recent appearing first.</p>
     *
     * **Parameters:**
     *
     * * `created: &str`
     * * `ending_before: &str` -- A cursor for use in pagination. `ending_before` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `limit: i64` -- A limit on the number of objects to be returned. Limit can range between 1 and 100, and the default is 10.
     * * `starting_after: &str` -- A cursor for use in pagination. `starting_after` is an object ID that defines your place in the list. For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
     */
    pub async fn get_report_runs(
        &self,
        _created: &str,
        ending_before: &str,
        limit: i64,
        starting_after: &str,
    ) -> ClientResult<Vec<crate::types::ReportingReportRun>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ending_before.is_empty() {
            query_args.push(("ending_before".to_string(), ending_before.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !starting_after.is_empty() {
            query_args.push(("starting_after".to_string(), starting_after.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/v1/reporting/report_runs?{}", query_), None);
        let resp: crate::types::GetReportingReportRunsResponse = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await?;

        // Return our response data.
        Ok(resp.data.to_vec())
    }
    /**
     * This function performs a `GET` to the `/v1/reporting/report_runs` endpoint.
     *
     * As opposed to `get_report_runs`, this function returns all the pages of the request at once.
     *
     * <p>Returns a list of Report Runs, with the most recent appearing first.</p>
     */
    pub async fn get_all_report_runs(
        &self,
        _created: &str,
    ) -> ClientResult<Vec<crate::types::ReportingReportRun>> {
        let url = self.client.url("/v1/reporting/report_runs", None);
        let mut resp: crate::types::GetReportingReportRunsResponse = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut data = resp.data;
        let mut has_more = resp.has_more;
        let mut page = "".to_string();

        // Paginate if we should.
        while has_more {
            if !data.is_empty() {
                let last = data.last().unwrap();
                let j = serde_json::json!(last);
                if let serde_json::Value::Object(o) = j {
                    if let Some(serde_json::Value::String(s)) = o.get("id") {
                        page = s.to_string();
                    }
                }
            }

            if !url.contains('?') {
                resp = self
                    .client
                    .get(
                        &format!("{}?startng_after={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                resp = self
                    .client
                    .get(
                        &format!("{}&starting_after={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            data.append(&mut resp.data);

            has_more = resp.has_more;
        }

        // Return our response data.
        Ok(data.to_vec())
    }
    /**
     * This function performs a `POST` to the `/v1/reporting/report_runs` endpoint.
     *
     * <p>Creates a new object and begin running the report. (Certain report types require a <a href="https://stripe.com/docs/keys#test-live-modes">live-mode API key</a>.)</p>
     */
    pub async fn post_report_run(&self) -> ClientResult<crate::types::ReportingReportRun> {
        let url = self.client.url("/v1/reporting/report_runs", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/v1/reporting/report_runs/{report_run}` endpoint.
     *
     * <p>Retrieves the details of an existing Report Run.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `report_run: &str` -- The account's country.
     */
    pub async fn get_report_runs_run(
        &self,
        report_run: &str,
    ) -> ClientResult<crate::types::ReportingReportRun> {
        let url = self.client.url(
            &format!(
                "/v1/reporting/report_runs/{}",
                crate::progenitor_support::encode_path(report_run),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/v1/reporting/report_types` endpoint.
     *
     * <p>Returns a full list of Report Types.</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     */
    pub async fn get_report_types(&self) -> ClientResult<Vec<crate::types::ReportingReportType>> {
        let url = self.client.url("/v1/reporting/report_types", None);
        let resp: crate::types::FinancialReportingFinanceReportTypeList = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await?;

        // Return our response data.
        Ok(resp.data.to_vec())
    }
    /**
     * This function performs a `GET` to the `/v1/reporting/report_types` endpoint.
     *
     * As opposed to `get_report_types`, this function returns all the pages of the request at once.
     *
     * <p>Returns a full list of Report Types.</p>
     */
    pub async fn get_all_report_types(
        &self,
    ) -> ClientResult<Vec<crate::types::ReportingReportType>> {
        let url = self.client.url("/v1/reporting/report_types", None);
        let mut resp: crate::types::FinancialReportingFinanceReportTypeList = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut data = resp.data;
        let mut has_more = resp.has_more;
        let mut page = "".to_string();

        // Paginate if we should.
        while has_more {
            if !data.is_empty() {
                let last = data.last().unwrap();
                let j = serde_json::json!(last);
                if let serde_json::Value::Object(o) = j {
                    if let Some(serde_json::Value::String(s)) = o.get("id") {
                        page = s.to_string();
                    }
                }
            }

            if !url.contains('?') {
                resp = self
                    .client
                    .get(
                        &format!("{}?startng_after={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                resp = self
                    .client
                    .get(
                        &format!("{}&starting_after={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            data.append(&mut resp.data);

            has_more = resp.has_more;
        }

        // Return our response data.
        Ok(data.to_vec())
    }
    /**
     * This function performs a `GET` to the `/v1/reporting/report_types/{report_type}` endpoint.
     *
     * <p>Retrieves the details of a Report Type. (Certain report types require a <a href="https://stripe.com/docs/keys#test-live-modes">live-mode API key</a>.)</p>
     *
     * **Parameters:**
     *
     * * `expand: &[String]` -- Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
     * * `report_type: &str` -- The account's country.
     */
    pub async fn get_report_types_type(
        &self,
        report_type: &str,
    ) -> ClientResult<crate::types::ReportingReportType> {
        let url = self.client.url(
            &format!(
                "/v1/reporting/report_types/{}",
                crate::progenitor_support::encode_path(report_type),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
}
