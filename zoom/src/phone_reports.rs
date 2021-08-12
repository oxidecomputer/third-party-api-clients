use anyhow::Result;

use crate::Client;

pub struct PhoneReports {
    client: Client,
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
    ) -> Result<crate::types::PaginationToken4ImChat> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !category_type.is_empty() {
            query_args.push(format!("category_type={}", category_type));
        }
        if !from.is_empty() {
            query_args.push(format!("from={}", from));
        }
        if !next_page_token.is_empty() {
            query_args.push(format!("next_page_token={}", next_page_token));
        }
        if page_size > 0 {
            query_args.push(format!("page_size={}", page_size));
        }
        if !to.is_empty() {
            query_args.push(format!("to={}", to));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/phone/reports/operationlogs?{}", query);

        self.client.get(&url, None).await
    }
}
