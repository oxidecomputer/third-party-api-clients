use crate::Client;
use crate::ClientResult;

pub struct MailSend {
    pub client: Client,
}

impl MailSend {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        MailSend { client }
    }

    /**
     * v3 Mail Send.
     *
     * This function performs a `POST` to the `/mail/send` endpoint.
     *
     * The Mail Send endpoint allows you to send email over SendGrid’s v3 Web API, the most recent version of our API. If you are looking for documentation about the v2 Mail Send endpoint, see our [v2 API Reference](https://sendgrid.com/docs/API_Reference/Web_API/mail.html).
     *
     * ## Helper Libraries
     *
     * Twilio SendGrid provides libraries to help you quickly and easily integrate with the v3 Web API in 7 different languages:
     *
     * * [C#](https://github.com/sendgrid/sendgrid-csharp)
     * * [Go](https://github.com/sendgrid/sendgrid-go)
     * * [Java](https://github.com/sendgrid/sendgrid-java)
     * * [Node JS](https://github.com/sendgrid/sendgrid-nodejs)
     * * [PHP](https://github.com/sendgrid/sendgrid-php)
     * * [Python](https://github.com/sendgrid/sendgrid-python)
     * * [Ruby](https://github.com/sendgrid/sendgrid-ruby)
     *
     * ## Dynamic Transactional Templates and Handlebars
     *
     * In order to send a dynamic template, specify the template ID with the `template_id` parameter.
     *
     * To specify handlebar substitutions, define your substitutions in the request JSON with this syntax:
     *
     * ```
     * "dynamic_template_data": {
     *       "guest": "Jane Doe",
     *       "partysize": "4",
     *       "english": true,
     *       "date": "April 1st, 2021"
     *     }
     * ```
     *
     * For more information about Dynamic Transactional Templates and Handlebars, see our documentation and reference pages.
     *
     * * [How to send an email with Dynamic Transactional Templates
     * ](https://sendgrid.com/docs/ui/sending-email/how-to-send-an-email-with-dynamic-transactional-templates/)
     * * [Using Handlebars](https://sendgrid.com/docs/for-developers/sending-email/using-handlebars/)
     *
     * ## Mail body compression
     *
     * Mail body compression is available to some high volume accounts. Talk to your CSM if you are interested in this functionality. Mail body compression works by setting up a JSON payload as defined on this page, then compressing it with gzip (the gzip file can be no more than 30mb).
     *
     * To use mail body compression:
     *
     * 1. Add a `Content-Encoding` header, with a value of `gzip`.  
     *    a. `Content-Encoding: gzip`
     * 2. Send the gzip as a data-binary.  
     *    a. `--data-binary '@data.json.gz'
     * `
     */
    pub async fn post(&self, body: &crate::types::PostMailSendRequest) -> ClientResult<()> {
        let url = self.client.url("/mail/send", None);
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
