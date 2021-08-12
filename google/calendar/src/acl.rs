use anyhow::Result;

use crate::Client;

pub struct Acl {
    client: Client,
}

impl Acl {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        Acl {
            client,
        }
    }

    /**
* This function performs a `GET` to the `/calendars/{calendarId}/acl` endpoint.
*
* Returns the rules in the access control list for the calendar.
*
* **Parameters:**
*
* * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
* * `max_results: i64` -- Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional.
* * `page_token: &str` -- Token specifying which result page to return. Optional.
* * `show_deleted: bool` -- Whether to include deleted ACLs in the result. Deleted ACLs are represented by role equal to "none". Deleted ACLs will always be included if syncToken is provided. Optional. The default is False.
* * `sync_token: &str` -- Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. All entries deleted since the previous list request will always be in the result set and it is not allowed to set showDeleted to False.
*  If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.
*  Learn more about incremental synchronization.
*  Optional. The default is to return all entries.
*/
pub async fn calendar_list(
&self,
alt: crate::types::Alt, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, user_ip: &str, calendar_id: &str, max_results: i64, page_token: &str, show_deleted: bool, sync_token: &str,
) -> Result<crate::types::Acl> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
query_args.push(format!("alt={}", alt));
if !fields.is_empty() { query_args.push(format!("fields={}", fields)); }
if !key.is_empty() { query_args.push(format!("key={}", key)); }
if max_results > 0 { query_args.push(format!("max_results={}", max_results)); }
if !oauth_token.is_empty() { query_args.push(format!("oauth_token={}", oauth_token)); }
if !page_token.is_empty() { query_args.push(format!("page_token={}", page_token)); }
if pretty_print { query_args.push(format!("pretty_print={}", pretty_print)); }
if !quota_user.is_empty() { query_args.push(format!("quota_user={}", quota_user)); }
if show_deleted { query_args.push(format!("show_deleted={}", show_deleted)); }
if !sync_token.is_empty() { query_args.push(format!("sync_token={}", sync_token)); }
if !user_ip.is_empty() { query_args.push(format!("user_ip={}", user_ip)); }
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/calendars/{}/acl?{}",
crate::progenitor_support::encode_path(&calendar_id.to_string()),query);

self.client.get(&url, None).await
}

/**
* This function performs a `POST` to the `/calendars/{calendarId}/acl` endpoint.
*
* Creates an access control rule.
*
* **Parameters:**
*
* * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
* * `send_notifications: bool` -- Whether to send notifications about the calendar sharing change. Optional. The default is True.
*/
pub async fn calendar_insert(
&self,
alt: crate::types::Alt, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, user_ip: &str, calendar_id: &str, send_notifications: bool,
body: &crate::types::AclRule
) -> Result<crate::types::AclRule> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
query_args.push(format!("alt={}", alt));
if !fields.is_empty() { query_args.push(format!("fields={}", fields)); }
if !key.is_empty() { query_args.push(format!("key={}", key)); }
if !oauth_token.is_empty() { query_args.push(format!("oauth_token={}", oauth_token)); }
if pretty_print { query_args.push(format!("pretty_print={}", pretty_print)); }
if !quota_user.is_empty() { query_args.push(format!("quota_user={}", quota_user)); }
if send_notifications { query_args.push(format!("send_notifications={}", send_notifications)); }
if !user_ip.is_empty() { query_args.push(format!("user_ip={}", user_ip)); }
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/calendars/{}/acl?{}",
crate::progenitor_support::encode_path(&calendar_id.to_string()),query);

self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* This function performs a `POST` to the `/calendars/{calendarId}/acl/watch` endpoint.
*
* Watch for changes to ACL resources.
*
* **Parameters:**
*
* * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
* * `max_results: i64` -- Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional.
* * `page_token: &str` -- Token specifying which result page to return. Optional.
* * `show_deleted: bool` -- Whether to include deleted ACLs in the result. Deleted ACLs are represented by role equal to "none". Deleted ACLs will always be included if syncToken is provided. Optional. The default is False.
* * `sync_token: &str` -- Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. All entries deleted since the previous list request will always be in the result set and it is not allowed to set showDeleted to False.
*  If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.
*  Learn more about incremental synchronization.
*  Optional. The default is to return all entries.
*/
pub async fn calendar_watch(
&self,
alt: crate::types::Alt, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, user_ip: &str, calendar_id: &str, max_results: i64, page_token: &str, show_deleted: bool, sync_token: &str,
body: &crate::types::Channel
) -> Result<crate::types::Channel> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
query_args.push(format!("alt={}", alt));
if !fields.is_empty() { query_args.push(format!("fields={}", fields)); }
if !key.is_empty() { query_args.push(format!("key={}", key)); }
if max_results > 0 { query_args.push(format!("max_results={}", max_results)); }
if !oauth_token.is_empty() { query_args.push(format!("oauth_token={}", oauth_token)); }
if !page_token.is_empty() { query_args.push(format!("page_token={}", page_token)); }
if pretty_print { query_args.push(format!("pretty_print={}", pretty_print)); }
if !quota_user.is_empty() { query_args.push(format!("quota_user={}", quota_user)); }
if show_deleted { query_args.push(format!("show_deleted={}", show_deleted)); }
if !sync_token.is_empty() { query_args.push(format!("sync_token={}", sync_token)); }
if !user_ip.is_empty() { query_args.push(format!("user_ip={}", user_ip)); }
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/calendars/{}/acl/watch?{}",
crate::progenitor_support::encode_path(&calendar_id.to_string()),query);

self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* This function performs a `GET` to the `/calendars/{calendarId}/acl/{ruleId}` endpoint.
*
* Returns an access control rule.
*
* **Parameters:**
*
* * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
* * `rule_id: &str` -- ACL rule identifier.
*/
pub async fn calendar_get(
&self,
alt: crate::types::Alt, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, user_ip: &str, calendar_id: &str, rule_id: &str,
) -> Result<crate::types::AclRule> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
query_args.push(format!("alt={}", alt));
if !fields.is_empty() { query_args.push(format!("fields={}", fields)); }
if !key.is_empty() { query_args.push(format!("key={}", key)); }
if !oauth_token.is_empty() { query_args.push(format!("oauth_token={}", oauth_token)); }
if pretty_print { query_args.push(format!("pretty_print={}", pretty_print)); }
if !quota_user.is_empty() { query_args.push(format!("quota_user={}", quota_user)); }
if !user_ip.is_empty() { query_args.push(format!("user_ip={}", user_ip)); }
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/calendars/{}/acl/{}?{}",
crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&rule_id.to_string()),query);

self.client.get(&url, None).await
}

/**
* This function performs a `PUT` to the `/calendars/{calendarId}/acl/{ruleId}` endpoint.
*
* Updates an access control rule.
*
* **Parameters:**
*
* * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
* * `rule_id: &str` -- ACL rule identifier.
* * `send_notifications: bool` -- Whether to send notifications about the calendar sharing change. Note that there are no notifications on access removal. Optional. The default is True.
*/
pub async fn calendar_update(
&self,
alt: crate::types::Alt, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, user_ip: &str, calendar_id: &str, rule_id: &str, send_notifications: bool,
body: &crate::types::AclRule
) -> Result<crate::types::AclRule> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
query_args.push(format!("alt={}", alt));
if !fields.is_empty() { query_args.push(format!("fields={}", fields)); }
if !key.is_empty() { query_args.push(format!("key={}", key)); }
if !oauth_token.is_empty() { query_args.push(format!("oauth_token={}", oauth_token)); }
if pretty_print { query_args.push(format!("pretty_print={}", pretty_print)); }
if !quota_user.is_empty() { query_args.push(format!("quota_user={}", quota_user)); }
if send_notifications { query_args.push(format!("send_notifications={}", send_notifications)); }
if !user_ip.is_empty() { query_args.push(format!("user_ip={}", user_ip)); }
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/calendars/{}/acl/{}?{}",
crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&rule_id.to_string()),query);

self.client.put(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* This function performs a `DELETE` to the `/calendars/{calendarId}/acl/{ruleId}` endpoint.
*
* Deletes an access control rule.
*
* **Parameters:**
*
* * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
* * `rule_id: &str` -- ACL rule identifier.
*/
pub async fn calendar_delete(
&self,
alt: crate::types::Alt, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, user_ip: &str, calendar_id: &str, rule_id: &str,
) -> Result<()> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
query_args.push(format!("alt={}", alt));
if !fields.is_empty() { query_args.push(format!("fields={}", fields)); }
if !key.is_empty() { query_args.push(format!("key={}", key)); }
if !oauth_token.is_empty() { query_args.push(format!("oauth_token={}", oauth_token)); }
if pretty_print { query_args.push(format!("pretty_print={}", pretty_print)); }
if !quota_user.is_empty() { query_args.push(format!("quota_user={}", quota_user)); }
if !user_ip.is_empty() { query_args.push(format!("user_ip={}", user_ip)); }
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/calendars/{}/acl/{}?{}",
crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&rule_id.to_string()),query);

self.client.delete(&url, None).await
}

/**
* This function performs a `PATCH` to the `/calendars/{calendarId}/acl/{ruleId}` endpoint.
*
* Updates an access control rule. This method supports patch semantics.
*
* **Parameters:**
*
* * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
* * `rule_id: &str` -- ACL rule identifier.
* * `send_notifications: bool` -- Whether to send notifications about the calendar sharing change. Note that there are no notifications on access removal. Optional. The default is True.
*/
pub async fn calendar_patch(
&self,
alt: crate::types::Alt, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, user_ip: &str, calendar_id: &str, rule_id: &str, send_notifications: bool,
body: &crate::types::AclRule
) -> Result<crate::types::AclRule> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
query_args.push(format!("alt={}", alt));
if !fields.is_empty() { query_args.push(format!("fields={}", fields)); }
if !key.is_empty() { query_args.push(format!("key={}", key)); }
if !oauth_token.is_empty() { query_args.push(format!("oauth_token={}", oauth_token)); }
if pretty_print { query_args.push(format!("pretty_print={}", pretty_print)); }
if !quota_user.is_empty() { query_args.push(format!("quota_user={}", quota_user)); }
if send_notifications { query_args.push(format!("send_notifications={}", send_notifications)); }
if !user_ip.is_empty() { query_args.push(format!("user_ip={}", user_ip)); }
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/calendars/{}/acl/{}?{}",
crate::progenitor_support::encode_path(&calendar_id.to_string()),crate::progenitor_support::encode_path(&rule_id.to_string()),query);

self.client.patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}


}