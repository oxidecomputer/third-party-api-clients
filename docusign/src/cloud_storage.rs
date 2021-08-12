use anyhow::Result;

use crate::Client;

pub struct CloudStorage {
    client: Client,
}

impl CloudStorage {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        CloudStorage {
            client,
        }
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
account_id: &str, service_id: &str, user_id: &str, cloud_storage_folder_path: &str, count: &str, order: &str, order_by: &str, search_text: &str, start_position: &str,
) -> Result<crate::types::ExternalFolder> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
if !cloud_storage_folder_path.is_empty() { query_args.push(format!("cloud_storage_folder_path={}", cloud_storage_folder_path)); }
if !count.is_empty() { query_args.push(format!("count={}", count)); }
if !order.is_empty() { query_args.push(format!("order={}", order)); }
if !order_by.is_empty() { query_args.push(format!("order_by={}", order_by)); }
if !search_text.is_empty() { query_args.push(format!("search_text={}", search_text)); }
if !start_position.is_empty() { query_args.push(format!("start_position={}", start_position)); }
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/v2.1/accounts/{}/users/{}/cloud_storage/{}/folders?{}",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&service_id.to_string()),query);

self.client.get(&url, None).await
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
account_id: &str, folder_id: &str, service_id: &str, user_id: &str, cloud_storage_folder_path: &str, cloud_storage_folderid_plain: &str, count: &str, order: &str, order_by: &str, search_text: &str, start_position: &str,
) -> Result<crate::types::ExternalFolder> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
if !cloud_storage_folder_path.is_empty() { query_args.push(format!("cloud_storage_folder_path={}", cloud_storage_folder_path)); }
if !cloud_storage_folderid_plain.is_empty() { query_args.push(format!("cloud_storage_folderid_plain={}", cloud_storage_folderid_plain)); }
if !count.is_empty() { query_args.push(format!("count={}", count)); }
if !order.is_empty() { query_args.push(format!("order={}", order)); }
if !order_by.is_empty() { query_args.push(format!("order_by={}", order_by)); }
if !search_text.is_empty() { query_args.push(format!("search_text={}", search_text)); }
if !start_position.is_empty() { query_args.push(format!("start_position={}", start_position)); }
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/v2.1/accounts/{}/users/{}/cloud_storage/{}/folders/{}?{}",
crate::progenitor_support::encode_path(&account_id.to_string()),crate::progenitor_support::encode_path(&user_id.to_string()),crate::progenitor_support::encode_path(&service_id.to_string()),crate::progenitor_support::encode_path(&folder_id.to_string()),query);

self.client.get(&url, None).await
}


}