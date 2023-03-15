use crate::Client;
use crate::ClientResult;

pub struct EmailCnameRecords {
    pub client: Client,
}

impl EmailCnameRecords {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EmailCnameRecords { client }
    }

    /**
     * Email DNS records to a co-worker.
     *
     * This function performs a `POST` to the `/whitelabel/dns/email` endpoint.
     *
     * **This endpoint is used to share DNS records with a colleagues**
     *
     * Use this endpoint to send SendGrid-generated DNS record information to a co-worker so they can enter it into your DNS provider to validate your domain and link branding.
     *
     * What type of records are sent will depend on whether you have chosen Automated Security or not. When using Automated Security, SendGrid provides you with three CNAME records. If you turn Automated Security off, you are instead given TXT and MX records.
     *
     * If you pass a `link_id` to this endpoint, the generated email will supply the DNS records necessary to complete [Link Branding](https://sendgrid.com/docs/ui/account-and-settings/how-to-set-up-link-branding/) setup. If you pass a `domain_id` to this endpoint, the generated email will supply the DNS records needed to complete [Domain Authentication](https://sendgrid.com/docs/ui/account-and-settings/how-to-set-up-domain-authentication/). Passing both IDs will generate an email with the records needed to complete both setup steps.
     *
     * You can retrieve all your domain IDs from the returned `id` fields for each domain using the "List all authenticated domains" endpoint. You can retrieve all of your link IDs using the "Retrieve all branded links" endpoint.
     */
    pub async fn post_whitelabel_dns_email(
        &self,
        body: &crate::types::PostWhitelabelDnsEmailRequest,
    ) -> ClientResult<()> {
        let url = self.client.url("/whitelabel/dns/email", None);
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
