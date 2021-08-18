use anyhow::Result;

use crate::Client;

pub struct Accounts {
    pub client: Client,
}

impl Accounts {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Accounts { client }
    }

    /**
     * List sub accounts.
     *
     * This function performs a `GET` to the `/accounts` endpoint.
     *
     * List all the sub accounts that have been created by a master account.<br><br>Zoom allows only [approved partners](https://marketplace.zoom.us/docs/api-reference/master-account-apis) to use master APIs and manage sub accounts. Email the partner programs team at **partner-success@zoom.us** for more details.
     *
     * <br>**Prerequisites:**<br>
     * * Pro or a higher paid account with master account option enabled. <br>
     *
     * **Scope**: `account:read:admin`
     * <br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     *
     *
     *
     * **Parameters:**
     *
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `page_number: i64` --
     *   **Deprecated** - This field has been deprecated and we will stop supporting it completely in a future release. Please use "next_page_token" for pagination instead of this field.
     *   
     *   The page number of the current page in the returned records.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn get(
        &self,
        page_size: i64,
        page_number: i64,
        next_page_token: &str,
    ) -> Result<crate::types::Domains> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_number > 0 {
            query_args.push(("page_number".to_string(), page_number.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/accounts?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Create a sub account.
     *
     * This function performs a `POST` to the `/accounts` endpoint.
     *
     * Create a sub account under a master account. Your account must be a master account in order to create sub accounts.
     * <br><br>Zoom allows only [approved partners](https://marketplace.zoom.us/docs/api-reference/master-account-apis) to use master APIs and manage sub accounts. Email the partner programs team at partner-success@zoom.us. for more details. Please note that the created account user will receive a confirmation email.<br><br>
     * <br>
     * **Prerequisites:**<br>
     * * Pro or a higher paid account with master account option enabled. <br>
     *
     * **Scope**: `account:write:admin`<br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     *
     *
     *  
     */
    pub async fn create(
        &self,
        body: &crate::types::AccountCreateRequest,
    ) -> Result<crate::types::AccountCreateResponse> {
        let url = "/accounts".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get sub account details.
     *
     * This function performs a `GET` to the `/accounts/{accountId}` endpoint.
     *
     * Get details of a sub account that is listed under a master account. Your account must be a master account in order to retrieve sub accounts' details. Zoom allows only [approved partners](https://marketplace.zoom.us/docs/api-reference/master-account-apis) to use master APIs and create sub accounts. Email the partner programs team at **partner-success@zoom.us** for more details.
     *
     * **Prerequisites:**
     * * Pro or a higher paid account with master account option enabled. <br>
     *
     * **Scope**: `account:write:admin`<br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     *
     *
     *  
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- User's first name.
     */
    pub async fn account(&self, account_id: &str) -> Result<crate::types::AccountResponse> {
        let url = format!(
            "/accounts/{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Disassociate a sub account.
     *
     * This function performs a `DELETE` to the `/accounts/{accountId}` endpoint.
     *
     * Disassociate a sub account from its master account. This will leave the sub account intact but it will no longer be associated with the master account.<br>  
     *
     * **Prerequisites:**
     * * Pro or a higher paid account with master account option enabled. <br>
     * * The account making this API request must be a [master account](https://marketplace.zoom.us/docs/api-reference/master-account-apis).<br><br>
     *
     *
     * **Scope**: `account:write:admin`<br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     *
     *
     *  
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- User's first name.
     */
    pub async fn disassociate(&self, account_id: &str) -> Result<()> {
        let url = format!(
            "/accounts/{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update options.
     *
     * This function performs a `PATCH` to the `/accounts/{accountId}/options` endpoint.
     *
     * Update a sub account's options under the master account.<br> <aside>Your account must be a master account in order to update the options for sub accounts. Zoom only assigns this privilege to trusted partners. </aside>
     *
     * **Prerequisites:**
     * * Pro or a higher paid account with master account option enabled.
     * * The account making this API request must be a [master account](https://marketplace.zoom.us/docs/api-reference/master-account-apis).<br><br>
     *
     * **Scope**: `account:write:admin`<br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     *
     *
     *  
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- User's first name.
     */
    pub async fn options_update(
        &self,
        account_id: &str,
        body: &crate::types::Options,
    ) -> Result<()> {
        let url = format!(
            "/accounts/{}/options",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get settings.
     *
     * This function performs a `GET` to the `/accounts/{accountId}/settings` endpoint.
     *
     * Use this API to get an account's settings information.
     *
     * To get the settings of a master account, use `me` as the value for the `accountId` path parameter.
     *
     * **Scopes:** `account:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites**:
     * * The account must be a paid account
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- User's first name.
     * * `option: crate::types::OptionData` -- Use the following options to filter the results of the account's information:
     *  \* `meeting_authentication` — View the account's [meeting authentication settings](https://support.zoom.us/hc/en-us/articles/360037117472-Authentication-Profiles-for-Meetings-and-Webinars).
     *  \* `recording_authentication` — View the account's [recording authentication settings](https://support.zoom.us/hc/en-us/articles/360037756671-Authentication-Profiles-for-Cloud-Recordings).
     *  \* `security` — View the account's security settings. For example, password requirements for user login or two-factor authentication.<br>
     *  \* `meeting_security` — View the account's meeting security settings.
     * * `custom_query_fields: &str` -- The name of the field by which to filter the response. For example, if you provide the `host_video` value for this field, you will get a response similar to the following:
     *   
     *   `{ "schedule_meeting": { "host_video": false    } }`
     *   
     *   To use multiple values, comma-separate each value. For example: `host_video,participant_video`.
     */
    pub async fn settings_domains(
        &self,
        account_id: &str,
        option: crate::types::OptionData,
        custom_query_fields: &str,
    ) -> Result<crate::types::Domains> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !custom_query_fields.is_empty() {
            query_args.push((
                "custom_query_fields".to_string(),
                custom_query_fields.to_string(),
            ));
        }
        if !option.to_string().is_empty() {
            query_args.push(("option".to_string(), option.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/accounts/{}/settings?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get settings.
     *
     * This function performs a `GET` to the `/accounts/{accountId}/settings` endpoint.
     *
     * Use this API to get an account's settings information.
     *
     * To get the settings of a master account, use `me` as the value for the `accountId` path parameter.
     *
     * **Scopes:** `account:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites**:
     * * The account must be a paid account
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- User's first name.
     * * `option: crate::types::OptionData` -- Use the following options to filter the results of the account's information:
     *  \* `meeting_authentication` — View the account's [meeting authentication settings](https://support.zoom.us/hc/en-us/articles/360037117472-Authentication-Profiles-for-Meetings-and-Webinars).
     *  \* `recording_authentication` — View the account's [recording authentication settings](https://support.zoom.us/hc/en-us/articles/360037756671-Authentication-Profiles-for-Cloud-Recordings).
     *  \* `security` — View the account's security settings. For example, password requirements for user login or two-factor authentication.<br>
     *  \* `meeting_security` — View the account's meeting security settings.
     * * `custom_query_fields: &str` -- The name of the field by which to filter the response. For example, if you provide the `host_video` value for this field, you will get a response similar to the following:
     *   
     *   `{ "schedule_meeting": { "host_video": false    } }`
     *   
     *   To use multiple values, comma-separate each value. For example: `host_video,participant_video`.
     */
    pub async fn settings_security(
        &self,
        account_id: &str,
        option: crate::types::OptionData,
        custom_query_fields: &str,
    ) -> Result<crate::types::Security> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !custom_query_fields.is_empty() {
            query_args.push((
                "custom_query_fields".to_string(),
                custom_query_fields.to_string(),
            ));
        }
        if !option.to_string().is_empty() {
            query_args.push(("option".to_string(), option.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/accounts/{}/settings?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get settings.
     *
     * This function performs a `GET` to the `/accounts/{accountId}/settings` endpoint.
     *
     * Use this API to get an account's settings information.
     *
     * To get the settings of a master account, use `me` as the value for the `accountId` path parameter.
     *
     * **Scopes:** `account:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites**:
     * * The account must be a paid account
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- User's first name.
     * * `option: crate::types::OptionData` -- Use the following options to filter the results of the account's information:
     *  \* `meeting_authentication` — View the account's [meeting authentication settings](https://support.zoom.us/hc/en-us/articles/360037117472-Authentication-Profiles-for-Meetings-and-Webinars).
     *  \* `recording_authentication` — View the account's [recording authentication settings](https://support.zoom.us/hc/en-us/articles/360037756671-Authentication-Profiles-for-Cloud-Recordings).
     *  \* `security` — View the account's security settings. For example, password requirements for user login or two-factor authentication.<br>
     *  \* `meeting_security` — View the account's meeting security settings.
     * * `custom_query_fields: &str` -- The name of the field by which to filter the response. For example, if you provide the `host_video` value for this field, you will get a response similar to the following:
     *   
     *   `{ "schedule_meeting": { "host_video": false    } }`
     *   
     *   To use multiple values, comma-separate each value. For example: `host_video,participant_video`.
     */
    pub async fn settings_account(
        &self,
        account_id: &str,
        option: crate::types::OptionData,
        custom_query_fields: &str,
    ) -> Result<crate::types::AccountSettings> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !custom_query_fields.is_empty() {
            query_args.push((
                "custom_query_fields".to_string(),
                custom_query_fields.to_string(),
            ));
        }
        if !option.to_string().is_empty() {
            query_args.push(("option".to_string(), option.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/accounts/{}/settings?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get settings.
     *
     * This function performs a `GET` to the `/accounts/{accountId}/settings` endpoint.
     *
     * Use this API to get an account's settings information.
     *
     * To get the settings of a master account, use `me` as the value for the `accountId` path parameter.
     *
     * **Scopes:** `account:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites**:
     * * The account must be a paid account
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- User's first name.
     * * `option: crate::types::OptionData` -- Use the following options to filter the results of the account's information:
     *  \* `meeting_authentication` — View the account's [meeting authentication settings](https://support.zoom.us/hc/en-us/articles/360037117472-Authentication-Profiles-for-Meetings-and-Webinars).
     *  \* `recording_authentication` — View the account's [recording authentication settings](https://support.zoom.us/hc/en-us/articles/360037756671-Authentication-Profiles-for-Cloud-Recordings).
     *  \* `security` — View the account's security settings. For example, password requirements for user login or two-factor authentication.<br>
     *  \* `meeting_security` — View the account's meeting security settings.
     * * `custom_query_fields: &str` -- The name of the field by which to filter the response. For example, if you provide the `host_video` value for this field, you will get a response similar to the following:
     *   
     *   `{ "schedule_meeting": { "host_video": false    } }`
     *   
     *   To use multiple values, comma-separate each value. For example: `host_video,participant_video`.
     */
    pub async fn settings_meeting_security(
        &self,
        account_id: &str,
        option: crate::types::OptionData,
        custom_query_fields: &str,
    ) -> Result<crate::types::MeetingSecuritySettings> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !custom_query_fields.is_empty() {
            query_args.push((
                "custom_query_fields".to_string(),
                custom_query_fields.to_string(),
            ));
        }
        if !option.to_string().is_empty() {
            query_args.push(("option".to_string(), option.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/accounts/{}/settings?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get settings.
     *
     * This function performs a `GET` to the `/accounts/{accountId}/settings` endpoint.
     *
     * Use this API to get an account's settings information.
     *
     * To get the settings of a master account, use `me` as the value for the `accountId` path parameter.
     *
     * **Scopes:** `account:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites**:
     * * The account must be a paid account
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- User's first name.
     * * `option: crate::types::OptionData` -- Use the following options to filter the results of the account's information:
     *  \* `meeting_authentication` — View the account's [meeting authentication settings](https://support.zoom.us/hc/en-us/articles/360037117472-Authentication-Profiles-for-Meetings-and-Webinars).
     *  \* `recording_authentication` — View the account's [recording authentication settings](https://support.zoom.us/hc/en-us/articles/360037756671-Authentication-Profiles-for-Cloud-Recordings).
     *  \* `security` — View the account's security settings. For example, password requirements for user login or two-factor authentication.<br>
     *  \* `meeting_security` — View the account's meeting security settings.
     * * `custom_query_fields: &str` -- The name of the field by which to filter the response. For example, if you provide the `host_video` value for this field, you will get a response similar to the following:
     *   
     *   `{ "schedule_meeting": { "host_video": false    } }`
     *   
     *   To use multiple values, comma-separate each value. For example: `host_video,participant_video`.
     */
    pub async fn setting(
        &self,
        account_id: &str,
        option: crate::types::OptionData,
        custom_query_fields: &str,
    ) -> Result<crate::types::AccountSettingsResponseOneOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !custom_query_fields.is_empty() {
            query_args.push((
                "custom_query_fields".to_string(),
                custom_query_fields.to_string(),
            ));
        }
        if !option.to_string().is_empty() {
            query_args.push(("option".to_string(), option.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/accounts/{}/settings?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Update settings.
     *
     * This function performs a `PATCH` to the `/accounts/{accountId}/settings` endpoint.
     *
     * Update the settings of a sub account that is under a master account.<br> To update the settings of the master account, use `me` as the value of the `accountId` path parameter.<br><br>
     * **Prerequisites**:
     *  * The sub account must be a paid account.<br>
     * **Scopes**: `account:write:admin`
     * <br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- User's first name.
     * * `option: crate::types::AccountSettingsUpdateOption`
     */
    pub async fn settings_update(
        &self,
        account_id: &str,
        option: crate::types::AccountSettingsUpdateOption,
        body: &crate::types::AccountSettingsUpdateRequestOneOf,
    ) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !option.to_string().is_empty() {
            query_args.push(("option".to_string(), option.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/accounts/{}/settings?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            query_
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get managed domains.
     *
     * This function performs a `GET` to the `/accounts/{accountId}/managed_domains` endpoint.
     *
     * Get a sub account's [managed domains](https://support.zoom.us/hc/en-us/articles/203395207-What-is-Managed-Domain-).<br><br>
     *
     * **Note:** This API can be used by Zoom Accounts that are on a Pro or a higher plan as well accounts that have master and sub accounts options enabled. <br><br>
     * To get managed domains of the master account, provide `me` as the value for accountId in the path parameter. Provide the sub account's Account ID as the value of accountId path parameter to get managed domains of the sub account.<br><br>
     * **Prerequisites:**<br>
     * * Pro or a higher paid account with master account option enabled. <br>
     *
     * **Scope:** `account:read:admin`<br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     *
     *
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- Unique Identifier of the account. To retrieve locked settings of the master account or a regular account, provide "me" as the value of this field. <br> To retrieve locked settings of a sub account, provide the Account ID of the sub account in this field.
     */
    pub async fn managed_domain(&self, account_id: &str) -> Result<crate::types::DomainsList> {
        let url = format!(
            "/accounts/{}/managed_domains",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Get trusted domains.
     *
     * This function performs a `GET` to the `/accounts/{accountId}/trusted_domains` endpoint.
     *
     * Get trusted domains of a sub account. To get the trusted domains of a master account, use `me` as the value for the `accountId` path parameter.
     *
     * **Prerequisites:**<br>
     * * The sub account must be a paid account.<br>
     * **Scope:** `account:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- User's first name.
     */
    pub async fn trusted_domain(
        &self,
        account_id: &str,
    ) -> Result<crate::types::AccountTrustedDomainResponse> {
        let url = format!(
            "/accounts/{}/trusted_domains",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Get locked settings.
     *
     * This function performs a `GET` to the `/accounts/{accountId}/lock_settings` endpoint.
     *
     * [Account Locked Settings](https://support.zoom.us/hc/en-us/articles/115005269866) allow you turn settings on or off for all users in your account. No user except the account admin or account owner can change these settings. With lock settings, you force the settings on for all users.
     * Use this API to retrieve an account's locked settings.
     *
     * **Note:** This API can be used by Zoom Accounts that are on a Pro or a higher plan as well accounts that have master and sub accounts options enabled. <br><br>
     * **Prerequisites:**
     * * Pro or a higher paid account. <br>
     *
     * **Scope**: `account:read:admin`.
     * <br> **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     *
     *
     *
     *
     *
     * **Scope:** account:read:admin
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- Unique Identifier of the account. To retrieve locked settings of the master account or a regular account, provide "me" as the value of this field. <br> To retrieve locked settings of a sub account, provide the Account ID of the sub account in this field.
     * * `option: &str` -- `meeting_security`: Use this query parameter to view meeting security settings applied on the account.<br>.
     * * `custom_query_fields: &str` -- Provide the name of the field by which you would like to filter the response. For example, if you provide "host_video" as the value of this field, you will get a response similar to the following:<br>
     *   {
     *       "schedule_meeting": {
     *           "host_video": false
     *       }
     *   }
     *   <br>You can provide multiple values by separating them with commas(example: "host_video,participant_video”).
     */
    pub async fn get_lock_setting(
        &self,
        account_id: &str,
        option: &str,
        custom_query_fields: &str,
    ) -> Result<crate::types::Domains> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !custom_query_fields.is_empty() {
            query_args.push((
                "custom_query_fields".to_string(),
                custom_query_fields.to_string(),
            ));
        }
        if !option.is_empty() {
            query_args.push(("option".to_string(), option.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/accounts/{}/lock_settings?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Update locked settings.
     *
     * This function performs a `PATCH` to the `/accounts/{accountId}/lock_settings` endpoint.
     *
     * [Account Locked Settings](https://support.zoom.us/hc/en-us/articles/115005269866) allow you turn settings on or off for all users in your account. No user except the account admin or account owner can change these settings. With lock settings, you force the settings on for all users. Use this API to update an account's locked settings.
     *
     * **Note:** This API can be used by Zoom Accounts that are on a Pro or a higher plan as well accounts that have master and sub accounts options enabled.<br><br>
     * **Prerequisites:**<br>
     * * Pro or a higher paid account. <br>
     *
     * **Scope:** `account:write:admin`<br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     *
     *
     *  
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- Unique Identifier of the account. To retrieve locked settings of the master account or a regular account, provide "me" as the value of this field. <br> To retrieve locked settings of a sub account, provide the Account ID of the sub account in this field.
     */
    pub async fn update_lock_settings(&self, account_id: &str) -> Result<()> {
        let url = format!(
            "/accounts/{}/lock_settings",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client.patch(&url, None).await
    }

    /**
     * Update the account owner.
     *
     * This function performs a `PUT` to the `/accounts/{accountId}/owner` endpoint.
     *
     * Use this API to change an account's owner.
     *
     * An account's current owner can [change the account's owner](https://support.zoom.us/hc/en-us/articles/115005686983-Change-Account-Owner) to another user on the same account.
     *
     * **Scopes:** `account:write:admin` or `account:master`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     *
     * **Prerequisites:**
     * * An account owner or admin permissions of an account
     * * The account making this API request must be on a Pro or a higher account plan with [Master account](https://marketplace.zoom.us/docs/api-reference/master-account-apis) privileges
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The account's account ID.
     */
    pub async fn update_owner(&self, account_id: &str, body: &crate::types::Members) -> Result<()> {
        let url = format!(
            "/accounts/{}/owner",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Upload virtual background files.
     *
     * This function performs a `POST` to the `/accounts/{accountId}/settings/virtual_backgrounds` endpoint.
     *
     * Use this API to [upload virtual background files](https://support.zoom.us/hc/en-us/articles/210707503-Virtual-Background#h_01EJF3YFEWGT8YA0ZJ079JEDQE) for all users on the account to use.
     *
     *
     * **Prerequisites:**<br>
     * * Virtual background feature must be [enabled](https://support.zoom.us/hc/en-us/articles/210707503-Virtual-Background#h_2ef28080-fce9-4ac2-b567-dc958afab1b7) on the account.
     * <br> **Scope:** `account:write:admin`<br><br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- Unique identifier of the account.
     */
    pub async fn upload_vb(
        &self,
        account_id: &str,
        body: &crate::types::UploadVbRequest,
    ) -> Result<crate::types::Files> {
        let url = format!(
            "/accounts/{}/settings/virtual_backgrounds",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete virtual background files.
     *
     * This function performs a `DELETE` to the `/accounts/{accountId}/settings/virtual_backgrounds` endpoint.
     *
     * Delete existing virtual background file(s) from an account.
     *
     * **Prerequisites:**<br>
     * * Virtual background feature must be [enabled](https://support.zoom.us/hc/en-us/articles/210707503-Virtual-Background#h_2ef28080-fce9-4ac2-b567-dc958afab1b7) on the account.
     * <br> **Scope:** `account:write:admin`<br> <br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`<br>
     *
     *
     *
     * **Parameters:**
     *
     * * `file_ids: &str` -- Provide the id of the file that is to be deleted. To delete multiple files, provide comma separated values for this field.
     */
    pub async fn del_vb(&self, account_id: &str, file_ids: &str) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !file_ids.is_empty() {
            query_args.push(("file_ids".to_string(), file_ids.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/accounts/{}/settings/virtual_backgrounds?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            query_
        );

        self.client.delete(&url, None).await
    }
}
