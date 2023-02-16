use anyhow::Result;

use crate::Client;

pub struct GroupUsers {
    pub client: Client,
}

impl GroupUsers {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        GroupUsers { client }
    }

    /**
     * Gets a list of users in a group.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/groups/{groupId}/users` endpoint.
     *
     * Retrieves a list of users in a group.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `group_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `count: &str` -- Number of records to return. The number must be greater than 1 and less than or equal to 100. .
     * * `start_position: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn groups_get(
        &self,
        account_id: &str,
        group_id: &str,
        count: &str,
        start_position: &str,
    ) -> Result<crate::types::UsersResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !count.is_empty() {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !start_position.is_empty() {
            query_args.push(("start_position".to_string(), start_position.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/v2.1/accounts/{}/groups/{}/users?{}",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&group_id.to_string()),
            query_
        );
        let url = self.client.url(&url, None);
        self.client.get(&url, None).await
    }
    /**
     * Adds one or more users to an existing group.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/groups/{groupId}/users` endpoint.
     *
     * Adds one or more existing DocuSign users to an existing group.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `group_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn groups_put(
        &self,
        account_id: &str,
        group_id: &str,
        body: &crate::types::UserInfoList,
    ) -> Result<crate::types::UsersResponse> {
        let url = format!(
            "/v2.1/accounts/{}/groups/{}/users",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&group_id.to_string()),
        );
        let url = self.client.url(&url, None);
        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
    /**
     * Deletes one or more users from a group.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/groups/{groupId}/users` endpoint.
     *
     * Deletes one or more users from a group. This request takes a `userInfoList` that contains the users that you want to delete.
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `group_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn groups_delete(
        &self,
        account_id: &str,
        group_id: &str,
        body: &crate::types::UserInfoList,
    ) -> Result<crate::types::UsersResponse> {
        let url = format!(
            "/v2.1/accounts/{}/groups/{}/users",
            crate::progenitor_support::encode_path(&account_id.to_string()),
            crate::progenitor_support::encode_path(&group_id.to_string()),
        );
        let url = self.client.url(&url, None);
        self.client
            .delete(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
