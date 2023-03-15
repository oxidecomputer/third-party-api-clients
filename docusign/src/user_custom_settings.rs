use crate::Client;
use crate::ClientResult;

pub struct UserCustomSettings {
    pub client: Client,
}

impl UserCustomSettings {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UserCustomSettings { client }
    }

    /**
     * Retrieves the custom user settings for a specified user.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/users/{userId}/custom_settings` endpoint.
     *
     * Retrieves a list of custom user settings for a single user.
     *
     * Custom settings provide a flexible way to store and retrieve custom user information that can be used in your own system.
     *
     * **Note**: Custom user settings are not the same as user account settings.
     *
     * ###Getting Grouped Custom User Settings###
     *
     * If the custom user settings you want to retrieve are grouped, you must include the following information in the header, after Content-Type, in the request:
     *
     * `X-DocuSign-User-Settings-Key:group_name`
     *
     * Where the `group_name` is your designated name for the group of customer user settings.
     *
     * If the extra header information is not included, only the custom user settings that were added without a group are retrieved.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     */
    pub async fn get(
        &self,
        account_id: &str,
        user_id: &str,
    ) -> ClientResult<crate::types::CustomSettingsInformation> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/custom_settings",
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
     * Adds or updates custom user settings for the specified user.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/users/{userId}/custom_settings` endpoint.
     *
     * Adds or updates custom user settings for the specified user.
     *
     * ***Note**: Custom user settings are not the same as user account settings.
     *
     * Custom settings provide a flexible way to store and retrieve custom user information that you can use in your own system.
     *
     * **Important**: There is a limit on the size for all the custom user settings for a single user. The limit is 4,000 characters, which includes the XML and JSON structure for the settings.
     *
     * ### Grouping Custom User Settings ###
     *
     * You can group custom user settings when adding them. Grouping allows you to retrieve settings that are in a specific group, instead of retrieving all the user custom settings.
     *
     * To group custom user settings, add the following information in the header, after Content-Type:
     *
     * `X-DocuSign-User-Settings-Key:group_name`
     *
     * Where the `group_name` is your designated name for the group of customer user settings. Grouping is shown in the Example Request Body below.
     *
     * When getting or deleting grouped custom user settings, you must include the extra header information.
     *
     * Grouping custom user settings is not required and if the extra header information is not included, the custom user settings are added normally and can be retrieved or deleted without including the additional header.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     */
    pub async fn put(
        &self,
        account_id: &str,
        user_id: &str,
        body: &crate::types::CustomSettingsInformation,
    ) -> ClientResult<crate::types::CustomSettingsInformation> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/custom_settings",
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
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Deletes custom user settings for a specified user.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/users/{userId}/custom_settings` endpoint.
     *
     * Deletes the specified custom user settings for a single user.
     *
     * ###Deleting Grouped Custom User Settings###
     *
     * If the custom user settings you want to delete are grouped, you must include the following information in the header, after Content-Type, in the request:
     *
     * `X-DocuSign-User-Settings-Key:group_name`
     *
     * Where the `group_name` is your designated name for the group of customer user settings.
     *
     * If the extra header information is not included, only the custom user settings that were added without a group are deleted.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     */
    pub async fn delete(
        &self,
        account_id: &str,
        user_id: &str,
        body: &crate::types::CustomSettingsInformation,
    ) -> ClientResult<crate::types::CustomSettingsInformation> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/custom_settings",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(user_id),
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
