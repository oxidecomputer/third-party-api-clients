use anyhow::Result;

use crate::Client;

pub struct Phone {
    pub client: Client,
}

impl Phone {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Phone { client }
    }

    /**
     * Set up a Zoom Phone account.
     *
     * This function performs a `POST` to the `/accounts/{accountId}/phone/setup` endpoint.
     *
     * After assigning a Zoom phone license to an account, an admin or account owner can proceed with the [initial Zoom phone set up](https://support.zoom.us/hc/en-us/articles/360001297663-Getting-started-with-Zoom-Phone-admin-#h_5ae26a3a-290c-4a8d-b3b0-6384ed267b13) using this API.
     *
     * **Scopes:** `phone:write:admin`, `phone:write`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Paid account
     *  * A Pro or a higher account plan
     * * Master account option enabled
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- Unique identifier of the account.
     */
    pub async fn set_up_account(
        &self,
        account_id: &str,
        body: &crate::types::SetUpAccountRequest,
    ) -> Result<()> {
        let url = format!(
            "/accounts/{}/phone/setup",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * List phone numbers.
     *
     * This function performs a `GET` to the `/phone/numbers` endpoint.
     *
     * Use this API to list all Zoom Phone numbers in a Zoom account.
     *
     * **Scopes:** `phone:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites:**
     * * A Pro or higher account plan
     * * A Zoom Phone license
     *
     * **Parameters:**
     *
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `type_: crate::types::ListAccountPhoneNumbersType` -- Query response by number assignment. The value can be one of the following:
     *  <br>
     *  `assigned`: The number has been assigned to either a user, a call queue, an auto-receptionist or a common area phone in an account. <br>`unassigned`: The number is not assigned to anyone.<br>
     *  `all`: Include both assigned and unassigned numbers in the response.<br>
     *  `byoc`: Include Bring Your Own Carrier (BYOC) numbers only in the response.
     * * `extension_type: crate::types::ExtensionType` -- The type of assignee to whom the number is assigned. The value can be one of the following:<br>
     *  `user`<br> `callQueue`<br> `autoReceptionist`<br>
     *  `commonAreaPhone`.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `number_type: crate::types::Type` -- The type of phone number. The value can be either `toll` or `tollfree`.
     * * `pending_numbers: bool` -- Enable/disable the option for a sub account to use shared [Virtual Room Connector(s)](https://support.zoom.us/hc/en-us/articles/202134758-Getting-Started-With-Virtual-Room-Connector) that are set up by the master account. Virtual Room Connectors can only be used by On-prem users.
     * * `site_id: &str` -- Unique identifier of the site. Use this query parameter if you have enabled multiple sites and would like to filter the response of this API call by a specific phone site. See [Managing multiple sites](https://support.zoom.us/hc/en-us/articles/360020809672-Managing-multiple-sites) or [Adding a site](https://support.zoom.us/hc/en-us/articles/360020809672-Managing-multiple-sites#h_05c88e35-1593-491f-b1a8-b7139a75dc15) for details.
     */
    pub async fn list_account_numbers(
        &self,
        next_page_token: &str,
        type_: crate::types::ListAccountPhoneNumbersType,
        extension_type: crate::types::ExtensionType,
        page_size: i64,
        number_type: crate::types::Type,
        pending_numbers: bool,
        site_id: &str,
    ) -> Result<Vec<crate::types::ListAccountPhoneNumbersResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !extension_type.to_string().is_empty() {
            query_args.push(("extension_type".to_string(), extension_type.to_string()));
        }
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if !number_type.to_string().is_empty() {
            query_args.push(("number_type".to_string(), number_type.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if pending_numbers {
            query_args.push(("pending_numbers".to_string(), pending_numbers.to_string()));
        }
        if !site_id.is_empty() {
            query_args.push(("site_id".to_string(), site_id.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/numbers?{}", query_);

        let resp: crate::types::ListAccountPhoneNumbersResponseData =
            self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.phone_numbers)
    }

    /**
     * List phone numbers.
     *
     * This function performs a `GET` to the `/phone/numbers` endpoint.
     *
     * As opposed to `list_account_numbers`, this function returns all the pages of the request at once.
     *
     * Use this API to list all Zoom Phone numbers in a Zoom account.
     *
     * **Scopes:** `phone:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites:**
     * * A Pro or higher account plan
     * * A Zoom Phone license
     */
    pub async fn list_all_account_numbers(
        &self,
        type_: crate::types::ListAccountPhoneNumbersType,
        extension_type: crate::types::ExtensionType,
        number_type: crate::types::Type,
        pending_numbers: bool,
        site_id: &str,
    ) -> Result<Vec<crate::types::ListAccountPhoneNumbersResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !extension_type.to_string().is_empty() {
            query_args.push(("extension_type".to_string(), extension_type.to_string()));
        }
        if !number_type.to_string().is_empty() {
            query_args.push(("number_type".to_string(), number_type.to_string()));
        }
        if pending_numbers {
            query_args.push(("pending_numbers".to_string(), pending_numbers.to_string()));
        }
        if !site_id.is_empty() {
            query_args.push(("site_id".to_string(), site_id.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/numbers?{}", query_);

        let mut resp: crate::types::ListAccountPhoneNumbersResponseData =
            self.client.get(&url, None).await?;

        let mut phone_numbers = resp.phone_numbers;
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

            phone_numbers.append(&mut resp.phone_numbers);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(phone_numbers)
    }

    /**
     * Get user's profile.
     *
     * This function performs a `GET` to the `/phone/users/{userId}` endpoint.
     *
     * Use this API to return a user's [Zoom phone](https://support.zoom.us/hc/en-us/articles/360001297663-Quickstart-Guide-for-Zoom-Phone-Administrators) profile. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `phone:read`, `phone:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     *  * A Business or Enterprise account
     * * A Zoom Phone license
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     */
    pub async fn user(&self, user_id: &str) -> Result<crate::types::PhoneUserResponse> {
        let url = format!(
            "/phone/users/{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update user's profile.
     *
     * This function performs a `PATCH` to the `/phone/users/{userId}` endpoint.
     *
     * Use this API to update a user's [Zoom Phone](https://support.zoom.us/hc/en-us/categories/360001370051-Zoom-Phone) profile. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `phone:write`, `phone:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     * * A Zoom Phone license
     */
    pub async fn update_user_profile(
        &self,
        user_id: &str,
        body: &crate::types::UpdateUserProfileRequest,
    ) -> Result<()> {
        let url = format!(
            "/phone/users/{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get account's setting.
     *
     * This function performs a `GET` to the `/phone/settings` endpoint.
     *
     * Use this API to return an account's settings.
     *
     * **Scopes:** `phone:read`, `phone:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     * * A Zoom Phone license
     */
    pub async fn setting(&self, account_id: &str) -> Result<crate::types::PhoneSettingResponse> {
        let url = "/phone/settings".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Update BYOC settings.
     *
     * This function performs a `PATCH` to the `/phone/settings` endpoint.
     *
     * [Master account owners](https://marketplace.zoom.us/docs/api-reference/master-account-apis) can use this API to enable the BYOC (Bring Your Own Carrier) option for a subaccount.
     *
     * **Scopes:** `phone:master`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- Unique identifier of the sub account.
     */
    pub async fn update_settings(
        &self,
        account_id: &str,
        body: &crate::types::UpdatePhoneSettingsRequest,
    ) -> Result<()> {
        let url = "/phone/settings".to_string();
        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get user's settings.
     *
     * This function performs a `GET` to the `/phone/users/{userId}/settings` endpoint.
     *
     * Use this API to get a user's [Zoom Phone profile settings](https://support.zoom.us/hc/en-us/articles/360021325712-Configuring-Settings). For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `phone:read`, `phone:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     * * A Zoom Phone license
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     */
    pub async fn user_settings(
        &self,
        user_id: &str,
    ) -> Result<crate::types::PhoneUserSettingsResponse> {
        let url = format!(
            "/phone/users/{}/settings",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * List setting templates.
     *
     * This function performs a `GET` to the `/phone/setting_templates` endpoint.
     *
     * Use this API to get a list of all the created phone template settings.
     *
     * **Scopes:** `phone:read:admin` or `phone:read`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     * * A Zoom Phone license
     *
     * **Parameters:**
     *
     * * `page_size: i64` -- Number of records returns within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `site_id: &str` -- Unique identifier of the site. This field is required only if multiple sites have been enabled.  of the site. Required only when multiple sites are enabled. See [Managing multiple sites](https://support.zoom.us/hc/en-us/articles/360020809672-Managing-multiple-sites) for details. If this is not provided, the response lists the account level setting templates.
     */
    pub async fn list_setting_templates(
        &self,
        page_size: i64,
        next_page_token: &str,
        site_id: &str,
    ) -> Result<Vec<crate::types::Templates>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !site_id.is_empty() {
            query_args.push(("site_id".to_string(), site_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/setting_templates?{}", query_);

        let resp: crate::types::ListSettingTemplatesResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.templates)
    }

    /**
     * List setting templates.
     *
     * This function performs a `GET` to the `/phone/setting_templates` endpoint.
     *
     * As opposed to `list_setting_templates`, this function returns all the pages of the request at once.
     *
     * Use this API to get a list of all the created phone template settings.
     *
     * **Scopes:** `phone:read:admin` or `phone:read`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     * * A Zoom Phone license
     */
    pub async fn list_all_setting_templates(
        &self,
        site_id: &str,
    ) -> Result<Vec<crate::types::Templates>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !site_id.is_empty() {
            query_args.push(("site_id".to_string(), site_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/setting_templates?{}", query_);

        let mut resp: crate::types::ListSettingTemplatesResponse =
            self.client.get(&url, None).await?;

        let mut templates = resp.templates;
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

            templates.append(&mut resp.templates);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(templates)
    }

    /**
     * Add a setting template.
     *
     * This function performs a `POST` to the `/phone/setting_templates` endpoint.
     *
     * Use this API to create a Zoom Phone setting template for an account. After creating a phone template, the defined settings will become the default settings for an account.
     *
     * **Scopes:** `phone:write:admin`, `phone:write`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Business or enterprise Zoom account
     * * A Zoom Phone license
     */
    pub async fn add_setting_template(
        &self,
        body: &crate::types::AddSettingTemplateRequest,
    ) -> Result<crate::types::AddSettingTemplateResponse> {
        let url = "/phone/setting_templates".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Batch add emergency service locations.
     *
     * This function performs a `POST` to the `/phone/batch_locations` endpoint.
     *
     * Use this API to batch add emergency service locations.
     */
    pub async fn batch_add_locations(
        &self,
        body: &crate::types::BatchAddLocationsRequest,
    ) -> Result<Vec<crate::types::BatchAddLocationsResponse>> {
        let url = "/phone/batch_locations".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * List emergency service locations.
     *
     * This function performs a `GET` to the `/phone/locations` endpoint.
     *
     * Use this API to list emergency service locations.
     *
     * **Scopes:** `phone:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions
     *
     * **Parameters:**
     *
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `page_size: i64` -- The number of records returned within a single API call.
     */
    pub async fn list_locations(
        &self,
        next_page_token: &str,
        page_size: i64,
    ) -> Result<Vec<crate::types::ListLocationsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/locations?{}", query_);

        let resp: crate::types::ListLocationsResponseData = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.locations)
    }

    /**
     * List emergency service locations.
     *
     * This function performs a `GET` to the `/phone/locations` endpoint.
     *
     * As opposed to `list_locations`, this function returns all the pages of the request at once.
     *
     * Use this API to list emergency service locations.
     *
     * **Scopes:** `phone:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions
     */
    pub async fn list_all_locations(&self) -> Result<Vec<crate::types::ListLocationsResponse>> {
        let url = "/phone/locations".to_string();
        let mut resp: crate::types::ListLocationsResponseData = self.client.get(&url, None).await?;

        let mut locations = resp.locations;
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

            locations.append(&mut resp.locations);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(locations)
    }

    /**
     * Add emergency service location.
     *
     * This function performs a `POST` to the `/phone/locations` endpoint.
     *
     * Use this API to add an emergency service location.
     *
     * **Scopes:** `phone:write:adminRate`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions
     */
    pub async fn add_location(
        &self,
        body: &crate::types::AddLocationRequest,
    ) -> Result<Vec<crate::types::Site>> {
        let url = "/phone/locations".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get emergency service location details.
     *
     * This function performs a `GET` to the `/phone/locations/{locationId}` endpoint.
     *
     * Use this API to return an emergency service location's information.
     *
     * **Scopes:** `phone:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions
     *
     * **Parameters:**
     *
     * * `location_id: &str` -- The emergency service location's ID.
     */
    pub async fn get_location(
        &self,
        location_id: &str,
    ) -> Result<crate::types::GetLocationResponse> {
        let url = format!(
            "/phone/locations/{}",
            crate::progenitor_support::encode_path(&location_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete an emergency location.
     *
     * This function performs a `DELETE` to the `/phone/locations/{locationId}` endpoint.
     *
     * Use this API to remove an emergency location.
     *
     * **Scopes:** `phone:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions
     *
     * **Parameters:**
     *
     * * `location_id: &str` -- The emergency service location's ID.
     */
    pub async fn delete_location(&self, location_id: &str) -> Result<()> {
        let url = format!(
            "/phone/locations/{}",
            crate::progenitor_support::encode_path(&location_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update emergency service location.
     *
     * This function performs a `PATCH` to the `/phone/locations/{locationId}` endpoint.
     *
     * Use this API to update an emergency location's information.
     *
     * **Scopes:** `phone:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions
     */
    pub async fn update_location(
        &self,
        location_id: &str,
        body: &crate::types::UpdateLocationRequest,
    ) -> Result<()> {
        let url = format!(
            "/phone/locations/{}",
            crate::progenitor_support::encode_path(&location_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * List SIP groups.
     *
     * This function performs a `GET` to the `/phone/sip_groups` endpoint.
     *
     * Use this API to list SIP (Session Initiation Protocol) groups.
     *
     * **Scopes:** `phone:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions
     *
     * **Parameters:**
     *
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `page_size: i64` -- The number of records returned within a single API call.
     */
    pub async fn list_sip_groups(
        &self,
        next_page_token: &str,
        page_size: i64,
    ) -> Result<Vec<crate::types::SipGroups>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/sip_groups?{}", query_);

        let resp: crate::types::ListSipGroupsResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.sip_groups)
    }

    /**
     * List SIP groups.
     *
     * This function performs a `GET` to the `/phone/sip_groups` endpoint.
     *
     * As opposed to `list_sip_groups`, this function returns all the pages of the request at once.
     *
     * Use this API to list SIP (Session Initiation Protocol) groups.
     *
     * **Scopes:** `phone:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions
     */
    pub async fn list_all_sip_groups(&self) -> Result<Vec<crate::types::SipGroups>> {
        let url = "/phone/sip_groups".to_string();
        let mut resp: crate::types::ListSipGroupsResponse = self.client.get(&url, None).await?;

        let mut sip_groups = resp.sip_groups;
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

            sip_groups.append(&mut resp.sip_groups);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(sip_groups)
    }

    /**
     * Get setting template details.
     *
     * This function performs a `GET` to the `/phone/setting_templates/{templateId}` endpoint.
     *
     * Use this API to return information about an account's phone template.
     *
     * **Scopes:** `phone:write:admin` or `phone:write`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     * * A Zoom Phone license
     *
     * **Parameters:**
     *
     * * `template_id: &str` -- Unique identifier of the template.
     * * `custom_query_fields: &str` -- Provide the name of the field to use to filter the response. For example, if you provide "description" as the value of the field, you will get a response similar to the following: {“description”: “template description”}.
     */
    pub async fn get_setting_template(
        &self,
        template_id: &str,
        custom_query_fields: &str,
    ) -> Result<crate::types::GetSettingTemplateResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !custom_query_fields.is_empty() {
            query_args.push((
                "custom_query_fields".to_string(),
                custom_query_fields.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/phone/setting_templates/{}?{}",
            crate::progenitor_support::encode_path(&template_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Update a setting template.
     *
     * This function performs a `PATCH` to the `/phone/setting_templates/{templateId}` endpoint.
     *
     * Use this API to update or modify a phone template's settings.
     *
     * **Scopes:** `phone:write:admin` or `phone:write`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     * * A Zoom Phone license
     *
     * **Parameters:**
     *
     * * `template_id: &str` -- User's first name.
     */
    pub async fn update_setting_template(
        &self,
        template_id: &str,
        body: &crate::types::UpdateSettingTemplateRequest,
    ) -> Result<()> {
        let url = format!(
            "/phone/setting_templates/{}",
            crate::progenitor_support::encode_path(&template_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get user's call logs.
     *
     * This function performs a `GET` to the `/phone/users/{userId}/call_logs` endpoint.
     *
     * Use this API to get a user's [Zoom phone](https://support.zoom.us/hc/en-us/articles/360001297663-Quickstart-Guide-for-Zoom-Phone-Administrators) call logs. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `phone:read`, `phone:read:admin`, `phone_call_log:read`, `phone_call_log:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     * * A Zoom Phone license
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `from: chrono::NaiveDate` -- Start date in 'yyyy-mm-dd' format. The date range defined by the "from" and "to" parameters should only be one month as the report includes only one month worth of data at once.
     * * `to: chrono::NaiveDate` -- Start Date.
     * * `type_: crate::types::PhoneUserCallLogsType`
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `phone_number: &str` -- Filter API responses to include call logs of only the phone number defined in this field.
     * * `time_type: crate::types::TimeType` -- Enables you to sort call logs by start or end time. Choose the sort time value. Values include `startTime` or `endTime`.
     */
    pub async fn user_call_logs(
        &self,
        user_id: &str,
        page_size: i64,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
        type_: crate::types::PhoneUserCallLogsType,
        next_page_token: &str,
        phone_number: &str,
        time_type: crate::types::TimeType,
    ) -> Result<Vec<crate::types::CallLogs>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !phone_number.is_empty() {
            query_args.push(("phone_number".to_string(), phone_number.to_string()));
        }
        if !time_type.to_string().is_empty() {
            query_args.push(("time_type".to_string(), time_type.to_string()));
        }
        if !to.to_string().is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/phone/users/{}/call_logs?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        let resp: crate::types::PhoneUserCallLogsResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.call_logs)
    }

    /**
     * Get user's call logs.
     *
     * This function performs a `GET` to the `/phone/users/{userId}/call_logs` endpoint.
     *
     * As opposed to `user_call_logs`, this function returns all the pages of the request at once.
     *
     * Use this API to get a user's [Zoom phone](https://support.zoom.us/hc/en-us/articles/360001297663-Quickstart-Guide-for-Zoom-Phone-Administrators) call logs. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `phone:read`, `phone:read:admin`, `phone_call_log:read`, `phone_call_log:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     * * A Zoom Phone license
     */
    pub async fn get_all_user_call_logs(
        &self,
        user_id: &str,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
        type_: crate::types::PhoneUserCallLogsType,
        phone_number: &str,
        time_type: crate::types::TimeType,
    ) -> Result<Vec<crate::types::CallLogs>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !phone_number.is_empty() {
            query_args.push(("phone_number".to_string(), phone_number.to_string()));
        }
        if !time_type.to_string().is_empty() {
            query_args.push(("time_type".to_string(), time_type.to_string()));
        }
        if !to.to_string().is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/phone/users/{}/call_logs?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        let mut resp: crate::types::PhoneUserCallLogsResponse = self.client.get(&url, None).await?;

        let mut call_logs = resp.call_logs;
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

            call_logs.append(&mut resp.call_logs);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(call_logs)
    }

    /**
     * Get user's recordings.
     *
     * This function performs a `GET` to the `/phone/users/{userId}/recordings` endpoint.
     *
     * Use this API to get a user's [Zoom Phone recordings](https://support.zoom.us/hc/en-us/articles/360021336671-Viewing-Call-History-and-Recordings). For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `phone:read`, `phone:read:admin`, `phone_recording:read`, `phone_recording:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     * * A Zoom Phone license
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `from: chrono::NaiveDate` -- Start date for the query in 'yyyy-mm-dd' format. The date range defined by the "from" and "to" parameters should only be one month as the response includes only one month worth of recording data. The month defined should fall within the last six months.
     * * `to: chrono::NaiveDate` -- Start Date.
     */
    pub async fn user_recordings(
        &self,
        user_id: &str,
        page_size: i64,
        next_page_token: &str,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    ) -> Result<Vec<crate::types::Recordings>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !to.to_string().is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/phone/users/{}/recordings?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        let resp: crate::types::PhoneUserRecordingsResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.recordings)
    }

    /**
     * Get user's recordings.
     *
     * This function performs a `GET` to the `/phone/users/{userId}/recordings` endpoint.
     *
     * As opposed to `user_recordings`, this function returns all the pages of the request at once.
     *
     * Use this API to get a user's [Zoom Phone recordings](https://support.zoom.us/hc/en-us/articles/360021336671-Viewing-Call-History-and-Recordings). For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `phone:read`, `phone:read:admin`, `phone_recording:read`, `phone_recording:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     * * A Zoom Phone license
     */
    pub async fn get_all_user_recordings(
        &self,
        user_id: &str,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    ) -> Result<Vec<crate::types::Recordings>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !to.to_string().is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/phone/users/{}/recordings?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        let mut resp: crate::types::PhoneUserRecordingsResponse =
            self.client.get(&url, None).await?;

        let mut recordings = resp.recordings;
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

            recordings.append(&mut resp.recordings);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(recordings)
    }

    /**
     * Get user's voicemails.
     *
     * This function performs a `GET` to the `/phone/users/{userId}/voice_mails` endpoint.
     *
     * Use this API to get a user's Zoom Phone voicemails. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `phone:read`, `phone:read:admin`, `phone_voicemail:read`, `phone_voicemail:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     * * A Zoom Phone license
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user. For user-level apps, pass `me` as the value for userId.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `status: crate::types::PhoneUserVoiceMailsStatus` -- Status of the voice mail.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `from: chrono::NaiveDate` -- Start date for the query in 'yyyy-mm-dd' format. The date range defined by the "from" and "to" parameters should only be one month as the response includes only one month worth of voicemail data. The month defined should fall within the last six months.
     * * `to: chrono::NaiveDate` -- Start Date.
     */
    pub async fn user_voice_mails(
        &self,
        user_id: &str,
        page_size: i64,
        status: crate::types::PhoneUserVoiceMailsStatus,
        next_page_token: &str,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    ) -> Result<Vec<crate::types::VoiceMails>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !to.to_string().is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/phone/users/{}/voice_mails?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        let resp: crate::types::PhoneUserVoiceMailsResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.voice_mails)
    }

    /**
     * Get user's voicemails.
     *
     * This function performs a `GET` to the `/phone/users/{userId}/voice_mails` endpoint.
     *
     * As opposed to `user_voice_mails`, this function returns all the pages of the request at once.
     *
     * Use this API to get a user's Zoom Phone voicemails. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `phone:read`, `phone:read:admin`, `phone_voicemail:read`, `phone_voicemail:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     * * A Zoom Phone license
     */
    pub async fn get_all_user_voice_mails(
        &self,
        user_id: &str,
        status: crate::types::PhoneUserVoiceMailsStatus,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
    ) -> Result<Vec<crate::types::VoiceMails>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !to.to_string().is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/phone/users/{}/voice_mails?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        let mut resp: crate::types::PhoneUserVoiceMailsResponse =
            self.client.get(&url, None).await?;

        let mut voice_mails = resp.voice_mails;
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

            voice_mails.append(&mut resp.voice_mails);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(voice_mails)
    }

    /**
     * Set up shared access.
     *
     * This function performs a `POST` to the `/phone/users/{userId}/settings/{settingType}` endpoint.
     *
     * Use this API to define the voicemail access permissions of a user. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * Phone users can access [shared voicemail inboxes](https://support.zoom.us/hc/en-us/articles/360033863991-Sharing-and-controlling-access-to-a-voicemail-inbox) in the Zoom desktop client, web portal, or provisioned desk phone.
     *
     * To view these settings in the Zoom web portal, navigate to the **Admin >> Phone System Management >> Users & Rooms** interface. Click the **Users** tab and select **User Settings**. Scroll down to **Voicemail & Call Recordings**.
     *
     * **Scopes:** `phone:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- Unique identifier of the user.
     * * `setting_type: &str` -- Corresponds to the setting item you wish to modify. Allowed values: `voice_mail`.
     */
    pub async fn add_user_setting(
        &self,
        user_id: &str,
        setting_type: &str,
        body: &crate::types::AddUserSettingRequest,
    ) -> Result<crate::types::AddUserSettingResponse> {
        let url = format!(
            "/phone/users/{}/settings/{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&setting_type.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Remove shared access.
     *
     * This function performs a `DELETE` to the `/phone/users/{userId}/settings/{settingType}` endpoint.
     *
     * Use this API to remove a user's shared voicemail access settings. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * To view these settings in your Zoom web portal, navigate to the **Admin >> Phone System Management >> Users & Rooms** interface. Click the **Users** tab and select **User Settings**. Scroll down to **Voicemail & Call Recordings**.
     *
     * **Scopes:** `phone:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- Unique identifier of the user.
     * * `setting_type: &str` -- Corresponds to the setting item you wish to remove. Allowed values: `voice_mail`.
     * * `shared_id: &str` -- Required only for voicemail setting type.
     */
    pub async fn delete_user_setting(
        &self,
        user_id: &str,
        setting_type: &str,
        shared_id: &str,
    ) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !shared_id.is_empty() {
            query_args.push(("shared_id".to_string(), shared_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/phone/users/{}/settings/{}?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&setting_type.to_string()),
            query_
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update shared access.
     *
     * This function performs a `PATCH` to the `/phone/users/{userId}/settings/{settingType}` endpoint.
     *
     * Use this API to update the voicemail access permissions of a user. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * Phone users can access [shared voicemail inboxes](https://support.zoom.us/hc/en-us/articles/360033863991-Sharing-and-controlling-access-to-a-voicemail-inbox) in the Zoom desktop client, web portal, or provisioned desk phone.
     *
     * To view these settings in the Zoom web portal, navigate to the **Admin >> Phone System Management >> Users & Rooms** interface. Click the **Users** tab and select **User Settings**. Scroll down to **Voicemail & Call Recordings**.
     *
     * **Scopes:** `phone:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     *
     * **Parameters:**
     *
     * * `setting_type: &str` -- Corresponds to the setting item you wish to modify. Allowed values: `voice_mail`.
     * * `user_id: &str` -- Unique identifier of the user.
     */
    pub async fn update_user_setting(
        &self,
        setting_type: &str,
        user_id: &str,
        body: &crate::types::UpdateUserSettingRequest,
    ) -> Result<()> {
        let url = format!(
            "/phone/users/{}/settings/{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&setting_type.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get account's call logs.
     *
     * This function performs a `GET` to the `/phone/call_logs` endpoint.
     *
     * Use this API to return an account's [call logs](https://support.zoom.us/hc/en-us/articles/360021114452-Viewing-Call-Logs).
     *
     * **Scopes:** `phone:read:admin`, `phone_call_log:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     * * A Zoom Phone license
     * * Account owner and a [role](https://support.zoom.us/hc/en-us/articles/115001078646-Role-Based-Access-Control) with Zoom Phone management
     *
     * **Parameters:**
     *
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `from: &str` -- Start date from which you would like to get the call logs. The start date should be within past six months. <br>
     *   
     *   The API only returns data pertaining to a month. Thus, the date range(defined using "from" and "to" fields) for which the call logs are to be returned must not exceed a month.
     * * `to: &str` -- The end date upto which you would like to get the call logs for. The end date should be within past six months.
     * * `type_: &str` -- The type of the call logs. The value can be either "all" or "missed".
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `path: &str` -- Filter the API response by [path](https://support.zoom.us/hc/en-us/articles/360021114452-Viewing-and-identifying-logs#h_646b46c6-0623-4ab1-8b8b-ea5b8bcef679) of the call. The value of this field can be one of the following: `voiceMail`, `message`, `forward`, `extension`, `callQueue`, `ivrMenu`, `companyDirectory`, `autoReceptionist`, `contactCenter`, `disconnected`, `commonAreaPhone`,
     *   `pstn`, `transfer`, `sharedLines`, `sharedLineGroup`, `tollFreeBilling`, `meetingService`, `parkPickup`,
     *   `parkTimeout`, `monitor`, `takeover`, `sipGroup`.
     * * `time_type: crate::types::TimeType` -- Enables you to sort call logs by start or end time. Choose the sort time value. Values include `startTime` or `endTime`.
     * * `site_id: &str` -- Unique identifier of the [site](https://support.zoom.us/hc/en-us/articles/360020809672-Managing-multiple-sites). Use this query parameter if you have enabled multiple sites and would like to filter the response of this API call by call logs of a specific phone site.
     */
    pub async fn account_call_logs(
        &self,
        page_size: i64,
        from: &str,
        to: &str,
        type_: &str,
        next_page_token: &str,
        path: &str,
        time_type: crate::types::TimeType,
        site_id: &str,
    ) -> Result<Vec<crate::types::AccountCallLogsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !path.is_empty() {
            query_args.push(("path".to_string(), path.to_string()));
        }
        if !site_id.is_empty() {
            query_args.push(("site_id".to_string(), site_id.to_string()));
        }
        if !time_type.to_string().is_empty() {
            query_args.push(("time_type".to_string(), time_type.to_string()));
        }
        if !to.is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/call_logs?{}", query_);

        let resp: crate::types::AccountCallLogsResponseData = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.call_logs)
    }

    /**
     * Get account's call logs.
     *
     * This function performs a `GET` to the `/phone/call_logs` endpoint.
     *
     * As opposed to `account_call_logs`, this function returns all the pages of the request at once.
     *
     * Use this API to return an account's [call logs](https://support.zoom.us/hc/en-us/articles/360021114452-Viewing-Call-Logs).
     *
     * **Scopes:** `phone:read:admin`, `phone_call_log:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     * * A Zoom Phone license
     * * Account owner and a [role](https://support.zoom.us/hc/en-us/articles/115001078646-Role-Based-Access-Control) with Zoom Phone management
     */
    pub async fn get_all_account_call_logs(
        &self,
        from: &str,
        to: &str,
        type_: &str,
        path: &str,
        time_type: crate::types::TimeType,
        site_id: &str,
    ) -> Result<Vec<crate::types::AccountCallLogsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !path.is_empty() {
            query_args.push(("path".to_string(), path.to_string()));
        }
        if !site_id.is_empty() {
            query_args.push(("site_id".to_string(), site_id.to_string()));
        }
        if !time_type.to_string().is_empty() {
            query_args.push(("time_type".to_string(), time_type.to_string()));
        }
        if !to.is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/call_logs?{}", query_);

        let mut resp: crate::types::AccountCallLogsResponseData =
            self.client.get(&url, None).await?;

        let mut call_logs = resp.call_logs;
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

            call_logs.append(&mut resp.call_logs);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(call_logs)
    }

    /**
     * Assign phone number to user.
     *
     * This function performs a `POST` to the `/phone/users/{userId}/phone_numbers` endpoint.
     *
     * Use this API to assign a [phone number](https://support.zoom.us/hc/en-us/articles/360020808292-Managing-Phone-Numbers) to a user who has already enabled Zoom Phone. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `phone:write`, `phone:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     * * A Zoom Phone license
     */
    pub async fn assign_number(
        &self,
        user_id: &str,
        body: &crate::types::AddByocNumberResponse,
    ) -> Result<crate::types::AddByocNumberResponse> {
        let url = format!(
            "/phone/users/{}/phone_numbers",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Unassign phone number.
     *
     * This function performs a `DELETE` to the `/phone/users/{userId}/phone_numbers/{phoneNumberId}` endpoint.
     *
     * Use this API to unassign Zoom Phone user's [phone number](https://support.zoom.us/hc/en-us/articles/360020808292-Managing-Phone-Numbers#h_38ba8b01-26e3-4b1b-a9b5-0717c00a7ca6). For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * After assigning a phone number, you can remove it if you do not want it to be assigned to anyone.
     *
     * **Scopes:** `phone:write`, `phone:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     * * A Zoom Phone license
     * * The user must have been previously assigned a Zoom Phone number
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- Provide either userId or email address of the user.
     * * `phone_number_id: &str` -- Provide either phone number or phoneNumberId of the user. .
     */
    pub async fn unassign_number(&self, user_id: &str, phone_number_id: &str) -> Result<()> {
        let url = format!(
            "/phone/users/{}/phone_numbers/{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&phone_number_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Assign calling plan to a user.
     *
     * This function performs a `POST` to the `/phone/users/{userId}/calling_plans` endpoint.
     *
     * Use this API to assign a [calling plan](https://marketplace.zoom.us/docs/api-reference/other-references/plans#zoom-phone-calling-plans) to a [Zoom Phone](https://support.zoom.us/hc/en-us/categories/360001370051-Zoom-Phone) user. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `phone:write`, `phone:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     * * A Zoom Phone license
     */
    pub async fn assign_calling_plan(
        &self,
        user_id: &str,
        body: &crate::types::AssignCallingPlanRequest,
    ) -> Result<()> {
        let url = format!(
            "/phone/users/{}/calling_plans",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Unassign user's calling plan.
     *
     * This function performs a `DELETE` to the `/phone/users/{userId}/calling_plans/{type}` endpoint.
     *
     * Use this API to unassign a a [Zoom Phone](https://support.zoom.us/hc/en-us/categories/360001370051) user's [calling plan](https://marketplace.zoom.us/docs/api-reference/other-references/plans#zoom-phone-calling-plans). For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `phone:write`, `phone:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     * * A Zoom Phone license
     *
     * **Parameters:**
     *
     * * `type_: &str` -- The [type](https://marketplace.zoom.us/docs/api-reference/other-references/plans#zoom-phone-calling-plans) of the calling plan that was assigned to user. (e.g: The value of type would be "200" for Unlimited US/Canada calling plan.)
     *   .
     */
    pub async fn unassign_calling_plan(&self, user_id: &str, type_: &str) -> Result<()> {
        let url = format!(
            "/phone/users/{}/calling_plans/{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&type_.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Get call recordings.
     *
     * This function performs a `GET` to the `/phone/recordings` endpoint.
     *
     * Use this API to list an account's [call recordings](https://support.zoom.us/hc/en-us/articles/360038521091-Accessing-and-sharing-call-recordings)
     *
     * **Scopes:** `phone:read:admin`, `phone:write:admin`,`phone_recording:read:admin`
     *
     * **Prerequisties:**
     * * A Pro or higher account plan
     * * A Zoom Phone license
     * * Account owner or admin privileges
     *
     * **Parameters:**
     *
     * * `page_size: i64` -- The number of records returned within a single API call. The default is **30**, and the maximum is **100**.
     * * `next_page_token: &str` -- The current page number of returned records.
     * * `from: &str` -- Start date and time in **yyyy-mm-dd** format or **yyyy-MM-dd’T’HH:mm:ss’Z’** format. The date range defined by the from and to parameters should only be one month as the report includes only one month worth of data at once.
     *   .
     * * `to: &str` -- End date and time in **yyyy-mm-dd** format or **yyyy-MM-dd’T’HH:mm:ss’Z’** format, the same formats supported by the `from` parameter.
     *   
     *   .
     * * `owner_type: &str` -- The owner type. The allowed values are null, `user`, or `callQueue`. The default is null. If null, returns all owner types.
     *   .
     * * `recording_type: &str` -- The recording type. The allowed values are null, `OnDemand`, or `Automatic`. The default is null. If null, returns all recording types.
     *   .
     * * `site_id: &str` -- The site ID. The default is `All sites`.
     * * `query_date_type: crate::types::QueryDateType` -- Date types:<br>`start_time` - Query by call start time.<br>`end_time` - Query by call end time.
     */
    pub async fn get_recordings(
        &self,
        page_size: i64,
        next_page_token: &str,
        from: &str,
        to: &str,
        owner_type: &str,
        recording_type: &str,
        site_id: &str,
        query_date_type: crate::types::QueryDateType,
    ) -> Result<Vec<crate::types::GetPhoneRecordingsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if !owner_type.is_empty() {
            query_args.push(("owner_type".to_string(), owner_type.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !query_date_type.to_string().is_empty() {
            query_args.push(("query_date_type".to_string(), query_date_type.to_string()));
        }
        if !recording_type.is_empty() {
            query_args.push(("recording_type".to_string(), recording_type.to_string()));
        }
        if !site_id.is_empty() {
            query_args.push(("site_id".to_string(), site_id.to_string()));
        }
        if !to.is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/recordings?{}", query_);

        let resp: crate::types::GetPhoneRecordingsResponseData =
            self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.recordings)
    }

    /**
     * Get call recordings.
     *
     * This function performs a `GET` to the `/phone/recordings` endpoint.
     *
     * As opposed to `get_recordings`, this function returns all the pages of the request at once.
     *
     * Use this API to list an account's [call recordings](https://support.zoom.us/hc/en-us/articles/360038521091-Accessing-and-sharing-call-recordings)
     *
     * **Scopes:** `phone:read:admin`, `phone:write:admin`,`phone_recording:read:admin`
     *
     * **Prerequisties:**
     * * A Pro or higher account plan
     * * A Zoom Phone license
     * * Account owner or admin privileges
     */
    pub async fn get_all_recordings(
        &self,
        from: &str,
        to: &str,
        owner_type: &str,
        recording_type: &str,
        site_id: &str,
        query_date_type: crate::types::QueryDateType,
    ) -> Result<Vec<crate::types::GetPhoneRecordingsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !owner_type.is_empty() {
            query_args.push(("owner_type".to_string(), owner_type.to_string()));
        }
        if !query_date_type.to_string().is_empty() {
            query_args.push(("query_date_type".to_string(), query_date_type.to_string()));
        }
        if !recording_type.is_empty() {
            query_args.push(("recording_type".to_string(), recording_type.to_string()));
        }
        if !site_id.is_empty() {
            query_args.push(("site_id".to_string(), site_id.to_string()));
        }
        if !to.is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/recordings?{}", query_);

        let mut resp: crate::types::GetPhoneRecordingsResponseData =
            self.client.get(&url, None).await?;

        let mut recordings = resp.recordings;
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

            recordings.append(&mut resp.recordings);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(recordings)
    }

    /**
     * List BYOC SIP trunks.
     *
     * This function performs a `GET` to the `/phone/sip_trunk/trunks` endpoint.
     *
     * Use this API to return a list of an account's assigned [BYOC (Bring Your Own Carrier) SIP (Session Initiation Protocol) trunks](https://zoom.us/docs/doc/Zoom-Bring%20Your%20Own%20Carrier.pdf).
     *
     * **Scopes:** `phone:write:admin` or `phone:master`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     *
     * **Parameters:**
     *
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `page_size: i64` -- The number of records returned within a single API call.
     */
    pub async fn list_byocsip_trunk(
        &self,
        next_page_token: &str,
        page_size: i64,
    ) -> Result<Vec<crate::types::ByocSipTrunk>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/sip_trunk/trunks?{}", query_);

        let resp: crate::types::ListByocsipTrunkResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.byoc_sip_trunk)
    }

    /**
     * List BYOC SIP trunks.
     *
     * This function performs a `GET` to the `/phone/sip_trunk/trunks` endpoint.
     *
     * As opposed to `list_byocsip_trunk`, this function returns all the pages of the request at once.
     *
     * Use this API to return a list of an account's assigned [BYOC (Bring Your Own Carrier) SIP (Session Initiation Protocol) trunks](https://zoom.us/docs/doc/Zoom-Bring%20Your%20Own%20Carrier.pdf).
     *
     * **Scopes:** `phone:write:admin` or `phone:master`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     */
    pub async fn list_all_byocsip_trunk(&self) -> Result<Vec<crate::types::ByocSipTrunk>> {
        let url = "/phone/sip_trunk/trunks".to_string();
        let mut resp: crate::types::ListByocsipTrunkResponse = self.client.get(&url, None).await?;

        let mut byoc_sip_trunk = resp.byoc_sip_trunk;
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

            byoc_sip_trunk.append(&mut resp.byoc_sip_trunk);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(byoc_sip_trunk)
    }

    /**
     * Assign SIP trunks.
     *
     * This function performs a `POST` to the `/accounts/{accountId}/phone/sip_trunk/trunks` endpoint.
     *
     * A [Master account](https://marketplace.zoom.us/docs/api-reference/master-account-apis) owner can use this API to assign SIP (Session Initiation Protocol) trunks to a subaccount.
     *
     * **Scopes:** `phone:master`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- Unique identifier of the account.
     */
    pub async fn post_sip_trunk(
        &self,
        account_id: &str,
        body: &crate::types::PostPhoneSipTrunkRequest,
    ) -> Result<crate::types::PostPhoneSipTrunkRequest> {
        let url = format!(
            "/accounts/{}/phone/sip_trunk/trunks",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Update SIP trunk details.
     *
     * This function performs a `PATCH` to the `/accounts/{accountId}/phone/sip_trunk/trunks/{sipTrunkId}` endpoint.
     *
     * Use this API to update a subaccount's assigned SIP (Session Initiation Protocol) trunk information.
     *
     * **Scopes:** `phone:master` <br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     *
     * **Parameters:**
     *
     * * `sip_trunk_id: &str` -- Unique identifier of the SIP trunk.
     * * `account_id: &str` -- Unique identifier of the sub account.
     */
    pub async fn update_sip_trunk(
        &self,
        sip_trunk_id: &str,
        account_id: &str,
        body: &crate::types::UpdatePhoneSipTrunkRequest,
    ) -> Result<()> {
        let url = format!(
            "/accounts/{}/phone/sip_trunk/trunks/{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&sip_trunk_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * List external contacts.
     *
     * This function performs a `GET` to the `/phone/external_contacts` endpoint.
     *
     * Use this API to list external contacts.
     *
     * **Scopes:** `phone:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions
     *
     * **Parameters:**
     *
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `page_size: i64` -- The number of records returned within a single API call.
     */
    pub async fn list_external_contacts(
        &self,
        next_page_token: &str,
        page_size: i64,
    ) -> Result<Vec<crate::types::ExternalContacts>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/external_contacts?{}", query_);

        let resp: crate::types::ListExternalContactsResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.external_contacts)
    }

    /**
     * List external contacts.
     *
     * This function performs a `GET` to the `/phone/external_contacts` endpoint.
     *
     * As opposed to `list_external_contacts`, this function returns all the pages of the request at once.
     *
     * Use this API to list external contacts.
     *
     * **Scopes:** `phone:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions
     */
    pub async fn list_all_external_contacts(&self) -> Result<Vec<crate::types::ExternalContacts>> {
        let url = "/phone/external_contacts".to_string();
        let mut resp: crate::types::ListExternalContactsResponse =
            self.client.get(&url, None).await?;

        let mut external_contacts = resp.external_contacts;
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

            external_contacts.append(&mut resp.external_contacts);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(external_contacts)
    }

    /**
     * Add an external contact.
     *
     * This function performs a `POST` to the `/phone/external_contacts` endpoint.
     *
     * Use this API to add an external contact.
     *
     * **Scopes:** `phone:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions
     */
    pub async fn add_external_contact(
        &self,
        body: &crate::types::AddExternalContactRequest,
    ) -> Result<()> {
        let url = "/phone/external_contacts".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get external contact details.
     *
     * This function performs a `GET` to the `/phone/external_contacts/{externalContactId}` endpoint.
     *
     * Use this API to get an external contact's information.
     *
     * **Scopes:** `phone:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions<br>
     *
     * **Parameters:**
     *
     * * `external_contact_id: &str` -- The external contact's ID.
     */
    pub async fn get_external_contact(
        &self,
        external_contact_id: &str,
    ) -> Result<crate::types::ExternalContacts> {
        let url = format!(
            "/phone/external_contacts/{}",
            crate::progenitor_support::encode_path(&external_contact_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete an external contact.
     *
     * This function performs a `DELETE` to the `/phone/external_contacts/{externalContactId}` endpoint.
     *
     * Use this API to remove an external contact.
     *
     * **Scopes:** `phone:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions
     *
     * **Parameters:**
     *
     * * `external_contact_id: &str` -- The external contact's ID.
     */
    pub async fn delete_external_contact(&self, external_contact_id: &str) -> Result<()> {
        let url = format!(
            "/phone/external_contacts/{}",
            crate::progenitor_support::encode_path(&external_contact_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update external contact.
     *
     * This function performs a `PATCH` to the `/phone/external_contacts/{externalContactId}` endpoint.
     *
     * Use this API to update an external contact's information.
     *
     * **Scopes:** `phone:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * Pro or a higher account with Zoom Phone license
     * * Account owner or admin permissions
     *
     * **Parameters:**
     *
     * * `external_contact_id: &str` -- User's first name.
     */
    pub async fn update_external_contact(
        &self,
        external_contact_id: &str,
        body: &crate::types::UpdateExternalContactRequest,
    ) -> Result<()> {
        let url = format!(
            "/phone/external_contacts/{}",
            crate::progenitor_support::encode_path(&external_contact_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get phone number details.
     *
     * This function performs a `GET` to the `/phone/numbers/{numberId}` endpoint.
     *
     * Use this API to get information about an account's Zoom Phone number.
     *
     * **Scopes:** `phone:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Pro or higher account plan
     * * A Zoom phone license
     *
     * **Parameters:**
     *
     * * `number_id: &str` -- Unique Identifier of the Phone Number. This can be retrieved from the List Phone Numbers API.
     */
    pub async fn get_number_details(
        &self,
        number_id: &str,
    ) -> Result<crate::types::GetPhoneNumberDetailsResponse> {
        let url = format!(
            "/phone/numbers/{}",
            crate::progenitor_support::encode_path(&number_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Update phone number details.
     *
     * This function performs a `PATCH` to the `/phone/numbers/{numberId}` endpoint.
     *
     * Use this API to update a Zoom Phone number's information.
     *
     * **Scopes:** `phone:write`, `phone:write:admin`, `phone:master`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Paid account
     *
     * **Parameters:**
     *
     * * `number_id: &str` -- User's first name.
     */
    pub async fn update_number_details(
        &self,
        number_id: &str,
        body: &crate::types::UpdatePhoneNumberDetailsRequest,
    ) -> Result<()> {
        let url = format!(
            "/phone/numbers/{}",
            crate::progenitor_support::encode_path(&number_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Change main company number.
     *
     * This function performs a `PUT` to the `/phone/company_number` endpoint.
     *
     * Use this API to [change an account's main company number](https://support.zoom.us/hc/en-us/articles/360028553691#h_82414c34-9df2-428a-85a4-efcf7f9e0d72).
     *
     * External users can use the [main company number](https://support.zoom.us/hc/en-us/articles/360028553691) to reach your Zoom Phone users by dialing the main company number and the user's extension. It can also be used by your account's Zoom Phone users as their caller ID when making calls.
     *
     * **Scopes:** `phone:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Pro or higher account plan
     * * Account owner or admin permissions
     */
    pub async fn change_main_company_number(
        &self,
        body: &crate::types::ChangeMainCompanyNumberRequest,
    ) -> Result<()> {
        let url = "/phone/company_number".to_string();
        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * List calling plans.
     *
     * This function performs a `GET` to the `/phone/calling_plans` endpoint.
     *
     * Use this API to return all of an account's Zoom Phone [calling plans](https://marketplace.zoom.us/docs/api-reference/other-references/plans#zoom-phone-calling-plans).
     *
     * **Scopes:** `phone:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Prerequisites:**
     * * A Pro or a higher account
     * * A Zoom Phone license
     */
    pub async fn list_calling_plan(&self) -> Result<crate::types::ListCallingPlansResponseData> {
        let url = "/phone/calling_plans".to_string();
        self.client.get(&url, None).await
    }

    /**
     * List phone users.
     *
     * This function performs a `GET` to the `/phone/users` endpoint.
     *
     * Use this API to return a list of all of an account's users who are assigned a Zoom Phone license.
     *
     * **Scopes:** `phone:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Pro or higher account plan
     * * A Zoom Phone license
     *
     * **Parameters:**
     *
     * * `page_size: i64` -- The number of records returned from a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `site_id: &str` -- Unique Identifier of the site. This can be retrieved from the [List Phone Sites](https://marketplace.zoom.us/docs/api-reference/zoom-api/phone-site/listphonesites) API.
     */
    pub async fn list_users(
        &self,
        page_size: i64,
        next_page_token: &str,
        site_id: &str,
    ) -> Result<Vec<crate::types::ListPhoneUsersResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !site_id.is_empty() {
            query_args.push(("site_id".to_string(), site_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/users?{}", query_);

        let resp: crate::types::ListPhoneUsersResponseData = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.users)
    }

    /**
     * List phone users.
     *
     * This function performs a `GET` to the `/phone/users` endpoint.
     *
     * As opposed to `list_users`, this function returns all the pages of the request at once.
     *
     * Use this API to return a list of all of an account's users who are assigned a Zoom Phone license.
     *
     * **Scopes:** `phone:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Pro or higher account plan
     * * A Zoom Phone license
     */
    pub async fn list_all_users(
        &self,
        site_id: &str,
    ) -> Result<Vec<crate::types::ListPhoneUsersResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !site_id.is_empty() {
            query_args.push(("site_id".to_string(), site_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/phone/users?{}", query_);

        let mut resp: crate::types::ListPhoneUsersResponseData =
            self.client.get(&url, None).await?;

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
     * Get call log details.
     *
     * This function performs a `GET` to the `/phone/call_logs/{callLogId}` endpoint.
     *
     * Use this API to return information about a [call log](https://support.zoom.us/hc/en-us/articles/360021114452-Viewing-and-identifying-logs).
     *
     * **Scopes:** `phone:read`, `phone:read:admin`, `phone_call_log:read`, `phone_call_log:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     *
     * **Prerequisites:**
     * * A Business or Enterprise account
     * * A Zoom Phone license
     *
     * **Parameters:**
     *
     * * `call_log_id: &str` -- Unique identifier of the call log. Both `callLogId` and `callId` can be used as path parameters. The value for this field can be retrieved from [account's call logs](https://marketplace.zoom.us/docs/api-reference/zoom-api/phone/accountcalllogs) or the [user's call logs](https://marketplace.zoom.us/docs/api-reference/zoom-api/phone/phoneusercalllogs).
     */
    pub async fn get_call_log_details(
        &self,
        call_log_id: &str,
    ) -> Result<crate::types::GetCallLogDetailsResponse> {
        let url = format!(
            "/phone/call_logs/{}",
            crate::progenitor_support::encode_path(&call_log_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a user's call log.
     *
     * This function performs a `DELETE` to the `/phone/users/{userId}/call_logs/{callLogId}` endpoint.
     *
     * Use this API to delete a user's [call log](https://support.zoom.us/hc/en-us/articles/360021114452-Viewing-and-identifying-logs). For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `phone:write`, `phone:write:admin`, `phone_call_log:write`, `phone_call_log:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * User must belong to a Business or Enterprise account
     * * User must have a Zoom Phone license
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address of the user.
     * * `call_log_id: &str` -- Unique identifier of the call log. The value for this field can be retrieved from [account's call logs](https://marketplace.zoom.us/docs/api-reference/zoom-api/phone/accountcalllogs) or [user's call logs](https://marketplace.zoom.us/docs/api-reference/zoom-api/phone/phoneusercalllogs).
     */
    pub async fn delete_call_log(&self, user_id: &str, call_log_id: &str) -> Result<()> {
        let url = format!(
            "/phone/users/{}/call_logs/{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&call_log_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Add BYOC phone numbers.
     *
     * This function performs a `POST` to the `/phone/byoc_numbers` endpoint.
     *
     * Use this API to add BYOC (Bring Your Own Carrier) phone numbers to Zoom Phone.
     *
     * **Scopes:** `phone:write:admin`, `phone:write`, or `phone:master`</br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Business or Enterprise plan
     * * A Zoom Phone license
     */
    pub async fn add_byoc_number(
        &self,
        body: &crate::types::AddByocNumberRequest,
    ) -> Result<crate::types::AddByocNumberResponse> {
        let url = "/phone/byoc_numbers".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete a voicemail.
     *
     * This function performs a `DELETE` to the `/phone/voice_mails/{voicemailId}` endpoint.
     *
     * Use this API to delete an account's [voicemail message](https://support.zoom.us/hc/en-us/articles/360021400211-Managing-voicemail-messages).
     *
     * **Scopes:** `phone:write:admin`, `phone:write`, `phone_voicemail:write`, `phone_voicemail:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Light`
     *
     * **Prerequisites:**
     * * A Zoom Phone license
     *
     * **Parameters:**
     *
     * * `voicemail_id: &str` -- Unique identifier of the voicemail. Retrieve the value for this field by calling the [Get voicemails](https://marketplace.zoom.us/docs/api-reference/zoom-api/phone/phoneuservoicemails) API.
     */
    pub async fn delete_voicemail(&self, voicemail_id: &str) -> Result<()> {
        let url = format!(
            "/phone/voice_mails/{}",
            crate::progenitor_support::encode_path(&voicemail_id.to_string()),
        );

        self.client.delete(&url, None).await
    }
}
