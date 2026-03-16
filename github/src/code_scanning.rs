use crate::Client;
use crate::ClientResult;

pub struct CodeScanning {
    pub client: Client,
}

impl CodeScanning {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CodeScanning { client }
    }

    /**
     * List code scanning alerts for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/code-scanning/alerts` endpoint.
     *
     * Lists code scanning alerts for the default branch for all eligible repositories in an organization. Eligible repositories are repositories that are owned by organizations that you own or for which you are a security manager. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
     *
     * The authenticated user must be an owner or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` or `repo`s cope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#list-code-scanning-alerts-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `tool_name: &str` -- The name of a code scanning tool. Only results by this tool will be listed. You can specify the tool by using either `tool_name` or `tool_guid`, but not both.
     * * `tool_guid: &str` -- The GUID of a code scanning tool. Only results by this tool will be listed. Note that some code scanning tools may not include a GUID in their analysis data. You can specify the tool by using either `tool_guid` or `tool_name`, but not both.
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `state: crate::types::CodeScanningAlertStateQuery` -- If specified, only code scanning alerts with this state will be returned.
     * * `sort: crate::types::SortData` -- The property by which to sort the results.
     * * `severity: crate::types::CodeScanningAlertSeverity` -- If specified, only code scanning alerts with this severity will be returned.
     * * `assignees: &str` -- Filter alerts by assignees. Provide a comma-separated list of user handles (e.g., `octocat` or `octocat,hubot`).
     *   Use `*` to list alerts with at least one assignee or `none` to list alerts with no assignees.
     *   .
     */
    pub async fn list_alerts_for_org(
        &self,
        org: &str,
        tool_name: &str,
        tool_guid: &str,
        before: &str,
        after: &str,
        page: i64,
        per_page: i64,
        direction: crate::types::Order,
        state: crate::types::CodeScanningAlertStateQuery,
        sort: crate::types::SortData,
        severity: crate::types::CodeScanningAlertSeverity,
        assignees: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeScanningOrganizationAlertItems>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !assignees.is_empty() {
            query_args.push(("assignees".to_string(), assignees.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !severity.to_string().is_empty() {
            query_args.push(("severity".to_string(), severity.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        if !tool_guid.is_empty() {
            query_args.push(("tool_guid".to_string(), tool_guid.to_string()));
        }
        if !tool_name.is_empty() {
            query_args.push(("tool_name".to_string(), tool_name.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/code-scanning/alerts?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List code scanning alerts for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/code-scanning/alerts` endpoint.
     *
     * As opposed to `list_alerts_for_org`, this function returns all the pages of the request at once.
     *
     * Lists code scanning alerts for the default branch for all eligible repositories in an organization. Eligible repositories are repositories that are owned by organizations that you own or for which you are a security manager. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
     *
     * The authenticated user must be an owner or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` or `repo`s cope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#list-code-scanning-alerts-for-an-organization>
     */
    pub async fn list_all_alerts_for_org(
        &self,
        org: &str,
        tool_name: &str,
        tool_guid: &str,
        before: &str,
        after: &str,
        direction: crate::types::Order,
        state: crate::types::CodeScanningAlertStateQuery,
        sort: crate::types::SortData,
        severity: crate::types::CodeScanningAlertSeverity,
        assignees: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeScanningOrganizationAlertItems>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !assignees.is_empty() {
            query_args.push(("assignees".to_string(), assignees.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !severity.to_string().is_empty() {
            query_args.push(("severity".to_string(), severity.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        if !tool_guid.is_empty() {
            query_args.push(("tool_guid".to_string(), tool_guid.to_string()));
        }
        if !tool_name.is_empty() {
            query_args.push(("tool_name".to_string(), tool_name.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/code-scanning/alerts?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List code scanning alerts for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/alerts` endpoint.
     *
     * Lists code scanning alerts.
     *
     * The response includes a `most_recent_instance` object.
     * This provides details of the most recent instance of this alert
     * for the default branch (or for the specified Git reference if you used `ref` in the request).
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#list-code-scanning-alerts-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `tool_name: &str` -- The name of a code scanning tool. Only results by this tool will be listed. You can specify the tool by using either `tool_name` or `tool_guid`, but not both.
     * * `tool_guid: &str` -- The GUID of a code scanning tool. Only results by this tool will be listed. Note that some code scanning tools may not include a GUID in their analysis data. You can specify the tool by using either `tool_guid` or `tool_name`, but not both.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `ref_: &str` -- The Git reference for the results you want to list. The `ref` for a branch can be formatted either as `refs/heads/<branch name>` or simply `<branch name>`. To reference a pull request use `refs/pull/<number>/merge`.
     * * `pr: i64` -- The number of the pull request for the results you want to list.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `sort: crate::types::SortData` -- The property by which to sort the results.
     * * `state: crate::types::CodeScanningAlertStateQuery` -- If specified, only code scanning alerts with this state will be returned.
     * * `severity: crate::types::CodeScanningAlertSeverity` -- If specified, only code scanning alerts with this severity will be returned.
     * * `assignees: &str` -- Filter alerts by assignees. Provide a comma-separated list of user handles (e.g., `octocat` or `octocat,hubot`).
     *   Use `*` to list alerts with at least one assignee or `none` to list alerts with no assignees.
     *   .
     */
    pub async fn list_alerts_for_repo(
        &self,
        owner: &str,
        repo: &str,
        tool_name: &str,
        tool_guid: &str,
        page: i64,
        per_page: i64,
        ref_: &str,
        pr: i64,
        direction: crate::types::Order,
        before: &str,
        after: &str,
        sort: crate::types::SortData,
        state: crate::types::CodeScanningAlertStateQuery,
        severity: crate::types::CodeScanningAlertSeverity,
        assignees: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeScanningAlertItems>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !assignees.is_empty() {
            query_args.push(("assignees".to_string(), assignees.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if pr > 0 {
            query_args.push(("pr".to_string(), pr.to_string()));
        }
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        if !severity.to_string().is_empty() {
            query_args.push(("severity".to_string(), severity.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        if !tool_guid.is_empty() {
            query_args.push(("tool_guid".to_string(), tool_guid.to_string()));
        }
        if !tool_name.is_empty() {
            query_args.push(("tool_name".to_string(), tool_name.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/alerts?{}",
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
     * List code scanning alerts for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/alerts` endpoint.
     *
     * As opposed to `list_alerts_for_repo`, this function returns all the pages of the request at once.
     *
     * Lists code scanning alerts.
     *
     * The response includes a `most_recent_instance` object.
     * This provides details of the most recent instance of this alert
     * for the default branch (or for the specified Git reference if you used `ref` in the request).
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#list-code-scanning-alerts-for-a-repository>
     */
    pub async fn list_all_alerts_for_repo(
        &self,
        owner: &str,
        repo: &str,
        tool_name: &str,
        tool_guid: &str,
        ref_: &str,
        pr: i64,
        direction: crate::types::Order,
        before: &str,
        after: &str,
        sort: crate::types::SortData,
        state: crate::types::CodeScanningAlertStateQuery,
        severity: crate::types::CodeScanningAlertSeverity,
        assignees: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeScanningAlertItems>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !assignees.is_empty() {
            query_args.push(("assignees".to_string(), assignees.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if pr > 0 {
            query_args.push(("pr".to_string(), pr.to_string()));
        }
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        if !severity.to_string().is_empty() {
            query_args.push(("severity".to_string(), severity.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        if !tool_guid.is_empty() {
            query_args.push(("tool_guid".to_string(), tool_guid.to_string()));
        }
        if !tool_name.is_empty() {
            query_args.push(("tool_name".to_string(), tool_name.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/alerts?{}",
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
     * Get a code scanning alert.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/alerts/{alert_number}` endpoint.
     *
     * Gets a single code scanning alert.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#get-a-code-scanning-alert>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `alert_number: i64` -- The number that identifies an alert. You can find this at the end of the URL for a code scanning alert within GitHub, and in the `number` field in the response from the `GET /repos/{owner}/{repo}/code-scanning/alerts` operation.
     */
    pub async fn get_alert(
        &self,
        owner: &str,
        repo: &str,
        alert_number: i64,
    ) -> ClientResult<crate::Response<crate::types::CodeScanningAlert>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/alerts/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&alert_number.to_string()),
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
     * Update a code scanning alert.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/code-scanning/alerts/{alert_number}` endpoint.
     *
     * Updates the status of a single code scanning alert.
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#update-a-code-scanning-alert>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `alert_number: i64` -- The number that identifies an alert. You can find this at the end of the URL for a code scanning alert within GitHub, and in the `number` field in the response from the `GET /repos/{owner}/{repo}/code-scanning/alerts` operation.
     */
    pub async fn update_alert(
        &self,
        owner: &str,
        repo: &str,
        alert_number: i64,
        body: &crate::types::CodeScanningUpdateAlertRequest,
    ) -> ClientResult<crate::Response<crate::types::CodeScanningAlert>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/alerts/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&alert_number.to_string()),
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
     * Get the status of an autofix for a code scanning alert.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/alerts/{alert_number}/autofix` endpoint.
     *
     * Gets the status and description of an autofix for a code scanning alert.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#get-the-status-of-an-autofix-for-a-code-scanning-alert>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `alert_number: i64` -- The number that identifies an alert. You can find this at the end of the URL for a code scanning alert within GitHub, and in the `number` field in the response from the `GET /repos/{owner}/{repo}/code-scanning/alerts` operation.
     */
    pub async fn get_autofix(
        &self,
        owner: &str,
        repo: &str,
        alert_number: i64,
    ) -> ClientResult<crate::Response<crate::types::CodeScanningAutofix>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/alerts/{}/autofix",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&alert_number.to_string()),
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
     * Create an autofix for a code scanning alert.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/code-scanning/alerts/{alert_number}/autofix` endpoint.
     *
     * Creates an autofix for a code scanning alert.
     *
     * If a new autofix is to be created as a result of this request or is currently being generated, then this endpoint will return a 202 Accepted response.
     *
     * If an autofix already exists for a given alert, then this endpoint will return a 200 OK response.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#create-an-autofix-for-a-code-scanning-alert>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `alert_number: i64` -- The number that identifies an alert. You can find this at the end of the URL for a code scanning alert within GitHub, and in the `number` field in the response from the `GET /repos/{owner}/{repo}/code-scanning/alerts` operation.
     */
    pub async fn create_autofix(
        &self,
        owner: &str,
        repo: &str,
        alert_number: i64,
    ) -> ClientResult<crate::Response<crate::types::CodeScanningAutofix>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/alerts/{}/autofix",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&alert_number.to_string()),
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
     * Commit an autofix for a code scanning alert.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/code-scanning/alerts/{alert_number}/autofix/commits` endpoint.
     *
     * Commits an autofix for a code scanning alert.
     *
     * If an autofix is committed as a result of this request, then this endpoint will return a 201 Created response.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#commit-an-autofix-for-a-code-scanning-alert>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `alert_number: i64` -- The number that identifies an alert. You can find this at the end of the URL for a code scanning alert within GitHub, and in the `number` field in the response from the `GET /repos/{owner}/{repo}/code-scanning/alerts` operation.
     */
    pub async fn commit_autofix(
        &self,
        owner: &str,
        repo: &str,
        alert_number: i64,
        body: &crate::types::CodeScanningAutofixCommits,
    ) -> ClientResult<crate::Response<crate::types::CodeScanningAutofixCommitsResponse>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/alerts/{}/autofix/commits",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&alert_number.to_string()),
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
     * List instances of a code scanning alert.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/alerts/{alert_number}/instances` endpoint.
     *
     * Lists all instances of the specified code scanning alert.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#list-instances-of-a-code-scanning-alert>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `alert_number: i64` -- The number that identifies an alert. You can find this at the end of the URL for a code scanning alert within GitHub, and in the `number` field in the response from the `GET /repos/{owner}/{repo}/code-scanning/alerts` operation.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `ref_: &str` -- The Git reference for the results you want to list. The `ref` for a branch can be formatted either as `refs/heads/<branch name>` or simply `<branch name>`. To reference a pull request use `refs/pull/<number>/merge`.
     * * `pr: i64` -- The number of the pull request for the results you want to list.
     */
    pub async fn list_alert_instances(
        &self,
        owner: &str,
        repo: &str,
        alert_number: i64,
        page: i64,
        per_page: i64,
        ref_: &str,
        pr: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeScanningAlertInstanceList>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if pr > 0 {
            query_args.push(("pr".to_string(), pr.to_string()));
        }
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/alerts/{}/instances?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&alert_number.to_string()),
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
     * List instances of a code scanning alert.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/alerts/{alert_number}/instances` endpoint.
     *
     * As opposed to `list_alert_instances`, this function returns all the pages of the request at once.
     *
     * Lists all instances of the specified code scanning alert.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#list-instances-of-a-code-scanning-alert>
     */
    pub async fn list_all_alert_instances(
        &self,
        owner: &str,
        repo: &str,
        alert_number: i64,
        ref_: &str,
        pr: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeScanningAlertInstanceList>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if pr > 0 {
            query_args.push(("pr".to_string(), pr.to_string()));
        }
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/alerts/{}/instances?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&alert_number.to_string()),
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
     * List code scanning analyses for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/analyses` endpoint.
     *
     * Lists the details of all code scanning analyses for a repository,
     * starting with the most recent.
     * The response is paginated and you can use the `page` and `per_page` parameters
     * to list the analyses you're interested in.
     * By default 30 analyses are listed per page.
     *
     * The `rules_count` field in the response give the number of rules
     * that were run in the analysis.
     * For very old analyses this data is not available,
     * and `0` is returned in this field.
     *
     * > [!WARNING]
     * > **Closing down notice:** The `tool_name` field is closing down and will, in future, not be included in the response for this endpoint. The example response reflects this change. The tool name can now be found inside the `tool` field.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#list-code-scanning-analyses-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `tool_name: &str` -- The name of a code scanning tool. Only results by this tool will be listed. You can specify the tool by using either `tool_name` or `tool_guid`, but not both.
     * * `tool_guid: &str` -- The GUID of a code scanning tool. Only results by this tool will be listed. Note that some code scanning tools may not include a GUID in their analysis data. You can specify the tool by using either `tool_guid` or `tool_name`, but not both.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `pr: i64` -- The number of the pull request for the results you want to list.
     * * `ref_: &str` -- The Git reference for the analyses you want to list. The `ref` for a branch can be formatted either as `refs/heads/<branch name>` or simply `<branch name>`. To reference a pull request use `refs/pull/<number>/merge`.
     * * `sarif_id: &str` -- Filter analyses belonging to the same SARIF upload.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `sort: crate::types::CodeScanningListRecentAnalysesSort` -- The property by which to sort the results.
     */
    pub async fn list_recent_analyses(
        &self,
        owner: &str,
        repo: &str,
        tool_name: &str,
        tool_guid: &str,
        page: i64,
        per_page: i64,
        pr: i64,
        ref_: &str,
        sarif_id: &str,
        direction: crate::types::Order,
        sort: crate::types::CodeScanningListRecentAnalysesSort,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeScanningAnalysis>>> {
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
        if pr > 0 {
            query_args.push(("pr".to_string(), pr.to_string()));
        }
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        if !sarif_id.is_empty() {
            query_args.push(("sarif_id".to_string(), sarif_id.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !tool_guid.is_empty() {
            query_args.push(("tool_guid".to_string(), tool_guid.to_string()));
        }
        if !tool_name.is_empty() {
            query_args.push(("tool_name".to_string(), tool_name.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/analyses?{}",
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
     * List code scanning analyses for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/analyses` endpoint.
     *
     * As opposed to `list_recent_analyses`, this function returns all the pages of the request at once.
     *
     * Lists the details of all code scanning analyses for a repository,
     * starting with the most recent.
     * The response is paginated and you can use the `page` and `per_page` parameters
     * to list the analyses you're interested in.
     * By default 30 analyses are listed per page.
     *
     * The `rules_count` field in the response give the number of rules
     * that were run in the analysis.
     * For very old analyses this data is not available,
     * and `0` is returned in this field.
     *
     * > [!WARNING]
     * > **Closing down notice:** The `tool_name` field is closing down and will, in future, not be included in the response for this endpoint. The example response reflects this change. The tool name can now be found inside the `tool` field.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#list-code-scanning-analyses-for-a-repository>
     */
    pub async fn list_all_recent_analyses(
        &self,
        owner: &str,
        repo: &str,
        tool_name: &str,
        tool_guid: &str,
        pr: i64,
        ref_: &str,
        sarif_id: &str,
        direction: crate::types::Order,
        sort: crate::types::CodeScanningListRecentAnalysesSort,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeScanningAnalysis>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if pr > 0 {
            query_args.push(("pr".to_string(), pr.to_string()));
        }
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        if !sarif_id.is_empty() {
            query_args.push(("sarif_id".to_string(), sarif_id.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !tool_guid.is_empty() {
            query_args.push(("tool_guid".to_string(), tool_guid.to_string()));
        }
        if !tool_name.is_empty() {
            query_args.push(("tool_name".to_string(), tool_name.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/analyses?{}",
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
     * Get a code scanning analysis for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/analyses/{analysis_id}` endpoint.
     *
     * Gets a specified code scanning analysis for a repository.
     *
     * The default JSON response contains fields that describe the analysis.
     * This includes the Git reference and commit SHA to which the analysis relates,
     * the datetime of the analysis, the name of the code scanning tool,
     * and the number of alerts.
     *
     * The `rules_count` field in the default response give the number of rules
     * that were run in the analysis.
     * For very old analyses this data is not available,
     * and `0` is returned in this field.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/sarif+json`**: Instead of returning a summary of the analysis, this endpoint returns a subset of the analysis data that was uploaded. The data is formatted as [SARIF version 2.1.0](https://docs.oasis-open.org/sarif/sarif/v2.1.0/cs01/sarif-v2.1.0-cs01.html). It also returns additional data such as the `github/alertNumber` and `github/alertUrl` properties.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#get-a-code-scanning-analysis-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `analysis_id: i64` -- The ID of the analysis, as returned from the `GET /repos/{owner}/{repo}/code-scanning/analyses` operation.
     */
    pub async fn get_analysis(
        &self,
        owner: &str,
        repo: &str,
        analysis_id: i64,
    ) -> ClientResult<crate::Response<crate::types::CodeScanningAnalysis>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/analyses/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&analysis_id.to_string()),
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
     * Delete a code scanning analysis from a repository.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/code-scanning/analyses/{analysis_id}` endpoint.
     *
     * Deletes a specified code scanning analysis from a repository.
     *
     * You can delete one analysis at a time.
     * To delete a series of analyses, start with the most recent analysis and work backwards.
     * Conceptually, the process is similar to the undo function in a text editor.
     *
     * When you list the analyses for a repository,
     * one or more will be identified as deletable in the response:
     *
     * ```
     * "deletable": true
     * ```
     *
     * An analysis is deletable when it's the most recent in a set of analyses.
     * Typically, a repository will have multiple sets of analyses
     * for each enabled code scanning tool,
     * where a set is determined by a unique combination of analysis values:
     *
     * * `ref`
     * * `tool`
     * * `category`
     *
     * If you attempt to delete an analysis that is not the most recent in a set,
     * you'll get a 400 response with the message:
     *
     * ```
     * Analysis specified is not deletable.
     * ```
     *
     * The response from a successful `DELETE` operation provides you with
     * two alternative URLs for deleting the next analysis in the set:
     * `next_analysis_url` and `confirm_delete_url`.
     * Use the `next_analysis_url` URL if you want to avoid accidentally deleting the final analysis
     * in a set. This is a useful option if you want to preserve at least one analysis
     * for the specified tool in your repository.
     * Use the `confirm_delete_url` URL if you are content to remove all analyses for a tool.
     * When you delete the last analysis in a set, the value of `next_analysis_url` and `confirm_delete_url`
     * in the 200 response is `null`.
     *
     * As an example of the deletion process,
     * let's imagine that you added a workflow that configured a particular code scanning tool
     * to analyze the code in a repository. This tool has added 15 analyses:
     * 10 on the default branch, and another 5 on a topic branch.
     * You therefore have two separate sets of analyses for this tool.
     * You've now decided that you want to remove all of the analyses for the tool.
     * To do this you must make 15 separate deletion requests.
     * To start, you must find an analysis that's identified as deletable.
     * Each set of analyses always has one that's identified as deletable.
     * Having found the deletable analysis for one of the two sets,
     * delete this analysis and then continue deleting the next analysis in the set until they're all deleted.
     * Then repeat the process for the second set.
     * The procedure therefore consists of a nested loop:
     *
     * **Outer loop**:
     * * List the analyses for the repository, filtered by tool.
     * * Parse this list to find a deletable analysis. If found:
     *
     *   **Inner loop**:
     *   * Delete the identified analysis.
     *   * Parse the response for the value of `confirm_delete_url` and, if found, use this in the next iteration.
     *
     * The above process assumes that you want to remove all trace of the tool's analyses from the GitHub user interface, for the specified repository, and it therefore uses the `confirm_delete_url` value. Alternatively, you could use the `next_analysis_url` value, which would leave the last analysis in each set undeleted to avoid removing a tool's analysis entirely.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#delete-a-code-scanning-analysis-from-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `analysis_id: i64` -- The ID of the analysis, as returned from the `GET /repos/{owner}/{repo}/code-scanning/analyses` operation.
     * * `confirm_delete: &str` -- Allow deletion if the specified analysis is the last in a set. If you attempt to delete the final analysis in a set without setting this parameter to `true`, you'll get a 400 response with the message: `Analysis is last of its type and deletion may result in the loss of historical alert data. Please specify confirm_delete.`.
     */
    pub async fn delete_analysis(
        &self,
        owner: &str,
        repo: &str,
        analysis_id: i64,
        confirm_delete: &str,
    ) -> ClientResult<crate::Response<crate::types::AnalysisDeletion>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !confirm_delete.is_empty() {
            query_args.push(("confirm_delete".to_string(), confirm_delete.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/analyses/{}?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&analysis_id.to_string()),
                query_
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
     * List CodeQL databases for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/codeql/databases` endpoint.
     *
     * Lists the CodeQL databases that are available in a repository.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#list-codeql-databases-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn list_codeql_databases(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeQlDatabase>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/codeql/databases",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * List CodeQL databases for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/codeql/databases` endpoint.
     *
     * As opposed to `list_codeql_databases`, this function returns all the pages of the request at once.
     *
     * Lists the CodeQL databases that are available in a repository.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#list-codeql-databases-for-a-repository>
     */
    pub async fn list_all_codeql_databases(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeQlDatabase>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/codeql/databases",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Get a CodeQL database for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/codeql/databases/{language}` endpoint.
     *
     * Gets a CodeQL database for a language in a repository.
     *
     * By default this endpoint returns JSON metadata about the CodeQL database. To
     * download the CodeQL database binary content, set the `Accept` header of the request
     * to [`application/zip`](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types), and make sure
     * your HTTP client is configured to follow redirects or use the `Location` header
     * to make a second request to get the redirect URL.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#get-a-codeql-database-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `language: &str` -- The language of the CodeQL database.
     */
    pub async fn get_codeql_database(
        &self,
        owner: &str,
        repo: &str,
        language: &str,
    ) -> ClientResult<crate::Response<crate::types::CodeQlDatabase>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/codeql/databases/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&language.to_string()),
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
     * Delete a CodeQL database.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/code-scanning/codeql/databases/{language}` endpoint.
     *
     * Deletes a CodeQL database for a language in a repository.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#delete-a-codeql-database>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `language: &str` -- The language of the CodeQL database.
     */
    pub async fn delete_codeql_database(
        &self,
        owner: &str,
        repo: &str,
        language: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/codeql/databases/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&language.to_string()),
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
     * Create a CodeQL variant analysis.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/code-scanning/codeql/variant-analyses` endpoint.
     *
     * Creates a new CodeQL variant analysis, which will run a CodeQL query against one or more repositories.
     *
     * Get started by learning more about [running CodeQL queries at scale with Multi-Repository Variant Analysis](https://docs.github.com/code-security/codeql-for-vs-code/getting-started-with-codeql-for-vs-code/running-codeql-queries-at-scale-with-multi-repository-variant-analysis).
     *
     * Use the `owner` and `repo` parameters in the URL to specify the controller repository that
     * will be used for running GitHub Actions workflows and storing the results of the CodeQL variant analysis.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#create-a-codeql-variant-analysis>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_variant_analysis(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::CodeScanningCreateVariantAnalysisRequest,
    ) -> ClientResult<crate::Response<crate::types::VariantAnalysis>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/codeql/variant-analyses",
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
     * Get the summary of a CodeQL variant analysis.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/codeql/variant-analyses/{codeql_variant_analysis_id}` endpoint.
     *
     * Gets the summary of a CodeQL variant analysis.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#get-the-summary-of-a-codeql-variant-analysis>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `codeql_variant_analysis_id: i64` -- The unique identifier of the variant analysis.
     */
    pub async fn get_variant_analysis(
        &self,
        owner: &str,
        repo: &str,
        codeql_variant_analysis_id: i64,
    ) -> ClientResult<crate::Response<crate::types::VariantAnalysis>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/codeql/variant-analyses/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&codeql_variant_analysis_id.to_string()),
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
     * Get the analysis status of a repository in a CodeQL variant analysis.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/codeql/variant-analyses/{codeql_variant_analysis_id}/repos/{repo_owner}/{repo_name}` endpoint.
     *
     * Gets the analysis status of a repository in a CodeQL variant analysis.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#get-the-analysis-status-of-a-repository-in-a-codeql-variant-analysis>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the controller repository.
     * * `codeql_variant_analysis_id: i64` -- The ID of the variant analysis.
     * * `repo_owner: &str` -- The account owner of the variant analysis repository. The name is not case sensitive.
     * * `repo_name: &str` -- The name of the variant analysis repository.
     */
    pub async fn get_variant_analysis_repo_task(
        &self,
        owner: &str,
        repo: &str,
        codeql_variant_analysis_id: i64,
        repo_owner: &str,
        repo_name: &str,
    ) -> ClientResult<crate::Response<crate::types::CodeScanningVariantAnalysisRepoTask>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/codeql/variant-analyses/{}/repos/{}/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&codeql_variant_analysis_id.to_string()),
                crate::progenitor_support::encode_path(&repo_owner.to_string()),
                crate::progenitor_support::encode_path(&repo_name.to_string()),
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
     * Get a code scanning default setup configuration.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/default-setup` endpoint.
     *
     * Gets a code scanning default setup configuration.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#get-a-code-scanning-default-setup-configuration>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_default_setup(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::CodeScanningDefaultSetup>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/default-setup",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Update a code scanning default setup configuration.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/code-scanning/default-setup` endpoint.
     *
     * Updates a code scanning default setup configuration.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#update-a-code-scanning-default-setup-configuration>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn update_default_setup(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::CodeScanningDefaultSetupUpdate,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/default-setup",
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
     * Upload an analysis as SARIF data.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/code-scanning/sarifs` endpoint.
     *
     * Uploads SARIF data containing the results of a code scanning analysis to make the results available in a repository. For troubleshooting information, see "[Troubleshooting SARIF uploads](https://docs.github.com/code-security/code-scanning/troubleshooting-sarif)."
     *
     * There are two places where you can upload code scanning results.
     *  - If you upload to a pull request, for example `--ref refs/pull/42/merge` or `--ref refs/pull/42/head`, then the results appear as alerts in a pull request check. For more information, see "[Triaging code scanning alerts in pull requests](/code-security/secure-coding/triaging-code-scanning-alerts-in-pull-requests)."
     *  - If you upload to a branch, for example `--ref refs/heads/my-branch`, then the results appear in the **Security** tab for your repository. For more information, see "[Managing code scanning alerts for your repository](/code-security/secure-coding/managing-code-scanning-alerts-for-your-repository#viewing-the-alerts-for-a-repository)."
     *
     * You must compress the SARIF-formatted analysis data that you want to upload, using `gzip`, and then encode it as a Base64 format string. For example:
     *
     * ```
     * gzip -c analysis-data.sarif | base64 -w0
     * ```
     *
     * SARIF upload supports a maximum number of entries per the following data objects, and an analysis will be rejected if any of these objects is above its maximum value. For some objects, there are additional values over which the entries will be ignored while keeping the most important entries whenever applicable.
     * To get the most out of your analysis when it includes data above the supported limits, try to optimize the analysis configuration. For example, for the CodeQL tool, identify and remove the most noisy queries. For more information, see "[SARIF results exceed one or more limits](https://docs.github.com/code-security/code-scanning/troubleshooting-sarif/results-exceed-limit)."
     *
     *
     * | **SARIF data**                   | **Maximum values** | **Additional limits**                                                            |
     * |----------------------------------|:------------------:|----------------------------------------------------------------------------------|
     * | Runs per file                    |         20         |                                                                                  |
     * | Results per run                  |       25,000       | Only the top 5,000 results will be included, prioritized by severity.            |
     * | Rules per run                    |       25,000       |                                                                                  |
     * | Tool extensions per run          |        100         |                                                                                  |
     * | Thread Flow Locations per result |       10,000       | Only the top 1,000 Thread Flow Locations will be included, using prioritization. |
     * | Location per result	             |       1,000        | Only 100 locations will be included.                                             |
     * | Tags per rule	                   |         20         | Only 10 tags will be included.                                                   |
     *
     *
     * The `202 Accepted` response includes an `id` value.
     * You can use this ID to check the status of the upload by using it in the `/sarifs/{sarif_id}` endpoint.
     * For more information, see "[Get information about a SARIF upload](/rest/code-scanning/code-scanning#get-information-about-a-sarif-upload)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * This endpoint is limited to 1,000 requests per hour for each user or app installation calling it.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#upload-an-analysis-as-sarif-data>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn upload_sarif(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::CodeScanningUploadSarifRequest,
    ) -> ClientResult<crate::Response<crate::types::CodeScanningSarifsReceipt>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/sarifs",
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
     * Get information about a SARIF upload.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/sarifs/{sarif_id}` endpoint.
     *
     * Gets information about a SARIF upload, including the status and the URL of the analysis that was uploaded so that you can retrieve details of the analysis. For more information, see "[Get a code scanning analysis for a repository](/rest/code-scanning/code-scanning#get-a-code-scanning-analysis-for-a-repository)."
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint with private or public repositories, or the `public_repo` scope to use this endpoint with only public repositories.
     *
     * FROM: <https://docs.github.com/rest/code-scanning/code-scanning#get-information-about-a-sarif-upload>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `sarif_id: &str` -- The SARIF ID obtained after uploading.
     */
    pub async fn get_sarif(
        &self,
        owner: &str,
        repo: &str,
        sarif_id: &str,
    ) -> ClientResult<crate::Response<crate::types::CodeScanningSarifsStatus>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/sarifs/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&sarif_id.to_string()),
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
