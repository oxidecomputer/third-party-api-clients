use crate::Client;
use crate::ClientResult;

pub struct TemplateViews {
    pub client: Client,
}

impl TemplateViews {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TemplateViews { client }
    }

    /**
     * Gets a URL for a template edit view.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/templates/{templateId}/views/edit` endpoint.
     *
     * This method returns a URL for starting an edit view of a template that uses the DocuSign Template UI.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn views_post_template_edit_view(
        &self,
        account_id: &str,
        template_id: &str,
        body: &crate::types::ReturnUrlRequest,
    ) -> ClientResult<crate::types::ViewUrl> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/views/edit",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
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
}
