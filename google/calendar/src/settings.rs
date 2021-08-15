use anyhow::Result;

use crate::Client;

pub struct Settings {
    client: Client,
}

impl Settings {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Settings { client }
    }

    /**
     * This function performs a `GET` to the `/users/me/settings` endpoint.
     *
     * Returns all user settings for the authenticated user.
     *
     * **Parameters:**
     *
     * * `max_results: i64` -- Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional.
     * * `page_token: &str` -- Token specifying which result page to return. Optional.
     * * `sync_token: &str` -- Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then.
     *   If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.
     *   Learn more about incremental synchronization.
     *   Optional. The default is to return all entries.
     */
    pub async fn calendar_list(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        max_results: i64,
        page_token: &str,
        sync_token: &str,
    ) -> Result<crate::types::Settings> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if max_results > 0 {
            query_args.push(format!("max_results={}", max_results));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if !page_token.is_empty() {
            query_args.push(format!("page_token={}", page_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !sync_token.is_empty() {
            query_args.push(format!("sync_token={}", sync_token));
        }
        if !user_ip.is_empty() {
            query_args.push(format!("user_ip={}", user_ip));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!("/users/me/settings?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/users/me/settings/watch` endpoint.
     *
     * Watch for changes to Settings resources.
     *
     * **Parameters:**
     *
     * * `max_results: i64` -- Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional.
     * * `page_token: &str` -- Token specifying which result page to return. Optional.
     * * `sync_token: &str` -- Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then.
     *   If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.
     *   Learn more about incremental synchronization.
     *   Optional. The default is to return all entries.
     */
    pub async fn calendar_watch(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        max_results: i64,
        page_token: &str,
        sync_token: &str,
        body: &crate::types::Channel,
    ) -> Result<crate::types::Channel> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if max_results > 0 {
            query_args.push(format!("max_results={}", max_results));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if !page_token.is_empty() {
            query_args.push(format!("page_token={}", page_token));
        }
        if pretty_print {
            query_args.push(format!("pretty_print={}", pretty_print));
        }
        if !quota_user.is_empty() {
            query_args.push(format!("quota_user={}", quota_user));
        }
        if !sync_token.is_empty() {
            query_args.push(format!("sync_token={}", sync_token));
        }
        if !user_ip.is_empty() {
            query_args.push(format!("user_ip={}", user_ip));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!("/users/me/settings/watch?{}", query_);

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `GET` to the `/users/me/settings/{setting}` endpoint.
     *
     * Returns a single user setting.
     *
     * **Parameters:**
     *
     * * `setting: &str` -- The id of the user setting.
     */
    pub async fn calendar_get(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        setting: &str,
    ) -> Result<crate::types::Setting> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
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
        if !user_ip.is_empty() {
            query_args.push(format!("user_ip={}", user_ip));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/users/me/settings/{}?{}",
            crate::progenitor_support::encode_path(&setting.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }
}
