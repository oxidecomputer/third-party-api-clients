use anyhow::Result;

use crate::Client;

pub struct TemplateResponsiveHtmlPreview {
    client: Client,
}

impl TemplateResponsiveHtmlPreview {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        TemplateResponsiveHtmlPreview {
            client,
        }
    }

    /**
* Creates a preview of the responsive versions of all of the documents associated with a template.
*
* This function performs a `POST` to the `/v2.1/accounts/{accountId}/templates/{templateId}/responsive_html_preview` endpoint.
*
* Creates a preview of the
* [responsive](https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/responsive/),
* HTML versions of all of the documents associated
* with a template. This method enables you to
* preview the PDF document conversions to responsive
* HTML across device types prior to sending.
* 
* The request body is a `documentHtmlDefinition`
* object, which holds the responsive signing
* parameters that define how to generate the HTML
* version of the documents.
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
*/
pub async fn responsive_html_post_template_preview(
&self,
account_id: &str, template_id: &str,
body: &crate::types::DocumentHtmlDefinition
) -> Result<crate::types::DocumentHtmlDefinitions> {
let url =
format!("/v2.1/accounts/{}/templates/{}/responsive_html_preview",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&template_id.to_string()),);

self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}


}