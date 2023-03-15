use crate::Client;
use crate::ClientResult;

pub struct SettingsMail {
    pub client: Client,
}

impl SettingsMail {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SettingsMail { client }
    }

    /**
     * Retrieve all mail settings.
     *
     * This function performs a `GET` to the `/mail_settings` endpoint.
     *
     * **This endpoint allows you to retrieve a list of all mail settings.**
     *
     * Each setting will be returned with an `enabled` status set to `true` or `false` and a short description that explains what the setting does.
     *
     * **Parameters:**
     *
     * * `limit: i64` -- The number of settings to return.
     * * `offset: i64` -- Where in the list of results to begin displaying settings.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_mail_settings(
        &self,
        limit: i64,
        offset: i64,
    ) -> ClientResult<crate::types::GetMailSettingsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/mail_settings?{}", query_), None);
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
     * Retrieve address whitelist mail settings.
     *
     * This function performs a `GET` to the `/mail_settings/address_whitelist` endpoint.
     *
     * **This endpoint allows you to retrieve your current email address whitelist settings.**
     *
     * The Address Whitelist setting allows you to specify email addresses or domains for which mail should never be suppressed.
     *
     * For example, if you own the domain `example.com`, and one or more of your recipients use `email@example.com` addresses, placing `example.com` in the address whitelist setting instructs Twilio SendGrid to ignore all bounces, blocks, and unsubscribes logged for that domain. In other words, all bounces, blocks, and unsubscribes will still be sent to `example.com` as if they were sent under normal sending conditions.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_mail_settings_address_whitelist(
        &self,
    ) -> ClientResult<crate::types::MailSettingsAddressWhitelabel> {
        let url = self.client.url("/mail_settings/address_whitelist", None);
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
     * Update address whitelist mail settings.
     *
     * This function performs a `PATCH` to the `/mail_settings/address_whitelist` endpoint.
     *
     * **This endpoint allows you to update your current email address whitelist settings.**
     *
     * You can select whether or not this setting should be enabled by assigning the `enabled` field a `true` or `false` value.
     *
     * Passing only the `enabled` field to this endpoint will not alter your current `list` of whitelist entries. However, any modifications to your `list` of entries will overwrite the entire list. For this reason, you must included all existing entries you wish to retain in your `list` in addition to any new entries you intend to add. To remove one or more `list` entries, pass a `list` with only the entries you wish to retain.
     *
     * You should not add generic domains such as `gmail.com` or `yahoo.com`  in your `list` because your emails will not honor recipients' unsubscribes. This may cause a legal violation of [CAN-SPAM](https://sendgrid.com/docs/glossary/can-spam/) and could damage your sending reputation.
     *
     * The Address Whitelist setting allows you to specify email addresses or domains for which mail should never be suppressed.
     *
     * For example, if you own the domain `example.com`, and one or more of your recipients use `email@example.com` addresses, placing `example.com` in the address whitelist setting instructs Twilio SendGrid to ignore all bounces, blocks, and unsubscribes logged for that domain. In other words, all bounces, blocks, and unsubscribes will still be sent to `example.com` as if they were sent under normal sending conditions.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_mail_settings_address_whitelist(
        &self,
        body: &crate::types::PatchMailSettingsAddressWhitelistRequest,
    ) -> ClientResult<crate::types::MailSettingsAddressWhitelabel> {
        let url = self.client.url("/mail_settings/address_whitelist", None);
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
     * Retrieve footer mail settings.
     *
     * This function performs a `GET` to the `/mail_settings/footer` endpoint.
     *
     * **This endpoint allows you to retrieve your current Footer mail settings.**
     *
     * The Footer setting will insert a custom footer at the bottom of your text and HTML email message bodies.
     *
     * You can insert your HTML or plain text directly using the "Update footer mail settings" endpoint, or you can create the footer using the [Mail Settings menu in the Twilio SendGrid App](https://app.sendgrid.com/settings/mail_settings).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_mail_settings_footer(&self) -> ClientResult<crate::types::MailSettingsFooter> {
        let url = self.client.url("/mail_settings/footer", None);
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
     * Update footer mail settings.
     *
     * This function performs a `PATCH` to the `/mail_settings/footer` endpoint.
     *
     * **This endpoint allows you to update your current Footer mail settings.**
     *
     * The Footer setting will insert a custom footer at the bottom of your text and HTML email message bodies.
     *
     * You can insert your HTML or plain text directly using this endpoint, or you can create the footer using the [Mail Settings menu in the Twilio SendGrid App](https://app.sendgrid.com/settings/mail_settings).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_mail_settings_footer(
        &self,
        body: &crate::types::MailSettingsFooter,
    ) -> ClientResult<crate::types::MailSettingsFooter> {
        let url = self.client.url("/mail_settings/footer", None);
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
     * Retrieve forward spam mail settings.
     *
     * This function performs a `GET` to the `/mail_settings/forward_spam` endpoint.
     *
     * **This endpoint allows you to retrieve your current Forward Spam mail settings.**
     *
     * Enabling the Forward Spam setting allows you to specify `email` addresses to which spam reports will be forwarded. This endpoint returns any email address(es) you have set to receive forwarded spam and an `enabled` status indicating if the setting is active.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_mail_settings_forward_spam(
        &self,
    ) -> ClientResult<crate::types::MailSettingsForwardSpam> {
        let url = self.client.url("/mail_settings/forward_spam", None);
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
     * Update forward spam mail settings.
     *
     * This function performs a `PATCH` to the `/mail_settings/forward_spam` endpoint.
     *
     * **This endpoint allows you to update your current Forward Spam mail settings.**
     *
     * Enabling the Forward Spam setting allows you to specify `email` addresses to which spam reports will be forwarded. You can set multiple addresses by passing this endpoint a comma separated list of emails in a single string.
     *
     * ```
     * {
     *   "email": "address1@example.com, address2@exapmle.com",
     *   "enabled": true
     * }
     * ```
     *
     * The Forward Spam setting may also be used to receive emails sent to `abuse@` and `postmaster@` role addresses if you have authenticated your domain.
     *
     * For example, if you authenticated `example.com` as your root domain and set a custom return path of `sub` for that domain, you could turn on Forward Spam, and any emails sent to `abuse@sub.example.com` or `postmaster@sub.example.com` would be forwarded to the email address you entered in the `email` field.
     *
     * You can authenticate your domain using the "Authenticate a domain" endpoint or in the [Sender Authentication section of the Twilio SendGrid App](https://app.sendgrid.com/settings/sender_auth). You can also configure the Forward Spam mail settings in the [Mail Settings section of the Twilio SendGrid App](https://app.sendgrid.com/settings/mail_settings).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_mail_settings_forward_spam(
        &self,
        body: &crate::types::MailSettingsForwardSpam,
    ) -> ClientResult<crate::types::MailSettingsForwardSpam> {
        let url = self.client.url("/mail_settings/forward_spam", None);
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
     * Retrieve legacy template mail settings.
     *
     * This function performs a `GET` to the `/mail_settings/template` endpoint.
     *
     * **This endpoint allows you to retrieve your current legacy email template settings.**
     *
     * This setting refers to our original email templates. We currently support more fully featured [Dynamic Transactional Templates](https://sendgrid.com/docs/ui/sending-email/how-to-send-an-email-with-dynamic-transactional-templates/).
     *
     * The legacy email template setting wraps an HTML template around your email content. This can be useful for sending out marketing email and/or other HTML formatted messages. For instructions on using legacy templates, see how to ["Create and Edit Legacy Transactional Templates](https://sendgrid.com/docs/ui/sending-email/create-and-edit-legacy-transactional-templates/). For help migrating to our current template system, see ["Migrating from Legacy Templates"](https://sendgrid.com/docs/ui/sending-email/migrating-from-legacy-templates/).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_mail_settings_template(
        &self,
    ) -> ClientResult<crate::types::MailSettingsTemplate> {
        let url = self.client.url("/mail_settings/template", None);
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
     * Update template mail settings.
     *
     * This function performs a `PATCH` to the `/mail_settings/template` endpoint.
     *
     * **This endpoint allows you to update your current legacy email template settings.**
     *
     * This setting refers to our original email templates. We currently support more fully featured [Dynamic Transactional Templates](https://sendgrid.com/docs/ui/sending-email/how-to-send-an-email-with-dynamic-transactional-templates/).
     *
     * The legacy email template setting wraps an HTML template around your email content. This can be useful for sending out marketing email and/or other HTML formatted messages. For instructions on using legacy templates, see how to ["Create and Edit Legacy Transactional Templates](https://sendgrid.com/docs/ui/sending-email/create-and-edit-legacy-transactional-templates/). For help migrating to our current template system, see ["Migrating from Legacy Templates"](https://sendgrid.com/docs/ui/sending-email/migrating-from-legacy-templates/).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_mail_settings_template(
        &self,
        body: &crate::types::PatchMailSettingsTemplateRequest,
    ) -> ClientResult<crate::types::PatchMailSettingsTemplateResponse> {
        let url = self.client.url("/mail_settings/template", None);
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
     * Retrieve bounce purge mail settings.
     *
     * This function performs a `GET` to the `/mail_settings/bounce_purge` endpoint.
     *
     * **This endpoint allows you to retrieve your current bounce and purge settings.**
     *
     * The Bounce Perge setting allows you to set a schedule that Twilio SendGrid will use to automatically delete contacts from your soft and hard bounce suppression lists.
     *
     * A hard bounce occurs when an email message has been returned to the sender because the recipient's address is invalid. A hard bounce might occur because the domain name doesn't exist or because the recipient is unknown.
     *
     * A soft bounce occurs when an email message reaches the recipient's mail server but is bounced back undelivered before it actually reaches the recipient. A soft bounce might occur because the recipient's inbox is full.
     *
     * You can also manage this setting in the [Mail Settings section of the Twilio SendGrid App](https://app.sendgrid.com/settings/mail_settings). You can manage your bounces manually using the [Bounces API](https://sendgrid.api-docs.io/v3.0/bounces-api) or the [Bounces menu in the Twilio SendGrid App](https://app.sendgrid.com/suppressions/bounces).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_mail_settings_bounce_purge(
        &self,
    ) -> ClientResult<crate::types::MailSettingsBouncePurge> {
        let url = self.client.url("/mail_settings/bounce_purge", None);
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
     * Update bounce purge mail settings.
     *
     * This function performs a `PATCH` to the `/mail_settings/bounce_purge` endpoint.
     *
     * **This endpoint allows you to update your current bounce and purge settings.**
     *
     * The Bounce Perge setting allows you to set a schedule that Twilio SendGrid will use to automatically delete contacts from your soft and hard bounce suppression lists. The schedule is set in full days by assigning the number of days, an integer, to the `soft_bounces` and/or `hard_bounces` fields.
     *
     * A hard bounce occurs when an email message has been returned to the sender because the recipient's address is invalid. A hard bounce might occur because the domain name doesn't exist or because the recipient is unknown.
     *
     * A soft bounce occurs when an email message reaches the recipient's mail server but is bounced back undelivered before it actually reaches the recipient. A soft bounce might occur because the recipient's inbox is full.
     *
     * You can also manage this setting in the [Mail Settings section of the Twilio SendGrid App](https://app.sendgrid.com/settings/mail_settings). You can manage your bounces manually using the [Bounces API](https://sendgrid.api-docs.io/v3.0/bounces-api) or the [Bounces menu in the Twilio SendGrid App](https://app.sendgrid.com/suppressions/bounces).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_mail_settings_bounce_purge(
        &self,
        body: &crate::types::MailSettingsBouncePurge,
    ) -> ClientResult<crate::types::MailSettingsBouncePurge> {
        let url = self.client.url("/mail_settings/bounce_purge", None);
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
     * Retrieve forward bounce mail settings.
     *
     * This function performs a `GET` to the `/mail_settings/forward_bounce` endpoint.
     *
     * **This endpoint allows you to retrieve your current bounce forwarding mail settings.**
     *
     * Enabling the Forward Bounce setting allows you to specify `email` addresses to which bounce reports will be forwarded. This endpoint returns the email address you have set to receive forwarded bounces and an `enabled` status indicating if the setting is active.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_mail_settings_forward_bounce(
        &self,
    ) -> ClientResult<crate::types::MailSettingsForwardBounce> {
        let url = self.client.url("/mail_settings/forward_bounce", None);
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
     * Update forward bounce mail settings.
     *
     * This function performs a `PATCH` to the `/mail_settings/forward_bounce` endpoint.
     *
     * **This endpoint allows you to update your current bounce forwarding mail settings.**
     *
     * Enabling the Forward Bounce setting allows you to specify an `email` address to which bounce reports will be forwarded.
     *
     * You can also configure the Forward Spam mail settings in the [Mail Settings section of the Twilio SendGrid App](https://app.sendgrid.com/settings/mail_settings).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_mail_settings_forward_bounce(
        &self,
        body: &crate::types::MailSettingsForwardBounce,
    ) -> ClientResult<crate::types::MailSettingsForwardBounce> {
        let url = self.client.url("/mail_settings/forward_bounce", None);
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
