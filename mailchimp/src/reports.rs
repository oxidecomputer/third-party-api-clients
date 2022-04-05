use anyhow::Result;

use crate::Client;

pub struct Reports {
    pub client: Client,
}

impl Reports {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Reports { client }
    }

    /**
    * List campaign reports.
    *
    * This function performs a `GET` to the `/reports` endpoint.
    *
    * Get campaign reports.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `type_: crate::types::CampaignType` -- There are four types of [campaigns](https://mailchimp.com/help/getting-started-with-campaigns/) you can create in Mailchimp. A/B Split campaigns have been deprecated and variate campaigns should be used instead.
    * * `before_send_time: chrono::DateTime<chrono::Utc>` -- Restrict the response to campaigns sent before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `since_send_time: chrono::DateTime<chrono::Utc>` -- Restrict the response to campaigns sent after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    */
    pub async fn get(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        type_: crate::types::CampaignType,
        before_send_time: Option<chrono::DateTime<chrono::Utc>>,
        since_send_time: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<crate::types::CampaignReportsData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if let Some(date) = before_send_time {
            query_args.push(("before_send_time".to_string(), date.to_rfc3339()));
        }
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if let Some(date) = since_send_time {
            query_args.push(("since_send_time".to_string(), date.to_rfc3339()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/reports?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * Get campaign report.
    *
    * This function performs a `GET` to the `/reports/{campaign_id}` endpoint.
    *
    * Get report details for a specific sent campaign.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn get_reports(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        campaign_id: &str,
    ) -> Result<crate::types::Reports> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reports/{}?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List abuse reports.
    *
    * This function performs a `GET` to the `/reports/{campaign_id}/abuse-reports` endpoint.
    *
    * Get a list of abuse complaints for a specific campaign.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn get_abuse(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        campaign_id: &str,
    ) -> Result<crate::types::AbuseComplaintsData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reports/{}/abuse-reports?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Get abuse report.
    *
    * This function performs a `GET` to the `/reports/{campaign_id}/abuse-reports/{report_id}` endpoint.
    *
    * Get information about a specific abuse report for a campaign.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `campaign_id: &str` -- The unique id for the campaign.
    * * `report_id: &str` -- The id for the abuse report.
    */
    pub async fn get_abuse_reports(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        campaign_id: &str,
        report_id: &str,
    ) -> Result<crate::types::AbuseComplaint> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reports/{}/abuse-reports/{}?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            crate::progenitor_support::encode_path(&report_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List campaign feedback.
    *
    * This function performs a `GET` to the `/reports/{campaign_id}/advice` endpoint.
    *
    * Get feedback based on a campaign's statistics. Advice feedback is based on campaign stats like opens, clicks, unsubscribes, bounces, and more.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn get_advice(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        campaign_id: &str,
    ) -> Result<crate::types::CampaignAdviceReport> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reports/{}/advice?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List campaign details.
    *
    * This function performs a `GET` to the `/reports/{campaign_id}/click-details` endpoint.
    *
    * Get information about clicks on specific links in your Mailchimp campaigns.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn get_click_detail(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        campaign_id: &str,
    ) -> Result<crate::types::ClickDetailReport> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reports/{}/click-details?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Get campaign link details.
    *
    * This function performs a `GET` to the `/reports/{campaign_id}/click-details/{link_id}` endpoint.
    *
    * Get click details for a specific link in a campaign.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `campaign_id: &str` -- The unique id for the campaign.
    * * `link_id: &str` -- The name of the folder.
    */
    pub async fn get_click_detail_reports(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        campaign_id: &str,
        link_id: &str,
    ) -> Result<crate::types::UrlsClicked> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reports/{}/click-details/{}?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            crate::progenitor_support::encode_path(&link_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List clicked link subscribers.
    *
    * This function performs a `GET` to the `/reports/{campaign_id}/click-details/{link_id}/members` endpoint.
    *
    * Get information about list members who clicked on a specific link in a campaign.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `campaign_id: &str` -- The unique id for the campaign.
    * * `link_id: &str` -- The name of the folder.
    */
    pub async fn get_click_details_member(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        campaign_id: &str,
        link_id: &str,
    ) -> Result<crate::types::ClickDetailMembers> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reports/{}/click-details/{}/members?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            crate::progenitor_support::encode_path(&link_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Get clicked link subscriber.
    *
    * This function performs a `GET` to the `/reports/{campaign_id}/click-details/{link_id}/members/{subscriber_hash}` endpoint.
    *
    * Get information about a specific subscriber who clicked a link in a specific campaign.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `campaign_id: &str` -- The unique id for the campaign.
    * * `link_id: &str` -- The name of the folder.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    */
    pub async fn get_click_details_member_reports(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        campaign_id: &str,
        link_id: &str,
        subscriber_hash: &str,
    ) -> Result<crate::types::ClickDetailMember> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reports/{}/click-details/{}/members/{}?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            crate::progenitor_support::encode_path(&link_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List campaign open details.
    *
    * This function performs a `GET` to the `/reports/{campaign_id}/open-details` endpoint.
    *
    * Get detailed information about any campaign emails that were opened by a list member.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `campaign_id: &str` -- The unique id for the campaign.
    * * `since: &str` -- Restrict results to campaign open events that occur after a specific time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    */
    pub async fn get_open_detail(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        campaign_id: &str,
        since: &str,
    ) -> Result<crate::types::OpenDetailReport> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !since.is_empty() {
            query_args.push(("since".to_string(), since.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reports/{}/open-details?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Get opened campaign subscriber.
    *
    * This function performs a `GET` to the `/reports/{campaign_id}/open-details/{subscriber_hash}` endpoint.
    *
    * Get information about a specific subscriber who opened a campaign.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `campaign_id: &str` -- The unique id for the campaign.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    */
    pub async fn get_open_details_member(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        campaign_id: &str,
        subscriber_hash: &str,
    ) -> Result<crate::types::OpenActivity> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reports/{}/open-details/{}?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List domain performance stats.
    *
    * This function performs a `GET` to the `/reports/{campaign_id}/domain-performance` endpoint.
    *
    * Get statistics for the top-performing email domains in a campaign.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn get_domain_performance(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        campaign_id: &str,
    ) -> Result<crate::types::DomainPerformance> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reports/{}/domain-performance?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List EepURL activity.
    *
    * This function performs a `GET` to the `/reports/{campaign_id}/eepurl` endpoint.
    *
    * Get a summary of social activity for the campaign, tracked by EepURL.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn get_eepurl(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        campaign_id: &str,
    ) -> Result<crate::types::EepurlActivity> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reports/{}/eepurl?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List email activity.
    *
    * This function performs a `GET` to the `/reports/{campaign_id}/email-activity` endpoint.
    *
    * Get a list of member's subscriber activity in a specific campaign.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `campaign_id: &str` -- The unique id for the campaign.
    * * `since: &str` -- Restrict results to email activity events that occur after a specific time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    */
    pub async fn get_email_activity(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        campaign_id: &str,
        since: &str,
    ) -> Result<crate::types::EmailActivityData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !since.is_empty() {
            query_args.push(("since".to_string(), since.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reports/{}/email-activity?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Get subscriber email activity.
    *
    * This function performs a `GET` to the `/reports/{campaign_id}/email-activity/{subscriber_hash}` endpoint.
    *
    * Get a specific list member's activity in a campaign including opens, clicks, and bounces.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `campaign_id: &str` -- The unique id for the campaign.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    * * `since: &str` -- Restrict results to email activity events that occur after a specific time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    */
    pub async fn get_email_activity_reports(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        campaign_id: &str,
        subscriber_hash: &str,
        since: &str,
    ) -> Result<crate::types::EmailActivity> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if !since.is_empty() {
            query_args.push(("since".to_string(), since.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reports/{}/email-activity/{}?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List top open activities.
    *
    * This function performs a `GET` to the `/reports/{campaign_id}/locations` endpoint.
    *
    * Get top open locations for a specific campaign.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `campaign_id: &str` -- The unique id for the campaign.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    */
    pub async fn get_location(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        campaign_id: &str,
        count: i64,
        offset: i64,
    ) -> Result<crate::types::OpenLocationsData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reports/{}/locations?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List campaign recipients.
    *
    * This function performs a `GET` to the `/reports/{campaign_id}/sent-to` endpoint.
    *
    * Get information about campaign recipients.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn get_sent(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        campaign_id: &str,
    ) -> Result<crate::types::SentData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reports/{}/sent-to?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Get campaign recipient info.
    *
    * This function performs a `GET` to the `/reports/{campaign_id}/sent-to/{subscriber_hash}` endpoint.
    *
    * Get information about a specific campaign recipient.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `campaign_id: &str` -- The unique id for the campaign.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    */
    pub async fn get_sent_reports(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        campaign_id: &str,
        subscriber_hash: &str,
    ) -> Result<crate::types::SentTo> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reports/{}/sent-to/{}?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List child campaign reports.
    *
    * This function performs a `GET` to the `/reports/{campaign_id}/sub-reports` endpoint.
    *
    * Get a list of reports with child campaigns for a specific parent campaign.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn get_sub(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        campaign_id: &str,
    ) -> Result<crate::types::CampaignSubReports> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reports/{}/sub-reports?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List unsubscribed members.
    *
    * This function performs a `GET` to the `/reports/{campaign_id}/unsubscribed` endpoint.
    *
    * Get information about members who have unsubscribed from a specific campaign.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn get_unsubscribed(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        campaign_id: &str,
    ) -> Result<crate::types::UnsubscribesData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reports/{}/unsubscribed?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Get unsubscribed member.
    *
    * This function performs a `GET` to the `/reports/{campaign_id}/unsubscribed/{subscriber_hash}` endpoint.
    *
    * Get information about a specific list member who unsubscribed from a campaign.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `campaign_id: &str` -- The unique id for the campaign.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    */
    pub async fn get_unsubscribed_reports(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        campaign_id: &str,
        subscriber_hash: &str,
    ) -> Result<crate::types::Unsubscribes> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reports/{}/unsubscribed/{}?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List campaign product activity.
    *
    * This function performs a `GET` to the `/reports/{campaign_id}/ecommerce-product-activity` endpoint.
    *
    * Get breakdown of product activity for a campaign
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `campaign_id: &str` -- The unique id for the campaign.
    * * `sort_field: crate::types::GetReportsEcommerceProductActivitySortField` -- Returns files sorted by the specified field.
    */
    pub async fn get_ecommerce_product_activity(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        campaign_id: &str,
        sort_field: crate::types::GetReportsEcommerceProductActivitySortField,
    ) -> Result<crate::types::GetReportsEcommerceProductActivityResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !sort_field.to_string().is_empty() {
            query_args.push(("sort_field".to_string(), sort_field.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/reports/{}/ecommerce-product-activity?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }
}
