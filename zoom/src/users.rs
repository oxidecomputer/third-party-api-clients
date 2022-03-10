use anyhow::Result;

use crate::Client;

pub struct Users {
    pub client: Client,
}

impl Users {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Users { client }
    }

    /**
     * List users.
     *
     * This function performs a `GET` to the `/users` endpoint.
     *
     * Use this API to list your account's users.
     *
     * **Scopes:** `user:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `status: crate::types::UsersStatus` -- The user's status:
     *  \* `active` — An active user.
     *  \* `inactive` — A deactivated user.
     *  \* `pending` — A pending user.
     *  
     *  This value defaults to `active`.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `role_id: &str` -- The role's unique ID. Use this parameter to filter the response by a specific role. You can use the [List Roles](https://marketplace.zoom.us/docs/api-reference/zoom-api/roles/roles) API to get a role's unique ID value.
     * * `page_number: &str` -- The page number of the current page in the returned records.
     * * `include_fields: crate::types::UsersIncludeFields` -- Use this parameter to display one of the following attributes in the API call's response:
     *  \* `custom_attributes` — Return the user's custom attributes.
     *  \* `host_key` — Return the user's [host key](https://support.zoom.us/hc/en-us/articles/205172555-Using-your-host-key).
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn get_page(
        &self,
        status: crate::types::UsersStatus,
        page_size: i64,
        role_id: &str,
        page_number: &str,
        include_fields: crate::types::UsersIncludeFields,
        next_page_token: &str,
    ) -> Result<Vec<crate::types::UsersResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include_fields.to_string().is_empty() {
            query_args.push(("include_fields".to_string(), include_fields.to_string()));
        }
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if !page_number.is_empty() {
            query_args.push(("page_number".to_string(), page_number.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !role_id.is_empty() {
            query_args.push(("role_id".to_string(), role_id.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/users?{}", query_);

        let resp: crate::types::UsersResponseData = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.users)
    }

    /**
     * List users.
     *
     * This function performs a `GET` to the `/users` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * Use this API to list your account's users.
     *
     * **Scopes:** `user:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     */
    pub async fn get_all(
        &self,
        status: crate::types::UsersStatus,
        role_id: &str,
        include_fields: crate::types::UsersIncludeFields,
    ) -> Result<Vec<crate::types::UsersResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include_fields.to_string().is_empty() {
            query_args.push(("include_fields".to_string(), include_fields.to_string()));
        }
        if !role_id.is_empty() {
            query_args.push(("role_id".to_string(), role_id.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/users?{}", query_);

        let mut resp: crate::types::UsersResponseData = self.client.get(&url, None).await?;

        let mut users = resp.users;
        let mut page = resp.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            // Check if we already have URL params and need to concat the token.
            if !url.contains('?') {
                resp = self
                    .client
                    .get(&format!("{}?next_page_token={}", url, page), None)
                    .await?;
            } else {
                resp = self
                    .client
                    .get(&format!("{}&next_page_token={}", url, page), None)
                    .await?;
            }

            users.append(&mut resp.users);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(users)
    }

    /**
     * Create users.
     *
     * This function performs a `POST` to the `/users` endpoint.
     *
     * A Zoom account can have one or more users. Use this API to add a new user to your account.<br><br>
     * **Prerequisites:**<br>
     * * Pro or higher plan<br><br>
     * **Scopes:** `user:write:admin` `user:write`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     */
    pub async fn create(
        &self,
        body: &crate::types::UserCreateRequest,
    ) -> Result<crate::types::UserCreateResponse> {
        let url = "/users".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get a user.
     *
     * This function performs a `GET` to the `/users/{userId}` endpoint.
     *
     * View a specific user's information on a Zoom account. A Zoom account can have one or more users. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * <p style="background-color:#e1f5fe; color:#000000; padding:8px"> <b>Note:</b> Users who have not activated their account have a status of <i>pending</i>, and the <code>created_at</code> time displays the time at which the API call was made.</p>
     *
     * **Scopes:** `user:read:admin`, `user:read`, `user_info:read`<p style="background-color:#e1f5fe; color:#000000; padding:8px"> <b>Note:</b> The `user_info:read` scope is only available when you pass the `me` value for the `$userId` value.</p></br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     * * `login_type: crate::types::LoginType` -- The user's login method:
     *  
     *  `0` — Facebook OAuth</br>`1` — Google OAuth</br>`24` — Apple OAuth</br>`27` — Microsoft OAuth</br>`97` — Mobile device</br>`98` — RingCentral OAuth</br>`99` — API user</br>`100` — Zoom Work email</br>`101` — Single Sign-On (SSO)
     *  
     *  The following login methods are only available in China:
     *  
     *  `11` — Phone number</br>`21` — WeChat</br>`23` — Alipay.
     * * `encrypted_email: bool` -- Enable/disable the option for a sub account to use shared [Virtual Room Connector(s)](https://support.zoom.us/hc/en-us/articles/202134758-Getting-Started-With-Virtual-Room-Connector) that are set up by the master account. Virtual Room Connectors can only be used by On-prem users.
     */
    pub async fn user(
        &self,
        user_id: &str,
        login_type: crate::types::LoginType,
        encrypted_email: bool,
    ) -> Result<crate::types::UserResponseAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if encrypted_email {
            query_args.push(("encrypted_email".to_string(), encrypted_email.to_string()));
        }
        if !login_type.to_string().is_empty() {
            query_args.push(("login_type".to_string(), login_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/users/{}?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a user.
     *
     * This function performs a `DELETE` to the `/users/{userId}` endpoint.
     *
     * Use this API to disassociate (unlink) a user or permanently delete a user. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Deleting** a user **permanently** removes the user and their data from Zoom. Users can create a new Zoom account using the same email address. An account owner or an account admin can transfer meetings, webinars and cloud recordings to another Zoom user account before deleting.
     *
     * **Disassociating** a user unlinks the user from the associated Zoom account and provides the user their own basic free Zoom account. The disassociated user can then purchase their own Zoom licenses. An account owner or account admin can transfer the user's meetings, webinars, and cloud recordings to another user before disassociation.
     *
     * **Scopes:** `user:write:admin`, `user:write`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     * * `action: crate::types::UserDeleteAction` -- Delete action options:<br>`disassociate` - Disassociate a user.<br>`delete`-  Permanently delete a user.<br>Note: To delete pending user in the account, use `disassociate`.
     * * `transfer_email: &str` -- User's first name.
     * * `transfer_meeting: bool` -- Enable/disable the option for a sub account to use shared [Virtual Room Connector(s)](https://support.zoom.us/hc/en-us/articles/202134758-Getting-Started-With-Virtual-Room-Connector) that are set up by the master account. Virtual Room Connectors can only be used by On-prem users.
     * * `transfer_webinar: bool` -- Enable/disable the option for a sub account to use shared [Virtual Room Connector(s)](https://support.zoom.us/hc/en-us/articles/202134758-Getting-Started-With-Virtual-Room-Connector) that are set up by the master account. Virtual Room Connectors can only be used by On-prem users.
     * * `transfer_recording: bool` -- Enable/disable the option for a sub account to use shared [Virtual Room Connector(s)](https://support.zoom.us/hc/en-us/articles/202134758-Getting-Started-With-Virtual-Room-Connector) that are set up by the master account. Virtual Room Connectors can only be used by On-prem users.
     */
    pub async fn delete(
        &self,
        user_id: &str,
        action: crate::types::UserDeleteAction,
        transfer_email: &str,
        transfer_meeting: bool,
        transfer_webinar: bool,
        transfer_recording: bool,
    ) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !action.to_string().is_empty() {
            query_args.push(("action".to_string(), action.to_string()));
        }
        if !transfer_email.is_empty() {
            query_args.push(("transfer_email".to_string(), transfer_email.to_string()));
        }
        if transfer_meeting {
            query_args.push(("transfer_meeting".to_string(), transfer_meeting.to_string()));
        }
        if transfer_recording {
            query_args.push((
                "transfer_recording".to_string(),
                transfer_recording.to_string(),
            ));
        }
        if transfer_webinar {
            query_args.push(("transfer_webinar".to_string(), transfer_webinar.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/users/{}?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update a user.
     *
     * This function performs a `PATCH` to the `/users/{userId}` endpoint.
     *
     * Update information on a user's [Zoom profile](https://support.zoom.us/hc/en-us/articles/201363203-My-Profile). For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `user:write:admin` `user:write`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     * * `login_type: crate::types::LoginType` -- The user's login method:
     *  
     *  `0` — Facebook OAuth</br>`1` — Google OAuth</br>`24` — Apple OAuth</br>`27` — Microsoft OAuth</br>`97` — Mobile device</br>`98` — RingCentral OAuth</br>`99` — API user</br>`100` — Zoom Work email</br>`101` — Single Sign-On (SSO)
     *  
     *  The following login methods are only available in China:
     *  
     *  `11` — Phone number</br>`21` — WeChat</br>`23` — Alipay.
     */
    pub async fn update(
        &self,
        user_id: &str,
        login_type: crate::types::LoginType,
        body: &crate::types::UserUpdate,
    ) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !login_type.to_string().is_empty() {
            query_args.push(("login_type".to_string(), login_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/users/{}?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get user's ZAK.
     *
     * This function performs a `GET` to the `/users/me/zak` endpoint.
     *
     * Get User’s Zoom Access Token (ZAK). You can use a ZAK to enable a non-login user to join a meeting on your app. Non-login users do not need to enter their username and password to join meetings.
     *
     * **Scope:** `user_zak:read`<br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     */
    pub async fn zak(&self) -> Result<crate::types::UserZakResponse> {
        let url = "/users/me/zak".to_string();
        self.client.get(&url, None).await
    }

    /**
     * List user assistants.
     *
     * This function performs a `GET` to the `/users/{userId}/assistants` endpoint.
     *
     * List a user's assistants. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * Assistants are the users to whom the current user has assigned [scheduling privilege](https://support.zoom.us/hc/en-us/articles/201362803-Scheduling-Privilege). These assistants can schedule meeting on behalf of the current user as well as manage and act as an alternative host for all meetings if the admin has enabled [Co-host option](https://zoom.us/account/setting) on the account.
     *
     * **Scopes:** `user:read:admin`, `user:read`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites:**
     * * Current user as well as the assistant must have Licensed or an On-prem license.
     * * Assistants must be under the current user's account.
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     */
    pub async fn assistant(&self, user_id: &str) -> Result<crate::types::UserAssistantsList> {
        let url = format!(
            "/users/{}/assistants",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Add assistants.
     *
     * This function performs a `POST` to the `/users/{userId}/assistants` endpoint.
     *
     * Use this API to assign assistants to a user. In the request body, provide either the user's ID or the user's email address. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * Assistants are the users to whom the current user has assigned [scheduling privilege](https://support.zoom.us/hc/en-us/articles/201362803-Scheduling-Privilege). These assistants can schedule meeting on behalf of the current user as well as manage and act as an alternative host for all meetings if the admin has enabled [Co-host option](https://zoom.us/account/setting) on the account.
     *
     * **Scopes:** `user:write:admin`, `user:write`</br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites:**
     * * The user as well as the assistant must have Licensed or an On-prem license.
     * * Assistants must be under the current user's account.
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     */
    pub async fn assistant_create(
        &self,
        user_id: &str,
        body: &crate::types::UserAssistantsList,
    ) -> Result<crate::types::AddRoleMembersResponse> {
        let url = format!(
            "/users/{}/assistants",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete user assistants.
     *
     * This function performs a `DELETE` to the `/users/{userId}/assistants` endpoint.
     *
     * Delete all assistants of the current user. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * Assistants are the users to whom the current user has assigned [scheduling privilege](https://support.zoom.us/hc/en-us/articles/201362803-Scheduling-Privilege). These assistants can schedule meeting on behalf of the current user as well as manage and act as an alternative host for all meetings if the admin has enabled [Co-host option](https://zoom.us/account/setting) on the account.
     *
     * **Scopes:** `user:write:admin`, `user:write`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * The user as well as the assistant must have Licensed or an On-prem license.
     * * Assistants must be under the current user's account.
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     */
    pub async fn assistants_delete(&self, user_id: &str) -> Result<()> {
        let url = format!(
            "/users/{}/assistants",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Delete a user assistant.
     *
     * This function performs a `DELETE` to the `/users/{userId}/assistants/{assistantId}` endpoint.
     *
     * Delete a specific assistant of a user. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * Assistants are the users to whom the current user has assigned [scheduling privilege](https://support.zoom.us/hc/en-us/articles/201362803-Scheduling-Privilege). These assistants can schedule meeting on behalf of the current user as well as manage and act as an alternative host for all meetings if the admin has enabled [Co-host option](https://zoom.us/account/setting) on the account.
     *
     * **Scopes:** `user:write:admin`, `user:write`</br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * The user as well as the assistant must have Licensed or an On-prem license.
     * * Assistants must be under the current user's account.
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     * * `assistant_id: &str` -- User's first name.
     */
    pub async fn assistant_delete(&self, user_id: &str, assistant_id: &str) -> Result<()> {
        let url = format!(
            "/users/{}/assistants/{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&assistant_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * List user schedulers.
     *
     * This function performs a `GET` to the `/users/{userId}/schedulers` endpoint.
     *
     * List all the schedulers of a user. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * Schedulers in this context are the users for whom the current user can schedule meetings for. For example, if the current user (the user whose `userId` was passed in the `path` parameter) is "user A", the response of this API will contain a list of user(s), for whom user A can schedule and manage meetings. User A is the assistant of these users and thus has scheduling privilege for these user(s).
     *
     * **Scopes:** `user:read:admin`, `user:read`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites:**
     * * Current user must be under the same account as the scheduler.
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     */
    pub async fn scheduler(&self, user_id: &str) -> Result<crate::types::UserSchedulersList> {
        let url = format!(
            "/users/{}/schedulers",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete user schedulers.
     *
     * This function performs a `DELETE` to the `/users/{userId}/schedulers` endpoint.
     *
     * Delete all of a user's schedulers. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * Schedulers are users on whose behalf the current user (assistant) can schedule meetings for. By calling this API, the current user will no longer be a scheduling assistant of any user.
     *
     * **Scopes:** `user:write:admin`, `user:write`</br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Current user (assistant) must be under the same account as the scheduler.
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     */
    pub async fn schedulers_delete(&self, user_id: &str) -> Result<()> {
        let url = format!(
            "/users/{}/schedulers",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Delete a scheduler.
     *
     * This function performs a `DELETE` to the `/users/{userId}/schedulers/{schedulerId}` endpoint.
     *
     * Delete a scheduler. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * Schedulers are users on whose behalf the current user (assistant) can schedule meetings for. By calling this API, the current user will no longer be a scheduling assistant of this scheduler.
     *
     * **Scopes:** `user:write:admin`, `user:write`</br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Current user must be under the same account as the scheduler.
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     * * `scheduler_id: &str` -- User's first name.
     */
    pub async fn scheduler_delete(&self, user_id: &str, scheduler_id: &str) -> Result<()> {
        let url = format!(
            "/users/{}/schedulers/{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&scheduler_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Upload a user's profile picture.
     *
     * This function performs a `POST` to the `/users/{userId}/picture` endpoint.
     *
     * Upload a user's profile picture. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * Provide `multipart/form-data` as the value of the `content-type` header for this request. This API supports `.jpeg` and `.png` file formats.
     *
     * **Scopes:** `user:write:admin`, `user:write`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     */
    pub async fn picture(&self, user_id: &str) -> Result<()> {
        let url = format!(
            "/users/{}/picture",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
     * Get user settings.
     *
     * This function performs a `GET` to the `/users/{userId}/settings` endpoint.
     *
     * Retrieve a user's settings. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `user:read:admin`, `user:read`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     * * `login_type: crate::types::LoginType` -- The user's login method:
     *  
     *  `0` — Facebook OAuth</br>`1` — Google OAuth</br>`24` — Apple OAuth</br>`27` — Microsoft OAuth</br>`97` — Mobile device</br>`98` — RingCentral OAuth</br>`99` — API user</br>`100` — Zoom Work email</br>`101` — Single Sign-On (SSO)
     *  
     *  The following login methods are only available in China:
     *  
     *  `11` — Phone number</br>`21` — WeChat</br>`23` — Alipay.
     * * `option: crate::types::OptionData` -- Use the following options to filter the results of the account's information:
     *  \* `meeting_authentication` — View the account's [meeting authentication settings](https://support.zoom.us/hc/en-us/articles/360037117472-Authentication-Profiles-for-Meetings-and-Webinars).
     *  \* `recording_authentication` — View the account's [recording authentication settings](https://support.zoom.us/hc/en-us/articles/360037756671-Authentication-Profiles-for-Cloud-Recordings).
     *  \* `security` — View the account's security settings. For example, password requirements for user login or two-factor authentication.<br>
     *  \* `meeting_security` — View the account's meeting security settings.
     * * `custom_query_fields: &str` -- Provide the name of the field by which you would like to filter the response. For example, if you provide "host_video" as the value of this field, you will get a response similar to the following:<br>
     *   {
     *       "schedule_meeting": {
     *           "host_video": false
     *       }
     *   }
     *   <br>You can provide multiple values by separating them with commas(example: "host_video,participant_video”).
     */
    pub async fn settings_domains(
        &self,
        user_id: &str,
        login_type: crate::types::LoginType,
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
        if !login_type.to_string().is_empty() {
            query_args.push(("login_type".to_string(), login_type.to_string()));
        }
        if !option.to_string().is_empty() {
            query_args.push(("option".to_string(), option.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/users/{}/settings?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get user settings.
     *
     * This function performs a `GET` to the `/users/{userId}/settings` endpoint.
     *
     * Retrieve a user's settings. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `user:read:admin`, `user:read`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     * * `login_type: crate::types::LoginType` -- The user's login method:
     *  
     *  `0` — Facebook OAuth</br>`1` — Google OAuth</br>`24` — Apple OAuth</br>`27` — Microsoft OAuth</br>`97` — Mobile device</br>`98` — RingCentral OAuth</br>`99` — API user</br>`100` — Zoom Work email</br>`101` — Single Sign-On (SSO)
     *  
     *  The following login methods are only available in China:
     *  
     *  `11` — Phone number</br>`21` — WeChat</br>`23` — Alipay.
     * * `option: crate::types::OptionData` -- Use the following options to filter the results of the account's information:
     *  \* `meeting_authentication` — View the account's [meeting authentication settings](https://support.zoom.us/hc/en-us/articles/360037117472-Authentication-Profiles-for-Meetings-and-Webinars).
     *  \* `recording_authentication` — View the account's [recording authentication settings](https://support.zoom.us/hc/en-us/articles/360037756671-Authentication-Profiles-for-Cloud-Recordings).
     *  \* `security` — View the account's security settings. For example, password requirements for user login or two-factor authentication.<br>
     *  \* `meeting_security` — View the account's meeting security settings.
     * * `custom_query_fields: &str` -- Provide the name of the field by which you would like to filter the response. For example, if you provide "host_video" as the value of this field, you will get a response similar to the following:<br>
     *   {
     *       "schedule_meeting": {
     *           "host_video": false
     *       }
     *   }
     *   <br>You can provide multiple values by separating them with commas(example: "host_video,participant_video”).
     */
    pub async fn settings_user(
        &self,
        user_id: &str,
        login_type: crate::types::LoginType,
        option: crate::types::OptionData,
        custom_query_fields: &str,
    ) -> Result<crate::types::UserSettings> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !custom_query_fields.is_empty() {
            query_args.push((
                "custom_query_fields".to_string(),
                custom_query_fields.to_string(),
            ));
        }
        if !login_type.to_string().is_empty() {
            query_args.push(("login_type".to_string(), login_type.to_string()));
        }
        if !option.to_string().is_empty() {
            query_args.push(("option".to_string(), option.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/users/{}/settings?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get user settings.
     *
     * This function performs a `GET` to the `/users/{userId}/settings` endpoint.
     *
     * Retrieve a user's settings. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `user:read:admin`, `user:read`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     * * `login_type: crate::types::LoginType` -- The user's login method:
     *  
     *  `0` — Facebook OAuth</br>`1` — Google OAuth</br>`24` — Apple OAuth</br>`27` — Microsoft OAuth</br>`97` — Mobile device</br>`98` — RingCentral OAuth</br>`99` — API user</br>`100` — Zoom Work email</br>`101` — Single Sign-On (SSO)
     *  
     *  The following login methods are only available in China:
     *  
     *  `11` — Phone number</br>`21` — WeChat</br>`23` — Alipay.
     * * `option: crate::types::OptionData` -- Use the following options to filter the results of the account's information:
     *  \* `meeting_authentication` — View the account's [meeting authentication settings](https://support.zoom.us/hc/en-us/articles/360037117472-Authentication-Profiles-for-Meetings-and-Webinars).
     *  \* `recording_authentication` — View the account's [recording authentication settings](https://support.zoom.us/hc/en-us/articles/360037756671-Authentication-Profiles-for-Cloud-Recordings).
     *  \* `security` — View the account's security settings. For example, password requirements for user login or two-factor authentication.<br>
     *  \* `meeting_security` — View the account's meeting security settings.
     * * `custom_query_fields: &str` -- Provide the name of the field by which you would like to filter the response. For example, if you provide "host_video" as the value of this field, you will get a response similar to the following:<br>
     *   {
     *       "schedule_meeting": {
     *           "host_video": false
     *       }
     *   }
     *   <br>You can provide multiple values by separating them with commas(example: "host_video,participant_video”).
     */
    pub async fn settings_meeting_security(
        &self,
        user_id: &str,
        login_type: crate::types::LoginType,
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
        if !login_type.to_string().is_empty() {
            query_args.push(("login_type".to_string(), login_type.to_string()));
        }
        if !option.to_string().is_empty() {
            query_args.push(("option".to_string(), option.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/users/{}/settings?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get user settings.
     *
     * This function performs a `GET` to the `/users/{userId}/settings` endpoint.
     *
     * Retrieve a user's settings. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `user:read:admin`, `user:read`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     * * `login_type: crate::types::LoginType` -- The user's login method:
     *  
     *  `0` — Facebook OAuth</br>`1` — Google OAuth</br>`24` — Apple OAuth</br>`27` — Microsoft OAuth</br>`97` — Mobile device</br>`98` — RingCentral OAuth</br>`99` — API user</br>`100` — Zoom Work email</br>`101` — Single Sign-On (SSO)
     *  
     *  The following login methods are only available in China:
     *  
     *  `11` — Phone number</br>`21` — WeChat</br>`23` — Alipay.
     * * `option: crate::types::OptionData` -- Use the following options to filter the results of the account's information:
     *  \* `meeting_authentication` — View the account's [meeting authentication settings](https://support.zoom.us/hc/en-us/articles/360037117472-Authentication-Profiles-for-Meetings-and-Webinars).
     *  \* `recording_authentication` — View the account's [recording authentication settings](https://support.zoom.us/hc/en-us/articles/360037756671-Authentication-Profiles-for-Cloud-Recordings).
     *  \* `security` — View the account's security settings. For example, password requirements for user login or two-factor authentication.<br>
     *  \* `meeting_security` — View the account's meeting security settings.
     * * `custom_query_fields: &str` -- Provide the name of the field by which you would like to filter the response. For example, if you provide "host_video" as the value of this field, you will get a response similar to the following:<br>
     *   {
     *       "schedule_meeting": {
     *           "host_video": false
     *       }
     *   }
     *   <br>You can provide multiple values by separating them with commas(example: "host_video,participant_video”).
     */
    pub async fn setting(
        &self,
        user_id: &str,
        login_type: crate::types::LoginType,
        option: crate::types::OptionData,
        custom_query_fields: &str,
    ) -> Result<crate::types::UserSettingsResponseOneOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !custom_query_fields.is_empty() {
            query_args.push((
                "custom_query_fields".to_string(),
                custom_query_fields.to_string(),
            ));
        }
        if !login_type.to_string().is_empty() {
            query_args.push(("login_type".to_string(), login_type.to_string()));
        }
        if !option.to_string().is_empty() {
            query_args.push(("option".to_string(), option.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/users/{}/settings?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Update user settings.
     *
     * This function performs a `PATCH` to the `/users/{userId}/settings` endpoint.
     *
     * Update a user's settings. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `user:write:admin`, `user:write`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `option: crate::types::UserSettingsUpdateOption`
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     */
    pub async fn settings_update(
        &self,
        option: crate::types::UserSettingsUpdateOption,
        user_id: &str,
        body: &crate::types::UserSettingsUpdateRequestOneOf,
    ) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !option.to_string().is_empty() {
            query_args.push(("option".to_string(), option.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/users/{}/settings?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Update user status.
     *
     * This function performs a `PUT` to the `/users/{userId}/status` endpoint.
     *
     * Use this API to [deactivate](https://support.zoom.us/hc/en-us/articles/115005269946-Remove-User-from-your-Account#h_6a9bc1c3-d739-4945-b1f2-00b3b88fb5cc) an active user or to [reactivate](https://support.zoom.us/hc/en-us/articles/115005269946-Remove-User-from-your-Account#h_16319724-d120-4be6-af5d-31582d134ea0) a deactivated user. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * An account owner or admins can deactivate as well as activate a user in a Zoom account. Deactivating a user will remove all licenses associated with a user. It will prevent the deactivated user from logging into their Zoom account. A deactivated user can be reactivated. Reactivating a user grants the user access to login to their Zoom account.
     *
     * **Scopes:** `user:write:admin`, `user:write`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     */
    pub async fn status(
        &self,
        user_id: &str,
        body: &crate::types::UserStatusRequest,
    ) -> Result<()> {
        let url = format!(
            "/users/{}/status",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Update a user's password.
     *
     * This function performs a `PUT` to the `/users/{userId}/password` endpoint.
     *
     * Update the [password](https://support.zoom.us/hc/en-us/articles/206344385-Change-a-User-s-Password) of a user using which the user can login to Zoom. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * After this request is processed successfully, an email notification will be sent to the user stating that the password was changed.<br>
     * **Scopes:** `user:write:admin` `user:write`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     * **Prerequisites:**<br>
     * * Owner or admin of the Zoom account.
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     */
    pub async fn password(
        &self,
        user_id: &str,
        body: &crate::types::UserPasswordRequest,
    ) -> Result<()> {
        let url = format!(
            "/users/{}/password",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get user permissions.
     *
     * This function performs a `GET` to the `/users/{userId}/permissions` endpoint.
     *
     * Use this API to get permissions that have been granted to the user. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * Users can be assigned a set of permissions that allows them to access only the pages/information that a user needs to view or edit.
     *
     * **Scopes:** `user:read:admin`, `user:read`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     */
    pub async fn permission(&self, user_id: &str) -> Result<crate::types::UserPermissions> {
        let url = format!(
            "/users/{}/permissions",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Get a user token.
     *
     * This function performs a `GET` to the `/users/{userId}/token` endpoint.
     *
     * Retrieve a user's token. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * This token is used for starting meetings with the Client SDK. If a user signed into Zoom using Google or Facebook, a null value will be returned for the token. To get the token with this API, ask the user to sign into Zoom using their email and password instead.
     *
     * **Scopes:** `user:read:admin`, `user:read`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     * * `type_: crate::types::UserTokenType` -- User token types:<br>`token` - Used for starting meetings with the client SDK. This token expires in 14 days and a new token will be returned after the expiry.<br>`zak` - Used for generating the start meeting URL. The token expiration time is two hours. For API users, the expiration time is 90 days.
     * * `ttl: i64` -- Use this field in conjunction with the `type` field where the value of `type` field is `zak`. The value of this field denotes the expiry time of the `zak` token in seconds. For example, if you would like the zak token to be expired after one hour of the token generation, the value of this field should be `3600`.
     */
    pub async fn token(
        &self,
        user_id: &str,
        type_: crate::types::UserTokenType,
        ttl: i64,
    ) -> Result<crate::types::UserZakResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if ttl > 0 {
            query_args.push(("ttl".to_string(), ttl.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/users/{}/token?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Revoke a user's SSO token.
     *
     * This function performs a `DELETE` to the `/users/{userId}/token` endpoint.
     *
     * Revoke a user's SSO token. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * After calling this API, the SSO user will be logged out of their current Zoom session.
     *
     * **Scopes:** `user:write:admin`, `user:write`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     */
    pub async fn sso_token_delete(&self, user_id: &str) -> Result<()> {
        let url = format!(
            "/users/{}/token",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Check a user email.
     *
     * This function performs a `GET` to the `/users/email` endpoint.
     *
     * Verify if a user's email is registered with Zoom.<br><br>
     *
     * <b>Note: </b>You can successfully check if a user is a registered Zoom user only if the user **signed up for Zoom via email and is within your account.** If you provide an email address of a user who is not in your account, the value of "existed_email" parameter will be "false" irrespective of whether or not the user is registered with Zoom. The response of this API call will not include users who joined Zoom using options such as "Sign in with SSO", "Sign in with Google" or "Sign in with Facebook" even if they are in the same account as yours.
     *
     * **Scopes:** `user:read:admin` `user:read`
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     *
     * **Parameters:**
     *
     * * `email: &str` -- The email address to be verified.
     */
    pub async fn email(&self, email: &str) -> Result<crate::types::UserEmailResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !email.is_empty() {
            query_args.push(("email".to_string(), email.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/users/email?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Update a user's email.
     *
     * This function performs a `PUT` to the `/users/{userId}/email` endpoint.
     *
     * Change a user's [email address](https://support.zoom.us/hc/en-us/articles/201362563-How-Do-I-Change-the-Email-on-My-Account-) on a Zoom account that has managed domain set up. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * * If the Zoom account in which the user belongs has multiple [managed domains](https://support.zoom.us/hc/en-us/articles/203395207-What-is-Managed-Domain-), then the email to be updated **must** match one of the managed domains.
     * * A user's email address can **only** be changed for a maximum of 3 times in a day (24 hours).
     *
     * **Scopes:** `user:write:admin`, `user:write`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Managed domain must be enabled in the account.
     * * The new email address should not already exist in Zoom.
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     */
    pub async fn email_update(&self, user_id: &str, body: &crate::types::Members) -> Result<()> {
        let url = format!(
            "/users/{}/email",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Check a user's PM room.
     *
     * This function performs a `GET` to the `/users/vanity_name` endpoint.
     *
     * A personal meeting room is a virtual meeting room that can be permanently assigned to a user.
     * Use this API to check if a personal meeting room with the given name exists or not.<br><br>
     * **Scopes:** `user:read:admin` `user:read`<br>
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `vanity_name: &str` -- Personal meeting room name.
     */
    pub async fn vanity_name(
        &self,
        vanity_name: &str,
    ) -> Result<crate::types::UserVanityNameResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !vanity_name.is_empty() {
            query_args.push(("vanity_name".to_string(), vanity_name.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/users/vanity_name?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Switch a user's account.
     *
     * This function performs a `PUT` to the `/accounts/{accountId}/users/{userId}/account` endpoint.
     *
     * Disassociate a user from one Account and move the user to another Account under the same master account.
     *
     * With this API, a user under a master account or a sub account can be moved to another sub account within the same master account. To move a user from a master account to a sub account, use `me` as the value for `accountId`. In this scenario, "me" refers to the Account ID of the master account.
     *
     * To move a user from one sub account to another sub account, provide the sub account's Account ID as the value for `accountId`.
     *
     * **Prerequisites**:
     * * The account should have Pro or a higher plan with master account option enabled.
     * * The user whose account needs to be switched should not be an admin or an owner of that account.
     * * The user should not have the same [managed domain](https://support.zoom.us/hc/en-us/articles/203395207-What-is-Managed-Domain-) as the account owner.
     *
     * **Scope:** `user:master`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- User's first name.
     */
    pub async fn switch_account(
        &self,
        account_id: &str,
        user_id: &str,
        body: &crate::types::SwitchUserAccountRequest,
    ) -> Result<()> {
        let url = format!(
            "/accounts/{}/users/{}/account",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Update a user's presence status.
     *
     * This function performs a `PUT` to the `/users/{userId}/presence_status` endpoint.
     *
     * Use this API to update a user's presence status. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * A user's status **cannot** be updated more than once per minute. For example, you can only submit a maximum of one update request per minute for a single user.
     *
     * Users in the Zoom desktop client and mobile apps are assigned with a [presence status](https://support.zoom.us/hc/en-us/articles/360032554051-Status-Icons). The presence status informs users of their contact's availability. Users can also change their own presence status to one the following:
     * * **Away**
     * * **Do not disturb**
     * * **Available**
     * * **In a calendar event**
     * * **Presenting**
     * * **In a Zoom meeting**
     * * **On a call**
     *
     * Note that a user's presence status **cannot** be updated via this API if the user is not logged in to the Zoom client.
     *
     * **Scopes:** `user:write`, `user:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     */
    pub async fn update_presence_status(
        &self,
        user_id: &str,
        body: &crate::types::UpdatePresenceStatusRequestData,
    ) -> Result<()> {
        let url = format!(
            "/users/{}/presence_status",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Upload virtual background files.
     *
     * This function performs a `POST` to the `/users/{userId}/settings/virtual_backgrounds` endpoint.
     *
     * Use this API to [upload virtual background files](https://support.zoom.us/hc/en-us/articles/210707503-Virtual-Background) to a user's profile. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * > **Note:** A user profile cannot exceed more than 10 files.
     *
     * **Scopes:** `user:write:admin`
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites:**
     * * Virtual background feature must be [enabled](https://support.zoom.us/hc/en-us/articles/210707503-Virtual-Background#h_2ef28080-fce9-4ac2-b567-dc958afab1b7) on the account.
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- Unique identifier of the user. Retrieve the value for this field by calling the [List users](https://marketplace.zoom.us/docs/api-reference/zoom-api/users/users) API.
     */
    pub async fn upload_v_buser(
        &self,
        user_id: &str,
        body: &crate::types::UploadVbRequest,
    ) -> Result<crate::types::Files> {
        let url = format!(
            "/users/{}/settings/virtual_backgrounds",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete virtual background files.
     *
     * This function performs a `DELETE` to the `/users/{userId}/settings/virtual_backgrounds` endpoint.
     *
     * Delete existing virtual background file(s) of a user. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `user:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Virtual background feature must be [enabled](https://support.zoom.us/hc/en-us/articles/210707503-Virtual-Background#h_2ef28080-fce9-4ac2-b567-dc958afab1b7) on the account.
     *
     * **Parameters:**
     *
     * * `file_ids: &str` -- Provide the id of the file that is to be deleted. To delete multiple files, provide comma separated values for this field.
     * * `user_id: &str` -- Unique identifier of the user. Retrieve the value of this field by calling the [List users](https://marketplace.zoom.us/docs/api-reference/zoom-api/users/users) API. .
     */
    pub async fn del_vb(&self, user_id: &str, file_ids: &str) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !file_ids.is_empty() {
            query_args.push(("file_ids".to_string(), file_ids.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/users/{}/settings/virtual_backgrounds?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        self.client.delete(&url, None).await
    }
}
