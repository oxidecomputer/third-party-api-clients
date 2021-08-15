use anyhow::Result;

use crate::Client;

pub struct TwoStepVerification {
    client: Client,
}

impl TwoStepVerification {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        TwoStepVerification {
            client,
        }
    }

    /**
* This function performs a `POST` to the `/admin/directory/v1/users/{userKey}/twoStepVerification/turnOff` endpoint.
*
* Turns off 2-Step Verification for user.
*
* **Parameters:**
*
* * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
*/
pub async fn directory_turn_off(
&self,
xgafv: crate::types::Xgafv, access_token: &str, alt: crate::types::Alt, callback: &str, fields: &str, key: &str, oauth_token: &str, pretty_print: bool, quota_user: &str, upload_protocol: &str, upload_type: &str, user_key: &str,
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
format!("/admin/directory/v1/users/{}/twoStepVerification/turnOff?{}",
crate::progenitor_support::encode_path(&user_key.to_string()),query);

self.client.post(&url, None).await
}


}