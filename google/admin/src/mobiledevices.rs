use anyhow::Result;

use crate::Client;

pub struct Mobiledevices {
    client: Client,
}

impl Mobiledevices {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        Mobiledevices {
            client,
        }
    }

    /**
* This function performs a `GET` to the `/admin/directory/v1/customer/{customerId}/devices/mobile` endpoint.
*
* Retrieves a paginated list of all mobile devices for an account.
*
* **Parameters:**
*
* * `customer_id: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users).
* * `max_results: i64` -- Maximum number of results to return. Max allowed value is 100.
* * `order_by: crate::types::DirectoryMobiledevicesListOrderBy` -- Device property to use for sorting results.
* * `page_token: &str` -- Token to specify next page in the list.
* * `projection: crate::types::Projection` -- Restrict information returned to a set of selected fields.
* * `query: &str` -- Search string in the format given at https://developers.google.com/admin-sdk/directory/v1/search-operators.
* * `sort_order: crate::types::SortOrder` -- Whether to return results in ascending or descending order. Must be used with the `orderBy` parameter.
*/
pub async fn directory_list(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, customer_id: &str, max_results: i64, order_by: crate::types::DirectoryMobiledevicesListOrderBy, page_token: &str, projection: crate::types::Projection, query: &str, sort_order: crate::types::SortOrder,
) -> Result<crate::types::MobileDevices> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
if !access_token.is_empty() { query_args.push(format!("access_token={}", access_token)); }
query_args.push(format!("alt={}", alt));
if !callback.is_empty() { query_args.push(format!("callback={}", callback)); }
if !fields.is_empty() { query_args.push(format!("fields={}", fields)); }
if !key.is_empty() { query_args.push(format!("key={}", key)); }
if max_results > 0 { query_args.push(format!("max_results={}", max_results)); }
if !oauth_token.is_empty() { query_args.push(format!("oauth_token={}", oauth_token)); }
query_args.push(format!("order_by={}", order_by));
if !page_token.is_empty() { query_args.push(format!("page_token={}", page_token)); }
if pretty_print { query_args.push(format!("pretty_print={}", pretty_print)); }
query_args.push(format!("projection={}", projection));
if !query.is_empty() { query_args.push(format!("query={}", query)); }
if !quota_user.is_empty() { query_args.push(format!("quota_user={}", quota_user)); }
query_args.push(format!("sort_order={}", sort_order));
if !upload_protocol.is_empty() { query_args.push(format!("upload_protocol={}", upload_protocol)); }
if !upload_type.is_empty() { query_args.push(format!("upload_type={}", upload_type)); }
query_args.push(format!("xgafv={}", xgafv));
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/admin/directory/v1/customer/{}/devices/mobile?{}",
crate::progenitor_support::encode_path(&customer_id.to_string()),query);

self.client.get(&url, None).await
}

/**
* This function performs a `GET` to the `/admin/directory/v1/customer/{customerId}/devices/mobile/{resourceId}` endpoint.
*
* Retrieves a mobile device's properties.
*
* **Parameters:**
*
* * `customer_id: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users).
* * `resource_id: &str` -- The unique ID the API service uses to identify the mobile device.
* * `projection: crate::types::Projection` -- Restrict information returned to a set of selected fields.
*/
pub async fn directory_get(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, customer_id: &str, resource_id: &str, projection: crate::types::Projection,
) -> Result<crate::types::MobileDevice> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
if !access_token.is_empty() { query_args.push(format!("access_token={}", access_token)); }
query_args.push(format!("alt={}", alt));
if !callback.is_empty() { query_args.push(format!("callback={}", callback)); }
if !fields.is_empty() { query_args.push(format!("fields={}", fields)); }
if !key.is_empty() { query_args.push(format!("key={}", key)); }
if !oauth_token.is_empty() { query_args.push(format!("oauth_token={}", oauth_token)); }
if pretty_print { query_args.push(format!("pretty_print={}", pretty_print)); }
query_args.push(format!("projection={}", projection));
if !quota_user.is_empty() { query_args.push(format!("quota_user={}", quota_user)); }
if !upload_protocol.is_empty() { query_args.push(format!("upload_protocol={}", upload_protocol)); }
if !upload_type.is_empty() { query_args.push(format!("upload_type={}", upload_type)); }
query_args.push(format!("xgafv={}", xgafv));
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/admin/directory/v1/customer/{}/devices/mobile/{}?{}",
crate::progenitor_support::encode_path(&customer_id.to_string()),crate::progenitor_support::encode_path(&resource_id.to_string()),query);

self.client.get(&url, None).await
}

/**
* This function performs a `DELETE` to the `/admin/directory/v1/customer/{customerId}/devices/mobile/{resourceId}` endpoint.
*
* Removes a mobile device.
*
* **Parameters:**
*
* * `customer_id: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users).
* * `resource_id: &str` -- The unique ID the API service uses to identify the mobile device.
*/
pub async fn directory_delete(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, customer_id: &str, resource_id: &str,
) -> Result<()> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
if !access_token.is_empty() { query_args.push(format!("access_token={}", access_token)); }
query_args.push(format!("alt={}", alt));
if !callback.is_empty() { query_args.push(format!("callback={}", callback)); }
if !fields.is_empty() { query_args.push(format!("fields={}", fields)); }
if !key.is_empty() { query_args.push(format!("key={}", key)); }
if !oauth_token.is_empty() { query_args.push(format!("oauth_token={}", oauth_token)); }
if pretty_print { query_args.push(format!("pretty_print={}", pretty_print)); }
if !quota_user.is_empty() { query_args.push(format!("quota_user={}", quota_user)); }
if !upload_protocol.is_empty() { query_args.push(format!("upload_protocol={}", upload_protocol)); }
if !upload_type.is_empty() { query_args.push(format!("upload_type={}", upload_type)); }
query_args.push(format!("xgafv={}", xgafv));
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/admin/directory/v1/customer/{}/devices/mobile/{}?{}",
crate::progenitor_support::encode_path(&customer_id.to_string()),crate::progenitor_support::encode_path(&resource_id.to_string()),query);

self.client.delete(&url, None).await
}

/**
* This function performs a `POST` to the `/admin/directory/v1/customer/{customerId}/devices/mobile/{resourceId}/action` endpoint.
*
* Takes an action that affects a mobile device. For example, remotely wiping a device.
*
* **Parameters:**
*
* * `customer_id: &str` -- The unique ID for the customer's Google Workspace account. As an account administrator, you can also use the `my_customer` alias to represent your account's `customerId`. The `customerId` is also returned as part of the [Users resource](/admin-sdk/directory/v1/reference/users).
* * `resource_id: &str` -- The unique ID the API service uses to identify the mobile device.
*/
pub async fn directory_action(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, customer_id: &str, resource_id: &str,
body: &crate::types::MobileDeviceAction
) -> Result<()> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
if !access_token.is_empty() { query_args.push(format!("access_token={}", access_token)); }
query_args.push(format!("alt={}", alt));
if !callback.is_empty() { query_args.push(format!("callback={}", callback)); }
if !fields.is_empty() { query_args.push(format!("fields={}", fields)); }
if !key.is_empty() { query_args.push(format!("key={}", key)); }
if !oauth_token.is_empty() { query_args.push(format!("oauth_token={}", oauth_token)); }
if pretty_print { query_args.push(format!("pretty_print={}", pretty_print)); }
if !quota_user.is_empty() { query_args.push(format!("quota_user={}", quota_user)); }
if !upload_protocol.is_empty() { query_args.push(format!("upload_protocol={}", upload_protocol)); }
if !upload_type.is_empty() { query_args.push(format!("upload_type={}", upload_type)); }
query_args.push(format!("xgafv={}", xgafv));
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/admin/directory/v1/customer/{}/devices/mobile/{}/action?{}",
crate::progenitor_support::encode_path(&customer_id.to_string()),crate::progenitor_support::encode_path(&resource_id.to_string()),query);

self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}


}