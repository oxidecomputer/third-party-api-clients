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
     * List code scanning alerts for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/alerts` endpoint.
     *
     * Lists all open code scanning alerts for the default branch (usually `main`
     * or `master`). You must use an access token with the `security_events` scope to use
     * this endpoint. GitHub Apps must have the `security_events` read permission to use
     * this endpoint.
     *
     * The response includes a `most_recent_instance` object.
     * This provides details of the most recent instance of this alert
     * for the default branch or for the specified Git reference
     * (if you used `ref` in the request).
     *
     * FROM: <https://docs.github.com/rest/reference/code-scanning#list-code-scanning-alerts-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `tool_name: &str` -- The name of a code scanning tool. Only results by this tool will be listed. You can specify the tool by using either `tool_name` or `tool_guid`, but not both.
     * * `tool_guid: &str` -- The GUID of a code scanning tool. Only results by this tool will be listed. Note that some code scanning tools may not include a GUID in their analysis data. You can specify the tool by using either `tool_guid` or `tool_name`, but not both.
     * * `page: i64` -- Page number of the results to fetch.
     * * `per_page: i64` -- Results per page (max 100).
     * * `ref_: &str` -- The Git reference for the results you want to list. The `ref` for a branch can be formatted either as `refs/heads/<branch name>` or simply `<branch name>`. To reference a pull request use `refs/pull/<number>/merge`.
     * * `state: crate::types::CodeScanningAlertState` -- Set to `open`, `fixed`, or `dismissed` to list code scanning alerts in a specific state.
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
        state: crate::types::CodeScanningAlertState,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeScanningAlertItems>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
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
     * List code scanning alerts for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/alerts` endpoint.
     *
     * As opposed to `list_alerts_for_repo`, this function returns all the pages of the request at once.
     *
     * Lists all open code scanning alerts for the default branch (usually `main`
     * or `master`). You must use an access token with the `security_events` scope to use
     * this endpoint. GitHub Apps must have the `security_events` read permission to use
     * this endpoint.
     *
     * The response includes a `most_recent_instance` object.
     * This provides details of the most recent instance of this alert
     * for the default branch or for the specified Git reference
     * (if you used `ref` in the request).
     *
     * FROM: <https://docs.github.com/rest/reference/code-scanning#list-code-scanning-alerts-for-a-repository>
     */
    pub async fn list_all_alerts_for_repo(
        &self,
        owner: &str,
        repo: &str,
        tool_name: &str,
        tool_guid: &str,
        ref_: &str,
        state: crate::types::CodeScanningAlertState,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeScanningAlertItems>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
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
     * Get a code scanning alert.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/alerts/{alert_number}` endpoint.
     *
     * Gets a single code scanning alert. You must use an access token with the `security_events` scope to use this endpoint. GitHub Apps must have the `security_events` read permission to use this endpoint.
     *
     * **Deprecation notice**:
     * The instances field is deprecated and will, in future, not be included in the response for this endpoint. The example response reflects this change. The same information can now be retrieved via a GET request to the URL specified by `instances_url`.
     *
     * FROM: <https://docs.github.com/rest/reference/code-scanning#get-a-code-scanning-alert>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Updates the status of a single code scanning alert. You must use an access token with the `security_events` scope to use this endpoint. GitHub Apps must have the `security_events` write permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/code-scanning#update-a-code-scanning-alert>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List instances of a code scanning alert.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/alerts/{alert_number}/instances` endpoint.
     *
     * Lists all instances of the specified code scanning alert. You must use an access token with the `security_events` scope to use this endpoint. GitHub Apps must have the `security_events` read permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/code-scanning#list-instances-of-a-code-scanning-alert>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `alert_number: i64` -- The number that identifies an alert. You can find this at the end of the URL for a code scanning alert within GitHub, and in the `number` field in the response from the `GET /repos/{owner}/{repo}/code-scanning/alerts` operation.
     * * `page: i64` -- Page number of the results to fetch.
     * * `per_page: i64` -- Results per page (max 100).
     * * `ref_: &str` -- The Git reference for the results you want to list. The `ref` for a branch can be formatted either as `refs/heads/<branch name>` or simply `<branch name>`. To reference a pull request use `refs/pull/<number>/merge`.
     */
    pub async fn list_alert_instances(
        &self,
        owner: &str,
        repo: &str,
        alert_number: i64,
        page: i64,
        per_page: i64,
        ref_: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeScanningAlertInstance>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/alerts/{}/instances?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Lists all instances of the specified code scanning alert. You must use an access token with the `security_events` scope to use this endpoint. GitHub Apps must have the `security_events` read permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/code-scanning#list-instances-of-a-code-scanning-alert>
     */
    pub async fn list_all_alert_instances(
        &self,
        owner: &str,
        repo: &str,
        alert_number: i64,
        ref_: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeScanningAlertInstance>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-scanning/alerts/{}/instances?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * You must use an access token with the `security_events` scope to use this endpoint.
     * GitHub Apps must have the `security_events` read permission to use this endpoint.
     *
     * **Deprecation notice**:
     * The `tool_name` field is deprecated and will, in future, not be included in the response for this endpoint. The example response reflects this change. The tool name can now be found inside the `tool` field.
     *
     * FROM: <https://docs.github.com/rest/reference/code-scanning#list-code-scanning-analyses-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `tool_name: &str` -- The name of a code scanning tool. Only results by this tool will be listed. You can specify the tool by using either `tool_name` or `tool_guid`, but not both.
     * * `tool_guid: &str` -- The GUID of a code scanning tool. Only results by this tool will be listed. Note that some code scanning tools may not include a GUID in their analysis data. You can specify the tool by using either `tool_guid` or `tool_name`, but not both.
     * * `page: i64` -- Page number of the results to fetch.
     * * `per_page: i64` -- Results per page (max 100).
     * * `ref_: &str` -- The Git reference for the analyses you want to list. The `ref` for a branch can be formatted either as `refs/heads/<branch name>` or simply `<branch name>`. To reference a pull request use `refs/pull/<number>/merge`.
     * * `sarif_id: &str` -- Filter analyses belonging to the same SARIF upload.
     */
    pub async fn list_recent_analyses(
        &self,
        owner: &str,
        repo: &str,
        tool_name: &str,
        tool_guid: &str,
        page: i64,
        per_page: i64,
        ref_: &str,
        sarif_id: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeScanningAnalysis>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        if !sarif_id.is_empty() {
            query_args.push(("sarif_id".to_string(), sarif_id.to_string()));
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
     * You must use an access token with the `security_events` scope to use this endpoint.
     * GitHub Apps must have the `security_events` read permission to use this endpoint.
     *
     * **Deprecation notice**:
     * The `tool_name` field is deprecated and will, in future, not be included in the response for this endpoint. The example response reflects this change. The tool name can now be found inside the `tool` field.
     *
     * FROM: <https://docs.github.com/rest/reference/code-scanning#list-code-scanning-analyses-for-a-repository>
     */
    pub async fn list_all_recent_analyses(
        &self,
        owner: &str,
        repo: &str,
        tool_name: &str,
        tool_guid: &str,
        ref_: &str,
        sarif_id: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeScanningAnalysis>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        if !sarif_id.is_empty() {
            query_args.push(("sarif_id".to_string(), sarif_id.to_string()));
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
     * Get a code scanning analysis for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/analyses/{analysis_id}` endpoint.
     *
     * Gets a specified code scanning analysis for a repository.
     * You must use an access token with the `security_events` scope to use this endpoint.
     * GitHub Apps must have the `security_events` read permission to use this endpoint.
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
     * If you use the Accept header `application/sarif+json`,
     * the response contains the analysis data that was uploaded.
     * This is formatted as
     * [SARIF version 2.1.0](https://docs.oasis-open.org/sarif/sarif/v2.1.0/cs01/sarif-v2.1.0-cs01.html).
     *
     * **Deprecation notice**:
     * The `tool_name` field is deprecated and will, in future, not be included in the response for this endpoint. The example response reflects this change. The tool name can now be found inside the `tool` field.
     *
     * FROM: <https://docs.github.com/rest/reference/code-scanning#get-a-code-scanning-analysis-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Deletes a specified code scanning analysis from a repository. For
     * private repositories, you must use an access token with the `repo` scope. For public repositories,
     * you must use an access token with `public_repo` and `repo:security_events` scopes.
     * GitHub Apps must have the `security_events` write permission to use this endpoint.
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
     * * `analysis_key`
     * * `environment`
     *
     * If you attempt to delete an analysis that is not the most recent in a set,
     * you'll get a 400 response with the message:
     *
     * ```
     * Analysis specified is not deletable.
     * ```
     *
     * The response from a successful `DELETE` operation provides you with
     * two alternative URLs for deleting the next analysis in the set
     * (see the example default response below).
     * Use the `next_analysis_url` URL if you want to avoid accidentally deleting the final analysis
     * in the set. This is a useful option if you want to preserve at least one analysis
     * for the specified tool in your repository.
     * Use the `confirm_delete_url` URL if you are content to remove all analyses for a tool.
     * When you delete the last analysis in a set the value of `next_analysis_url` and `confirm_delete_url`
     * in the 200 response is `null`.
     *
     * As an example of the deletion process,
     * let's imagine that you added a workflow that configured a particular code scanning tool
     * to analyze the code in a repository. This tool has added 15 analyses:
     * 10 on the default branch, and another 5 on a topic branch.
     * You therefore have two separate sets of analyses for this tool.
     * You've now decided that you want to remove all of the analyses for the tool.
     * To do this you must make 15 separate deletion requests.
     * To start, you must find the deletable analysis for one of the sets,
     * step through deleting the analyses in that set,
     * and then repeat the process for the second set.
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
     * FROM: <https://docs.github.com/rest/reference/code-scanning#delete-a-code-scanning-analysis-from-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Upload an analysis as SARIF data.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/code-scanning/sarifs` endpoint.
     *
     * Uploads SARIF data containing the results of a code scanning analysis to make the results available in a repository. You must use an access token with the `security_events` scope to use this endpoint. GitHub Apps must have the `security_events` write permission to use this endpoint.
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
     * SARIF upload supports a maximum of 5000 results per analysis run. Any results over this limit are ignored and any SARIF uploads with more than 25,000 results are rejected. Typically, but not necessarily, a SARIF file contains a single run of a single tool. If a code scanning tool generates too many results, you should update the analysis configuration to run only the most important rules or queries.
     *
     * The `202 Accepted`, response includes an `id` value.
     * You can use this ID to check the status of the upload by using this for the `/sarifs/{sarif_id}` endpoint.
     * For more information, see "[Get information about a SARIF upload](/rest/reference/code-scanning#get-information-about-a-sarif-upload)."
     *
     * FROM: <https://docs.github.com/rest/reference/code-scanning#upload-a-sarif-file>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
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
     * Get information about a SARIF upload.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-scanning/sarifs/{sarif_id}` endpoint.
     *
     * Gets information about a SARIF upload, including the status and the URL of the analysis that was uploaded so that you can retrieve details of the analysis. For more information, see "[Get a code scanning analysis for a repository](/rest/reference/code-scanning#get-a-code-scanning-analysis-for-a-repository)." You must use an access token with the `security_events` scope to use this endpoint. GitHub Apps must have the `security_events` read permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/code-scanning#list-recent-code-scanning-analyses-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(sarif_id),
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
