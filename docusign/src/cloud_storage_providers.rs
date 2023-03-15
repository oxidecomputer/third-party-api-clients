use crate::Client;
use crate::ClientResult;

pub struct CloudStorageProviders {
    pub client: Client,
}

impl CloudStorageProviders {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CloudStorageProviders { client }
    }

    /**
     * Get the Cloud Storage Provider configuration for the specified user.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/users/{userId}/cloud_storage` endpoint.
     *
     * Retrieves the list of cloud storage providers enabled for the account and the configuration information for the user.
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     * * `redirect_url: &str` --  The URL the user is redirected to after the cloud storage provider authenticates the user. Using this will append the redirectUrl to the authenticationUrl.
     *   
     *   The redirectUrl is restricted to URLs in the docusign.com or docusign.net domains.
     *    .
     */
    pub async fn cloud_storage_get_provider(
        &self,
        account_id: &str,
        user_id: &str,
        redirect_url: &str,
    ) -> ClientResult<crate::types::CloudStorageProvidersData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !redirect_url.is_empty() {
            query_args.push(("redirectUrl".to_string(), redirect_url.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/cloud_storage?{}",
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
     * Configures the redirect URL information  for one or more cloud storage providers for the specified user.
     *
     * This function performs a `POST` to the `/v2.1/accounts/{accountId}/users/{userId}/cloud_storage` endpoint.
     *
     * Configures the redirect URL information  for one or more cloud storage providers for the specified user. The redirect URL is added to the authentication URL to complete the return route.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     */
    pub async fn cloud_storage_post(
        &self,
        account_id: &str,
        user_id: &str,
        body: &crate::types::CloudStorageProvidersData,
    ) -> ClientResult<crate::types::CloudStorageProvidersData> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/cloud_storage",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(user_id),
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
     * Deletes the user authentication information for one or more cloud storage providers.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/users/{userId}/cloud_storage` endpoint.
     *
     * Deletes the user authentication information for one or more cloud storage providers. The next time the user tries to access the cloud storage provider, they must pass normal authentication.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     */
    pub async fn cloud_storage_delete_providers(
        &self,
        account_id: &str,
        user_id: &str,
        body: &crate::types::CloudStorageProvidersData,
    ) -> ClientResult<crate::types::CloudStorageProvidersData> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/cloud_storage",
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
    /**
     * Gets the specified Cloud Storage Provider configuration for the User.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/users/{userId}/cloud_storage/{serviceId}` endpoint.
     *
     * Retrieves the list of cloud storage providers enabled for the account and the configuration information for the user.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `service_id: &str` -- The ID of the service to access.
     *   
     *   Valid values are the service name ("Box") or the numerical serviceId ("4136").
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     * * `redirect_url: &str` --  The URL the user is redirected to after the cloud storage provider authenticates the user. Using this will append the redirectUrl to the authenticationUrl.
     *   
     *   The redirectUrl is restricted to URLs in the docusign.com or docusign.net domains.
     *    .
     */
    pub async fn cloud_storage_get(
        &self,
        account_id: &str,
        service_id: &str,
        user_id: &str,
        redirect_url: &str,
    ) -> ClientResult<crate::types::CloudStorageProvidersData> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !redirect_url.is_empty() {
            query_args.push(("redirectUrl".to_string(), redirect_url.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/cloud_storage/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(service_id),
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
     * Deletes the user authentication information for the specified cloud storage provider.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/users/{userId}/cloud_storage/{serviceId}` endpoint.
     *
     * Deletes the user authentication information for the specified cloud storage provider. The next time the user tries to access the cloud storage provider, they must pass normal authentication for this cloud storage provider.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `service_id: &str` -- The ID of the service to access.
     *   
     *   Valid values are the service name ("Box") or the numerical serviceId ("4136").
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     */
    pub async fn cloud_storage_delete(
        &self,
        account_id: &str,
        service_id: &str,
        user_id: &str,
    ) -> ClientResult<crate::types::CloudStorageProvidersData> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/cloud_storage/{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(service_id),
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
