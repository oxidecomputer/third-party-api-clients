use crate::Client;
use crate::ClientResult;

pub struct CloudStorage {
    pub client: Client,
}

impl CloudStorage {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CloudStorage { client }
    }

    /**
     * Retrieves a list of all the items in a specified folder from the specified cloud storage provider.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/users/{userId}/cloud_storage/{serviceId}/folders` endpoint.
     *
     * Retrieves a list of all the items in a specified folder from the specified cloud storage provider.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `service_id: &str` -- The ID of the service to access.
     *   
     *   Valid values are the service name ("Box") or the numerical serviceId ("4136").
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     * * `cloud_storage_folder_path: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `count: &str` -- An optional value that sets how many items are included in the response.
     *   
     *   The default setting for this is 25.
     * * `order: &str` -- (Optional) The order in which to sort the results.
     *   
     *   Valid values are:
     *   
     *   
     *   * `asc`: Ascending order.
     *   * `desc`: Descending order.
     * * `order_by: &str` -- (Optional) The file attribute to use to sort the results.
     *   
     *   Valid values are:
     *   
     *   * `modified`
     *   * `name`.
     * * `search_text: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `start_position: &str` -- Indicates the starting point of the first item included in the response set. It uses a 0-based index. The default setting for this is 0.  .
     */
    pub async fn folder_get_all(
        &self,
        account_id: &str,
        service_id: &str,
        user_id: &str,
        cloud_storage_folder_path: &str,
        count: &str,
        order: &str,
        order_by: &str,
        search_text: &str,
        start_position: &str,
    ) -> ClientResult<crate::types::ExternalFolder> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cloud_storage_folder_path.is_empty() {
            query_args.push((
                "cloud_storage_folder_path".to_string(),
                cloud_storage_folder_path.to_string(),
            ));
        }
        if !count.is_empty() {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !order_by.is_empty() {
            query_args.push(("order_by".to_string(), order_by.to_string()));
        }
        if !search_text.is_empty() {
            query_args.push(("search_text".to_string(), search_text.to_string()));
        }
        if !start_position.is_empty() {
            query_args.push(("start_position".to_string(), start_position.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/cloud_storage/{}/folders?{}",
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
     * Gets a list of items from a cloud storage provider.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/users/{userId}/cloud_storage/{serviceId}/folders/{folderId}` endpoint.
     *
     * Retrieves a list of the user's items from the specified cloud storage provider.
     *
     * To limit the scope of the items returned, provide a comma-separated list of folder ids in the request.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `folder_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `service_id: &str` -- The ID of the service to access.
     *   
     *   Valid values are the service name ("Box") or the numerical serviceId ("4136").
     * * `user_id: &str` -- The ID of the user to access. Generally this is the ID of the current authenticated user, but if the authenticated user is an Administrator on the account, `userId` can represent another user whom the Administrator is accessing.
     *   .
     * * `cloud_storage_folder_path: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `cloud_storage_folderid_plain: &str` -- A plain-text folder id that you can use as an alternative to the existing folder id. This property is mainly used for rooms. Enter multiple folder ids as a comma-separated list.
     * * `count: &str` -- An optional value that sets how many items are included in the response.
     *   
     *   The default setting for this is 25.
     * * `order: &str` -- (Optional) The order in which to sort the results.
     *   
     *   Valid values are:
     *   
     *   
     *   * `asc`: Ascending order.
     *   * `desc`: Descending order.
     * * `order_by: &str` -- (Optional) The file attribute to use to sort the results.
     *   
     *   Valid values are:
     *   
     *   * `modified`
     *   * `name`.
     * * `search_text: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     * * `start_position: &str` -- The starting index position in the result set from which to start returning values. The default setting is `0`.
     */
    pub async fn folder_get(
        &self,
        account_id: &str,
        folder_id: &str,
        service_id: &str,
        user_id: &str,
        cloud_storage_folder_path: &str,
        cloud_storage_folderid_plain: &str,
        count: &str,
        order: &str,
        order_by: &str,
        search_text: &str,
        start_position: &str,
    ) -> ClientResult<crate::types::ExternalFolder> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cloud_storage_folder_path.is_empty() {
            query_args.push((
                "cloud_storage_folder_path".to_string(),
                cloud_storage_folder_path.to_string(),
            ));
        }
        if !cloud_storage_folderid_plain.is_empty() {
            query_args.push((
                "cloud_storage_folderid_plain".to_string(),
                cloud_storage_folderid_plain.to_string(),
            ));
        }
        if !count.is_empty() {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !order.is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !order_by.is_empty() {
            query_args.push(("order_by".to_string(), order_by.to_string()));
        }
        if !search_text.is_empty() {
            query_args.push(("search_text".to_string(), search_text.to_string()));
        }
        if !start_position.is_empty() {
            query_args.push(("start_position".to_string(), start_position.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/users/{}/cloud_storage/{}/folders/{}?{}",
                crate::progenitor_support::encode_path(account_id),
                crate::progenitor_support::encode_path(user_id),
                crate::progenitor_support::encode_path(service_id),
                crate::progenitor_support::encode_path(folder_id),
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
}
