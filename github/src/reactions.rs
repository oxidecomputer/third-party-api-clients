use crate::Client;
use crate::ClientResult;

pub struct Reactions {
    pub client: Client,
}

impl Reactions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Reactions { client }
    }

    /**
     * List reactions for a commit comment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/comments/{comment_id}/reactions` endpoint.
     *
     * List the reactions to a [commit comment](https://docs.github.com/rest/commits/comments#get-a-commit-comment).
     *
     * FROM: <https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-commit-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `comment_id: i64` -- The unique identifier of the comment.
     * * `content: crate::types::Content` -- Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to a commit comment.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_for_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        content: crate::types::Content,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/comments/{}/reactions?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * List reactions for a commit comment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/comments/{comment_id}/reactions` endpoint.
     *
     * As opposed to `list_for_commit_comment`, this function returns all the pages of the request at once.
     *
     * List the reactions to a [commit comment](https://docs.github.com/rest/commits/comments#get-a-commit-comment).
     *
     * FROM: <https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-commit-comment>
     */
    pub async fn list_all_for_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        content: crate::types::Content,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/comments/{}/reactions?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
    /**
     * Create reaction for a commit comment.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/comments/{comment_id}/reactions` endpoint.
     *
     * Create a reaction to a [commit comment](https://docs.github.com/rest/commits/comments#get-a-commit-comment). A response with an HTTP `200` status means that you already added the reaction type to this commit comment.
     *
     * FROM: <https://docs.github.com/rest/reactions/reactions#create-reaction-for-a-commit-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `comment_id: i64` -- The unique identifier of the comment.
     */
    pub async fn create_for_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &crate::types::ReactionsCreateIssueRequest,
    ) -> ClientResult<crate::Response<crate::types::Reaction>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/comments/{}/reactions",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * Delete a commit comment reaction.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/comments/{comment_id}/reactions/{reaction_id}` endpoint.
     *
     * > [!NOTE]
     * > You can also specify a repository by `repository_id` using the route `DELETE /repositories/:repository_id/comments/:comment_id/reactions/:reaction_id`.
     *
     * Delete a reaction to a [commit comment](https://docs.github.com/rest/commits/comments#get-a-commit-comment).
     *
     * FROM: <https://docs.github.com/rest/reactions/reactions#delete-a-commit-comment-reaction>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `comment_id: i64` -- The unique identifier of the comment.
     * * `reaction_id: i64` -- The unique identifier of the reaction.
     */
    pub async fn delete_for_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        reaction_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/comments/{}/reactions/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
                crate::progenitor_support::encode_path(&reaction_id.to_string()),
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
     * List reactions for an issue comment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/comments/{comment_id}/reactions` endpoint.
     *
     * List the reactions to an [issue comment](https://docs.github.com/rest/issues/comments#get-an-issue-comment).
     *
     * FROM: <https://docs.github.com/rest/reactions/reactions#list-reactions-for-an-issue-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `comment_id: i64` -- The unique identifier of the comment.
     * * `content: crate::types::Content` -- Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to an issue comment.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_for_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        content: crate::types::Content,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/comments/{}/reactions?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * List reactions for an issue comment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/comments/{comment_id}/reactions` endpoint.
     *
     * As opposed to `list_for_issue_comment`, this function returns all the pages of the request at once.
     *
     * List the reactions to an [issue comment](https://docs.github.com/rest/issues/comments#get-an-issue-comment).
     *
     * FROM: <https://docs.github.com/rest/reactions/reactions#list-reactions-for-an-issue-comment>
     */
    pub async fn list_all_for_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        content: crate::types::Content,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/comments/{}/reactions?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
    /**
     * Create reaction for an issue comment.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/issues/comments/{comment_id}/reactions` endpoint.
     *
     * Create a reaction to an [issue comment](https://docs.github.com/rest/issues/comments#get-an-issue-comment). A response with an HTTP `200` status means that you already added the reaction type to this issue comment.
     *
     * FROM: <https://docs.github.com/rest/reactions/reactions#create-reaction-for-an-issue-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `comment_id: i64` -- The unique identifier of the comment.
     */
    pub async fn create_for_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &crate::types::ReactionsCreateIssueRequest,
    ) -> ClientResult<crate::Response<crate::types::Reaction>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/comments/{}/reactions",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * Delete an issue comment reaction.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/issues/comments/{comment_id}/reactions/{reaction_id}` endpoint.
     *
     * > [!NOTE]
     * > You can also specify a repository by `repository_id` using the route `DELETE delete /repositories/:repository_id/issues/comments/:comment_id/reactions/:reaction_id`.
     *
     * Delete a reaction to an [issue comment](https://docs.github.com/rest/issues/comments#get-an-issue-comment).
     *
     * FROM: <https://docs.github.com/rest/reactions/reactions#delete-an-issue-comment-reaction>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `comment_id: i64` -- The unique identifier of the comment.
     * * `reaction_id: i64` -- The unique identifier of the reaction.
     */
    pub async fn delete_for_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        reaction_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/comments/{}/reactions/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
                crate::progenitor_support::encode_path(&reaction_id.to_string()),
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
     * List reactions for an issue.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/reactions` endpoint.
     *
     * List the reactions to an [issue](https://docs.github.com/rest/issues/issues#get-an-issue).
     *
     * FROM: <https://docs.github.com/rest/reactions/reactions#list-reactions-for-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     * * `content: crate::types::Content` -- Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to an issue.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_for_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        content: crate::types::Content,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/reactions?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * List reactions for an issue.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/issues/{issue_number}/reactions` endpoint.
     *
     * As opposed to `list_for_issue`, this function returns all the pages of the request at once.
     *
     * List the reactions to an [issue](https://docs.github.com/rest/issues/issues#get-an-issue).
     *
     * FROM: <https://docs.github.com/rest/reactions/reactions#list-reactions-for-an-issue>
     */
    pub async fn list_all_for_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        content: crate::types::Content,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/reactions?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
    /**
     * Create reaction for an issue.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/issues/{issue_number}/reactions` endpoint.
     *
     * Create a reaction to an [issue](https://docs.github.com/rest/issues/issues#get-an-issue). A response with an HTTP `200` status means that you already added the reaction type to this issue.
     *
     * FROM: <https://docs.github.com/rest/reactions/reactions#create-reaction-for-an-issue>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     */
    pub async fn create_for_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        body: &crate::types::ReactionsCreateIssueRequest,
    ) -> ClientResult<crate::Response<crate::types::Reaction>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/reactions",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
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
     * Delete an issue reaction.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/issues/{issue_number}/reactions/{reaction_id}` endpoint.
     *
     * > [!NOTE]
     * > You can also specify a repository by `repository_id` using the route `DELETE /repositories/:repository_id/issues/:issue_number/reactions/:reaction_id`.
     *
     * Delete a reaction to an [issue](https://docs.github.com/rest/issues/issues#get-an-issue).
     *
     * FROM: <https://docs.github.com/rest/reactions/reactions#delete-an-issue-reaction>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `issue_number: i64` -- The number that identifies the issue.
     * * `reaction_id: i64` -- The unique identifier of the reaction.
     */
    pub async fn delete_for_issue(
        &self,
        owner: &str,
        repo: &str,
        issue_number: i64,
        reaction_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/issues/{}/reactions/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&issue_number.to_string()),
                crate::progenitor_support::encode_path(&reaction_id.to_string()),
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
     * List reactions for a pull request review comment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions` endpoint.
     *
     * List the reactions to a [pull request review comment](https://docs.github.com/rest/pulls/comments#get-a-review-comment-for-a-pull-request).
     *
     * FROM: <https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-pull-request-review-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `comment_id: i64` -- The unique identifier of the comment.
     * * `content: crate::types::Content` -- Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to a pull request review comment.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_for_pull_request_review_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        content: crate::types::Content,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/comments/{}/reactions?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * List reactions for a pull request review comment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions` endpoint.
     *
     * As opposed to `list_for_pull_request_review_comment`, this function returns all the pages of the request at once.
     *
     * List the reactions to a [pull request review comment](https://docs.github.com/rest/pulls/comments#get-a-review-comment-for-a-pull-request).
     *
     * FROM: <https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-pull-request-review-comment>
     */
    pub async fn list_all_for_pull_request_review_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        content: crate::types::Content,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/comments/{}/reactions?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
    /**
     * Create reaction for a pull request review comment.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions` endpoint.
     *
     * Create a reaction to a [pull request review comment](https://docs.github.com/rest/pulls/comments#get-a-review-comment-for-a-pull-request). A response with an HTTP `200` status means that you already added the reaction type to this pull request review comment.
     *
     * FROM: <https://docs.github.com/rest/reactions/reactions#create-reaction-for-a-pull-request-review-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `comment_id: i64` -- The unique identifier of the comment.
     */
    pub async fn create_for_pull_request_review_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &crate::types::ReactionsCreateIssueRequest,
    ) -> ClientResult<crate::Response<crate::types::Reaction>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/comments/{}/reactions",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
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
     * Delete a pull request comment reaction.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/pulls/comments/{comment_id}/reactions/{reaction_id}` endpoint.
     *
     * > [!NOTE]
     * > You can also specify a repository by `repository_id` using the route `DELETE /repositories/:repository_id/pulls/comments/:comment_id/reactions/:reaction_id.`
     *
     * Delete a reaction to a [pull request review comment](https://docs.github.com/rest/pulls/comments#get-a-review-comment-for-a-pull-request).
     *
     * FROM: <https://docs.github.com/rest/reactions/reactions#delete-a-pull-request-comment-reaction>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `comment_id: i64` -- The unique identifier of the comment.
     * * `reaction_id: i64` -- The unique identifier of the reaction.
     */
    pub async fn delete_for_pull_request_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        reaction_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/comments/{}/reactions/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&comment_id.to_string()),
                crate::progenitor_support::encode_path(&reaction_id.to_string()),
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
     * List reactions for a release.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/releases/{release_id}/reactions` endpoint.
     *
     * List the reactions to a [release](https://docs.github.com/rest/releases/releases#get-a-release).
     *
     * FROM: <https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-release>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `release_id: i64` -- The unique identifier of the release.
     * * `content: crate::types::ReactionsListReleaseContent` -- Returns a single [reaction type](https://docs.github.com/rest/reactions/reactions#about-reactions). Omit this parameter to list all reactions to a release.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_for_release(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
        content: crate::types::ReactionsListReleaseContent,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/{}/reactions?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&release_id.to_string()),
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
     * List reactions for a release.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/releases/{release_id}/reactions` endpoint.
     *
     * As opposed to `list_for_release`, this function returns all the pages of the request at once.
     *
     * List the reactions to a [release](https://docs.github.com/rest/releases/releases#get-a-release).
     *
     * FROM: <https://docs.github.com/rest/reactions/reactions#list-reactions-for-a-release>
     */
    pub async fn list_all_for_release(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
        content: crate::types::ReactionsListReleaseContent,
    ) -> ClientResult<crate::Response<Vec<crate::types::Reaction>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !content.to_string().is_empty() {
            query_args.push(("content".to_string(), content.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/{}/reactions?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&release_id.to_string()),
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
    /**
     * Create reaction for a release.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/releases/{release_id}/reactions` endpoint.
     *
     * Create a reaction to a [release](https://docs.github.com/rest/releases/releases#get-a-release). A response with a `Status: 200 OK` means that you already added the reaction type to this release.
     *
     * FROM: <https://docs.github.com/rest/reactions/reactions#create-reaction-for-a-release>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `release_id: i64` -- The unique identifier of the release.
     */
    pub async fn create_for_release(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
        body: &crate::types::ReactionsCreateReleaseRequest,
    ) -> ClientResult<crate::Response<crate::types::Reaction>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/{}/reactions",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&release_id.to_string()),
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
     * Delete a release reaction.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/releases/{release_id}/reactions/{reaction_id}` endpoint.
     *
     * > [!NOTE]
     * > You can also specify a repository by `repository_id` using the route `DELETE delete /repositories/:repository_id/releases/:release_id/reactions/:reaction_id`.
     *
     * Delete a reaction to a [release](https://docs.github.com/rest/releases/releases#get-a-release).
     *
     * FROM: <https://docs.github.com/rest/reactions/reactions#delete-a-release-reaction>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `release_id: i64` -- The unique identifier of the release.
     * * `reaction_id: i64` -- The unique identifier of the reaction.
     */
    pub async fn delete_for_release(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
        reaction_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/{}/reactions/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&release_id.to_string()),
                crate::progenitor_support::encode_path(&reaction_id.to_string()),
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
}
