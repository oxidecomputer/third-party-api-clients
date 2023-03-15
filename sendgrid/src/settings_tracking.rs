use crate::Client;
use crate::ClientResult;

pub struct SettingsTracking {
    pub client: Client,
}

impl SettingsTracking {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SettingsTracking { client }
    }

    /**
     * Retrieve Tracking Settings.
     *
     * This function performs a `GET` to the `/tracking_settings` endpoint.
     *
     * **This endpoint allows you to retrieve a list of all tracking settings on your account.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_tracking_settings(
        &self,
    ) -> ClientResult<crate::types::GetTrackingSettingsResponse> {
        let url = self.client.url("/tracking_settings", None);
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
     * Retrieve Click Track Settings.
     *
     * This function performs a `GET` to the `/tracking_settings/click` endpoint.
     *
     * **This endpoint allows you to retrieve your current click tracking setting.**
     *
     * Click Tracking overrides all the links and URLs in your emails and points them to either SendGrid’s servers or the domain with which you branded your link. When a customer clicks a link, SendGrid tracks those [clicks](https://sendgrid.com/docs/glossary/clicks/).
     *
     * Click tracking helps you understand how users are engaging with your communications. SendGrid can track up to 1000 links per email
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_tracking_settings_click(&self) -> ClientResult<crate::types::ClickTracking> {
        let url = self.client.url("/tracking_settings/click", None);
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
     * Update Click Tracking Settings.
     *
     * This function performs a `PATCH` to the `/tracking_settings/click` endpoint.
     *
     * **This endpoint allows you to enable or disable your current click tracking setting.**
     *
     * Click Tracking overrides all the links and URLs in your emails and points them to either SendGrid’s servers or the domain with which you branded your link. When a customer clicks a link, SendGrid tracks those [clicks](https://sendgrid.com/docs/glossary/clicks/).
     *
     * Click tracking helps you understand how users are engaging with your communications. SendGrid can track up to 1000 links per email
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_tracking_settings_click(
        &self,
        body: &crate::types::PatchTrackingSettingsOpenRequest,
    ) -> ClientResult<crate::types::ClickTracking> {
        let url = self.client.url("/tracking_settings/click", None);
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
     * Retrieve Google Analytics Settings.
     *
     * This function performs a `GET` to the `/tracking_settings/google_analytics` endpoint.
     *
     * **This endpoint allows you to retrieve your current setting for Google Analytics.**
     *
     *
     * Google Analytics helps you understand how users got to your site and what they're doing there. For more information about using Google Analytics, please refer to [Google’s URL Builder](https://support.google.com/analytics/answer/1033867?hl=en) and their article on ["Best Practices for Campaign Building"](https://support.google.com/analytics/answer/1037445).
     *
     * We default the settings to Google’s recommendations. For more information, see [Google Analytics Demystified](https://sendgrid.com/docs/ui/analytics-and-reporting/google-analytics/).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_tracking_settings_google_analytic(
        &self,
    ) -> ClientResult<crate::types::GoogleAnalyticsSettings> {
        let url = self.client.url("/tracking_settings/google_analytics", None);
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
     * Update Google Analytics Settings.
     *
     * This function performs a `PATCH` to the `/tracking_settings/google_analytics` endpoint.
     *
     * **This endpoint allows you to update your current setting for Google Analytics.**
     *
     * Google Analytics helps you understand how users got to your site and what they're doing there. For more information about using Google Analytics, please refer to [Google’s URL Builder](https://support.google.com/analytics/answer/1033867?hl=en) and their article on ["Best Practices for Campaign Building"](https://support.google.com/analytics/answer/1037445).
     *
     * We default the settings to Google’s recommendations. For more information, see [Google Analytics Demystified](https://sendgrid.com/docs/ui/analytics-and-reporting/google-analytics/).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_tracking_settings_google_analytics(
        &self,
        body: &crate::types::GoogleAnalyticsSettings,
    ) -> ClientResult<crate::types::GoogleAnalyticsSettings> {
        let url = self.client.url("/tracking_settings/google_analytics", None);
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
     * Get Open Tracking Settings.
     *
     * This function performs a `GET` to the `/tracking_settings/open` endpoint.
     *
     * **This endpoint allows you to retrieve your current settings for open tracking.**
     *
     * Open Tracking adds an invisible image at the end of the email which can track email opens.
     *
     * If the email recipient has images enabled on their email client, a request to SendGrid’s server for the invisible image is executed and an open event is logged.
     *
     * These events are logged in the Statistics portal, Email Activity interface, and are reported by the Event Webhook.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_tracking_settings_open(
        &self,
    ) -> ClientResult<crate::types::GetTrackingSettingsOpenResponse> {
        let url = self.client.url("/tracking_settings/open", None);
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
     * Update Open Tracking Settings.
     *
     * This function performs a `PATCH` to the `/tracking_settings/open` endpoint.
     *
     * **This endpoint allows you to update your current settings for open tracking.**
     *
     * Open Tracking adds an invisible image at the end of the email which can track email opens.
     *
     * If the email recipient has images enabled on their email client, a request to SendGrid’s server for the invisible image is executed and an open event is logged.
     *
     * These events are logged in the Statistics portal, Email Activity interface, and are reported by the Event Webhook.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_tracking_settings_open(
        &self,
        body: &crate::types::PatchTrackingSettingsOpenRequest,
    ) -> ClientResult<crate::types::GetTrackingSettingsOpenResponse> {
        let url = self.client.url("/tracking_settings/open", None);
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
     * Retrieve Subscription Tracking Settings.
     *
     * This function performs a `GET` to the `/tracking_settings/subscription` endpoint.
     *
     * **This endpoint allows you to retrieve your current settings for subscription tracking.**
     *
     * Subscription tracking adds links to the bottom of your emails that allows your recipients to subscribe to, or unsubscribe from, your emails.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_tracking_settings_subscription(
        &self,
    ) -> ClientResult<crate::types::SubscriptionTrackingSettings> {
        let url = self.client.url("/tracking_settings/subscription", None);
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
     * Update Subscription Tracking Settings.
     *
     * This function performs a `PATCH` to the `/tracking_settings/subscription` endpoint.
     *
     * **This endpoint allows you to update your current settings for subscription tracking.**
     *
     * Subscription tracking adds links to the bottom of your emails that allows your recipients to subscribe to, or unsubscribe from, your emails.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_tracking_settings_subscription(
        &self,
        body: &crate::types::SubscriptionTrackingSettings,
    ) -> ClientResult<crate::types::SubscriptionTrackingSettings> {
        let url = self.client.url("/tracking_settings/subscription", None);
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
}
