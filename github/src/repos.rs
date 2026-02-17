use crate::Client;
use crate::ClientResult;

#[derive(Debug, Default, Clone)]
pub struct ReposUploadReleaseAssetDefaultServer {}

impl ReposUploadReleaseAssetDefaultServer {
    pub fn default_url(&self) -> &str {
        "https://uploads.github.com"
    }
}

pub struct Repos {
    pub client: Client,
}

impl Repos {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Repos { client }
    }

    /**
     * List organization repositories.
     *
     * This function performs a `GET` to the `/orgs/{org}/repos` endpoint.
     *
     * Lists repositories for the specified organization.
     *
     * > [!NOTE]
     * > In order to see the `security_and_analysis` block for a repository you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
     *
     * FROM: <https://docs.github.com/rest/repos/repos#list-organization-repositories>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `type_: crate::types::ReposListOrgType` -- Specifies the types of repositories you want returned.
     * * `sort: crate::types::ReposListOrgSort` -- The property to sort the results by.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_for_org(
        &self,
        org: &str,
        type_: crate::types::ReposListOrgType,
        sort: crate::types::ReposListOrgSort,
        direction: crate::types::Order,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::MinimalRepository>>> {
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
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/repos?{}",
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
     * List organization repositories.
     *
     * This function performs a `GET` to the `/orgs/{org}/repos` endpoint.
     *
     * As opposed to `list_for_org`, this function returns all the pages of the request at once.
     *
     * Lists repositories for the specified organization.
     *
     * > [!NOTE]
     * > In order to see the `security_and_analysis` block for a repository you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
     *
     * FROM: <https://docs.github.com/rest/repos/repos#list-organization-repositories>
     */
    pub async fn list_all_for_org(
        &self,
        org: &str,
        type_: crate::types::ReposListOrgType,
        sort: crate::types::ReposListOrgSort,
        direction: crate::types::Order,
    ) -> ClientResult<crate::Response<Vec<crate::types::MinimalRepository>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/repos?{}",
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
     * Create an organization repository.
     *
     * This function performs a `POST` to the `/orgs/{org}/repos` endpoint.
     *
     * Creates a new repository in the specified organization. The authenticated user must be a member of the organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `public_repo` or `repo` scope to create a public repository, and `repo` scope to create a private repository.
     *
     * FROM: <https://docs.github.com/rest/repos/repos#create-an-organization-repository>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn create_in_org(
        &self,
        org: &str,
        body: &crate::types::ReposCreateInOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::FullRepository>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/repos",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Get all organization repository rulesets.
     *
     * This function performs a `GET` to the `/orgs/{org}/rulesets` endpoint.
     *
     * Get all the repository rulesets for an organization.
     *
     * FROM: <https://docs.github.com/rest/orgs/rules#get-all-organization-repository-rulesets>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `targets: &str` -- A comma-separated list of rule targets to filter by.
     *   If provided, only rulesets that apply to the specified targets will be returned.
     *   For example, `branch,tag,push`.
     *   .
     */
    pub async fn get_org_rulesets(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
        targets: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::RepositoryRuleset>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !targets.is_empty() {
            query_args.push(("targets".to_string(), targets.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/rulesets?{}",
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
     * Get all organization repository rulesets.
     *
     * This function performs a `GET` to the `/orgs/{org}/rulesets` endpoint.
     *
     * As opposed to `get_org_rulesets`, this function returns all the pages of the request at once.
     *
     * Get all the repository rulesets for an organization.
     *
     * FROM: <https://docs.github.com/rest/orgs/rules#get-all-organization-repository-rulesets>
     */
    pub async fn get_all_org_rulesets(
        &self,
        org: &str,
        targets: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::RepositoryRuleset>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !targets.is_empty() {
            query_args.push(("targets".to_string(), targets.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/rulesets?{}",
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
     * Create an organization repository ruleset.
     *
     * This function performs a `POST` to the `/orgs/{org}/rulesets` endpoint.
     *
     * Create a repository ruleset for an organization.
     *
     * FROM: <https://docs.github.com/rest/orgs/rules#create-an-organization-repository-ruleset>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn create_org_ruleset(
        &self,
        org: &str,
        body: &crate::types::ReposCreateOrgRulesetRequest,
    ) -> ClientResult<crate::Response<crate::types::RepositoryRuleset>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/rulesets",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List organization rule suites.
     *
     * This function performs a `GET` to the `/orgs/{org}/rulesets/rule-suites` endpoint.
     *
     * Lists suites of rule evaluations at the organization level.
     * For more information, see "[Managing rulesets for repositories in your organization](https://docs.github.com/organizations/managing-organization-settings/managing-rulesets-for-repositories-in-your-organization#viewing-insights-for-rulesets)."
     *
     * FROM: <https://docs.github.com/rest/orgs/rule-suites#list-organization-rule-suites>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `ref_: &str` -- The name of the ref. Cannot contain wildcard characters. Optionally prefix with `refs/heads/` to limit to branches or `refs/tags/` to limit to tags. Omit the prefix to search across all refs. When specified, only rule evaluations triggered for this ref will be returned.
     * * `repository_name: &str` -- The name of the repository to filter on.
     * * `time_period: crate::types::TimePeriodData` -- The time period to filter by.
     *  
     *  For example, `day` will filter for rule suites that occurred in the past 24 hours, and `week` will filter for rule suites that occurred in the past 7 days (168 hours).
     * * `actor_name: &str` -- The handle for the GitHub user account to filter on. When specified, only rule evaluations triggered by this actor will be returned.
     * * `rule_suite_result: crate::types::RuleSuiteResult` -- The rule suite results to filter on. When specified, only suites with this result will be returned.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn get_org_rule_suites(
        &self,
        org: &str,
        ref_: &str,
        repository_name: &str,
        time_period: crate::types::TimePeriodData,
        actor_name: &str,
        rule_suite_result: crate::types::RuleSuiteResult,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::RuleSuites>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !actor_name.is_empty() {
            query_args.push(("actor_name".to_string(), actor_name.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        if !repository_name.is_empty() {
            query_args.push(("repository_name".to_string(), repository_name.to_string()));
        }
        if !rule_suite_result.to_string().is_empty() {
            query_args.push((
                "rule_suite_result".to_string(),
                rule_suite_result.to_string(),
            ));
        }
        if !time_period.to_string().is_empty() {
            query_args.push(("time_period".to_string(), time_period.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/rulesets/rule-suites?{}",
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
     * List organization rule suites.
     *
     * This function performs a `GET` to the `/orgs/{org}/rulesets/rule-suites` endpoint.
     *
     * As opposed to `get_org_rule_suites`, this function returns all the pages of the request at once.
     *
     * Lists suites of rule evaluations at the organization level.
     * For more information, see "[Managing rulesets for repositories in your organization](https://docs.github.com/organizations/managing-organization-settings/managing-rulesets-for-repositories-in-your-organization#viewing-insights-for-rulesets)."
     *
     * FROM: <https://docs.github.com/rest/orgs/rule-suites#list-organization-rule-suites>
     */
    pub async fn get_all_org_rule_suites(
        &self,
        org: &str,
        ref_: &str,
        repository_name: &str,
        time_period: crate::types::TimePeriodData,
        actor_name: &str,
        rule_suite_result: crate::types::RuleSuiteResult,
    ) -> ClientResult<crate::Response<Vec<crate::types::RuleSuites>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !actor_name.is_empty() {
            query_args.push(("actor_name".to_string(), actor_name.to_string()));
        }
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        if !repository_name.is_empty() {
            query_args.push(("repository_name".to_string(), repository_name.to_string()));
        }
        if !rule_suite_result.to_string().is_empty() {
            query_args.push((
                "rule_suite_result".to_string(),
                rule_suite_result.to_string(),
            ));
        }
        if !time_period.to_string().is_empty() {
            query_args.push(("time_period".to_string(), time_period.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/rulesets/rule-suites?{}",
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
     * Get an organization rule suite.
     *
     * This function performs a `GET` to the `/orgs/{org}/rulesets/rule-suites/{rule_suite_id}` endpoint.
     *
     * Gets information about a suite of rule evaluations from within an organization.
     * For more information, see "[Managing rulesets for repositories in your organization](https://docs.github.com/organizations/managing-organization-settings/managing-rulesets-for-repositories-in-your-organization#viewing-insights-for-rulesets)."
     *
     * FROM: <https://docs.github.com/rest/orgs/rule-suites#get-an-organization-rule-suite>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `rule_suite_id: i64` -- The unique identifier of the rule suite result.
     *   To get this ID, you can use [GET /repos/{owner}/{repo}/rulesets/rule-suites](https://docs.github.com/rest/repos/rule-suites#list-repository-rule-suites)
     *   for repositories and [GET /orgs/{org}/rulesets/rule-suites](https://docs.github.com/rest/orgs/rule-suites#list-organization-rule-suites)
     *   for organizations.
     */
    pub async fn get_org_rule_suite(
        &self,
        org: &str,
        rule_suite_id: i64,
    ) -> ClientResult<crate::Response<crate::types::RuleSuite>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/rulesets/rule-suites/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&rule_suite_id.to_string()),
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
     * Get an organization repository ruleset.
     *
     * This function performs a `GET` to the `/orgs/{org}/rulesets/{ruleset_id}` endpoint.
     *
     * Get a repository ruleset for an organization.
     *
     * **Note:** To prevent leaking sensitive information, the `bypass_actors` property is only returned if the user
     * making the API request has write access to the ruleset.
     *
     * FROM: <https://docs.github.com/rest/orgs/rules#get-an-organization-repository-ruleset>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `ruleset_id: i64` -- The ID of the ruleset.
     */
    pub async fn get_org_ruleset(
        &self,
        org: &str,
        ruleset_id: i64,
    ) -> ClientResult<crate::Response<crate::types::RepositoryRuleset>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/rulesets/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&ruleset_id.to_string()),
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
     * Update an organization repository ruleset.
     *
     * This function performs a `PUT` to the `/orgs/{org}/rulesets/{ruleset_id}` endpoint.
     *
     * Update a ruleset for an organization.
     *
     * FROM: <https://docs.github.com/rest/orgs/rules#update-an-organization-repository-ruleset>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `ruleset_id: i64` -- The ID of the ruleset.
     */
    pub async fn update_org_ruleset(
        &self,
        org: &str,
        ruleset_id: i64,
        body: &crate::types::ReposUpdateOrgRulesetRequest,
    ) -> ClientResult<crate::Response<crate::types::RepositoryRuleset>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/rulesets/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&ruleset_id.to_string()),
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
     * Delete an organization repository ruleset.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/rulesets/{ruleset_id}` endpoint.
     *
     * Delete a ruleset for an organization.
     *
     * FROM: <https://docs.github.com/rest/orgs/rules#delete-an-organization-repository-ruleset>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `ruleset_id: i64` -- The ID of the ruleset.
     */
    pub async fn delete_org_ruleset(
        &self,
        org: &str,
        ruleset_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/rulesets/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&ruleset_id.to_string()),
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
     * Get a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}` endpoint.
     *
     * The `parent` and `source` objects are present when the repository is a fork. `parent` is the repository this repository was forked from, `source` is the ultimate source for the network.
     *
     * > [!NOTE]
     * > - In order to see the `security_and_analysis` block for a repository you must have admin permissions for the repository or be an owner or security manager for the organization that owns the repository. For more information, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
     * > - To view merge-related settings, you must have the `contents:read` and `contents:write` permissions.
     *
     * FROM: <https://docs.github.com/rest/repos/repos#get-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::FullRepository>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}",
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
     * Delete a repository.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}` endpoint.
     *
     * Deleting a repository requires admin access.
     *
     * If an organization owner has configured the organization to prevent members from deleting organization-owned
     * repositories, you will get a `403 Forbidden` response.
     *
     * OAuth app tokens and personal access tokens (classic) need the `delete_repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/repos/repos#delete-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn delete(&self, owner: &str, repo: &str) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Update a repository.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}` endpoint.
     *
     * **Note**: To edit a repository's topics, use the [Replace all repository topics](https://docs.github.com/rest/repos/repos#replace-all-repository-topics) endpoint.
     *
     * FROM: <https://docs.github.com/rest/repos/repos#update-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn update(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposUpdateRequest,
    ) -> ClientResult<crate::Response<crate::types::FullRepository>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}",
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
     * List repository activities.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/activity` endpoint.
     *
     * Lists a detailed history of changes to a repository, such as pushes, merges, force pushes, and branch changes, and associates these changes with commits and users.
     *
     * For more information about viewing repository activity,
     * see "[Viewing activity and data for your repository](https://docs.github.com/repositories/viewing-activity-and-data-for-your-repository)."
     *
     * FROM: <https://docs.github.com/rest/repos/repos#list-repository-activities>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `ref_: &str` -- The Git reference for the activities you want to list.
     *   
     *   The `ref` for a branch can be formatted either as `refs/heads/BRANCH_NAME` or `BRANCH_NAME`, where `BRANCH_NAME` is the name of your branch.
     * * `actor: &str` -- The GitHub username to use to filter by the actor who performed the activity.
     * * `time_period: crate::types::ReposListActivitiesTimePeriod` -- The time period to filter by.
     *  
     *  For example, `day` will filter for activity that occurred in the past 24 hours, and `week` will filter for activity that occurred in the past 7 days (168 hours).
     * * `activity_type: crate::types::ActivityType` -- The activity type to filter by.
     *   
     *   For example, you can choose to filter by "force_push", to see all force pushes to the repository.
     */
    pub async fn list_activities(
        &self,
        owner: &str,
        repo: &str,
        direction: crate::types::Order,
        per_page: i64,
        before: &str,
        after: &str,
        ref_: &str,
        actor: &str,
        time_period: crate::types::ReposListActivitiesTimePeriod,
        activity_type: crate::types::ActivityType,
    ) -> ClientResult<crate::Response<Vec<crate::types::Activity>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !activity_type.to_string().is_empty() {
            query_args.push(("activity_type".to_string(), activity_type.to_string()));
        }
        if !actor.is_empty() {
            query_args.push(("actor".to_string(), actor.to_string()));
        }
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        if !time_period.to_string().is_empty() {
            query_args.push(("time_period".to_string(), time_period.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/activity?{}",
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
     * List repository activities.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/activity` endpoint.
     *
     * As opposed to `list_activities`, this function returns all the pages of the request at once.
     *
     * Lists a detailed history of changes to a repository, such as pushes, merges, force pushes, and branch changes, and associates these changes with commits and users.
     *
     * For more information about viewing repository activity,
     * see "[Viewing activity and data for your repository](https://docs.github.com/repositories/viewing-activity-and-data-for-your-repository)."
     *
     * FROM: <https://docs.github.com/rest/repos/repos#list-repository-activities>
     */
    pub async fn list_all_activities(
        &self,
        owner: &str,
        repo: &str,
        direction: crate::types::Order,
        before: &str,
        after: &str,
        ref_: &str,
        actor: &str,
        time_period: crate::types::ReposListActivitiesTimePeriod,
        activity_type: crate::types::ActivityType,
    ) -> ClientResult<crate::Response<Vec<crate::types::Activity>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !activity_type.to_string().is_empty() {
            query_args.push(("activity_type".to_string(), activity_type.to_string()));
        }
        if !actor.is_empty() {
            query_args.push(("actor".to_string(), actor.to_string()));
        }
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        if !time_period.to_string().is_empty() {
            query_args.push(("time_period".to_string(), time_period.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/activity?{}",
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
     * Create an attestation.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/attestations` endpoint.
     *
     * Store an artifact attestation and associate it with a repository.
     *
     * The authenticated user must have write permission to the repository and, if using a fine-grained access token, the `attestations:write` permission is required.
     *
     * Artifact attestations are meant to be created using the [attest action](https://github.com/actions/attest). For more information, see our guide on [using artifact attestations to establish a build's provenance](https://docs.github.com/actions/security-guides/using-artifact-attestations-to-establish-provenance-for-builds).
     *
     * FROM: <https://docs.github.com/rest/repos/attestations#create-an-attestation>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_attestation(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposCreateAttestationRequest,
    ) -> ClientResult<crate::Response<crate::types::ReposCreateAttestationResponse>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/attestations",
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
     * List attestations.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/attestations/{subject_digest}` endpoint.
     *
     * List a collection of artifact attestations with a given subject digest that are associated with a repository.
     *
     * The authenticated user making the request must have read access to the repository. In addition, when using a fine-grained access token the `attestations:read` permission is required.
     *
     * **Please note:** in order to offer meaningful security benefits, an attestation's signature and timestamps **must** be cryptographically verified, and the identity of the attestation signer **must** be validated. Attestations can be verified using the [GitHub CLI `attestation verify` command](https://cli.github.com/manual/gh_attestation_verify). For more information, see [our guide on how to use artifact attestations to establish a build's provenance](https://docs.github.com/actions/security-guides/using-artifact-attestations-to-establish-provenance-for-builds).
     *
     * FROM: <https://docs.github.com/rest/repos/attestations#list-attestations>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `subject_digest: &str` -- The parameter should be set to the attestation's subject's SHA256 digest, in the form `sha256:HEX_DIGEST`.
     * * `predicate_type: &str` -- Optional filter for fetching attestations with a given predicate type.
     *   This option accepts `provenance`, `sbom`, `release`, or freeform text
     *   for custom predicate types.
     */
    pub async fn list_attestations(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        before: &str,
        after: &str,
        subject_digest: &str,
        predicate_type: &str,
    ) -> ClientResult<crate::Response<crate::types::UsersListAttestationsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !predicate_type.is_empty() {
            query_args.push(("predicate_type".to_string(), predicate_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/attestations/{}?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&subject_digest.to_string()),
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
     * Get all autolinks of a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/autolinks` endpoint.
     *
     * Gets all autolinks that are configured for a repository.
     *
     * Information about autolinks are only available to repository administrators.
     *
     * FROM: <https://docs.github.com/rest/repos/autolinks#get-all-autolinks-of-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn list_autolinks(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Autolink>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/autolinks",
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
     * Get all autolinks of a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/autolinks` endpoint.
     *
     * As opposed to `list_autolinks`, this function returns all the pages of the request at once.
     *
     * Gets all autolinks that are configured for a repository.
     *
     * Information about autolinks are only available to repository administrators.
     *
     * FROM: <https://docs.github.com/rest/repos/autolinks#get-all-autolinks-of-a-repository>
     */
    pub async fn list_all_autolinks(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Autolink>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/autolinks",
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
     * Create an autolink reference for a repository.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/autolinks` endpoint.
     *
     * Users with admin access to the repository can create an autolink.
     *
     * FROM: <https://docs.github.com/rest/repos/autolinks#create-an-autolink-reference-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_autolink(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposCreateAutolinkRequest,
    ) -> ClientResult<crate::Response<crate::types::Autolink>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/autolinks",
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
     * Get an autolink reference of a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/autolinks/{autolink_id}` endpoint.
     *
     * This returns a single autolink reference by ID that was configured for the given repository.
     *
     * Information about autolinks are only available to repository administrators.
     *
     * FROM: <https://docs.github.com/rest/repos/autolinks#get-an-autolink-reference-of-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `autolink_id: i64` -- The unique identifier of the autolink.
     */
    pub async fn get_autolink(
        &self,
        owner: &str,
        repo: &str,
        autolink_id: i64,
    ) -> ClientResult<crate::Response<crate::types::Autolink>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/autolinks/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&autolink_id.to_string()),
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
     * Delete an autolink reference from a repository.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/autolinks/{autolink_id}` endpoint.
     *
     * This deletes a single autolink reference by ID that was configured for the given repository.
     *
     * Information about autolinks are only available to repository administrators.
     *
     * FROM: <https://docs.github.com/rest/repos/autolinks#delete-an-autolink-reference-from-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `autolink_id: i64` -- The unique identifier of the autolink.
     */
    pub async fn delete_autolink(
        &self,
        owner: &str,
        repo: &str,
        autolink_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/autolinks/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&autolink_id.to_string()),
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
     * Check if Dependabot security updates are enabled for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/automated-security-fixes` endpoint.
     *
     * Shows whether Dependabot security updates are enabled, disabled or paused for a repository. The authenticated user must have admin read access to the repository. For more information, see "[Configuring Dependabot security updates](https://docs.github.com/articles/configuring-automated-security-fixes)".
     *
     * FROM: <https://docs.github.com/rest/repos/repos#check-if-dependabot-security-updates-are-enabled-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn check_automated_security_fixes(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::CheckAutomatedSecurityFixes>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/automated-security-fixes",
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
     * Enable Dependabot security updates.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/automated-security-fixes` endpoint.
     *
     * Enables Dependabot security updates for a repository. The authenticated user must have admin access to the repository. For more information, see "[Configuring Dependabot security updates](https://docs.github.com/articles/configuring-automated-security-fixes)".
     *
     * FROM: <https://docs.github.com/rest/repos/repos#enable-dependabot-security-updates>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn enable_automated_security_fixes(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/automated-security-fixes",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Disable Dependabot security updates.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/automated-security-fixes` endpoint.
     *
     * Disables Dependabot security updates for a repository. The authenticated user must have admin access to the repository. For more information, see "[Configuring Dependabot security updates](https://docs.github.com/articles/configuring-automated-security-fixes)".
     *
     * FROM: <https://docs.github.com/rest/repos/repos#disable-dependabot-security-updates>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn disable_automated_security_fixes(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/automated-security-fixes",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * List branches.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/branches` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/branches/branches#list-branches>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `protected: bool` -- Setting to `true` returns only branches protected by branch protections or rulesets. When set to `false`, only unprotected branches are returned. Omitting this parameter returns all branches.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_branches(
        &self,
        owner: &str,
        repo: &str,
        protected: bool,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::ShortBranch>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if protected {
            query_args.push(("protected".to_string(), protected.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches?{}",
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
     * List branches.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/branches` endpoint.
     *
     * As opposed to `list_branches`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/branches/branches#list-branches>
     */
    pub async fn list_all_branches(
        &self,
        owner: &str,
        repo: &str,
        protected: bool,
    ) -> ClientResult<crate::Response<Vec<crate::types::ShortBranch>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if protected {
            query_args.push(("protected".to_string(), protected.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches?{}",
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
     * Get a branch.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/branches/{branch}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/branches/branches#get-a-branch>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn get_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<crate::types::BranchWithProtection>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Get branch protection.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/branches/{branch}/protection` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#get-branch-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn get_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<crate::types::BranchProtection>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Update branch protection.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/branches/{branch}/protection` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Protecting a branch requires admin or owner permissions to the repository.
     *
     * > [!NOTE]
     * > Passing new arrays of `users` and `teams` replaces their previous values.
     *
     * > [!NOTE]
     * > The list of users, apps, and teams in total is limited to 100 items.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#update-branch-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn update_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposUpdateBranchProtectionRequestData,
    ) -> ClientResult<crate::Response<crate::types::ProtectedBranch>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Delete branch protection.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/branches/{branch}/protection` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#delete-branch-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn delete_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Get admin branch protection.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#get-admin-branch-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn get_admin_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<crate::types::EnforceAdmins>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/enforce_admins",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Set admin branch protection.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Adding admin enforcement requires admin or owner permissions to the repository and branch protection to be enabled.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#set-admin-branch-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn set_admin_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<crate::types::EnforceAdmins>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/enforce_admins",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Delete admin branch protection.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/branches/{branch}/protection/enforce_admins` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Removing admin enforcement requires admin or owner permissions to the repository and branch protection to be enabled.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#delete-admin-branch-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn delete_admin_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/enforce_admins",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Get pull request review protection.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#get-pull-request-review-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn get_pull_request_review_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<crate::types::ProtectedBranchPullRequestReview>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_pull_request_reviews",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Delete pull request review protection.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#delete-pull-request-review-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn delete_pull_request_review_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_pull_request_reviews",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Update pull request review protection.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/branches/{branch}/protection/required_pull_request_reviews` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Updating pull request review enforcement requires admin or owner permissions to the repository and branch protection to be enabled.
     *
     * > [!NOTE]
     * > Passing new arrays of `users` and `teams` replaces their previous values.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#update-pull-request-review-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn update_pull_request_review_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposUpdatePullRequestReviewProtection,
    ) -> ClientResult<crate::Response<crate::types::ProtectedBranchPullRequestReview>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_pull_request_reviews",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Get commit signature protection.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/branches/{branch}/protection/required_signatures` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * When authenticated with admin or owner permissions to the repository, you can use this endpoint to check whether a branch requires signed commits. An enabled status of `true` indicates you must sign commits on this branch. For more information, see [Signing commits with GPG](https://docs.github.com/articles/signing-commits-with-gpg) in GitHub Help.
     *
     * > [!NOTE]
     * > You must enable branch protection to require signed commits.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#get-commit-signature-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn get_commit_signature_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<crate::types::EnforceAdmins>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_signatures",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Create commit signature protection.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/branches/{branch}/protection/required_signatures` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * When authenticated with admin or owner permissions to the repository, you can use this endpoint to require signed commits on a branch. You must enable branch protection to require signed commits.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#create-commit-signature-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn create_commit_signature_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<crate::types::EnforceAdmins>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_signatures",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Delete commit signature protection.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/branches/{branch}/protection/required_signatures` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * When authenticated with admin or owner permissions to the repository, you can use this endpoint to disable required signed commits on a branch. You must enable branch protection to require signed commits.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#delete-commit-signature-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn delete_commit_signature_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_signatures",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Get status checks protection.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#get-status-checks-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn get_status_checks_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<crate::types::StatusCheckPolicy>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_status_checks",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Remove status check protection.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#remove-status-check-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn remove_status_check_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_status_checks",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Update status check protection.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Updating required status checks requires admin or owner permissions to the repository and branch protection to be enabled.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#update-status-check-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn update_status_check_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposUpdateStatusCheckProtectionRequest,
    ) -> ClientResult<crate::Response<crate::types::StatusCheckPolicy>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_status_checks",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Get all status check contexts.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#get-all-status-check-contexts>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn get_all_status_check_contexts(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<Vec<String>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_status_checks/contexts",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Get all status check contexts.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts` endpoint.
     *
     * As opposed to `get_all_status_check_contexts`, this function returns all the pages of the request at once.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#get-all-status-check-contexts>
     */
    pub async fn get_all_all_status_check_contexts(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<Vec<String>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_status_checks/contexts",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Set status check contexts.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#set-status-check-contexts>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn set_status_check_contexts(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddStatusCheckContextsRequestOneOf,
    ) -> ClientResult<crate::Response<Vec<String>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_status_checks/contexts",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Add status check contexts.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#add-status-check-contexts>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn add_status_check_contexts(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddStatusCheckContextsRequestOneOf,
    ) -> ClientResult<crate::Response<Vec<String>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_status_checks/contexts",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Remove status check contexts.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/branches/{branch}/protection/required_status_checks/contexts` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#remove-status-check-contexts>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn remove_status_check_contexts(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddStatusCheckContextsRequestOneOf,
    ) -> ClientResult<crate::Response<Vec<String>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_status_checks/contexts",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Get access restrictions.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Lists who has access to this protected branch.
     *
     * > [!NOTE]
     * > Users, apps, and teams `restrictions` are only available for organization-owned repositories.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#get-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn get_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<crate::types::BranchRestrictionPolicy>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Delete access restrictions.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Disables the ability to restrict who can push to this branch.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#delete-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn delete_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Get apps with access to the protected branch.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Lists the GitHub Apps that have push access to this branch. Only GitHub Apps that are installed on the repository and that have been granted write access to the repository contents can be added as authorized actors on a protected branch.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#get-apps-with-access-to-the-protected-branch>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn get_apps_with_access_to_protected_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::GitHubApp>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/apps",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Get apps with access to the protected branch.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps` endpoint.
     *
     * As opposed to `get_apps_with_access_to_protected_branch`, this function returns all the pages of the request at once.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Lists the GitHub Apps that have push access to this branch. Only GitHub Apps that are installed on the repository and that have been granted write access to the repository contents can be added as authorized actors on a protected branch.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#get-apps-with-access-to-the-protected-branch>
     */
    pub async fn get_all_apps_with_access_to_protected_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::GitHubApp>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/apps",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Set app access restrictions.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Replaces the list of apps that have push access to this branch. This removes all apps that previously had push access and grants push access to the new list of apps. Only GitHub Apps that are installed on the repository and that have been granted write access to the repository contents can be added as authorized actors on a protected branch.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#set-app-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn set_app_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddAppAccessRestrictionsRequest,
    ) -> ClientResult<crate::Response<Vec<crate::types::GitHubApp>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/apps",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Add app access restrictions.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Grants the specified apps push access for this branch. Only GitHub Apps that are installed on the repository and that have been granted write access to the repository contents can be added as authorized actors on a protected branch.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#add-app-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn add_app_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddAppAccessRestrictionsRequest,
    ) -> ClientResult<crate::Response<Vec<crate::types::GitHubApp>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/apps",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Remove app access restrictions.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/apps` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Removes the ability of an app to push to this branch. Only GitHub Apps that are installed on the repository and that have been granted write access to the repository contents can be added as authorized actors on a protected branch.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#remove-app-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn remove_app_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddAppAccessRestrictionsRequest,
    ) -> ClientResult<crate::Response<Vec<crate::types::GitHubApp>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/apps",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Get teams with access to the protected branch.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Lists the teams who have push access to this branch. The list includes child teams.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#get-teams-with-access-to-the-protected-branch>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn get_teams_with_access_to_protected_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Team>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/teams",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Get teams with access to the protected branch.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams` endpoint.
     *
     * As opposed to `get_teams_with_access_to_protected_branch`, this function returns all the pages of the request at once.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Lists the teams who have push access to this branch. The list includes child teams.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#get-teams-with-access-to-the-protected-branch>
     */
    pub async fn get_all_teams_with_access_to_protected_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Team>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/teams",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Set team access restrictions.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Replaces the list of teams that have push access to this branch. This removes all teams that previously had push access and grants push access to the new list of teams. Team restrictions include child teams.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#set-team-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn set_team_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddTeamAccessRestrictionsRequestOneOf,
    ) -> ClientResult<crate::Response<Vec<crate::types::Team>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/teams",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Add team access restrictions.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Grants the specified teams push access for this branch. You can also give push access to child teams.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#add-team-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn add_team_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddTeamAccessRestrictionsRequestOneOf,
    ) -> ClientResult<crate::Response<Vec<crate::types::Team>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/teams",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Remove team access restrictions.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/teams` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Removes the ability of a team to push to this branch. You can also remove push access for child teams.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#remove-team-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn remove_team_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddTeamAccessRestrictionsRequestOneOf,
    ) -> ClientResult<crate::Response<Vec<crate::types::Team>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/teams",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Get users with access to the protected branch.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Lists the people who have push access to this branch.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#get-users-with-access-to-the-protected-branch>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn get_users_with_access_to_protected_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/users",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Get users with access to the protected branch.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users` endpoint.
     *
     * As opposed to `get_users_with_access_to_protected_branch`, this function returns all the pages of the request at once.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Lists the people who have push access to this branch.
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#get-users-with-access-to-the-protected-branch>
     */
    pub async fn get_all_users_with_access_to_protected_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/users",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Set user access restrictions.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Replaces the list of people that have push access to this branch. This removes all people that previously had push access and grants push access to the new list of people.
     *
     * | Type    | Description                                                                                                                   |
     * | ------- | ----------------------------------------------------------------------------------------------------------------------------- |
     * | `array` | Usernames for people who can have push access. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#set-user-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn set_user_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddUserAccessRestrictionsRequest,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/users",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Add user access restrictions.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Grants the specified people push access for this branch.
     *
     * | Type    | Description                                                                                                                   |
     * | ------- | ----------------------------------------------------------------------------------------------------------------------------- |
     * | `array` | Usernames for people who can have push access. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#add-user-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn add_user_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddUserAccessRestrictionsRequest,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/users",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Remove user access restrictions.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/branches/{branch}/protection/restrictions/users` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Removes the ability of a user to push to this branch.
     *
     * | Type    | Description                                                                                                                                   |
     * | ------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
     * | `array` | Usernames of the people who should no longer have push access. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
     *
     * FROM: <https://docs.github.com/rest/branches/branch-protection#remove-user-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn remove_user_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddUserAccessRestrictionsRequest,
    ) -> ClientResult<crate::Response<Vec<crate::types::SimpleUser>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/users",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Rename a branch.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/branches/{branch}/rename` endpoint.
     *
     * Renames a branch in a repository.
     *
     * > [!NOTE]
     * > Although the API responds immediately, the branch rename process might take some extra time to complete in the background. You won't be able to push to the old branch name while the rename process is in progress. For more information, see "[Renaming a branch](https://docs.github.com/github/administering-a-repository/renaming-a-branch)".
     *
     * The authenticated user must have push access to the branch. If the branch is the default branch, the authenticated user must also have admin or owner permissions.
     *
     * In order to rename the default branch, fine-grained access tokens also need the `administration:write` repository permission.
     *
     * FROM: <https://docs.github.com/rest/branches/branches#rename-a-branch>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     */
    pub async fn rename_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposRenameBranchRequest,
    ) -> ClientResult<crate::Response<crate::types::BranchWithProtection>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/rename",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * List CODEOWNERS errors.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/codeowners/errors` endpoint.
     *
     * List any syntax errors that are detected in the CODEOWNERS
     * file.
     *
     * For more information about the correct CODEOWNERS syntax,
     * see "[About code owners](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners)."
     *
     * FROM: <https://docs.github.com/rest/repos/repos#list-codeowners-errors>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `ref_: &str` -- A branch, tag or commit name used to determine which version of the CODEOWNERS file to use. Default: the repository's default branch (e.g. `main`).
     */
    pub async fn codeowners_errors(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> ClientResult<crate::Response<crate::types::CodeownersErrorsData>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/codeowners/errors?{}",
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
     * List repository collaborators.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/collaborators` endpoint.
     *
     * For organization-owned repositories, the list of collaborators includes outside collaborators, organization members that are direct collaborators, organization members with access through team memberships, organization members with access through default organization permissions, and organization owners.
     * The `permissions` hash returned in the response contains the base role permissions of the collaborator. The `role_name` is the highest role assigned to the collaborator after considering all sources of grants, including: repo, teams, organization, and enterprise.
     * There is presently not a way to differentiate between an organization level grant and a repository level grant from this endpoint response.
     *
     * Team members will include the members of child teams.
     *
     * The authenticated user must have write, maintain, or admin privileges on the repository to use this endpoint. For organization-owned repositories, the authenticated user needs to be a member of the organization.
     * OAuth app tokens and personal access tokens (classic) need the `read:org` and `repo` scopes to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/collaborators/collaborators#list-repository-collaborators>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `affiliation: crate::types::Affiliation` -- Filter collaborators returned by their affiliation. `outside` means all outside collaborators of an organization-owned repository. `direct` means all collaborators with permissions to an organization-owned repository, regardless of organization membership status. `all` means all collaborators the authenticated user can see.
     * * `permission: crate::types::ReposListCollaboratorsPermission` -- Filter collaborators by the permissions they have on the repository. If not specified, all collaborators will be returned.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_collaborators(
        &self,
        owner: &str,
        repo: &str,
        affiliation: crate::types::Affiliation,
        permission: crate::types::ReposListCollaboratorsPermission,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Collaborator>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !affiliation.to_string().is_empty() {
            query_args.push(("affiliation".to_string(), affiliation.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !permission.to_string().is_empty() {
            query_args.push(("permission".to_string(), permission.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/collaborators?{}",
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
     * List repository collaborators.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/collaborators` endpoint.
     *
     * As opposed to `list_collaborators`, this function returns all the pages of the request at once.
     *
     * For organization-owned repositories, the list of collaborators includes outside collaborators, organization members that are direct collaborators, organization members with access through team memberships, organization members with access through default organization permissions, and organization owners.
     * The `permissions` hash returned in the response contains the base role permissions of the collaborator. The `role_name` is the highest role assigned to the collaborator after considering all sources of grants, including: repo, teams, organization, and enterprise.
     * There is presently not a way to differentiate between an organization level grant and a repository level grant from this endpoint response.
     *
     * Team members will include the members of child teams.
     *
     * The authenticated user must have write, maintain, or admin privileges on the repository to use this endpoint. For organization-owned repositories, the authenticated user needs to be a member of the organization.
     * OAuth app tokens and personal access tokens (classic) need the `read:org` and `repo` scopes to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/collaborators/collaborators#list-repository-collaborators>
     */
    pub async fn list_all_collaborators(
        &self,
        owner: &str,
        repo: &str,
        affiliation: crate::types::Affiliation,
        permission: crate::types::ReposListCollaboratorsPermission,
    ) -> ClientResult<crate::Response<Vec<crate::types::Collaborator>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !affiliation.to_string().is_empty() {
            query_args.push(("affiliation".to_string(), affiliation.to_string()));
        }
        if !permission.to_string().is_empty() {
            query_args.push(("permission".to_string(), permission.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/collaborators?{}",
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
     * Check if a user is a repository collaborator.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/collaborators/{username}` endpoint.
     *
     * For organization-owned repositories, the list of collaborators includes outside collaborators, organization members that are direct collaborators, organization members with access through team memberships, organization members with access through default organization permissions, and organization owners.
     *
     * Team members will include the members of child teams.
     *
     * The authenticated user must have push access to the repository to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:org` and `repo` scopes to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/collaborators/collaborators#check-if-a-user-is-a-repository-collaborator>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn check_collaborator(
        &self,
        owner: &str,
        repo: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/collaborators/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Add a repository collaborator.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/collaborators/{username}` endpoint.
     *
     * Add a user to a repository with a specified level of access. If the repository is owned by an organization, this API does not add the user to the organization - a user that has repository access without being an organization member is called an "outside collaborator" (if they are not an Enterprise Managed User) or a "repository collaborator" if they are an Enterprise Managed User. These users are exempt from some organization policies - see "[Adding outside collaborators to repositories](https://docs.github.com/organizations/managing-user-access-to-your-organizations-repositories/managing-outside-collaborators/adding-outside-collaborators-to-repositories-in-your-organization)" to learn more about these collaborator types.
     *
     * This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications).
     *
     * Adding an outside collaborator may be restricted by enterprise and organization administrators. For more information, see "[Enforcing repository management policies in your enterprise](https://docs.github.com/admin/policies/enforcing-policies-for-your-enterprise/enforcing-repository-management-policies-in-your-enterprise#enforcing-a-policy-for-inviting-outside-collaborators-to-repositories)" and "[Setting permissions for adding outside collaborators](https://docs.github.com/organizations/managing-organization-settings/setting-permissions-for-adding-outside-collaborators)" for organization settings.
     *
     * For more information on permission levels, see "[Repository permission levels for an organization](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/repository-permission-levels-for-an-organization#permission-levels-for-repositories-owned-by-an-organization)". There are restrictions on which permissions can be granted to organization members when an organization base role is in place. In this case, the role being given must be equal to or higher than the org base permission. Otherwise, the request will fail with:
     *
     * ```
     * Cannot assign {member} permission of {role name}
     * ```
     *
     * Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP method](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#http-method)."
     *
     * The invitee will receive a notification that they have been invited to the repository, which they must accept or decline. They may do this via the notifications page, the email they receive, or by using the [API](https://docs.github.com/rest/collaborators/invitations).
     *
     * For Enterprise Managed Users, this endpoint does not send invitations - these users are automatically added to organizations and repositories. Enterprise Managed Users can only be added to organizations and repositories within their enterprise.
     *
     * **Updating an existing collaborator's permission level**
     *
     * The endpoint can also be used to change the permissions of an existing collaborator without first removing and re-adding the collaborator. To change the permissions, use the same endpoint and pass a different `permission` parameter. The response will be a `204`, with no other indication that the permission level changed.
     *
     * **Rate limits**
     *
     * You are limited to sending 50 invitations to a repository per 24 hour period. Note there is no limit if you are inviting organization members to an organization repository.
     *
     * FROM: <https://docs.github.com/rest/collaborators/collaborators#add-a-repository-collaborator>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn add_collaborator(
        &self,
        owner: &str,
        repo: &str,
        username: &str,
        body: &crate::types::ReposAddCollaboratorRequest,
    ) -> ClientResult<crate::Response<crate::types::RepositoryInvitation>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/collaborators/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Remove a repository collaborator.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/collaborators/{username}` endpoint.
     *
     * Removes a collaborator from a repository.
     *
     * To use this endpoint, the authenticated user must either be an administrator of the repository or target themselves for removal.
     *
     * This endpoint also:
     * - Cancels any outstanding invitations sent by the collaborator
     * - Unassigns the user from any issues
     * - Removes access to organization projects if the user is not an organization member and is not a collaborator on any other organization repositories.
     * - Unstars the repository
     * - Updates access permissions to packages
     *
     * Removing a user as a collaborator has the following effects on forks:
     *  - If the user had access to a fork through their membership to this repository, the user will also be removed from the fork.
     *  - If the user had their own fork of the repository, the fork will be deleted.
     *  - If the user still has read access to the repository, open pull requests by this user from a fork will be denied.
     *
     * > [!NOTE]
     * > A user can still have access to the repository through organization permissions like base repository permissions.
     *
     * Although the API responds immediately, the additional permission updates might take some extra time to complete in the background.
     *
     * For more information on fork permissions, see "[About permissions and visibility of forks](https://docs.github.com/pull-requests/collaborating-with-pull-requests/working-with-forks/about-permissions-and-visibility-of-forks)".
     *
     * FROM: <https://docs.github.com/rest/collaborators/collaborators#remove-a-repository-collaborator>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn remove_collaborator(
        &self,
        owner: &str,
        repo: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/collaborators/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * Get repository permissions for a user.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/collaborators/{username}/permission` endpoint.
     *
     * Checks the repository permission and role of a collaborator.
     *
     * The `permission` attribute provides the legacy base roles of `admin`, `write`, `read`, and `none`, where the
     * `maintain` role is mapped to `write` and the `triage` role is mapped to `read`.
     * The `role_name` attribute provides the name of the assigned role, including custom roles. The
     * `permission` can also be used to determine which base level of access the collaborator has to the repository.
     *
     * The calculated permissions are the highest role assigned to the collaborator after considering all sources of grants, including: repo, teams, organization, and enterprise.
     * There is presently not a way to differentiate between an organization level grant and a repository level grant from this endpoint response.
     *
     * FROM: <https://docs.github.com/rest/collaborators/collaborators#get-repository-permissions-for-a-user>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn get_collaborator_permission_level(
        &self,
        owner: &str,
        repo: &str,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::RepositoryCollaboratorPermission>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/collaborators/{}/permission",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * List commit comments for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/comments` endpoint.
     *
     * Lists the commit comments for a specified repository. Comments are ordered by ascending ID.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/commits/comments#list-commit-comments-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_commit_comments_for_repo(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::CommitComment>>> {
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
                "/repos/{}/{}/comments?{}",
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
     * List commit comments for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/comments` endpoint.
     *
     * As opposed to `list_commit_comments_for_repo`, this function returns all the pages of the request at once.
     *
     * Lists the commit comments for a specified repository. Comments are ordered by ascending ID.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/commits/comments#list-commit-comments-for-a-repository>
     */
    pub async fn list_all_commit_comments_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CommitComment>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/comments",
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
     * Get a commit comment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/comments/{comment_id}` endpoint.
     *
     * Gets a specified commit comment.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/commits/comments#get-a-commit-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `comment_id: i64` -- The unique identifier of the comment.
     */
    pub async fn get_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> ClientResult<crate::Response<crate::types::CommitComment>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/comments/{}",
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
     * Delete a commit comment.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/comments/{comment_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/commits/comments#delete-a-commit-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `comment_id: i64` -- The unique identifier of the comment.
     */
    pub async fn delete_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/comments/{}",
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
     * Update a commit comment.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/comments/{comment_id}` endpoint.
     *
     * Updates the contents of a specified commit comment.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/commits/comments#update-a-commit-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `comment_id: i64` -- The unique identifier of the comment.
     */
    pub async fn update_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &crate::types::PullsUpdateReviewRequest,
    ) -> ClientResult<crate::Response<crate::types::CommitComment>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/comments/{}",
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
     * List commits.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/commits` endpoint.
     *
     * **Signature verification object**
     *
     * The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
     *
     * | Name | Type | Description |
     * | ---- | ---- | ----------- |
     * | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
     * | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
     * | `signature` | `string` | The signature that was extracted from the commit. |
     * | `payload` | `string` | The value that was signed. |
     * | `verified_at` | `string` | The date the signature was verified by GitHub. |
     *
     * These are the possible values for `reason` in the `verification` object:
     *
     * | Value | Description |
     * | ----- | ----------- |
     * | `expired_key` | The key that made the signature is expired. |
     * | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
     * | `gpgverify_error` | There was an error communicating with the signature verification service. |
     * | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
     * | `unsigned` | The object does not include a signature. |
     * | `unknown_signature_type` | A non-PGP signature was found in the commit. |
     * | `no_user` | No user was associated with the `committer` email address in the commit. |
     * | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on their account. |
     * | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
     * | `unknown_key` | The key that made the signature has not been registered with any user's account. |
     * | `malformed_signature` | There was an error parsing the signature. |
     * | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
     * | `valid` | None of the above errors applied, so the signature is considered to be verified. |
     *
     * FROM: <https://docs.github.com/rest/commits/commits#list-commits>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `sha: &str` -- SHA or branch to start listing commits from. Default: the repository’s default branch (usually `main`).
     * * `path: &str` -- Only commits containing this file path will be returned.
     * * `author: &str` -- GitHub username or email address to use to filter by commit author.
     * * `committer: &str` -- GitHub username or email address to use to filter by commit committer.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. Due to limitations of Git, timestamps must be between 1970-01-01 and 2099-12-31 (inclusive) or unexpected results may be returned.
     * * `until: chrono::DateTime<chrono::Utc>` -- Only commits before this date will be returned. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`. Due to limitations of Git, timestamps must be between 1970-01-01 and 2099-12-31 (inclusive) or unexpected results may be returned.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_commits(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        path: &str,
        author: &str,
        committer: &str,
        since: Option<chrono::DateTime<chrono::Utc>>,
        until: Option<chrono::DateTime<chrono::Utc>>,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::CommitDataType>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !author.is_empty() {
            query_args.push(("author".to_string(), author.to_string()));
        }
        if !committer.is_empty() {
            query_args.push(("committer".to_string(), committer.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if !path.is_empty() {
            query_args.push(("path".to_string(), path.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !sha.is_empty() {
            query_args.push(("sha".to_string(), sha.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        if let Some(date) = until {
            query_args.push(("until".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/commits?{}",
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
     * List commits.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/commits` endpoint.
     *
     * As opposed to `list_commits`, this function returns all the pages of the request at once.
     *
     * **Signature verification object**
     *
     * The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
     *
     * | Name | Type | Description |
     * | ---- | ---- | ----------- |
     * | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
     * | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
     * | `signature` | `string` | The signature that was extracted from the commit. |
     * | `payload` | `string` | The value that was signed. |
     * | `verified_at` | `string` | The date the signature was verified by GitHub. |
     *
     * These are the possible values for `reason` in the `verification` object:
     *
     * | Value | Description |
     * | ----- | ----------- |
     * | `expired_key` | The key that made the signature is expired. |
     * | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
     * | `gpgverify_error` | There was an error communicating with the signature verification service. |
     * | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
     * | `unsigned` | The object does not include a signature. |
     * | `unknown_signature_type` | A non-PGP signature was found in the commit. |
     * | `no_user` | No user was associated with the `committer` email address in the commit. |
     * | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on their account. |
     * | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
     * | `unknown_key` | The key that made the signature has not been registered with any user's account. |
     * | `malformed_signature` | There was an error parsing the signature. |
     * | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
     * | `valid` | None of the above errors applied, so the signature is considered to be verified. |
     *
     * FROM: <https://docs.github.com/rest/commits/commits#list-commits>
     */
    pub async fn list_all_commits(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        path: &str,
        author: &str,
        committer: &str,
        since: Option<chrono::DateTime<chrono::Utc>>,
        until: Option<chrono::DateTime<chrono::Utc>>,
    ) -> ClientResult<crate::Response<Vec<crate::types::CommitDataType>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !author.is_empty() {
            query_args.push(("author".to_string(), author.to_string()));
        }
        if !committer.is_empty() {
            query_args.push(("committer".to_string(), committer.to_string()));
        }
        if !path.is_empty() {
            query_args.push(("path".to_string(), path.to_string()));
        }
        if !sha.is_empty() {
            query_args.push(("sha".to_string(), sha.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        if let Some(date) = until {
            query_args.push(("until".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/commits?{}",
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
     * List branches for HEAD commit.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/commits/{commit_sha}/branches-where-head` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Returns all branches where the given commit SHA is the HEAD, or latest commit for the branch.
     *
     * FROM: <https://docs.github.com/rest/commits/commits#list-branches-for-head-commit>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `commit_sha: &str` -- The SHA of the commit.
     */
    pub async fn list_branches_for_head_commit(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::BranchShort>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/commits/{}/branches-where-head",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&commit_sha.to_string()),
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
     * List branches for HEAD commit.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/commits/{commit_sha}/branches-where-head` endpoint.
     *
     * As opposed to `list_branches_for_head_commit`, this function returns all the pages of the request at once.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Returns all branches where the given commit SHA is the HEAD, or latest commit for the branch.
     *
     * FROM: <https://docs.github.com/rest/commits/commits#list-branches-for-head-commit>
     */
    pub async fn list_all_branches_for_head_commit(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::BranchShort>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/commits/{}/branches-where-head",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&commit_sha.to_string()),
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
     * List commit comments.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/commits/{commit_sha}/comments` endpoint.
     *
     * Lists the comments for a specified commit.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/commits/comments#list-commit-comments>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `commit_sha: &str` -- The SHA of the commit.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_comments_for_commit(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::CommitComment>>> {
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
                "/repos/{}/{}/commits/{}/comments?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&commit_sha.to_string()),
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
     * List commit comments.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/commits/{commit_sha}/comments` endpoint.
     *
     * As opposed to `list_comments_for_commit`, this function returns all the pages of the request at once.
     *
     * Lists the comments for a specified commit.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/commits/comments#list-commit-comments>
     */
    pub async fn list_all_comments_for_commit(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CommitComment>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/commits/{}/comments",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&commit_sha.to_string()),
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
     * Create a commit comment.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/commits/{commit_sha}/comments` endpoint.
     *
     * Create a comment for a commit using its `:commit_sha`.
     *
     * This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)" and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github-commitcomment.raw+json`**: Returns the raw markdown body. Response will include `body`. This is the default if you do not pass any specific media type.
     * - **`application/vnd.github-commitcomment.text+json`**: Returns a text only representation of the markdown body. Response will include `body_text`.
     * - **`application/vnd.github-commitcomment.html+json`**: Returns HTML rendered from the body's markdown. Response will include `body_html`.
     * - **`application/vnd.github-commitcomment.full+json`**: Returns raw, text, and HTML representations. Response will include `body`, `body_text`, and `body_html`.
     *
     * FROM: <https://docs.github.com/rest/commits/comments#create-a-commit-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `commit_sha: &str` -- The SHA of the commit.
     */
    pub async fn create_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
        body: &crate::types::ReposCreateCommitCommentRequest,
    ) -> ClientResult<crate::Response<crate::types::CommitComment>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/commits/{}/comments",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&commit_sha.to_string()),
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
     * List pull requests associated with a commit.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/commits/{commit_sha}/pulls` endpoint.
     *
     * Lists the merged pull request that introduced the commit to the repository. If the commit is not present in the default branch, it will return merged and open pull requests associated with the commit.
     *
     * To list the open or merged pull requests associated with a branch, you can set the `commit_sha` parameter to the branch name.
     *
     * FROM: <https://docs.github.com/rest/commits/commits#list-pull-requests-associated-with-a-commit>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `commit_sha: &str` -- The SHA of the commit.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_pull_requests_associated_with_commit(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::PullRequestSimple>>> {
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
                "/repos/{}/{}/commits/{}/pulls?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&commit_sha.to_string()),
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
     * List pull requests associated with a commit.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/commits/{commit_sha}/pulls` endpoint.
     *
     * As opposed to `list_pull_requests_associated_with_commit`, this function returns all the pages of the request at once.
     *
     * Lists the merged pull request that introduced the commit to the repository. If the commit is not present in the default branch, it will return merged and open pull requests associated with the commit.
     *
     * To list the open or merged pull requests associated with a branch, you can set the `commit_sha` parameter to the branch name.
     *
     * FROM: <https://docs.github.com/rest/commits/commits#list-pull-requests-associated-with-a-commit>
     */
    pub async fn list_all_pull_requests_associated_with_commit(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::PullRequestSimple>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/commits/{}/pulls",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&commit_sha.to_string()),
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
     * Get a commit.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/commits/{ref}` endpoint.
     *
     * Returns the contents of a single commit reference. You must have `read` access for the repository to use this endpoint.
     *
     * > [!NOTE]
     * > If there are more than 300 files in the commit diff and the default JSON media type is requested, the response will include pagination link headers for the remaining files, up to a limit of 3000 files. Each page contains the static commit information, and the only changes are to the file listing.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)." Pagination query parameters are not supported for these media types.
     *
     * - **`application/vnd.github.diff`**: Returns the diff of the commit. Larger diffs may time out and return a 5xx status code.
     * - **`application/vnd.github.patch`**: Returns the patch of the commit. Diffs with binary data will have no `patch` property. Larger diffs may time out and return a 5xx status code.
     * - **`application/vnd.github.sha`**: Returns the commit's SHA-1 hash. You can use this endpoint to check if a remote reference's SHA-1 hash is the same as your local reference's SHA-1 hash by providing the local SHA-1 reference as the ETag.
     *
     * **Signature verification object**
     *
     * The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
     *
     * | Name | Type | Description |
     * | ---- | ---- | ----------- |
     * | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
     * | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
     * | `signature` | `string` | The signature that was extracted from the commit. |
     * | `payload` | `string` | The value that was signed. |
     * | `verified_at` | `string` | The date the signature was verified by GitHub. |
     *
     * These are the possible values for `reason` in the `verification` object:
     *
     * | Value | Description |
     * | ----- | ----------- |
     * | `expired_key` | The key that made the signature is expired. |
     * | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
     * | `gpgverify_error` | There was an error communicating with the signature verification service. |
     * | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
     * | `unsigned` | The object does not include a signature. |
     * | `unknown_signature_type` | A non-PGP signature was found in the commit. |
     * | `no_user` | No user was associated with the `committer` email address in the commit. |
     * | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on their account. |
     * | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
     * | `unknown_key` | The key that made the signature has not been registered with any user's account. |
     * | `malformed_signature` | There was an error parsing the signature. |
     * | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
     * | `valid` | None of the above errors applied, so the signature is considered to be verified. |
     *
     * FROM: <https://docs.github.com/rest/commits/commits#get-a-commit>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `ref_: &str` -- The commit reference. Can be a commit SHA, branch name (`heads/BRANCH_NAME`), or tag name (`tags/TAG_NAME`). For more information, see "[Git References](https://git-scm.com/book/en/v2/Git-Internals-Git-References)" in the Git documentation.
     */
    pub async fn get_commit(
        &self,
        owner: &str,
        repo: &str,
        page: i64,
        per_page: i64,
        ref_: &str,
    ) -> ClientResult<crate::Response<crate::types::CommitDataType>> {
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
                "/repos/{}/{}/commits/{}?{}",
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
     * Get the combined status for a specific reference.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/commits/{ref}/status` endpoint.
     *
     * Users with pull access in a repository can access a combined view of commit statuses for a given ref. The ref can be a SHA, a branch name, or a tag name.
     *
     *
     * Additionally, a combined `state` is returned. The `state` is one of:
     *
     * *   **failure** if any of the contexts report as `error` or `failure`
     * *   **pending** if there are no statuses or a context is `pending`
     * *   **success** if the latest status for all contexts is `success`
     *
     * FROM: <https://docs.github.com/rest/commits/statuses#get-the-combined-status-for-a-specific-reference>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `ref_: &str` -- The commit reference. Can be a commit SHA, branch name (`heads/BRANCH_NAME`), or tag name (`tags/TAG_NAME`). For more information, see "[Git References](https://git-scm.com/book/en/v2/Git-Internals-Git-References)" in the Git documentation.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn get_combined_status_for_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::CombinedCommitStatus>> {
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
                "/repos/{}/{}/commits/{}/status?{}",
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
     * List commit statuses for a reference.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/commits/{ref}/statuses` endpoint.
     *
     * Users with pull access in a repository can view commit statuses for a given ref. The ref can be a SHA, a branch name, or a tag name. Statuses are returned in reverse chronological order. The first status in the list will be the latest one.
     *
     * This resource is also available via a legacy route: `GET /repos/:owner/:repo/statuses/:ref`.
     *
     * FROM: <https://docs.github.com/rest/commits/statuses#list-commit-statuses-for-a-reference>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `ref_: &str` -- The commit reference. Can be a commit SHA, branch name (`heads/BRANCH_NAME`), or tag name (`tags/TAG_NAME`). For more information, see "[Git References](https://git-scm.com/book/en/v2/Git-Internals-Git-References)" in the Git documentation.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_commit_statuses_for_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::StatusData>>> {
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
                "/repos/{}/{}/commits/{}/statuses?{}",
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
     * List commit statuses for a reference.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/commits/{ref}/statuses` endpoint.
     *
     * As opposed to `list_commit_statuses_for_ref`, this function returns all the pages of the request at once.
     *
     * Users with pull access in a repository can view commit statuses for a given ref. The ref can be a SHA, a branch name, or a tag name. Statuses are returned in reverse chronological order. The first status in the list will be the latest one.
     *
     * This resource is also available via a legacy route: `GET /repos/:owner/:repo/statuses/:ref`.
     *
     * FROM: <https://docs.github.com/rest/commits/statuses#list-commit-statuses-for-a-reference>
     */
    pub async fn list_all_commit_statuses_for_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::StatusData>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/commits/{}/statuses",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&ref_.to_string()),
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
     * Get community profile metrics.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/community/profile` endpoint.
     *
     * Returns all community profile metrics for a repository. The repository cannot be a fork.
     *
     * The returned metrics include an overall health score, the repository description, the presence of documentation, the
     * detected code of conduct, the detected license, and the presence of ISSUE\_TEMPLATE, PULL\_REQUEST\_TEMPLATE,
     * README, and CONTRIBUTING files.
     *
     * The `health_percentage` score is defined as a percentage of how many of
     * the recommended community health files are present. For more information, see
     * "[About community profiles for public repositories](https://docs.github.com/communities/setting-up-your-project-for-healthy-contributions/about-community-profiles-for-public-repositories)."
     *
     * `content_reports_enabled` is only returned for organization-owned repositories.
     *
     * FROM: <https://docs.github.com/rest/metrics/community#get-community-profile-metrics>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_community_profile_metrics(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::CommunityProfile>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/community/profile",
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
     * Compare two commits.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/compare/{basehead}` endpoint.
     *
     * Compares two commits against one another. You can compare refs (branches or tags) and commit SHAs in the same repository, or you can compare refs and commit SHAs that exist in different repositories within the same repository network, including fork branches. For more information about how to view a repository's network, see "[Understanding connections between repositories](https://docs.github.com/repositories/viewing-activity-and-data-for-your-repository/understanding-connections-between-repositories)."
     *
     * This endpoint is equivalent to running the `git log BASE..HEAD` command, but it returns commits in a different order. The `git log BASE..HEAD` command returns commits in reverse chronological order, whereas the API returns commits in chronological order.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.diff`**: Returns the diff of the commit.
     * - **`application/vnd.github.patch`**: Returns the patch of the commit. Diffs with binary data will have no `patch` property.
     *
     * The API response includes details about the files that were changed between the two commits. This includes the status of the change (if a file was added, removed, modified, or renamed), and details of the change itself. For example, files with a `renamed` status have a `previous_filename` field showing the previous filename of the file, and files with a `modified` status have a `patch` field showing the changes made to the file.
     *
     * When calling this endpoint without any paging parameter (`per_page` or `page`), the returned list is limited to 250 commits, and the last commit in the list is the most recent of the entire comparison.
     *
     * **Working with large comparisons**
     *
     * To process a response with a large number of commits, use a query parameter (`per_page` or `page`) to paginate the results. When using pagination:
     *
     * - The list of changed files is only shown on the first page of results, and it includes up to 300 changed files for the entire comparison.
     * - The results are returned in chronological order, but the last commit in the returned list may not be the most recent one in the entire set if there are more pages of results.
     *
     * For more information on working with pagination, see "[Using pagination in the REST API](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api)."
     *
     * **Signature verification object**
     *
     * The response will include a `verification` object that describes the result of verifying the commit's signature. The `verification` object includes the following fields:
     *
     * | Name | Type | Description |
     * | ---- | ---- | ----------- |
     * | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
     * | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
     * | `signature` | `string` | The signature that was extracted from the commit. |
     * | `payload` | `string` | The value that was signed. |
     * | `verified_at` | `string` | The date the signature was verified by GitHub. |
     *
     * These are the possible values for `reason` in the `verification` object:
     *
     * | Value | Description |
     * | ----- | ----------- |
     * | `expired_key` | The key that made the signature is expired. |
     * | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
     * | `gpgverify_error` | There was an error communicating with the signature verification service. |
     * | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
     * | `unsigned` | The object does not include a signature. |
     * | `unknown_signature_type` | A non-PGP signature was found in the commit. |
     * | `no_user` | No user was associated with the `committer` email address in the commit. |
     * | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on their account. |
     * | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
     * | `unknown_key` | The key that made the signature has not been registered with any user's account. |
     * | `malformed_signature` | There was an error parsing the signature. |
     * | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
     * | `valid` | None of the above errors applied, so the signature is considered to be verified. |
     *
     * FROM: <https://docs.github.com/rest/commits/commits#compare-two-commits>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `basehead: &str` -- The base branch and head branch to compare. This parameter expects the format `BASE...HEAD`. Both must be branch names in `repo`. To compare with a branch that exists in a different repository in the same network as `repo`, the `basehead` parameter expects the format `USERNAME:BASE...USERNAME:HEAD`.
     */
    pub async fn compare_commits(
        &self,
        owner: &str,
        repo: &str,
        page: i64,
        per_page: i64,
        basehead: &str,
    ) -> ClientResult<crate::Response<crate::types::CommitComparison>> {
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
                "/repos/{}/{}/compare/{}?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&basehead.to_string()),
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
     * Get repository content.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/contents/{path}` endpoint.
     *
     * Gets the contents of a file or directory in a repository. Specify the file path or directory with the `path` parameter. If you omit the `path` parameter, you will receive the contents of the repository's root directory.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw file contents for files and symlinks.
     * - **`application/vnd.github.html+json`**: Returns the file contents in HTML. Markup languages are rendered to HTML using GitHub's open-source [Markup library](https://github.com/github/markup).
     * - **`application/vnd.github.object+json`**: Returns the contents in a consistent object format regardless of the content type. For example, instead of an array of objects for a directory, the response will be an object with an `entries` attribute containing the array of objects.
     *
     * If the content is a directory, the response will be an array of objects, one object for each item in the directory. When listing the contents of a directory, submodules have their "type" specified as "file". Logically, the value _should_ be "submodule". This behavior exists [for backwards compatibility purposes](https://git.io/v1YCW). In the next major version of the API, the type will be returned as "submodule".
     *
     * If the content is a symlink and the symlink's target is a normal file in the repository, then the API responds with the content of the file. Otherwise, the API responds with an object describing the symlink itself.
     *
     * If the content is a submodule, the `submodule_git_url` field identifies the location of the submodule repository, and the `sha` identifies a specific commit within the submodule repository. Git uses the given URL when cloning the submodule repository, and checks out the submodule at that specific commit. If the submodule repository is not hosted on github.com, the Git URLs (`git_url` and `_links["git"]`) and the github.com URLs (`html_url` and `_links["html"]`) will have null values.
     *
     * **Notes**:
     *
     * - To get a repository's contents recursively, you can [recursively get the tree](https://docs.github.com/rest/git/trees#get-a-tree).
     * - This API has an upper limit of 1,000 files for a directory. If you need to retrieve
     * more files, use the [Git Trees API](https://docs.github.com/rest/git/trees#get-a-tree).
     * - Download URLs expire and are meant to be used just once. To ensure the download URL does not expire, please use the contents API to obtain a fresh download URL for each download.
     * - If the requested file's size is:
     *   - 1 MB or smaller: All features of this endpoint are supported.
     *   - Between 1-100 MB: Only the `raw` or `object` custom media types are supported. Both will work as normal, except that when using the `object` media type, the `content` field will be an empty
     * string and the `encoding` field will be `"none"`. To get the contents of these larger files, use the `raw` media type.
     *   - Greater than 100 MB: This endpoint is not supported.
     *
     * FROM: <https://docs.github.com/rest/repos/contents#get-repository-content>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `path: &str` -- path parameter.
     * * `ref_: &str` -- The name of the commit/branch/tag. Default: the repository’s default branch.
     */
    pub async fn get_content_vec_directory(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        ref_: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ContentDirectory>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/contents/{}?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&path.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Get repository content.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/contents/{path}` endpoint.
     *
     * Gets the contents of a file or directory in a repository. Specify the file path or directory with the `path` parameter. If you omit the `path` parameter, you will receive the contents of the repository's root directory.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw file contents for files and symlinks.
     * - **`application/vnd.github.html+json`**: Returns the file contents in HTML. Markup languages are rendered to HTML using GitHub's open-source [Markup library](https://github.com/github/markup).
     * - **`application/vnd.github.object+json`**: Returns the contents in a consistent object format regardless of the content type. For example, instead of an array of objects for a directory, the response will be an object with an `entries` attribute containing the array of objects.
     *
     * If the content is a directory, the response will be an array of objects, one object for each item in the directory. When listing the contents of a directory, submodules have their "type" specified as "file". Logically, the value _should_ be "submodule". This behavior exists [for backwards compatibility purposes](https://git.io/v1YCW). In the next major version of the API, the type will be returned as "submodule".
     *
     * If the content is a symlink and the symlink's target is a normal file in the repository, then the API responds with the content of the file. Otherwise, the API responds with an object describing the symlink itself.
     *
     * If the content is a submodule, the `submodule_git_url` field identifies the location of the submodule repository, and the `sha` identifies a specific commit within the submodule repository. Git uses the given URL when cloning the submodule repository, and checks out the submodule at that specific commit. If the submodule repository is not hosted on github.com, the Git URLs (`git_url` and `_links["git"]`) and the github.com URLs (`html_url` and `_links["html"]`) will have null values.
     *
     * **Notes**:
     *
     * - To get a repository's contents recursively, you can [recursively get the tree](https://docs.github.com/rest/git/trees#get-a-tree).
     * - This API has an upper limit of 1,000 files for a directory. If you need to retrieve
     * more files, use the [Git Trees API](https://docs.github.com/rest/git/trees#get-a-tree).
     * - Download URLs expire and are meant to be used just once. To ensure the download URL does not expire, please use the contents API to obtain a fresh download URL for each download.
     * - If the requested file's size is:
     *   - 1 MB or smaller: All features of this endpoint are supported.
     *   - Between 1-100 MB: Only the `raw` or `object` custom media types are supported. Both will work as normal, except that when using the `object` media type, the `content` field will be an empty
     * string and the `encoding` field will be `"none"`. To get the contents of these larger files, use the `raw` media type.
     *   - Greater than 100 MB: This endpoint is not supported.
     *
     * FROM: <https://docs.github.com/rest/repos/contents#get-repository-content>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `path: &str` -- path parameter.
     * * `ref_: &str` -- The name of the commit/branch/tag. Default: the repository’s default branch.
     */
    pub async fn get_content_file(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        ref_: &str,
    ) -> ClientResult<crate::Response<crate::types::ContentFile>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/contents/{}?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&path.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Get repository content.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/contents/{path}` endpoint.
     *
     * Gets the contents of a file or directory in a repository. Specify the file path or directory with the `path` parameter. If you omit the `path` parameter, you will receive the contents of the repository's root directory.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw file contents for files and symlinks.
     * - **`application/vnd.github.html+json`**: Returns the file contents in HTML. Markup languages are rendered to HTML using GitHub's open-source [Markup library](https://github.com/github/markup).
     * - **`application/vnd.github.object+json`**: Returns the contents in a consistent object format regardless of the content type. For example, instead of an array of objects for a directory, the response will be an object with an `entries` attribute containing the array of objects.
     *
     * If the content is a directory, the response will be an array of objects, one object for each item in the directory. When listing the contents of a directory, submodules have their "type" specified as "file". Logically, the value _should_ be "submodule". This behavior exists [for backwards compatibility purposes](https://git.io/v1YCW). In the next major version of the API, the type will be returned as "submodule".
     *
     * If the content is a symlink and the symlink's target is a normal file in the repository, then the API responds with the content of the file. Otherwise, the API responds with an object describing the symlink itself.
     *
     * If the content is a submodule, the `submodule_git_url` field identifies the location of the submodule repository, and the `sha` identifies a specific commit within the submodule repository. Git uses the given URL when cloning the submodule repository, and checks out the submodule at that specific commit. If the submodule repository is not hosted on github.com, the Git URLs (`git_url` and `_links["git"]`) and the github.com URLs (`html_url` and `_links["html"]`) will have null values.
     *
     * **Notes**:
     *
     * - To get a repository's contents recursively, you can [recursively get the tree](https://docs.github.com/rest/git/trees#get-a-tree).
     * - This API has an upper limit of 1,000 files for a directory. If you need to retrieve
     * more files, use the [Git Trees API](https://docs.github.com/rest/git/trees#get-a-tree).
     * - Download URLs expire and are meant to be used just once. To ensure the download URL does not expire, please use the contents API to obtain a fresh download URL for each download.
     * - If the requested file's size is:
     *   - 1 MB or smaller: All features of this endpoint are supported.
     *   - Between 1-100 MB: Only the `raw` or `object` custom media types are supported. Both will work as normal, except that when using the `object` media type, the `content` field will be an empty
     * string and the `encoding` field will be `"none"`. To get the contents of these larger files, use the `raw` media type.
     *   - Greater than 100 MB: This endpoint is not supported.
     *
     * FROM: <https://docs.github.com/rest/repos/contents#get-repository-content>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `path: &str` -- path parameter.
     * * `ref_: &str` -- The name of the commit/branch/tag. Default: the repository’s default branch.
     */
    pub async fn get_content_symlink(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        ref_: &str,
    ) -> ClientResult<crate::Response<crate::types::SymlinkContent>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/contents/{}?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&path.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Get repository content.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/contents/{path}` endpoint.
     *
     * Gets the contents of a file or directory in a repository. Specify the file path or directory with the `path` parameter. If you omit the `path` parameter, you will receive the contents of the repository's root directory.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw file contents for files and symlinks.
     * - **`application/vnd.github.html+json`**: Returns the file contents in HTML. Markup languages are rendered to HTML using GitHub's open-source [Markup library](https://github.com/github/markup).
     * - **`application/vnd.github.object+json`**: Returns the contents in a consistent object format regardless of the content type. For example, instead of an array of objects for a directory, the response will be an object with an `entries` attribute containing the array of objects.
     *
     * If the content is a directory, the response will be an array of objects, one object for each item in the directory. When listing the contents of a directory, submodules have their "type" specified as "file". Logically, the value _should_ be "submodule". This behavior exists [for backwards compatibility purposes](https://git.io/v1YCW). In the next major version of the API, the type will be returned as "submodule".
     *
     * If the content is a symlink and the symlink's target is a normal file in the repository, then the API responds with the content of the file. Otherwise, the API responds with an object describing the symlink itself.
     *
     * If the content is a submodule, the `submodule_git_url` field identifies the location of the submodule repository, and the `sha` identifies a specific commit within the submodule repository. Git uses the given URL when cloning the submodule repository, and checks out the submodule at that specific commit. If the submodule repository is not hosted on github.com, the Git URLs (`git_url` and `_links["git"]`) and the github.com URLs (`html_url` and `_links["html"]`) will have null values.
     *
     * **Notes**:
     *
     * - To get a repository's contents recursively, you can [recursively get the tree](https://docs.github.com/rest/git/trees#get-a-tree).
     * - This API has an upper limit of 1,000 files for a directory. If you need to retrieve
     * more files, use the [Git Trees API](https://docs.github.com/rest/git/trees#get-a-tree).
     * - Download URLs expire and are meant to be used just once. To ensure the download URL does not expire, please use the contents API to obtain a fresh download URL for each download.
     * - If the requested file's size is:
     *   - 1 MB or smaller: All features of this endpoint are supported.
     *   - Between 1-100 MB: Only the `raw` or `object` custom media types are supported. Both will work as normal, except that when using the `object` media type, the `content` field will be an empty
     * string and the `encoding` field will be `"none"`. To get the contents of these larger files, use the `raw` media type.
     *   - Greater than 100 MB: This endpoint is not supported.
     *
     * FROM: <https://docs.github.com/rest/repos/contents#get-repository-content>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `path: &str` -- path parameter.
     * * `ref_: &str` -- The name of the commit/branch/tag. Default: the repository’s default branch.
     */
    pub async fn get_content_submodule(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        ref_: &str,
    ) -> ClientResult<crate::Response<crate::types::SubmoduleContent>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/contents/{}?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&path.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Get repository content.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/contents/{path}` endpoint.
     *
     * Gets the contents of a file or directory in a repository. Specify the file path or directory with the `path` parameter. If you omit the `path` parameter, you will receive the contents of the repository's root directory.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw file contents for files and symlinks.
     * - **`application/vnd.github.html+json`**: Returns the file contents in HTML. Markup languages are rendered to HTML using GitHub's open-source [Markup library](https://github.com/github/markup).
     * - **`application/vnd.github.object+json`**: Returns the contents in a consistent object format regardless of the content type. For example, instead of an array of objects for a directory, the response will be an object with an `entries` attribute containing the array of objects.
     *
     * If the content is a directory, the response will be an array of objects, one object for each item in the directory. When listing the contents of a directory, submodules have their "type" specified as "file". Logically, the value _should_ be "submodule". This behavior exists [for backwards compatibility purposes](https://git.io/v1YCW). In the next major version of the API, the type will be returned as "submodule".
     *
     * If the content is a symlink and the symlink's target is a normal file in the repository, then the API responds with the content of the file. Otherwise, the API responds with an object describing the symlink itself.
     *
     * If the content is a submodule, the `submodule_git_url` field identifies the location of the submodule repository, and the `sha` identifies a specific commit within the submodule repository. Git uses the given URL when cloning the submodule repository, and checks out the submodule at that specific commit. If the submodule repository is not hosted on github.com, the Git URLs (`git_url` and `_links["git"]`) and the github.com URLs (`html_url` and `_links["html"]`) will have null values.
     *
     * **Notes**:
     *
     * - To get a repository's contents recursively, you can [recursively get the tree](https://docs.github.com/rest/git/trees#get-a-tree).
     * - This API has an upper limit of 1,000 files for a directory. If you need to retrieve
     * more files, use the [Git Trees API](https://docs.github.com/rest/git/trees#get-a-tree).
     * - Download URLs expire and are meant to be used just once. To ensure the download URL does not expire, please use the contents API to obtain a fresh download URL for each download.
     * - If the requested file's size is:
     *   - 1 MB or smaller: All features of this endpoint are supported.
     *   - Between 1-100 MB: Only the `raw` or `object` custom media types are supported. Both will work as normal, except that when using the `object` media type, the `content` field will be an empty
     * string and the `encoding` field will be `"none"`. To get the contents of these larger files, use the `raw` media type.
     *   - Greater than 100 MB: This endpoint is not supported.
     *
     * FROM: <https://docs.github.com/rest/repos/contents#get-repository-content>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `path: &str` -- path parameter.
     * * `ref_: &str` -- The name of the commit/branch/tag. Default: the repository’s default branch.
     */
    pub async fn get_content(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        ref_: &str,
    ) -> ClientResult<crate::Response<crate::types::ReposGetContentResponseOneOf>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/contents/{}?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&path.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Create or update file contents.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/contents/{path}` endpoint.
     *
     * Creates a new file or replaces an existing file in a repository.
     *
     * > [!NOTE]
     * > If you use this endpoint and the "[Delete a file](https://docs.github.com/rest/repos/contents/#delete-a-file)" endpoint in parallel, the concurrent requests will conflict and you will receive errors. You must use these endpoints serially instead.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint. The `workflow` scope is also required in order to modify files in the `.github/workflows` directory.
     *
     * FROM: <https://docs.github.com/rest/repos/contents#create-or-update-file-contents>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `path: &str` -- path parameter.
     */
    pub async fn create_or_update_file_contents(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        body: &crate::types::ReposCreateUpdateFileContentsRequest,
    ) -> ClientResult<crate::Response<crate::types::FileCommitData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/contents/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&path.to_string()),
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
     * Delete a file.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/contents/{path}` endpoint.
     *
     * Deletes a file in a repository.
     *
     * You can provide an additional `committer` parameter, which is an object containing information about the committer. Or, you can provide an `author` parameter, which is an object containing information about the author.
     *
     * The `author` section is optional and is filled in with the `committer` information if omitted. If the `committer` information is omitted, the authenticated user's information is used.
     *
     * You must provide values for both `name` and `email`, whether you choose to use `author` or `committer`. Otherwise, you'll receive a `422` status code.
     *
     * > [!NOTE]
     * > If you use this endpoint and the "[Create or update file contents](https://docs.github.com/rest/repos/contents/#create-or-update-file-contents)" endpoint in parallel, the concurrent requests will conflict and you will receive errors. You must use these endpoints serially instead.
     *
     * FROM: <https://docs.github.com/rest/repos/contents#delete-a-file>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `path: &str` -- path parameter.
     */
    pub async fn delete_file(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        body: &crate::types::ReposDeleteFileRequest,
    ) -> ClientResult<crate::Response<crate::types::FileCommitData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/contents/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&path.to_string()),
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
     * List repository contributors.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/contributors` endpoint.
     *
     * Lists contributors to the specified repository and sorts them by the number of commits per contributor in descending order. This endpoint may return information that is a few hours old because the GitHub REST API caches contributor data to improve performance.
     *
     * GitHub identifies contributors by author email address. This endpoint groups contribution counts by GitHub user, which includes all associated email addresses. To improve performance, only the first 500 author email addresses in the repository link to GitHub users. The rest will appear as anonymous contributors without associated GitHub user information.
     *
     * FROM: <https://docs.github.com/rest/repos/repos#list-repository-contributors>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `anon: &str` -- Set to `1` or `true` to include anonymous contributors in results.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_contributors(
        &self,
        owner: &str,
        repo: &str,
        anon: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Contributor>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !anon.is_empty() {
            query_args.push(("anon".to_string(), anon.to_string()));
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
                "/repos/{}/{}/contributors?{}",
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
     * List repository contributors.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/contributors` endpoint.
     *
     * As opposed to `list_contributors`, this function returns all the pages of the request at once.
     *
     * Lists contributors to the specified repository and sorts them by the number of commits per contributor in descending order. This endpoint may return information that is a few hours old because the GitHub REST API caches contributor data to improve performance.
     *
     * GitHub identifies contributors by author email address. This endpoint groups contribution counts by GitHub user, which includes all associated email addresses. To improve performance, only the first 500 author email addresses in the repository link to GitHub users. The rest will appear as anonymous contributors without associated GitHub user information.
     *
     * FROM: <https://docs.github.com/rest/repos/repos#list-repository-contributors>
     */
    pub async fn list_all_contributors(
        &self,
        owner: &str,
        repo: &str,
        anon: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Contributor>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !anon.is_empty() {
            query_args.push(("anon".to_string(), anon.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/contributors?{}",
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
     * List deployments.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/deployments` endpoint.
     *
     * Simple filtering of deployments is available via query parameters:
     *
     * FROM: <https://docs.github.com/rest/deployments/deployments#list-deployments>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `sha: &str` -- The SHA recorded at creation time.
     * * `ref_: &str` -- The name of the ref. This can be a branch, tag, or SHA.
     * * `task: &str` -- The name of the task for the deployment (e.g., `deploy` or `deploy:migrations`).
     * * `environment: &str` -- The name of the environment that was deployed to (e.g., `staging` or `production`).
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_deployments(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        ref_: &str,
        task: &str,
        environment: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Deployment>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !environment.is_empty() {
            query_args.push(("environment".to_string(), environment.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        if !sha.is_empty() {
            query_args.push(("sha".to_string(), sha.to_string()));
        }
        if !task.is_empty() {
            query_args.push(("task".to_string(), task.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/deployments?{}",
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
     * List deployments.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/deployments` endpoint.
     *
     * As opposed to `list_deployments`, this function returns all the pages of the request at once.
     *
     * Simple filtering of deployments is available via query parameters:
     *
     * FROM: <https://docs.github.com/rest/deployments/deployments#list-deployments>
     */
    pub async fn list_all_deployments(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        ref_: &str,
        task: &str,
        environment: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Deployment>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !environment.is_empty() {
            query_args.push(("environment".to_string(), environment.to_string()));
        }
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        if !sha.is_empty() {
            query_args.push(("sha".to_string(), sha.to_string()));
        }
        if !task.is_empty() {
            query_args.push(("task".to_string(), task.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/deployments?{}",
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
     * Create a deployment.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/deployments` endpoint.
     *
     * Deployments offer a few configurable parameters with certain defaults.
     *
     * The `ref` parameter can be any named branch, tag, or SHA. At GitHub we often deploy branches and verify them
     * before we merge a pull request.
     *
     * The `environment` parameter allows deployments to be issued to different runtime environments. Teams often have
     * multiple environments for verifying their applications, such as `production`, `staging`, and `qa`. This parameter
     * makes it easier to track which environments have requested deployments. The default environment is `production`.
     *
     * The `auto_merge` parameter is used to ensure that the requested ref is not behind the repository's default branch. If
     * the ref _is_ behind the default branch for the repository, we will attempt to merge it for you. If the merge succeeds,
     * the API will return a successful merge commit. If merge conflicts prevent the merge from succeeding, the API will
     * return a failure response.
     *
     * By default, [commit statuses](https://docs.github.com/rest/commits/statuses) for every submitted context must be in a `success`
     * state. The `required_contexts` parameter allows you to specify a subset of contexts that must be `success`, or to
     * specify contexts that have not yet been submitted. You are not required to use commit statuses to deploy. If you do
     * not require any contexts or create any commit statuses, the deployment will always succeed.
     *
     * The `payload` parameter is available for any extra information that a deployment system might need. It is a JSON text
     * field that will be passed on when a deployment event is dispatched.
     *
     * The `task` parameter is used by the deployment system to allow different execution paths. In the web world this might
     * be `deploy:migrations` to run schema changes on the system. In the compiled world this could be a flag to compile an
     * application with debugging enabled.
     *
     * Merged branch response:
     *
     * You will see this response when GitHub automatically merges the base branch into the topic branch instead of creating
     * a deployment. This auto-merge happens when:
     * *   Auto-merge option is enabled in the repository
     * *   Topic branch does not include the latest changes on the base branch, which is `master` in the response example
     * *   There are no merge conflicts
     *
     * If there are no new commits in the base branch, a new request to create a deployment should give a successful
     * response.
     *
     * Merge conflict response:
     *
     * This error happens when the `auto_merge` option is enabled and when the default branch (in this case `master`), can't
     * be merged into the branch that's being deployed (in this case `topic-branch`), due to merge conflicts.
     *
     * Failed commit status checks:
     *
     * This error happens when the `required_contexts` parameter indicates that one or more contexts need to have a `success`
     * status for the commit to be deployed, but one or more of the required contexts do not have a state of `success`.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` or `repo_deployment` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/deployments/deployments#create-a-deployment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_deployment(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposCreateDeploymentRequest,
    ) -> ClientResult<crate::Response<crate::types::Deployment>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/deployments",
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
     * Get a deployment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/deployments/{deployment_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/deployments/deployments#get-a-deployment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `deployment_id: i64` -- deployment_id parameter.
     */
    pub async fn get_deployment(
        &self,
        owner: &str,
        repo: &str,
        deployment_id: i64,
    ) -> ClientResult<crate::Response<crate::types::Deployment>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/deployments/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&deployment_id.to_string()),
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
     * Delete a deployment.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/deployments/{deployment_id}` endpoint.
     *
     * If the repository only has one deployment, you can delete the deployment regardless of its status. If the repository has more than one deployment, you can only delete inactive deployments. This ensures that repositories with multiple deployments will always have an active deployment.
     *
     * To set a deployment as inactive, you must:
     *
     * *   Create a new deployment that is active so that the system has a record of the current state, then delete the previously active deployment.
     * *   Mark the active deployment as inactive by adding any non-successful deployment status.
     *
     * For more information, see "[Create a deployment](https://docs.github.com/rest/deployments/deployments/#create-a-deployment)" and "[Create a deployment status](https://docs.github.com/rest/deployments/statuses#create-a-deployment-status)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` or `repo_deployment` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/deployments/deployments#delete-a-deployment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `deployment_id: i64` -- deployment_id parameter.
     */
    pub async fn delete_deployment(
        &self,
        owner: &str,
        repo: &str,
        deployment_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/deployments/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&deployment_id.to_string()),
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
     * List deployment statuses.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/deployments/{deployment_id}/statuses` endpoint.
     *
     * Users with pull access can view deployment statuses for a deployment:
     *
     * FROM: <https://docs.github.com/rest/deployments/statuses#list-deployment-statuses>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `deployment_id: i64` -- deployment_id parameter.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_deployment_statuses(
        &self,
        owner: &str,
        repo: &str,
        deployment_id: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::DeploymentStatus>>> {
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
                "/repos/{}/{}/deployments/{}/statuses?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&deployment_id.to_string()),
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
     * List deployment statuses.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/deployments/{deployment_id}/statuses` endpoint.
     *
     * As opposed to `list_deployment_statuses`, this function returns all the pages of the request at once.
     *
     * Users with pull access can view deployment statuses for a deployment:
     *
     * FROM: <https://docs.github.com/rest/deployments/statuses#list-deployment-statuses>
     */
    pub async fn list_all_deployment_statuses(
        &self,
        owner: &str,
        repo: &str,
        deployment_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::DeploymentStatus>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/deployments/{}/statuses",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&deployment_id.to_string()),
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
     * Create a deployment status.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/deployments/{deployment_id}/statuses` endpoint.
     *
     * Users with `push` access can create deployment statuses for a given deployment.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo_deployment` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/deployments/statuses#create-a-deployment-status>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `deployment_id: i64` -- deployment_id parameter.
     */
    pub async fn create_deployment_status(
        &self,
        owner: &str,
        repo: &str,
        deployment_id: i64,
        body: &crate::types::ReposCreateDeploymentStatusRequest,
    ) -> ClientResult<crate::Response<crate::types::DeploymentStatus>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/deployments/{}/statuses",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&deployment_id.to_string()),
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
     * Get a deployment status.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/deployments/{deployment_id}/statuses/{status_id}` endpoint.
     *
     * Users with pull access can view a deployment status for a deployment:
     *
     * FROM: <https://docs.github.com/rest/deployments/statuses#get-a-deployment-status>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `deployment_id: i64` -- deployment_id parameter.
     * * `status_id: i64`
     */
    pub async fn get_deployment_status(
        &self,
        owner: &str,
        repo: &str,
        deployment_id: i64,
        status_id: i64,
    ) -> ClientResult<crate::Response<crate::types::DeploymentStatus>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/deployments/{}/statuses/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&deployment_id.to_string()),
                crate::progenitor_support::encode_path(&status_id.to_string()),
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
     * Create a repository dispatch event.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/dispatches` endpoint.
     *
     * You can use this endpoint to trigger a webhook event called `repository_dispatch` when you want activity that happens outside of GitHub to trigger a GitHub Actions workflow or GitHub App webhook. You must configure your GitHub Actions workflow or GitHub App to run when the `repository_dispatch` event occurs. For an example `repository_dispatch` webhook payload, see "[RepositoryDispatchEvent](https://docs.github.com/webhooks/event-payloads/#repository_dispatch)."
     *
     * The `client_payload` parameter is available for any extra information that your workflow might need. This parameter is a JSON payload that will be passed on when the webhook event is dispatched. For example, the `client_payload` can include a message that a user would like to send using a GitHub Actions workflow. Or the `client_payload` can be used as a test to debug your workflow.
     *
     * This input example shows how you can use the `client_payload` as a test to debug your workflow.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/repos/repos#create-a-repository-dispatch-event>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_dispatch_event(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposCreateDispatchEventRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/dispatches",
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
     * List environments.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/environments` endpoint.
     *
     * Lists the environments for a repository.
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
     *
     * FROM: <https://docs.github.com/rest/deployments/environments#list-environments>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn get_all_environments(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ReposGetAllEnvironmentsResponse>> {
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
                "/repos/{}/{}/environments?{}",
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
     * Get an environment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/environments/{environment_name}` endpoint.
     *
     * > [!NOTE]
     * > To get information about name patterns that branches must match in order to deploy to this environment, see "[Get a deployment branch policy](/rest/deployments/branch-policies#get-a-deployment-branch-policy)."
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
     *
     * FROM: <https://docs.github.com/rest/deployments/environments#get-an-environment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     */
    pub async fn get_environment(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
    ) -> ClientResult<crate::Response<crate::types::EnvironmentData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&environment_name.to_string()),
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
     * Create or update an environment.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/environments/{environment_name}` endpoint.
     *
     * Create or update an environment with protection rules, such as required reviewers. For more information about environment protection rules, see "[Environments](/actions/reference/environments#environment-protection-rules)."
     *
     * > [!NOTE]
     * > To create or update name patterns that branches must match in order to deploy to this environment, see "[Deployment branch policies](/rest/deployments/branch-policies)."
     *
     * > [!NOTE]
     * > To create or update secrets for an environment, see "[GitHub Actions secrets](/rest/actions/secrets)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/deployments/environments#create-or-update-an-environment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     */
    pub async fn create_or_update_environment(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
        body: &crate::types::ReposCreateUpdateEnvironmentRequest,
    ) -> ClientResult<crate::Response<crate::types::EnvironmentData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&environment_name.to_string()),
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
     * Delete an environment.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/environments/{environment_name}` endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/deployments/environments#delete-an-environment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     */
    pub async fn delete_an_environment(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&environment_name.to_string()),
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
     * List deployment branch policies.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/environments/{environment_name}/deployment-branch-policies` endpoint.
     *
     * Lists the deployment branch policies for an environment.
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
     *
     * FROM: <https://docs.github.com/rest/deployments/branch-policies#list-deployment-branch-policies>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_deployment_branch_policies(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ReposListDeploymentBranchPoliciesResponse>>
    {
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
                "/repos/{}/{}/environments/{}/deployment-branch-policies?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&environment_name.to_string()),
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
     * Create a deployment branch policy.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/environments/{environment_name}/deployment-branch-policies` endpoint.
     *
     * Creates a deployment branch or tag policy for an environment.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/deployments/branch-policies#create-a-deployment-branch-policy>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     */
    pub async fn create_deployment_branch_policy(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
        body: &crate::types::DeploymentBranchTagPolicyNamePattern,
    ) -> ClientResult<crate::Response<crate::types::DeploymentBranchPolicy>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}/deployment-branch-policies",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&environment_name.to_string()),
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
     * Get a deployment branch policy.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/environments/{environment_name}/deployment-branch-policies/{branch_policy_id}` endpoint.
     *
     * Gets a deployment branch or tag policy for an environment.
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
     *
     * FROM: <https://docs.github.com/rest/deployments/branch-policies#get-a-deployment-branch-policy>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     * * `branch_policy_id: i64` -- The unique identifier of the branch policy.
     */
    pub async fn get_deployment_branch_policy(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
        branch_policy_id: i64,
    ) -> ClientResult<crate::Response<crate::types::DeploymentBranchPolicy>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}/deployment-branch-policies/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&environment_name.to_string()),
                crate::progenitor_support::encode_path(&branch_policy_id.to_string()),
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
     * Update a deployment branch policy.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/environments/{environment_name}/deployment-branch-policies/{branch_policy_id}` endpoint.
     *
     * Updates a deployment branch or tag policy for an environment.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/deployments/branch-policies#update-a-deployment-branch-policy>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     * * `branch_policy_id: i64` -- The unique identifier of the branch policy.
     */
    pub async fn update_deployment_branch_policy(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
        branch_policy_id: i64,
        body: &crate::types::DeploymentBranchPolicyNamePattern,
    ) -> ClientResult<crate::Response<crate::types::DeploymentBranchPolicy>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}/deployment-branch-policies/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&environment_name.to_string()),
                crate::progenitor_support::encode_path(&branch_policy_id.to_string()),
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
     * Delete a deployment branch policy.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/environments/{environment_name}/deployment-branch-policies/{branch_policy_id}` endpoint.
     *
     * Deletes a deployment branch or tag policy for an environment.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/deployments/branch-policies#delete-a-deployment-branch-policy>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     * * `branch_policy_id: i64` -- The unique identifier of the branch policy.
     */
    pub async fn delete_deployment_branch_policy(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
        branch_policy_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}/deployment-branch-policies/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&environment_name.to_string()),
                crate::progenitor_support::encode_path(&branch_policy_id.to_string()),
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
     * Get all deployment protection rules for an environment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/environments/{environment_name}/deployment_protection_rules` endpoint.
     *
     * Gets all custom deployment protection rules that are enabled for an environment. Anyone with read access to the repository can use this endpoint. For more information about environments, see "[Using environments for deployment](https://docs.github.com/actions/deployment/targeting-different-environments/using-environments-for-deployment)."
     *
     * For more information about the app that is providing this custom deployment rule, see the [documentation for the `GET /apps/{app_slug}` endpoint](https://docs.github.com/rest/apps/apps#get-an-app).
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
     *
     * FROM: <https://docs.github.com/rest/deployments/protection-rules#get-all-deployment-protection-rules-for-an-environment>
     *
     * **Parameters:**
     *
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     */
    pub async fn get_all_deployment_protection_rules(
        &self,
        environment_name: &str,
        repo: &str,
        owner: &str,
    ) -> ClientResult<crate::Response<crate::types::ReposGetAllDeploymentProtectionRulesResponse>>
    {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}/deployment_protection_rules",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&environment_name.to_string()),
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
     * Create a custom deployment protection rule on an environment.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/environments/{environment_name}/deployment_protection_rules` endpoint.
     *
     * Enable a custom deployment protection rule for an environment.
     *
     * The authenticated user must have admin or owner permissions to the repository to use this endpoint.
     *
     * For more information about the app that is providing this custom deployment rule, see the [documentation for the `GET /apps/{app_slug}` endpoint](https://docs.github.com/rest/apps/apps#get-an-app), as well as the [guide to creating custom deployment protection rules](https://docs.github.com/actions/managing-workflow-runs-and-deployments/managing-deployments/creating-custom-deployment-protection-rules).
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/deployments/protection-rules#create-a-custom-deployment-protection-rule-on-an-environment>
     *
     * **Parameters:**
     *
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     */
    pub async fn create_deployment_protection_rule(
        &self,
        environment_name: &str,
        repo: &str,
        owner: &str,
        body: &crate::types::ReposCreateDeploymentProtectionRuleRequest,
    ) -> ClientResult<crate::Response<crate::types::DeploymentProtectionRule>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}/deployment_protection_rules",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&environment_name.to_string()),
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
     * List custom deployment rule integrations available for an environment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/environments/{environment_name}/deployment_protection_rules/apps` endpoint.
     *
     * Gets all custom deployment protection rule integrations that are available for an environment.
     *
     * The authenticated user must have admin or owner permissions to the repository to use this endpoint.
     *
     * For more information about environments, see "[Using environments for deployment](https://docs.github.com/actions/deployment/targeting-different-environments/using-environments-for-deployment)."
     *
     * For more information about the app that is providing this custom deployment rule, see "[GET an app](https://docs.github.com/rest/apps/apps#get-an-app)".
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
     *
     * FROM: <https://docs.github.com/rest/deployments/protection-rules#list-custom-deployment-rule-integrations-available-for-an-environment>
     *
     * **Parameters:**
     *
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_custom_deployment_rule_integrations(
        &self,
        environment_name: &str,
        repo: &str,
        owner: &str,
        page: i64,
        per_page: i64,
    ) -> ClientResult<
        crate::Response<crate::types::ReposListCustomDeploymentRuleIntegrationsResponse>,
    > {
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
                "/repos/{}/{}/environments/{}/deployment_protection_rules/apps?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&environment_name.to_string()),
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
     * Get a custom deployment protection rule.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/environments/{environment_name}/deployment_protection_rules/{protection_rule_id}` endpoint.
     *
     * Gets an enabled custom deployment protection rule for an environment. Anyone with read access to the repository can use this endpoint. For more information about environments, see "[Using environments for deployment](https://docs.github.com/actions/deployment/targeting-different-environments/using-environments-for-deployment)."
     *
     * For more information about the app that is providing this custom deployment rule, see [`GET /apps/{app_slug}`](https://docs.github.com/rest/apps/apps#get-an-app).
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
     *
     * FROM: <https://docs.github.com/rest/deployments/protection-rules#get-a-custom-deployment-protection-rule>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     * * `protection_rule_id: i64` -- The unique identifier of the protection rule.
     */
    pub async fn get_custom_deployment_protection_rule(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
        protection_rule_id: i64,
    ) -> ClientResult<crate::Response<crate::types::DeploymentProtectionRule>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}/deployment_protection_rules/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&environment_name.to_string()),
                crate::progenitor_support::encode_path(&protection_rule_id.to_string()),
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
     * Disable a custom protection rule for an environment.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/environments/{environment_name}/deployment_protection_rules/{protection_rule_id}` endpoint.
     *
     * Disables a custom deployment protection rule for an environment.
     *
     * The authenticated user must have admin or owner permissions to the repository to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/deployments/protection-rules#disable-a-custom-protection-rule-for-an-environment>
     *
     * **Parameters:**
     *
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `protection_rule_id: i64` -- The unique identifier of the protection rule.
     */
    pub async fn disable_deployment_protection_rule(
        &self,
        environment_name: &str,
        repo: &str,
        owner: &str,
        protection_rule_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}/deployment_protection_rules/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&environment_name.to_string()),
                crate::progenitor_support::encode_path(&protection_rule_id.to_string()),
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
     * List forks.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/forks` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/repos/forks#list-forks>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `sort: crate::types::ReposListForksSort` -- The sort order. `stargazers` will sort by star count.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_forks(
        &self,
        owner: &str,
        repo: &str,
        sort: crate::types::ReposListForksSort,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::MinimalRepository>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/forks?{}",
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
     * List forks.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/forks` endpoint.
     *
     * As opposed to `list_forks`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/repos/forks#list-forks>
     */
    pub async fn list_all_forks(
        &self,
        owner: &str,
        repo: &str,
        sort: crate::types::ReposListForksSort,
    ) -> ClientResult<crate::Response<Vec<crate::types::MinimalRepository>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/forks?{}",
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
     * Create a fork.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/forks` endpoint.
     *
     * Create a fork for the authenticated user.
     *
     * > [!NOTE]
     * > Forking a Repository happens asynchronously. You may have to wait a short period of time before you can access the git objects. If this takes longer than 5 minutes, be sure to contact [GitHub Support](https://support.github.com/contact?tags=dotcom-rest-api).
     *
     * > [!NOTE]
     * > Although this endpoint works with GitHub Apps, the GitHub App must be installed on the destination account with access to all repositories and on the source account with access to the source repository.
     *
     * FROM: <https://docs.github.com/rest/repos/forks#create-a-fork>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_fork(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposCreateForkRequest,
    ) -> ClientResult<crate::Response<crate::types::FullRepository>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/forks",
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
     * List repository webhooks.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/hooks` endpoint.
     *
     * Lists webhooks for a repository. `last response` may return null if there have not been any deliveries within 30 days.
     *
     * FROM: <https://docs.github.com/rest/repos/webhooks#list-repository-webhooks>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_webhooks(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Hook>>> {
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
                "/repos/{}/{}/hooks?{}",
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
     * List repository webhooks.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/hooks` endpoint.
     *
     * As opposed to `list_webhooks`, this function returns all the pages of the request at once.
     *
     * Lists webhooks for a repository. `last response` may return null if there have not been any deliveries within 30 days.
     *
     * FROM: <https://docs.github.com/rest/repos/webhooks#list-repository-webhooks>
     */
    pub async fn list_all_webhooks(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Hook>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks",
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
     * Create a repository webhook.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/hooks` endpoint.
     *
     * Repositories can have multiple webhooks installed. Each webhook should have a unique `config`. Multiple webhooks can
     * share the same `config` as long as those webhooks do not have any `events` that overlap.
     *
     * FROM: <https://docs.github.com/rest/repos/webhooks#create-a-repository-webhook>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_webhook(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposCreateWebhookRequest,
    ) -> ClientResult<crate::Response<crate::types::Hook>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks",
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
     * Get a repository webhook.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/hooks/{hook_id}` endpoint.
     *
     * Returns a webhook configured in a repository. To get only the webhook `config` properties, see "[Get a webhook configuration for a repository](/rest/webhooks/repo-config#get-a-webhook-configuration-for-a-repository)."
     *
     * FROM: <https://docs.github.com/rest/repos/webhooks#get-a-repository-webhook>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `hook_id: i64` -- The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery.
     */
    pub async fn get_webhook(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
    ) -> ClientResult<crate::Response<crate::types::Hook>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
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
     * Delete a repository webhook.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/hooks/{hook_id}` endpoint.
     *
     * Delete a webhook for an organization.
     *
     * The authenticated user must be a repository owner, or have admin access in the repository, to delete the webhook.
     *
     * FROM: <https://docs.github.com/rest/repos/webhooks#delete-a-repository-webhook>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `hook_id: i64` -- The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery.
     */
    pub async fn delete_webhook(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
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
     * Update a repository webhook.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/hooks/{hook_id}` endpoint.
     *
     * Updates a webhook configured in a repository. If you previously had a `secret` set, you must provide the same `secret` or set a new `secret` or the secret will be removed. If you are only updating individual webhook `config` properties, use "[Update a webhook configuration for a repository](/rest/webhooks/repo-config#update-a-webhook-configuration-for-a-repository)."
     *
     * FROM: <https://docs.github.com/rest/repos/webhooks#update-a-repository-webhook>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `hook_id: i64` -- The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery.
     */
    pub async fn update_webhook(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
        body: &crate::types::ReposUpdateWebhookRequest,
    ) -> ClientResult<crate::Response<crate::types::Hook>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
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
     * Get a webhook configuration for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/hooks/{hook_id}/config` endpoint.
     *
     * Returns the webhook configuration for a repository. To get more information about the webhook, including the `active` state and `events`, use "[Get a repository webhook](/rest/webhooks/repos#get-a-repository-webhook)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:repo_hook` or `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/repos/webhooks#get-a-webhook-configuration-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `hook_id: i64` -- The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery.
     */
    pub async fn get_webhook_config_for_repo(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
    ) -> ClientResult<crate::Response<crate::types::WebhookConfig>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks/{}/config",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
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
     * Update a webhook configuration for a repository.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/hooks/{hook_id}/config` endpoint.
     *
     * Updates the webhook configuration for a repository. To update more information about the webhook, including the `active` state and `events`, use "[Update a repository webhook](/rest/webhooks/repos#update-a-repository-webhook)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `write:repo_hook` or `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/repos/webhooks#update-a-webhook-configuration-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `hook_id: i64` -- The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery.
     */
    pub async fn update_webhook_config_for_repo(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
        body: &crate::types::AppsUpdateWebhookConfigAppRequest,
    ) -> ClientResult<crate::Response<crate::types::WebhookConfig>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks/{}/config",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
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
     * List deliveries for a repository webhook.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/hooks/{hook_id}/deliveries` endpoint.
     *
     * Returns a list of webhook deliveries for a webhook configured in a repository.
     *
     * FROM: <https://docs.github.com/rest/repos/webhooks#list-deliveries-for-a-repository-webhook>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `hook_id: i64` -- The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `cursor: &str` -- Used for pagination: the starting delivery from which the page of deliveries is fetched. Refer to the `link` header for the next and previous page cursors.
     */
    pub async fn list_webhook_deliveries(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
        per_page: i64,
        cursor: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::HookDeliveryItem>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks/{}/deliveries?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
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
     * List deliveries for a repository webhook.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/hooks/{hook_id}/deliveries` endpoint.
     *
     * As opposed to `list_webhook_deliveries`, this function returns all the pages of the request at once.
     *
     * Returns a list of webhook deliveries for a webhook configured in a repository.
     *
     * FROM: <https://docs.github.com/rest/repos/webhooks#list-deliveries-for-a-repository-webhook>
     */
    pub async fn list_all_webhook_deliveries(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
        cursor: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::HookDeliveryItem>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks/{}/deliveries?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
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
     * Get a delivery for a repository webhook.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/hooks/{hook_id}/deliveries/{delivery_id}` endpoint.
     *
     * Returns a delivery for a webhook configured in a repository.
     *
     * FROM: <https://docs.github.com/rest/repos/webhooks#get-a-delivery-for-a-repository-webhook>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `hook_id: i64` -- The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery.
     * * `delivery_id: i64`
     */
    pub async fn get_webhook_delivery(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
        delivery_id: i64,
    ) -> ClientResult<crate::Response<crate::types::HookDelivery>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks/{}/deliveries/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
                crate::progenitor_support::encode_path(&delivery_id.to_string()),
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
     * Redeliver a delivery for a repository webhook.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/hooks/{hook_id}/deliveries/{delivery_id}/attempts` endpoint.
     *
     * Redeliver a webhook delivery for a webhook configured in a repository.
     *
     * FROM: <https://docs.github.com/rest/repos/webhooks#redeliver-a-delivery-for-a-repository-webhook>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `hook_id: i64` -- The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery.
     * * `delivery_id: i64`
     */
    pub async fn redeliver_webhook_delivery(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
        delivery_id: i64,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks/{}/deliveries/{}/attempts",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
                crate::progenitor_support::encode_path(&delivery_id.to_string()),
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
     * Ping a repository webhook.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/hooks/{hook_id}/pings` endpoint.
     *
     * This will trigger a [ping event](https://docs.github.com/webhooks/#ping-event) to be sent to the hook.
     *
     * FROM: <https://docs.github.com/rest/repos/webhooks#ping-a-repository-webhook>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `hook_id: i64` -- The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery.
     */
    pub async fn ping_webhook(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks/{}/pings",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
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
     * Test the push repository webhook.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/hooks/{hook_id}/tests` endpoint.
     *
     * This will trigger the hook with the latest push to the current repository if the hook is subscribed to `push` events. If the hook is not subscribed to `push` events, the server will respond with 204 but no test POST will be generated.
     *
     * > [!NOTE]
     * > Previously `/repos/:owner/:repo/hooks/:hook_id/test`
     *
     * FROM: <https://docs.github.com/rest/repos/webhooks#test-the-push-repository-webhook>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `hook_id: i64` -- The unique identifier of the hook. You can find this value in the `X-GitHub-Hook-ID` header of a webhook delivery.
     */
    pub async fn test_push_webhook(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks/{}/tests",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&hook_id.to_string()),
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
     * Check if immutable releases are enabled for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/immutable-releases` endpoint.
     *
     * Shows whether immutable releases are enabled or disabled. Also identifies whether immutability is being
     * enforced by the repository owner.  The authenticated user must have admin read access to the repository.
     *
     * FROM: <https://docs.github.com/rest/repos/repos#check-if-immutable-releases-are-enabled-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn check_immutable_releases(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::CheckImmutableReleases>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/immutable-releases",
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
     * Enable immutable releases.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/immutable-releases` endpoint.
     *
     * Enables immutable releases for a repository. The authenticated user must have admin access to the repository.
     *
     * FROM: <https://docs.github.com/rest/repos/repos#enable-immutable-releases>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn enable_immutable_releases(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<String>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/immutable-releases",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Disable immutable releases.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/immutable-releases` endpoint.
     *
     * Disables immutable releases for a repository. The authenticated user must have admin access to the repository.
     *
     * FROM: <https://docs.github.com/rest/repos/repos#disable-immutable-releases>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn disable_immutable_releases(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<String>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/immutable-releases",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * List repository invitations.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/invitations` endpoint.
     *
     * When authenticating as a user with admin rights to a repository, this endpoint will list all currently open repository invitations.
     *
     * FROM: <https://docs.github.com/rest/collaborators/invitations#list-repository-invitations>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_invitations(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::RepositoryInvitation>>> {
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
                "/repos/{}/{}/invitations?{}",
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
     * List repository invitations.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/invitations` endpoint.
     *
     * As opposed to `list_invitations`, this function returns all the pages of the request at once.
     *
     * When authenticating as a user with admin rights to a repository, this endpoint will list all currently open repository invitations.
     *
     * FROM: <https://docs.github.com/rest/collaborators/invitations#list-repository-invitations>
     */
    pub async fn list_all_invitations(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::RepositoryInvitation>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/invitations",
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
     * Delete a repository invitation.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/invitations/{invitation_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/collaborators/invitations#delete-a-repository-invitation>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `invitation_id: i64` -- The unique identifier of the invitation.
     */
    pub async fn delete_invitation(
        &self,
        owner: &str,
        repo: &str,
        invitation_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/invitations/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&invitation_id.to_string()),
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
     * Update a repository invitation.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/invitations/{invitation_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/collaborators/invitations#update-a-repository-invitation>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `invitation_id: i64` -- The unique identifier of the invitation.
     */
    pub async fn update_invitation(
        &self,
        owner: &str,
        repo: &str,
        invitation_id: i64,
        body: &crate::types::ReposUpdateInvitationRequest,
    ) -> ClientResult<crate::Response<crate::types::RepositoryInvitation>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/invitations/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&invitation_id.to_string()),
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
     * List deploy keys.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/keys` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/deploy-keys/deploy-keys#list-deploy-keys>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_deploy_keys(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::DeployKey>>> {
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
                "/repos/{}/{}/keys?{}",
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
     * List deploy keys.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/keys` endpoint.
     *
     * As opposed to `list_deploy_keys`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/deploy-keys/deploy-keys#list-deploy-keys>
     */
    pub async fn list_all_deploy_keys(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::DeployKey>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/keys",
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
     * Create a deploy key.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/keys` endpoint.
     *
     * You can create a read-only deploy key.
     *
     * FROM: <https://docs.github.com/rest/deploy-keys/deploy-keys#create-a-deploy-key>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_deploy_key(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposCreateDeployKeyRequest,
    ) -> ClientResult<crate::Response<crate::types::DeployKey>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/keys",
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
     * Get a deploy key.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/keys/{key_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/deploy-keys/deploy-keys#get-a-deploy-key>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `key_id: i64` -- The unique identifier of the key.
     */
    pub async fn get_deploy_key(
        &self,
        owner: &str,
        repo: &str,
        key_id: i64,
    ) -> ClientResult<crate::Response<crate::types::DeployKey>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/keys/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&key_id.to_string()),
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
     * Delete a deploy key.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/keys/{key_id}` endpoint.
     *
     * Deploy keys are immutable. If you need to update a key, remove the key and create a new one instead.
     *
     * FROM: <https://docs.github.com/rest/deploy-keys/deploy-keys#delete-a-deploy-key>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `key_id: i64` -- The unique identifier of the key.
     */
    pub async fn delete_deploy_key(
        &self,
        owner: &str,
        repo: &str,
        key_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/keys/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&key_id.to_string()),
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
     * List repository languages.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/languages` endpoint.
     *
     * Lists languages for the specified repository. The value shown for each language is the number of bytes of code written in that language.
     *
     * FROM: <https://docs.github.com/rest/repos/repos#list-repository-languages>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn list_languages(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<i64>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/languages",
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
     * Sync a fork branch with the upstream repository.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/merge-upstream` endpoint.
     *
     * Sync a branch of a forked repository to keep it up-to-date with the upstream repository.
     *
     * FROM: <https://docs.github.com/rest/branches/branches#sync-a-fork-branch-with-the-upstream-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn merge_upstream(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposMergeUpstreamRequest,
    ) -> ClientResult<crate::Response<crate::types::MergedUpstream>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/merge-upstream",
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
     * Merge a branch.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/merges` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/branches/branches#merge-a-branch>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn merge(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposMergeRequest,
    ) -> ClientResult<crate::Response<crate::types::CommitDataType>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/merges",
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
     * Get a GitHub Pages site.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pages` endpoint.
     *
     * Gets information about a GitHub Pages site.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/pages/pages#get-a-apiname-pages-site>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_pages(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::Page>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pages",
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
     * Update information about a GitHub Pages site.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/pages` endpoint.
     *
     * Updates information for a GitHub Pages site. For more information, see "[About GitHub Pages](/github/working-with-github-pages/about-github-pages).
     *
     * The authenticated user must be a repository administrator, maintainer, or have the 'manage GitHub Pages settings' permission.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/pages/pages#update-information-about-a-apiname-pages-site>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn update_information_about_pages_site(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposUpdateInformationAboutPagesSiteRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pages",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Create a GitHub Pages site.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/pages` endpoint.
     *
     * Configures a GitHub Pages site. For more information, see "[About GitHub Pages](/github/working-with-github-pages/about-github-pages)."
     *
     * The authenticated user must be a repository administrator, maintainer, or have the 'manage GitHub Pages settings' permission.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/pages/pages#create-a-apiname-pages-site>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_pages_site(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposCreatePagesSiteRequest,
    ) -> ClientResult<crate::Response<crate::types::Page>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pages",
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
     * Delete a GitHub Pages site.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/pages` endpoint.
     *
     * Deletes a GitHub Pages site. For more information, see "[About GitHub Pages](/github/working-with-github-pages/about-github-pages).
     *
     * The authenticated user must be a repository administrator, maintainer, or have the 'manage GitHub Pages settings' permission.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/pages/pages#delete-a-apiname-pages-site>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn delete_pages_site(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pages",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * List GitHub Pages builds.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pages/builds` endpoint.
     *
     * Lists builts of a GitHub Pages site.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/pages/pages#list-apiname-pages-builds>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_pages_builds(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::PageBuild>>> {
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
                "/repos/{}/{}/pages/builds?{}",
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
     * List GitHub Pages builds.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pages/builds` endpoint.
     *
     * As opposed to `list_pages_builds`, this function returns all the pages of the request at once.
     *
     * Lists builts of a GitHub Pages site.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/pages/pages#list-apiname-pages-builds>
     */
    pub async fn list_all_pages_builds(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::PageBuild>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pages/builds",
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
     * Request a GitHub Pages build.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/pages/builds` endpoint.
     *
     * You can request that your site be built from the latest revision on the default branch. This has the same effect as pushing a commit to your default branch, but does not require an additional commit. Manually triggering page builds can be helpful when diagnosing build warnings and failures.
     *
     * Build requests are limited to one concurrent build per repository and one concurrent build per requester. If you request a build while another is still in progress, the second request will be queued until the first completes.
     *
     * FROM: <https://docs.github.com/rest/pages/pages#request-a-apiname-pages-build>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn request_pages_build(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::PageBuildStatus>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pages/builds",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Get latest Pages build.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pages/builds/latest` endpoint.
     *
     * Gets information about the single most recent build of a GitHub Pages site.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/pages/pages#get-latest-pages-build>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_latest_pages_build(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::PageBuild>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pages/builds/latest",
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
     * Get GitHub Pages build.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pages/builds/{build_id}` endpoint.
     *
     * Gets information about a GitHub Pages build.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/pages/pages#get-apiname-pages-build>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `build_id: i64`
     */
    pub async fn get_pages_build(
        &self,
        owner: &str,
        repo: &str,
        build_id: i64,
    ) -> ClientResult<crate::Response<crate::types::PageBuild>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pages/builds/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&build_id.to_string()),
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
     * Create a GitHub Pages deployment.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/pages/deployments` endpoint.
     *
     * Create a GitHub Pages deployment for a repository.
     *
     * The authenticated user must have write permission to the repository.
     *
     * FROM: <https://docs.github.com/rest/pages/pages#create-a-github-pages-deployment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_pages_deployment(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposCreatePagesDeploymentRequest,
    ) -> ClientResult<crate::Response<crate::types::GitHubPages>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pages/deployments",
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
     * Get the status of a GitHub Pages deployment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pages/deployments/{pages_deployment_id}` endpoint.
     *
     * Gets the current status of a GitHub Pages deployment.
     *
     * The authenticated user must have read permission for the GitHub Pages site.
     *
     * FROM: <https://docs.github.com/rest/pages/pages#get-the-status-of-a-github-pages-deployment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pages_deployment_id: &str` -- The ID of the Pages deployment. You can also give the commit SHA of the deployment.
     */
    pub async fn get_pages_deployment(
        &self,
        owner: &str,
        repo: &str,
        pages_deployment_id: &str,
    ) -> ClientResult<crate::Response<crate::types::PagesDeploymentStatusData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pages/deployments/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&pages_deployment_id.to_string()),
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
     * Cancel a GitHub Pages deployment.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/pages/deployments/{pages_deployment_id}/cancel` endpoint.
     *
     * Cancels a GitHub Pages deployment.
     *
     * The authenticated user must have write permissions for the GitHub Pages site.
     *
     * FROM: <https://docs.github.com/rest/pages/pages#cancel-a-github-pages-deployment>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pages_deployment_id: &str` -- The ID of the Pages deployment. You can also give the commit SHA of the deployment.
     */
    pub async fn cancel_pages_deployment(
        &self,
        owner: &str,
        repo: &str,
        pages_deployment_id: &str,
    ) -> ClientResult<crate::Response<String>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pages/deployments/{}/cancel",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&pages_deployment_id.to_string()),
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
     * Get a DNS health check for GitHub Pages.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pages/health` endpoint.
     *
     * Gets a health check of the DNS settings for the `CNAME` record configured for a repository's GitHub Pages.
     *
     * The first request to this endpoint returns a `202 Accepted` status and starts an asynchronous background task to get the results for the domain. After the background task completes, subsequent requests to this endpoint return a `200 OK` status with the health check results in the response.
     *
     * The authenticated user must be a repository administrator, maintainer, or have the 'manage GitHub Pages settings' permission to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/pages/pages#get-a-dns-health-check-for-github-pages>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_pages_health_check(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::PagesHealthCheck>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pages/health",
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
     * Check if private vulnerability reporting is enabled for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/private-vulnerability-reporting` endpoint.
     *
     * Returns a boolean indicating whether or not private vulnerability reporting is enabled for the repository. For more information, see "[Evaluating the security settings of a repository](https://docs.github.com/code-security/security-advisories/working-with-repository-security-advisories/evaluating-the-security-settings-of-a-repository)".
     *
     * FROM: <https://docs.github.com/rest/repos/repos#check-if-private-vulnerability-reporting-is-enabled-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn check_private_vulnerability_reporting(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::BlockCreations>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/private-vulnerability-reporting",
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
     * Enable private vulnerability reporting for a repository.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/private-vulnerability-reporting` endpoint.
     *
     * Enables private vulnerability reporting for a repository. The authenticated user must have admin access to the repository. For more information, see "[Privately reporting a security vulnerability](https://docs.github.com/code-security/security-advisories/guidance-on-reporting-and-writing/privately-reporting-a-security-vulnerability)."
     *
     * FROM: <https://docs.github.com/rest/repos/repos#enable-private-vulnerability-reporting-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn enable_private_vulnerability_reporting(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<String>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/private-vulnerability-reporting",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Disable private vulnerability reporting for a repository.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/private-vulnerability-reporting` endpoint.
     *
     * Disables private vulnerability reporting for a repository. The authenticated user must have admin access to the repository. For more information, see "[Privately reporting a security vulnerability](https://docs.github.com/code-security/security-advisories/guidance-on-reporting-and-writing/privately-reporting-a-security-vulnerability)".
     *
     * FROM: <https://docs.github.com/rest/repos/repos#disable-private-vulnerability-reporting-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn disable_private_vulnerability_reporting(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<String>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/private-vulnerability-reporting",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Get all custom property values for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/properties/values` endpoint.
     *
     * Gets all custom property values that are set for a repository.
     * Users with read access to the repository can use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/repos/custom-properties#get-all-custom-property-values-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn custom_properties_for_repos_get_repository_values(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CustomPropertyValue>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/properties/values",
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
     * Get all custom property values for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/properties/values` endpoint.
     *
     * As opposed to `custom_properties_for_repos_get_repository_values`, this function returns all the pages of the request at once.
     *
     * Gets all custom property values that are set for a repository.
     * Users with read access to the repository can use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/repos/custom-properties#get-all-custom-property-values-for-a-repository>
     */
    pub async fn custom_properties_for_repos_get_all_repository_values(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CustomPropertyValue>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/properties/values",
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
     * Create or update custom property values for a repository.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/properties/values` endpoint.
     *
     * Create new or update existing custom property values for a repository.
     * Using a value of `null` for a custom property will remove or 'unset' the property value from the repository.
     *
     * Repository admins and other users with the repository-level "edit custom property values" fine-grained permission can use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/repos/custom-properties#create-or-update-custom-property-values-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn custom_properties_for_repos_create_or_update_repository_values(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposCustomPropertiesCreateUpdateRepositoryValuesRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/properties/values",
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
     * Get a repository README.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/readme` endpoint.
     *
     * Gets the preferred README for a repository.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw file contents. This is the default if you do not specify a media type.
     * - **`application/vnd.github.html+json`**: Returns the README in HTML. Markup languages are rendered to HTML using GitHub's open-source [Markup library](https://github.com/github/markup).
     *
     * FROM: <https://docs.github.com/rest/repos/contents#get-a-repository-readme>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `ref_: &str` -- The name of the commit/branch/tag. Default: the repository’s default branch.
     */
    pub async fn get_readme(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> ClientResult<crate::Response<crate::types::ContentFile>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/readme?{}",
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
     * Get a repository README for a directory.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/readme/{dir}` endpoint.
     *
     * Gets the README from a repository directory.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw file contents. This is the default if you do not specify a media type.
     * - **`application/vnd.github.html+json`**: Returns the README in HTML. Markup languages are rendered to HTML using GitHub's open-source [Markup library](https://github.com/github/markup).
     *
     * FROM: <https://docs.github.com/rest/repos/contents#get-a-repository-readme-for-a-directory>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `dir: &str` -- The alternate path to look for a README file.
     * * `ref_: &str` -- The name of the commit/branch/tag. Default: the repository’s default branch.
     */
    pub async fn get_readme_in_directory(
        &self,
        owner: &str,
        repo: &str,
        dir: &str,
        ref_: &str,
    ) -> ClientResult<crate::Response<crate::types::ContentFile>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/readme/{}?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&dir.to_string()),
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
     * List releases.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/releases` endpoint.
     *
     * This returns a list of releases, which does not include regular Git tags that have not been associated with a release. To get a list of Git tags, use the [Repository Tags API](https://docs.github.com/rest/repos/repos#list-repository-tags).
     *
     * Information about published releases are available to everyone. Only users with push access will receive listings for draft releases.
     *
     * FROM: <https://docs.github.com/rest/releases/releases#list-releases>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_releases(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Release>>> {
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
                "/repos/{}/{}/releases?{}",
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
     * List releases.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/releases` endpoint.
     *
     * As opposed to `list_releases`, this function returns all the pages of the request at once.
     *
     * This returns a list of releases, which does not include regular Git tags that have not been associated with a release. To get a list of Git tags, use the [Repository Tags API](https://docs.github.com/rest/repos/repos#list-repository-tags).
     *
     * Information about published releases are available to everyone. Only users with push access will receive listings for draft releases.
     *
     * FROM: <https://docs.github.com/rest/releases/releases#list-releases>
     */
    pub async fn list_all_releases(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Release>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases",
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
     * Create a release.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/releases` endpoint.
     *
     * Users with push access to the repository can create a release.
     *
     * This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. For more information, see "[Rate limits for the API](https://docs.github.com/rest/using-the-rest-api/rate-limits-for-the-rest-api#about-secondary-rate-limits)" and "[Best practices for using the REST API](https://docs.github.com/rest/guides/best-practices-for-using-the-rest-api)."
     *
     * FROM: <https://docs.github.com/rest/releases/releases#create-a-release>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_release(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposCreateReleaseRequest,
    ) -> ClientResult<crate::Response<crate::types::Release>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases",
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
     * Get a release asset.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/releases/assets/{asset_id}` endpoint.
     *
     * To download the asset's binary content:
     *
     * - If within a browser, fetch the location specified in the `browser_download_url` key provided in the response.
     * - Alternatively, set the `Accept` header of the request to
     *   [`application/octet-stream`](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types).
     *   The API will either redirect the client to the location, or stream it directly if possible.
     *   API clients should handle both a `200` or `302` response.
     *
     * FROM: <https://docs.github.com/rest/releases/assets#get-a-release-asset>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `asset_id: i64` -- The unique identifier of the asset.
     */
    pub async fn get_release_asset(
        &self,
        owner: &str,
        repo: &str,
        asset_id: i64,
    ) -> ClientResult<crate::Response<crate::types::ReleaseAsset>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/assets/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&asset_id.to_string()),
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
     * Delete a release asset.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/releases/assets/{asset_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/releases/assets#delete-a-release-asset>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `asset_id: i64` -- The unique identifier of the asset.
     */
    pub async fn delete_release_asset(
        &self,
        owner: &str,
        repo: &str,
        asset_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/assets/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&asset_id.to_string()),
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
     * Update a release asset.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/releases/assets/{asset_id}` endpoint.
     *
     * Users with push access to the repository can edit a release asset.
     *
     * FROM: <https://docs.github.com/rest/releases/assets#update-a-release-asset>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `asset_id: i64` -- The unique identifier of the asset.
     */
    pub async fn update_release_asset(
        &self,
        owner: &str,
        repo: &str,
        asset_id: i64,
        body: &crate::types::ReposUpdateReleaseAssetRequest,
    ) -> ClientResult<crate::Response<crate::types::ReleaseAsset>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/assets/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&asset_id.to_string()),
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
     * Generate release notes content for a release.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/releases/generate-notes` endpoint.
     *
     * Generate a name and body describing a [release](https://docs.github.com/rest/releases/releases#get-a-release). The body content will be markdown formatted and contain information like the changes since last release and users who contributed. The generated release notes are not saved anywhere. They are intended to be generated and used when creating a new release.
     *
     * FROM: <https://docs.github.com/rest/releases/releases#generate-release-notes-content-for-a-release>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn generate_release_notes(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposGenerateReleaseNotesRequest,
    ) -> ClientResult<crate::Response<crate::types::ReleaseNotesContent>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/generate-notes",
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
     * Get the latest release.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/releases/latest` endpoint.
     *
     * View the latest published full release for the repository.
     *
     * The latest release is the most recent non-prerelease, non-draft release, sorted by the `created_at` attribute. The `created_at` attribute is the date of the commit used for the release, and not the date when the release was drafted or published.
     *
     * FROM: <https://docs.github.com/rest/releases/releases#get-the-latest-release>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_latest_release(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::Release>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/latest",
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
     * Get a release by tag name.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/releases/tags/{tag}` endpoint.
     *
     * Get a published release with the specified tag.
     *
     * FROM: <https://docs.github.com/rest/releases/releases#get-a-release-by-tag-name>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `tag: &str` -- tag parameter.
     */
    pub async fn get_release_by_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> ClientResult<crate::Response<crate::types::Release>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/tags/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&tag.to_string()),
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
     * Get a release.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/releases/{release_id}` endpoint.
     *
     * Gets a public release with the specified release ID.
     *
     * > [!NOTE]
     * > This returns an `upload_url` key corresponding to the endpoint for uploading release assets. This key is a hypermedia resource. For more information, see "[Getting started with the REST API](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#hypermedia)."
     *
     * FROM: <https://docs.github.com/rest/releases/releases#get-a-release>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `release_id: i64` -- The unique identifier of the release.
     */
    pub async fn get_release(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
    ) -> ClientResult<crate::Response<crate::types::Release>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&release_id.to_string()),
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
     * Delete a release.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/releases/{release_id}` endpoint.
     *
     * Users with push access to the repository can delete a release.
     *
     * FROM: <https://docs.github.com/rest/releases/releases#delete-a-release>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `release_id: i64` -- The unique identifier of the release.
     */
    pub async fn delete_release(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&release_id.to_string()),
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
     * Update a release.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/releases/{release_id}` endpoint.
     *
     * Users with push access to the repository can edit a release.
     *
     * FROM: <https://docs.github.com/rest/releases/releases#update-a-release>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `release_id: i64` -- The unique identifier of the release.
     */
    pub async fn update_release(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
        body: &crate::types::ReposUpdateReleaseRequest,
    ) -> ClientResult<crate::Response<crate::types::Release>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&release_id.to_string()),
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
     * List release assets.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/releases/{release_id}/assets` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/releases/assets#list-release-assets>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `release_id: i64` -- The unique identifier of the release.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_release_assets(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::ReleaseAsset>>> {
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
                "/repos/{}/{}/releases/{}/assets?{}",
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
     * List release assets.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/releases/{release_id}/assets` endpoint.
     *
     * As opposed to `list_release_assets`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/releases/assets#list-release-assets>
     */
    pub async fn list_all_release_assets(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::ReleaseAsset>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/{}/assets",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&release_id.to_string()),
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
     * Upload a release asset.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/releases/{release_id}/assets` endpoint.
     *
     * This endpoint makes use of a [Hypermedia relation](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#hypermedia) to determine which URL to access. The endpoint you call to upload release assets is specific to your release. Use the `upload_url` returned in
     * the response of the [Create a release endpoint](https://docs.github.com/rest/releases/releases#create-a-release) to upload a release asset.
     *
     * You need to use an HTTP client which supports [SNI](http://en.wikipedia.org/wiki/Server_Name_Indication) to make calls to this endpoint.
     *
     * Most libraries will set the required `Content-Length` header automatically. Use the required `Content-Type` header to provide the media type of the asset. For a list of media types, see [Media Types](https://www.iana.org/assignments/media-types/media-types.xhtml). For example:
     *
     * `application/zip`
     *
     * GitHub expects the asset data in its raw binary form, rather than JSON. You will send the raw binary content of the asset as the request body. Everything else about the endpoint is the same as the rest of the API. For example,
     * you'll still need to pass your authentication to be able to upload an asset.
     *
     * When an upstream failure occurs, you will receive a `502 Bad Gateway` status. This may leave an empty asset with a state of `starter`. It can be safely deleted.
     *
     * **Notes:**
     * *   GitHub renames asset filenames that have special characters, non-alphanumeric characters, and leading or trailing periods. The "[List release assets](https://docs.github.com/rest/releases/assets#list-release-assets)"
     * endpoint lists the renamed filenames. For more information and help, contact [GitHub Support](https://support.github.com/contact?tags=dotcom-rest-api).
     * *   To find the `release_id` query the [`GET /repos/{owner}/{repo}/releases/latest` endpoint](https://docs.github.com/rest/releases/releases#get-the-latest-release).
     * *   If you upload an asset with the same filename as another uploaded asset, you'll receive an error and must delete the old file before you can re-upload the new asset.
     *
     * FROM: <https://docs.github.com/rest/releases/assets#upload-a-release-asset>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `release_id: i64` -- The unique identifier of the release.
     * * `name: &str`
     * * `label: &str`
     */
    pub async fn upload_release_asset<B: Into<reqwest::Body>>(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
        name: &str,
        label: &str,
        body: B,
    ) -> ClientResult<crate::Response<crate::types::ReleaseAsset>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !label.is_empty() {
            query_args.push(("label".to_string(), label.to_string()));
        }
        if !name.is_empty() {
            query_args.push(("name".to_string(), name.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/{}/assets?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&release_id.to_string()),
                query_
            ),
            Some(ReposUploadReleaseAssetDefaultServer::default().default_url()),
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(body.into()),
                    content_type: Some("application/octet-stream".to_string()),
                },
            )
            .await
    }
    /**
     * Get rules for a branch.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/rules/branches/{branch}` endpoint.
     *
     * Returns all active rules that apply to the specified branch. The branch does not need to exist; rules that would apply
     * to a branch with that name will be returned. All active rules that apply will be returned, regardless of the level
     * at which they are configured (e.g. repository or organization). Rules in rulesets with "evaluate" or "disabled"
     * enforcement statuses are not returned.
     *
     * FROM: <https://docs.github.com/rest/repos/rules#get-rules-for-a-branch>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `branch: &str` -- The name of the branch. Cannot contain wildcard characters. To use wildcard characters in branch names, use [the GraphQL API](https://docs.github.com/graphql).
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn get_branch_rules(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrgRules>>> {
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
                "/repos/{}/{}/rules/branches/{}?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Get rules for a branch.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/rules/branches/{branch}` endpoint.
     *
     * As opposed to `get_branch_rules`, this function returns all the pages of the request at once.
     *
     * Returns all active rules that apply to the specified branch. The branch does not need to exist; rules that would apply
     * to a branch with that name will be returned. All active rules that apply will be returned, regardless of the level
     * at which they are configured (e.g. repository or organization). Rules in rulesets with "evaluate" or "disabled"
     * enforcement statuses are not returned.
     *
     * FROM: <https://docs.github.com/rest/repos/rules#get-rules-for-a-branch>
     */
    pub async fn get_all_branch_rules(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::OrgRules>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/rules/branches/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&branch.to_string()),
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
     * Get all repository rulesets.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/rulesets` endpoint.
     *
     * Get all the rulesets for a repository.
     *
     * FROM: <https://docs.github.com/rest/repos/rules#get-all-repository-rulesets>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `includes_parents: bool` -- Include rulesets configured at higher levels that apply to this repository.
     * * `targets: &str` -- A comma-separated list of rule targets to filter by.
     *   If provided, only rulesets that apply to the specified targets will be returned.
     *   For example, `branch,tag,push`.
     *   .
     */
    pub async fn get_repo_rulesets(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
        includes_parents: bool,
        targets: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::RepositoryRuleset>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if includes_parents {
            query_args.push(("includes_parents".to_string(), includes_parents.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !targets.is_empty() {
            query_args.push(("targets".to_string(), targets.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/rulesets?{}",
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
     * Get all repository rulesets.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/rulesets` endpoint.
     *
     * As opposed to `get_repo_rulesets`, this function returns all the pages of the request at once.
     *
     * Get all the rulesets for a repository.
     *
     * FROM: <https://docs.github.com/rest/repos/rules#get-all-repository-rulesets>
     */
    pub async fn get_all_repo_rulesets(
        &self,
        owner: &str,
        repo: &str,
        includes_parents: bool,
        targets: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::RepositoryRuleset>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if includes_parents {
            query_args.push(("includes_parents".to_string(), includes_parents.to_string()));
        }
        if !targets.is_empty() {
            query_args.push(("targets".to_string(), targets.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/rulesets?{}",
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
     * Create a repository ruleset.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/rulesets` endpoint.
     *
     * Create a ruleset for a repository.
     *
     * FROM: <https://docs.github.com/rest/repos/rules#create-a-repository-ruleset>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_repo_ruleset(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposCreateRepoRulesetRequest,
    ) -> ClientResult<crate::Response<crate::types::RepositoryRuleset>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/rulesets",
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
     * List repository rule suites.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/rulesets/rule-suites` endpoint.
     *
     * Lists suites of rule evaluations at the repository level.
     * For more information, see "[Managing rulesets for a repository](https://docs.github.com/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/managing-rulesets-for-a-repository#viewing-insights-for-rulesets)."
     *
     * FROM: <https://docs.github.com/rest/repos/rule-suites#list-repository-rule-suites>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `ref_: &str` -- The name of the ref. Cannot contain wildcard characters. Optionally prefix with `refs/heads/` to limit to branches or `refs/tags/` to limit to tags. Omit the prefix to search across all refs. When specified, only rule evaluations triggered for this ref will be returned.
     * * `time_period: crate::types::TimePeriodData` -- The time period to filter by.
     *  
     *  For example, `day` will filter for rule suites that occurred in the past 24 hours, and `week` will filter for rule suites that occurred in the past 7 days (168 hours).
     * * `actor_name: &str` -- The handle for the GitHub user account to filter on. When specified, only rule evaluations triggered by this actor will be returned.
     * * `rule_suite_result: crate::types::RuleSuiteResult` -- The rule suite results to filter on. When specified, only suites with this result will be returned.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn get_repo_rule_suites(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        time_period: crate::types::TimePeriodData,
        actor_name: &str,
        rule_suite_result: crate::types::RuleSuiteResult,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::RuleSuites>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !actor_name.is_empty() {
            query_args.push(("actor_name".to_string(), actor_name.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        if !rule_suite_result.to_string().is_empty() {
            query_args.push((
                "rule_suite_result".to_string(),
                rule_suite_result.to_string(),
            ));
        }
        if !time_period.to_string().is_empty() {
            query_args.push(("time_period".to_string(), time_period.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/rulesets/rule-suites?{}",
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
     * List repository rule suites.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/rulesets/rule-suites` endpoint.
     *
     * As opposed to `get_repo_rule_suites`, this function returns all the pages of the request at once.
     *
     * Lists suites of rule evaluations at the repository level.
     * For more information, see "[Managing rulesets for a repository](https://docs.github.com/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/managing-rulesets-for-a-repository#viewing-insights-for-rulesets)."
     *
     * FROM: <https://docs.github.com/rest/repos/rule-suites#list-repository-rule-suites>
     */
    pub async fn get_all_repo_rule_suites(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        time_period: crate::types::TimePeriodData,
        actor_name: &str,
        rule_suite_result: crate::types::RuleSuiteResult,
    ) -> ClientResult<crate::Response<Vec<crate::types::RuleSuites>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !actor_name.is_empty() {
            query_args.push(("actor_name".to_string(), actor_name.to_string()));
        }
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        if !rule_suite_result.to_string().is_empty() {
            query_args.push((
                "rule_suite_result".to_string(),
                rule_suite_result.to_string(),
            ));
        }
        if !time_period.to_string().is_empty() {
            query_args.push(("time_period".to_string(), time_period.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/rulesets/rule-suites?{}",
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
     * Get a repository rule suite.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/rulesets/rule-suites/{rule_suite_id}` endpoint.
     *
     * Gets information about a suite of rule evaluations from within a repository.
     * For more information, see "[Managing rulesets for a repository](https://docs.github.com/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/managing-rulesets-for-a-repository#viewing-insights-for-rulesets)."
     *
     * FROM: <https://docs.github.com/rest/repos/rule-suites#get-a-repository-rule-suite>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `rule_suite_id: i64` -- The unique identifier of the rule suite result.
     *   To get this ID, you can use [GET /repos/{owner}/{repo}/rulesets/rule-suites](https://docs.github.com/rest/repos/rule-suites#list-repository-rule-suites)
     *   for repositories and [GET /orgs/{org}/rulesets/rule-suites](https://docs.github.com/rest/orgs/rule-suites#list-organization-rule-suites)
     *   for organizations.
     */
    pub async fn get_repo_rule_suite(
        &self,
        owner: &str,
        repo: &str,
        rule_suite_id: i64,
    ) -> ClientResult<crate::Response<crate::types::RuleSuite>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/rulesets/rule-suites/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&rule_suite_id.to_string()),
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
     * Get a repository ruleset.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/rulesets/{ruleset_id}` endpoint.
     *
     * Get a ruleset for a repository.
     *
     * **Note:** To prevent leaking sensitive information, the `bypass_actors` property is only returned if the user
     * making the API request has write access to the ruleset.
     *
     * FROM: <https://docs.github.com/rest/repos/rules#get-a-repository-ruleset>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `ruleset_id: i64` -- The ID of the ruleset.
     * * `includes_parents: bool` -- Include rulesets configured at higher levels that apply to this repository.
     */
    pub async fn get_repo_ruleset(
        &self,
        owner: &str,
        repo: &str,
        ruleset_id: i64,
        includes_parents: bool,
    ) -> ClientResult<crate::Response<crate::types::RepositoryRuleset>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if includes_parents {
            query_args.push(("includes_parents".to_string(), includes_parents.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/rulesets/{}?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&ruleset_id.to_string()),
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
     * Update a repository ruleset.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/rulesets/{ruleset_id}` endpoint.
     *
     * Update a ruleset for a repository.
     *
     * FROM: <https://docs.github.com/rest/repos/rules#update-a-repository-ruleset>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `ruleset_id: i64` -- The ID of the ruleset.
     */
    pub async fn update_repo_ruleset(
        &self,
        owner: &str,
        repo: &str,
        ruleset_id: i64,
        body: &crate::types::ReposUpdateRepoRulesetRequest,
    ) -> ClientResult<crate::Response<crate::types::RepositoryRuleset>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/rulesets/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&ruleset_id.to_string()),
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
     * Delete a repository ruleset.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/rulesets/{ruleset_id}` endpoint.
     *
     * Delete a ruleset for a repository.
     *
     * FROM: <https://docs.github.com/rest/repos/rules#delete-a-repository-ruleset>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `ruleset_id: i64` -- The ID of the ruleset.
     */
    pub async fn delete_repo_ruleset(
        &self,
        owner: &str,
        repo: &str,
        ruleset_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/rulesets/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&ruleset_id.to_string()),
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
     * Get repository ruleset history.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/rulesets/{ruleset_id}/history` endpoint.
     *
     * Get the history of a repository ruleset.
     *
     * FROM: <https://docs.github.com/rest/repos/rules#get-repository-ruleset-history>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `ruleset_id: i64` -- The ID of the ruleset.
     */
    pub async fn get_repo_ruleset_history(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
        ruleset_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::RulesetVersion>>> {
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
                "/repos/{}/{}/rulesets/{}/history?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&ruleset_id.to_string()),
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
     * Get repository ruleset history.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/rulesets/{ruleset_id}/history` endpoint.
     *
     * As opposed to `get_repo_ruleset_history`, this function returns all the pages of the request at once.
     *
     * Get the history of a repository ruleset.
     *
     * FROM: <https://docs.github.com/rest/repos/rules#get-repository-ruleset-history>
     */
    pub async fn get_all_repo_ruleset_history(
        &self,
        owner: &str,
        repo: &str,
        ruleset_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::RulesetVersion>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/rulesets/{}/history",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&ruleset_id.to_string()),
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
     * Get repository ruleset version.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/rulesets/{ruleset_id}/history/{version_id}` endpoint.
     *
     * Get a version of a repository ruleset.
     *
     * FROM: <https://docs.github.com/rest/repos/rules#get-repository-ruleset-version>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `ruleset_id: i64` -- The ID of the ruleset.
     * * `version_id: i64` -- The ID of the version.
     */
    pub async fn get_repo_ruleset_version(
        &self,
        owner: &str,
        repo: &str,
        ruleset_id: i64,
        version_id: i64,
    ) -> ClientResult<crate::Response<crate::types::RulesetVersionWithStateAllOf>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/rulesets/{}/history/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&ruleset_id.to_string()),
                crate::progenitor_support::encode_path(&version_id.to_string()),
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
     * Get the weekly commit activity.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/stats/code_frequency` endpoint.
     *
     * Returns a weekly aggregate of the number of additions and deletions pushed to a repository.
     *
     * > [!NOTE]
     * > This endpoint can only be used for repositories with fewer than 10,000 commits. If the repository contains 10,000 or more commits, a 422 status code will be returned.
     *
     * FROM: <https://docs.github.com/rest/metrics/statistics#get-the-weekly-commit-activity>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_code_frequency_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<Vec<i64>>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/stats/code_frequency",
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
     * Get the weekly commit activity.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/stats/code_frequency` endpoint.
     *
     * As opposed to `get_code_frequency_stats`, this function returns all the pages of the request at once.
     *
     * Returns a weekly aggregate of the number of additions and deletions pushed to a repository.
     *
     * > [!NOTE]
     * > This endpoint can only be used for repositories with fewer than 10,000 commits. If the repository contains 10,000 or more commits, a 422 status code will be returned.
     *
     * FROM: <https://docs.github.com/rest/metrics/statistics#get-the-weekly-commit-activity>
     */
    pub async fn get_all_code_frequency_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<Vec<i64>>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/stats/code_frequency",
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
     * Get the last year of commit activity.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/stats/commit_activity` endpoint.
     *
     * Returns the last year of commit activity grouped by week. The `days` array is a group of commits per day, starting on `Sunday`.
     *
     * FROM: <https://docs.github.com/rest/metrics/statistics#get-the-last-year-of-commit-activity>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_commit_activity_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CommitActivity>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/stats/commit_activity",
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
     * Get the last year of commit activity.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/stats/commit_activity` endpoint.
     *
     * As opposed to `get_commit_activity_stats`, this function returns all the pages of the request at once.
     *
     * Returns the last year of commit activity grouped by week. The `days` array is a group of commits per day, starting on `Sunday`.
     *
     * FROM: <https://docs.github.com/rest/metrics/statistics#get-the-last-year-of-commit-activity>
     */
    pub async fn get_all_commit_activity_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CommitActivity>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/stats/commit_activity",
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
     * Get all contributor commit activity.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/stats/contributors` endpoint.
     *
     *
     * Returns the `total` number of commits authored by the contributor. In addition, the response includes a Weekly Hash (`weeks` array) with the following information:
     *
     * *   `w` - Start of the week, given as a [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time).
     * *   `a` - Number of additions
     * *   `d` - Number of deletions
     * *   `c` - Number of commits
     *
     * > [!NOTE]
     * > This endpoint will return `0` values for all addition and deletion counts in repositories with 10,000 or more commits.
     *
     * FROM: <https://docs.github.com/rest/metrics/statistics#get-all-contributor-commit-activity>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_contributors_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ContributorActivity>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/stats/contributors",
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
     * Get all contributor commit activity.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/stats/contributors` endpoint.
     *
     * As opposed to `get_contributors_stats`, this function returns all the pages of the request at once.
     *
     *
     * Returns the `total` number of commits authored by the contributor. In addition, the response includes a Weekly Hash (`weeks` array) with the following information:
     *
     * *   `w` - Start of the week, given as a [Unix timestamp](https://en.wikipedia.org/wiki/Unix_time).
     * *   `a` - Number of additions
     * *   `d` - Number of deletions
     * *   `c` - Number of commits
     *
     * > [!NOTE]
     * > This endpoint will return `0` values for all addition and deletion counts in repositories with 10,000 or more commits.
     *
     * FROM: <https://docs.github.com/rest/metrics/statistics#get-all-contributor-commit-activity>
     */
    pub async fn get_all_contributors_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ContributorActivity>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/stats/contributors",
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
     * Get the weekly commit count.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/stats/participation` endpoint.
     *
     * Returns the total commit counts for the `owner` and total commit counts in `all`. `all` is everyone combined, including the `owner` in the last 52 weeks. If you'd like to get the commit counts for non-owners, you can subtract `owner` from `all`.
     *
     * The array order is oldest week (index 0) to most recent week.
     *
     * The most recent week is seven days ago at UTC midnight to today at UTC midnight.
     *
     * FROM: <https://docs.github.com/rest/metrics/statistics#get-the-weekly-commit-count>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_participation_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::ParticipationStats>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/stats/participation",
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
     * Get the hourly commit count for each day.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/stats/punch_card` endpoint.
     *
     * Each array contains the day number, hour number, and number of commits:
     *
     * *   `0-6`: Sunday - Saturday
     * *   `0-23`: Hour of day
     * *   Number of commits
     *
     * For example, `[2, 14, 25]` indicates that there were 25 total commits, during the 2:00pm hour on Tuesdays. All times are based on the time zone of individual commits.
     *
     * FROM: <https://docs.github.com/rest/metrics/statistics#get-the-hourly-commit-count-for-each-day>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_punch_card_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<Vec<i64>>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/stats/punch_card",
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
     * Get the hourly commit count for each day.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/stats/punch_card` endpoint.
     *
     * As opposed to `get_punch_card_stats`, this function returns all the pages of the request at once.
     *
     * Each array contains the day number, hour number, and number of commits:
     *
     * *   `0-6`: Sunday - Saturday
     * *   `0-23`: Hour of day
     * *   Number of commits
     *
     * For example, `[2, 14, 25]` indicates that there were 25 total commits, during the 2:00pm hour on Tuesdays. All times are based on the time zone of individual commits.
     *
     * FROM: <https://docs.github.com/rest/metrics/statistics#get-the-hourly-commit-count-for-each-day>
     */
    pub async fn get_all_punch_card_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<Vec<i64>>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/stats/punch_card",
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
     * Create a commit status.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/statuses/{sha}` endpoint.
     *
     * Users with push access in a repository can create commit statuses for a given SHA.
     *
     * Note: there is a limit of 1000 statuses per `sha` and `context` within a repository. Attempts to create more than 1000 statuses will result in a validation error.
     *
     * FROM: <https://docs.github.com/rest/commits/statuses#create-a-commit-status>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `sha: &str`
     */
    pub async fn create_commit_status(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        body: &crate::types::ReposCreateCommitStatusRequest,
    ) -> ClientResult<crate::Response<crate::types::StatusData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/statuses/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&sha.to_string()),
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
     * List repository tags.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/tags` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/repos/repos#list-repository-tags>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_tags(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Tag>>> {
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
                "/repos/{}/{}/tags?{}",
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
     * List repository tags.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/tags` endpoint.
     *
     * As opposed to `list_tags`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/repos/repos#list-repository-tags>
     */
    pub async fn list_all_tags(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Tag>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/tags",
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
     * Download a repository archive (tar).
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/tarball/{ref}` endpoint.
     *
     * Gets a redirect URL to download a tar archive for a repository. If you omit `:ref`, the repository’s default branch (usually
     * `main`) will be used. Please make sure your HTTP framework is configured to follow redirects or you will need to use
     * the `Location` header to make a second `GET` request.
     *
     * > [!NOTE]
     * > For private repositories, these links are temporary and expire after five minutes.
     *
     * FROM: <https://docs.github.com/rest/repos/contents#download-a-repository-archive-tar>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `ref_: &str`
     */
    pub async fn download_tarball_archive(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/tarball/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&ref_.to_string()),
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
     * List repository teams.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/teams` endpoint.
     *
     * Lists the teams that have access to the specified repository and that are also visible to the authenticated user.
     *
     * For a public repository, a team is listed only if that team added the public repository explicitly.
     *
     * OAuth app tokens and personal access tokens (classic) need the `public_repo` or `repo` scope to use this endpoint with a public repository, and `repo` scope to use this endpoint with a private repository.
     *
     * FROM: <https://docs.github.com/rest/repos/repos#list-repository-teams>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_teams(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::Team>>> {
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
                "/repos/{}/{}/teams?{}",
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
     * List repository teams.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/teams` endpoint.
     *
     * As opposed to `list_teams`, this function returns all the pages of the request at once.
     *
     * Lists the teams that have access to the specified repository and that are also visible to the authenticated user.
     *
     * For a public repository, a team is listed only if that team added the public repository explicitly.
     *
     * OAuth app tokens and personal access tokens (classic) need the `public_repo` or `repo` scope to use this endpoint with a public repository, and `repo` scope to use this endpoint with a private repository.
     *
     * FROM: <https://docs.github.com/rest/repos/repos#list-repository-teams>
     */
    pub async fn list_all_teams(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::Team>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/teams",
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
     * Get all repository topics.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/topics` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/repos/repos#get-all-repository-topics>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn get_all_topics(
        &self,
        owner: &str,
        repo: &str,
        page: i64,
        per_page: i64,
    ) -> ClientResult<crate::Response<crate::types::Topic>> {
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
                "/repos/{}/{}/topics?{}",
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
     * Replace all repository topics.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/topics` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/repos/repos#replace-all-repository-topics>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn replace_all_topics(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::Topic,
    ) -> ClientResult<crate::Response<crate::types::Topic>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/topics",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Get repository clones.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/traffic/clones` endpoint.
     *
     * Get the total number of clones and breakdown per day or week for the last 14 days. Timestamps are aligned to UTC midnight of the beginning of the day or week. Week begins on Monday.
     *
     * FROM: <https://docs.github.com/rest/metrics/traffic#get-repository-clones>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per: crate::types::Per` -- The time frame to display results for.
     */
    pub async fn get_clones(
        &self,
        owner: &str,
        repo: &str,
        per: crate::types::Per,
    ) -> ClientResult<crate::Response<crate::types::CloneTraffic>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !per.to_string().is_empty() {
            query_args.push(("per".to_string(), per.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/traffic/clones?{}",
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
     * Get top referral paths.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/traffic/popular/paths` endpoint.
     *
     * Get the top 10 popular contents over the last 14 days.
     *
     * FROM: <https://docs.github.com/rest/metrics/traffic#get-top-referral-paths>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_top_paths(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ContentTraffic>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/traffic/popular/paths",
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
     * Get top referral paths.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/traffic/popular/paths` endpoint.
     *
     * As opposed to `get_top_paths`, this function returns all the pages of the request at once.
     *
     * Get the top 10 popular contents over the last 14 days.
     *
     * FROM: <https://docs.github.com/rest/metrics/traffic#get-top-referral-paths>
     */
    pub async fn get_all_top_paths(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ContentTraffic>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/traffic/popular/paths",
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
     * Get top referral sources.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/traffic/popular/referrers` endpoint.
     *
     * Get the top 10 referrers over the last 14 days.
     *
     * FROM: <https://docs.github.com/rest/metrics/traffic#get-top-referral-sources>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_top_referrers(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ReferrerTraffic>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/traffic/popular/referrers",
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
     * Get top referral sources.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/traffic/popular/referrers` endpoint.
     *
     * As opposed to `get_top_referrers`, this function returns all the pages of the request at once.
     *
     * Get the top 10 referrers over the last 14 days.
     *
     * FROM: <https://docs.github.com/rest/metrics/traffic#get-top-referral-sources>
     */
    pub async fn get_all_top_referrers(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::ReferrerTraffic>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/traffic/popular/referrers",
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
     * Get page views.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/traffic/views` endpoint.
     *
     * Get the total number of views and breakdown per day or week for the last 14 days. Timestamps are aligned to UTC midnight of the beginning of the day or week. Week begins on Monday.
     *
     * FROM: <https://docs.github.com/rest/metrics/traffic#get-page-views>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per: crate::types::Per` -- The time frame to display results for.
     */
    pub async fn get_views(
        &self,
        owner: &str,
        repo: &str,
        per: crate::types::Per,
    ) -> ClientResult<crate::Response<crate::types::ViewTraffic>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !per.to_string().is_empty() {
            query_args.push(("per".to_string(), per.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/traffic/views?{}",
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
     * Transfer a repository.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/transfer` endpoint.
     *
     * A transfer request will need to be accepted by the new owner when transferring a personal repository to another user. The response will contain the original `owner`, and the transfer will continue asynchronously. For more details on the requirements to transfer personal and organization-owned repositories, see [about repository transfers](https://docs.github.com/articles/about-repository-transfers/).
     *
     * FROM: <https://docs.github.com/rest/repos/repos#transfer-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn transfer(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposTransferRequest,
    ) -> ClientResult<crate::Response<crate::types::MinimalRepository>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/transfer",
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
     * Check if vulnerability alerts are enabled for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/vulnerability-alerts` endpoint.
     *
     * Shows whether dependency alerts are enabled or disabled for a repository. The authenticated user must have admin read access to the repository. For more information, see "[About security alerts for vulnerable dependencies](https://docs.github.com/articles/about-security-alerts-for-vulnerable-dependencies)".
     *
     * FROM: <https://docs.github.com/rest/repos/repos#check-if-vulnerability-alerts-are-enabled-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn check_vulnerability_alerts(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/vulnerability-alerts",
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
     * Enable vulnerability alerts.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/vulnerability-alerts` endpoint.
     *
     * Enables dependency alerts and the dependency graph for a repository. The authenticated user must have admin access to the repository. For more information, see "[About security alerts for vulnerable dependencies](https://docs.github.com/articles/about-security-alerts-for-vulnerable-dependencies)".
     *
     * FROM: <https://docs.github.com/rest/repos/repos#enable-vulnerability-alerts>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn enable_vulnerability_alerts(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/vulnerability-alerts",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Disable vulnerability alerts.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/vulnerability-alerts` endpoint.
     *
     * Disables dependency alerts and the dependency graph for a repository.
     * The authenticated user must have admin access to the repository. For more information,
     * see "[About security alerts for vulnerable dependencies](https://docs.github.com/articles/about-security-alerts-for-vulnerable-dependencies)".
     *
     * FROM: <https://docs.github.com/rest/repos/repos#disable-vulnerability-alerts>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn disable_vulnerability_alerts(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/vulnerability-alerts",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Download a repository archive (zip).
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/zipball/{ref}` endpoint.
     *
     * Gets a redirect URL to download a zip archive for a repository. If you omit `:ref`, the repository’s default branch (usually
     * `main`) will be used. Please make sure your HTTP framework is configured to follow redirects or you will need to use
     * the `Location` header to make a second `GET` request.
     *
     * > [!NOTE]
     * > For private repositories, these links are temporary and expire after five minutes. If the repository is empty, you will receive a 404 when you follow the redirect.
     *
     * FROM: <https://docs.github.com/rest/repos/contents#download-a-repository-archive-zip>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `ref_: &str`
     */
    pub async fn download_zipball_archive(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/zipball/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&ref_.to_string()),
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
     * Create a repository using a template.
     *
     * This function performs a `POST` to the `/repos/{template_owner}/{template_repo}/generate` endpoint.
     *
     * Creates a new repository using a repository template. Use the `template_owner` and `template_repo` route parameters to specify the repository to use as the template. If the repository is not public, the authenticated user must own or be a member of an organization that owns the repository. To check if a repository is available to use as a template, get the repository's information using the [Get a repository](https://docs.github.com/rest/repos/repos#get-a-repository) endpoint and check that the `is_template` key is `true`.
     *
     * OAuth app tokens and personal access tokens (classic) need the `public_repo` or `repo` scope to create a public repository, and `repo` scope to create a private repository.
     *
     * FROM: <https://docs.github.com/rest/repos/repos#create-a-repository-using-a-template>
     *
     * **Parameters:**
     *
     * * `template_owner: &str` -- The account owner of the template repository. The name is not case sensitive.
     * * `template_repo: &str` -- The name of the template repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_using_template(
        &self,
        template_owner: &str,
        template_repo: &str,
        body: &crate::types::ReposCreateUsingTemplateRequest,
    ) -> ClientResult<crate::Response<crate::types::FullRepository>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/generate",
                crate::progenitor_support::encode_path(&template_owner.to_string()),
                crate::progenitor_support::encode_path(&template_repo.to_string()),
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
     * List public repositories.
     *
     * This function performs a `GET` to the `/repositories` endpoint.
     *
     * Lists all public repositories in the order that they were created.
     *
     * Note:
     * - For GitHub Enterprise Server, this endpoint will only list repositories available to all users on the enterprise.
     * - Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers) to get the URL for the next page of repositories.
     *
     * FROM: <https://docs.github.com/rest/repos/repos#list-public-repositories>
     *
     * **Parameters:**
     *
     * * `since: i64` -- A repository ID. Only return repositories with an ID greater than this ID.
     */
    pub async fn list_public(
        &self,
        since: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::MinimalRepository>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if since > 0 {
            query_args.push(("since".to_string(), since.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/repositories?{}", query_), None);
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
     * List public repositories.
     *
     * This function performs a `GET` to the `/repositories` endpoint.
     *
     * As opposed to `list_public`, this function returns all the pages of the request at once.
     *
     * Lists all public repositories in the order that they were created.
     *
     * Note:
     * - For GitHub Enterprise Server, this endpoint will only list repositories available to all users on the enterprise.
     * - Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers) to get the URL for the next page of repositories.
     *
     * FROM: <https://docs.github.com/rest/repos/repos#list-public-repositories>
     */
    pub async fn list_all_public(
        &self,
        since: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::MinimalRepository>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if since > 0 {
            query_args.push(("since".to_string(), since.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/repositories?{}", query_), None);
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
     * List repositories for the authenticated user.
     *
     * This function performs a `GET` to the `/user/repos` endpoint.
     *
     * Lists repositories that the authenticated user has explicit permission (`:read`, `:write`, or `:admin`) to access.
     *
     * The authenticated user has explicit permission to access repositories they own, repositories where they are a collaborator, and repositories that they can access through an organization membership.
     *
     * FROM: <https://docs.github.com/rest/repos/repos#list-repositories-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `visibility: crate::types::ReposListVisibility` -- Limit results to repositories with the specified visibility.
     * * `affiliation: &str` -- Comma-separated list of values. Can include:  
     *    * `owner`: Repositories that are owned by the authenticated user.  
     *    * `collaborator`: Repositories that the user has been added to as a collaborator.  
     *    * `organization_member`: Repositories that the user has access to through being a member of an organization. This includes every repository on every team that the user is on.
     * * `type_: crate::types::ReposListType` -- Limit results to repositories of the specified type. Will cause a `422` error if used in the same request as \*\*visibility\*\* or \*\*affiliation\*\*.
     * * `sort: crate::types::ReposListOrgSort` -- The property to sort the results by.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show repositories updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `before: chrono::DateTime<chrono::Utc>` -- Only show repositories updated before the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     */
    pub async fn list_for_authenticated_user(
        &self,
        visibility: crate::types::ReposListVisibility,
        affiliation: &str,
        type_: crate::types::ReposListType,
        sort: crate::types::ReposListOrgSort,
        direction: crate::types::Order,
        per_page: i64,
        page: i64,
        since: Option<chrono::DateTime<chrono::Utc>>,
        before: Option<chrono::DateTime<chrono::Utc>>,
    ) -> ClientResult<crate::Response<Vec<crate::types::Repository>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !affiliation.is_empty() {
            query_args.push(("affiliation".to_string(), affiliation.to_string()));
        }
        if let Some(date) = before {
            query_args.push(("before".to_string(), date.to_rfc3339()));
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
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        if !visibility.to_string().is_empty() {
            query_args.push(("visibility".to_string(), visibility.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/user/repos?{}", query_), None);
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
     * List repositories for the authenticated user.
     *
     * This function performs a `GET` to the `/user/repos` endpoint.
     *
     * As opposed to `list_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * Lists repositories that the authenticated user has explicit permission (`:read`, `:write`, or `:admin`) to access.
     *
     * The authenticated user has explicit permission to access repositories they own, repositories where they are a collaborator, and repositories that they can access through an organization membership.
     *
     * FROM: <https://docs.github.com/rest/repos/repos#list-repositories-for-the-authenticated-user>
     */
    pub async fn list_all_for_authenticated_user(
        &self,
        visibility: crate::types::ReposListVisibility,
        affiliation: &str,
        type_: crate::types::ReposListType,
        sort: crate::types::ReposListOrgSort,
        direction: crate::types::Order,
        since: Option<chrono::DateTime<chrono::Utc>>,
        before: Option<chrono::DateTime<chrono::Utc>>,
    ) -> ClientResult<crate::Response<Vec<crate::types::Repository>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !affiliation.is_empty() {
            query_args.push(("affiliation".to_string(), affiliation.to_string()));
        }
        if let Some(date) = before {
            query_args.push(("before".to_string(), date.to_rfc3339()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if let Some(date) = since {
            query_args.push(("since".to_string(), date.to_rfc3339()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        if !visibility.to_string().is_empty() {
            query_args.push(("visibility".to_string(), visibility.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/user/repos?{}", query_), None);
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
     * Create a repository for the authenticated user.
     *
     * This function performs a `POST` to the `/user/repos` endpoint.
     *
     * Creates a new repository for the authenticated user.
     *
     * OAuth app tokens and personal access tokens (classic) need the `public_repo` or `repo` scope to create a public repository, and `repo` scope to create a private repository.
     *
     * FROM: <https://docs.github.com/rest/repos/repos#create-a-repository-for-the-authenticated-user>
     */
    pub async fn create_for_authenticated_user(
        &self,
        body: &crate::types::ReposCreateRequest,
    ) -> ClientResult<crate::Response<crate::types::FullRepository>> {
        let url = self.client.url(&"/user/repos".to_string(), None);
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
     * List repository invitations for the authenticated user.
     *
     * This function performs a `GET` to the `/user/repository_invitations` endpoint.
     *
     * When authenticating as a user, this endpoint will list all currently open repository invitations for that user.
     *
     * FROM: <https://docs.github.com/rest/collaborators/invitations#list-repository-invitations-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_invitations_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::RepositoryInvitation>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/user/repository_invitations?{}", query_), None);
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
     * List repository invitations for the authenticated user.
     *
     * This function performs a `GET` to the `/user/repository_invitations` endpoint.
     *
     * As opposed to `list_invitations_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * When authenticating as a user, this endpoint will list all currently open repository invitations for that user.
     *
     * FROM: <https://docs.github.com/rest/collaborators/invitations#list-repository-invitations-for-the-authenticated-user>
     */
    pub async fn list_all_invitations_for_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::RepositoryInvitation>>> {
        let url = self
            .client
            .url(&"/user/repository_invitations".to_string(), None);
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
     * Decline a repository invitation.
     *
     * This function performs a `DELETE` to the `/user/repository_invitations/{invitation_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/collaborators/invitations#decline-a-repository-invitation>
     *
     * **Parameters:**
     *
     * * `invitation_id: i64` -- The unique identifier of the invitation.
     */
    pub async fn decline_invitation_for_authenticated_user(
        &self,
        invitation_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/repository_invitations/{}",
                crate::progenitor_support::encode_path(&invitation_id.to_string()),
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
     * Accept a repository invitation.
     *
     * This function performs a `PATCH` to the `/user/repository_invitations/{invitation_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/collaborators/invitations#accept-a-repository-invitation>
     *
     * **Parameters:**
     *
     * * `invitation_id: i64` -- The unique identifier of the invitation.
     */
    pub async fn accept_invitation_for_authenticated_user(
        &self,
        invitation_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/repository_invitations/{}",
                crate::progenitor_support::encode_path(&invitation_id.to_string()),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * List repositories for a user.
     *
     * This function performs a `GET` to the `/users/{username}/repos` endpoint.
     *
     * Lists public repositories for the specified user.
     *
     * FROM: <https://docs.github.com/rest/repos/repos#list-repositories-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     * * `type_: crate::types::ReposListUserType` -- Limit results to repositories of the specified type.
     * * `sort: crate::types::ReposListOrgSort` -- The property to sort the results by.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_for_user(
        &self,
        username: &str,
        type_: crate::types::ReposListUserType,
        sort: crate::types::ReposListOrgSort,
        direction: crate::types::Order,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::MinimalRepository>>> {
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
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/repos?{}",
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * List repositories for a user.
     *
     * This function performs a `GET` to the `/users/{username}/repos` endpoint.
     *
     * As opposed to `list_for_user`, this function returns all the pages of the request at once.
     *
     * Lists public repositories for the specified user.
     *
     * FROM: <https://docs.github.com/rest/repos/repos#list-repositories-for-a-user>
     */
    pub async fn list_all_for_user(
        &self,
        username: &str,
        type_: crate::types::ReposListUserType,
        sort: crate::types::ReposListOrgSort,
        direction: crate::types::Order,
    ) -> ClientResult<crate::Response<Vec<crate::types::MinimalRepository>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/repos?{}",
                crate::progenitor_support::encode_path(&username.to_string()),
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
