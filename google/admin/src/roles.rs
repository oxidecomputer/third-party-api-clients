use anyhow::Result;

use crate::Client;

pub struct Roles {
    client: Client,
}

impl Roles {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        Roles {
            client,
        }
    }

    /**
* This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/roles` endpoint.
*
* Retrieves a paginated list of all the roles in a domain.
*
* **Parameters:**
*
* * `customer: &str` -- Immutable ID of the Google Workspace account.
* * `max_results: i64` -- Maximum number of results to return.
* * `page_token: &str` -- Token to specify the next page in the list.
*/
pub async fn directory_list(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, customer: &str, max_results: i64, page_token: &str,
) -> Result<crate::types::Roles> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
if !access_token.is_empty() { query_args.push(format!("access_token={}", access_token)); }
query_args.push(format!("alt={}", alt));
if !callback.is_empty() { query_args.push(format!("callback={}", callback)); }
if !fields.is_empty() { query_args.push(format!("fields={}", fields)); }
if !key.is_empty() { query_args.push(format!("key={}", key)); }
if max_results > 0 { query_args.push(format!("max_results={}", max_results)); }
if !oauth_token.is_empty() { query_args.push(format!("oauth_token={}", oauth_token)); }
if !page_token.is_empty() { query_args.push(format!("page_token={}", page_token)); }
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
format!("/admin/directory/v1/customer/{}/roles?{}",
crate::progenitor_support::encode_path(&customer.to_string()),query);

self.client.get(&url, None).await
}

/**
* This function performs a `POST` to the `/admin/directory/v1/customer/{customer}/roles` endpoint.
*
* Creates a role.
*
* **Parameters:**
*
* * `customer: &str` -- Immutable ID of the Google Workspace account.
*/
pub async fn directory_insert(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, customer: &str,
body: &crate::types::Role
) -> Result<crate::types::Role> {
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
format!("/admin/directory/v1/customer/{}/roles?{}",
crate::progenitor_support::encode_path(&customer.to_string()),query);

self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/roles/{roleId}` endpoint.
*
* Retrieves a role.
*
* **Parameters:**
*
* * `customer: &str` -- Immutable ID of the Google Workspace account.
* * `role_id: &str` -- Immutable ID of the role.
*/
pub async fn directory_get(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, customer: &str, role_id: &str,
) -> Result<crate::types::Role> {
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
format!("/admin/directory/v1/customer/{}/roles/{}?{}",
crate::progenitor_support::encode_path(&customer.to_string()),crate::progenitor_support::encode_path(&role_id.to_string()),query);

self.client.get(&url, None).await
}

/**
* This function performs a `PUT` to the `/admin/directory/v1/customer/{customer}/roles/{roleId}` endpoint.
*
* Updates a role.
*
* **Parameters:**
*
* * `customer: &str` -- Immutable ID of the Google Workspace account.
* * `role_id: &str` -- Immutable ID of the role.
*/
pub async fn directory_update(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, customer: &str, role_id: &str,
body: &crate::types::Role
) -> Result<crate::types::Role> {
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
format!("/admin/directory/v1/customer/{}/roles/{}?{}",
crate::progenitor_support::encode_path(&customer.to_string()),crate::progenitor_support::encode_path(&role_id.to_string()),query);

self.client.put(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* This function performs a `DELETE` to the `/admin/directory/v1/customer/{customer}/roles/{roleId}` endpoint.
*
* Deletes a role.
*
* **Parameters:**
*
* * `customer: &str` -- Immutable ID of the Google Workspace account.
* * `role_id: &str` -- Immutable ID of the role.
*/
pub async fn directory_delete(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, customer: &str, role_id: &str,
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
format!("/admin/directory/v1/customer/{}/roles/{}?{}",
crate::progenitor_support::encode_path(&customer.to_string()),crate::progenitor_support::encode_path(&role_id.to_string()),query);

self.client.delete(&url, None).await
}

/**
* This function performs a `PATCH` to the `/admin/directory/v1/customer/{customer}/roles/{roleId}` endpoint.
*
* Patches a role.
*
* **Parameters:**
*
* * `customer: &str` -- Immutable ID of the Google Workspace account.
* * `role_id: &str` -- Immutable ID of the role.
*/
pub async fn directory_patch(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, customer: &str, role_id: &str,
body: &crate::types::Role
) -> Result<crate::types::Role> {
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
format!("/admin/directory/v1/customer/{}/roles/{}?{}",
crate::progenitor_support::encode_path(&customer.to_string()),crate::progenitor_support::encode_path(&role_id.to_string()),query);

self.client.patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}


}