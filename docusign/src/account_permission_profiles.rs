use crate::Client;
use crate::ClientResult;

pub struct AccountPermissionProfiles {
    pub client: Client,
}

impl AccountPermissionProfiles {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AccountPermissionProfiles { client }
    }

    /**
     * Gets a list of permission profiles.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/permission_profiles` endpoint.
     *
     * This method returns a list of permission profiles that are associated with an account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include: &str` -- A comma-separated list of additional properties to return in the response. Valid values are:
     *   
     *   - `user_count`: The total number of users associated with the permission profile.
     *   - `closed_users`: Includes closed users in the `user_count`.
     *   - `account_management`: The account management settings.
     *   - `metadata`: Metadata indicating whether the properties associated with the account permission profile are editable.
     *   
     *   Example: `user_count,closed_users`
     *   .
     */
    pub async fn permission_profiles_get(
        &self,
        account_id: &str,
        include: &str,
    ) -> ClientResult<crate::types::PermissionProfileInformation> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include.is_empty() {
            query_args.push(("include".to_string(), include.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/permission_profiles?{}",
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
     * Creates a new permission profile for an account.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/permission_profiles` endpoint.
     *
     * This method creates a new permission profile for an account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `include: &str` -- A comma-separated list of additional properties to return in the response. The only valid value for this request is `metadata`, which returns metadata indicating whether the properties associated with the account permission profile are editable.
     */
    pub async fn permission_profiles_post(
        &self,
        account_id: &str,
        include: &str,
        body: &crate::types::PermissionProfile,
    ) -> ClientResult<crate::types::PermissionProfile> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include.is_empty() {
            query_args.push(("include".to_string(), include.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/permission_profiles?{}",
                crate::progenitor_support::encode_path(account_id),
                query_
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Returns a permission profile for an account.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/permission_profiles/{permissionProfileId}` endpoint.
     *
     * This method returns information about a specific permission profile that is associated with an account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `permission_profile_id: &str` -- The ID of the permission profile. Possible values include:
     *   
     *   - `2301416` (for the `DocuSign Viewer` profile)
     *   - `2301415` (for the `DocuSign Sender` profile)
     *   - `2301414` (for the `Account Administrator` profile)
     *   
     *   In addition, any custom permission profiles associated with your account will have an automatically generated `permissionProfileId`.
     * * `include: &str` -- A comma-separated list of additional properties to return in the response. The only valid value for this request is `metadata`, which returns metadata indicating whether the properties associated with the account permission profile are editable.
     */
    pub async fn permission_profiles_get_profile(
        &self,
        account_id: &str,
        permission_profile_id: &str,
        include: &str,
    ) -> ClientResult<crate::types::PermissionProfile> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include.is_empty() {
            query_args.push(("include".to_string(), include.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/permission_profiles/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(permission_profile_id),
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
     * Updates a permission profile.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/permission_profiles/{permissionProfileId}` endpoint.
     *
     * This method updates an account permission profile.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `permission_profile_id: &str` -- The ID of the permission profile. Possible values include:
     *   
     *   - `2301416` (for the `DocuSign Viewer` profile)
     *   - `2301415` (for the `DocuSign Sender` profile)
     *   - `2301414` (for the `Account Administrator` profile)
     *   
     *   In addition, any custom permission profiles associated with your account will have an automatically generated `permissionProfileId`.
     * * `include: &str` -- A comma-separated list of additional properties to return in the response. The only valid value for this request is `metadata`, which returns metadata indicating whether the properties associated with the account permission profile are editable.
     */
    pub async fn permission_profiles_put(
        &self,
        account_id: &str,
        permission_profile_id: &str,
        include: &str,
        body: &crate::types::PermissionProfile,
    ) -> ClientResult<crate::types::PermissionProfile> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !include.is_empty() {
            query_args.push(("include".to_string(), include.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/permission_profiles/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(permission_profile_id),
                query_
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
     * Deletes a permission profile from an account.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/permission_profiles/{permissionProfileId}` endpoint.
     *
     * This method deletes a permission profile from an account.
     *
     * To delete a permission profile, it must not have any users associated with it. When you use this method to delete a permission profile, you can reassign the users associated with it to a new permission profile at the same time by using the `move_users_to` query parameter.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `permission_profile_id: &str` -- The ID of the permission profile. Possible values include:
     *   
     *   - `2301416` (for the `DocuSign Viewer` profile)
     *   - `2301415` (for the `DocuSign Sender` profile)
     *   - `2301414` (for the `Account Administrator` profile)
     *   
     *   In addition, any custom permission profiles associated with your account will have an automatically generated `permissionProfileId`.
     * * `move_users_to: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn permission_profiles_delete(
        &self,
        account_id: &str,
        permission_profile_id: &str,
        move_users_to: &str,
    ) -> ClientResult<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !move_users_to.is_empty() {
            query_args.push(("move_users_to".to_string(), move_users_to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/permission_profiles/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(permission_profile_id),
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
}
