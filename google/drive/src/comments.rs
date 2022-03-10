use anyhow::Result;

use crate::Client;

pub struct Comments {
    pub client: Client,
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
     * * `page_size: i64` -- A map of maximum import sizes by MIME type, in bytes.
     * * `page_token: &str` -- The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
     * * `start_modified_time: &str` -- The minimum value of 'modifiedTime' for the result comments (RFC 3339 date-time).
     */
    pub async fn list(
        &self,
        file_id: &str,
        include_deleted: bool,
        page_size: i64,
        page_token: &str,
        start_modified_time: &str,
    ) -> Result<Vec<crate::types::Comment>> {
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
        if !start_modified_time.is_empty() {
            query_args.push((
                "startModifiedTime".to_string(),
                start_modified_time.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/files/{}/comments?{}",
            crate::progenitor_support::encode_path(file_id),
            query_
        );

        let resp: crate::types::CommentList = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.comments)
    }

    /**
     * This function performs a `GET` to the `/files/{fileId}/comments` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Lists a file's comments.
     */
    pub async fn list_all(
        &self,
        file_id: &str,
        include_deleted: bool,
        start_modified_time: &str,
    ) -> Result<Vec<crate::types::Comment>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if include_deleted {
            query_args.push(("includeDeleted".to_string(), include_deleted.to_string()));
        }
        if !start_modified_time.is_empty() {
            query_args.push((
                "startModifiedTime".to_string(),
                start_modified_time.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/files/{}/comments?{}",
            crate::progenitor_support::encode_path(file_id),
            query_
        );

        let mut resp: crate::types::CommentList = self.client.get(&url, None).await?;

        let mut comments = resp.comments;
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

            comments.append(&mut resp.comments);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(comments)
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
    pub async fn create(
        &self,
        file_id: &str,
        body: &crate::types::Comment,
    ) -> Result<crate::types::Comment> {
        let url = format!(
            "/files/{}/comments",
            crate::progenitor_support::encode_path(file_id),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
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
    pub async fn get(
        &self,
        file_id: &str,
        comment_id: &str,
        include_deleted: bool,
    ) -> Result<crate::types::Comment> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if include_deleted {
            query_args.push(("includeDeleted".to_string(), include_deleted.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/files/{}/comments/{}?{}",
            crate::progenitor_support::encode_path(file_id),
            crate::progenitor_support::encode_path(comment_id),
            query_
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
    pub async fn delete(&self, file_id: &str, comment_id: &str) -> Result<()> {
        let url = format!(
            "/files/{}/comments/{}",
            crate::progenitor_support::encode_path(file_id),
            crate::progenitor_support::encode_path(comment_id),
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
    pub async fn update(
        &self,
        file_id: &str,
        comment_id: &str,
        body: &crate::types::Comment,
    ) -> Result<crate::types::Comment> {
        let url = format!(
            "/files/{}/comments/{}",
            crate::progenitor_support::encode_path(file_id),
            crate::progenitor_support::encode_path(comment_id),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
