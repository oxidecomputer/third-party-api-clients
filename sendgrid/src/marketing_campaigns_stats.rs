use crate::Client;
use crate::ClientResult;

pub struct MarketingCampaignsStats {
    pub client: Client,
}

impl MarketingCampaignsStats {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        MarketingCampaignsStats { client }
    }

    /**
     * Get All Automation Stats.
     *
     * This function performs a `GET` to the `/marketing/stats/automations` endpoint.
     *
     * **This endpoint allows you to retrieve stats for all your Automations.**
     *
     * By default, all of your Automations will be returned, but you can specify a selection by passing in a comma-separated list of Automation IDs as the value of the query string parameter `automation_ids`.
     *
     * Responses are paginated. You can limit the number of responses returned per batch using the `page_size` query string parameter. The default is 50, but you specify a value between 1 and 100.
     *
     * You can retrieve a specific page of responses with the `page_token` query string parameter.
     *
     * **Parameters:**
     *
     * * `automation_ids: &[String]` -- This endpoint returns all automation IDs if no `automation_ids` are specified.
     * * `page_size: i64` -- The number of elements you want returned on each page.
     * * `page_token: &str` -- The stats endpoints are paginated. To get the next page, call the passed `_metadata.next` URL. If `_metadata.prev` doesn't exist, you're at the first page. Similarly, if `_metadata.next` is not present, you're at the last page.
     */
    pub async fn getall_automation_stats(
        &self,
        automation_ids: &[String],
        page_size: i64,
        page_token: &str,
    ) -> ClientResult<crate::types::AutomationsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !automation_ids.is_empty() {
            query_args.push(("automation_ids".to_string(), automation_ids.join(" ")));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/marketing/stats/automations?{}", query_), None);
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
     * Get Automation Stats by ID.
     *
     * This function performs a `GET` to the `/marketing/stats/automations/{id}` endpoint.
     *
     * **This endpoint allows you to retrieve stats for a single Automation using its ID.**
     *
     * Multiple Automation IDs can be retrieved using the "Get All Automation Stats" endpoint. Once you have an ID, this endpoint will return detailed stats for the single automation specified.
     *
     * You may constrain the stats returned using the `start_date` and `end_date` query string parameters. You can also use the `group_by` and `aggregated_by` query string parameters to further refine the stats returned.
     *
     * **Parameters:**
     *
     * * `group_by: &[String]` -- Automations can have multiple steps. Including `step_id` as a `group_by` metric allows further granularity of stats.
     * * `step_ids: &[String]` -- The recipient IDs of the recipients that already existed from this request.
     * * `aggregated_by: crate::types::AggregatedBy` -- Dictates how the stats are time-sliced. Currently, `"total"` and `"day"` are supported.
     * * `start_date: chrono::NaiveDate` -- Format: `YYYY-MM-DD`. If this parameter is included, the stats' start date is included in the search.
     * * `end_date: chrono::NaiveDate` -- Format: `YYYY-MM-DD`.If this parameter is included, the stats' end date is included in the search.
     * * `timezone: &str` -- [IANA Area/Region](https://en.wikipedia.org/wiki/Tz_database#Names_of_time_zones) string representing the timezone in which the stats are to be presented, e.g., "America/Chicago".
     * * `page_size: i64` -- The number of elements you want returned on each page.
     * * `page_token: &str` -- The stats endpoints are paginated. To get the next page, call the passed `_metadata.next` URL. If `_metadata.prev` doesn't exist, you're at the first page. Similarly, if `_metadata.next` is not present, you're at the last page.
     */
    pub async fn get_automation_stat(
        &self,
        id: &str,
        group_by: &[String],
        step_ids: &[String],
        aggregated_by: crate::types::AggregatedBy,
        start_date: chrono::NaiveDate,
        end_date: chrono::NaiveDate,
        timezone: &str,
        page_size: i64,
        page_token: &str,
    ) -> ClientResult<crate::types::AutomationsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.to_string().is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
        }
        if !end_date.to_string().is_empty() {
            query_args.push(("end_date".to_string(), end_date.to_string()));
        }
        if !group_by.is_empty() {
            query_args.push(("group_by".to_string(), group_by.join(" ")));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        if !start_date.to_string().is_empty() {
            query_args.push(("start_date".to_string(), start_date.to_string()));
        }
        if !step_ids.is_empty() {
            query_args.push(("step_ids".to_string(), step_ids.join(" ")));
        }
        if !timezone.is_empty() {
            query_args.push(("timezone".to_string(), timezone.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/marketing/stats/automations/{}?{}",
                crate::progenitor_support::encode_path(id),
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
     * Get All Single Sends Stats.
     *
     * This function performs a `GET` to the `/marketing/stats/singlesends` endpoint.
     *
     * **This endpoint allows you to retrieve stats for all your Single Sends.**
     *
     * By default, all of your Single Sends will be returned, but you can specify a selection by passing in a comma-separated list of Single Send IDs as the value of the query string parameter `singlesend_ids`.
     *
     * Responses are paginated. You can limit the number of responses returned per batch using the `page_size` query string parameter. The default is 50, but you specify a value between 1 and 100.
     *
     * You can retrieve a specific page of responses with the `page_token` query string parameter.
     *
     * **Parameters:**
     *
     * * `singlesend_ids: &[String]` -- This endpoint returns all Single Send IDs if no IDs are included in `singlesend_ids`.
     * * `page_size: i64` -- The number of elements you want returned on each page.
     * * `page_token: &str` -- The stats endpoints are paginated. To get the next page, call the passed `_metadata.next` URL. If `_metadata.prev` doesn't exist, you're at the first page. Similarly, if `_metadata.next` is not present, you're at the last page.
     */
    pub async fn getall_singlesend_stats(
        &self,
        singlesend_ids: &[String],
        page_size: i64,
        page_token: &str,
    ) -> ClientResult<crate::types::SinglesendsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        if !singlesend_ids.is_empty() {
            query_args.push(("singlesend_ids".to_string(), singlesend_ids.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/marketing/stats/singlesends?{}", query_), None);
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
     * Get Single Send Stats by ID.
     *
     * This function performs a `GET` to the `/marketing/stats/singlesends/{id}` endpoint.
     *
     * **This endpoint allows you to retrieve stats for an individual Single Send using a Single Send ID.**
     *
     * Multiple Single Send IDs can be retrieved using the "Get All Single Sends Stats" endpoint. Once you have an ID, this endpoint will return detailed stats for the Single Send specified.
     *
     * You may constrain the stats returned using the `start_date` and `end_date` query string parameters. You can also use the `group_by` and `aggregated_by` query string parameters to further refine the stats returned.
     *
     * **Parameters:**
     *
     * * `aggregated_by: crate::types::AggregatedBy` -- Dictates how the stats are time-sliced. Currently, `"total"` and `"day"` are supported.
     * * `start_date: chrono::NaiveDate` -- Format: `YYYY-MM-DD`. If this parameter is included, the stats' start date is included in the search.
     * * `end_date: chrono::NaiveDate` -- Format: `YYYY-MM-DD`.If this parameter is included, the stats' end date is included in the search.
     * * `timezone: &str` -- [IANA Area/Region](https://en.wikipedia.org/wiki/Tz_database#Names_of_time_zones) string representing the timezone in which the stats are to be presented, e.g., "America/Chicago".
     * * `page_size: i64` -- The number of elements you want returned on each page.
     * * `page_token: &str` -- The stats endpoints are paginated. To get the next page, call the passed `_metadata.next` URL. If `_metadata.prev` doesn't exist, you're at the first page. Similarly, if `_metadata.next` is not present, you're at the last page.
     * * `group_by: &[String]` -- A/B Single Sends have multiple variation IDs and phase IDs. Including these additional fields allows further granularity of stats by these fields.
     */
    pub async fn get_singlesend_stat(
        &self,
        id: &str,
        aggregated_by: crate::types::AggregatedBy,
        start_date: chrono::NaiveDate,
        end_date: chrono::NaiveDate,
        timezone: &str,
        page_size: i64,
        page_token: &str,
        group_by: &[String],
    ) -> ClientResult<crate::types::SinglesendsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.to_string().is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
        }
        if !end_date.to_string().is_empty() {
            query_args.push(("end_date".to_string(), end_date.to_string()));
        }
        if !group_by.is_empty() {
            query_args.push(("group_by".to_string(), group_by.join(" ")));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        if !start_date.to_string().is_empty() {
            query_args.push(("start_date".to_string(), start_date.to_string()));
        }
        if !timezone.is_empty() {
            query_args.push(("timezone".to_string(), timezone.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/marketing/stats/singlesends/{}?{}",
                crate::progenitor_support::encode_path(id),
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
     * Get Automation Click Tracking Stats by ID.
     *
     * This function performs a `GET` to the `/marketing/stats/automations/{id}/links` endpoint.
     *
     * **This endpoint lets you retrieve click-tracking stats for a single Automation**.
     *
     * The stats returned list the URLs embedded in your Automation and the number of clicks each one received.
     *
     * Responses are paginated. You can limit the number of responses returned per batch using the `page_size` query string parameter. The default is 50, but you specify a value between 1 and 100.
     *
     * You can retrieve a specific page of responses with the `page_token` query string parameter.
     *
     * **Parameters:**
     *
     * * `group_by: &[String]` -- Automations can have multiple steps. Including `step_id` as a `group_by` metric allows further granularity of stats.
     * * `step_ids: &[String]` -- The recipient IDs of the recipients that already existed from this request.
     * * `page_size: i64` -- The number of elements you want returned on each page.
     * * `page_token: &str` -- The stats endpoints are paginated. To get the next page, call the passed `_metadata.next` URL. If `_metadata.prev` doesn't exist, you're at the first page. Similarly, if `_metadata.next` is not present, you're at the last page.
     */
    pub async fn get_automation_link_stat(
        &self,
        id: &str,
        group_by: &[String],
        step_ids: &[String],
        page_size: i64,
        page_token: &str,
    ) -> ClientResult<crate::types::AutomationsLinkStatsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !group_by.is_empty() {
            query_args.push(("group_by".to_string(), group_by.join(" ")));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        if !step_ids.is_empty() {
            query_args.push(("step_ids".to_string(), step_ids.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/marketing/stats/automations/{}/links?{}",
                crate::progenitor_support::encode_path(id),
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
     * Get Single Send Click Tracking Stats by ID.
     *
     * This function performs a `GET` to the `/marketing/stats/singlesends/{id}/links` endpoint.
     *
     * **This endpoint lets you retrieve click-tracking stats for one Single Send**.
     *
     * The stats returned list the URLs embedded in the specified Single Send and the number of clicks each one received.
     *
     * Responses are paginated. You can limit the number of responses returned per batch using the `page_size` query string parameter. The default is 50, but you specify a value between 1 and 100.
     *
     * You can retrieve a specific page of responses with the `page_token` query string parameter.
     *
     * **Parameters:**
     *
     * * `page_size: i64` -- The number of elements you want returned on each page.
     * * `page_token: &str` -- The stats endpoints are paginated. To get the next page, call the passed `_metadata.next` URL. If `_metadata.prev` doesn't exist, you're at the first page. Similarly, if `_metadata.next` is not present, you're at the last page.
     * * `group_by: &[String]` -- A/B Single Sends have multiple variation IDs and phase IDs. Including these additional fields allows further granularity of stats by these fields.
     * * `ab_variation_id: &str` -- The license key provided with your New Relic account.
     * * `ab_phase_id: crate::types::AbPhaseId`
     */
    pub async fn get_singlesend_link_stat(
        &self,
        id: &str,
        page_size: i64,
        page_token: &str,
        group_by: &[String],
        ab_variation_id: &str,
        ab_phase_id: crate::types::AbPhaseId,
    ) -> ClientResult<crate::types::SinglesendsLinkStatsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ab_phase_id.to_string().is_empty() {
            query_args.push(("ab_phase_id".to_string(), ab_phase_id.to_string()));
        }
        if !ab_variation_id.is_empty() {
            query_args.push(("ab_variation_id".to_string(), ab_variation_id.to_string()));
        }
        if !group_by.is_empty() {
            query_args.push(("group_by".to_string(), group_by.join(" ")));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("page_token".to_string(), page_token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/marketing/stats/singlesends/{}/links?{}",
                crate::progenitor_support::encode_path(id),
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
     * Export Single Send Stats.
     *
     * This function performs a `GET` to the `/marketing/stats/singlesends/export` endpoint.
     *
     * **This endpoint allows you to export Single Send stats as .CSV data**.
     *
     * You can specify one Single Send or many: include as many Single Send IDs as you need, separating them with commas, as the value of the `ids` query string paramter.
     *
     * The data is returned as plain text response but in .CSV format, so your application making the call can present the information in whatever way is most appropriate, or just save the data as a .csv file.
     *
     * **Parameters:**
     *
     * * `ids: &[String]` -- The recipient IDs of the recipients that already existed from this request.
     * * `timezone: &str` -- The [IANA Area/Region](https://en.wikipedia.org/wiki/Tz_database#Names_of_time_zones) string representing the timezone in which the stats are to be presented; i.e. `"America/Chicago"`. This parameter changes the timezone format only; it does not alter which stats are returned.
     */
    pub async fn get_singlesend_stats_export(
        &self,
        ids: &[String],
        timezone: &str,
    ) -> ClientResult<String> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.join(" ")));
        }
        if !timezone.is_empty() {
            query_args.push(("timezone".to_string(), timezone.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/marketing/stats/singlesends/export?{}", query_),
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
     * Export Automation Stats.
     *
     * This function performs a `GET` to the `/marketing/stats/automations/export` endpoint.
     *
     * **This endpoint allows you to export Automation stats as CSV data**.
     *
     * You can specify one Automation or many: include as many Automation IDs as you need, separating them with commas, as the value of the `ids` query string paramter.
     *
     * The data is returned as plain text response but in CSV format, so your application making the call can present the information in whatever way is most appropriate, or just save the data as a `.csv` file.
     *
     * **Parameters:**
     *
     * * `ids: &[String]` -- The recipient IDs of the recipients that already existed from this request.
     * * `timezone: &str` -- The [IANA Area/Region](https://en.wikipedia.org/wiki/Tz_database#Names_of_time_zones) string representing the timezone in which the stats are to be presented; i.e. `"America/Chicago"`. This parameter changes the timezone format only; it does not alter which stats are returned.
     */
    pub async fn get_automations_stats_export(
        &self,
        ids: &[String],
        timezone: &str,
    ) -> ClientResult<String> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ids.is_empty() {
            query_args.push(("ids".to_string(), ids.join(" ")));
        }
        if !timezone.is_empty() {
            query_args.push(("timezone".to_string(), timezone.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/marketing/stats/automations/export?{}", query_),
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
