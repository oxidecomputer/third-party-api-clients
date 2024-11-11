use crate::Client;
use crate::ClientResult;

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
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/accounts/{}/phone/setup",
                crate::progenitor_support::encode_path(account_id),
            ),
            None,
        );
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
    ) -> ClientResult<crate::Response<Vec<crate::types::ListAccountPhoneNumbersResponse>>> {
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
        let url = self.client.url(&format!("/phone/numbers?{}", query_), None);
        let resp: crate::Response<crate::types::ListAccountPhoneNumbersResponseData> = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(crate::Response::new(
            resp.status,
            resp.headers,
            resp.body.phone_numbers.to_vec(),
        ))
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
    ) -> ClientResult<crate::Response<Vec<crate::types::ListAccountPhoneNumbersResponse>>> {
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
        let url = self.client.url(&format!("/phone/numbers?{}", query_), None);
        let crate::Response::<crate::types::ListAccountPhoneNumbersResponseData> {
            mut status,
            mut headers,
            mut body,
        } = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut phone_numbers = body.phone_numbers;
        let mut page = body.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            // Check if we already have URL params and need to concat the token.
            if !url.contains('?') {
                crate::Response::<crate::types::ListAccountPhoneNumbersResponseData> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}?next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                crate::Response::<crate::types::ListAccountPhoneNumbersResponseData> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}&next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            phone_numbers.append(&mut body.phone_numbers);

            if !body.next_page_token.is_empty() && body.next_page_token != page {
                page = body.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(crate::Response::new(status, headers, phone_numbers))
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
    pub async fn user(
        &self,
        user_id: &str,
    ) -> ClientResult<crate::Response<crate::types::PhoneUserResponse>> {
        let url = self.client.url(
            &format!(
                "/phone/users/{}",
                crate::progenitor_support::encode_path(user_id),
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
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/phone/users/{}",
                crate::progenitor_support::encode_path(user_id),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
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
    pub async fn setting(
        &self,
        account_id: &str,
    ) -> ClientResult<crate::Response<crate::types::PhoneSettingResponse>> {
        let url = self.client.url("/phone/settings", None);
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
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url("/phone/settings", None);
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
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
    ) -> ClientResult<crate::Response<crate::types::PhoneUserSettingsResponse>> {
        let url = self.client.url(
            &format!(
                "/phone/users/{}/settings",
                crate::progenitor_support::encode_path(user_id),
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
    ) -> ClientResult<crate::Response<Vec<crate::types::Templates>>> {
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
        let url = self
            .client
            .url(&format!("/phone/setting_templates?{}", query_), None);
        let resp: crate::Response<crate::types::ListSettingTemplatesResponse> = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(crate::Response::new(
            resp.status,
            resp.headers,
            resp.body.templates.to_vec(),
        ))
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
    ) -> ClientResult<crate::Response<Vec<crate::types::Templates>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !site_id.is_empty() {
            query_args.push(("site_id".to_string(), site_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/phone/setting_templates?{}", query_), None);
        let crate::Response::<crate::types::ListSettingTemplatesResponse> {
            mut status,
            mut headers,
            mut body,
        } = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut templates = body.templates;
        let mut page = body.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            // Check if we already have URL params and need to concat the token.
            if !url.contains('?') {
                crate::Response::<crate::types::ListSettingTemplatesResponse> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}?next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                crate::Response::<crate::types::ListSettingTemplatesResponse> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}&next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            templates.append(&mut body.templates);

            if !body.next_page_token.is_empty() && body.next_page_token != page {
                page = body.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(crate::Response::new(status, headers, templates))
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
    ) -> ClientResult<crate::Response<crate::types::AddSettingTemplateResponse>> {
        let url = self
            .client
            .url("/phone/setting_templates", None);
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
     * Batch add emergency service locations.
     *
     * This function performs a `POST` to the `/phone/batch_locations` endpoint.
     *
     * Use this API to batch add emergency service locations.
     */
    pub async fn batch_add_locations(
        &self,
        body: &crate::types::BatchAddLocationsRequest,
    ) -> ClientResult<crate::Response<Vec<crate::types::BatchAddLocationsResponse>>> {
        let url = self.client.url("/phone/batch_locations", None);
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
    ) -> ClientResult<crate::Response<Vec<crate::types::ListLocationsResponse>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/phone/locations?{}", query_), None);
        let resp: crate::Response<crate::types::ListLocationsResponseData> = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(crate::Response::new(
            resp.status,
            resp.headers,
            resp.body.locations.to_vec(),
        ))
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
    pub async fn list_all_locations(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::ListLocationsResponse>>> {
        let url = self.client.url("/phone/locations", None);
        let crate::Response::<crate::types::ListLocationsResponseData> {
            mut status,
            mut headers,
            mut body,
        } = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut locations = body.locations;
        let mut page = body.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            // Check if we already have URL params and need to concat the token.
            if !url.contains('?') {
                crate::Response::<crate::types::ListLocationsResponseData> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}?next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                crate::Response::<crate::types::ListLocationsResponseData> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}&next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            locations.append(&mut body.locations);

            if !body.next_page_token.is_empty() && body.next_page_token != page {
                page = body.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(crate::Response::new(status, headers, locations))
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
    ) -> ClientResult<crate::Response<Vec<crate::types::Site>>> {
        let url = self.client.url("/phone/locations", None);
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
    ) -> ClientResult<crate::Response<crate::types::GetLocationResponse>> {
        let url = self.client.url(
            &format!(
                "/phone/locations/{}",
                crate::progenitor_support::encode_path(location_id),
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
    pub async fn delete_location(&self, location_id: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/phone/locations/{}",
                crate::progenitor_support::encode_path(location_id),
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
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/phone/locations/{}",
                crate::progenitor_support::encode_path(location_id),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
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
    ) -> ClientResult<crate::Response<Vec<crate::types::SipGroups>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/phone/sip_groups?{}", query_), None);
        let resp: crate::Response<crate::types::ListSipGroupsResponse> = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(crate::Response::new(
            resp.status,
            resp.headers,
            resp.body.sip_groups.to_vec(),
        ))
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
    pub async fn list_all_sip_groups(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::SipGroups>>> {
        let url = self.client.url("/phone/sip_groups", None);
        let crate::Response::<crate::types::ListSipGroupsResponse> {
            mut status,
            mut headers,
            mut body,
        } = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut sip_groups = body.sip_groups;
        let mut page = body.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            // Check if we already have URL params and need to concat the token.
            if !url.contains('?') {
                crate::Response::<crate::types::ListSipGroupsResponse> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}?next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                crate::Response::<crate::types::ListSipGroupsResponse> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}&next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            sip_groups.append(&mut body.sip_groups);

            if !body.next_page_token.is_empty() && body.next_page_token != page {
                page = body.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(crate::Response::new(status, headers, sip_groups))
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
    ) -> ClientResult<crate::Response<crate::types::GetSettingTemplateResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !custom_query_fields.is_empty() {
            query_args.push((
                "custom_query_fields".to_string(),
                custom_query_fields.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/phone/setting_templates/{}?{}",
                crate::progenitor_support::encode_path(template_id),
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
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/phone/setting_templates/{}",
                crate::progenitor_support::encode_path(template_id),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
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
    ) -> ClientResult<crate::Response<Vec<crate::types::CallLogs>>> {
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
        let url = self.client.url(
            &format!(
                "/phone/users/{}/call_logs?{}",
                crate::progenitor_support::encode_path(user_id),
                query_
            ),
            None,
        );
        let resp: crate::Response<crate::types::PhoneUserCallLogsResponse> = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(crate::Response::new(
            resp.status,
            resp.headers,
            resp.body.call_logs.to_vec(),
        ))
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
    ) -> ClientResult<crate::Response<Vec<crate::types::CallLogs>>> {
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
        let url = self.client.url(
            &format!(
                "/phone/users/{}/call_logs?{}",
                crate::progenitor_support::encode_path(user_id),
                query_
            ),
            None,
        );
        let crate::Response::<crate::types::PhoneUserCallLogsResponse> {
            mut status,
            mut headers,
            mut body,
        } = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut call_logs = body.call_logs;
        let mut page = body.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            // Check if we already have URL params and need to concat the token.
            if !url.contains('?') {
                crate::Response::<crate::types::PhoneUserCallLogsResponse> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}?next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                crate::Response::<crate::types::PhoneUserCallLogsResponse> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}&next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            call_logs.append(&mut body.call_logs);

            if !body.next_page_token.is_empty() && body.next_page_token != page {
                page = body.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(crate::Response::new(status, headers, call_logs))
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
    ) -> ClientResult<crate::Response<Vec<crate::types::Recordings>>> {
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
        let url = self.client.url(
            &format!(
                "/phone/users/{}/recordings?{}",
                crate::progenitor_support::encode_path(user_id),
                query_
            ),
            None,
        );
        let resp: crate::Response<crate::types::PhoneUserRecordingsResponse> = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(crate::Response::new(
            resp.status,
            resp.headers,
            resp.body.recordings.to_vec(),
        ))
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
    ) -> ClientResult<crate::Response<Vec<crate::types::Recordings>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !to.to_string().is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/phone/users/{}/recordings?{}",
                crate::progenitor_support::encode_path(user_id),
                query_
            ),
            None,
        );
        let crate::Response::<crate::types::PhoneUserRecordingsResponse> {
            mut status,
            mut headers,
            mut body,
        } = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut recordings = body.recordings;
        let mut page = body.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            // Check if we already have URL params and need to concat the token.
            if !url.contains('?') {
                crate::Response::<crate::types::PhoneUserRecordingsResponse> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}?next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                crate::Response::<crate::types::PhoneUserRecordingsResponse> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}&next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            recordings.append(&mut body.recordings);

            if !body.next_page_token.is_empty() && body.next_page_token != page {
                page = body.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(crate::Response::new(status, headers, recordings))
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
    ) -> ClientResult<crate::Response<Vec<crate::types::VoiceMails>>> {
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
        let url = self.client.url(
            &format!(
                "/phone/users/{}/voice_mails?{}",
                crate::progenitor_support::encode_path(user_id),
                query_
            ),
            None,
        );
        let resp: crate::Response<crate::types::PhoneUserVoiceMailsResponse> = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(crate::Response::new(
            resp.status,
            resp.headers,
            resp.body.voice_mails.to_vec(),
        ))
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
    ) -> ClientResult<crate::Response<Vec<crate::types::VoiceMails>>> {
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
        let url = self.client.url(
            &format!(
                "/phone/users/{}/voice_mails?{}",
                crate::progenitor_support::encode_path(user_id),
                query_
            ),
            None,
        );
        let crate::Response::<crate::types::PhoneUserVoiceMailsResponse> {
            mut status,
            mut headers,
            mut body,
        } = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut voice_mails = body.voice_mails;
        let mut page = body.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            // Check if we already have URL params and need to concat the token.
            if !url.contains('?') {
                crate::Response::<crate::types::PhoneUserVoiceMailsResponse> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}?next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                crate::Response::<crate::types::PhoneUserVoiceMailsResponse> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}&next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            voice_mails.append(&mut body.voice_mails);

            if !body.next_page_token.is_empty() && body.next_page_token != page {
                page = body.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(crate::Response::new(status, headers, voice_mails))
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
    ) -> ClientResult<crate::Response<crate::types::AddUserSettingResponse>> {
        let url = self.client.url(
            &format!(
                "/phone/users/{}/settings/{}",
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(setting_type),
            ),
            None,
        );
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
    ) -> ClientResult<crate::Response<()>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !shared_id.is_empty() {
            query_args.push(("shared_id".to_string(), shared_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/phone/users/{}/settings/{}?{}",
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(setting_type),
                query_
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
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/phone/users/{}/settings/{}",
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(setting_type),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
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
    ) -> ClientResult<crate::Response<Vec<crate::types::AccountCallLogsResponse>>> {
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
        let url = self
            .client
            .url(&format!("/phone/call_logs?{}", query_), None);
        let resp: crate::Response<crate::types::AccountCallLogsResponseData> = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(crate::Response::new(
            resp.status,
            resp.headers,
            resp.body.call_logs.to_vec(),
        ))
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
    ) -> ClientResult<crate::Response<Vec<crate::types::AccountCallLogsResponse>>> {
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
        let url = self
            .client
            .url(&format!("/phone/call_logs?{}", query_), None);
        let crate::Response::<crate::types::AccountCallLogsResponseData> {
            mut status,
            mut headers,
            mut body,
        } = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut call_logs = body.call_logs;
        let mut page = body.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            // Check if we already have URL params and need to concat the token.
            if !url.contains('?') {
                crate::Response::<crate::types::AccountCallLogsResponseData> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}?next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                crate::Response::<crate::types::AccountCallLogsResponseData> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}&next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            call_logs.append(&mut body.call_logs);

            if !body.next_page_token.is_empty() && body.next_page_token != page {
                page = body.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(crate::Response::new(status, headers, call_logs))
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
    ) -> ClientResult<crate::Response<crate::types::AddByocNumberResponse>> {
        let url = self.client.url(
            &format!(
                "/phone/users/{}/phone_numbers",
                crate::progenitor_support::encode_path(user_id),
            ),
            None,
        );
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
    pub async fn unassign_number(
        &self,
        user_id: &str,
        phone_number_id: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/phone/users/{}/phone_numbers/{}",
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(phone_number_id),
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
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/phone/users/{}/calling_plans",
                crate::progenitor_support::encode_path(user_id),
            ),
            None,
        );
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
    pub async fn unassign_calling_plan(
        &self,
        user_id: &str,
        type_: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/phone/users/{}/calling_plans/{}",
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(type_),
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
    ) -> ClientResult<crate::Response<Vec<crate::types::GetPhoneRecordingsResponse>>> {
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
        let url = self
            .client
            .url(&format!("/phone/recordings?{}", query_), None);
        let resp: crate::Response<crate::types::GetPhoneRecordingsResponseData> = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(crate::Response::new(
            resp.status,
            resp.headers,
            resp.body.recordings.to_vec(),
        ))
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
    ) -> ClientResult<crate::Response<Vec<crate::types::GetPhoneRecordingsResponse>>> {
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
        let url = self
            .client
            .url(&format!("/phone/recordings?{}", query_), None);
        let crate::Response::<crate::types::GetPhoneRecordingsResponseData> {
            mut status,
            mut headers,
            mut body,
        } = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut recordings = body.recordings;
        let mut page = body.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            // Check if we already have URL params and need to concat the token.
            if !url.contains('?') {
                crate::Response::<crate::types::GetPhoneRecordingsResponseData> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}?next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                crate::Response::<crate::types::GetPhoneRecordingsResponseData> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}&next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            recordings.append(&mut body.recordings);

            if !body.next_page_token.is_empty() && body.next_page_token != page {
                page = body.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(crate::Response::new(status, headers, recordings))
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
    ) -> ClientResult<crate::Response<Vec<crate::types::ByocSipTrunk>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/phone/sip_trunk/trunks?{}", query_), None);
        let resp: crate::Response<crate::types::ListByocsipTrunkResponse> = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(crate::Response::new(
            resp.status,
            resp.headers,
            resp.body.byoc_sip_trunk.to_vec(),
        ))
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
    pub async fn list_all_byocsip_trunk(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::ByocSipTrunk>>> {
        let url = self
            .client
            .url("/phone/sip_trunk/trunks", None);
        let crate::Response::<crate::types::ListByocsipTrunkResponse> {
            mut status,
            mut headers,
            mut body,
        } = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut byoc_sip_trunk = body.byoc_sip_trunk;
        let mut page = body.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            // Check if we already have URL params and need to concat the token.
            if !url.contains('?') {
                crate::Response::<crate::types::ListByocsipTrunkResponse> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}?next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                crate::Response::<crate::types::ListByocsipTrunkResponse> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}&next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            byoc_sip_trunk.append(&mut body.byoc_sip_trunk);

            if !body.next_page_token.is_empty() && body.next_page_token != page {
                page = body.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(crate::Response::new(status, headers, byoc_sip_trunk))
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
    ) -> ClientResult<crate::Response<crate::types::PostPhoneSipTrunkRequest>> {
        let url = self.client.url(
            &format!(
                "/accounts/{}/phone/sip_trunk/trunks",
                crate::progenitor_support::encode_path(account_id),
            ),
            None,
        );
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
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/accounts/{}/phone/sip_trunk/trunks/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(sip_trunk_id),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
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
    ) -> ClientResult<crate::Response<Vec<crate::types::ExternalContacts>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/phone/external_contacts?{}", query_), None);
        let resp: crate::Response<crate::types::ListExternalContactsResponse> = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(crate::Response::new(
            resp.status,
            resp.headers,
            resp.body.external_contacts.to_vec(),
        ))
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
    pub async fn list_all_external_contacts(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::ExternalContacts>>> {
        let url = self
            .client
            .url("/phone/external_contacts", None);
        let crate::Response::<crate::types::ListExternalContactsResponse> {
            mut status,
            mut headers,
            mut body,
        } = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut external_contacts = body.external_contacts;
        let mut page = body.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            // Check if we already have URL params and need to concat the token.
            if !url.contains('?') {
                crate::Response::<crate::types::ListExternalContactsResponse> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}?next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                crate::Response::<crate::types::ListExternalContactsResponse> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}&next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            external_contacts.append(&mut body.external_contacts);

            if !body.next_page_token.is_empty() && body.next_page_token != page {
                page = body.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(crate::Response::new(status, headers, external_contacts))
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
    ) -> ClientResult<crate::Response<()>> {
        let url = self
            .client
            .url("/phone/external_contacts", None);
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
    ) -> ClientResult<crate::Response<crate::types::ExternalContacts>> {
        let url = self.client.url(
            &format!(
                "/phone/external_contacts/{}",
                crate::progenitor_support::encode_path(external_contact_id),
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
    pub async fn delete_external_contact(
        &self,
        external_contact_id: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/phone/external_contacts/{}",
                crate::progenitor_support::encode_path(external_contact_id),
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
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/phone/external_contacts/{}",
                crate::progenitor_support::encode_path(external_contact_id),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
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
    ) -> ClientResult<crate::Response<crate::types::GetPhoneNumberDetailsResponse>> {
        let url = self.client.url(
            &format!(
                "/phone/numbers/{}",
                crate::progenitor_support::encode_path(number_id),
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
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/phone/numbers/{}",
                crate::progenitor_support::encode_path(number_id),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
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
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url("/phone/company_number", None);
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
    pub async fn list_calling_plan(
        &self,
    ) -> ClientResult<crate::Response<crate::types::ListCallingPlansResponseData>> {
        let url = self.client.url("/phone/calling_plans", None);
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
    ) -> ClientResult<crate::Response<Vec<crate::types::ListPhoneUsersResponse>>> {
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
        let url = self.client.url(&format!("/phone/users?{}", query_), None);
        let resp: crate::Response<crate::types::ListPhoneUsersResponseData> = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(crate::Response::new(
            resp.status,
            resp.headers,
            resp.body.users.to_vec(),
        ))
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
    ) -> ClientResult<crate::Response<Vec<crate::types::ListPhoneUsersResponse>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !site_id.is_empty() {
            query_args.push(("site_id".to_string(), site_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/phone/users?{}", query_), None);
        let crate::Response::<crate::types::ListPhoneUsersResponseData> {
            mut status,
            mut headers,
            mut body,
        } = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut users = body.users;
        let mut page = body.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            // Check if we already have URL params and need to concat the token.
            if !url.contains('?') {
                crate::Response::<crate::types::ListPhoneUsersResponseData> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}?next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                crate::Response::<crate::types::ListPhoneUsersResponseData> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}&next_page_token={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            users.append(&mut body.users);

            if !body.next_page_token.is_empty() && body.next_page_token != page {
                page = body.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(crate::Response::new(status, headers, users))
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
    ) -> ClientResult<crate::Response<crate::types::GetCallLogDetailsResponse>> {
        let url = self.client.url(
            &format!(
                "/phone/call_logs/{}",
                crate::progenitor_support::encode_path(call_log_id),
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
    pub async fn delete_call_log(
        &self,
        user_id: &str,
        call_log_id: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/phone/users/{}/call_logs/{}",
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(call_log_id),
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
    ) -> ClientResult<crate::Response<crate::types::AddByocNumberResponse>> {
        let url = self.client.url("/phone/byoc_numbers", None);
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
    pub async fn delete_voicemail(&self, voicemail_id: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/phone/voice_mails/{}",
                crate::progenitor_support::encode_path(voicemail_id),
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
}
