use crate::Client;
use crate::ClientResult;

pub struct Pulls {
    pub client: Client,
}

impl Pulls {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Pulls { client }
    }

    /**
     * List pull requests.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls` endpoint.
     *
     * Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#list-pull-requests>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `state: crate::types::IssuesListState` -- Indicates the state of the issues to return. Can be either `open`, `closed`, or `all`.
     * * `head: &str` -- Filter pulls by head user or head organization and branch name in the format of `user:ref-name` or `organization:ref-name`. For example: `github:new-script-format` or `octocat:test-branch`.
     * * `base: &str` -- Filter pulls by base branch name. Example: `gh-pages`.
     * * `sort: crate::types::PullsListSort` -- What to sort results by. Can be either `created`, `updated`, `popularity` (comment count) or `long-running` (age, filtering by pulls updated in the last month).
     * * `direction: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list(
        &self,
        owner: &str,
        repo: &str,
        state: crate::types::IssuesListState,
        head: &str,
        base: &str,
        sort: crate::types::PullsListSort,
        direction: crate::types::Order,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::PullRequestSimple>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !base.is_empty() {
            query_args.push(("base".to_string(), base.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !head.is_empty() {
            query_args.push(("head".to_string(), head.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List pull requests.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#list-pull-requests>
     */
    pub async fn list_all(
        &self,
        owner: &str,
        repo: &str,
        state: crate::types::IssuesListState,
        head: &str,
        base: &str,
        sort: crate::types::PullsListSort,
        direction: crate::types::Order,
    ) -> ClientResult<crate::Response<Vec<crate::types::PullRequestSimple>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !base.is_empty() {
            query_args.push(("base".to_string(), base.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !head.is_empty() {
            query_args.push(("head".to_string(), head.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Create a pull request.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/pulls` endpoint.
     *
     * Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * To open or update a pull request in a public repository, you must have write access to the head or the source branch. For organization-owned repositories, you must be a member of the organization that owns the repository to open or update a pull request.
     *
     * You can create a new pull request.
     *
     * This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-rate-limits)" for details.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#create-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn create(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::PullsCreateRequest,
    ) -> ClientResult<crate::Response<crate::types::PullRequestData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List review comments in a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/comments` endpoint.
     *
     * Lists review comments for all pull requests in a repository. By default, review comments are in ascending order by ID.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#list-review-comments-in-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `sort: crate::types::PullsListReviewCommentsRepoSort`
     * * `direction: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_review_comments_for_repo(
        &self,
        owner: &str,
        repo: &str,
        sort: crate::types::PullsListReviewCommentsRepoSort,
        direction: crate::types::Order,
        since: Option<chrono::DateTime<chrono::Utc>>,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::PullRequestReviewComment>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/comments?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List review comments in a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/comments` endpoint.
     *
     * As opposed to `list_review_comments_for_repo`, this function returns all the pages of the request at once.
     *
     * Lists review comments for all pull requests in a repository. By default, review comments are in ascending order by ID.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#list-review-comments-in-a-repository>
     */
    pub async fn list_all_review_comments_for_repo(
        &self,
        owner: &str,
        repo: &str,
        sort: crate::types::PullsListReviewCommentsRepoSort,
        direction: crate::types::Order,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> ClientResult<crate::Response<Vec<crate::types::PullRequestReviewComment>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/comments?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Get a review comment for a pull request.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/comments/{comment_id}` endpoint.
     *
     * Provides details for a review comment.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#get-a-review-comment-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `comment_id: i64` -- comment_id parameter.
     */
    pub async fn get_review_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> ClientResult<crate::Response<crate::types::PullRequestReviewComment>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/comments/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Delete a review comment for a pull request.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/pulls/comments/{comment_id}` endpoint.
     *
     * Deletes a review comment.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#delete-a-review-comment-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `comment_id: i64` -- comment_id parameter.
     */
    pub async fn delete_review_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/comments/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Update a review comment for a pull request.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/pulls/comments/{comment_id}` endpoint.
     *
     * Enables you to edit a review comment.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#update-a-review-comment-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `comment_id: i64` -- comment_id parameter.
     */
    pub async fn update_review_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &crate::types::PullsUpdateReviewRequest,
    ) -> ClientResult<crate::Response<crate::types::PullRequestReviewComment>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/comments/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Get a pull request.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/{pull_number}` endpoint.
     *
     * Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Lists details of a pull request by providing its number.
     *
     * When you get, [create](https://docs.github.com/rest/reference/pulls/#create-a-pull-request), or [edit](https://docs.github.com/rest/reference/pulls#update-a-pull-request) a pull request, GitHub creates a merge commit to test whether the pull request can be automatically merged into the base branch. This test commit is not added to the base branch or the head branch. You can review the status of the test commit using the `mergeable` key. For more information, see "[Checking mergeability of pull requests](https://docs.github.com/rest/guides/getting-started-with-the-git-database-api#checking-mergeability-of-pull-requests)".
     *
     * The value of the `mergeable` attribute can be `true`, `false`, or `null`. If the value is `null`, then GitHub has started a background job to compute the mergeability. After giving the job time to complete, resubmit the request. When the job finishes, you will see a non-`null` value for the `mergeable` attribute in the response. If `mergeable` is `true`, then `merge_commit_sha` will be the SHA of the _test_ merge commit.
     *
     * The value of the `merge_commit_sha` attribute changes depending on the state of the pull request. Before merging a pull request, the `merge_commit_sha` attribute holds the SHA of the _test_ merge commit. After merging a pull request, the `merge_commit_sha` attribute changes depending on how you merged the pull request:
     *
     * *   If merged as a [merge commit](https://help.github.com/articles/about-merge-methods-on-github/), `merge_commit_sha` represents the SHA of the merge commit.
     * *   If merged via a [squash](https://help.github.com/articles/about-merge-methods-on-github/#squashing-your-merge-commits), `merge_commit_sha` represents the SHA of the squashed commit on the base branch.
     * *   If [rebased](https://help.github.com/articles/about-merge-methods-on-github/#rebasing-and-merging-your-commits), `merge_commit_sha` represents the commit that the base branch was updated to.
     *
     * Pass the appropriate [media type](https://docs.github.com/rest/overview/media-types/#commits-commit-comparison-and-pull-requests) to fetch diff and patch formats.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#get-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     */
    pub async fn get(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
    ) -> ClientResult<crate::Response<crate::types::PullRequestData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
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
     * Update a pull request.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/pulls/{pull_number}` endpoint.
     *
     * Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * To open or update a pull request in a public repository, you must have write access to the head or the source branch. For organization-owned repositories, you must be a member of the organization that owns the repository to open or update a pull request.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls/#update-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     */
    pub async fn update(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        body: &crate::types::PullsUpdateRequest,
    ) -> ClientResult<crate::Response<crate::types::PullRequestData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
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
     * List review comments on a pull request.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/{pull_number}/comments` endpoint.
     *
     * Lists all review comments for a pull request. By default, review comments are in ascending order by ID.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#list-review-comments-on-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     * * `sort: crate::types::Sort` -- One of `created` (when the repository was starred) or `updated` (when it was last pushed to).
     * * `direction: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_review_comments(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        sort: crate::types::Sort,
        direction: crate::types::Order,
        since: Option<chrono::DateTime<chrono::Utc>>,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::PullRequestReviewComment>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/comments?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
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
     * List review comments on a pull request.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/{pull_number}/comments` endpoint.
     *
     * As opposed to `list_review_comments`, this function returns all the pages of the request at once.
     *
     * Lists all review comments for a pull request. By default, review comments are in ascending order by ID.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#list-review-comments-on-a-pull-request>
     */
    pub async fn list_all_review_comments(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        sort: crate::types::Sort,
        direction: crate::types::Order,
        since: Option<chrono::DateTime<chrono::Utc>>,
    ) -> ClientResult<crate::Response<Vec<crate::types::PullRequestReviewComment>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/comments?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
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
     * Create a review comment for a pull request.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/pulls/{pull_number}/comments` endpoint.
     *
     *
     * Creates a review comment in the pull request diff. To add a regular comment to a pull request timeline, see "[Create an issue comment](https://docs.github.com/rest/reference/issues#create-an-issue-comment)." We recommend creating a review comment using `line`, `side`, and optionally `start_line` and `start_side` if your comment applies to more than one line in the pull request diff.
     *
     * You can still create a review comment using the `position` parameter. When you use `position`, the `line`, `side`, `start_line`, and `start_side` parameters are not required. For more information, see the [`comfort-fade` preview notice](https://docs.github.com/rest/reference/pulls#create-a-review-comment-for-a-pull-request-preview-notices).
     *
     * **Note:** The position value equals the number of lines down from the first "@@" hunk header in the file you want to add a comment. The line just below the "@@" line is position 1, the next line is position 2, and so on. The position in the diff continues to increase through lines of whitespace and additional hunks until the beginning of a new file.
     *
     * This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#create-a-review-comment-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     */
    pub async fn create_review_comment(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        body: &crate::types::PullsCreateReviewCommentRequest,
    ) -> ClientResult<crate::Response<crate::types::PullRequestReviewComment>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/comments",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
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
     * Create a reply for a review comment.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/pulls/{pull_number}/comments/{comment_id}/replies` endpoint.
     *
     * Creates a reply to a review comment for a pull request. For the `comment_id`, provide the ID of the review comment you are replying to. This must be the ID of a _top-level review comment_, not a reply to that comment. Replies to replies are not supported.
     *
     * This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#create-a-reply-for-a-review-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     * * `comment_id: i64` -- comment_id parameter.
     */
    pub async fn create_reply_for_review_comment(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        comment_id: i64,
        body: &crate::types::PullsUpdateReviewRequest,
    ) -> ClientResult<crate::Response<crate::types::PullRequestReviewComment>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/comments/{}/replies",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
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
     * List commits on a pull request.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/{pull_number}/commits` endpoint.
     *
     * Lists a maximum of 250 commits for a pull request. To receive a complete commit list for pull requests with more than 250 commits, use the [List commits](https://docs.github.com/rest/reference/repos#list-commits) endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#list-commits-on-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_commits(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::CommitDataType>>> {
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
                "/repos/{}/{}/pulls/{}/commits?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
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
     * List commits on a pull request.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/{pull_number}/commits` endpoint.
     *
     * As opposed to `list_commits`, this function returns all the pages of the request at once.
     *
     * Lists a maximum of 250 commits for a pull request. To receive a complete commit list for pull requests with more than 250 commits, use the [List commits](https://docs.github.com/rest/reference/repos#list-commits) endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#list-commits-on-a-pull-request>
     */
    pub async fn list_all_commits(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::CommitDataType>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/commits",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
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
     * List pull requests files.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/{pull_number}/files` endpoint.
     *
     * **Note:** Responses include a maximum of 3000 files. The paginated response returns 30 files per page by default.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#list-pull-requests-files>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_files(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::DiffEntry>>> {
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
                "/repos/{}/{}/pulls/{}/files?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
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
     * List pull requests files.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/{pull_number}/files` endpoint.
     *
     * As opposed to `list_files`, this function returns all the pages of the request at once.
     *
     * **Note:** Responses include a maximum of 3000 files. The paginated response returns 30 files per page by default.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#list-pull-requests-files>
     */
    pub async fn list_all_files(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::DiffEntry>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/files",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
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
     * Check if a pull request has been merged.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/{pull_number}/merge` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#check-if-a-pull-request-has-been-merged>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     */
    pub async fn check_if_merged(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/merge",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
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
     * Merge a pull request.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/pulls/{pull_number}/merge` endpoint.
     *
     * This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#merge-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     */
    pub async fn merge(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        body: &crate::types::PullsMergeRequest,
    ) -> ClientResult<crate::Response<crate::types::PullRequestMergeResult>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/merge",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * List requested reviewers for a pull request.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#list-requested-reviewers-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_requested_reviewers(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::PullRequestReview>> {
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
                "/repos/{}/{}/pulls/{}/requested_reviewers?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
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
     * Request reviewers for a pull request.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers` endpoint.
     *
     * This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#request-reviewers-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     */
    pub async fn request_reviewers(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        body: &crate::types::PullsRequestReviewers,
    ) -> ClientResult<crate::Response<crate::types::PullRequestSimple>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/requested_reviewers",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
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
     * Remove requested reviewers from a pull request.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#remove-requested-reviewers-from-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     */
    pub async fn remove_requested_reviewers(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        body: &crate::types::PullsRemoveRequestedReviewersRequest,
    ) -> ClientResult<crate::Response<crate::types::PullRequestSimple>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/requested_reviewers",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * List reviews for a pull request.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/{pull_number}/reviews` endpoint.
     *
     * The list of reviews returns in chronological order.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#list-reviews-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_reviews(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::PullRequestReviewData>>> {
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
                "/repos/{}/{}/pulls/{}/reviews?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
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
     * List reviews for a pull request.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/{pull_number}/reviews` endpoint.
     *
     * As opposed to `list_reviews`, this function returns all the pages of the request at once.
     *
     * The list of reviews returns in chronological order.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#list-reviews-for-a-pull-request>
     */
    pub async fn list_all_reviews(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::PullRequestReviewData>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/reviews",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
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
     * Create a review for a pull request.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/pulls/{pull_number}/reviews` endpoint.
     *
     * This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
     *
     * Pull request reviews created in the `PENDING` state do not include the `submitted_at` property in the response.
     *
     * **Note:** To comment on a specific line in a file, you need to first determine the _position_ of that line in the diff. The GitHub REST API v3 offers the `application/vnd.github.v3.diff` [media type](https://docs.github.com/rest/overview/media-types#commits-commit-comparison-and-pull-requests). To see a pull request diff, add this media type to the `Accept` header of a call to the [single pull request](https://docs.github.com/rest/reference/pulls#get-a-pull-request) endpoint.
     *
     * The `position` value equals the number of lines down from the first "@@" hunk header in the file you want to add a comment. The line just below the "@@" line is position 1, the next line is position 2, and so on. The position in the diff continues to increase through lines of whitespace and additional hunks until the beginning of a new file.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#create-a-review-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     */
    pub async fn create_review(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        body: &crate::types::PullsCreateReviewRequest,
    ) -> ClientResult<crate::Response<crate::types::PullRequestReviewData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/reviews",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
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
     * Get a review for a pull request.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#get-a-review-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     * * `review_id: i64` -- review_id parameter.
     */
    pub async fn get_review(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        review_id: i64,
    ) -> ClientResult<crate::Response<crate::types::PullRequestReviewData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/reviews/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
                crate::progenitor_support::encode_path(&review_id.to_string()),
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
     * Update a review for a pull request.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}` endpoint.
     *
     * Update the review summary comment with new text.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#update-a-review-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     * * `review_id: i64` -- review_id parameter.
     */
    pub async fn update_review(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        review_id: i64,
        body: &crate::types::PullsUpdateReviewRequest,
    ) -> ClientResult<crate::Response<crate::types::PullRequestReviewData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/reviews/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
                crate::progenitor_support::encode_path(&review_id.to_string()),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Delete a pending review for a pull request.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#delete-a-pending-review-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     * * `review_id: i64` -- review_id parameter.
     */
    pub async fn delete_pending_review(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        review_id: i64,
    ) -> ClientResult<crate::Response<crate::types::PullRequestReviewData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/reviews/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
                crate::progenitor_support::encode_path(&review_id.to_string()),
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
     * List comments for a pull request review.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/comments` endpoint.
     *
     * List comments for a specific pull request review.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#list-comments-for-a-pull-request-review>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     * * `review_id: i64` -- review_id parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_comments_for_review(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        review_id: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::ReviewComment>>> {
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
                "/repos/{}/{}/pulls/{}/reviews/{}/comments?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
                crate::progenitor_support::encode_path(&review_id.to_string()),
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
     * List comments for a pull request review.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/comments` endpoint.
     *
     * As opposed to `list_comments_for_review`, this function returns all the pages of the request at once.
     *
     * List comments for a specific pull request review.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#list-comments-for-a-pull-request-review>
     */
    pub async fn list_all_comments_for_review(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        review_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::ReviewComment>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/reviews/{}/comments",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
                crate::progenitor_support::encode_path(&review_id.to_string()),
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
     * Dismiss a review for a pull request.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/dismissals` endpoint.
     *
     * **Note:** To dismiss a pull request review on a [protected branch](https://docs.github.com/rest/reference/repos#branches), you must be a repository administrator or be included in the list of people or teams who can dismiss pull request reviews.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#dismiss-a-review-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     * * `review_id: i64` -- review_id parameter.
     */
    pub async fn dismiss_review(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        review_id: i64,
        body: &crate::types::PullsDismissReviewRequest,
    ) -> ClientResult<crate::Response<crate::types::PullRequestReviewData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/reviews/{}/dismissals",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
                crate::progenitor_support::encode_path(&review_id.to_string()),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Submit a review for a pull request.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/pulls/{pull_number}/reviews/{review_id}/events` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#submit-a-review-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     * * `review_id: i64` -- review_id parameter.
     */
    pub async fn submit_review(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        review_id: i64,
        body: &crate::types::PullsSubmitReviewRequest,
    ) -> ClientResult<crate::Response<crate::types::PullRequestReviewData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/reviews/{}/events",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
                crate::progenitor_support::encode_path(&review_id.to_string()),
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
     * Update a pull request branch.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/pulls/{pull_number}/update-branch` endpoint.
     *
     * Updates the pull request branch with the latest upstream changes by merging HEAD from the base branch into the pull request branch.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#update-a-pull-request-branch>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     */
    pub async fn update_branch(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        body: &crate::types::PullsUpdateBranchRequest,
    ) -> ClientResult<crate::Response<crate::types::PullsUpdateBranchResponse>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/update-branch",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
}
