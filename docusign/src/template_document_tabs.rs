use crate::Client;
use crate::ClientResult;

pub struct TemplateDocumentTabs {
    pub client: Client,
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
    ) -> ClientResult<crate::Response<crate::types::TemplateDocumentTabs>> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/documents/{}/pages/{}/tabs",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
                crate::progenitor_support::encode_path(document_id),
                crate::progenitor_support::encode_path(page_number),
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
    ) -> ClientResult<crate::Response<crate::types::TemplateDocumentTabs>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !page_numbers.is_empty() {
            query_args.push(("page_numbers".to_string(), page_numbers.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/documents/{}/tabs?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
                crate::progenitor_support::encode_path(document_id),
                query_
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
    ) -> ClientResult<crate::Response<crate::types::Tabs>> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/documents/{}/tabs",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
                crate::progenitor_support::encode_path(document_id),
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
    ) -> ClientResult<crate::Response<crate::types::Tabs>> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/documents/{}/tabs",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
                crate::progenitor_support::encode_path(document_id),
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
    ) -> ClientResult<crate::Response<crate::types::Tabs>> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/templates/{}/documents/{}/tabs",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(template_id),
                crate::progenitor_support::encode_path(document_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
}
