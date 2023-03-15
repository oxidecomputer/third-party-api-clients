use crate::Client;
use crate::ClientResult;

pub struct EnvelopeRecipients {
    pub client: Client,
}

impl EnvelopeRecipients {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EnvelopeRecipients { client }
    }

    /**
     * Gets the status of recipients for an envelope.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/recipients` endpoint.
     *
     * Retrieves the status of all recipients in a single envelope and identifies the current recipient in the routing list. This method can also be used to retrieve the tab values.
     *
     * The `currentRoutingOrder` property of the response contains the `routingOrder` value of the current recipient indicating that the envelope has been sent to the recipient, but the recipient has not completed their actions.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include_anchor_tab_locations: &str` --  When set to **true** and `include_tabs` value is set to **true**, all tabs with anchor tab properties are included in the response. .
     * * `include_extended: &str` --  When set to **true**, the extended properties are included in the response. .
     * * `include_metadata: &str` -- Boolean value that specifies whether to include metadata associated with the recipients (for envelopes only, not templates).
     * * `include_tabs: &str` -- When set to **true**, the tab information associated with the recipient is included in the response.
     */
    pub async fn recipients_get(
        &self,
        account_id: &str,
        envelope_id: &str,
        include_anchor_tab_locations: &str,
        include_extended: &str,
        include_metadata: &str,
        include_tabs: &str,
    ) -> ClientResult<crate::types::EnvelopeRecipients> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include_anchor_tab_locations.is_empty() {
            query_args.push((
                "include_anchor_tab_locations".to_string(),
                include_anchor_tab_locations.to_string(),
            ));
        }
        if !include_extended.is_empty() {
            query_args.push(("include_extended".to_string(), include_extended.to_string()));
        }
        if !include_metadata.is_empty() {
            query_args.push(("include_metadata".to_string(), include_metadata.to_string()));
        }
        if !include_tabs.is_empty() {
            query_args.push(("include_tabs".to_string(), include_tabs.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/recipients?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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
     * Updates recipients in a draft envelope or corrects recipient information for an in-process envelope.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/recipients` endpoint.
     *
     * Updates the recipients of a draft envelope or corrects recipient information for an in-process envelope.
     *
     * If you send information for a recipient that does not already exist in a draft envelope, the recipient is added to the envelope (similar to the EnvelopeRecipients::Create method).
     *
     * You can also use this method to resend an envelope to a recipient by using the `resend_envelope` option.
     *
     * **Updating Sent Envelopes**
     *
     * After an envelope has been sent, you can edit only the following properties:
     *
     * - `accessCode`
     * - `agentCanEditName`
     * - `agentCanEditEmail`
     * - `customFields`
     * - `deliveryMethod`
     * - `documentVisibility`
     * - `email`
     * - `emailNotification`
     * - `idCheckConfigurationName`
     * - `name`
     * - `note`
     * - `phoneAuthentication`
     * - `recipientType` (For this to work, you must also change the recipient object to match the recipient type.)
     * - `requireIdLookup`
     * - `routingOrder`
     * - `routingOrder`
     * - `signingGroupId` (You can change this id to switch to a different signing group and its corresponding set of recipients.)
     * - `smsAuthentication`
     * - `suppressEmails`
     * - `userName`
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `combine_same_order_recipients: &str` -- When set to **true**, recipients are combined or merged with matching recipients. Recipient matching occurs as part of [template matching](https://docs.docusign.com/DocuSignHelp/Content/automatic-template-matching.htm), and is based on Recipient Role and Routing Order.
     * * `offline_signing: &str` -- Indicates if offline signing is enabled for the recipient when a network connection is unavailable. .
     * * `resend_envelope: &str` -- When set to **true**, resends the   envelope if the new recipient's routing order is before or the same as the envelope's next recipient.
     */
    pub async fn recipients_put(
        &self,
        account_id: &str,
        envelope_id: &str,
        combine_same_order_recipients: &str,
        offline_signing: &str,
        resend_envelope: &str,
        body: &crate::types::EnvelopeRecipients,
    ) -> ClientResult<crate::types::RecipientsUpdateSummary> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !combine_same_order_recipients.is_empty() {
            query_args.push((
                "combine_same_order_recipients".to_string(),
                combine_same_order_recipients.to_string(),
            ));
        }
        if !offline_signing.is_empty() {
            query_args.push(("offline_signing".to_string(), offline_signing.to_string()));
        }
        if !resend_envelope.is_empty() {
            query_args.push(("resend_envelope".to_string(), resend_envelope.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/recipients?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                query_
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
     * Adds one or more recipients to an envelope.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/recipients` endpoint.
     *
     * Adds one or more recipients to an envelope.
     *
     * For an in process envelope, one that has been sent and has not been completed or voided, an email is sent to a new recipient when they are reached in the routing order. If the new recipient's routing order is before or the same as the envelope's next recipient, an email is only sent if the optional `resend_envelope` query string is set to **true**.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `resend_envelope: &str` -- When set to **true**, resends the   envelope if the new recipient's routing order is before or the same as the envelope's next recipient.
     */
    pub async fn recipients_post(
        &self,
        account_id: &str,
        envelope_id: &str,
        resend_envelope: &str,
        body: &crate::types::EnvelopeRecipients,
    ) -> ClientResult<crate::types::EnvelopeRecipients> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !resend_envelope.is_empty() {
            query_args.push(("resend_envelope".to_string(), resend_envelope.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/recipients?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                query_
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
     * Deletes recipients from an envelope.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/recipients` endpoint.
     *
     * Deletes one or more recipients from a draft or sent envelope. List the recipients that you want to delete in the body of the request. This method uses the `recipientId` as the key for deleting recipients.
     *
     * If the envelope is `In Process`, meaning that it has been sent and has not been completed or voided, recipients that have completed their actions cannot be deleted.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn recipients_delete(
        &self,
        account_id: &str,
        envelope_id: &str,
        body: &crate::types::EnvelopeRecipients,
    ) -> ClientResult<crate::types::EnvelopeRecipients> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/recipients",
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
    /**
     * Updates document visibility for recipients.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/recipients/document_visibility` endpoint.
     *
     * This method updates document visibility for one or more recipients based on the `recipientId` and `visible` values that you include in the request body.
     *
     * **Note**: A document cannot be hidden from a recipient if the recipient has tabs assigned to them on the document. Carbon Copy, Certified Delivery (Needs to Sign), Editor, and Agent recipients can always see all documents.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn recipients_put_document_visibility(
        &self,
        account_id: &str,
        envelope_id: &str,
        body: &crate::types::DocumentVisibilityList,
    ) -> ClientResult<crate::types::DocumentVisibilityList> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/recipients/document_visibility",
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
     * Deletes a recipient from an envelope.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/recipients/{recipientId}` endpoint.
     *
     * Deletes a recipient from a `draft` or `sent` envelope.
     *
     * If the envelope is "In Process" (has been sent and is not completed or voided), recipients that have completed their actions cannot be deleted.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `recipient_id: &str` -- A local reference that senders use to map recipients to other objects, such as specific document tabs. Within an envelope, each `recipientId` must be unique, but there is no uniqueness requirement across envelopes. For example, many envelopes assign the first recipient a `recipientId` of `1`.
     */
    pub async fn recipients_delete_recipient(
        &self,
        account_id: &str,
        envelope_id: &str,
        recipient_id: &str,
    ) -> ClientResult<crate::types::EnvelopeRecipients> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/recipients/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                crate::progenitor_support::encode_path(recipient_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates a resource token for a sender to request ID Evidence data. .
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/recipients/{recipientId}/identity_proof_token` endpoint.
     *
     * Creates a resource token for a sender. This token allows a sender to return identification data for a recipient using the [ID Evidence API](https://developers.docusign.com/docs/idevidence-api/).
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `recipient_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn recipients_post_recipient_proof_file_resource_token(
        &self,
        account_id: &str,
        envelope_id: &str,
        recipient_id: &str,
    ) -> ClientResult<crate::types::ProofServiceResourceToken> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/recipients/{}/identity_proof_token",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                crate::progenitor_support::encode_path(recipient_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Create the link to the page for manually reviewing IDs.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/recipients/{recipientId}/views/identity_manual_review` endpoint.
     *
     * This method returns the URL of the page that allows a sender to [manually review](https://support.docusign.com/en/guides/ndse-user-guide-send-documents-with-id-verification) the ID of a recipient.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- A value that identifies your account. This value is automatically generated by DocuSign for any account you create. Copy the value from the API Account ID field in the [AppsI and Keys](https://support.docusign.com/en/guides/ndse-admin-guide-api-and-keys) page.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `recipient_id: &str` -- A GUID value that DocuSign assigns to identify each recipient in an envelope. This value is globally unique for all recipients, not just those in your account.
     *   
     *   The specified recipient must belong to a workflow that allows the [manual review](https://support.docusign.com/en/guides/Identity-Verification-DocuSign-eSignature-Admin-Guide) of IDs. In addition, the status of the automatic verification for this recipient must return `Failed` and the value of the `vendorFailureStatusCode` field must be `MANUAL_REVIEW_STARTED` as shown in the following extract of a response to the [GET ENVELOPE](https://developers.docusign.com/docs/esign-rest-api/reference/envelopes/envelopes/get/) method:
     *   <p>
     *   
     *   ```
     *   "recipientAuthenticationStatus": {
     *          "identityVerificationResult": {
     *                "status": "Failed",
     *                "eventTimestamp": "2020-09-04T16:59:42.8045667Z",
     *                "vendorFailureStatusCode": "MANUAL_REVIEW_STARTED"
     *           }
     *     }
     *   ```.
     */
    pub async fn views_post_recipient_manual_review_view(
        &self,
        account_id: &str,
        envelope_id: &str,
        recipient_id: &str,
    ) -> ClientResult<crate::types::ViewUrl> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/recipients/{}/views/identity_manual_review",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                crate::progenitor_support::encode_path(recipient_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Creates an envelope recipient preview.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/views/recipient_preview` endpoint.
     *
     * This method returns a URL for an envelope recipient preview  in the DocuSign UI that you can embed in your application. You use this method to enable the sender to preview the recipients' experience.
     *
     * For more information, see [Preview and Send](https://support.docusign.com/en/guides/ndse-user-guide-send-your-documents).
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn views_post_preview(
        &self,
        account_id: &str,
        envelope_id: &str,
        body: &crate::types::RecipientPreviewRequest,
    ) -> ClientResult<crate::types::ViewUrl> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/views/recipient_preview",
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
}
