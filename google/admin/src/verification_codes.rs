use anyhow::Result;

use crate::Client;

pub struct VerificationCodes {
    client: Client,
}

impl VerificationCodes {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        VerificationCodes { client }
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/users/{userKey}/verificationCodes` endpoint.
     *
     * Returns the current set of valid backup verification codes for the specified user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn directory_list(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        user_key: &str,
    ) -> Result<crate::types::VerificationCodes> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/users/{}/verificationCodes?{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/admin/directory/v1/users/{userKey}/verificationCodes/generate` endpoint.
     *
     * Generates new backup verification codes for the user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Email or immutable ID of the user.
     */
    pub async fn directory_generate(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        user_key: &str,
    ) -> Result<()> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/users/{}/verificationCodes/generate?{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
            query_
        );

        self.client.post(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/admin/directory/v1/users/{userKey}/verificationCodes/invalidate` endpoint.
     *
     * Invalidates the current backup verification codes for the user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Email or immutable ID of the user.
     */
    pub async fn directory_invalidate(
        &self,
        xgafv: crate::types::Xgafv,
        access_token: &str,
        alt: crate::types::Alt,
        callback: &str,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        upload_protocol: &str,
        upload_type: &str,
        user_key: &str,
    ) -> Result<()> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if !access_token.is_empty() {
            query_args.push(format!("access_token={}", access_token));
        }
        query_args.push(format!("alt={}", alt));
        if !callback.is_empty() {
            query_args.push(format!("callback={}", callback));
        }
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !upload_protocol.is_empty() {
            query_args.push(format!("upload_protocol={}", upload_protocol));
        }
        if !upload_type.is_empty() {
            query_args.push(format!("upload_type={}", upload_type));
        }
        query_args.push(format!("xgafv={}", xgafv));
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/admin/directory/v1/users/{}/verificationCodes/invalidate?{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
            query_
        );

        self.client.post(&url, None).await
    }
}
