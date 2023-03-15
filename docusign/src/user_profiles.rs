use crate::Client;
use crate::ClientResult;

pub struct UserProfiles {
    pub client: Client,
}

impl UserProfiles {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UserProfiles { client }
    }

    /**
     * Retrieves the user profile for a specified user.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/users/{userId}/profile` endpoint.
     *
     * Retrieves the user profile information, the privacy settings and personal information (address, phone number, etc.) for the specified user.
     *
     * The userId parameter specified in the endpoint must match the authenticated user's user ID and the user must be a member of the specified account.
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
    ) -> ClientResult<crate::types::UserProfile> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/profile",
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
     * Updates the user profile information for the specified user.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/users/{userId}/profile` endpoint.
     *
     * Updates the user's detail information, profile information, privacy settings, and personal information in the user ID card.
     *
     * You can also change a user's name by changing the information in the `userDetails` property. When changing a user's name, you can either change the information in the `userName` property OR change the information in `firstName`, `middleName`, `lastName, suffixName`, and `title` properties. Changes to `firstName`, `middleName`, `lastName`, `suffixName`, and `title` properties take precedence over changes to the `userName` property.
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
        body: &crate::types::UserProfile,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/profile",
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
