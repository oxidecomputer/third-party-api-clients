use crate::Client;
use crate::ClientResult;

pub struct Stats {
    pub client: Client,
}

impl Stats {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Stats { client }
    }

    /**
     * Retrieve global email statistics.
     *
     * This function performs a `GET` to the `/stats` endpoint.
     *
     * **This endpoint allows you to retrieve all of your global email statistics between a given date range.**
     *
     * Parent accounts will see aggregated stats for their account and all subuser accounts. Subuser accounts will only see their own stats.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     * * `offset: i64` -- The point in the list to begin retrieving results.
     * * `offset: i64` -- The point in the list to begin retrieving results.
     * * `aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy` -- How to group the statistics. Must be either "day", "week", or "month".
     * * `start_date: &str` -- The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD.
     * * `end_date: &str` -- The end date of the statistics to retrieve. Defaults to today. Must follow format YYYY-MM-DD.
     */
    pub async fn get_page(
        &self,
        offset: i64,
        aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy,
        start_date: &str,
        end_date: &str,
    ) -> ClientResult<Vec<crate::types::GetStatsResponseData>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.to_string().is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
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
        let url = self.client.url(&format!("/stats?{}", query_), None);
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
     * Retrieve global email statistics.
     *
     * This function performs a `GET` to the `/stats` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve all of your global email statistics between a given date range.**
     *
     * Parent accounts will see aggregated stats for their account and all subuser accounts. Subuser accounts will only see their own stats.
     */
    pub async fn get_all(
        &self,
        offset: i64,
        aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy,
        start_date: &str,
        end_date: &str,
    ) -> ClientResult<Vec<crate::types::GetStatsResponseData>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.to_string().is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
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
        let url = self.client.url(&format!("/stats?{}", query_), None);
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
     * Retrieve email statistics by country and state/province.
     *
     * This function performs a `GET` to the `/geo/stats` endpoint.
     *
     * **This endpoint allows you to retrieve your email statistics segmented by country and state/province.**
     *
     * **We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.
     *
     * Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [User Guide](https://sendgrid.com/docs/User_Guide/Statistics/index.html).
     *
     * **Parameters:**
     *
     * * `country: crate::types::Country` -- The country you would like to see statistics for. Currently only supported for US and CA.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     * * `offset: i64` -- The point in the list to begin retrieving results.
     * * `offset: i64` -- The point in the list to begin retrieving results.
     * * `aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy` -- How to group the statistics. Must be either "day", "week", or "month".
     * * `start_date: &str` -- The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD.
     * * `end_date: &str` -- The end date of the statistics to retrieve. Defaults to today. Must follow format YYYY-MM-DD.
     */
    pub async fn get_geo(
        &self,
        country: crate::types::Country,
        offset: i64,
        aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy,
        start_date: &str,
        end_date: &str,
    ) -> ClientResult<Vec<crate::types::GetGeoStatsResponseData>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.to_string().is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
        }
        if !country.to_string().is_empty() {
            query_args.push(("country".to_string(), country.to_string()));
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
        let url = self.client.url(&format!("/geo/stats?{}", query_), None);
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
     * Retrieve email statistics by country and state/province.
     *
     * This function performs a `GET` to the `/geo/stats` endpoint.
     *
     * As opposed to `get_geo`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve your email statistics segmented by country and state/province.**
     *
     * **We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.
     *
     * Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [User Guide](https://sendgrid.com/docs/User_Guide/Statistics/index.html).
     */
    pub async fn get_all_geo(
        &self,
        country: crate::types::Country,
        offset: i64,
        aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy,
        start_date: &str,
        end_date: &str,
    ) -> ClientResult<Vec<crate::types::GetGeoStatsResponseData>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.to_string().is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
        }
        if !country.to_string().is_empty() {
            query_args.push(("country".to_string(), country.to_string()));
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
        let url = self.client.url(&format!("/geo/stats?{}", query_), None);
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
     * Retrieve email statistics by device type.
     *
     * This function performs a `GET` to the `/devices/stats` endpoint.
     *
     * **This endpoint allows you to retrieve your email statistics segmented by the device type.**
     *
     * **We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.
     *
     * ## Available Device Types
     * | **Device** | **Description** | **Example** |
     * |---|---|---|
     * | Desktop | Email software on desktop computer. | I.E., Outlook, Sparrow, or Apple Mail. |
     * | Webmail |	A web-based email client. | I.E., Yahoo, Google, AOL, or Outlook.com. |
     * | Phone | A smart phone. | iPhone, Android, Blackberry, etc.
     * | Tablet | A tablet computer. | iPad, android based tablet, etc. |
     * | Other | An unrecognized device. |
     *
     * Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [Statistics Overview](https://sendgrid.com/docs/ui/analytics-and-reporting/stats-overview/).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     * * `offset: i64` -- The point in the list to begin retrieving results.
     * * `offset: i64` -- The point in the list to begin retrieving results.
     * * `aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy` -- How to group the statistics. Must be either "day", "week", or "month".
     * * `start_date: &str` -- The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD.
     * * `end_date: &str` -- The end date of the statistics to retrieve. Defaults to today. Must follow format YYYY-MM-DD.
     */
    pub async fn get_devices(
        &self,
        offset: i64,
        aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy,
        start_date: &str,
        end_date: &str,
    ) -> ClientResult<Vec<crate::types::GetClientsStatsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.to_string().is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
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
        let url = self.client.url(&format!("/devices/stats?{}", query_), None);
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
     * Retrieve email statistics by device type.
     *
     * This function performs a `GET` to the `/devices/stats` endpoint.
     *
     * As opposed to `get_devices`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve your email statistics segmented by the device type.**
     *
     * **We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.
     *
     * ## Available Device Types
     * | **Device** | **Description** | **Example** |
     * |---|---|---|
     * | Desktop | Email software on desktop computer. | I.E., Outlook, Sparrow, or Apple Mail. |
     * | Webmail |	A web-based email client. | I.E., Yahoo, Google, AOL, or Outlook.com. |
     * | Phone | A smart phone. | iPhone, Android, Blackberry, etc.
     * | Tablet | A tablet computer. | iPad, android based tablet, etc. |
     * | Other | An unrecognized device. |
     *
     * Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [Statistics Overview](https://sendgrid.com/docs/ui/analytics-and-reporting/stats-overview/).
     */
    pub async fn get_all_devices(
        &self,
        offset: i64,
        aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy,
        start_date: &str,
        end_date: &str,
    ) -> ClientResult<Vec<crate::types::GetClientsStatsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.to_string().is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
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
        let url = self.client.url(&format!("/devices/stats?{}", query_), None);
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
     * Retrieve email statistics by client type.
     *
     * This function performs a `GET` to the `/clients/stats` endpoint.
     *
     * **This endpoint allows you to retrieve your email statistics segmented by client type.**
     *
     * **We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.
     *
     * Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [Statistics Overview](https://sendgrid.com/docs/ui/analytics-and-reporting/stats-overview/).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     * * `start_date: &str` -- The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD.
     * * `end_date: &str` -- The end date of the statistics to retrieve. Defaults to today. Must follow format YYYY-MM-DD.
     * * `aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy` -- How to group the statistics. Must be either "day", "week", or "month".
     */
    pub async fn get_clients(
        &self,
        start_date: &str,
        end_date: &str,
        aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy,
    ) -> ClientResult<Vec<crate::types::GetClientsStatsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.to_string().is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
        }
        if !end_date.is_empty() {
            query_args.push(("end_date".to_string(), end_date.to_string()));
        }
        if !start_date.is_empty() {
            query_args.push(("start_date".to_string(), start_date.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/clients/stats?{}", query_), None);
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
     * Retrieve email statistics by client type.
     *
     * This function performs a `GET` to the `/clients/stats` endpoint.
     *
     * As opposed to `get_clients`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve your email statistics segmented by client type.**
     *
     * **We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.
     *
     * Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [Statistics Overview](https://sendgrid.com/docs/ui/analytics-and-reporting/stats-overview/).
     */
    pub async fn get_all_clients(
        &self,
        start_date: &str,
        end_date: &str,
        aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy,
    ) -> ClientResult<Vec<crate::types::GetClientsStatsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.to_string().is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
        }
        if !end_date.is_empty() {
            query_args.push(("end_date".to_string(), end_date.to_string()));
        }
        if !start_date.is_empty() {
            query_args.push(("start_date".to_string(), start_date.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/clients/stats?{}", query_), None);
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
     * Retrieve stats by a specific client type.
     *
     * This function performs a `GET` to the `/clients/{client_type}/stats` endpoint.
     *
     * **This endpoint allows you to retrieve your email statistics segmented by a specific client type.**
     *
     * **We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.
     *
     * ## Available Client Types
     * - phone
     * - tablet
     * - webmail
     * - desktop
     *
     * Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [Statistics Overview](https://sendgrid.com/docs/ui/analytics-and-reporting/stats-overview/).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     * * `start_date: &str` -- The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD.
     * * `end_date: &str` -- The end date of the statistics to retrieve. Defaults to today. Must follow format YYYY-MM-DD.
     * * `aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy` -- How to group the statistics. Must be either "day", "week", or "month".
     */
    pub async fn get_clients_client_type(
        &self,
        client_type: crate::types::ClientType,
        start_date: &str,
        end_date: &str,
        aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy,
    ) -> ClientResult<Vec<crate::types::GetClientsStatsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.to_string().is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
        }
        if !end_date.is_empty() {
            query_args.push(("end_date".to_string(), end_date.to_string()));
        }
        if !start_date.is_empty() {
            query_args.push(("start_date".to_string(), start_date.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/clients/{}/stats?{}",
                crate::progenitor_support::encode_path(&client_type.to_string()),
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
     * Retrieve stats by a specific client type.
     *
     * This function performs a `GET` to the `/clients/{client_type}/stats` endpoint.
     *
     * As opposed to `get_clients_client_type`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve your email statistics segmented by a specific client type.**
     *
     * **We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.
     *
     * ## Available Client Types
     * - phone
     * - tablet
     * - webmail
     * - desktop
     *
     * Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [Statistics Overview](https://sendgrid.com/docs/ui/analytics-and-reporting/stats-overview/).
     */
    pub async fn get_all_clients_client_type(
        &self,
        client_type: crate::types::ClientType,
        start_date: &str,
        end_date: &str,
        aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy,
    ) -> ClientResult<Vec<crate::types::GetClientsStatsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.to_string().is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
        }
        if !end_date.is_empty() {
            query_args.push(("end_date".to_string(), end_date.to_string()));
        }
        if !start_date.is_empty() {
            query_args.push(("start_date".to_string(), start_date.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/clients/{}/stats?{}",
                crate::progenitor_support::encode_path(&client_type.to_string()),
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
     * Retrieve email statistics by mailbox provider.
     *
     * This function performs a `GET` to the `/mailbox_providers/stats` endpoint.
     *
     * **This endpoint allows you to retrieve your email statistics segmented by recipient mailbox provider.**
     *
     * **We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.
     *
     * Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [Statistics Overview](https://sendgrid.com/docs/ui/analytics-and-reporting/stats-overview/).
     *
     * **Parameters:**
     *
     * * `mailbox_providers: &str` -- The mail box providers to get statistics for. You can include up to 10 by including this parameter multiple times.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     * * `offset: i64` -- The point in the list to begin retrieving results.
     * * `offset: i64` -- The point in the list to begin retrieving results.
     * * `aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy` -- How to group the statistics. Must be either "day", "week", or "month".
     * * `start_date: &str` -- The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD.
     * * `end_date: &str` -- The end date of the statistics to retrieve. Defaults to today. Must follow format YYYY-MM-DD.
     */
    pub async fn get_mailbox_providers(
        &self,
        mailbox_providers: &str,
        offset: i64,
        aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy,
        start_date: &str,
        end_date: &str,
    ) -> ClientResult<Vec<crate::types::GetMailboxProvidersStatsResponseData>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.to_string().is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
        }
        if !end_date.is_empty() {
            query_args.push(("end_date".to_string(), end_date.to_string()));
        }
        if !mailbox_providers.is_empty() {
            query_args.push((
                "mailbox_providers".to_string(),
                mailbox_providers.to_string(),
            ));
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
            .url(&format!("/mailbox_providers/stats?{}", query_), None);
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
     * Retrieve email statistics by mailbox provider.
     *
     * This function performs a `GET` to the `/mailbox_providers/stats` endpoint.
     *
     * As opposed to `get_mailbox_providers`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve your email statistics segmented by recipient mailbox provider.**
     *
     * **We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.
     *
     * Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [Statistics Overview](https://sendgrid.com/docs/ui/analytics-and-reporting/stats-overview/).
     */
    pub async fn get_all_mailbox_providers(
        &self,
        mailbox_providers: &str,
        offset: i64,
        aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy,
        start_date: &str,
        end_date: &str,
    ) -> ClientResult<Vec<crate::types::GetMailboxProvidersStatsResponseData>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.to_string().is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
        }
        if !end_date.is_empty() {
            query_args.push(("end_date".to_string(), end_date.to_string()));
        }
        if !mailbox_providers.is_empty() {
            query_args.push((
                "mailbox_providers".to_string(),
                mailbox_providers.to_string(),
            ));
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
            .url(&format!("/mailbox_providers/stats?{}", query_), None);
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
     * Retrieve email statistics by browser.
     *
     * This function performs a `GET` to the `/browsers/stats` endpoint.
     *
     * **This endpoint allows you to retrieve your email statistics segmented by browser type.**
     *
     * **We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.
     *
     * Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [Statistics Overview](https://sendgrid.com/docs/ui/analytics-and-reporting/stats-overview/).
     *
     * **Parameters:**
     *
     * * `browsers: &str` -- The browsers to get statistics for. You can include up to 10 different browsers by including this parameter multiple times.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     * * `offset: i64` -- The point in the list to begin retrieving results.
     * * `offset: i64` -- The point in the list to begin retrieving results.
     * * `aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy` -- How to group the statistics. Must be either "day", "week", or "month".
     * * `start_date: &str` -- The starting date of the statistics to retrieve. Must follow format YYYY-MM-DD.
     * * `end_date: &str` -- The end date of the statistics to retrieve. Defaults to today. Must follow format YYYY-MM-DD.
     */
    pub async fn get_browsers(
        &self,
        browsers: &str,
        offset: i64,
        aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy,
        start_date: &str,
        end_date: &str,
    ) -> ClientResult<Vec<crate::types::GetBrowsersStatsResponseData>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.to_string().is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
        }
        if !browsers.is_empty() {
            query_args.push(("browsers".to_string(), browsers.to_string()));
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
            .url(&format!("/browsers/stats?{}", query_), None);
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
     * Retrieve email statistics by browser.
     *
     * This function performs a `GET` to the `/browsers/stats` endpoint.
     *
     * As opposed to `get_browsers`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve your email statistics segmented by browser type.**
     *
     * **We only store up to 7 days of email activity in our database.** By default, 500 items will be returned per request via the Advanced Stats API endpoints.
     *
     * Advanced Stats provide a more in-depth view of your email statistics and the actions taken by your recipients. You can segment these statistics by geographic location, device type, client type, browser, and mailbox provider. For more information about statistics, please see our [Statistics Overview](https://sendgrid.com/docs/ui/analytics-and-reporting/stats-overview/).
     */
    pub async fn get_all_browsers(
        &self,
        browsers: &str,
        offset: i64,
        aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy,
        start_date: &str,
        end_date: &str,
    ) -> ClientResult<Vec<crate::types::GetBrowsersStatsResponseData>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.to_string().is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
        }
        if !browsers.is_empty() {
            query_args.push(("browsers".to_string(), browsers.to_string()));
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
            .url(&format!("/browsers/stats?{}", query_), None);
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
