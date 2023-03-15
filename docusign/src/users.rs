use crate::Client;
use crate::ClientResult;

pub struct Users {
    pub client: Client,
}

impl Users {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Users { client }
    }

    /**
     * Retrieves the list of users for the specified account.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/users` endpoint.
     *
     * Retrieves the list of users for the specified account.
     *
     * The response returns the list of users for the account, with information about the result set. If the `additional_info` query is added to the endpoint and set to **true**, full user information is returned for each user.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `additional_info: &str` -- When set to **true**, the custom settings information is returned for each user in the account. If this parameter is omitted, the default behavior is **false**.
     * * `count: &str` -- The number of records to return. This number must be greater than `0` and less than or equal to `100`. .
     * * `email: &str` -- Filters results based on the email address associated with the user that you want to return.
     *   
     *   **Note**: You can use either this parameter or the `email_substring` parameter, but not both. For older accounts, this parameter might return multiple users who are associated with a single email address.
     * * `email_substring: &str` -- Filters results based on a fragment of an email address. For example, you could enter `gmail` to return all users who have Gmail addresses.
     *   
     *   **Note**: You do not use a wildcard character with this parameter. You can use either this parameter or the `email` parameter, but not both.
     * * `group_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include_usersettings_for_csv: &str` -- When set to **true**, the response includes the `userSettings` object data in CSV format.
     * * `login_status: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `not_group_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `start_position: &str` -- The position within the total result set from which to start returning values.
     * * `status: &str` -- Filters results by user account status. Possible values are:
     *   
     *   * `ActivationRequired`
     *   * `ActivationSent`
     *   * `Active`
     *   * `Closed`
     *   * `Disabled`
     *   .
     * * `user_name_substring: &str` -- Filters results based on a full or partial user name.
     *   
     *   **Note**: When you enter a partial user name, you do not use a wildcard character.
     */
    pub async fn get(
        &self,
        account_id: &str,
        additional_info: &str,
        count: &str,
        email: &str,
        email_substring: &str,
        group_id: &str,
        include_usersettings_for_csv: &str,
        login_status: &str,
        not_group_id: &str,
        start_position: &str,
        status: &str,
        user_name_substring: &str,
    ) -> ClientResult<crate::types::UserInformationList> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !additional_info.is_empty() {
            query_args.push(("additional_info".to_string(), additional_info.to_string()));
        }
        if !count.is_empty() {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !email.is_empty() {
            query_args.push(("email".to_string(), email.to_string()));
        }
        if !email_substring.is_empty() {
            query_args.push(("email_substring".to_string(), email_substring.to_string()));
        }
        if !group_id.is_empty() {
            query_args.push(("group_id".to_string(), group_id.to_string()));
        }
        if !include_usersettings_for_csv.is_empty() {
            query_args.push((
                "include_usersettings_for_csv".to_string(),
                include_usersettings_for_csv.to_string(),
            ));
        }
        if !login_status.is_empty() {
            query_args.push(("login_status".to_string(), login_status.to_string()));
        }
        if !not_group_id.is_empty() {
            query_args.push(("not_group_id".to_string(), not_group_id.to_string()));
        }
        if !start_position.is_empty() {
            query_args.push(("start_position".to_string(), start_position.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !user_name_substring.is_empty() {
            query_args.push((
                "user_name_substring".to_string(),
                user_name_substring.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users?{}",
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
     * Changes one or more users in the specified account.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/users` endpoint.
     *
     * This method updates the information about one or more account users.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn put(
        &self,
        account_id: &str,
        body: &crate::types::UserInformationList,
    ) -> ClientResult<crate::types::UserInformationList> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users",
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
     * Adds new users to the specified account.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/users` endpoint.
     *
     * Adds new users to an account.
     *
     * The body of this request is an array of `newUsers` objects. For each new user, you must provide at least the `userName` and `email` properties.
     *
     * The `userSettings` property specifies the actions users can perform. In the example below, Tal Mason will be able to send envelopes, and the activation email will be in French because the `locale` is set to `fr`.
     *
     *
     * ```
     * POST /restapi/v2.1/accounts/{accountId}/users
     * Content-Type: application/json
     * ```
     * ```
     * {
     *   "newUsers": [
     *     {
     *       "userName": "Claire Horace",
     *       "email": "claire@example.com"
     *     },
     *     {
     *       "userName": "Tal Mason",
     *       "email": "talmason@example.com",
     *       "company": "TeleSel",
     *       "userSettings": {
     *         "locale": "fr",
     *         "canSendEnvelope": true
     *       }
     *     }
     *   ]
     * }
     * ```
     *
     * A successful response is a `newUsers` array with information about the newly created users. If there was a problem in creating a user, that user entry will contain an `errorDetails` property that describes what went wrong.
     *
     * ```json
     * {
     *   "newUsers": [
     *     {
     *       "userId": "18f3be12-xxxx-xxxx-xxxx-883d8f9b8ade",
     *       "uri": "/users/18f3be12-xxxx-xxxx-xxxx-883d8f9b8ade",
     *       "email": "claire@example.com",
     *       "userName": "Claire Horace",
     *       "createdDateTime": "0001-01-01T08:00:00.0000000Z",
     *       "errorDetails": {
     *         "errorCode": "USER_ALREADY_EXISTS_IN_ACCOUNT",
     *         "message": "Username and email combination already exists for this account."
     *       }
     *     },
     *     {
     *       "userId": "be9899a3-xxxx-xxxx-xxxx-2c8dd7156e33",
     *       "uri": "/users/be9899a3-xxxx-xxxx-xxxx-2c8dd7156e33",
     *       "email": "talmason@example.com",
     *       "userName": "Tal Mason",
     *       "userStatus": "ActivationSent",
     *       "createdDateTime": "2020-05-26T23:25:30.7330000Z"
     *     }
     *   ]
     * }
     * ```
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn post(
        &self,
        account_id: &str,
        body: &crate::types::NewUsersDefinition,
    ) -> ClientResult<crate::types::NewUsersSummary> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users",
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
     * Removes users account privileges.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/users` endpoint.
     *
     * Closes one or more user records in the account. Users are never deleted from an account, but closing a user prevents them from using account functions.
     *
     * The response specifies whether the API execution succeeded (200 - OK) or failed (400 - Error). The response contains a user structure similar to the request and includes the user changes. If an error occurred during the DELETE operation for any of the users, the response for that user contains an `errorDetails` property with `errorCode` and `message` properties.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `delete: &str` -- ID of the user to delete. This parameter takes a comma-separated list of values in the format: `Groups,PermissionSet,SigningGroupsEmail`.
     */
    pub async fn delete(
        &self,
        account_id: &str,
        delete: &str,
        body: &crate::types::UserInfoList,
    ) -> ClientResult<crate::types::UsersResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !delete.is_empty() {
            query_args.push(("delete".to_string(), delete.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users?{}",
                crate::progenitor_support::encode_path(account_id),
                query_
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
     * Gets the user information for a specified user.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/users/{userId}` endpoint.
     *
     * Retrieves the user information for the specified user.
     *
     * To return additional user information that details the last login date, login status, and the user's password expiration date, set the optional `additional_info` query string parameter to **true**.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     * * `additional_info: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `email: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn get_users(
        &self,
        account_id: &str,
        user_id: &str,
        additional_info: &str,
        email: &str,
    ) -> ClientResult<crate::types::UserInformation> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !additional_info.is_empty() {
            query_args.push(("additional_info".to_string(), additional_info.to_string()));
        }
        if !email.is_empty() {
            query_args.push(("email".to_string(), email.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(user_id),
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
     * Updates user information for the specified user.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/users/{userId}` endpoint.
     *
     * To update user information for a specific user, submit a [Users](#Users) object with updated field values in the request body of this operation.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     */
    pub async fn put_users(
        &self,
        account_id: &str,
        user_id: &str,
        body: &crate::types::UserInformation,
    ) -> ClientResult<crate::types::UserInformation> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(user_id),
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
     * Retrieves the user profile image for the specified user.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/users/{userId}/profile/image` endpoint.
     *
     * Retrieves the user profile picture for the specified user. The image is returned in the same format as uploaded.
     *
     * The userId parameter specified in the endpoint must match the authenticated user's user ID and the user must be a member of the specified account.
     *
     * If successful, the response returns a 200 - OK and the user profile image.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     * * `encoding: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn profile_image_get(
        &self,
        account_id: &str,
        user_id: &str,
        encoding: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !encoding.is_empty() {
            query_args.push(("encoding".to_string(), encoding.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/profile/image?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(user_id),
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
     * Updates the user profile image for a specified user.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/users/{userId}/profile/image` endpoint.
     *
     * Updates the user profile image by uploading an image to the user profile.
     *
     * The supported image formats are: gif, png, jpeg, and bmp. The file must be less than 200K. For best viewing results, DocuSign recommends that the image is no more than 79 pixels wide and high.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     */
    pub async fn profile_image_put(&self, account_id: &str, user_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/profile/image",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(user_id),
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
     * Deletes the user profile image for the specified user.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/users/{userId}/profile/image` endpoint.
     *
     * Deletes the user profile image from the  specified user's profile.
     *
     * The userId parameter specified in the endpoint must match the authenticated user's user ID and the user must be a member of the specified account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     */
    pub async fn profile_image_delete(&self, account_id: &str, user_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/profile/image",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(user_id),
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
     * Gets the user account settings for a specified user.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/users/{userId}/settings` endpoint.
     *
     * Retrieves a list of the account settings and email
     * notification information for the specified user.
     *
     * The response returns the account setting
     * name/value information and the email notification
     * settings for the specified user. For more
     * information, see
     * [Users:create](https://developers.docusign.com/docs/esign-rest-api/reference/Users/Users/create/).
     *
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     */
    pub async fn settings_get(
        &self,
        account_id: &str,
        user_id: &str,
    ) -> ClientResult<crate::types::UserSettingsInformation> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/settings",
                crate::progenitor_support::encode_path(account_id),
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
     * Updates the user account settings for a specified user.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/users/{userId}/settings` endpoint.
     *
     * Updates the account settings list and email notification types for the specified user.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     */
    pub async fn settings_put(
        &self,
        account_id: &str,
        user_id: &str,
        body: &crate::types::UserSettingsInformation,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/settings",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(user_id),
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
}
