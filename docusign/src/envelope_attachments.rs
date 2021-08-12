use anyhow::Result;

use crate::Client;

pub struct EnvelopeAttachments {
    client: Client,
}

impl EnvelopeAttachments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        EnvelopeAttachments {
            client,
        }
    }

    /**
* Returns a list of attachments associated with the specified envelope.
*
* This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/attachments` endpoint.
*
* 
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
*/
pub async fn attachments_get(
&self,
account_id: &str, envelope_id: &str,
) -> Result<crate::types::EnvelopeAttachmentsResult> {
let url =
format!("/v2.1/accounts/{}/envelopes/{}/attachments",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&envelope_id.to_string()),);

self.client.get(&url, None).await
}

/**
* Add one or more attachments to a draft or in-process envelope.
*
* This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/attachments` endpoint.
*
* Supported attachment formats include: .asp, .bmp, .csv, .doc, .docm, .docx, .dot, .dotm, .dotx, .gif, .htm, .html, .jpeg, .jpg, .msg, .pdf, .png, .pot, .potx, .pps, .ppt, .pptm, .pptx, .ps, .rtf, .tif, .tiff, .txt, .wpd, .xls, .xlsm, .xlsx, .xml, and .xps. For more information about supported file formats, see [Supported File Formats](https://support.docusign.com/guides/ndse-user-guide-supported-file-formats).
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
*/
pub async fn attachments_put(
&self,
account_id: &str, envelope_id: &str,
body: &crate::types::EnvelopeAttachmentsRequest
) -> Result<crate::types::EnvelopeAttachmentsResult> {
let url =
format!("/v2.1/accounts/{}/envelopes/{}/attachments",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&envelope_id.to_string()),);

self.client.put(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Delete one or more attachments from a DRAFT envelope.
*
* This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/attachments` endpoint.
*
* 
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
*/
pub async fn attachments_delete(
&self,
account_id: &str, envelope_id: &str,
body: &crate::types::EnvelopeAttachmentsRequest
) -> Result<crate::types::EnvelopeAttachmentsResult> {
let url =
format!("/v2.1/accounts/{}/envelopes/{}/attachments",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&envelope_id.to_string()),);

self.client.delete(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Retrieves an attachment from the envelope.
*
* This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/attachments/{attachmentId}` endpoint.
*
* 
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `attachment_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
*/
pub async fn attachments_get_attachment(
&self,
account_id: &str, attachment_id: &str, envelope_id: &str,
) -> Result<()> {
let url =
format!("/v2.1/accounts/{}/envelopes/{}/attachments/{}",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&envelope_id.to_string()),crate::progenitor_support::encode_path(&attachment_id.to_string()),);

self.client.get(&url, None).await
}

/**
* Add an attachment to a DRAFT or IN-PROCESS envelope.
*
* This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/attachments/{attachmentId}` endpoint.
*
* 
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `attachment_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
*/
pub async fn attachments_put_attachment(
&self,
account_id: &str, attachment_id: &str, envelope_id: &str,
body: &crate::types::Attachment
) -> Result<crate::types::EnvelopeAttachmentsResult> {
let url =
format!("/v2.1/accounts/{}/envelopes/{}/attachments/{}",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&envelope_id.to_string()),crate::progenitor_support::encode_path(&attachment_id.to_string()),);

self.client.put(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}


}