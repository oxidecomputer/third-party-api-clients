use crate::Client;
use crate::ClientResult;

pub struct Revisions {
    pub client: Client,
}

impl Revisions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Revisions { client }
    }

    /**
     * This function performs a `GET` to the `/files/{fileId}/revisions` endpoint.
     *
     * Lists a file's revisions.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `page_size: i64` -- A map of maximum import sizes by MIME type, in bytes.
     * * `page_token: &str` -- The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.
     */
    pub async fn list(
        &self,
        file_id: &str,
        page_size: i64,
        page_token: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Revision>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page_size > 0 {
            query_args.push(("pageSize".to_string(), page_size.to_string()));
        }
        if !page_token.is_empty() {
            query_args.push(("pageToken".to_string(), page_token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/files/{}/revisions?{}",
                crate::progenitor_support::encode_path(file_id),
                query_
            ),
            None,
        );
        let resp: crate::Response<crate::types::RevisionList> = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        // Return our response data.
        Ok(crate::Response::new(
            resp.status,
            resp.headers,
            resp.body.revisions.to_vec(),
        ))
    }
    /**
     * This function performs a `GET` to the `/files/{fileId}/revisions` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Lists a file's revisions.
     */
    pub async fn list_all(
        &self,
        file_id: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Revision>>> {
        let url = self.client.url(
            &format!(
                "/files/{}/revisions",
                crate::progenitor_support::encode_path(file_id),
            ),
            None,
        );
        let crate::Response::<crate::types::RevisionList> {
            mut status,
            mut headers,
            mut body,
        } = self
            .client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await?;

        let mut revisions = body.revisions;
        let mut page = body.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            if !url.contains('?') {
                crate::Response::<crate::types::RevisionList> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}?pageToken={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            } else {
                crate::Response::<crate::types::RevisionList> {
                    status,
                    headers,
                    body,
                } = self
                    .client
                    .get(
                        &format!("{}&pageToken={}", url, page),
                        crate::Message {
                            body: None,
                            content_type: None,
                        },
                    )
                    .await?;
            }

            revisions.append(&mut body.revisions);

            if !body.next_page_token.is_empty() && body.next_page_token != page {
                page = body.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(crate::Response::new(status, headers, revisions))
    }
    /**
     * This function performs a `GET` to the `/files/{fileId}/revisions/{revisionId}` endpoint.
     *
     * Gets a revision's metadata or content by ID.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `revision_id: &str` -- A link to this theme's background image.
     * * `acknowledge_abuse: bool` -- Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when alt=media.
     */
    pub async fn get(
        &self,
        file_id: &str,
        revision_id: &str,
        acknowledge_abuse: bool,
    ) -> ClientResult<crate::Response<crate::types::Revision>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if acknowledge_abuse {
            query_args.push((
                "acknowledgeAbuse".to_string(),
                acknowledge_abuse.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/files/{}/revisions/{}?{}",
                crate::progenitor_support::encode_path(file_id),
                crate::progenitor_support::encode_path(revision_id),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * This function performs a `DELETE` to the `/files/{fileId}/revisions/{revisionId}` endpoint.
     *
     * Permanently deletes a file version. You can only delete revisions for files with binary content in Google Drive, like images or videos. Revisions for other files, like Google Docs or Sheets, and the last remaining file version can't be deleted.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `revision_id: &str` -- A link to this theme's background image.
     */
    pub async fn delete(
        &self,
        file_id: &str,
        revision_id: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/files/{}/revisions/{}",
                crate::progenitor_support::encode_path(file_id),
                crate::progenitor_support::encode_path(revision_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * This function performs a `PATCH` to the `/files/{fileId}/revisions/{revisionId}` endpoint.
     *
     * Updates a revision with patch semantics.
     *
     * **Parameters:**
     *
     * * `file_id: &str` -- A link to this theme's background image.
     * * `revision_id: &str` -- A link to this theme's background image.
     */
    pub async fn update(
        &self,
        file_id: &str,
        revision_id: &str,
        body: &crate::types::Revision,
    ) -> ClientResult<crate::Response<crate::types::Revision>> {
        let url = self.client.url(
            &format!(
                "/files/{}/revisions/{}",
                crate::progenitor_support::encode_path(file_id),
                crate::progenitor_support::encode_path(revision_id),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
}
