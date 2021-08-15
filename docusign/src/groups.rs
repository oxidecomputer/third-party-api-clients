use anyhow::Result;

use crate::Client;

pub struct Groups {
    client: Client,
}

impl Groups {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Groups { client }
    }

    /**
     * Gets information about groups associated with the account.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/groups` endpoint.
     *
     * Retrieves information about groups associated with the account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `count: &str` -- Number of records to return. The number must be greater than 1 and less than or equal to 100.
     * * `group_type: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include_usercount: &str` -- When set to **true**, every group returned in the response includes a `userCount` property that contains the total number of users in the group. The default is **true**.
     * * `search_text: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `start_position: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn get(
        &self,
        account_id: &str,
        count: &str,
        group_type: &str,
        include_usercount: &str,
        search_text: &str,
        start_position: &str,
    ) -> Result<crate::types::GroupInformation> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !count.is_empty() {
            query_args.push(format!("count={}", count));
        }
        if !group_type.is_empty() {
            query_args.push(format!("group_type={}", group_type));
        }
        if !include_usercount.is_empty() {
            query_args.push(format!("include_usercount={}", include_usercount));
        }
        if !search_text.is_empty() {
            query_args.push(format!("search_text={}", search_text));
        }
        if !start_position.is_empty() {
            query_args.push(format!("start_position={}", start_position));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/v2.1/accounts/{}/groups?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Updates the group information for a group.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/groups` endpoint.
     *
     * Updates the group name and modifies, or sets, the permission profile for the group.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn put(
        &self,
        account_id: &str,
        body: &crate::types::GroupInformation,
    ) -> Result<crate::types::GroupInformation> {
        let url = format!(
            "/v2.1/accounts/{}/groups",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Creates one or more groups for the account.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/groups` endpoint.
     *
     * Creates one or more groups for the account.
     *
     * Groups can be used to help manage users by associating users with a group. You can associate a group with a Permission Profile, which sets the user permissions for users in that group without having to set the `userSettings` property for each user. You are not required to set Permission Profiles for a group, but it makes it easier to manage user permissions for a large number of users. You can also use groups with template sharing to limit user access to templates.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn post(
        &self,
        account_id: &str,
        body: &crate::types::GroupInformation,
    ) -> Result<crate::types::GroupInformation> {
        let url = format!(
            "/v2.1/accounts/{}/groups",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Deletes an existing user group.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/groups` endpoint.
     *
     * Deletes an existing user group.
     *
     * When you delete a group, you include only the `groupId` in the request body.
     *
     * Example:
     *
     * ```
     * {
     *   "groups": [
     *     {
     *       "groupId": "12345"
     *     }
     * }
     * ```
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn delete(
        &self,
        account_id: &str,
        body: &crate::types::GroupInformation,
    ) -> Result<crate::types::GroupInformation> {
        let url = format!(
            "/v2.1/accounts/{}/groups",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .delete(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
