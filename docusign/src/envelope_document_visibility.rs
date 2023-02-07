use anyhow::Result;

use crate::Client;

pub struct EnvelopeDocumentVisibility {
    pub client: Client,
}

impl EnvelopeDocumentVisibility {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        EnvelopeDocumentVisibility {
            client,
        }
    }

    /**
* Returns document visibility for a recipient.
*
* This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/recipients/{recipientId}/document_visibility` endpoint.
*
* This method returns information about document visibility for a recipient.
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `recipient_id: &str` -- A local reference that senders use to map recipients to other objects, such as specific document tabs. Within an envelope, each `recipientId` must be unique, but there is no uniqueness requirement across envelopes. For example, many envelopes assign the first recipient a `recipientId` of `1`.
*/
pub async fn recipients_get_recipient_document_visibility(
&self,
account_id: &str, envelope_id: &str, recipient_id: &str,
) -> Result<crate::types::DocumentVisibilityList> {
let url =
format!("/v2.1/accounts/{}/envelopes/{}/recipients/{}/document_visibility",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&envelope_id.to_string()),crate::progenitor_support::encode_path(&recipient_id.to_string()),);

self.client.get(&url, None).await
}

/**
* Updates document visibility for a recipient.
*
* This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/recipients/{recipientId}/document_visibility` endpoint.
*
* This method updates document visibility for a recipient.
* 
* **Note**: A document cannot be hidden from a recipient if the recipient has tabs assigned to them on the document. Carbon Copy, Certified Delivery (Needs to Sign), Editor, and Agent recipients can always see all documents.
*
* **Parameters:**
*
* * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
* * `recipient_id: &str` -- A local reference that senders use to map recipients to other objects, such as specific document tabs. Within an envelope, each `recipientId` must be unique, but there is no uniqueness requirement across envelopes. For example, many envelopes assign the first recipient a `recipientId` of `1`.
*/
pub async fn recipients_put_recipient_document_visibility(
&self,
account_id: &str, envelope_id: &str, recipient_id: &str,
body: &crate::types::DocumentVisibilityList
) -> Result<crate::types::DocumentVisibilityList> {
let url =
format!("/v2.1/accounts/{}/envelopes/{}/recipients/{}/document_visibility",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&envelope_id.to_string()),crate::progenitor_support::encode_path(&recipient_id.to_string()),);

self.client.put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?))).await
}


}