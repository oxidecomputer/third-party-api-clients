use anyhow::Result;

use crate::Client;

pub struct EnvelopeDocumentTabs {
    client: Client,
}

impl EnvelopeDocumentTabs {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        EnvelopeDocumentTabs {
            client,
        }
    }

    /**
* Returns tabs on the specified page.
*
* This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents/{documentId}/pages/{pageNumber}/tabs` endpoint.
*
* 
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
* * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `page_number: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
*/
pub async fn tabs_get_page(
&self,
account_id: &str, document_id: &str, envelope_id: &str, page_number: &str,
) -> Result<crate::types::EnvelopeDocumentTabs> {
let url =
format!("/v2.1/accounts/{}/envelopes/{}/documents/{}/pages/{}/tabs",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&envelope_id.to_string()),crate::progenitor_support::encode_path(&document_id.to_string()),crate::progenitor_support::encode_path(&page_number.to_string()),);

self.client.get(&url, None).await
}

/**
* Returns the tabs on a document.
*
* This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents/{documentId}/tabs` endpoint.
*
* This method returns the tabs associated with a document.
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
* * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `include_metadata: &str` -- When set to **true**, the response includes metadata indicating which properties are editable.
* * `page_numbers: &str` -- Filters for tabs that occur on the pages that you specify. Enter as a comma-separated list of page GUIDs.
*   
*   Example: `page_numbers=2,6`
*   
*   Note: You can only enter individual page numbers, and not a page range.
*/
pub async fn tabs_get_document(
&self,
account_id: &str, document_id: &str, envelope_id: &str, include_metadata: &str, page_numbers: &str,
) -> Result<crate::types::EnvelopeDocumentTabs> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
if !include_metadata.is_empty() { query_args.push(format!("include_metadata={}", include_metadata)); }
if !page_numbers.is_empty() { query_args.push(format!("page_numbers={}", page_numbers)); }
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/v2.1/accounts/{}/envelopes/{}/documents/{}/tabs?{}",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&envelope_id.to_string()),crate::progenitor_support::encode_path(&document_id.to_string()),query);

self.client.get(&url, None).await
}

/**
* Updates the tabs for an envelope document.
*
* This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents/{documentId}/tabs` endpoint.
*
* 
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
* * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
*/
pub async fn tabs_put_document(
&self,
account_id: &str, document_id: &str, envelope_id: &str,
body: &crate::types::Tabs
) -> Result<crate::types::Tabs> {
let url =
format!("/v2.1/accounts/{}/envelopes/{}/documents/{}/tabs",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&envelope_id.to_string()),crate::progenitor_support::encode_path(&document_id.to_string()),);

self.client.put(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Adds the tabs to an envelope document.
*
* This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents/{documentId}/tabs` endpoint.
*
* 
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
* * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
*/
pub async fn tabs_post_document(
&self,
account_id: &str, document_id: &str, envelope_id: &str,
body: &crate::types::Tabs
) -> Result<crate::types::Tabs> {
let url =
format!("/v2.1/accounts/{}/envelopes/{}/documents/{}/tabs",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&envelope_id.to_string()),crate::progenitor_support::encode_path(&document_id.to_string()),);

self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Deletes tabs from an envelope document.
*
* This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents/{documentId}/tabs` endpoint.
*
* 
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
* * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
*/
pub async fn tabs_delete_document(
&self,
account_id: &str, document_id: &str, envelope_id: &str,
body: &crate::types::Tabs
) -> Result<crate::types::Tabs> {
let url =
format!("/v2.1/accounts/{}/envelopes/{}/documents/{}/tabs",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&envelope_id.to_string()),crate::progenitor_support::encode_path(&document_id.to_string()),);

self.client.delete(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}


}