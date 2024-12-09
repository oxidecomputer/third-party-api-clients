use crate::Client;
use crate::ClientResult;

pub struct Gists {
    pub client: Client,
}

impl Gists {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Gists { client }
    }

    /**
     * List gists for the authenticated user.
     *
     * This function performs a `GET` to the `/gists` endpoint.
     *
     * Lists the authenticated user's gists or if called anonymously, this endpoint returns all public gists:
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-gists-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list(
        &self,
        since: Option<chrono::DateTime<chrono::Utc>>,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::BaseGist>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/gists?{}", query_), None);
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
     * List gists for the authenticated user.
     *
     * This function performs a `GET` to the `/gists` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Lists the authenticated user's gists or if called anonymously, this endpoint returns all public gists:
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-gists-for-the-authenticated-user>
     */
    pub async fn list_all(
        &self,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> ClientResult<crate::Response<Vec<crate::types::BaseGist>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/gists?{}", query_), None);
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Create a gist.
     *
     * This function performs a `POST` to the `/gists` endpoint.
     *
     * Allows you to add a new gist with one or more files.
     *
     * **Note:** Don't name your files "gistfile" with a numerical suffix. This is the format of the automatic naming scheme that Gist uses internally.
     *
     * FROM: <https://docs.github.com/rest/reference/gists#create-a-gist>
     */
    pub async fn create(
        &self,
        body: &crate::types::GistsCreateRequest,
    ) -> ClientResult<crate::Response<crate::types::GistSimple>> {
        let url = self.client.url("/gists", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * List public gists.
     *
     * This function performs a `GET` to the `/gists/public` endpoint.
     *
     * List public gists sorted by most recently updated to least recently updated.
     *
     * Note: With [pagination](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination), you can fetch up to 3000 gists. For example, you can fetch 100 pages with 30 gists per page or 30 pages with 100 gists per page.
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-public-gists>
     *
     * **Parameters:**
     *
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_public(
        &self,
        since: Option<chrono::DateTime<chrono::Utc>>,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::BaseGist>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/gists/public?{}", query_), None);
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
     * List public gists.
     *
     * This function performs a `GET` to the `/gists/public` endpoint.
     *
     * As opposed to `list_public`, this function returns all the pages of the request at once.
     *
     * List public gists sorted by most recently updated to least recently updated.
     *
     * Note: With [pagination](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination), you can fetch up to 3000 gists. For example, you can fetch 100 pages with 30 gists per page or 30 pages with 100 gists per page.
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-public-gists>
     */
    pub async fn list_all_public(
        &self,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> ClientResult<crate::Response<Vec<crate::types::BaseGist>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/gists/public?{}", query_), None);
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * List starred gists.
     *
     * This function performs a `GET` to the `/gists/starred` endpoint.
     *
     * List the authenticated user's starred gists:
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-starred-gists>
     *
     * **Parameters:**
     *
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_starred(
        &self,
        since: Option<chrono::DateTime<chrono::Utc>>,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::BaseGist>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/gists/starred?{}", query_), None);
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
     * List starred gists.
     *
     * This function performs a `GET` to the `/gists/starred` endpoint.
     *
     * As opposed to `list_starred`, this function returns all the pages of the request at once.
     *
     * List the authenticated user's starred gists:
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-starred-gists>
     */
    pub async fn list_all_starred(
        &self,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> ClientResult<crate::Response<Vec<crate::types::BaseGist>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/gists/starred?{}", query_), None);
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get a gist.
     *
     * This function performs a `GET` to the `/gists/{gist_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#get-a-gist>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     */
    pub async fn get(
        &self,
        gist_id: &str,
    ) -> ClientResult<crate::Response<crate::types::GistSimple>> {
        let url = self.client.url(
            &format!("/gists/{}", crate::progenitor_support::encode_path(gist_id),),
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
     * Delete a gist.
     *
     * This function performs a `DELETE` to the `/gists/{gist_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#delete-a-gist>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     */
    pub async fn delete(&self, gist_id: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!("/gists/{}", crate::progenitor_support::encode_path(gist_id),),
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
     * Update a gist.
     *
     * This function performs a `PATCH` to the `/gists/{gist_id}` endpoint.
     *
     * Allows you to update or delete a gist file and rename gist files. Files from the previous version of the gist that aren't explicitly changed during an edit are unchanged.
     *
     * FROM: <https://docs.github.com/rest/reference/gists/#update-a-gist>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     */
    pub async fn update(
        &self,
        gist_id: &str,
        body: &crate::types::GistsUpdateRequest,
    ) -> ClientResult<crate::Response<crate::types::GistSimple>> {
        let url = self.client.url(
            &format!("/gists/{}", crate::progenitor_support::encode_path(gist_id),),
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
    /**
     * List gist comments.
     *
     * This function performs a `GET` to the `/gists/{gist_id}/comments` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-gist-comments>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_comments(
        &self,
        gist_id: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::GistComment>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/gists/{}/comments?{}",
                crate::progenitor_support::encode_path(gist_id),
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
     * List gist comments.
     *
     * This function performs a `GET` to the `/gists/{gist_id}/comments` endpoint.
     *
     * As opposed to `list_comments`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-gist-comments>
     */
    pub async fn list_all_comments(
        &self,
        gist_id: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::GistComment>>> {
        let url = self.client.url(
            &format!(
                "/gists/{}/comments",
                crate::progenitor_support::encode_path(gist_id),
            ),
            None,
        );
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Create a gist comment.
     *
     * This function performs a `POST` to the `/gists/{gist_id}/comments` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#create-a-gist-comment>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     */
    pub async fn create_comment(
        &self,
        gist_id: &str,
        body: &crate::types::PullsUpdateReviewRequest,
    ) -> ClientResult<crate::Response<crate::types::GistComment>> {
        let url = self.client.url(
            &format!(
                "/gists/{}/comments",
                crate::progenitor_support::encode_path(gist_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Get a gist comment.
     *
     * This function performs a `GET` to the `/gists/{gist_id}/comments/{comment_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#get-a-gist-comment>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     * * `comment_id: i64` -- comment_id parameter.
     */
    pub async fn get_comment(
        &self,
        gist_id: &str,
        comment_id: i64,
    ) -> ClientResult<crate::Response<crate::types::GistComment>> {
        let url = self.client.url(
            &format!(
                "/gists/{}/comments/{}",
                crate::progenitor_support::encode_path(gist_id),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * Delete a gist comment.
     *
     * This function performs a `DELETE` to the `/gists/{gist_id}/comments/{comment_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#delete-a-gist-comment>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     * * `comment_id: i64` -- comment_id parameter.
     */
    pub async fn delete_comment(
        &self,
        gist_id: &str,
        comment_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/gists/{}/comments/{}",
                crate::progenitor_support::encode_path(gist_id),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * Update a gist comment.
     *
     * This function performs a `PATCH` to the `/gists/{gist_id}/comments/{comment_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#update-a-gist-comment>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     * * `comment_id: i64` -- comment_id parameter.
     */
    pub async fn update_comment(
        &self,
        gist_id: &str,
        comment_id: i64,
        body: &crate::types::PullsUpdateReviewRequest,
    ) -> ClientResult<crate::Response<crate::types::GistComment>> {
        let url = self.client.url(
            &format!(
                "/gists/{}/comments/{}",
                crate::progenitor_support::encode_path(gist_id),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
    /**
     * List gist commits.
     *
     * This function performs a `GET` to the `/gists/{gist_id}/commits` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-gist-commits>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_commits(
        &self,
        gist_id: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::GistCommit>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/gists/{}/commits?{}",
                crate::progenitor_support::encode_path(gist_id),
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
     * List gist commits.
     *
     * This function performs a `GET` to the `/gists/{gist_id}/commits` endpoint.
     *
     * As opposed to `list_commits`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-gist-commits>
     */
    pub async fn list_all_commits(
        &self,
        gist_id: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::GistCommit>>> {
        let url = self.client.url(
            &format!(
                "/gists/{}/commits",
                crate::progenitor_support::encode_path(gist_id),
            ),
            None,
        );
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * List gist forks.
     *
     * This function performs a `GET` to the `/gists/{gist_id}/forks` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-gist-forks>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_forks(
        &self,
        gist_id: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::GistSimple>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/gists/{}/forks?{}",
                crate::progenitor_support::encode_path(gist_id),
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
     * List gist forks.
     *
     * This function performs a `GET` to the `/gists/{gist_id}/forks` endpoint.
     *
     * As opposed to `list_forks`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-gist-forks>
     */
    pub async fn list_all_forks(
        &self,
        gist_id: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::GistSimple>>> {
        let url = self.client.url(
            &format!(
                "/gists/{}/forks",
                crate::progenitor_support::encode_path(gist_id),
            ),
            None,
        );
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Fork a gist.
     *
     * This function performs a `POST` to the `/gists/{gist_id}/forks` endpoint.
     *
     * **Note**: This was previously `/gists/:gist_id/fork`.
     *
     * FROM: <https://docs.github.com/rest/reference/gists#fork-a-gist>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     */
    pub async fn fork(
        &self,
        gist_id: &str,
    ) -> ClientResult<crate::Response<crate::types::BaseGist>> {
        let url = self.client.url(
            &format!(
                "/gists/{}/forks",
                crate::progenitor_support::encode_path(gist_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Check if a gist is starred.
     *
     * This function performs a `GET` to the `/gists/{gist_id}/star` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#check-if-a-gist-is-starred>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     */
    pub async fn check_is_starred(&self, gist_id: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/gists/{}/star",
                crate::progenitor_support::encode_path(gist_id),
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
     * Star a gist.
     *
     * This function performs a `PUT` to the `/gists/{gist_id}/star` endpoint.
     *
     * Note that you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs)."
     *
     * FROM: <https://docs.github.com/rest/reference/gists#star-a-gist>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     */
    pub async fn star(&self, gist_id: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/gists/{}/star",
                crate::progenitor_support::encode_path(gist_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Unstar a gist.
     *
     * This function performs a `DELETE` to the `/gists/{gist_id}/star` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#unstar-a-gist>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     */
    pub async fn unstar(&self, gist_id: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/gists/{}/star",
                crate::progenitor_support::encode_path(gist_id),
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
     * Get a gist revision.
     *
     * This function performs a `GET` to the `/gists/{gist_id}/{sha}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/gists#get-a-gist-revision>
     *
     * **Parameters:**
     *
     * * `gist_id: &str` -- gist_id parameter.
     * * `sha: &str`
     */
    pub async fn get_revision(
        &self,
        gist_id: &str,
        sha: &str,
    ) -> ClientResult<crate::Response<crate::types::GistSimple>> {
        let url = self.client.url(
            &format!(
                "/gists/{}/{}",
                crate::progenitor_support::encode_path(gist_id),
                crate::progenitor_support::encode_path(sha),
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
     * List gists for a user.
     *
     * This function performs a `GET` to the `/users/{username}/gists` endpoint.
     *
     * Lists public gists for the specified user:
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-gists-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_user(
        &self,
        username: &str,
        since: Option<chrono::DateTime<chrono::Utc>>,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::BaseGist>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/gists?{}",
                crate::progenitor_support::encode_path(username),
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
     * List gists for a user.
     *
     * This function performs a `GET` to the `/users/{username}/gists` endpoint.
     *
     * As opposed to `list_for_user`, this function returns all the pages of the request at once.
     *
     * Lists public gists for the specified user:
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-gists-for-a-user>
     */
    pub async fn list_all_for_user(
        &self,
        username: &str,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> ClientResult<crate::Response<Vec<crate::types::BaseGist>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/gists?{}",
                crate::progenitor_support::encode_path(username),
                query_
            ),
            None,
        );
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}
