use anyhow::Result;

use crate::Client;

pub struct CalendarList {
    client: Client,
}

impl CalendarList {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CalendarList { client }
    }

    /**
     * This function performs a `GET` to the `/users/me/calendarList` endpoint.
     *
     * Returns the calendars on the user's calendar list.
     *
     * **Parameters:**
     *
     * * `max_results: i64` -- Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional.
     * * `min_access_role: crate::types::MinAccessRole` -- The minimum access role for the user in the returned entries. Optional. The default is no restriction.
     * * `page_token: &str` -- Token specifying which result page to return. Optional.
     * * `show_deleted: bool` -- Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     * * `show_hidden: bool` -- Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     * * `sync_token: &str` -- Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. If only read-only fields such as calendar properties or ACLs have changed, the entry won't be returned. All entries deleted and hidden since the previous list request will always be in the result set and it is not allowed to set showDeleted neither showHidden to False.
     *   To ensure client state consistency minAccessRole query parameter cannot be specified together with nextSyncToken.
     *   If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.
     *   Learn more about incremental synchronization.
     *   Optional. The default is to return all entries.
     */
    pub async fn get(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        max_results: i64,
        min_access_role: crate::types::MinAccessRole,
        page_token: &str,
        show_deleted: bool,
        show_hidden: bool,
        sync_token: &str,
    ) -> Result<crate::types::CalendarList> {
        let mut query = String::new();
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
        query_args.push(format!("min_access_role={}", min_access_role));
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
        if show_deleted {
            query_args.push(format!("show_deleted={}", show_deleted));
        }
        if show_hidden {
            query_args.push(format!("show_hidden={}", show_hidden));
        }
        if !sync_token.is_empty() {
            query_args.push(format!("sync_token={}", sync_token));
        }
        if !user_ip.is_empty() {
            query_args.push(format!("user_ip={}", user_ip));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/users/me/calendarList?{}", query);

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/users/me/calendarList` endpoint.
     *
     * Inserts an existing calendar into the user's calendar list.
     *
     * **Parameters:**
     *
     * * `color_rgb_format: bool` -- Whether to use the foregroundColor and backgroundColor fields to write the calendar colors (RGB). If this feature is used, the index-based colorId field will be set to the best matching option automatically. Optional. The default is False.
     */
    pub async fn insert(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        color_rgb_format: bool,
        body: &crate::types::CalendarListEntry,
    ) -> Result<crate::types::CalendarListEntry> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if color_rgb_format {
            query_args.push(format!("color_rgb_format={}", color_rgb_format));
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
        if !user_ip.is_empty() {
            query_args.push(format!("user_ip={}", user_ip));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/users/me/calendarList?{}", query);

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `POST` to the `/users/me/calendarList/watch` endpoint.
     *
     * Watch for changes to CalendarList resources.
     *
     * **Parameters:**
     *
     * * `max_results: i64` -- Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional.
     * * `min_access_role: crate::types::MinAccessRole` -- The minimum access role for the user in the returned entries. Optional. The default is no restriction.
     * * `page_token: &str` -- Token specifying which result page to return. Optional.
     * * `show_deleted: bool` -- Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     * * `show_hidden: bool` -- Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.
     * * `sync_token: &str` -- Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. If only read-only fields such as calendar properties or ACLs have changed, the entry won't be returned. All entries deleted and hidden since the previous list request will always be in the result set and it is not allowed to set showDeleted neither showHidden to False.
     *   To ensure client state consistency minAccessRole query parameter cannot be specified together with nextSyncToken.
     *   If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.
     *   Learn more about incremental synchronization.
     *   Optional. The default is to return all entries.
     */
    pub async fn watch(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        max_results: i64,
        min_access_role: crate::types::MinAccessRole,
        page_token: &str,
        show_deleted: bool,
        show_hidden: bool,
        sync_token: &str,
        body: &crate::types::Channel,
    ) -> Result<crate::types::Channel> {
        let mut query = String::new();
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
        query_args.push(format!("min_access_role={}", min_access_role));
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
        if show_deleted {
            query_args.push(format!("show_deleted={}", show_deleted));
        }
        if show_hidden {
            query_args.push(format!("show_hidden={}", show_hidden));
        }
        if !sync_token.is_empty() {
            query_args.push(format!("sync_token={}", sync_token));
        }
        if !user_ip.is_empty() {
            query_args.push(format!("user_ip={}", user_ip));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/users/me/calendarList/watch?{}", query);

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `GET` to the `/users/me/calendarList/{calendarId}` endpoint.
     *
     * Returns a calendar from the user's calendar list.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     */
    pub async fn get_calendar_list(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        calendar_id: &str,
    ) -> Result<crate::types::CalendarListEntry> {
        let mut query = String::new();
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
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/users/me/calendarList/{}?{}",
            crate::progenitor_support::encode_path(&calendar_id.to_string()),
            query
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `PUT` to the `/users/me/calendarList/{calendarId}` endpoint.
     *
     * Updates an existing calendar on the user's calendar list.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     * * `color_rgb_format: bool` -- Whether to use the foregroundColor and backgroundColor fields to write the calendar colors (RGB). If this feature is used, the index-based colorId field will be set to the best matching option automatically. Optional. The default is False.
     */
    pub async fn update(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        calendar_id: &str,
        color_rgb_format: bool,
        body: &crate::types::CalendarListEntry,
    ) -> Result<crate::types::CalendarListEntry> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if color_rgb_format {
            query_args.push(format!("color_rgb_format={}", color_rgb_format));
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
        if !user_ip.is_empty() {
            query_args.push(format!("user_ip={}", user_ip));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/users/me/calendarList/{}?{}",
            crate::progenitor_support::encode_path(&calendar_id.to_string()),
            query
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `DELETE` to the `/users/me/calendarList/{calendarId}` endpoint.
     *
     * Removes a calendar from the user's calendar list.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     */
    pub async fn delete(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        calendar_id: &str,
    ) -> Result<()> {
        let mut query = String::new();
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
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/users/me/calendarList/{}?{}",
            crate::progenitor_support::encode_path(&calendar_id.to_string()),
            query
        );

        self.client.delete(&url, None).await
    }

    /**
     * This function performs a `PATCH` to the `/users/me/calendarList/{calendarId}` endpoint.
     *
     * Updates an existing calendar on the user's calendar list. This method supports patch semantics.
     *
     * **Parameters:**
     *
     * * `calendar_id: &str` -- Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
     * * `color_rgb_format: bool` -- Whether to use the foregroundColor and backgroundColor fields to write the calendar colors (RGB). If this feature is used, the index-based colorId field will be set to the best matching option automatically. Optional. The default is False.
     */
    pub async fn patch(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        calendar_id: &str,
        color_rgb_format: bool,
        body: &crate::types::CalendarListEntry,
    ) -> Result<crate::types::CalendarListEntry> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if color_rgb_format {
            query_args.push(format!("color_rgb_format={}", color_rgb_format));
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
        if !user_ip.is_empty() {
            query_args.push(format!("user_ip={}", user_ip));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!(
            "/users/me/calendarList/{}?{}",
            crate::progenitor_support::encode_path(&calendar_id.to_string()),
            query
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
