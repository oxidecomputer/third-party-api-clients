use crate::Client;
use crate::ClientResult;

pub struct BulkSend {
    pub client: Client,
}

impl BulkSend {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        BulkSend { client }
    }

    /**
     * Returns a list of bulk send batch summaries. .
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/bulk_send_batch` endpoint.
     *
     * Returns a summary of bulk send batches.
     *
     * Use the `batch_ids` query parameter to narrow the list of batches.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `batch_ids: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `count: &str` -- The number of results to report. Must be a value from 1 to 1000.
     *   
     *   Default: 100.
     * * `start_position: &str` -- The start position for results. Essentially the number of results to skip before collecting them. Must be a value greater than 0.
     * * `status: &str` -- The kind of results to collect. Must be one of:
     *   
     *   - all
     *   - failed
     *   - sent
     *   - queued.
     */
    pub async fn batch_get_batche(
        &self,
        account_id: &str,
        batch_ids: &str,
        count: &str,
        start_position: &str,
        status: &str,
    ) -> ClientResult<crate::types::BulkSendBatchSummaries> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !batch_ids.is_empty() {
            query_args.push(("batch_ids".to_string(), batch_ids.to_string()));
        }
        if !count.is_empty() {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !start_position.is_empty() {
            query_args.push(("start_position".to_string(), start_position.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/bulk_send_batch?{}",
                crate::progenitor_support::encode_path(account_id),
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
     * Gets the status of a specific bulk send batch.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/bulk_send_batch/{bulkSendBatchId}` endpoint.
     *
     * Gets the general status of a specific bulk send batch such as:
     *
     * - number of successes
     * - number pending
     * - number of errors
     *
     * The `bulkErrors` property of the response object contains more information about the errors.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `bulk_send_batch_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn batch_get_statu(
        &self,
        account_id: &str,
        bulk_send_batch_id: &str,
    ) -> ClientResult<crate::types::BulkSendBatchStatus> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/bulk_send_batch/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(bulk_send_batch_id),
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
     * Updates a specific bulk send batch status.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/bulk_send_batch/{bulkSendBatchId}` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `bulk_send_batch_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn batch_put_status(
        &self,
        account_id: &str,
        bulk_send_batch_id: &str,
        body: &crate::types::BulkSendBatchRequest,
    ) -> ClientResult<crate::types::BulkSendBatchStatus> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/bulk_send_batch/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(bulk_send_batch_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Gets bulk send lists.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/bulk_send_lists` endpoint.
     *
     * This method returns a list of bulk send lists belonging to the current user, as well as basic information about each list.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn v_2crud_get_list(
        &self,
        account_id: &str,
    ) -> ClientResult<crate::types::BulkSendingListSummaries> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/bulk_send_lists",
                crate::progenitor_support::encode_path(account_id),
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
     * Creates a bulk send list.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/bulk_send_lists` endpoint.
     *
     * This method creates a bulk send list that you can use to send an envelope to up to 1,000 recipients at once.
     *
     * It returns the following errors:
     *
     * | Error code                                              | Description                                                                                                                                                                                                                                                                              |
     * | :------------------------------------------------------ | :--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
     * | BULK_SEND_MAX_COPIES_EXCEEDED                           | A bulk sending list cannot specify more than XXX copies in the mailing list. Call the settings API endpoint to find the maximum number of copies in a batch allowed for your account. In almost all cases, the default limit is 1000.                                                    |
     * | BULK_SEND_RECIPIENT_IDS_MUST_BE_UNIQUE                  | No two recipientIds can be same within a bulkCopy. Same restriction as a regular envelope applies. Specify unique recipient IDs for each recipient within a bulkCopy (model for envelope in mailing list).                                                                               |
     * | BULK_SEND_RECIPIENT_ID_REQUIRED                         | If no RoleName is present, recipientID is required in mailing list's bulkCopy. Add a roleName (that coalesces with template/envelope) or a recipientID.                                                                                                                                  |
     * | BULK_SEND_RECIPIENT_NAME_REQUIRED                       | Recipient {0} has no name. Specify a name for the recipient.                                                                                                                                                                                                                             |
     * | BULK_SEND_EMAIL_ADDRESS_REQUIRED_FOR_EMAIL_RECIPIENT    | Recipient {0} is an email recipient with no email address. Specify an email for the email recipient.                                                                                                                                                                                     |
     * | BULK_SEND_FAX_NUMBER_REQUIRED_FOR_FAX_RECIPIENT         | Recipient {0} is a fax recipient with no fax number. Specify a fax number for the fax recipient.                                                                                                                                                                                         |
     * | BULK_SEND_FAX_NUMBER_NOT_VALID                          | Recipient {0} specifies fax number {1}, which is not valid. Specify a valid fax number for the fax recipient.                                                                                                                                                                            |
     * | BULK_SEND_EMAIL_ADDRESS_NOT_VALID                       | Recipient {0} specifies email address {1}, which is not a valid email address.  Specify a valid email address for the recipient.                                                                                                                                                         |
     * | BULK_SEND_PHONE_NUMBER_REQURED_FOR_SMS_AUTH             | Recipient {0} specifies SMS auth, but no number was provided. Specify a phone number for the SMS auth recipient.                                                                                                                                                                         |
     * | BULK_SEND_PHONE_NUMBER_INVALID_FOR_SMS_AUTH             | Recipient {0} specifies phone number {1} for SMS auth, which is not valid. Specify a valid phone number for the SMS auth recipient.                                                                                                                                                      |
     * | BULK_SEND_ROLE_NAMES_MUST_BE_UNIQUE                     | Recipient role names cannot be duplicated; role {duplicateRecipientRole} appears multiple times. Use unique roleNames for recipients.                                                                                                                                                    |
     * | BULK_SEND_CANNOT_USE_BOTH_ROLE_AND_ID_ON_SAME_RECIPIENT | Recipients cannot have both ID and Role; {0} has both. Specify a roleName or recipientId, but not both for the same recipient.                                                                                                                                                           |
     * | BULK_SEND_CANNOT_USE_BOTH_ROLE_AND_ID_IN_SAME_LIST      | Cannot use both recipient role and ID in the same list. Specify a roleName or recipientId, but not both in the same list.                                                                                                                                                                |
     * | BULK_SEND_INVALID_ID_CHECK_CONFIGURATION                | Recipient {0} specified SMS authentication, but no SMS auth settings were provided. Provide an SMS auth setting (proper ID configuration) if SMS auth is specified.                                                                                                                      |
     * | BULK_SEND_INVALID_SBS_INPUT_CONFIGURATION               | Recipient {0} has more than one signature provider specified. Or signingProviderName is not specified. Or Signature provider : {0} is either unknown or not an available pen for this account. One or more SBS configuration is missing or invalid. The error details provide specifics. |
     * | BULK_SEND_TAB_LABELS_MUST_BE_UNIQUE                     | Tab label {0} is duplicated. Needs to be unique. Use a unique tab label.                                                                                                                                                                                                                 |
     * | BULK_SEND_TAB_LABEL_REQUIRED                            | Tab label is required. Specify a tab label.                                                                                                                                                                                                                                              |
     * | BULK_SEND_TAB_VALUE_REQUIRED                            | Tab Label value is required. Specify a value for the tab label.                                                                                                                                                                                                                          |
     * | BULK_SEND_ENVELOPE_CUSTOM_FIELD_NAME_MUST_BE_UNIQUE     | Custom fields must have distinct names. The field {0} appears more than once in a copy. Use unique names for custom fields.                                                                                                                                                              |
     * | BULK_SEND_ENVELOPE_CUSTOM_FIELD_NAME_REQUIRED           | All custom fields must have names. Specify a name for the custom field.                                                                                                                                                                                                                  |
     * | BULK_SEND_ENVELOPE_CUSTOM_FIELD_VALUE_REQUIRED          | Custom field {0} has no value. A custom field can have an empty value, but it cannot have a null value. Specify a value for the custom field.                                                                                                                                            |
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn v_2crud_post_list(
        &self,
        account_id: &str,
        body: &crate::types::BulkSendingList,
    ) -> ClientResult<crate::types::BulkSendingList> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/bulk_send_lists",
                crate::progenitor_support::encode_path(account_id),
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
     * Gets a specific bulk send list.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/bulk_send_lists/{bulkSendListId}` endpoint.
     *
     * This method returns all of the details associated with a specific bulk send list that belongs to the current user.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `bulk_send_list_id: &str` -- The GUID of the bulk send list. This property is created after you post a new bulk send list.
     */
    pub async fn v_2crud_get_list_bulk_send(
        &self,
        account_id: &str,
        bulk_send_list_id: &str,
    ) -> ClientResult<crate::types::BulkSendingList> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/bulk_send_lists/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(bulk_send_list_id),
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
     * Updates a bulk send list.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/bulk_send_lists/{bulkSendListId}` endpoint.
     *
     * This method replaces the definition of an existing bulk send list.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `bulk_send_list_id: &str` -- The GUID of the bulk send list. This property is created after you post a new bulk send list.
     */
    pub async fn v_2crud_put_list(
        &self,
        account_id: &str,
        bulk_send_list_id: &str,
        body: &crate::types::BulkSendingList,
    ) -> ClientResult<crate::types::BulkSendingList> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/bulk_send_lists/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(bulk_send_list_id),
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
     * Deletes a bulk send list.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/bulk_send_lists/{bulkSendListId}` endpoint.
     *
     * This method deletes a bulk send list.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `bulk_send_list_id: &str` -- The GUID of the bulk send list. This property is created after you post a new bulk send list.
     */
    pub async fn v_2crud_delete_list(
        &self,
        account_id: &str,
        bulk_send_list_id: &str,
    ) -> ClientResult<crate::types::BulkSendingListSummaries> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/bulk_send_lists/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(bulk_send_list_id),
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
     * Creates a bulk send request.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/bulk_send_lists/{bulkSendListId}/send` endpoint.
     *
     * This method initiates the bulk send process. It generates a bulk send request based on an [existing bulk send list][create_list] and an envelope or template.
     *
     * Consider using the [BulkSend::createBulkSendTestRequest][create_test] method to test your bulk send list for compatibility with the envelope or template that you want to send first. To learn about the complete bulk send flow, see the [Bulk Send overview][BulkSendOverview].
     *
     * If the envelopes were successfully queued for asynchronous processing, the response contains a `batchId` that you can use to get the status of the batch. If a failure occurs, the API returns an error message.
     *
     * **Note**: Partial success or failure generally does not occur. Only the entire batch is queued for asynchronous processing.
     *
     * This method returns the following errors:
     *
     * | Error code                                                 | Description                                                                                                                                                                                                                                                                                            |
     * | :--------------------------------------------------------- | :----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
     * | BULK_SEND_ENVELOPE_NOT_IN_SENDABLE_STATE                   | Envelope {0} is not in a sendable state. The envelope is not complete. Make sure it has a sendable status, such as `created`.                                                                                                                                                                          |
     * | BULK_SEND_ENVELOPE_LIST_CONTAINS_NO_COPIES                 | Cannot send an envelope with a bulk sending list which contains no copies.  The list you're trying to bulk send is empty. Make sure the bulk sending list you're using contains recipients.                                                                                                            |
     * | BULK_SEND_ENVELOPE_LIST_CONTAINS_TOO_MANY_COPIES           | Bulk sending list contains more than {0} copies. The list you're trying to bulk send will cause your account to exceed the 1,000-copy limit imposed for all accounts. Try sending two or more separate lists.                                                                                          |
     * | BULK_SEND_ENVELOPE_CANNOT_BE_NULL                          | Cannot send a bulk list without an envelope. Specify the envelope ID or template ID for the envelope you want to bulk send.                                                                                                                                                                            |
     * | BULK_SEND_BLOB_STORE_ERROR                                 | Could not save copy of bulk sending list. An error writing to the database occurred. Try again. If the error persists, contact DocuSign Support.                                                                                                                                                       |
     * | BULK_SEND_ACCOUNT_HAS_TOO_MANY_QUEUED_ENVELOPES            | Cannot send this bulk sending list because doing so would exceed the maximum of {maxCopies} in-flight envelopes. This account currently has {unprocessedEnvelopes} envelopes waiting to be processed. Please try again later." .Try again later, or contact DocuSign Support to request a higher tier. |
     * | BULK_SEND_ENVELOPE_NOT_FOUND                               | Envelope {envelopeOrTemplateId} does not exist or you do not have permission to access it. The envelopeId or templateId specified could not be found. Specify a valid value.                                                                                                                           |
     * | BULK_SEND_LIST_NOT_FOUND                                   | Bulk Sending list {listId} does not exist or you do not have permission to access it. The mailingListId specified could not be found. Specify a valid value.                                                                                                                                           |
     * | BULK_SEND_ENVELOPE_HAS_NO_RECIPIENTS                       | Bulk sending copy contains recipients, but the specified envelope does not. The recipients on the envelope and the bulk send list do not match. Make sure the envelope and list are compatible for sending.                                                                                            |
     * | BULK_SEND_RECIPIENT_ID_DOES_NOT_EXIST_IN_ENVELOPE          | Recipient {0} does not exist in the envelope. Add the missing recipient to the envelope.                                                                                                                                                                                                               |
     * | BULK_SEND_RECIPIENT_ID_DOES_NOT_MATCH                      | Recipient ID {envelopeMember.ID} does not match. Make sure the recipient information in the list and the envelope match up.                                                                                                                                                                            |
     * | BULK_SEND_ENVELOPE_HAS_BULK_RECIPIENT                      | Recipient {0} is a bulk recipient.  This is not supported. No legacy 'bulk recipient' allowed. In v2.1 of the eSignature API, you must use a bulk send list instead of a bulk recipient. See the API documentation for details.                                                                        |
     * | BULK_SEND_RECIPIENT_ROLE_DOES_NOT_MATCH                    | Recipient Role {envelopeMember.RoleName} does not match. Make sure the recipient information in the list and the envelope is compatible.                                                                                                                                                               |
     * | BULK_SEND_DUPLICATE_RECIPIENT_DETECTED                     | Duplicate recipients ({0}) not allowed, unless recipients have unique routing order specified on envelope.                                                                                                                                                                                             |
     * | BULK_SEND_ENVELOPE_HAS_NO_TABS                             | Bulk sending copy contains tabs, but the specified envelope does not. List and envelope tabs cannot be coalesced. Make sure they are compatible for sending.                                                                                                                                           |
     * | BULK_SEND_TAB_LABEL_DOES_NOT_EXIST_IN_ENVELOPE             | Tab with label {0} does not exist in envelope. Add the tab label to the envelope, remove the label from the list, or make sure you're specifying the correct list and envelope.                                                                                                                        |
     * | BULK_SEND_TAB_DOES_NOT_MATCH                               | Tab {0} does not match {0} in envelope. A tab exists on the list that does not match or is missing on the envelope. Make sure the tabs on the list and the envelope match.                                                                                                                             |
     * | BULK_SEND_ENVELOPE_HAS_NO_ENVELOPE_CUSTOM_FIELDS           | Bulk sending copy contains custom fields, but the specified envelope does not. There are custom fields on the list that the envelope does not have. Make sure that any custom fields on the list and the envelope match.                                                                               |
     * | BULK_SEND_ENVELOPE_CUSTOM_FIELD_DOES_NOT_EXIST_IN_ENVELOPE | Custom field {0} does not exist in the envelope. Either add the custom field on the list to the envelope, remove the custom field from the list, or make sure you're specifying the correct list and envelope.                                                                                         |
     * | BULK_SEND_ENVELOPE_CUSTOM_FIELD_NAME_DOES_NOT_MATCH        | Custom field names must match. {0} and {1} do not match. The custom field names on the list and the envelope do not match. Use identical names for both.                                                                                                                                               |
     *
     * [create_list]:      https://developers.docusign.com/docs/esign-rest-api/reference/BulkEnvelopes/BulkSend/createBulkSendList
     * [create_test]:      https://developers.docusign.com/docs/esign-rest-api/reference/BulkEnvelopes/BulkSend/createBulkSendTestRequest
     * [BulkSendOverview]: https://developers.docusign.com/docs/esign-rest-api/reference/BulkEnvelopes/BulkSend
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `bulk_send_list_id: &str` -- The GUID of the bulk send list. This property is created after you post a new bulk send list.
     */
    pub async fn post_request(
        &self,
        account_id: &str,
        bulk_send_list_id: &str,
        body: &crate::types::BulkSendRequest,
    ) -> ClientResult<crate::types::BulkSendResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/bulk_send_lists/{}/send",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(bulk_send_list_id),
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
     * Creates a bulk send test.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/bulk_send_lists/{bulkSendListId}/test` endpoint.
     *
     * This method tests a bulk send list for compatibility with the envelope or template that you want to send. For example, a template that has three roles is not compatible with a bulk send list that has only two recipients. For this reason, you might want to test compatibility first.
     *
     * A successful test result returns `true` for the `canBeSent` property. An unsuccessful test returns a JSON response that contains information about the errors that occurred.
     *
     * If the test is successful, you can then send the envelope or template by using the [BulkSend::createBulkSendRequest][BulkSendRequest] method.
     *
     * ## Envelope Compatibility Checks
     *
     * This section describes the envelope compatibility checks that the system performs.
     *
     * **Top-Level Issues**
     *
     * - Envelopes must be in a sendable state.
     * - The bulk send list must contain at least one copy (instance of an envelope), and no more than the maximum number of copies allowed for the account.
     * - The envelope must not be null and must be visible to the current user.
     * - The account cannot have more queued envelopes than the maximum number configured for the account.
     * - The bulk send list must exist.
     *
     * **Recipients**
     *
     * - The envelope must have recipients.
     * - If you are using an envelope, all of the recipients defined in the bulk send list must have corresponding recipient IDs in the envelope definition. If you are using a template, you must either match the recipient IDs or role IDs.
     * - The envelope cannot contain a bulk recipient (an artifact of the legacy version of DocuSign's bulk send
     *   functionality).
     *
     * **Recipient Tabs**
     *
     * - Every `recipient ID, tab label` pair in the bulk send list must correspond to a tab in the envelope.
     *
     * **Custom Fields**
     *
     * - Each envelope-level custom field in the bulk send list must correspond to the name of a `customField` in the
     *   envelope definition. You do not have to match the recipient-level custom fields.
     *
     * [BulkSendRequest]:  https://developers.docusign.com/docs/esign-rest-api/reference/BulkEnvelopes/BulkSend/createBulkSendRequest
     *
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `bulk_send_list_id: &str` -- The GUID of the bulk send list. This property is created after you post a new bulk send list.
     */
    pub async fn test_post_request(
        &self,
        account_id: &str,
        bulk_send_list_id: &str,
        body: &crate::types::BulkSendRequest,
    ) -> ClientResult<crate::types::BulkSendTestResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/bulk_send_lists/{}/test",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(bulk_send_list_id),
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
