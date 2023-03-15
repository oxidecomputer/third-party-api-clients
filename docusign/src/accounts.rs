use crate::Client;
use crate::ClientResult;

pub struct Accounts {
    pub client: Client,
}

impl Accounts {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Accounts { client }
    }

    /**
     * Creates new accounts.
     *
     * This function performs a `POST` to the `/v2.1/accounts` endpoint.
     *
     * Creates new DocuSign accounts.
     * You can use this method to create
     * a single account
     * or up to 100 accounts at a time.
     *
     * **Note**:  This method is restricted to partner integrations. You must work with DocuSign Professional Services or DocuSign Business Development, who will provide you with the Distributor Code and Distributor Password that you need to include in the request body.
     *
     * You must include the `X-DocuSign-Authentication`  header.
     *
     * Example:
     *
     * `<DocuSignCredentials><IntegratorKey>{{integratorKey}}</IntegratorKey></DocuSignCredentials>`
     *
     * When creating a single account,
     * the body of the request is a
     * [`newAccountRequest`][newAccountRequest]
     * object.
     *
     * Example:
     *
     * ```
     * {
     * 	"newAccountRequest": [
     * 		{
     * 			"accountName":"Test Account",
     * 			"distributorCode":"MY_DIST_CODE",
     * 			"distributorPassword":"MY_DIST_PWD",
     * 			"initialUser":{
     * 				"email":"user@emaildomain.com",
     * 				"firstName":"John",
     * 				"middleName": "Harry",
     * 				"lastName":"Doe",
     * 				"suffixName": "",
     * 				"userName": "John Doe",
     * 				"jobTitle": "Engineer",
     * 				"company": "Test Company"
     * 			},
     * 			"addressInformation":{
     * 				"address1": "1234 Main Street",
     * 				"address2": "Suite 100",
     * 				"city": "Seattle",
     * 				"state": "WA",
     * 				"postalCode": "98101",
     * 				"country": "US",
     * 				"phone": "1234567890",
     * 				"fax": "1234567891"
     * 			},
     * 			"planInformation":{
     * 				"planId":"37085696-xxxx-xxxx-xxxx-7ea067752959"
     * 			},
     * 			"referralInformation":{
     * 				"includedSeats": "1",
     * 				"referralCode": "code",
     * 				"referrerName": "name"
     * 			}
     * 		}
     * 	]
     * }
     *
     * ```
     * If the request succeeds,
     * it returns a
     * 201 (Created) HTTP response code.
     * The response returns the new account ID, password, and the default user
     * information for each newly created account.
     *
     *
     * When creating multiple accounts,
     * the body of the request is a
     * `newAccountRequests`
     * object,
     * which contains one or more
     * [`newAccountDefinition`][newAccountDefinition]
     * objects.
     * You can create up to 100 new accounts
     * at a time this way.
     *
     * The body for a multi-account
     * creation request
     * looks like this in JSON:
     *
     * ```
     * {
     *   "newAccountRequests": [
     *     {
     *       "accountName": "accountone",
     *       . . .
     *     },
     *     {
     *       "accountName": "accounttwo",
     *       . . .
     *     }
     *   ]
     * }
     * ```
     *
     * A multi-account request
     * looks like this in XML:
     *
     * ```
     * <newAccountsDefinition xmlns:i="http://www.w3.org/2001/XMLSchema-instance" xmlns="http://www.docusign.com/restapi">
     *   <newAccountRequests>
     *     <newAccountDefinition>
     *       . . .
     *     </newAccountDefinition>
     *     <newAccountDefinition>
     *       . . .
     *     </newAccountDefinition>
     *   </newAccountRequests>
     * </newAccountsDefinition>
     * ```
     *
     * A multi-account creation request
     * may succeed (report a 201 code)
     * even if some accounts could not be created.
     * In this case, the `errorDetails` property
     * in the response contains specific information
     * about the failure.
     *
     *
     *
     * [newAccountDefinition]: #/definitions/newAccountDefinition
     * [nameValue]: #/definitions/nameValue
     * [newAccountRequest]: #/definitions/newAccountRequest
     *
     */
    pub async fn post(
        &self,
        body: &crate::types::NewAccountDefinition,
    ) -> ClientResult<crate::types::NewAccountSummary> {
        let url = self.client.url("/v2.1/accounts", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieves the account provisioning information for the account.
     *
     * This function performs a `GET` to the `/v2.1/accounts/provisioning` endpoint.
     *
     * Retrieves the account provisioning information for the account.
     */
    pub async fn get_provisioning(&self) -> ClientResult<crate::types::ProvisioningInformation> {
        let url = self.client.url("/v2.1/accounts/provisioning", None);
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
     * Retrieves the account information for the specified account.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}` endpoint.
     *
     * Retrieves the account information for the specified account.
     *
     * **Response**
     * The `canUpgrade` property contains is a Boolean that indicates whether the account can be upgraded through the API.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include_account_settings: &str` -- When set to **true**, includes account settings
     *   in the response. If you omit this parameter, the default behavior is **false**.
     */
    pub async fn get(
        &self,
        account_id: &str,
        include_account_settings: &str,
    ) -> ClientResult<crate::types::AccountInformation> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include_account_settings.is_empty() {
            query_args.push((
                "include_account_settings".to_string(),
                include_account_settings.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}?{}",
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
     * Deletes the specified account.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}` endpoint.
     *
     * This closes the specified account. You must be an account admin to close your account. Once closed, an account must be reopened by DocuSign.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn delete(&self, account_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}",
                crate::progenitor_support::encode_path(account_id),
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
     * Gets list of recurring and usage charges for the account.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/billing_charges` endpoint.
     *
     * Retrieves the list of recurring and usage charges for the account. This can be used to determine the charge structure and usage of charge plan items.
     *
     * Privileges required: account administrator
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include_charges: &str` -- Specifies which billing charges to return.
     *   Valid values are:
     *   
     *   * envelopes
     *   * seats
     *   .
     */
    pub async fn billing_charges_get(
        &self,
        account_id: &str,
        include_charges: &str,
    ) -> ClientResult<crate::types::BillingChargeResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include_charges.is_empty() {
            query_args.push(("include_charges".to_string(), include_charges.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/billing_charges?{}",
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
     * Deletes the signature for one or more captive recipient records.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/captive_recipients/{recipientPart}` endpoint.
     *
     * This method deletes the signature for one or more captive recipient records. It is primarily used for testing. This functionality provides a way to reset the signature associated with a client user ID so that a new signature can be created the next time the client user ID is used.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `recipient_part: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn captive_recipients_delete_part(
        &self,
        account_id: &str,
        recipient_part: &str,
        body: &crate::types::CaptiveRecipientInformation,
    ) -> ClientResult<crate::types::CaptiveRecipientInformation> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/captive_recipients/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(recipient_part),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Gets the recipient names associated with an email address.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/recipient_names` endpoint.
     *
     * Retrieves a list of all of the names associated with the email address that you pass in. This list can include variants of a single recipient's name that are used for signing, as well as the names of multiple different recipients.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `email: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn recipient_names_get(
        &self,
        account_id: &str,
        email: &str,
    ) -> ClientResult<crate::types::RecipientNamesResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !email.is_empty() {
            query_args.push(("email".to_string(), email.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/recipient_names?{}",
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
     * Gets account settings information.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/settings` endpoint.
     *
     * Retrieves the account settings information for the specified account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn settings_get(
        &self,
        account_id: &str,
    ) -> ClientResult<crate::types::AccountSettingsInformation> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/settings",
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
     * Updates the account settings for an account.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/settings` endpoint.
     *
     * Updates the account settings for the specified account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn settings_put(
        &self,
        account_id: &str,
        body: &crate::types::AccountSettingsInformation,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/settings",
                crate::progenitor_support::encode_path(account_id),
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
     * Gets the envelope purge configuration for an account.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/settings/envelope_purge_configuration` endpoint.
     *
     * An envelope purge configuration enables account administrators to permanently remove documents and their field data from completed and voided envelopes after a specified retention period (`retentionDays`). This method retrieves the current envelope purge configuration for your account.
     *
     * **Note**: To use this method, you must be an account administrator.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_purge_configuration_get(
        &self,
        account_id: &str,
    ) -> ClientResult<crate::types::EnvelopePurgeConfiguration> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/settings/envelope_purge_configuration",
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
     * Sets the envelope purge configuration for an account.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/settings/envelope_purge_configuration` endpoint.
     *
     * An envelope purge configuration enables account administrators to permanently remove documents and their field data from completed and voided envelopes after a specified retention period (`retentionDays`). This method sets the envelope purge configuration for your account.
     *
     * **Note**: To use this method, you must be an account administrator.
     *
     * For more information, see [Purge Envelopes](https://support.docusign.com/en/guides/ndse-user-guide-purge-envelopes).
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn envelope_purge_configuration_put(
        &self,
        account_id: &str,
        body: &crate::types::EnvelopePurgeConfiguration,
    ) -> ClientResult<crate::types::EnvelopePurgeConfiguration> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/settings/envelope_purge_configuration",
                crate::progenitor_support::encode_path(account_id),
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
     * Gets envelope notification defaults.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/settings/notification_defaults` endpoint.
     *
     * This method returns the default settings for the email notifications that signers and senders receive about envelopes.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn notification_defaults_get(
        &self,
        account_id: &str,
    ) -> ClientResult<crate::types::NotificationDefaultsData> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/settings/notification_defaults",
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
     * Updates envelope notification default settings.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/settings/notification_defaults` endpoint.
     *
     * This method changes the default settings for the email notifications that signers and senders receive about envelopes.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn notification_defaults_put(
        &self,
        account_id: &str,
        body: &crate::types::NotificationDefaultsData,
    ) -> ClientResult<crate::types::NotificationDefaultsData> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/settings/notification_defaults",
                crate::progenitor_support::encode_path(account_id),
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
     * Reserved: Gets the shared item status for one or more users.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/shared_access` endpoint.
     *
     * Retrieves shared item status for one or more users and types of items.
     *
     * Users with account administration privileges can retrieve shared access information for all account users. Users without account administrator privileges can only retrieve shared access information for themselves, and the returned information is limited to retrieving the status of the members of the account that are sharing their folders to the user. This is equivalent to setting the `shared` parameter to `shared_from`.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `count: &str` -- Specifies the maximum number of results included in the response. If no value is specified, this defaults to 1000.
     * * `envelopes_not_shared_user_status: &str` -- This query parameter works in conjunction with `user_ids`. When you specify one of the following user statuses, the query limits the results to only users that match the specified status:
     *   - `ActivationRequired`: Membership Activation required
     *   - `ActivationSent`: Membership activation sent to user
     *   - `Active`: User Membership is active
     *   - `Closed`: User Membership is closed
     *   - `Disabled`: User Membership is disabled.
     * * `folder_ids: &str` -- A comma-separated list of folder IDs for which to return shared item information. If `item_type` is set to `folders`, at least one folder ID is required.
     * * `item_type: &str` -- Specifies the type of shared item being requested. The possible values are:
     *   
     *   - `envelopes`: Get information about envelope sharing between users.
     *   - `templates`: Get information about template sharing among users and groups.
     *   - `folders`: Get information about folder sharing among users and groups.
     *   .
     * * `search_text: &str` -- Filter user names based on the specified string. The wild-card '*' (asterisk) can be used in the string.
     * * `shared: &str` -- A comma-separated list of sharing filters that specifies which users appear in the response.
     *   
     *   - `not_shared`: The response lists users who do not share items of `item_type` with the current user.
     *   
     *   - `shared_to`: The response lists users in `user_list` who are sharing items to current user.
     *   
     *   - `shared_from`: The response lists users in `user_list` who are sharing items from the current user.
     *   
     *   - `shared_to_and_from`: The response lists users in `user_list` who are sharing items to and from the current user.
     *   
     *   If the current user does not have administrative privileges, only the `shared_to` option is valid.
     * * `start_position: &str` -- If the number of responses is greater than `count`, this specifies the number of responses to skip. Typically this value is a multiple of `count`. The default is 0.
     * * `user_ids: &str` -- A comma-separated list of user IDs for whom the shared item information is being requested.
     */
    pub async fn shared_access_get(
        &self,
        account_id: &str,
        count: &str,
        envelopes_not_shared_user_status: &str,
        folder_ids: &str,
        item_type: &str,
        search_text: &str,
        shared: &str,
        start_position: &str,
        user_ids: &str,
    ) -> ClientResult<crate::types::AccountSharedAccess> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !count.is_empty() {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !envelopes_not_shared_user_status.is_empty() {
            query_args.push((
                "envelopes_not_shared_user_status".to_string(),
                envelopes_not_shared_user_status.to_string(),
            ));
        }
        if !folder_ids.is_empty() {
            query_args.push(("folder_ids".to_string(), folder_ids.to_string()));
        }
        if !item_type.is_empty() {
            query_args.push(("item_type".to_string(), item_type.to_string()));
        }
        if !search_text.is_empty() {
            query_args.push(("search_text".to_string(), search_text.to_string()));
        }
        if !shared.is_empty() {
            query_args.push(("shared".to_string(), shared.to_string()));
        }
        if !start_position.is_empty() {
            query_args.push(("start_position".to_string(), start_position.to_string()));
        }
        if !user_ids.is_empty() {
            query_args.push(("user_ids".to_string(), user_ids.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/shared_access?{}",
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
     * Reserved: Sets the shared access information for users.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/shared_access` endpoint.
     *
     * This sets the shared access status for one or more users or templates.
     *
     * When setting user shared access, only users with account administration privileges can set shared access status for envelopes.
     *
     * When setting template shared access, only users who own a template and have sharing permission or with account administration privileges can set shared access for templates.
     *
     * Changes to the shared items status are not additive. The change always replaces the current status.
     *
     * To change template shared access, add the query parameter `item_type` = `templates` to the request. When this is set, the user and envelopes properties are not required.
     *
     * **Note**: This functionality is a newer version of the [Update Group Share](https://developers.docusign.com/docs/esign-rest-api/reference/Templates/Templates/updateGroupShare) functionality.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `item_type: &str` -- Specifies the type of shared item being set:
     *   - `envelopes`: Set envelope sharing between users.
     *   - `templates`: Set information about template sharing among users and groups.
     *   - `folders`: Get information about folder sharing among users and groups.
     *   .
     * * `preserve_existing_shared_access: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_ids: &str` -- A comma-separated list of IDs for users whose shared item access is being set.
     */
    pub async fn shared_access_put(
        &self,
        account_id: &str,
        item_type: &str,
        preserve_existing_shared_access: &str,
        user_ids: &str,
        body: &crate::types::AccountSharedAccess,
    ) -> ClientResult<crate::types::AccountSharedAccess> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !item_type.is_empty() {
            query_args.push(("item_type".to_string(), item_type.to_string()));
        }
        if !preserve_existing_shared_access.is_empty() {
            query_args.push((
                "preserve_existing_shared_access".to_string(),
                preserve_existing_shared_access.to_string(),
            ));
        }
        if !user_ids.is_empty() {
            query_args.push(("user_ids".to_string(), user_ids.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/shared_access?{}",
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
     * Gets the supported languages for envelope recipients.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/supported_languages` endpoint.
     *
     * Retrieves a list of supported languages that you can set for an individual recipient when creating an envelope, as well as their simple type enumeration values. These are the languages that you can set for the standard email format and signing view for each recipient.
     *
     * For example, in the recipient's email notification, this setting affects elements such as the standard introductory text describing the request to sign. It also determines the language used for buttons and tabs in both the email notification and the signing experience.
     *
     * **Note**: Setting a language for a recipient affects only the DocuSign standard text. Any custom text that you enter for the `emailBody` and `emailSubject` of the notification is not translated, and appears exactly as you enter it.
     *
     * For more information, see [Set Recipient Language and Specify Custom Email Messages](https://support.docusign.com/en/guides/ndse-user-guide-recipient-language).
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn supported_languages_get(
        &self,
        account_id: &str,
    ) -> ClientResult<crate::types::SupportedLanguages> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/supported_languages",
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
     * Gets a list of unsupported file types.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/unsupported_file_types` endpoint.
     *
     * Retrieves a list of file types (mime-types and file-extensions) that are not supported for upload through the DocuSign system.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn unsupported_file_types_get(
        &self,
        account_id: &str,
    ) -> ClientResult<crate::types::FileTypeList> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/unsupported_file_types",
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
     * Retrieves an account settings comparison.
     *
     * This function performs a `GET` to the `/v2.1/organization_exports/{organizationId}/account_settings/{resultId}` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `organization_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `result_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn organization_exports_get_settings_export(
        &self,
        organization_id: &str,
        result_id: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/organization_exports/{}/account_settings/{}",
                crate::progenitor_support::encode_path(organization_id),
                crate::progenitor_support::encode_path(result_id),
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
}
