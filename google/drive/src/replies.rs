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
    pub async fn list(
        &self,
        file_id: &str,
        comment_id: &str,
        include_deleted: bool,
        page_size: i64,
        page_token: &str,
    ) -> Result<Vec<crate::types::Reply>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if include_deleted {
            query_args.push(("includeDeleted".to_string(), include_deleted.to_string()));
        }
        if page_size > 0 {
            query_args.push(("pageSize".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/files/{}/comments/{}/replies?{}",
            crate::progenitor_support::encode_path(file_id),
            crate::progenitor_support::encode_path(comment_id),
            query_
        );

        let resp: crate::types::ReplyList = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.replies)
    }

    /**
     * This function performs a `GET` to the `/files/{fileId}/comments/{commentId}/replies` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Lists a comment's replies.
     */
    pub async fn list_all(
        &self,
        file_id: &str,
        comment_id: &str,
        include_deleted: bool,
    ) -> Result<Vec<crate::types::Reply>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if include_deleted {
            query_args.push(("includeDeleted".to_string(), include_deleted.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/files/{}/comments/{}/replies?{}",
            crate::progenitor_support::encode_path(file_id),
            crate::progenitor_support::encode_path(comment_id),
            query_
        );

        let mut resp: crate::types::ReplyList = self.client.get(&url, None).await?;

        let mut replies = resp.replies;
        let mut page = resp.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                resp = self
                    .client
                    .get(&format!("{}?pageToken={}", url, page), None)
                    .await?;
            } else {
                resp = self
                    .client
                    .get(&format!("{}&pageToken={}", url, page), None)
                    .await?;
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
    pub async fn create(
        &self,
        file_id: &str,
        comment_id: &str,
        body: &crate::types::Reply,
    ) -> Result<crate::types::Reply> {
        let url = format!(
            "/files/{}/comments/{}/replies",
            crate::progenitor_support::encode_path(file_id),
            crate::progenitor_support::encode_path(comment_id),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
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
    pub async fn get(
        &self,
        file_id: &str,
        comment_id: &str,
        reply_id: &str,
        include_deleted: bool,
    ) -> Result<crate::types::Reply> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if include_deleted {
            query_args.push(("includeDeleted".to_string(), include_deleted.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/files/{}/comments/{}/replies/{}?{}",
            crate::progenitor_support::encode_path(file_id),
            crate::progenitor_support::encode_path(comment_id),
            crate::progenitor_support::encode_path(reply_id),
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
    pub async fn delete(&self, file_id: &str, comment_id: &str, reply_id: &str) -> Result<()> {
        let url = format!(
            "/files/{}/comments/{}/replies/{}",
            crate::progenitor_support::encode_path(file_id),
            crate::progenitor_support::encode_path(comment_id),
            crate::progenitor_support::encode_path(reply_id),
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
    pub async fn update(
        &self,
        file_id: &str,
        comment_id: &str,
        reply_id: &str,
        body: &crate::types::Reply,
    ) -> Result<crate::types::Reply> {
        let url = format!(
            "/files/{}/comments/{}/replies/{}",
            crate::progenitor_support::encode_path(file_id),
            crate::progenitor_support::encode_path(comment_id),
            crate::progenitor_support::encode_path(reply_id),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
