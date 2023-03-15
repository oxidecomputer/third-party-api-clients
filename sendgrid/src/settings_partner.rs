use crate::Client;
use crate::ClientResult;

pub struct SettingsPartner {
    pub client: Client,
}

impl SettingsPartner {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SettingsPartner { client }
    }

    /**
     * Returns all New Relic partner settings.
     *
     * This function performs a `GET` to the `/partner_settings/new_relic` endpoint.
     *
     * **This endpoint allows you to retrieve your current New Relic partner settings.**
     *
     * Our partner settings allow you to integrate your SendGrid account with our partners to increase your SendGrid experience and functionality. For more information about our partners, and how you can begin integrating with them, please visit our [Partners documentation](https://sendgrid.com/docs/ui/account-and-settings/partners/).
     *
     * By integrating with New Relic, you can send your SendGrid email statistics to your New Relic Dashboard. If you enable this setting, your stats will be sent to New Relic every 5 minutes. You will need your New Relic License Key to enable this setting. For more information, please see our [SendGrid for New Relic documentation](https://sendgrid.com/docs/ui/analytics-and-reporting/tracking-stats-using-new-relic/).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_partner_settings_new_relic(
        &self,
    ) -> ClientResult<crate::types::PartnerSettingsNewRelic> {
        let url = self.client.url("/partner_settings/new_relic", None);
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
     * Updates New Relic partner settings.
     *
     * This function performs a `PATCH` to the `/partner_settings/new_relic` endpoint.
     *
     * **This endpoint allows you to update or change your New Relic partner settings.**
     *
     * Our partner settings allow you to integrate your SendGrid account with our partners to increase your SendGrid experience and functionality. For more information about our partners, and how you can begin integrating with them, please visit our [Partners documentation](https://sendgrid.com/docs/ui/account-and-settings/partners/).
     *
     * By integrating with New Relic, you can send your SendGrid email statistics to your New Relic Dashboard. If you enable this setting, your stats will be sent to New Relic every 5 minutes. You will need your New Relic License Key to enable this setting. For more information, please see our [SendGrid for New Relic documentation](https://sendgrid.com/docs/ui/analytics-and-reporting/tracking-stats-using-new-relic/).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_partner_settings_new_relic(
        &self,
        body: &crate::types::PatchPartnerSettingsNewRelicRequest,
    ) -> ClientResult<crate::types::PartnerSettingsNewRelic> {
        let url = self.client.url("/partner_settings/new_relic", None);
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
     * Returns a list of all partner settings.
     *
     * This function performs a `GET` to the `/partner_settings` endpoint.
     *
     * **This endpoint allows you to retrieve a list of all partner settings that you can enable.**
     *
     * Our partner settings allow you to integrate your SendGrid account with our partners to increase your SendGrid experience and functionality. For more information about our partners, and how you can begin integrating with them, please visit our [Partners documentation](https://sendgrid.com/docs/ui/account-and-settings/partners/).
     *
     * **Parameters:**
     *
     * * `limit: i64` -- The number of settings to return per page.
     * * `offset: i64` -- The paging offset.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_partner_settings(
        &self,
        limit: i64,
        offset: i64,
    ) -> ClientResult<crate::types::GetPartnerSettingsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/partner_settings?{}", query_), None);
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
