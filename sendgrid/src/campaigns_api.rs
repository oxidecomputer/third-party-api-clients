use crate::Client;
use crate::ClientResult;

pub struct CampaignsApi {
    pub client: Client,
}

impl CampaignsApi {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CampaignsApi { client }
    }

    /**
     * Retrieve all Campaigns.
     *
     * This function performs a `GET` to the `/campaigns` endpoint.
     *
     * **This endpoint allows you to retrieve a list of all of your campaigns.**
     *
     * Returns campaigns in reverse order they were created (newest first).
     *
     * Returns an empty array if no campaigns exist.
     *
     * **Parameters:**
     *
     * * `limit: i64` -- The number of results you would like to receive at a time.
     * * `offset: i64` -- The index of the first campaign to return, where 0 is the first campaign.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_campaigns(
        &self,
        limit: i64,
        offset: i64,
    ) -> ClientResult<crate::types::GetCampaignsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/campaigns?{}", query_), None);
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
     * Create a Campaign.
     *
     * This function performs a `POST` to the `/campaigns` endpoint.
     *
     * **This endpoint allows you to create a campaign.**
     *
     * In order to send or schedule the campaign, you will be required to provide a subject, sender ID, content (we suggest both html and plain text), and at least one list or segment ID. This information is not required when you create a campaign.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_campaign(
        &self,
        body: &crate::types::CampaignsRequest,
    ) -> ClientResult<crate::types::CampaignResponseAllOf> {
        let url = self.client.url("/campaigns", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieve a single campaign.
     *
     * This function performs a `GET` to the `/campaigns/{campaign_id}` endpoint.
     *
     * **This endpoint allows you to retrieve a specific campaign.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_campaigns_campaign(
        &self,
        campaign_id: i64,
    ) -> ClientResult<crate::types::GetCampaignsCampaignResponse> {
        let url = self.client.url(
            &format!(
                "/campaigns/{}",
                crate::progenitor_support::encode_path(&campaign_id.to_string()),
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
     * Delete a Campaign.
     *
     * This function performs a `DELETE` to the `/campaigns/{campaign_id}` endpoint.
     *
     * **This endpoint allows you to delete a specific campaign.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_campaigns_campaign(&self, campaign_id: i64) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/campaigns/{}",
                crate::progenitor_support::encode_path(&campaign_id.to_string()),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Update a Campaign.
     *
     * This function performs a `PATCH` to the `/campaigns/{campaign_id}` endpoint.
     *
     * **This endpoint allows you to update a specific campaign.**
     *
     * This is especially useful if you only set up the campaign using POST /campaigns, but didn't set many of the parameters.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_campaigns_campaign(
        &self,
        campaign_id: i64,
        body: &crate::types::UpdateACampaignRequest,
    ) -> ClientResult<crate::types::CampaignResponseAllOf> {
        let url = self.client.url(
            &format!(
                "/campaigns/{}",
                crate::progenitor_support::encode_path(&campaign_id.to_string()),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Send a Campaign.
     *
     * This function performs a `POST` to the `/campaigns/{campaign_id}/schedules/now` endpoint.
     *
     * **This endpoint allows you to immediately send an existing campaign.**
     *
     * Normally a POST request would have a body, but since this endpoint is telling us to send a resource that is already created, a request body is not needed.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_campaigns_campaign_schedules_now(
        &self,
        campaign_id: i64,
    ) -> ClientResult<crate::types::SendACampaignResponse> {
        let url = self.client.url(
            &format!(
                "/campaigns/{}/schedules/now",
                crate::progenitor_support::encode_path(&campaign_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * View Scheduled Time of a Campaign.
     *
     * This function performs a `GET` to the `/campaigns/{campaign_id}/schedules` endpoint.
     *
     * **This endpoint allows you to retrieve the date and time that a campaign has been scheduled to be sent.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_campaigns_campaign_schedule(
        &self,
        campaign_id: i64,
    ) -> ClientResult<crate::types::ScheduleACampaignRequest> {
        let url = self.client.url(
            &format!(
                "/campaigns/{}/schedules",
                crate::progenitor_support::encode_path(&campaign_id.to_string()),
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
     * Schedule a Campaign.
     *
     * This function performs a `POST` to the `/campaigns/{campaign_id}/schedules` endpoint.
     *
     * **This endpoint allows you to schedule a specific date and time for your campaign to be sent.**
     *
     * If you have the flexibility, it's better to schedule mail for off-peak times. Most emails are scheduled and sent at the top of the hour or half hour. Scheduling email to avoid those times (for example, scheduling at 10:53) can result in lower deferral rates because it won't be going through our servers at the same times as everyone else's mail.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_campaigns_campaign_schedule(
        &self,
        campaign_id: i64,
        body: &crate::types::ScheduleACampaignRequest,
    ) -> ClientResult<crate::types::ScheduleACampaignResponse> {
        let url = self.client.url(
            &format!(
                "/campaigns/{}/schedules",
                crate::progenitor_support::encode_path(&campaign_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Unschedule a Scheduled Campaign.
     *
     * This function performs a `DELETE` to the `/campaigns/{campaign_id}/schedules` endpoint.
     *
     * **This endpoint allows you to unschedule a campaign that has already been scheduled to be sent.**
     *
     * A successful unschedule will return a 204.
     * If the specified campaign is in the process of being sent, the only option is to cancel (a different method).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_campaigns_campaign_schedules(&self, campaign_id: i64) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/campaigns/{}/schedules",
                crate::progenitor_support::encode_path(&campaign_id.to_string()),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Update a Scheduled Campaign.
     *
     * This function performs a `PATCH` to the `/campaigns/{campaign_id}/schedules` endpoint.
     *
     * **This endpoint allows to you change the scheduled time and date for a campaign to be sent.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_campaigns_campaign_schedules(
        &self,
        campaign_id: i64,
        body: &crate::types::ScheduleACampaignRequest,
    ) -> ClientResult<crate::types::UpdateAScheduledCampaignResponse> {
        let url = self.client.url(
            &format!(
                "/campaigns/{}/schedules",
                crate::progenitor_support::encode_path(&campaign_id.to_string()),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Send a Test Campaign.
     *
     * This function performs a `POST` to the `/campaigns/{campaign_id}/schedules/test` endpoint.
     *
     * **This endpoint allows you to send a test campaign.**
     *
     * To send to multiple addresses, use an array for the JSON "to" value ["one@address","two@address"]
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_campaigns_campaign_schedules_test(
        &self,
        campaign_id: i64,
        body: &crate::types::SendATestCampaignRequest,
    ) -> ClientResult<crate::types::SendATestCampaignRequest> {
        let url = self.client.url(
            &format!(
                "/campaigns/{}/schedules/test",
                crate::progenitor_support::encode_path(&campaign_id.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
}
