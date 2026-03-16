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
     * Creates a new check run for a specific commit in a repository.
     *
     * To create a check run, you must use a GitHub App. OAuth apps and authenticated users are not able to create a check suite.
     *
     * In a check suite, GitHub limits the number of check runs with the same name to 1000. Once these check runs exceed 1000, GitHub will start to automatically delete older check runs.
     *
     * > [!NOTE]
     * > The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
     *
     * FROM: <https://docs.github.com/rest/checks/runs#create-a-check-run>
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
        body: &crate::types::ChecksCreateRequest,
    ) -> ClientResult<crate::Response<crate::types::CheckRun>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/check-runs",
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
     * Get a check run.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/check-runs/{check_run_id}` endpoint.
     *
     * Gets a single check run using its `id`.
     *
     * > [!NOTE]
     * > The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint on a private repository.
     *
     * FROM: <https://docs.github.com/rest/checks/runs#get-a-check-run>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `check_run_id: i64` -- The unique identifier of the check run.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Updates a check run for a specific commit in a repository.
     *
     * > [!NOTE]
     * > The endpoints to manage checks only look for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
     *
     * OAuth apps and personal access tokens (classic) cannot use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/checks/runs#update-a-check-run>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `check_run_id: i64` -- The unique identifier of the check run.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Lists annotations for a check run using the annotation `id`.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint on a private repository.
     *
     * FROM: <https://docs.github.com/rest/checks/runs#list-check-run-annotations>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `check_run_id: i64` -- The unique identifier of the check run.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Lists annotations for a check run using the annotation `id`.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint on a private repository.
     *
     * FROM: <https://docs.github.com/rest/checks/runs#list-check-run-annotations>
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Rerequest a check run.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/check-runs/{check_run_id}/rerequest` endpoint.
     *
     * Triggers GitHub to rerequest an existing check run, without pushing new code to a repository. This endpoint will trigger the [`check_run` webhook](https://docs.github.com/webhooks/event-payloads/#check_run) event with the action `rerequested`. When a check run is `rerequested`, the `status` of the check suite it belongs to is reset to `queued` and the `conclusion` is cleared. The check run itself is not updated. GitHub apps recieving the [`check_run` webhook](https://docs.github.com/webhooks/event-payloads/#check_run) with the `rerequested` action should then decide if the check run should be reset or updated and call the [update `check_run` endpoint](https://docs.github.com/rest/checks/runs#update-a-check-run) to update the check_run if desired.
     *
     * For more information about how to re-run GitHub Actions jobs, see "[Re-run a job from a workflow run](https://docs.github.com/rest/actions/workflow-runs#re-run-a-job-from-a-workflow-run)".
     *
     * FROM: <https://docs.github.com/rest/checks/runs#rerequest-a-check-run>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `check_run_id: i64` -- The unique identifier of the check run.
     */
    pub async fn rerequest_run(
        &self,
        owner: &str,
        repo: &str,
        check_run_id: i64,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/check-runs/{}/rerequest",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&check_run_id.to_string()),
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
     * Create a check suite.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/check-suites` endpoint.
     *
     * Creates a check suite manually. By default, check suites are automatically created when you create a [check run](https://docs.github.com/rest/checks/runs). You only need to use this endpoint for manually creating check suites when you've disabled automatic creation using "[Update repository preferences for check suites](https://docs.github.com/rest/checks/suites#update-repository-preferences-for-check-suites)".
     *
     * > [!NOTE]
     * > The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array and a `null` value for `head_branch`.
     *
     * OAuth apps and personal access tokens (classic) cannot use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/checks/suites#create-a-check-suite>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
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
     * Update repository preferences for check suites.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/check-suites/preferences` endpoint.
     *
     * Changes the default automatic flow when creating check suites. By default, a check suite is automatically created each time code is pushed to a repository. When you disable the automatic creation of check suites, you can manually [Create a check suite](https://docs.github.com/rest/checks/suites#create-a-check-suite).
     * You must have admin permissions in the repository to set preferences for check suites.
     *
     * FROM: <https://docs.github.com/rest/checks/suites#update-repository-preferences-for-check-suites>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Gets a single check suite using its `id`.
     *
     * > [!NOTE]
     * > The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array and a `null` value for `head_branch`.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint on a private repository.
     *
     * FROM: <https://docs.github.com/rest/checks/suites#get-a-check-suite>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `check_suite_id: i64` -- The unique identifier of the check suite.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Lists check runs for a check suite using its `id`.
     *
     * > [!NOTE]
     * > The endpoints to manage checks only look for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint on a private repository.
     *
     * FROM: <https://docs.github.com/rest/checks/runs#list-check-runs-in-a-check-suite>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `check_suite_id: i64` -- The unique identifier of the check suite.
     * * `check_name: &str` -- Returns check runs with the specified `name`.
     * * `status: crate::types::JobStepsStatus` -- The phase of the lifecycle that the job is currently in.
     * * `filter: crate::types::ActionsListJobsWorkflowRunFilter` -- Filters jobs by their `completed_at` timestamp. `latest` returns jobs from the most recent execution of the workflow run. `all` returns all jobs for a workflow run, including from old executions of the workflow run.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_for_suite(
        &self,
        owner: &str,
        repo: &str,
        check_suite_id: i64,
        check_name: &str,
        status: crate::types::JobStepsStatus,
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * FROM: <https://docs.github.com/rest/checks/suites#rerequest-a-check-suite>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `check_suite_id: i64` -- The unique identifier of the check suite.
     */
    pub async fn rerequest_suite(
        &self,
        owner: &str,
        repo: &str,
        check_suite_id: i64,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/check-suites/{}/rerequest",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Lists check runs for a commit ref. The `ref` can be a SHA, branch name, or a tag name.
     *
     * > [!NOTE]
     * > The endpoints to manage checks only look for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array.
     *
     * If there are more than 1000 check suites on a single git reference, this endpoint will limit check runs to the 1000 most recent check suites. To iterate over all possible check runs, use the [List check suites for a Git reference](https://docs.github.com/rest/reference/checks#list-check-suites-for-a-git-reference) endpoint and provide the `check_suite_id` parameter to the [List check runs in a check suite](https://docs.github.com/rest/reference/checks#list-check-runs-in-a-check-suite) endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint on a private repository.
     *
     * FROM: <https://docs.github.com/rest/checks/runs#list-check-runs-for-a-git-reference>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `ref_: &str` -- The commit reference. Can be a commit SHA, branch name (`heads/BRANCH_NAME`), or tag name (`tags/TAG_NAME`). For more information, see "[Git References](https://git-scm.com/book/en/v2/Git-Internals-Git-References)" in the Git documentation.
     * * `check_name: &str` -- Returns check runs with the specified `name`.
     * * `status: crate::types::JobStepsStatus` -- The phase of the lifecycle that the job is currently in.
     * * `filter: crate::types::ActionsListJobsWorkflowRunFilter` -- Filters jobs by their `completed_at` timestamp. `latest` returns jobs from the most recent execution of the workflow run. `all` returns all jobs for a workflow run, including from old executions of the workflow run.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `app_id: i64`
     */
    pub async fn list_for_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        check_name: &str,
        status: crate::types::JobStepsStatus,
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&ref_.to_string()),
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
     * Lists check suites for a commit `ref`. The `ref` can be a SHA, branch name, or a tag name.
     *
     * > [!NOTE]
     * > The endpoints to manage checks only look for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array and a `null` value for `head_branch`.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint on a private repository.
     *
     * FROM: <https://docs.github.com/rest/checks/suites#list-check-suites-for-a-git-reference>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `ref_: &str` -- The commit reference. Can be a commit SHA, branch name (`heads/BRANCH_NAME`), or tag name (`tags/TAG_NAME`). For more information, see "[Git References](https://git-scm.com/book/en/v2/Git-Internals-Git-References)" in the Git documentation.
     * * `app_id: i64` -- Filters check suites by GitHub App `id`.
     * * `check_name: &str` -- Returns check runs with the specified `name`.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&ref_.to_string()),
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
