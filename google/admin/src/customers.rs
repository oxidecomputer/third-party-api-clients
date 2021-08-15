use anyhow::Result;

use crate::Client;

pub struct Customers {
    client: Client,
}

impl Customers {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        Customers {
            client,
        }
    }

    /**
* This function performs a `GET` to the `/admin/directory/v1/customers/{customerKey}` endpoint.
*
* Retrieves a customer.
*
* **Parameters:**
*
* * `customer_key: &str` -- Id of the customer to be retrieved.
*/
pub async fn directory_get(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, customer_key: &str,
) -> Result<crate::types::Customer> {
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
format!("/admin/directory/v1/customers/{}?{}",
crate::progenitor_support::encode_path(&customer_key.to_string()),query);

self.client.get(&url, None).await
}

/**
* This function performs a `PUT` to the `/admin/directory/v1/customers/{customerKey}` endpoint.
*
* Updates a customer.
*
* **Parameters:**
*
* * `customer_key: &str` -- Id of the customer to be updated.
*/
pub async fn directory_update(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, customer_key: &str,
body: &crate::types::Customer
) -> Result<crate::types::Customer> {
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
format!("/admin/directory/v1/customers/{}?{}",
crate::progenitor_support::encode_path(&customer_key.to_string()),query);

self.client.put(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* This function performs a `PATCH` to the `/admin/directory/v1/customers/{customerKey}` endpoint.
*
* Patches a customer.
*
* **Parameters:**
*
* * `customer_key: &str` -- Id of the customer to be updated.
*/
pub async fn directory_patch(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, customer_key: &str,
body: &crate::types::Customer
) -> Result<crate::types::Customer> {
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
format!("/admin/directory/v1/customers/{}?{}",
crate::progenitor_support::encode_path(&customer_key.to_string()),query);

self.client.patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* This function performs a `GET` to the `/admin/directory/v1/{name}` endpoint.
*
* Returns a `Printer` resource (printer's config).
*
* **Parameters:**
*
* * `name: &str` -- Required. The name of the printer to retrieve. Format: customers/{customer_id}/chrome/printers/{printer_id}.
*/
pub async fn admin_chrome_printers_get(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, name: &str,
) -> Result<crate::types::Printer> {
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
format!("/admin/directory/v1/{}?{}",
crate::progenitor_support::encode_path(&name.to_string()),query);

self.client.get(&url, None).await
}

/**
* This function performs a `DELETE` to the `/admin/directory/v1/{name}` endpoint.
*
* Deletes a `Printer`.
*
* **Parameters:**
*
* * `name: &str` -- Required. The name of the printer to be updated. Format: customers/{customer_id}/chrome/printers/{printer_id}.
*/
pub async fn admin_chrome_printers_delete(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, name: &str,
) -> Result<crate::types::Empty> {
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
format!("/admin/directory/v1/{}?{}",
crate::progenitor_support::encode_path(&name.to_string()),query);

self.client.delete(&url, None).await
}

/**
* This function performs a `PATCH` to the `/admin/directory/v1/{name}` endpoint.
*
* Updates a `Printer` resource.
*
* **Parameters:**
*
* * `name: &str` -- The resource name of the Printer object, in the format customers/{customer-id}/printers/{printer-id} (During printer creation leave empty).
* * `clear_mask: &str` -- The list of fields to be cleared. Note, some of the fields are read only and cannot be updated. Values for not specified fields will be patched.
* * `update_mask: &str` -- The list of fields to be updated. Note, some of the fields are read only and cannot be updated. Values for not specified fields will be patched.
*/
pub async fn admin_chrome_printers_patch(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, name: &str, clear_mask: &str, update_mask: &str,
body: &crate::types::Printer
) -> Result<crate::types::Printer> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
if !access_token.is_empty() { query_args.push(format!("access_token={}", access_token)); }
query_args.push(format!("alt={}", alt));
if !callback.is_empty() { query_args.push(format!("callback={}", callback)); }
if !clear_mask.is_empty() { query_args.push(format!("clear_mask={}", clear_mask)); }
if !fields.is_empty() { query_args.push(format!("fields={}", fields)); }
if !key.is_empty() { query_args.push(format!("key={}", key)); }
if !oauth_token.is_empty() { query_args.push(format!("oauth_token={}", oauth_token)); }
if pretty_print { query_args.push(format!("pretty_print={}", pretty_print)); }
if !quota_user.is_empty() { query_args.push(format!("quota_user={}", quota_user)); }
if !update_mask.is_empty() { query_args.push(format!("update_mask={}", update_mask)); }
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
format!("/admin/directory/v1/{}?{}",
crate::progenitor_support::encode_path(&name.to_string()),query);

self.client.patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* This function performs a `GET` to the `/admin/directory/v1/{parent}/chrome/printers` endpoint.
*
* List printers configs.
*
* **Parameters:**
*
* * `parent: &str` -- Required. The name of the customer who owns this collection of printers. Format: customers/{customer_id}.
* * `filter: &str` -- Search query. Search syntax is shared between this api and Admin Console printers pages.
* * `org_unit_id: &str` -- Organization Unit that we want to list the printers for. When org_unit is not present in the request then all printers of the customer are returned (or filtered). When org_unit is present in the request then only printers available to this OU will be returned (owned or inherited). You may see if printer is owned or inherited for this OU by looking at Printer.org_unit_id.
* * `page_size: i64` -- The maximum number of objects to return. The service may return fewer than this value.
* * `page_token: &str` -- A page token, received from a previous call.
*/
pub async fn admin_chrome_printers_list(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, parent: &str, filter: &str, org_unit_id: &str, page_size: i64, page_token: &str,
) -> Result<crate::types::ListPrintersResponse> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
if !access_token.is_empty() { query_args.push(format!("access_token={}", access_token)); }
query_args.push(format!("alt={}", alt));
if !callback.is_empty() { query_args.push(format!("callback={}", callback)); }
if !fields.is_empty() { query_args.push(format!("fields={}", fields)); }
if !filter.is_empty() { query_args.push(format!("filter={}", filter)); }
if !key.is_empty() { query_args.push(format!("key={}", key)); }
if !oauth_token.is_empty() { query_args.push(format!("oauth_token={}", oauth_token)); }
if !org_unit_id.is_empty() { query_args.push(format!("org_unit_id={}", org_unit_id)); }
if page_size > 0 { query_args.push(format!("page_size={}", page_size)); }
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
format!("/admin/directory/v1/{}/chrome/printers?{}",
crate::progenitor_support::encode_path(&parent.to_string()),query);

self.client.get(&url, None).await
}

/**
* This function performs a `POST` to the `/admin/directory/v1/{parent}/chrome/printers` endpoint.
*
* Creates a printer under given Organization Unit.
*
* **Parameters:**
*
* * `parent: &str` -- Required. The name of the customer. Format: customers/{customer_id}.
*/
pub async fn admin_chrome_printers_create(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, parent: &str,
body: &crate::types::Printer
) -> Result<crate::types::Printer> {
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
format!("/admin/directory/v1/{}/chrome/printers?{}",
crate::progenitor_support::encode_path(&parent.to_string()),query);

self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* This function performs a `POST` to the `/admin/directory/v1/{parent}/chrome/printers:batchCreatePrinters` endpoint.
*
* Creates printers under given Organization Unit.
*
* **Parameters:**
*
* * `parent: &str` -- Required. The name of the customer. Format: customers/{customer_id}.
*/
pub async fn admin_chrome_printers_batch_create(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, parent: &str,
body: &crate::types::BatchCreatePrintersRequest
) -> Result<crate::types::BatchCreatePrintersResponse> {
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
format!("/admin/directory/v1/{}/chrome/printers:batchCreatePrinters?{}",
crate::progenitor_support::encode_path(&parent.to_string()),query);

self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* This function performs a `POST` to the `/admin/directory/v1/{parent}/chrome/printers:batchDeletePrinters` endpoint.
*
* Deletes printers in batch.
*
* **Parameters:**
*
* * `parent: &str` -- Required. The name of the customer. Format: customers/{customer_id}.
*/
pub async fn admin_chrome_printers_batch_delete(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, parent: &str,
body: &crate::types::BatchDeletePrintersRequest
) -> Result<crate::types::BatchDeletePrintersResponse> {
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
format!("/admin/directory/v1/{}/chrome/printers:batchDeletePrinters?{}",
crate::progenitor_support::encode_path(&parent.to_string()),query);

self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* This function performs a `GET` to the `/admin/directory/v1/{parent}/chrome/printers:listPrinterModels` endpoint.
*
* Lists the supported printer models.
*
* **Parameters:**
*
* * `parent: &str` -- Required. The name of the customer who owns this collection of printers. Format: customers/{customer_id}.
* * `filter: &str` -- Filer to list only models by a given manufacturer in format: "manufacturer:Brother". Search syntax is shared between this api and Admin Console printers pages.
* * `page_size: i64` -- The maximum number of objects to return. The service may return fewer than this value.
* * `page_token: &str` -- A page token, received from a previous call.
*/
pub async fn admin_chrome_printers_list_printer_models(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, parent: &str, filter: &str, page_size: i64, page_token: &str,
) -> Result<crate::types::ListPrinterModelsResponse> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
if !access_token.is_empty() { query_args.push(format!("access_token={}", access_token)); }
query_args.push(format!("alt={}", alt));
if !callback.is_empty() { query_args.push(format!("callback={}", callback)); }
if !fields.is_empty() { query_args.push(format!("fields={}", fields)); }
if !filter.is_empty() { query_args.push(format!("filter={}", filter)); }
if !key.is_empty() { query_args.push(format!("key={}", key)); }
if !oauth_token.is_empty() { query_args.push(format!("oauth_token={}", oauth_token)); }
if page_size > 0 { query_args.push(format!("page_size={}", page_size)); }
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
format!("/admin/directory/v1/{}/chrome/printers:listPrinterModels?{}",
crate::progenitor_support::encode_path(&parent.to_string()),query);

self.client.get(&url, None).await
}


}