use crate::Client;
use crate::ClientResult;

pub struct Webhooks {
    pub client: Client,
}

impl Webhooks {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Webhooks { client }
    }

    /**
     * Retrieve Event Webhook settings.
     *
     * This function performs a `GET` to the `/user/webhooks/event/settings` endpoint.
     *
     * **This endpoint allows you to retrieve your current event webhook settings.**
     *
     * If an event type is marked as `true`, then the event webhook will include information about that event.
     *
     * SendGrid’s Event Webhook will notify a URL of your choice via HTTP POST with information about events that occur as SendGrid processes your email.
     *
     * Common uses of this data are to remove unsubscribes, react to spam reports, determine unengaged recipients, identify bounced email addresses, or create advanced analytics of your email program.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_user_event_settings(
        &self,
    ) -> ClientResult<crate::types::WebhooksEventWebhookResponse> {
        let url = self.client.url("/user/webhooks/event/settings", None);
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
     * Update Event Notification Settings.
     *
     * This function performs a `PATCH` to the `/user/webhooks/event/settings` endpoint.
     *
     * **This endpoint allows you to update your current event webhook settings.**
     *
     * If an event type is marked as `true`, then the event webhook will include information about that event.
     *
     * SendGrid’s Event Webhook will notify a URL of your choice via HTTP POST with information about events that occur as SendGrid processes your email.
     *
     * Common uses of this data are to remove unsubscribes, react to spam reports, determine unengaged recipients, identify bounced email addresses, or create advanced analytics of your email program.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_user_event_settings(
        &self,
        body: &crate::types::WebhooksEventWebhookUpdateWithOAuthRequest,
    ) -> ClientResult<crate::types::WebhooksEventWebhookResponse> {
        let url = self.client.url("/user/webhooks/event/settings", None);
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
     * Retrieve all parse settings.
     *
     * This function performs a `GET` to the `/user/webhooks/parse/settings` endpoint.
     *
     * **This endpoint allows you to retrieve all of your current inbound parse settings.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_user_parse_settings(
        &self,
    ) -> ClientResult<crate::types::GetUserWebhooksParseSettingsResponse> {
        let url = self.client.url("/user/webhooks/parse/settings", None);
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
     * Retrieves Inbound Parse Webhook statistics.
     *
     * This function performs a `GET` to the `/user/webhooks/parse/stats` endpoint.
     *
     * **This endpoint allows you to retrieve the statistics for your Parse Webhook useage.**
     *
     * SendGrid's Inbound Parse Webhook allows you to parse the contents and attachments of incomming emails. The Parse API can then POST the parsed emails to a URL that you specify. The Inbound Parse Webhook cannot parse messages greater than 30MB in size, including all attachments.
     *
     * There are a number of pre-made integrations for the SendGrid Parse Webhook which make processing events easy. You can find these integrations in the [Library Index](https://sendgrid.com/docs/Integrate/libraries.html#-Webhook-Libraries).
     *
     * **Parameters:**
     *
     * * `limit: &str` -- The license key provided with your New Relic account.
     * * `offset: &str` -- The license key provided with your New Relic account.
     * * `aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy` -- How to group the statistics. Must be either "day", "week", or "month".
     * * `start_date: &str` -- The starting date of the statistics you want to retrieve. Must be in the format YYYY-MM-DD.
     * * `end_date: &str` -- The end date of the statistics you want to retrieve. Must be in the format YYYY-MM-DD.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_user_parse_stats(
        &self,
        limit: &str,
        offset: &str,
        aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy,
        start_date: &str,
        end_date: &str,
    ) -> ClientResult<Vec<crate::types::GetUserWebhooksParseStatsResponseData>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.to_string().is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
        }
        if !end_date.is_empty() {
            query_args.push(("end_date".to_string(), end_date.to_string()));
        }
        if !limit.is_empty() {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !offset.is_empty() {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !start_date.is_empty() {
            query_args.push(("start_date".to_string(), start_date.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/user/webhooks/parse/stats?{}", query_), None);
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
     * Retrieves Inbound Parse Webhook statistics.
     *
     * This function performs a `GET` to the `/user/webhooks/parse/stats` endpoint.
     *
     * As opposed to `get_user_parse_stats`, this function returns all the pages of the request at once.
     *
     * **This endpoint allows you to retrieve the statistics for your Parse Webhook useage.**
     *
     * SendGrid's Inbound Parse Webhook allows you to parse the contents and attachments of incomming emails. The Parse API can then POST the parsed emails to a URL that you specify. The Inbound Parse Webhook cannot parse messages greater than 30MB in size, including all attachments.
     *
     * There are a number of pre-made integrations for the SendGrid Parse Webhook which make processing events easy. You can find these integrations in the [Library Index](https://sendgrid.com/docs/Integrate/libraries.html#-Webhook-Libraries).
     */
    pub async fn get_all_user_parse_stats(
        &self,
        offset: &str,
        aggregated_by: crate::types::TraitStatsAdvancedBaseQueryStringsAggregatedBy,
        start_date: &str,
        end_date: &str,
    ) -> ClientResult<Vec<crate::types::GetUserWebhooksParseStatsResponseData>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !aggregated_by.to_string().is_empty() {
            query_args.push(("aggregated_by".to_string(), aggregated_by.to_string()));
        }
        if !end_date.is_empty() {
            query_args.push(("end_date".to_string(), end_date.to_string()));
        }
        if !offset.is_empty() {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if !start_date.is_empty() {
            query_args.push(("start_date".to_string(), start_date.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/user/webhooks/parse/stats?{}", query_), None);
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
     * Retrieve Signed Webhook Public Key.
     *
     * This function performs a `GET` to the `/user/webhooks/event/settings/signed` endpoint.
     *
     * **This endpoint allows you to retrieve your signed webhook's public key.**
     *
     * Once you have enabled signing of the Event Webhook, you will need the public key provided to verify the signatures on requests coming from Twilio SendGrid. You can retrieve the public key from this endpoint at any time.
     *
     * For more information about cryptographically signing the Event Webhook, see [Getting Started with the Event Webhook Security Features](https://sendgrid.com/docs/for-developers/tracking-events/getting-started-event-webhook-security-features).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_user_event_settings_signed(
        &self,
    ) -> ClientResult<crate::types::GetUserWebhooksEventSettingsSignedResponse> {
        let url = self
            .client
            .url("/user/webhooks/event/settings/signed", None);
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
     * Enable/Disable Signed Webhook.
     *
     * This function performs a `PATCH` to the `/user/webhooks/event/settings/signed` endpoint.
     *
     * **This endpoint allows you to enable or disable signing of the Event Webhook.**
     *
     * This endpoint takes a single boolean request parameter, `enabled`. You may either enable or disable signing of the Event Webhook using this endpoint. Once enabled, you can retrieve your public key using the `/webhooks/event/settings/signed` endpoint.
     *
     * For more information about cryptographically signing the Event Webhook, see [Getting Started with the Event Webhook Security Features](https://sendgrid.com/docs/for-developers/tracking-events/getting-started-event-webhook-security-features).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_user_event_settings_signed(
        &self,
        body: &crate::types::GetTrackingSettingsOpenResponse,
    ) -> ClientResult<crate::types::GetUserWebhooksEventSettingsSignedResponse> {
        let url = self
            .client
            .url("/user/webhooks/event/settings/signed", None);
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
     * Test Event Notification Settings.
     *
     * This function performs a `POST` to the `/user/webhooks/event/test` endpoint.
     *
     * **This endpoint allows you to test your event webhook by sending a fake event notification post to the provided URL.**
     *
     * SendGrid’s Event Webhook will notify a URL of your choice via HTTP POST with information about events that occur as SendGrid processes your email.
     *
     * Common uses of this data are to remove unsubscribes, react to spam reports, determine unengaged recipients, identify bounced email addresses, or create advanced analytics of your email program.
     *
     * >**Tip**: Retry logic for this endpoint differs from other endpoints, which use a rolling 24-hour retry.
     *
     * If your web server does not return a 2xx response type, we will retry a POST request until we receive a 2xx response or the maximum time of 10 minutes has expired.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_user_event_test(
        &self,
        body: &crate::types::PostUserWebhooksEventTestRequest,
    ) -> ClientResult<()> {
        let url = self.client.url("/user/webhooks/event/test", None);
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
