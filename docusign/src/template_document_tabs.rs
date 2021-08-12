use anyhow::Result;

use crate::Client;

pub struct TemplateDocumentTabs {
    client: Client,
}

impl TemplateDocumentTabs {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TemplateDocumentTabs { client }
    }

    /**
     * Returns tabs on the specified page.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/templates/{templateId}/documents/{documentId}/pages/{pageNumber}/tabs` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `page_number: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn tabs_get_template_page(
        &self,
        account_id: &str,
        document_id: &str,
        page_number: &str,
        template_id: &str,
    ) -> Result<crate::types::TemplateTabs> {
        let url = format!(
            "/v2.1/accounts/{}/templates/{}/documents/{}/pages/{}/tabs",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&template_id.to_string()),
            crate::progenitor_support::encode_path(&document_id.to_string()),
            crate::progenitor_support::encode_path(&page_number.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Returns tabs on the document.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/templates/{templateId}/documents/{documentId}/tabs` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `page_numbers: &str` -- Filters for tabs that occur on the pages that you specify. Enter as a comma-separated list of page Guids.
     *   
     *   Example: `page_numbers=2,6`.
     */
    pub async fn tabs_get_template_document(
        &self,
        account_id: &str,
        document_id: &str,
        template_id: &str,
        page_numbers: &str,
    ) -> Result<crate::types::TemplateTabs> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !page_numbers.is_empty() {
            query_args.push(format!("page_numbers={}", page_numbers));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/v2.1/accounts/{}/templates/{}/documents/{}/tabs?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&template_id.to_string()),
            crate::progenitor_support::encode_path(&document_id.to_string()),
            query
        );

        self.client.get(&url, None).await
    }

    /**
     * Updates the tabs for a template.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/templates/{templateId}/documents/{documentId}/tabs` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn tabs_put_template_document(
        &self,
        account_id: &str,
        document_id: &str,
        template_id: &str,
        body: &crate::types::TemplateTabs,
    ) -> Result<crate::types::Tabs> {
        let url = format!(
            "/v2.1/accounts/{}/templates/{}/documents/{}/tabs",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&template_id.to_string()),
            crate::progenitor_support::encode_path(&document_id.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Create Template Document Tabs.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/templates/{templateId}/documents/{documentId}/tabs` endpoint.
     *
     * This method creates Template Document Tabs.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn tabs_post_template_document(
        &self,
        account_id: &str,
        document_id: &str,
        template_id: &str,
        body: &crate::types::TemplateTabs,
    ) -> Result<crate::types::Tabs> {
        let url = format!(
            "/v2.1/accounts/{}/templates/{}/documents/{}/tabs",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&template_id.to_string()),
            crate::progenitor_support::encode_path(&document_id.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Deletes tabs from an envelope document.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/templates/{templateId}/documents/{documentId}/tabs` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `template_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn tabs_delete_template_document(
        &self,
        account_id: &str,
        document_id: &str,
        template_id: &str,
        body: &crate::types::TemplateTabs,
    ) -> Result<crate::types::Tabs> {
        let url = format!(
            "/v2.1/accounts/{}/templates/{}/documents/{}/tabs",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&template_id.to_string()),
            crate::progenitor_support::encode_path(&document_id.to_string()),
        );

        self.client
            .delete(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
