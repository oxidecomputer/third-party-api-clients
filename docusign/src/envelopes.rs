use crate::Client;
use crate::ClientResult;

pub struct Envelopes {
    pub client: Client,
}

impl Envelopes {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Envelopes { client }
    }

    /**
     * Gets status changes for one or more envelopes.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes` endpoint.
     *
     * Retrieves a list of envelopes that match your request.
     * A large set of optional filters let you filter
     * by date,
     * by envelope ID,
     * or by status codes.
     *
     * Your request must include one or more of the following parameters:
     *
     * * `from_date`
     * * `envelope_ids`
     * * `transaction_ids`
     *
     *
     * Getting envelope status using `transaction_ids` is useful
     * for offline signing situations where it can be used
     * determine if an envelope was created or not. It can be used
     * for the cases where a network connection was lost, before
     * the envelope status could be returned.
     *
     * To avoid unnecessary database queries, the DocuSign
     * signature platform first checks requests to ensure that the
     * filter set supplied does not result in a zero-size response
     * before querying the database.
     *
     * For example, for a request with a `from_to_status` of
     * `delivered` and a current `status` of `created,sent`,
     * DocuSign will always return an empty list.
     * This is because the request translates to: find the
     * envelopes that were delivered between the `from_date` and
     * `to_date` dates that have a current status of `created` or
     * `sent`. Since an envelope that has been delivered can
     * never have a status of `created` or `sent`, a zero-size
     * response would be generated.
     * In this case, DocuSign does not query the database
     * and returns an empty list immediately.
     *
     *
     * The following table shows the valid current envelope
     * statuses (`status` parameter) for the different status
     * qualifiers (`from_to_status` parameter) in the request. If
     * the status and status qualifiers in the API request do not
     * contain any of the values shown in the Valid Current
     * Statuses column, then an empty list is returned.
     *
     * Client applications should check that the statuses (`status`
     * parameter) they are requesting make sense for a given
     * `from_to_status` parameter value.
     *
     * | Status Qualifier<br>(`from_to_status`) | Effective Status Qualifier | Valid Current Statuses                                                      |
     * | :------------------------------------- | :------------------------- | :-------------------------------------------------------------------------- |
     * | any (changed)                          | StatusChanged              | any, created, sent, delivered, signed, completed, declined, voided, deleted |
     * | created                                | Created                    | any, created, sent, delivered, signed, completed, declined, voided, deleted |
     * | sent                                   | Sent                       | any, sent, delivered, signed, completed, declined, voided, deleted          |
     * | delivered                              | StatusChanged              | any, delivered, signed, completed, declined, voided, deleted                |
     * | signed                                 | StatusChanged              | any, signed, completed, declined, voided, deleted                           |
     * | completed                              | Completed                  | any, completed, declined, voided, deleted                                   |
     * | declined                               | StatusChanged              | any, declined, voided, deleted                                              |
     * | timedout<br>always return zero results | StatusChanged              | any, voided, deleted                                                        |
     * | voided                                 | Voided                     | any, voided, deleted                                                        |
     * | deleted                                | StatusChanged              | any, deleted                                                                |
     *
     * ## Extraneous results
     *
     * In some cases, a request for a specific envelope status will
     * include envelopes with additional statuses. For example, in
     * a request with a `from_date` of 2017-01-01, a `to_date` of
     * 2017-01-07 and the status qualifier (`from_to_status`) set
     * to `delivered`, the response set might contain envelopes
     * that were created during that time period, but not delivered
     * during the time period. As a workaround, check the envelope
     * status values in the result set as needed.
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `ac_status: &str` -- Specifies the Authoritative Copy Status for the envelopes. The possible values are: Unknown, Original, Transferred, AuthoritativeCopy, AuthoritativeCopyExportPending, AuthoritativeCopyExported, DepositPending, Deposited, DepositedEO, or DepositFailed.
     * * `block: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `cdse_mode: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `continuation_token: &str` -- A token returned in the response to a previous API call that is used to resume a search query from a specific point.
     * * `count: &str` -- Optional. Number of items to return. Currently there is no implicit maximum limit of the number of items that can be returned.
     *   .
     * * `custom_field: &str` -- Optional. Specifies a envelope custom field name and value searched for in the envelopes. Format: `custom_envelope_field_name=desired_value`
     *   
     *   Example: If you have an envelope custom field named "Region" and you want to search for all envelopes where the value is "West" you would use set this parameter to `Region=West`.
     *   
     *   .
     * * `email: &str` -- Limit results to envelopes
     *   sent by the account user
     *   with this email address.
     *   
     *   `user_name` must be given as well,
     *   and both `email` and `user_name`
     *   must refer to an existing account user.
     *   .
     * * `envelope_ids: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `exclude: &str` -- Excludes information from the response. Enter  as a comma-separated list (e.g., `folders,powerforms`). Valid values are:
     *   
     *   - `recipients`
     *   - `powerforms`
     *   - `folders`.
     * * `folder_ids: &str` -- Returns the envelopes from specific folders. Enter as a comma-separated list of either valid folder Guids or the following values:
     *   
     *   - `awaiting_my_signature`
     *   - `completed`
     *   - `draft`
     *   - `drafts`
     *   - `expiring_soon`
     *   - `inbox`
     *   - `out_for_signature`
     *   - `recyclebin`
     *   - `sentitems`
     *   - `waiting_for_others`.
     * * `folder_types: &str` -- A comma-separated list of folder types you want to retrieve envelopes from. Valid values are:
     *   
     *   - `normal`
     *   - `inbox`
     *   - `sentitems`
     *   - `draft`
     *   - `templates`.
     * * `from_date: &str` -- Specifies the date and time
     *   to start looking for status changes.
     *   This parameter is required
     *   unless `envelopeIds` or `transactionIds`
     *   are set.
     *   
     *   
     *   Although you can use any date format
     *   supported by the .NET system library's
     *   [`DateTime.Parse()`][msoft] function,
     *   DocuSign recommends
     *   using [ISO 8601][] format dates
     *   with an explicit time zone offset
     *   If you do not provide
     *   a time zone offset,
     *   the method uses the server's time zone.
     *   
     *   For example, the following dates and times refer to the same instant:
     *   
     *   * `2017-05-02T01:44Z`
     *   * `2017-05-01T21:44-04:00`
     *   * `2017-05-01T18:44-07:00`
     *   
     *   
     *   [msoft]: https://msdn.microsoft.com/en-us/library/system.datetime.parse(v=vs.110).aspx#StringToParse
     *   [ISO 8601]: https://en.wikipedia.org/wiki/ISO_8601.
     * * `from_to_status: &str` -- This is the status type checked for in the `from_date`/`to_date` period. If `changed` is specified, then envelopes that changed status during the period are found. If for example, `created` is specified, then envelopes created during the period are found. Default is `changed`.
     *   
     *   Possible values are: Voided, Changed, Created, Deleted, Sent, Delivered, Signed, Completed, Declined, TimedOut and Processing.
     * * `include: &str` -- Specifies additional information to return  about the envelopes. Enter a comma-separated list, such as `tabs,recipients`. Valid values are:
     *   
     *   - `custom_fields`: The custom fields associated with the envelope.
     *   - `documents`: The documents associated with the envelope.
     *   - `attachments`: The attachments associated with the envelope.
     *   - `extensions`: Information about the email settings associated with the envelope.
     *   - `folders`: The folders where the envelope exists.
     *   - `recipients`: The recipients associated with the envelope.
     *   - `powerform`: The PowerForms associated with the envelope.
     *   - `payment_tabs`: The payment tabs associated with the envelope.
     *   .
     * * `include_purge_information: &str` -- When set to **true**, information about envelopes that have been deleted is included in the response.
     * * `intersecting_folder_ids: &str` -- A comma-separated list of folders that you want want to get envelopes from. Valid values are:
     *   
     *   - `normal`
     *   - `inbox`
     *   - `sentitems`
     *   - `draft`
     *   - `templates`.
     * * `last_queried_date: &str` -- Returns envelopes that were modified prior to the specified date and time.
     *   
     *   Example: `2020-05-09T21:56:12.2500000Z`.
     * * `order: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `order_by: &str` -- Sorts results according to a specific property. Valid values are:
     *   
     *   - `last_modified`
     *   - `action_required`
     *   - `created`
     *   - `completed`
     *   - `envelope_name`
     *   - `expire`
     *   - `sent`
     *   - `signer_list`
     *   - `status`
     *   - `subject`
     *   - `user_name`
     *   - `status_changed`
     *   - `last_modified`.
     * * `powerformids: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `query_budget: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `requester_date_format: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `search_text: &str` -- Free text search criteria that you can use to filter the list of envelopes that is returned.
     * * `start_position: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `status: &str` -- A comma-separated list of current envelope statuses to included in the response. Possible values are:
     *   
     *   * `completed`
     *   * `created`
     *   * `declined`
     *   * `deleted`
     *   * `delivered`
     *   * `processing`
     *   * `sent`
     *   * `signed`
     *   * `timedout`
     *   * `voided`
     *   
     *   The `any` value is equivalent to any status.
     *   .
     * * `to_date: &str` -- Specifies the date and time
     *   to stop looking for status changes.
     *   The default is the current date and time.
     *   
     *   Although you can use any date format
     *   supported by the .NET system library's
     *   [`DateTime.Parse()`][msoft] function,
     *   DocuSign recommends
     *   using [ISO 8601][] format dates
     *   with an explicit time zone offset
     *   If you do not provide
     *   a time zone offset,
     *   the method uses the server's time zone.
     *   
     *   For example, the following dates and times refer to the same instant:
     *   
     *   * `2017-05-02T01:44Z`
     *   * `2017-05-01T21:44-04:00`
     *   * `2017-05-01T18:44-07:00`
     *   
     *   
     *   [msoft]: https://msdn.microsoft.com/en-us/library/system.datetime.parse(v=vs.110).aspx#StringToParse
     *   [ISO 8601]: https://en.wikipedia.org/wiki/ISO_8601
     *   .
     * * `transaction_ids: &str` -- If included in the query string, this is a comma separated list of envelope `transactionId`s.
     *   
     *   If included in the `request_body`, this is a list of envelope `transactionId`s.
     *   
     *   ###### Note: `transactionId`s are only valid in the DocuSign system for seven days.
     *   .
     * * `user_filter: &str` -- Returns envelopes where the current user is the recipient, the sender, or the recipient only. (For example, `user_filter=sender`.) Valid values are:
     *   
     *   - `sender`
     *   - `recipient`
     *   - `recipient_only`.
     * * `user_id: &str` -- The ID of the user who created the envelopes to be retrieved. Note that an account can have multiple users, and any user with account access can retrieve envelopes by user_id from the account.
     * * `user_name: &str` -- Limit results to envelopes
     *   sent by the account user
     *   with this user name.
     *   
     *   `email` must be given as well,
     *   and both `email` and `user_name`
     *   must refer to an existing account user.
     *   .
     */
    pub async fn get(
        &self,
        account_id: &str,
        ac_status: &str,
        block: &str,
        cdse_mode: &str,
        continuation_token: &str,
        count: &str,
        custom_field: &str,
        email: &str,
        envelope_ids: &str,
        exclude: &str,
        folder_ids: &str,
        folder_types: &str,
        from_date: &str,
        from_to_status: &str,
        include: &str,
        include_purge_information: &str,
        intersecting_folder_ids: &str,
        last_queried_date: &str,
        order: &str,
        order_by: &str,
        powerformids: &str,
        query_budget: &str,
        requester_date_format: &str,
        search_text: &str,
        start_position: &str,
        status: &str,
        to_date: &str,
        transaction_ids: &str,
        user_filter: &str,
        user_id: &str,
        user_name: &str,
    ) -> ClientResult<crate::types::EnvelopesInformation> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ac_status.is_empty() {
            query_args.push(("ac_status".to_string(), ac_status.to_string()));
        }
        if !block.is_empty() {
            query_args.push(("block".to_string(), block.to_string()));
        }
        if !cdse_mode.is_empty() {
            query_args.push(("cdse_mode".to_string(), cdse_mode.to_string()));
        }
        if !continuation_token.is_empty() {
            query_args.push((
                "continuation_token".to_string(),
                continuation_token.to_string(),
            ));
        }
        if !count.is_empty() {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !custom_field.is_empty() {
            query_args.push(("custom_field".to_string(), custom_field.to_string()));
        }
        if !email.is_empty() {
            query_args.push(("email".to_string(), email.to_string()));
        }
        if !envelope_ids.is_empty() {
            query_args.push(("envelope_ids".to_string(), envelope_ids.to_string()));
        }
        if !exclude.is_empty() {
            query_args.push(("exclude".to_string(), exclude.to_string()));
        }
        if !folder_ids.is_empty() {
            query_args.push(("folder_ids".to_string(), folder_ids.to_string()));
        }
        if !folder_types.is_empty() {
            query_args.push(("folder_types".to_string(), folder_types.to_string()));
        }
        if !from_date.is_empty() {
            query_args.push(("from_date".to_string(), from_date.to_string()));
        }
        if !from_to_status.is_empty() {
            query_args.push(("from_to_status".to_string(), from_to_status.to_string()));
        }
        if !include.is_empty() {
            query_args.push(("include".to_string(), include.to_string()));
        }
        if !include_purge_information.is_empty() {
            query_args.push((
                "include_purge_information".to_string(),
                include_purge_information.to_string(),
            ));
        }
        if !intersecting_folder_ids.is_empty() {
            query_args.push((
                "intersecting_folder_ids".to_string(),
                intersecting_folder_ids.to_string(),
            ));
        }
        if !last_queried_date.is_empty() {
            query_args.push((
                "last_queried_date".to_string(),
                last_queried_date.to_string(),
            ));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !order_by.is_empty() {
            query_args.push(("order_by".to_string(), order_by.to_string()));
        }
        if !powerformids.is_empty() {
            query_args.push(("powerformids".to_string(), powerformids.to_string()));
        }
        if !query_budget.is_empty() {
            query_args.push(("query_budget".to_string(), query_budget.to_string()));
        }
        if !requester_date_format.is_empty() {
            query_args.push((
                "requester_date_format".to_string(),
                requester_date_format.to_string(),
            ));
        }
        if !search_text.is_empty() {
            query_args.push(("search_text".to_string(), search_text.to_string()));
        }
        if !start_position.is_empty() {
            query_args.push(("start_position".to_string(), start_position.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !to_date.is_empty() {
            query_args.push(("to_date".to_string(), to_date.to_string()));
        }
        if !transaction_ids.is_empty() {
            query_args.push(("transaction_ids".to_string(), transaction_ids.to_string()));
        }
        if !user_filter.is_empty() {
            query_args.push(("user_filter".to_string(), user_filter.to_string()));
        }
        if !user_id.is_empty() {
            query_args.push(("user_id".to_string(), user_id.to_string()));
        }
        if !user_name.is_empty() {
            query_args.push(("user_name".to_string(), user_name.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes?{}",
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
     * Creates an envelope.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/envelopes` endpoint.
     *
     * Creates and sends an envelope or creates a draft envelope.
     * Envelopes are fundamental resources in the DocuSign platform.
     *
     * With this method you can:
     *
     * * Create and send an envelope
     *   with [documents][], [recipients][], and [tabs][].
     * * [Create and send an envelope from a template](https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/templates/).
     * * [Create and send an envelope from
     *   a combination of documents and templates](https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/templates/composite/).
     * * Create a draft envelope.
     *
     *
     * When you use this method
     * to create and send an envelope
     * in a single request,
     * the following parameters in the request body (an [`envelopeDefinition`][envelopeDefinition]) are required:
     *
     * | Parameter      | Description |
     * | :--------      | :---------- |
     * | `status`       | Set to `sent` to send the envelope to recipients.<br>Set to `created` (or don't set at all) to save the envelope as a draft. |
     * | `emailSubject` | The subject of the email used to send the envelope. |
     * | `documents`    | The [documents][] to be signed. |
     * | `recipients`   | The email addresses of the envelope [recipients][]. |
     *
     *
     * **Note**: If the envelope has a workflow definition
     * and the `workflowStatus` is `paused`,
     * the envelope will not be sent immediately,
     * even if the envelope's `status` is `sent`.
     *
     * There are many ways to use envelopes.
     * You can create and send an envelope
     * with a single API request,
     * or you can use several API requests
     * to create, populate, and send envelopes.
     *
     *
     * | See:                  | To learn about:                                                                                                                    |
     * | :----------------------- | :--------------------------------------------------------------------------------------------------------------------------------- |
     * | [Envelopes][envelopes]   | Envelopes, [adding documents][addingdocs], [tracking][], [locking][], [deleting][], [templates][]                                  |
     * | [Documents][documents]   | Documents, [attachments][], [supplemental documents][supdocs], [authoritative copies][authcopies], [purging][]                     |
     * | [Recipients][recipients] | Recipients, [recipient types][reciptypes], [recipient status][recipstatus]                                                         |
     * | [Tabs][tabs]             | Tabs, [anchoring tabs][tabanchor],   [custom tabs][tabcustom], [payments][] |
     *
     *
     * [addingdocs]:           https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/envelopes/
     * [attachments]:          https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/documents/
     * [authcopies]:           https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/documents/
     * [conoverview]:          https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/overview
     * [deleting]:             https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/envelopes/
     * [documents]:            https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/documents
     * [envelopeDefinition]:   https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/Envelopes/create/
     * [envelopes]:            https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/envelopes
     * [locking]:              https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/envelopes/
     * [payments]:             https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/tabs/payment/
     * [purging]:              https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/documents/
     * [recipients]:           https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/recipients
     * [recipstatus]:          https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/recipients/
     * [reciptypes]:           https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/recipients/
     * [supdocs]:              https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/documents/
     * [tabanchor]:            https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/tabs/auto-place/
     * [tabcustom]:            https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/tabs/custom-tabs/
     * [tabs]:                 https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/tabs
     * [tabtypes]:             https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/tabs/
     * [templates]:            https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/envelopes/
     * [tracking]:             https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/envelopes/
     *
     * **Note**: When you create an envelope by using a [composite template](https://developers.docusign.com/docs/esign-rest-api/esign101/concepts/templates/composite/), you should specify the envelope custom fields in the inline template. Any custom fields that you specify at the root level are ignored.
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `cdse_mode: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `change_routing_order: &str` -- When true, users can define the routing order of recipients while sending documents for signature.
     * * `completed_documents_only: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `merge_roles_on_draft: &str` -- When set to **true**, template roles will be merged, and empty recipients will be removed. This parameter applies when you create a draft envelope with multiple templates. (To create a draft envelope, the `status` field is set to `created`.)
     *   
     *   **Note**: DocuSign recommends that this parameter should be set to **true** whenever you create a draft envelope with multiple templates.
     */
    pub async fn post(
        &self,
        account_id: &str,
        cdse_mode: &str,
        change_routing_order: &str,
        completed_documents_only: &str,
        merge_roles_on_draft: &str,
        body: &crate::types::EnvelopeDefinition,
    ) -> ClientResult<crate::types::EnvelopeSummary> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cdse_mode.is_empty() {
            query_args.push(("cdse_mode".to_string(), cdse_mode.to_string()));
        }
        if !change_routing_order.is_empty() {
            query_args.push((
                "change_routing_order".to_string(),
                change_routing_order.to_string(),
            ));
        }
        if !completed_documents_only.is_empty() {
            query_args.push((
                "completed_documents_only".to_string(),
                completed_documents_only.to_string(),
            ));
        }
        if !merge_roles_on_draft.is_empty() {
            query_args.push((
                "merge_roles_on_draft".to_string(),
                merge_roles_on_draft.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes?{}",
                crate::progenitor_support::encode_path(account_id),
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
     * Gets envelope statuses for a set of envelopes.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/status` endpoint.
     *
     * Retrieves envelope statuses for a set of envelopes.
     *
     * You must specify _one_ of the following query parameters:
     *
     * | Parameter         | Description                                                                      |
     * | :---------------- | :------------------------------------------------------------------------------- |
     * | `from_date`       | a valid UTC DateTime:  `2016-01-01`                                              |
     * | `envelope_ids`    | A comma-separated list of envelope IDs<br>or the special value `request_body`    |
     * | `transaction_ids` | A comma-separated list of transaction IDs<br>or the special value `request_body` |
     *
     * When you use the special value `request_body`, the request body looks like this:
     *
     * ```
     * {
     *   "envelopeIds": [
     *     "44c5ad6c-xxxx-xxxx-xxxx-ebda5e2dfe15",
     *     "8e26040d-xxxx-xxxx-xxxx-1e29b924d237",
     *     "c8b40a2d-xxxx-xxxx-xxxx-4fe56fe10f95"
     *   ]
     * }
     * ```
     *
     * **Note**: It is an error omit the request body altogether.
     * The request body must be at least `{}`.
     *
     * ### You can find an example of using this API endpoint in the following how-to:
     *
     * * [How to list envelope status changes](https://developers.docusign.com/docs/esign-rest-api/how-to/list-envelope-status-changes/)
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `ac_status: &str` -- Specifies the Authoritative Copy Status for the envelopes. The possible values are:
     *   
     *   - `Unknown`
     *   - `Original`
     *   - `Transferred`
     *   - `AuthoritativeCopy`
     *   - `AuthoritativeCopyExportPending`
     *   - `AuthoritativeCopyExported`
     *   - `DepositPending`
     *   - `Deposited`
     *   - `DepositedEO`
     *   - `DepositFailed`.
     * * `block: &str` -- If set to **true**, removes any results that match one of the provided `transaction_ids`.
     * * `count: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `email: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_ids: &str` -- The envelope IDs to include in the results.
     *   
     *   The value of this property can be:
     *   - A comma-separated list of envelope IDs
     *   - The special value `request_body`. In this case, the method uses the envelope IDs in the request body.
     * * `from_date: &str` -- The date/time setting that specifies when the request begins checking for status changes for envelopes in the account. This is required unless parameters `envelope_ids` and/or `transaction_Ids` are provided.
     *   
     *   ****Note****: This parameter must be set to a valid  `DateTime`, or  `envelope_ids` and/or `transaction_ids` must be specified.
     * * `from_to_status: &str` -- The envelope status that you are checking for. Possible values are:
     *   
     *   
     *   - `Changed` (default)
     *   - `Completed`
     *   - `Created`
     *   - `Declined`
     *   - `Deleted`
     *   - `Delivered`
     *   - `Processing`
     *   - `Sent`
     *   - `Signed`
     *   - `TimedOut`
     *   - `Voided`
     *   
     *   For example, if you specify `Changed`, this method
     *   returns a list of envelopes that changed status
     *   during the `from_date` to `to_date` time period.
     *   .
     * * `start_position: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `status: &str` -- A comma-separated list of envelope status to search for. Possible values are:
     *   
     *   - `completed`
     *   - `created`
     *   - `declined`
     *   - `deleted`
     *   - `delivered`
     *   - `processing`
     *   - `sent`
     *   - `signed`
     *   - `template`
     *   - `voided`
     *   .
     * * `to_date: &str` -- Optional date/time setting
     *   that specifies the last date/time
     *   or envelope status changes in the result set.
     *   
     *   The default value is the time that you call the method.
     *   .
     * * `transaction_ids: &str` -- The transaction IDs to include in the results. Note that transaction IDs are valid for seven days.
     *   
     *   The value of this property can be:
     *   - A list of comma-separated transaction IDs
     *   - The special value `request_body`. In this case, this method uses the transaction IDs in the request body.
     * * `user_name: &str` -- Limits results to envelopes
     *   sent by the account user
     *   with this user name.
     *   
     *   `email` must be given as well,
     *   and both `email` and `user_name`
     *   must refer to an existing account user.
     *   .
     */
    pub async fn put_status(
        &self,
        account_id: &str,
        ac_status: &str,
        block: &str,
        count: &str,
        email: &str,
        envelope_ids: &str,
        from_date: &str,
        from_to_status: &str,
        start_position: &str,
        status: &str,
        to_date: &str,
        transaction_ids: &str,
        user_name: &str,
        body: &crate::types::EnvelopeIdsRequest,
    ) -> ClientResult<crate::types::EnvelopesInformation> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ac_status.is_empty() {
            query_args.push(("ac_status".to_string(), ac_status.to_string()));
        }
        if !block.is_empty() {
            query_args.push(("block".to_string(), block.to_string()));
        }
        if !count.is_empty() {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !email.is_empty() {
            query_args.push(("email".to_string(), email.to_string()));
        }
        if !envelope_ids.is_empty() {
            query_args.push(("envelope_ids".to_string(), envelope_ids.to_string()));
        }
        if !from_date.is_empty() {
            query_args.push(("from_date".to_string(), from_date.to_string()));
        }
        if !from_to_status.is_empty() {
            query_args.push(("from_to_status".to_string(), from_to_status.to_string()));
        }
        if !start_position.is_empty() {
            query_args.push(("start_position".to_string(), start_position.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !to_date.is_empty() {
            query_args.push(("to_date".to_string(), to_date.to_string()));
        }
        if !transaction_ids.is_empty() {
            query_args.push(("transaction_ids".to_string(), transaction_ids.to_string()));
        }
        if !user_name.is_empty() {
            query_args.push(("user_name".to_string(), user_name.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/status?{}",
                crate::progenitor_support::encode_path(account_id),
                query_
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
     * Gets the status of a single envelope.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}` endpoint.
     *
     * Retrieves the overall status for the specified envelope.
     * To get the status of a list of envelopes, use
     * [Envelope: listStatusChanges ](https://developers.docusign.com/docs/esign-rest-api/reference/Envelopes/Envelopes/listStatusChanges/).
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `advanced_update: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include: &str` -- Specifies additional information about the envelope to return. Enter a comma-separated list, such as `tabs,recipients`. Valid values are:
     *   
     *   - `custom_fields`: The custom fields associated with the envelope.
     *   - `documents`: The documents associated with the envelope.
     *   - `attachments`: The attachments associated with the envelope.
     *   - `extensions`: Information about the email settings associated with the envelope.
     *   - `folders`: The folder where the envelope exists.
     *   - `recipients`: The recipients associated with the envelope.
     *   - `powerform`: The PowerForms associated with the envelope.
     *   - `tabs`: The tabs associated with the envelope.
     *   - `payment_tabs`: The payment tabs associated with the envelope.
     *   .
     */
    pub async fn get_envelopes(
        &self,
        account_id: &str,
        envelope_id: &str,
        advanced_update: &str,
        include: &str,
    ) -> ClientResult<crate::types::Envelope> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !advanced_update.is_empty() {
            query_args.push(("advanced_update".to_string(), advanced_update.to_string()));
        }
        if !include.is_empty() {
            query_args.push(("include".to_string(), include.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}?{}",
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
     * Send, void, or modify a draft envelope. Purge documents from a completed envelope.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}` endpoint.
     *
     * This method enables you to make changes to an envelope.
     * You can use it to:
     *
     * * Send a draft envelope
     * * Void an in-process envelope
     * * Modify a draft envelope
     * * Purge documents and envelope metadata from the DocuSign platform
     *
     *
     * <div class="highlight highlight-info">
     * <p markdown="1">
     *
     * Although the request body for this method
     * is a complete envelope definition,
     * you only need to provide
     * the properties that
     * you're updating.
     *
     * </p>
     * </div>
     *
     *
     * ## Sending a Draft Envelope
     *
     * To send a draft envelope, include the following code in the request body:
     *
     * ```json
     * {
     *   "status": "sent"
     * }
     * ```
     *
     * You can attach a workflow before sending the envelope:
     *
     * ```json
     * {
     *   "status": "sent",
     *   "workflow": {
     *     "workflowSteps": [
     *       {
     *         "action": "pause_before",
     *         "description": "pause_before routing order 2",
     *         "itemId": 2,
     *         "triggerOnItem": "routing_order"
     *       }
     *     ]
     *   }
     * }
     * ```
     *
     * ## Working with Workflows
     *
     * To unpause a workflow, the request body should include this:
     *
     * ```json
     * {
     *   "workflow": {
     *     "workflowStatus": "in_progress"
     *   }
     * }
     * ```
     *
     * ## Voiding an In-Process Envelope
     *
     * To void an in-process envelope, include the following code in the request body:
     *
     * ```json
     * {
     *   "status": "voided",
     *   "voidedReason": "The reason for voiding the envelope"
     * }
     * ```
     *
     * ## Modifying Envelope Email Information
     *
     * To change the email subject and message of a draft envelope,
     * include the following code in the request body:
     *
     * ```json
     * {
     *   "emailSubject": "new email subject",
     *   "emailBlurb": "new email message"
     * }
     * ```
     *
     * ## Purging Documents from DocuSign
     *
     *
     * To place only the documents
     * in the purge queue,
     * leaving any
     * corresponding attachments
     * and tabs in the DocuSign platform,
     * set the `purgeState` property
     * to `documents_queued`.
     *
     *
     * ```json
     * {
     *   "purgeState": "documents_queued"
     * }
     * ```
     *
     * To place documents,
     * attachments,
     * and tabs
     * in the purge queue,
     * set the `purgeState` property
     * to `documents_and_metadata_queued`.
     *
     * ```json
     * {
     *   "purgeState": "documents_and_metadata_queued"
     * }
     * ```
     *
     *
     * You can purge documents
     * only from completed envelopes
     * that are not marked as the authoritative copy.
     * The user requesting the purge
     * must have permission to purge documents
     * and
     * must be the sender or be acting on behalf of the sender.
     *
     *
     *
     * When the purge request is initiated
     * the items to be purged
     * are placed in the purge queue
     * for deletion in 14 days.
     * The sender
     * and
     * all recipients with DocuSign accounts
     * associated with the envelope
     * get an email notification
     * the the documents will be deleted in 14 days.
     * The notification contains a link
     * to the documents.
     * A second email notification
     * is sent 7 days later.
     * At the end of the 14-day period
     * the documents are deleted from the system.
     * Recipients without DocuSign accounts
     * do not receive email notifications.
     *
     *
     * If your account has a Document Retention policy,
     * envelope documents
     * are automatically placed
     * in the purge queue,
     * and notification emails are sent
     * at the end of the retention period.
     * Setting a Document Retention policy is the same as setting a
     * schedule for purging documents.
     *
     * ## Removing Documents from the Purge Queue
     *
     * To remove documents from the purge queue, include the following code in the request body:
     *
     * ```
     * {
     *   "purgeState": "documents_dequeued"
     * }
     * ```
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `advanced_update: &str` -- When set to **true**, allows the caller to update recipients, tabs, custom fields, notification, email settings and other envelope attributes.
     * * `resend_envelope: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn put(
        &self,
        account_id: &str,
        envelope_id: &str,
        advanced_update: &str,
        resend_envelope: &str,
        body: &crate::types::Envelope,
    ) -> ClientResult<crate::types::EnvelopeUpdateSummary> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !advanced_update.is_empty() {
            query_args.push(("advanced_update".to_string(), advanced_update.to_string()));
        }
        if !resend_envelope.is_empty() {
            query_args.push(("resend_envelope".to_string(), resend_envelope.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}?{}",
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
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Gets the envelope audit events for an envelope.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/audit_events` endpoint.
     *
     * Gets the envelope audit events for the specified envelope.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn audit_events_get(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<crate::types::EnvelopeAuditEventResponse> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/audit_events",
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
     * Returns document page image(s) based on input.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents/{documentId}/pages` endpoint.
     *
     * Returns images of the pages in a document for display based on the parameters that you specify.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `count: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `dpi: &str` -- The number of dots per inch (DPI) for the resulting images. Valid values are 1-310 DPI. The default value is 94.
     * * `max_height: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `max_width: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `nocache: &str` -- If **true**, using cache is disabled and image information is retrieved from a database. **True** is the default value. .
     * * `show_changes: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `start_position: &str` -- The position within the total result set from which to start returning values. The value **thumbnail** may be used to return the page image.
     */
    pub async fn pages_get_page_image(
        &self,
        account_id: &str,
        document_id: &str,
        envelope_id: &str,
        count: &str,
        dpi: &str,
        max_height: &str,
        max_width: &str,
        nocache: &str,
        show_changes: &str,
        start_position: &str,
    ) -> ClientResult<crate::types::PageImages> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !count.is_empty() {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !dpi.is_empty() {
            query_args.push(("dpi".to_string(), dpi.to_string()));
        }
        if !max_height.is_empty() {
            query_args.push(("max_height".to_string(), max_height.to_string()));
        }
        if !max_width.is_empty() {
            query_args.push(("max_width".to_string(), max_width.to_string()));
        }
        if !nocache.is_empty() {
            query_args.push(("nocache".to_string(), nocache.to_string()));
        }
        if !show_changes.is_empty() {
            query_args.push(("show_changes".to_string(), show_changes.to_string()));
        }
        if !start_position.is_empty() {
            query_args.push(("start_position".to_string(), start_position.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/documents/{}/pages?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
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
     * Deletes a page from a document in an envelope.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents/{documentId}/pages/{pageNumber}` endpoint.
     *
     * Deletes a page from a document in an envelope based on the page number.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `page_number: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn pages_delete_page(
        &self,
        account_id: &str,
        document_id: &str,
        envelope_id: &str,
        page_number: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/documents/{}/pages/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                crate::progenitor_support::encode_path(document_id),
                crate::progenitor_support::encode_path(page_number),
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
     * Gets a page image from an envelope for display.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents/{documentId}/pages/{pageNumber}/page_image` endpoint.
     *
     * Returns an image of a page in a document for display.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `page_number: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `dpi: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `max_height: &str` -- Sets the maximum height for the page image in pixels. The DPI is recalculated based on this setting.
     * * `max_width: &str` -- Sets the maximum width for the page image in pixels. The DPI is recalculated based on this setting.
     * * `show_changes: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn pages_get_page_image_envelopes(
        &self,
        account_id: &str,
        document_id: &str,
        envelope_id: &str,
        page_number: &str,
        dpi: &str,
        max_height: &str,
        max_width: &str,
        show_changes: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !dpi.is_empty() {
            query_args.push(("dpi".to_string(), dpi.to_string()));
        }
        if !max_height.is_empty() {
            query_args.push(("max_height".to_string(), max_height.to_string()));
        }
        if !max_width.is_empty() {
            query_args.push(("max_width".to_string(), max_width.to_string()));
        }
        if !show_changes.is_empty() {
            query_args.push(("show_changes".to_string(), show_changes.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/documents/{}/pages/{}/page_image?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                crate::progenitor_support::encode_path(document_id),
                crate::progenitor_support::encode_path(page_number),
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
     * Rotates page image from an envelope for display.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/documents/{documentId}/pages/{pageNumber}/page_image` endpoint.
     *
     * Rotates page image from an envelope for display. The page image can be rotated to the left or right.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `document_id: &str` -- The `documentId` is set by the API client. It is an integer that falls between `1` and 2,147,483,647. The value is encoded as a string without commas. The values `1`, `2`, `3`, and so on are typically used to identify the first few documents in an envelope. Tab definitions include a `documentId` property that specifies the document on which to place the tab.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `page_number: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn pages_put_page_image(
        &self,
        account_id: &str,
        document_id: &str,
        envelope_id: &str,
        page_number: &str,
        body: &crate::types::PageRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/documents/{}/pages/{}/page_image",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                crate::progenitor_support::encode_path(document_id),
                crate::progenitor_support::encode_path(page_number),
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
     * Gets envelope notification information.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/notification` endpoint.
     *
     * Retrieves the envelope notification, reminders and expirations, information for an existing envelope.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn notification_get(
        &self,
        account_id: &str,
        envelope_id: &str,
    ) -> ClientResult<crate::types::Notification> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/notification",
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
     * Sets envelope notifications for an existing envelope.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/notification` endpoint.
     *
     * This method sets the notifications (reminders and expirations) for an existing envelope. The request body sends a structure containing reminders and expirations settings. It also specifies whether to use the settings specified in the request, or the account default notification settings for the envelope.
     *
     * Note that this request only specifies when notifications are sent; it does not initiate sending of email messages.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn notification_put(
        &self,
        account_id: &str,
        envelope_id: &str,
        body: &crate::types::EnvelopeNotificationRequest,
    ) -> ClientResult<crate::types::Notification> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/notification",
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
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Gets the initials image for a user.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/recipients/{recipientId}/initials_image` endpoint.
     *
     * Retrieves the initials image for the specified user. The image is returned in the same format as it was uploaded. In the request you can specify if the chrome (the added line and identifier around the initial image) is returned with the image.
     *
     * The userId specified in the endpoint must match the authenticated user's user id and the user must be a member of the account.
     *
     * The `signatureIdOrName` paramter accepts signature ID or signature name. DocuSign recommends you use signature ID (`signatureId`), since some names contain characters that do not properly URL encode. If you use the user name, it is likely that the name includes spaces and you might need to URL encode the name before using it in the endpoint.
     *
     * For example: "Bob Smith" to "Bob%20Smith"
     *
     * Older envelopes might only contain chromed images. If getting the non-chromed image fails, try getting the chromed image.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `recipient_id: &str` -- A local reference that senders use to map recipients to other objects, such as specific document tabs. Within an envelope, each `recipientId` must be unique, but there is no uniqueness requirement across envelopes. For example, many envelopes assign the first recipient a `recipientId` of `1`.
     * * `include_chrome: &str` -- The added line and identifier around the initial image. Note: Older envelopes might only have chromed images. If getting the non-chromed image fails, try getting the chromed image.
     */
    pub async fn recipients_get_recipient_initials_image(
        &self,
        account_id: &str,
        envelope_id: &str,
        recipient_id: &str,
        include_chrome: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include_chrome.is_empty() {
            query_args.push(("include_chrome".to_string(), include_chrome.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/recipients/{}/initials_image?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                crate::progenitor_support::encode_path(recipient_id),
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
     * Sets the initials image for an accountless signer.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/recipients/{recipientId}/initials_image` endpoint.
     *
     * Updates the initials image for a signer that does not have a DocuSign account. The supported image formats for this file are: gif, png, jpeg, and bmp. The file size must be less than 200K.
     *
     * For the Authentication/Authorization for this call, the credentials must match the sender of the envelope, the recipient must be an accountless signer or in person signer. The account must have the `CanSendEnvelope` property set to **true** and the `ExpressSendOnly` property in `SendingUser` structure must be set to **false**.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `recipient_id: &str` -- A local reference that senders use to map recipients to other objects, such as specific document tabs. Within an envelope, each `recipientId` must be unique, but there is no uniqueness requirement across envelopes. For example, many envelopes assign the first recipient a `recipientId` of `1`.
     */
    pub async fn recipients_put_recipient_initials_image(
        &self,
        account_id: &str,
        envelope_id: &str,
        recipient_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/recipients/{}/initials_image",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                crate::progenitor_support::encode_path(recipient_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Gets signature information for a signer or sign-in-person recipient.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/recipients/{recipientId}/signature` endpoint.
     *
     * Retrieves signature information for a signer or sign-in-person recipient.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `recipient_id: &str` -- A local reference that senders use to map recipients to other objects, such as specific document tabs. Within an envelope, each `recipientId` must be unique, but there is no uniqueness requirement across envelopes. For example, many envelopes assign the first recipient a `recipientId` of `1`.
     */
    pub async fn recipients_get_recipient_signature(
        &self,
        account_id: &str,
        envelope_id: &str,
        recipient_id: &str,
    ) -> ClientResult<crate::types::UserSignature> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/recipients/{}/signature",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                crate::progenitor_support::encode_path(recipient_id),
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
     * Retrieve signature image information for a signer/sign-in-person recipient.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/recipients/{recipientId}/signature_image` endpoint.
     *
     * Retrieves the specified user signature image. The image is returned in the same format as uploaded. In the request you can specify if the chrome (the added line and identifier around the initial image) is returned with the image.
     *
     * The userId specified in the endpoint must match the authenticated user's user ID and the user must be a member of the account.
     *
     * The `signatureIdOrName` parameter accepts signature ID or signature name. DocuSign recommends you use signature ID (`signatureId`), since some names contain characters that don't properly URL encode. If you use the user name, it is likely that the name includes spaces and you might need to URL encode the name before using it in the endpoint.
     *
     * For example: "Bob Smith" to "Bob%20Smith"
     *
     * Older envelopes might only have chromed images. If getting the non-chromed image fails, try getting the chromed image.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `recipient_id: &str` -- A local reference that senders use to map recipients to other objects, such as specific document tabs. Within an envelope, each `recipientId` must be unique, but there is no uniqueness requirement across envelopes. For example, many envelopes assign the first recipient a `recipientId` of `1`.
     * * `include_chrome: &str` -- When set to **true**, the response includes the chromed version of the signature image.
     */
    pub async fn recipients_get_recipient_signature_image(
        &self,
        account_id: &str,
        envelope_id: &str,
        recipient_id: &str,
        include_chrome: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include_chrome.is_empty() {
            query_args.push(("include_chrome".to_string(), include_chrome.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/recipients/{}/signature_image?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                crate::progenitor_support::encode_path(recipient_id),
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
     * Sets the signature image for an accountless signer.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/envelopes/{envelopeId}/recipients/{recipientId}/signature_image` endpoint.
     *
     * Updates the signature image for an accountless signer. The supported image formats for this file are: gif, png, jpeg, and bmp. The file size must be less than 200K.
     *
     * For the Authentication/Authorization for this call, the credentials must match the sender of the envelope, the recipient must be an accountless signer or in person signer. The account must have the `CanSendEnvelope` property set to **true** and the `ExpressSendOnly` property in `SendingUser` structure must be set to **false**.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `envelope_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `recipient_id: &str` -- A local reference that senders use to map recipients to other objects, such as specific document tabs. Within an envelope, each `recipientId` must be unique, but there is no uniqueness requirement across envelopes. For example, many envelopes assign the first recipient a `recipientId` of `1`.
     */
    pub async fn recipients_put_recipient_signature_image(
        &self,
        account_id: &str,
        envelope_id: &str,
        recipient_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/envelopes/{}/recipients/{}/signature_image",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(envelope_id),
                crate::progenitor_support::encode_path(recipient_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}
