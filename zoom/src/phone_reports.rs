use anyhow::Result;

use crate::Client;

pub struct PhoneReports {
    pub client: Client,
}

impl PhoneReports {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        PhoneReports { client }
    }

    /**
     * Get operation logs report.
     *
     * This function performs a `GET` to the `/phone/reports/operationlogs` endpoint.
     *
     * The **Phone System operation logs report** allows account owners and admins to view monthly Zoom phone related admin operation details.
     *
     * Use this API to retrieve the **Phone System Operation Logs Report**. Account owners and admins can also access this information by logging into their Zoom accounts and navigating to [Phone System Operation Logs](https://zoom.us/pbx/page/report/operations#/report/operation-logs).<br><br> **Scopes:** `phone:read:admin`, `phone:write:admin` <br> **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`<br>
     * <br> **Prerequisites:** <br>
     * * Account must be enrollled in Pro or a higher plan
     * * Account must be enrolled in a [Zoom Phone](https://zoom.us/pricing/zoom-phone) plan
     *
     *
     *
     * **Parameters:**
     *
     * * `from: &str` -- Start date in 'yyyy-mm-dd' format. The date range defined by the "from" and "to" parameters should only be one month as the report provides only one month worth of data per API request.
     * * `to: &str` -- End date in 'yyyy-mm-dd' format.
     * * `category_type: &str` -- Filter the response by the category of the action performed. By default, the value of this field is "all" and thus, the response will include log of all operations for the defined period.<br><br>To only include response for a specific category type, provide a value for `category_type` from this [table](http://marketplace.zoom.us/docs/phone-operation-categories ).
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn get_ps_operation_log(
        &self,
        from: &str,
        to: &str,
        category_type: &str,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<crate::types::ReportOperationLogsResponseAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !category_type.is_empty() {
            query_args.push(("category_type".to_string(), category_type.to_string()));
        }
        if !from.is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !to.is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/reports/operationlogs?{}", query_);

        self.client.get(&url, None).await
    }
}
