use anyhow::Result;

use crate::Client;

pub struct Campaigns {
    pub client: Client,
}

impl Campaigns {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Campaigns { client }
    }

    /**
    * List campaigns.
    *
    * This function performs a `GET` to the `/campaigns` endpoint.
    *
    * Get all campaigns in an account.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `type_: crate::types::CampaignType` -- There are four types of [campaigns](https://mailchimp.com/help/getting-started-with-campaigns/) you can create in Mailchimp. A/B Split campaigns have been deprecated and variate campaigns should be used instead.
    * * `status: crate::types::GetCampaignsStatus` -- The status of the campaign.
    * * `before_send_time: chrono::DateTime<chrono::Utc>` -- Restrict the response to campaigns sent before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `since_send_time: chrono::DateTime<chrono::Utc>` -- Restrict the response to campaigns sent after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `before_create_time: chrono::DateTime<chrono::Utc>` -- Restrict the response to campaigns created before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `since_create_time: chrono::DateTime<chrono::Utc>` -- Restrict the response to campaigns created after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `list_id: &str` -- The unique id for the list.
    * * `folder_id: &str` -- The name of the folder.
    * * `member_id: &str` -- Retrieve campaigns sent to a particular list member. Member ID is The MD5 hash of the lowercase version of the list member’s email address.
    * * `sort_field: crate::types::SortField` -- Returns files sorted by the specified field.
    * * `sort_dir: crate::types::SortDir` -- Determines the order direction for sorted results.
    */
    pub async fn get(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        type_: crate::types::CampaignType,
        status: crate::types::GetCampaignsStatus,
        before_send_time: Option<chrono::DateTime<chrono::Utc>>,
        since_send_time: Option<chrono::DateTime<chrono::Utc>>,
        before_create_time: Option<chrono::DateTime<chrono::Utc>>,
        since_create_time: Option<chrono::DateTime<chrono::Utc>>,
        list_id: &str,
        folder_id: &str,
        member_id: &str,
        sort_field: crate::types::SortField,
        sort_dir: crate::types::SortDir,
    ) -> Result<crate::types::GetCampaignsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if let Some(date) = before_create_time {
            query_args.push(("before_create_time".to_string(), date.to_rfc3339()));
        }
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
        if !folder_id.is_empty() {
            query_args.push(("folder_id".to_string(), folder_id.to_string()));
        }
        if !list_id.is_empty() {
            query_args.push(("list_id".to_string(), list_id.to_string()));
        }
        if !member_id.is_empty() {
            query_args.push(("member_id".to_string(), member_id.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if let Some(date) = since_create_time {
            query_args.push(("since_create_time".to_string(), date.to_rfc3339()));
        }
        if let Some(date) = since_send_time {
            query_args.push(("since_send_time".to_string(), date.to_rfc3339()));
        }
        if !sort_dir.to_string().is_empty() {
            query_args.push(("sort_dir".to_string(), sort_dir.to_string()));
        }
        if !sort_field.to_string().is_empty() {
            query_args.push(("sort_field".to_string(), sort_field.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/campaigns?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * Add campaign.
    *
    * This function performs a `POST` to the `/campaigns` endpoint.
    *
    * Create a new Mailchimp campaign.
    */
    pub async fn post(
        &self,
        body: &crate::types::CreatedCampaign,
    ) -> Result<crate::types::Campaign> {
        let url = "/campaigns".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get campaign info.
    *
    * This function performs a `GET` to the `/campaigns/{campaign_id}` endpoint.
    *
    * Get information about a specific campaign.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn get_campaigns(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        campaign_id: &str,
    ) -> Result<crate::types::Campaign> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/campaigns/{}?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete campaign.
    *
    * This function performs a `DELETE` to the `/campaigns/{campaign_id}` endpoint.
    *
    * Remove a campaign from your Mailchimp account.
    *
    * **Parameters:**
    *
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn delete(&self, campaign_id: &str) -> Result<()> {
        let url = format!(
            "/campaigns/{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update campaign settings.
    *
    * This function performs a `PATCH` to the `/campaigns/{campaign_id}` endpoint.
    *
    * Update some or all of the settings for a specific campaign.
    *
    * **Parameters:**
    *
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn patch(
        &self,
        campaign_id: &str,
        body: &crate::types::CampaignData,
    ) -> Result<crate::types::Campaign> {
        let url = format!(
            "/campaigns/{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Cancel campaign.
    *
    * This function performs a `POST` to the `/campaigns/{campaign_id}/actions/cancel-send` endpoint.
    *
    * Cancel a Regular or Plain-Text Campaign after you send, before all of your recipients receive it. This feature is included with Mailchimp Pro.
    *
    * **Parameters:**
    *
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn post_actions_cancel_send(&self, campaign_id: &str) -> Result<()> {
        let url = format!(
            "/campaigns/{}/actions/cancel-send",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * Replicate campaign.
    *
    * This function performs a `POST` to the `/campaigns/{campaign_id}/actions/replicate` endpoint.
    *
    * Replicate a campaign in saved or send status.
    *
    * **Parameters:**
    *
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn post_actions_replicate(
        &self,
        campaign_id: &str,
    ) -> Result<crate::types::CampaignDataType> {
        let url = format!(
            "/campaigns/{}/actions/replicate",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * Send campaign.
    *
    * This function performs a `POST` to the `/campaigns/{campaign_id}/actions/send` endpoint.
    *
    * Send a Mailchimp campaign. For RSS Campaigns, the campaign will send according to its schedule. All other campaigns will send immediately.
    *
    * **Parameters:**
    *
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn post_actions_send(&self, campaign_id: &str) -> Result<()> {
        let url = format!(
            "/campaigns/{}/actions/send",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * Schedule campaign.
    *
    * This function performs a `POST` to the `/campaigns/{campaign_id}/actions/schedule` endpoint.
    *
    * Schedule a campaign for delivery. If you're using Multivariate Campaigns to test send times or sending RSS Campaigns, use the send action instead.
    *
    * **Parameters:**
    *
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn post_actions_schedule(
        &self,
        campaign_id: &str,
        body: &crate::types::PostCampaignsActionsScheduleRequest,
    ) -> Result<()> {
        let url = format!(
            "/campaigns/{}/actions/schedule",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Unschedule campaign.
    *
    * This function performs a `POST` to the `/campaigns/{campaign_id}/actions/unschedule` endpoint.
    *
    * Unschedule a scheduled campaign that hasn't started sending.
    *
    * **Parameters:**
    *
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn post_actions_unschedule(&self, campaign_id: &str) -> Result<()> {
        let url = format!(
            "/campaigns/{}/actions/unschedule",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * Send test email.
    *
    * This function performs a `POST` to the `/campaigns/{campaign_id}/actions/test` endpoint.
    *
    * Send a test email.
    *
    * **Parameters:**
    *
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn post_actions_test(
        &self,
        campaign_id: &str,
        body: &crate::types::PostCampaignsActionsTestRequest,
    ) -> Result<()> {
        let url = format!(
            "/campaigns/{}/actions/test",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Pause rss campaign.
    *
    * This function performs a `POST` to the `/campaigns/{campaign_id}/actions/pause` endpoint.
    *
    * Pause an RSS-Driven campaign.
    *
    * **Parameters:**
    *
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn post_actions_pause(&self, campaign_id: &str) -> Result<()> {
        let url = format!(
            "/campaigns/{}/actions/pause",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * Resume rss campaign.
    *
    * This function performs a `POST` to the `/campaigns/{campaign_id}/actions/resume` endpoint.
    *
    * Resume an RSS-Driven campaign.
    *
    * **Parameters:**
    *
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn post_actions_resume(&self, campaign_id: &str) -> Result<()> {
        let url = format!(
            "/campaigns/{}/actions/resume",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * Resend campaign.
    *
    * This function performs a `POST` to the `/campaigns/{campaign_id}/actions/create-resend` endpoint.
    *
    * Creates a Resend to Non-Openers version of this campaign. We will also check if this campaign meets the criteria for Resend to Non-Openers campaigns.
    *
    * **Parameters:**
    *
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn post_actions_create_resend(
        &self,
        campaign_id: &str,
    ) -> Result<crate::types::CampaignDataType> {
        let url = format!(
            "/campaigns/{}/actions/create-resend",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * Get campaign content.
    *
    * This function performs a `GET` to the `/campaigns/{campaign_id}/content` endpoint.
    *
    * Get the the HTML and plain-text content for a campaign.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn get_content(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        campaign_id: &str,
    ) -> Result<crate::types::CampaignContent> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/campaigns/{}/content?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Set campaign content.
    *
    * This function performs a `PUT` to the `/campaigns/{campaign_id}/content` endpoint.
    *
    * Set the content for a campaign.
    *
    * **Parameters:**
    *
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn put_content(
        &self,
        campaign_id: &str,
        body: &crate::types::CampaignContentData,
    ) -> Result<crate::types::CampaignContent> {
        let url = format!(
            "/campaigns/{}/content",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List campaign feedback.
    *
    * This function performs a `GET` to the `/campaigns/{campaign_id}/feedback` endpoint.
    *
    * Get team feedback while you're working together on a Mailchimp campaign.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn get_feedback(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        campaign_id: &str,
    ) -> Result<crate::types::CampaignReports> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/campaigns/{}/feedback?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Add campaign feedback.
    *
    * This function performs a `POST` to the `/campaigns/{campaign_id}/feedback` endpoint.
    *
    * Add feedback on a specific campaign.
    *
    * **Parameters:**
    *
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn post_feedback(
        &self,
        campaign_id: &str,
        body: &crate::types::CampaignFeedback,
    ) -> Result<crate::types::CampaignFeedbackData> {
        let url = format!(
            "/campaigns/{}/feedback",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get campaign feedback message.
    *
    * This function performs a `GET` to the `/campaigns/{campaign_id}/feedback/{feedback_id}` endpoint.
    *
    * Get a specific feedback message from a campaign.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `campaign_id: &str` -- The unique id for the campaign.
    * * `feedback_id: &str` -- The unique id for the feedback message.
    */
    pub async fn get_feedback_campaigns(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        campaign_id: &str,
        feedback_id: &str,
    ) -> Result<crate::types::CampaignFeedbackData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/campaigns/{}/feedback/{}?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            crate::progenitor_support::encode_path(&feedback_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete campaign feedback message.
    *
    * This function performs a `DELETE` to the `/campaigns/{campaign_id}/feedback/{feedback_id}` endpoint.
    *
    * Remove a specific feedback message for a campaign.
    *
    * **Parameters:**
    *
    * * `campaign_id: &str` -- The unique id for the campaign.
    * * `feedback_id: &str` -- The unique id for the feedback message.
    */
    pub async fn delete_feedback(&self, campaign_id: &str, feedback_id: &str) -> Result<()> {
        let url = format!(
            "/campaigns/{}/feedback/{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            crate::progenitor_support::encode_path(&feedback_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update campaign feedback message.
    *
    * This function performs a `PATCH` to the `/campaigns/{campaign_id}/feedback/{feedback_id}` endpoint.
    *
    * Update a specific feedback message for a campaign.
    *
    * **Parameters:**
    *
    * * `campaign_id: &str` -- The unique id for the campaign.
    * * `feedback_id: &str` -- The unique id for the feedback message.
    */
    pub async fn patch_feedback(
        &self,
        campaign_id: &str,
        feedback_id: &str,
        body: &crate::types::CampaignFeedbackDataType,
    ) -> Result<crate::types::CampaignFeedbackData> {
        let url = format!(
            "/campaigns/{}/feedback/{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            crate::progenitor_support::encode_path(&feedback_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get campaign send checklist.
    *
    * This function performs a `GET` to the `/campaigns/{campaign_id}/send-checklist` endpoint.
    *
    * Review the send checklist for a campaign, and resolve any issues before sending.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn get_send_checklist(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        campaign_id: &str,
    ) -> Result<crate::types::SendChecklist> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/campaigns/{}/send-checklist?{}",
            crate::progenitor_support::encode_path(&campaign_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }
}
