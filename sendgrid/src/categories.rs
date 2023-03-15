use crate::Client;
use crate::ClientResult;

pub struct Categories {
    pub client: Client,
}

impl Categories {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Categories { client }
    }

    /**
     * Retrieve all categories.
     *
     * This function performs a `GET` to the `/categories` endpoint.
     *
     * **This endpoint allows you to retrieve a list of all of your categories.**
     *
     * **Parameters:**
     *
     * * `limit: i64` -- The number of categories to display per page.
     * * `category: &str` -- Allows you to perform a prefix search on this particular category.
     * * `offset: i64` -- The point in the list that you would like to begin displaying results.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_page(
        &self,
        limit: i64,
        category: &str,
        offset: i64,
    ) -> ClientResult<Vec<crate::types::GetCategoriesResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !category.is_empty() {
            query_args.push(("category".to_string(), category.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/categories?{}", query_), None);
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
     * Retrieve all categories.
     *
     * This function performs a `GET` to the `/categories` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve a list of all of your categories.**
     */
    pub async fn get_all(
        &self,
        category: &str,
        offset: i64,
    ) -> ClientResult<Vec<crate::types::GetCategoriesResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !category.is_empty() {
            query_args.push(("category".to_string(), category.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/categories?{}", query_), None);
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
     * Retrieve sums of email stats for each category [Needs: Stats object defined, has category ID?].
     *
     * This function performs a `GET` to the `/categories/stats/sums` endpoint.
     *
     * **This endpoint allows you to retrieve the total sum of each email statistic for every category over the given date range.**
     *
     * If you do not define any query parameters, this endpoint will return a sum for each category in groups of 10.
     *
     * **Parameters:**
     *
     * * `sort_by_metric: &str` -- The metric that you want to sort by.  Must be a single metric.
     * * `sort_by_direction: crate::types::SortByDirection` -- The direction you want to sort.
     * * `start_date: &str` -- The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD.
     * * `end_date: &str` -- The end date of the statistics to retrieve. Defaults to today. Must follow format YYYY-MM-DD.
     * * `limit: i64` -- Limits the number of results returned.
     * * `offset: i64` -- The point in the list to begin retrieving results.
     * * `aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy` -- How to group the statistics. Must be either "day", "week", or "month".
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_stats_sum(
        &self,
        sort_by_metric: &str,
        sort_by_direction: crate::types::SortByDirection,
        start_date: &str,
        end_date: &str,
        limit: i64,
        offset: i64,
        aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy,
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
            .url(&format!("/categories/stats/sums?{}", query_), None);
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
     * Retrieve Email Statistics for Categories.
     *
     * This function performs a `GET` to the `/categories/stats` endpoint.
     *
     * **This endpoint allows you to retrieve all of your email statistics for each of your categories.**
     *
     * If you do not define any query parameters, this endpoint will return a sum for each category in groups of 10.
     *
     * **Parameters:**
     *
     * * `start_date: &str` -- The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD.
     * * `end_date: &str` -- The end date of the statistics to retrieve. Defaults to today. Must follow format YYYY-MM-DD.
     * * `categories: &str` -- The individual categories that you want to retrieve statistics for. You may include up to 10 different categories.
     * * `limit: i64` -- The number of results to include.
     * * `offset: i64` -- The number of results to skip.
     * * `aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy` -- How to group the statistics. Must be either "day", "week", or "month".
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_stats(
        &self,
        start_date: &str,
        end_date: &str,
        categories: &str,
        limit: i64,
        offset: i64,
        aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy,
    ) -> ClientResult<Vec<crate::types::CategoryStats>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.to_string().is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
        }
        if !categories.is_empty() {
            query_args.push(("categories".to_string(), categories.to_string()));
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
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/categories/stats?{}", query_), None);
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
     * Retrieve Email Statistics for Categories.
     *
     * This function performs a `GET` to the `/categories/stats` endpoint.
     *
     * As opposed to `get_stats`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve all of your email statistics for each of your categories.**
     *
     * If you do not define any query parameters, this endpoint will return a sum for each category in groups of 10.
     */
    pub async fn get_all_stats(
        &self,
        start_date: &str,
        end_date: &str,
        categories: &str,
        offset: i64,
        aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy,
    ) -> ClientResult<Vec<crate::types::CategoryStats>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.to_string().is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
        }
        if !categories.is_empty() {
            query_args.push(("categories".to_string(), categories.to_string()));
        }
        if !end_date.is_empty() {
            query_args.push(("end_date".to_string(), end_date.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !start_date.is_empty() {
            query_args.push(("start_date".to_string(), start_date.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/categories/stats?{}", query_), None);
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
