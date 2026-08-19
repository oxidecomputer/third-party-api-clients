use crate::Client;
use crate::ClientResult;

pub struct Checks {
    pub client: Client,
}

impl Checks {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Checks { client }
    }

    /**
     * Create a check run.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/check-runs` endpoint.
     *
     * **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
     *
     * Creates a new check run for a specific commit in a repository. Your GitHub App must have the `checks:write` permission to create check runs.
     *
     * In a check suite, GitHub limits the number of check runs with the same name to 1000. Once these check runs exceed 1000, GitHub will start to automatically delete older check runs.
     *
     * FROM: <https://docs.github.com/rest/reference/checks#create-a-check-run>
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
        body: &crate::types::ChecksCreateRequest,
    ) -> ClientResult<crate::Response<crate::types::CheckRun>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/check-runs",
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
     * Get a check run.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/check-runs/{check_run_id}` endpoint.
     *
     * **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
     *
     * Gets a single check run using its `id`. GitHub Apps must have the `checks:read` permission on a private repository or pull access to a public repository to get check runs. OAuth Apps and authenticated users must have the `repo` scope to get check runs in a private repository.
     *
     * FROM: <https://docs.github.com/rest/reference/checks#get-a-check-run>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `check_run_id: i64` -- check_run_id parameter.
     */
    pub async fn get(
        &self,
        owner: &str,
        repo: &str,
        check_run_id: i64,
    ) -> ClientResult<crate::Response<crate::types::CheckRun>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/check-runs/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&check_run_id.to_string()),
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
     * Update a check run.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/check-runs/{check_run_id}` endpoint.
     *
     * **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
     *
     * Updates a check run for a specific commit in a repository. Your GitHub App must have the `checks:write` permission to edit check runs.
     *
     * FROM: <https://docs.github.com/rest/reference/checks#update-a-check-run>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `check_run_id: i64` -- check_run_id parameter.
     */
    pub async fn update(
        &self,
        owner: &str,
        repo: &str,
        check_run_id: i64,
        body: &crate::types::ChecksUpdateRequest,
    ) -> ClientResult<crate::Response<crate::types::CheckRun>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/check-runs/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&check_run_id.to_string()),
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
     * List check run annotations.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/check-runs/{check_run_id}/annotations` endpoint.
     *
     * Lists annotations for a check run using the annotation `id`. GitHub Apps must have the `checks:read` permission on a private repository or pull access to a public repository to get annotations for a check run. OAuth Apps and authenticated users must have the `repo` scope to get annotations for a check run in a private repository.
     *
     * FROM: <https://docs.github.com/rest/reference/checks#list-check-run-annotations>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `check_run_id: i64` -- check_run_id parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_annotations(
        &self,
        owner: &str,
        repo: &str,
        check_run_id: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::CheckAnnotation>>> {
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
                "/repos/{}/{}/check-runs/{}/annotations?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&check_run_id.to_string()),
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
     * List check run annotations.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/check-runs/{check_run_id}/annotations` endpoint.
     *
     * As opposed to `list_annotations`, this function returns all the pages of the request at once.
     *
     * Lists annotations for a check run using the annotation `id`. GitHub Apps must have the `checks:read` permission on a private repository or pull access to a public repository to get annotations for a check run. OAuth Apps and authenticated users must have the `repo` scope to get annotations for a check run in a private repository.
     *
     * FROM: <https://docs.github.com/rest/reference/checks#list-check-run-annotations>
     */
    pub async fn list_all_annotations(
        &self,
        owner: &str,
        repo: &str,
        check_run_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::CheckAnnotation>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/check-runs/{}/annotations",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&check_run_id.to_string()),
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
     * Create a check suite.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/check-suites` endpoint.
     *
     * **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array and a `null` value for `head_branch`.
     *
     * By default, check suites are automatically created when you create a [check run](https://docs.github.com/rest/reference/checks#check-runs). You only need to use this endpoint for manually creating check suites when you've disabled automatic creation using "[Update repository preferences for check suites](https://docs.github.com/rest/reference/checks#update-repository-preferences-for-check-suites)". Your GitHub App must have the `checks:write` permission to create check suites.
     *
     * FROM: <https://docs.github.com/rest/reference/checks#create-a-check-suite>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn create_suite(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ChecksCreateSuiteRequest,
    ) -> ClientResult<crate::Response<crate::types::CheckSuiteData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/check-suites",
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
     * Update repository preferences for check suites.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/check-suites/preferences` endpoint.
     *
     * Changes the default automatic flow when creating check suites. By default, a check suite is automatically created each time code is pushed to a repository. When you disable the automatic creation of check suites, you can manually [Create a check suite](https://docs.github.com/rest/reference/checks#create-a-check-suite). You must have admin permissions in the repository to set preferences for check suites.
     *
     * FROM: <https://docs.github.com/rest/reference/checks#update-repository-preferences-for-check-suites>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn set_suites_preferences(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::Preferences,
    ) -> ClientResult<crate::Response<crate::types::CheckSuitePreference>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/check-suites/preferences",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Get a check suite.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/check-suites/{check_suite_id}` endpoint.
     *
     * **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array and a `null` value for `head_branch`.
     *
     * Gets a single check suite using its `id`. GitHub Apps must have the `checks:read` permission on a private repository or pull access to a public repository to get check suites. OAuth Apps and authenticated users must have the `repo` scope to get check suites in a private repository.
     *
     * FROM: <https://docs.github.com/rest/reference/checks#get-a-check-suite>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `check_suite_id: i64` -- check_suite_id parameter.
     */
    pub async fn get_suite(
        &self,
        owner: &str,
        repo: &str,
        check_suite_id: i64,
    ) -> ClientResult<crate::Response<crate::types::CheckSuiteData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/check-suites/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&check_suite_id.to_string()),
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
     * List check runs in a check suite.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/check-suites/{check_suite_id}/check-runs` endpoint.
     *
     * **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
     *
     * Lists check runs for a check suite using its `id`. GitHub Apps must have the `checks:read` permission on a private repository or pull access to a public repository to get check runs. OAuth Apps and authenticated users must have the `repo` scope to get check runs in a private repository.
     *
     * FROM: <https://docs.github.com/rest/reference/checks#list-check-runs-in-a-check-suite>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `check_suite_id: i64` -- check_suite_id parameter.
     * * `check_name: &str` -- Returns check runs with the specified `name`.
     * * `status: crate::types::JobStatus` -- Returns check runs with the specified `status`. Can be one of `queued`, `in_progress`, or `completed`.
     * * `filter: crate::types::ActionsListJobsWorkflowRunFilter` -- Filters jobs by their `completed_at` timestamp. Can be one of:  
     *  \\* `latest`: Returns jobs from the most recent execution of the workflow run.  
     *  \\* `all`: Returns all jobs for a workflow run, including from old executions of the workflow run.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_suite(
        &self,
        owner: &str,
        repo: &str,
        check_suite_id: i64,
        check_name: &str,
        status: crate::types::JobStatus,
        filter: crate::types::ActionsListJobsWorkflowRunFilter,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ChecksListRefResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !check_name.is_empty() {
            query_args.push(("check_name".to_string(), check_name.to_string()));
        }
        if !filter.to_string().is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/check-suites/{}/check-runs?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&check_suite_id.to_string()),
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
     * Rerequest a check suite.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/check-suites/{check_suite_id}/rerequest` endpoint.
     *
     * Triggers GitHub to rerequest an existing check suite, without pushing new code to a repository. This endpoint will trigger the [`check_suite` webhook](https://docs.github.com/webhooks/event-payloads/#check_suite) event with the action `rerequested`. When a check suite is `rerequested`, its `status` is reset to `queued` and the `conclusion` is cleared.
     *
     * To rerequest a check suite, your GitHub App must have the `checks:read` permission on a private repository or pull access to a public repository.
     *
     * FROM: <https://docs.github.com/rest/reference/checks#rerequest-a-check-suite>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `check_suite_id: i64` -- check_suite_id parameter.
     */
    pub async fn rerequest_suite(
        &self,
        owner: &str,
        repo: &str,
        check_suite_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/check-suites/{}/rerequest",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&check_suite_id.to_string()),
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
     * List check runs for a Git reference.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/commits/{ref}/check-runs` endpoint.
     *
     * **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
     *
     * Lists check runs for a commit ref. The `ref` can be a SHA, branch name, or a tag name. GitHub Apps must have the `checks:read` permission on a private repository or pull access to a public repository to get check runs. OAuth Apps and authenticated users must have the `repo` scope to get check runs in a private repository.
     *
     * FROM: <https://docs.github.com/rest/reference/checks#list-check-runs-for-a-git-reference>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `ref_: &str` -- ref parameter.
     * * `check_name: &str` -- Returns check runs with the specified `name`.
     * * `status: crate::types::JobStatus` -- Returns check runs with the specified `status`. Can be one of `queued`, `in_progress`, or `completed`.
     * * `filter: crate::types::ActionsListJobsWorkflowRunFilter` -- Filters jobs by their `completed_at` timestamp. Can be one of:  
     *  \\* `latest`: Returns jobs from the most recent execution of the workflow run.  
     *  \\* `all`: Returns all jobs for a workflow run, including from old executions of the workflow run.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     * * `app_id: i64`
     */
    pub async fn list_for_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        check_name: &str,
        status: crate::types::JobStatus,
        filter: crate::types::ActionsListJobsWorkflowRunFilter,
        per_page: i64,
        page: i64,
        app_id: i64,
    ) -> ClientResult<crate::Response<crate::types::ChecksListRefResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if app_id > 0 {
            query_args.push(("app_id".to_string(), app_id.to_string()));
        }
        if !check_name.is_empty() {
            query_args.push(("check_name".to_string(), check_name.to_string()));
        }
        if !filter.to_string().is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/commits/{}/check-runs?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(ref_),
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
     * List check suites for a Git reference.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/commits/{ref}/check-suites` endpoint.
     *
     * **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array and a `null` value for `head_branch`.
     *
     * Lists check suites for a commit `ref`. The `ref` can be a SHA, branch name, or a tag name. GitHub Apps must have the `checks:read` permission on a private repository or pull access to a public repository to list check suites. OAuth Apps and authenticated users must have the `repo` scope to get check suites in a private repository.
     *
     * FROM: <https://docs.github.com/rest/reference/checks#list-check-suites-for-a-git-reference>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `ref_: &str` -- ref parameter.
     * * `app_id: i64` -- Filters check suites by GitHub App `id`.
     * * `check_name: &str` -- Returns check runs with the specified `name`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_suites_for_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        app_id: i64,
        check_name: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ChecksListSuitesRefResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if app_id > 0 {
            query_args.push(("app_id".to_string(), app_id.to_string()));
        }
        if !check_name.is_empty() {
            query_args.push(("check_name".to_string(), check_name.to_string()));
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
                "/repos/{}/{}/commits/{}/check-suites?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(ref_),
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
}
