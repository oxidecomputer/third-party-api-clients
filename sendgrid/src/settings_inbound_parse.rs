use anyhow::Result;

use crate::Client;

pub struct SettingsInboundParse {
    pub client: Client,
}

impl SettingsInboundParse {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SettingsInboundParse { client }
    }

    /**
     * Create a parse setting.
     *
     * This function performs a `POST` to the `/user/webhooks/parse/settings` endpoint.
     *
     * **This endpoint allows you to create a new inbound parse setting.**
     *
     * Creating an Inbound Parse setting requires two pieces of information: a `url` and a `hostname`.
     *
     * The `hostname` must correspond to a domain authenticated by Twilio SendGrid on your account. If you need to complete domain authentication, you can use the [Twilio SendGrid App](https://app.sendgrid.com/settings/sender_auth) or the "Authenticate a domain" endpoint. See "[How to Set Up Domain Authentication](https://sendgrid.com/docs/ui/account-and-settings/how-to-set-up-domain-authentication/)" for instructions.
     *
     * Any email received by the `hostname` will be parsed when you complete this setup. You must also add a Twilio SendGrid MX record to this domain's DNS records. See "[Setting up the Inbound Parse Webhook](https://sendgrid.com/docs/for-developers/parsing-email/setting-up-the-inbound-parse-webhook/)" for full instructions.
     *
     * The `url` represents a location where the parsed message data will be delivered. Twilio SendGrid will make an HTTP POST request to this `url` with the message data. The `url` must be publicly reachable, and your application must return a `200` status code to signal that the message data has been received.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_user_webhooks_parse_setting(
        &self,
        body: &crate::types::ParseSetting,
    ) -> Result<crate::types::ParseSetting> {
        let url = "/user/webhooks/parse/settings".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Retrieve a specific parse setting.
     *
     * This function performs a `GET` to the `/user/webhooks/parse/settings/{hostname}` endpoint.
     *
     * **This endpoint allows you to retrieve a specific inbound parse setting by hostname.**
     *
     * You can retrieve all your Inbound Parse settings and their associated host names with the "Retrieve all parse settings" endpoint.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_user_webhooks_parse_settings_hostname(
        &self,
        hostname: &str,
    ) -> Result<crate::types::ParseSetting> {
        let url = format!(
            "/user/webhooks/parse/settings/{}",
            crate::progenitor_support::encode_path(&hostname.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a parse setting.
     *
     * This function performs a `DELETE` to the `/user/webhooks/parse/settings/{hostname}` endpoint.
     *
     * **This endpoint allows you to delete a specific inbound parse setting by hostname.**
     *
     * You can retrieve all your Inbound Parse settings and their associated host names with the "Retrieve all parse settings" endpoint.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_user_webhooks_parse_settings_hostname(
        &self,
        hostname: &str,
    ) -> Result<crate::types::Help> {
        let url = format!(
            "/user/webhooks/parse/settings/{}",
            crate::progenitor_support::encode_path(&hostname.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update a parse setting.
     *
     * This function performs a `PATCH` to the `/user/webhooks/parse/settings/{hostname}` endpoint.
     *
     * **This endpoint allows you to update a specific inbound parse setting by hostname.**
     *
     * You can retrieve all your Inbound Parse settings and their associated host names with the "Retrieve all parse settings" endpoint.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_user_webhooks_parse_settings_hostname(
        &self,
        hostname: &str,
        body: &crate::types::ParseSetting,
    ) -> Result<crate::types::ParseSetting> {
        let url = format!(
            "/user/webhooks/parse/settings/{}",
            crate::progenitor_support::encode_path(&hostname.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
