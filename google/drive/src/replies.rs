use anyhow::Result;

use crate::Client;

pub struct Replies {
    pub client: Client,
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
     * * `page_size: i64` -- A map of maximum import sizes by MIME type, in bytes.
     * * `page_token: &str` -- The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
     */
    pub async fn drive_list(
        &self,
        file_id: &str,
        comment_id: &str,
        include_deleted: bool,
        page_size: i64,
        page_token: &str,
    ) -> Result<Vec<crate::types::Reply>> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if include_deleted {
            query_args.push(format!("include_deleted={}", include_deleted));
        }
        if page_size > 0 {
            query_args.push(format!("page_size={}", page_size));
        }
        if !page_token.is_empty() {
            query_args.push(format!("page_token={}", page_token));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/files/{}/comments/{}/replies?{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
            crate::progenitor_support::encode_path(&comment_id.to_string()),
            query_
        );

        let resp: crate::types::ReplyList = self.client.get(&url, None).await.unwrap();

        // Return our response data.
        Ok(resp.replies)
    }

    /**
     * This function performs a `GET` to the `/files/{fileId}/comments/{commentId}/replies` endpoint.
     *
     * As opposed to `drive_list`, this function returns all the pages of the request at once.
     *
     * Lists a comment's replies.
     */
    pub async fn drive_list_replies(
        &self,
        file_id: &str,
        comment_id: &str,
        include_deleted: bool,
    ) -> Result<Vec<crate::types::Reply>> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if include_deleted {
            query_args.push(format!("include_deleted={}", include_deleted));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/files/{}/comments/{}/replies?{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
            crate::progenitor_support::encode_path(&comment_id.to_string()),
            query_
        );

        let mut resp: crate::types::ReplyList = self.client.get(&url, None).await.unwrap();

        let mut replies = resp.replies;
        let mut page = resp.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                resp = self
                    .client
                    .get(&format!("{}?pageToken={}", url, page), None)
                    .await
                    .unwrap();
            } else {
                resp = self
                    .client
                    .get(&format!("{}&pageToken={}", url, page), None)
                    .await
                    .unwrap();
            }

            replies.append(&mut resp.replies);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(replies)
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
        file_id: &str,
        comment_id: &str,
        body: &crate::types::Reply,
    ) -> Result<crate::types::Reply> {
        let url = format!(
            "/files/{}/comments/{}/replies",
            crate::progenitor_support::encode_path(&file_id.to_string()),
            crate::progenitor_support::encode_path(&comment_id.to_string()),
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
        file_id: &str,
        comment_id: &str,
        reply_id: &str,
        include_deleted: bool,
    ) -> Result<crate::types::Reply> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if include_deleted {
            query_args.push(format!("include_deleted={}", include_deleted));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!(
            "/files/{}/comments/{}/replies/{}?{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
            crate::progenitor_support::encode_path(&comment_id.to_string()),
            crate::progenitor_support::encode_path(&reply_id.to_string()),
            query_
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
        file_id: &str,
        comment_id: &str,
        reply_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/files/{}/comments/{}/replies/{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
            crate::progenitor_support::encode_path(&comment_id.to_string()),
            crate::progenitor_support::encode_path(&reply_id.to_string()),
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
        file_id: &str,
        comment_id: &str,
        reply_id: &str,
        body: &crate::types::Reply,
    ) -> Result<crate::types::Reply> {
        let url = format!(
            "/files/{}/comments/{}/replies/{}",
            crate::progenitor_support::encode_path(&file_id.to_string()),
            crate::progenitor_support::encode_path(&comment_id.to_string()),
            crate::progenitor_support::encode_path(&reply_id.to_string()),
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
