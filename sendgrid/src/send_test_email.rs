use crate::Client;
use crate::ClientResult;

pub struct SendTestEmail {
    pub client: Client,
}

impl SendTestEmail {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SendTestEmail { client }
    }

    /**
     * Send a Test Marketing Email.
     *
     * This function performs a `POST` to the `/marketing/test/send_email` endpoint.
     *
     * **This endpoint allows you to send a test marketing email to a list of email addresses**.
     *
     * Before sending a marketing message, you can test it using this endpoint. You may specify up to **10 contacts** in the `emails` request body field. You must also specify a `template_id` and include either a `from_address` or `sender_id`. You can manage your templates with the [Twilio SendGrid App](https://mc.sendgrid.com/dynamic-templates) or the [Transactional Templates API](https://sendgrid.api-docs.io/v3.0/transactional-templates).
     *
     * > Please note that this endpoint works with Dynamic Transactional Templates only. Legacy Transactional Templates will not be delivered.
     *
     * For more information about managing Dynamic Transactional Templates, see [How to Send Email with Dynamic Transactional Templates](https://sendgrid.com/docs/ui/sending-email/how-to-send-an-email-with-dynamic-transactional-templates/).
     *
     * You can also test your Single Sends in the [Twilio SendGrid Marketing Campaigns UI](https://mc.sendgrid.com/single-sends).
     */
    pub async fn post_marketing_test_send_email(
        &self,
        body: &crate::types::PostMarketingTestSendEmailRequest,
    ) -> ClientResult<crate::types::Help> {
        let url = self.client.url("/marketing/test/send_email", None);
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
