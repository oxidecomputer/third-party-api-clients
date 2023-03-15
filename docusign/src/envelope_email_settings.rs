use crate::Client;
use crate::ClientResult;

pub struct EnvelopeEmailSettings {
    pub client: Client,
}

impl EnvelopeEmailSettings {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EnvelopeEmailSettings { client }
    }

    /**
     * Gets the email setting overrides for an envelope.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/email_settings` endpoint.
     *
     * Retrieves the email override settings for the specified envelope.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn email_settings_get(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<crate::types::EmailSettings> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/email_settings",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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
     * Updates the email setting overrides for an envelope.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/email_settings` endpoint.
     *
     * Updates the existing email override settings for the specified envelope. Note that modifying email settings will only affect email communications that occur after the modification was made.
     *
     * This can also be used to delete an individual email override setting by using an empty string for the value to be deleted.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn email_settings_put(
        &self,
        account_id: &str,
        envelope_id: &str,
        body: &crate::types::EmailSettings,
    ) -> ClientResult<crate::types::EmailSettings> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/email_settings",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Adds email setting overrides to an envelope.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/email_settings` endpoint.
     *
     * Adds email override settings, changing the email address to reply to an email address, name, or the BCC for email archive information, for the envelope. Note that adding email settings will only affect email communications that occur after the addition was made.
     *
     * ### Important: The BCC Email address feature is designed to provide a copy of all email communications for external archiving purposes. DocuSign recommends that envelopes sent using the BCC for Email Archive feature, including the BCC Email Override option, include additional signer authentication options. To send a copy of the envelope to a recipient who does not need to sign, use a Carbon Copy or Certified Delivery recipient type.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn email_settings_post(
        &self,
        account_id: &str,
        envelope_id: &str,
        body: &crate::types::EmailSettings,
    ) -> ClientResult<crate::types::EmailSettings> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/email_settings",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Deletes the email setting overrides for an envelope.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/email_settings` endpoint.
     *
     * Deletes all existing email override settings for the envelope. If you want to delete an individual email override setting, use the PUT and set the value to an empty string. Note that deleting email settings will only affect email communications that occur after the deletion and the normal account email settings are used for future email communications.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn email_settings_delete(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<crate::types::EmailSettings> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/email_settings",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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
}
