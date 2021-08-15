use anyhow::Result;

use crate::Client;

pub struct DomainAliases {
    client: Client,
}

impl DomainAliases {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        DomainAliases {
            client,
        }
    }

    /**
* This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/domainaliases` endpoint.
*
* Lists the domain aliases of the customer.
*
* **Parameters:**
*
* * `customer: &str` -- Immutable ID of the Google Workspace account.
* * `parent_domain_name: &str` -- Name of the parent domain for which domain aliases are to be fetched.
*/
pub async fn directory_list(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, customer: &str, parent_domain_name: &str,
) -> Result<crate::types::DomainAliases> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
if !access_token.is_empty() { query_args.push(format!("access_token={}", access_token)); }
query_args.push(format!("alt={}", alt));
if !callback.is_empty() { query_args.push(format!("callback={}", callback)); }
if !fields.is_empty() { query_args.push(format!("fields={}", fields)); }
if !key.is_empty() { query_args.push(format!("key={}", key)); }
if !oauth_token.is_empty() { query_args.push(format!("oauth_token={}", oauth_token)); }
if !parent_domain_name.is_empty() { query_args.push(format!("parent_domain_name={}", parent_domain_name)); }
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
format!("/admin/directory/v1/customer/{}/domainaliases?{}",
crate::progenitor_support::encode_path(&customer.to_string()),query);

self.client.get(&url, None).await
}

/**
* This function performs a `POST` to the `/admin/directory/v1/customer/{customer}/domainaliases` endpoint.
*
* Inserts a domain alias of the customer.
*
* **Parameters:**
*
* * `customer: &str` -- Immutable ID of the Google Workspace account.
*/
pub async fn directory_insert(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, customer: &str,
body: &crate::types::DomainAlias
) -> Result<crate::types::DomainAlias> {
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
format!("/admin/directory/v1/customer/{}/domainaliases?{}",
crate::progenitor_support::encode_path(&customer.to_string()),query);

self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* This function performs a `GET` to the `/admin/directory/v1/customer/{customer}/domainaliases/{domainAliasName}` endpoint.
*
* Retrieves a domain alias of the customer.
*
* **Parameters:**
*
* * `customer: &str` -- Immutable ID of the Google Workspace account.
* * `domain_alias_name: &str` -- Name of domain alias to be retrieved.
*/
pub async fn directory_get(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, customer: &str, domain_alias_name: &str,
) -> Result<crate::types::DomainAlias> {
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
format!("/admin/directory/v1/customer/{}/domainaliases/{}?{}",
crate::progenitor_support::encode_path(&customer.to_string()),crate::progenitor_support::encode_path(&domain_alias_name.to_string()),query);

self.client.get(&url, None).await
}

/**
* This function performs a `DELETE` to the `/admin/directory/v1/customer/{customer}/domainaliases/{domainAliasName}` endpoint.
*
* Deletes a domain Alias of the customer.
*
* **Parameters:**
*
* * `customer: &str` -- Immutable ID of the Google Workspace account.
* * `domain_alias_name: &str` -- Name of domain alias to be retrieved.
*/
pub async fn directory_delete(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, customer: &str, domain_alias_name: &str,
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
format!("/admin/directory/v1/customer/{}/domainaliases/{}?{}",
crate::progenitor_support::encode_path(&customer.to_string()),crate::progenitor_support::encode_path(&domain_alias_name.to_string()),query);

self.client.delete(&url, None).await
}


}