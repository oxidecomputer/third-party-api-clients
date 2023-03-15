use crate::Client;
use crate::ClientResult;

pub struct SettingsEnforcedTls {
    pub client: Client,
}

impl SettingsEnforcedTls {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SettingsEnforcedTls { client }
    }

    /**
     * Retrieve current Enforced TLS settings.
     *
     * This function performs a `GET` to the `/user/settings/enforced_tls` endpoint.
     *
     * **This endpoint allows you to retrieve your current Enforced TLS settings.**
     *
     * The Enforced TLS settings specify whether or not the recipient is required to support TLS or have a valid certificate.
     *
     * If either `require_tls` or `require_valid_cert` is set to `true`, the recipient must support TLS 1.1 or higher or have a valid certificate. If these conditions are not met, Twilio SendGrid will drop the message and send a block event with “TLS required but not supported” as the description.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_user(&self) -> ClientResult<crate::types::EnforcedTlsRequestResponse> {
        let url = self.client.url("/user/settings/enforced_tls", None);
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
     * Update Enforced TLS settings.
     *
     * This function performs a `PATCH` to the `/user/settings/enforced_tls` endpoint.
     *
     * **This endpoint allows you to update your Enforced TLS settings.**
     *
     * To require TLS from recipients, set `require_tls` to `true`. If either `require_tls` or `require_valid_cert` is set to `true`, the recipient must support TLS 1.1 or higher or have a valid certificate. If these conditions are not met, Twilio SendGrid will drop the message and send a block event with “TLS required but not supported” as the description.
     *
     * > Twilio SendGrid supports TLS 1.1 and higher and does not support older versions of TLS due to security vulnerabilities.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_user(
        &self,
        body: &crate::types::EnforcedTlsRequestResponse,
    ) -> ClientResult<crate::types::EnforcedTlsRequestResponse> {
        let url = self.client.url("/user/settings/enforced_tls", None);
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
