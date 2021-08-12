use anyhow::Result;

use crate::Client;

pub struct TemplateHtmlDefinitions {
    client: Client,
}

impl TemplateHtmlDefinitions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TemplateHtmlDefinitions { client }
    }

    /**
     * Gets the Original HTML Definition used to generate the Responsive HTML for the template.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/templates/{templateId}/html_definitions` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn responsive_html_get_template_definition(
        &self,
        account_id: &str,
        template_id: &str,
    ) -> Result<crate::types::EnvelopeHtmlDefinitions> {
        let url = format!(
            "/v2.1/accounts/{}/templates/{}/html_definitions",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&template_id.to_string()),
        );

        self.client.get(&url, None).await
    }
}
