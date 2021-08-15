use anyhow::Result;

use crate::Client;

pub struct Replies {
    client: Client,
}

impl Replies {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Replies { client }
    }

    /**
     * This function performs a `GET` to the `/files/{fileId}/comments/{commentId}/replies` endpoint.
     *
     * Lists a comment's replies.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `comment_id: &str` -- A link to this theme's background image.
     * * `include_deleted: bool` -- Whether to include deleted replies. Deleted replies will not include their original content.
     * * `page_size: i64` -- The maximum number of replies to return per page.
     * * `page_token: &str` -- The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
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
        comment_id: &str,
        include_deleted: bool,
        page_size: i64,
        page_token: &str,
    ) -> Result<crate::types::ReplyList> {
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
            "/files/{}/comments/{}/replies?{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
            crate::progenitor_support::encode_path(&comment_id.to_string()),
            query
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/files/{fileId}/comments/{commentId}/replies` endpoint.
     *
     * Creates a new reply to a comment.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `comment_id: &str` -- A link to this theme's background image.
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
        comment_id: &str,
        body: &crate::types::Reply,
    ) -> Result<crate::types::Reply> {
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
            "/files/{}/comments/{}/replies?{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
            crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * This function performs a `GET` to the `/files/{fileId}/comments/{commentId}/replies/{replyId}` endpoint.
     *
     * Gets a reply by ID.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `comment_id: &str` -- A link to this theme's background image.
     * * `reply_id: &str` -- A link to this theme's background image.
     * * `include_deleted: bool` -- Whether to return deleted replies. Deleted replies will not include their original content.
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
        reply_id: &str,
        include_deleted: bool,
    ) -> Result<crate::types::Reply> {
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
            "/files/{}/comments/{}/replies/{}?{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
            crate::progenitor_support::encode_path(&comment_id.to_string()),
            crate::progenitor_support::encode_path(&reply_id.to_string()),
            query
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `DELETE` to the `/files/{fileId}/comments/{commentId}/replies/{replyId}` endpoint.
     *
     * Deletes a reply.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `comment_id: &str` -- A link to this theme's background image.
     * * `reply_id: &str` -- A link to this theme's background image.
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
        reply_id: &str,
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
            "/files/{}/comments/{}/replies/{}?{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
            crate::progenitor_support::encode_path(&comment_id.to_string()),
            crate::progenitor_support::encode_path(&reply_id.to_string()),
            query
        );

        self.client.delete(&url, None).await
    }

    /**
     * This function performs a `PATCH` to the `/files/{fileId}/comments/{commentId}/replies/{replyId}` endpoint.
     *
     * Updates a reply with patch semantics.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `comment_id: &str` -- A link to this theme's background image.
     * * `reply_id: &str` -- A link to this theme's background image.
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
        reply_id: &str,
        body: &crate::types::Reply,
    ) -> Result<crate::types::Reply> {
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
            "/files/{}/comments/{}/replies/{}?{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
            crate::progenitor_support::encode_path(&comment_id.to_string()),
            crate::progenitor_support::encode_path(&reply_id.to_string()),
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
