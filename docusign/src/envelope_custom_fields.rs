use crate::Client;
use crate::ClientResult;

pub struct EnvelopeCustomFields {
    pub client: Client,
}

impl EnvelopeCustomFields {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EnvelopeCustomFields { client }
    }

    /**
     * Gets the custom field information for the specified envelope.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/custom_fields` endpoint.
     *
     * Retrieves the custom field information for the specified envelope. You can use these fields in the envelopes for your account to record information about the envelope, help search for envelopes, and track information. The envelope custom fields are shown in the Envelope Settings section when a user is creating an envelope in the DocuSign member console. The envelope custom fields are not seen by the envelope recipients.
     *
     * There are two types of envelope custom fields, text, and list. A text custom field lets the sender enter the value for the field. With a list custom field, the sender selects the value of the field from a pre-made list.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn custom_fields_get(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<crate::types::CustomFieldsEnvelope> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/custom_fields",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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
     * Updates envelope custom fields in an envelope.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/custom_fields` endpoint.
     *
     * Updates the envelope custom fields in draft and in-process envelopes.
     *
     * Each custom field used in an envelope must have a unique name.
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn custom_fields_put(
        &self,
        account_id: &str,
        envelope_id: &str,
        body: &crate::types::EnvelopeCustomFields,
    ) -> ClientResult<crate::types::EnvelopeCustomFields> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/custom_fields",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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
     * Creates envelope custom fields for an envelope.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/custom_fields` endpoint.
     *
     * Updates the envelope custom fields for draft and in-process envelopes.
     *
     * You may assign up to three envelope custom fields to an envelope. This limit does not include account (document) custom fields. Each custom field used in an envelope must have a unique name.
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn custom_fields_post(
        &self,
        account_id: &str,
        envelope_id: &str,
        body: &crate::types::EnvelopeCustomFields,
    ) -> ClientResult<crate::types::EnvelopeCustomFields> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/custom_fields",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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
     * Deletes envelope custom fields for draft and in-process envelopes.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/custom_fields` endpoint.
     *
     * Deletes envelope custom fields for draft and in-process envelopes.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn custom_fields_delete(
        &self,
        account_id: &str,
        envelope_id: &str,
        body: &crate::types::EnvelopeCustomFields,
    ) -> ClientResult<crate::types::EnvelopeCustomFields> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/custom_fields",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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
