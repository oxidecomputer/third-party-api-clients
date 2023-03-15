use crate::Client;
use crate::ClientResult;

pub struct SubuserStatistics {
    pub client: Client,
}

impl SubuserStatistics {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SubuserStatistics { client }
    }

    /**
     * Retrieve the monthly email statistics for a single subuser.
     *
     * This function performs a `GET` to the `/subusers/{subuser_name}/stats/monthly` endpoint.
     *
     * **This endpoint allows you to retrive the monthly email statistics for a specific subuser.**
     *
     * When using the `sort_by_metric` to sort your stats by a specific metric, you can not sort by the following metrics:
     * `bounce_drops`, `deferred`, `invalid_emails`, `processed`, `spam_report_drops`, `spam_reports`, or `unsubscribe_drops`.
     *
     * **Parameters:**
     *
     * * `date: &str` -- The date of the month to retrieve statistics for. Must be formatted YYYY-MM-DD.
     * * `sort_by_metric: &str` -- The metric that you want to sort by. Metrics that you can sort by are: `blocks`, `bounces`, `clicks`, `delivered`, `opens`, `requests`, `unique_clicks`, `unique_opens`, and `unsubscribes`.'.
     * * `sort_by_direction: crate::types::SortByDirection` -- The direction you want to sort.
     * * `limit: i64` -- Optional field to limit the number of results returned.
     * * `offset: i64` -- Optional beginning point in the list to retrieve from.
     */
    pub async fn get_subusers_subuser_name_stats_monthly(
        &self,
        subuser_name: &str,
        date: &str,
        sort_by_metric: &str,
        sort_by_direction: crate::types::SortByDirection,
        limit: i64,
        offset: i64,
    ) -> ClientResult<crate::types::SubuserStatsData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !date.is_empty() {
            query_args.push(("date".to_string(), date.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !sort_by_direction.to_string().is_empty() {
            query_args.push((
                "sort_by_direction".to_string(),
                sort_by_direction.to_string(),
            ));
        }
        if !sort_by_metric.is_empty() {
            query_args.push(("sort_by_metric".to_string(), sort_by_metric.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/subusers/{}/stats/monthly?{}",
                crate::progenitor_support::encode_path(subuser_name),
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
     * Retrieve monthly stats for all subusers.
     *
     * This function performs a `GET` to the `/subusers/stats/monthly` endpoint.
     *
     * **This endpoint allows you to retrieve the monthly email statistics for all subusers over the given date range.**
     *
     * When using the `sort_by_metric` to sort your stats by a specific metric, you can not sort by the following metrics:
     * `bounce_drops`, `deferred`, `invalid_emails`, `processed`, `spam_report_drops`, `spam_reports`, or `unsubscribe_drops`.
     *
     * **Parameters:**
     *
     * * `date: &str` -- The date of the month to retrieve statistics for. Must be formatted YYYY-MM-DD.
     * * `subuser: &str` -- The license key provided with your New Relic account.
     * * `sort_by_metric: crate::types::SortByMetric` -- The metric that you want to sort by. Metrics that you can sort by are: `blocks`, `bounces`, `clicks`, `delivered`, `opens`, `requests`, `unique_clicks`, `unique_opens`, and `unsubscribes`.'.
     * * `sort_by_direction: crate::types::SortByDirection` -- The direction you want to sort.
     * * `limit: i64` -- Optional field to limit the number of results returned.
     * * `offset: i64` -- Optional beginning point in the list to retrieve from.
     */
    pub async fn get_subusers_stats_monthly(
        &self,
        date: &str,
        subuser: &str,
        sort_by_metric: crate::types::SortByMetric,
        sort_by_direction: crate::types::SortByDirection,
        limit: i64,
        offset: i64,
    ) -> ClientResult<crate::types::SubuserStatsData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !date.is_empty() {
            query_args.push(("date".to_string(), date.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !sort_by_direction.to_string().is_empty() {
            query_args.push((
                "sort_by_direction".to_string(),
                sort_by_direction.to_string(),
            ));
        }
        if !sort_by_metric.to_string().is_empty() {
            query_args.push(("sort_by_metric".to_string(), sort_by_metric.to_string()));
        }
        if !subuser.is_empty() {
            query_args.push(("subuser".to_string(), subuser.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/subusers/stats/monthly?{}", query_), None);
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
     * Retrieve the totals for each email statistic metric for all subusers.
     *
     * This function performs a `GET` to the `/subusers/stats/sums` endpoint.
     *
     * **This endpoint allows you to retrieve the total sums of each email statistic metric for all subusers over the given date range.**
     *
     * **Parameters:**
     *
     * * `sort_by_direction: crate::types::SortByDirection` -- The direction you want to sort.
     * * `start_date: &str` -- The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD.
     * * `end_date: &str` -- The end date of the statistics to retrieve. Defaults to today. Must follow format YYYY-MM-DD.
     * * `limit: i64` -- Limits the number of results returned per page.
     * * `offset: i64` -- The point in the list to begin retrieving results from.
     * * `aggregated_by: &str` -- How to group the statistics. Defaults to today. Must follow format YYYY-MM-DD.
     * * `sort_by_metric: &str` -- The metric that you want to sort by.  Must be a single metric.
     */
    pub async fn get_subusers_stats_sum(
        &self,
        sort_by_direction: crate::types::SortByDirection,
        start_date: &str,
        end_date: &str,
        limit: i64,
        offset: i64,
        aggregated_by: &str,
        sort_by_metric: &str,
    ) -> ClientResult<crate::types::CategoryStats> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
        }
        if !end_date.is_empty() {
            query_args.push(("end_date".to_string(), end_date.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !sort_by_direction.to_string().is_empty() {
            query_args.push((
                "sort_by_direction".to_string(),
                sort_by_direction.to_string(),
            ));
        }
        if !sort_by_metric.is_empty() {
            query_args.push(("sort_by_metric".to_string(), sort_by_metric.to_string()));
        }
        if !start_date.is_empty() {
            query_args.push(("start_date".to_string(), start_date.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/subusers/stats/sums?{}", query_), None);
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
     * Retrieve email statistics for your subusers.
     *
     * This function performs a `GET` to the `/subusers/stats` endpoint.
     *
     * **This endpoint allows you to retrieve the email statistics for the given subusers.**
     *
     * You may retrieve statistics for up to 10 different subusers by including an additional _subusers_ parameter for each additional subuser.
     *
     * **Parameters:**
     *
     * * `limit: i64` -- Limits the number of results returned per page.
     * * `offset: i64` -- The point in the list to begin retrieving results from.
     * * `aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy` -- How to group the statistics. Must be either "day", "week", or "month".
     * * `subusers: &str` -- The subuser you want to retrieve statistics for. You may include this parameter up to 10 times to retrieve statistics for multiple subusers.
     * * `start_date: &str` -- The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD.
     * * `end_date: &str` -- The end date of the statistics to retrieve. Defaults to today.
     */
    pub async fn get_subusers_stat(
        &self,
        limit: i64,
        offset: i64,
        aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy,
        subusers: &str,
        start_date: &str,
        end_date: &str,
    ) -> ClientResult<crate::types::CategoryStats> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.to_string().is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
        }
        if !end_date.is_empty() {
            query_args.push(("end_date".to_string(), end_date.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !start_date.is_empty() {
            query_args.push(("start_date".to_string(), start_date.to_string()));
        }
        if !subusers.is_empty() {
            query_args.push(("subusers".to_string(), subusers.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/subusers/stats?{}", query_), None);
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
