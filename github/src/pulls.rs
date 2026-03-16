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
     * Lists pull requests in a specified repository.
     *
     * Draft pull requests are available in public repositories with GitHub
     * Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing
     * plans, and in public and private repositories with GitHub Team and GitHub Enterprise
     * Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)
     * in the GitHub Help documentation.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/pulls#list-pull-requests>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `state: crate::types::IssuesListState` -- Either `open`, `closed`, or `all` to filter by state.
     * * `head: &str` -- Filter pulls by head user or head organization and branch name in the format of `user:ref-name` or `organization:ref-name`. For example: `github:new-script-format` or `octocat:test-branch`.
     * * `base: &str` -- Filter pulls by base branch name. Example: `gh-pages`.
     * * `sort: crate::types::PullsListSort` -- What to sort results by. `popularity` will sort by the number of comments. `long-running` will sort by date created and will limit the results to pull requests that have been open for more than a month and have had activity within the past month.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Lists pull requests in a specified repository.
     *
     * Draft pull requests are available in public repositories with GitHub
     * Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing
     * plans, and in public and private repositories with GitHub Team and GitHub Enterprise
     * Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)
     * in the GitHub Help documentation.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/pulls#list-pull-requests>
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * To open or update a pull request in a public repository, you must have write access to the head or the source branch. For organization-owned repositories, you must be a member of the organization that owns the repository to open or update a pull request.
     *
     * This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)" and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/pulls#create-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::PullsCreateRequest,
    ) -> ClientResult<crate::Response<crate::types::PullRequestDataType>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Lists review comments for all pull requests in a repository. By default,
     * review comments are in ascending order by ID.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/comments#list-review-comments-in-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `sort: crate::types::PullsListReviewCommentsRepoSort`
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Lists review comments for all pull requests in a repository. By default,
     * review comments are in ascending order by ID.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/comments#list-review-comments-in-a-repository>
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Provides details for a specified review comment.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/comments#get-a-review-comment-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `comment_id: i64` -- The unique identifier of the comment.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * FROM: <https://docs.github.com/rest/pulls/comments#delete-a-review-comment-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `comment_id: i64` -- The unique identifier of the comment.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Edits the content of a specified review comment.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/comments#update-a-review-comment-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `comment_id: i64` -- The unique identifier of the comment.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Lists details of a pull request by providing its number.
     *
     * When you get, [create](https://docs.github.com/rest/pulls/pulls/#create-a-pull-request), or [edit](https://docs.github.com/rest/pulls/pulls#update-a-pull-request) a pull request, GitHub creates a merge commit to test whether the pull request can be automatically merged into the base branch. This test commit is not added to the base branch or the head branch. You can review the status of the test commit using the `mergeable` key. For more information, see "[Checking mergeability of pull requests](https://docs.github.com/rest/guides/getting-started-with-the-git-database-api#checking-mergeability-of-pull-requests)".
     *
     * The value of the `mergeable` attribute can be `true`, `false`, or `null`. If the value is `null`, then GitHub has started a background job to compute the mergeability. After giving the job time to complete, resubmit the request. When the job finishes, you will see a non-`null` value for the `mergeable` attribute in the response. If `mergeable` is `true`, then `merge_commit_sha` will be the SHA of the _test_ merge commit.
     *
     * The value of the `merge_commit_sha` attribute changes depending on the state of the pull request. Before merging a pull request, the `merge_commit_sha` attribute holds the SHA of the _test_ merge commit. After merging a pull request, the `merge_commit_sha` attribute changes depending on how you merged the pull request:
     *
     * *   If merged as a [merge commit](https://docs.github.com/articles/about-merge-methods-on-github/), `merge_commit_sha` represents the SHA of the merge commit.
     * *   If merged via a [squash](https://docs.github.com/articles/about-merge-methods-on-github/#squashing-your-merge-commits), `merge_commit_sha` represents the SHA of the squashed commit on the base branch.
     * *   If [rebased](https://docs.github.com/articles/about-merge-methods-on-github/#rebasing-and-merging-your-commits), `merge_commit_sha` represents the commit that the base branch was updated to.
     *
     * Pass the appropriate [media type](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types) to fetch diff and patch formats.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     * - **`application/vnd.github.diff`**: For more information, see "[git-diff](https://git-scm.com/docs/git-diff)" in the Git documentation. If a diff is corrupt, contact us through the [GitHub Support portal](https://support.github.com/). Include the repository name and pull request ID in your message.
     *
     * FROM: <https://docs.github.com/rest/pulls/pulls#get-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
     */
    pub async fn get(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
    ) -> ClientResult<crate::Response<crate::types::PullRequestDataType>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Draft pull requests are available in public repositories with GitHub Free and GitHub Free for organizations, GitHub Pro, and legacy per-repository billing plans, and in public and private repositories with GitHub Team and GitHub Enterprise Cloud. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * To open or update a pull request in a public repository, you must have write access to the head or the source branch. For organization-owned repositories, you must be a member of the organization that owns the repository to open or update a pull request.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/pulls#update-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
     */
    pub async fn update(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        body: &crate::types::PullsUpdateRequest,
    ) -> ClientResult<crate::Response<crate::types::PullRequestDataType>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Lists all review comments for a specified pull request. By default, review comments
     * are in ascending order by ID.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/comments#list-review-comments-on-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
     * * `sort: crate::types::SortData` -- The property to sort the results by.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_review_comments(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        sort: crate::types::SortData,
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Lists all review comments for a specified pull request. By default, review comments
     * are in ascending order by ID.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/comments#list-review-comments-on-a-pull-request>
     */
    pub async fn list_all_review_comments(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        sort: crate::types::SortData,
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Creates a review comment on the diff of a specified pull request. To add a regular comment to a pull request timeline, see "[Create an issue comment](https://docs.github.com/rest/issues/comments#create-an-issue-comment)."
     *
     * If your comment applies to more than one line in the pull request diff, you should use the parameters `line`, `side`, and optionally `start_line` and `start_side` in your request.
     *
     * The `position` parameter is closing down. If you use `position`, the `line`, `side`, `start_line`, and `start_side` parameters are not required.
     *
     * This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)"
     * and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/comments#create-a-review-comment-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)"
     * and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/comments#create-a-reply-for-a-review-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
     * * `comment_id: i64` -- The unique identifier of the comment.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Lists a maximum of 250 commits for a pull request. To receive a complete
     * commit list for pull requests with more than 250 commits, use the [List commits](https://docs.github.com/rest/commits/commits#list-commits)
     * endpoint.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/pulls#list-commits-on-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Lists a maximum of 250 commits for a pull request. To receive a complete
     * commit list for pull requests with more than 250 commits, use the [List commits](https://docs.github.com/rest/commits/commits#list-commits)
     * endpoint.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/pulls#list-commits-on-a-pull-request>
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Lists the files in a specified pull request.
     *
     * > [!NOTE]
     * > Responses include a maximum of 3000 files. The paginated response returns 30 files per page by default.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/pulls#list-pull-requests-files>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Lists the files in a specified pull request.
     *
     * > [!NOTE]
     * > Responses include a maximum of 3000 files. The paginated response returns 30 files per page by default.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/pulls#list-pull-requests-files>
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Checks if a pull request has been merged into the base branch. The HTTP status of the response indicates whether or not the pull request has been merged; the response body is empty.
     *
     * FROM: <https://docs.github.com/rest/pulls/pulls#check-if-a-pull-request-has-been-merged>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Merges a pull request into the base branch.
     * This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)" and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
     *
     * FROM: <https://docs.github.com/rest/pulls/pulls#merge-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Get all requested reviewers for a pull request.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers` endpoint.
     *
     * Gets the users or teams whose review is requested for a pull request. Once a requested reviewer submits a review, they are no longer considered a requested reviewer. Their review will instead be returned by the [List reviews for a pull request](https://docs.github.com/rest/pulls/reviews#list-reviews-for-a-pull-request) operation.
     *
     * FROM: <https://docs.github.com/rest/pulls/review-requests#get-all-requested-reviewers-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
     */
    pub async fn list_requested_reviewers(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
    ) -> ClientResult<crate::Response<crate::types::PullRequestReview>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/requested_reviewers",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Request reviewers for a pull request.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/pulls/{pull_number}/requested_reviewers` endpoint.
     *
     * Requests reviews for a pull request from a given set of users and/or teams.
     * This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)" and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
     *
     * FROM: <https://docs.github.com/rest/pulls/review-requests#request-reviewers-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Removes review requests from a pull request for a given set of users and/or teams.
     *
     * FROM: <https://docs.github.com/rest/pulls/review-requests#remove-requested-reviewers-from-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
     */
    pub async fn remove_requested_reviewers(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        body: &crate::types::PullsRequestReviewers,
    ) -> ClientResult<crate::Response<crate::types::PullRequestSimple>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/requested_reviewers",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Lists all reviews for a specified pull request. The list of reviews returns in chronological order.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/reviews#list-reviews-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Lists all reviews for a specified pull request. The list of reviews returns in chronological order.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/reviews#list-reviews-for-a-pull-request>
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Creates a review on a specified pull request.
     *
     * This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)" and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
     *
     * Pull request reviews created in the `PENDING` state are not submitted and therefore do not include the `submitted_at` property in the response. To create a pending review for a pull request, leave the `event` parameter blank. For more information about submitting a `PENDING` review, see "[Submit a review for a pull request](https://docs.github.com/rest/pulls/reviews#submit-a-review-for-a-pull-request)."
     *
     * > [!NOTE]
     * > To comment on a specific line in a file, you need to first determine the position of that line in the diff. To see a pull request diff, add the `application/vnd.github.v3.diff` media type to the `Accept` header of a call to the [Get a pull request](https://docs.github.com/rest/pulls/pulls#get-a-pull-request) endpoint.
     *
     * The `position` value equals the number of lines down from the first "@@" hunk header in the file you want to add a comment. The line just below the "@@" line is position 1, the next line is position 2, and so on. The position in the diff continues to increase through lines of whitespace and additional hunks until the beginning of a new file.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/reviews#create-a-review-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Retrieves a pull request review by its ID.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/reviews#get-a-review-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
     * * `review_id: i64` -- The unique identifier of the review.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Updates the contents of a specified review summary comment.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/reviews#update-a-review-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
     * * `review_id: i64` -- The unique identifier of the review.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Deletes a pull request review that has not been submitted. Submitted reviews cannot be deleted.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/reviews#delete-a-pending-review-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
     * * `review_id: i64` -- The unique identifier of the review.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Lists comments for a specific pull request review.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/reviews#list-comments-for-a-pull-request-review>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
     * * `review_id: i64` -- The unique identifier of the review.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Lists comments for a specific pull request review.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/reviews#list-comments-for-a-pull-request-review>
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Dismisses a specified review on a pull request.
     *
     * > [!NOTE]
     * > To dismiss a pull request review on a [protected branch](https://docs.github.com/rest/branches/branch-protection), you must be a repository administrator or be included in the list of people or teams who can dismiss pull request reviews.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/reviews#dismiss-a-review-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
     * * `review_id: i64` -- The unique identifier of the review.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Submits a pending review for a pull request. For more information about creating a pending review for a pull request, see "[Create a review for a pull request](https://docs.github.com/rest/pulls/reviews#create-a-review-for-a-pull-request)."
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/pulls/reviews#submit-a-review-for-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
     * * `review_id: i64` -- The unique identifier of the review.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Note: If making a request on behalf of a GitHub App you must also have permissions to write the contents of the head repository.
     *
     * FROM: <https://docs.github.com/rest/pulls/pulls#update-a-pull-request-branch>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
