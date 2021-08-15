use anyhow::Result;

use crate::Client;

pub struct Comments {
    client: Client,
}

impl Comments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Comments { client }
    }

    /**
     * This function performs a `GET` to the `/files/{fileId}/comments` endpoint.
     *
     * Lists a file's comments.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `include_deleted: bool` -- Whether to include deleted comments. Deleted comments will not include their original content.
     * * `page_size: i64` -- The maximum number of comments to return per page.
     * * `page_token: &str` -- The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
     * * `start_modified_time: &str` -- The minimum value of 'modifiedTime' for the result comments (RFC 3339 date-time).
     */
    pub async fn drive_list(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        file_id: &str,
        include_deleted: bool,
        page_size: i64,
        page_token: &str,
        start_modified_time: &str,
    ) -> Result<crate::types::CommentList> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if include_deleted {
            query_args.push(format!("include_deleted={}", include_deleted));
        }
        if !key.is_empty() {
            query_args.push(format!("key={}", key));
        }
        if !oauth_token.is_empty() {
            query_args.push(format!("oauth_token={}", oauth_token));
        }
        if page_size > 0 {
            query_args.push(format!("page_size={}", page_size));
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
        if !start_modified_time.is_empty() {
            query_args.push(format!("start_modified_time={}", start_modified_time));
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
            "/files/{}/comments?{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
            query
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/files/{fileId}/comments` endpoint.
     *
     * Creates a new comment on a file.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     */
    pub async fn drive_create(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        file_id: &str,
        body: &crate::types::Comment,
    ) -> Result<crate::types::Comment> {
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
            "/files/{}/comments?{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
            query
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * This function performs a `GET` to the `/files/{fileId}/comments/{commentId}` endpoint.
     *
     * Gets a comment by ID.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `comment_id: &str` -- A link to this theme's background image.
     * * `include_deleted: bool` -- Whether to return deleted comments. Deleted comments will not include their original content.
     */
    pub async fn drive_get(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        file_id: &str,
        comment_id: &str,
        include_deleted: bool,
    ) -> Result<crate::types::Comment> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        query_args.push(format!("alt={}", alt));
        if !fields.is_empty() {
            query_args.push(format!("fields={}", fields));
        }
        if include_deleted {
            query_args.push(format!("include_deleted={}", include_deleted));
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
            "/files/{}/comments/{}?{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
            crate::progenitor_support::encode_path(&comment_id.to_string()),
            query
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `DELETE` to the `/files/{fileId}/comments/{commentId}` endpoint.
     *
     * Deletes a comment.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `comment_id: &str` -- A link to this theme's background image.
     */
    pub async fn drive_delete(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        file_id: &str,
        comment_id: &str,
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
            "/files/{}/comments/{}?{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
            crate::progenitor_support::encode_path(&comment_id.to_string()),
            query
        );

        self.client.delete(&url, None).await
    }

    /**
     * This function performs a `PATCH` to the `/files/{fileId}/comments/{commentId}` endpoint.
     *
     * Updates a comment with patch semantics.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `comment_id: &str` -- A link to this theme's background image.
     */
    pub async fn drive_update(
        &self,
        alt: crate::types::Alt,
        fields: &str,
        key: &str,
        oauth_token: &str,
        pretty_print: bool,
        quota_user: &str,
        user_ip: &str,
        file_id: &str,
        comment_id: &str,
        body: &crate::types::Comment,
    ) -> Result<crate::types::Comment> {
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
            "/files/{}/comments/{}?{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
            crate::progenitor_support::encode_path(&comment_id.to_string()),
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
