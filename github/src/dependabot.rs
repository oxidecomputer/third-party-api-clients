use crate::Client;
use crate::ClientResult;

pub struct Dependabot {
    pub client: Client,
}

impl Dependabot {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Dependabot { client }
    }

    /**
     * List Dependabot alerts for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/dependabot/alerts` endpoint.
     *
     * Lists Dependabot alerts for repositories that are owned by the specified enterprise.
     *
     * The authenticated user must be a member of the enterprise to use this endpoint.
     *
     * Alerts are only returned for organizations in the enterprise for which you are an organization owner or a security manager. For more information about security managers, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/dependabot/alerts#list-dependabot-alerts-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `state: &str` -- A comma-separated list of states. If specified, only alerts with these states will be returned.
     *   
     *   Can be: `auto_dismissed`, `dismissed`, `fixed`, `open`.
     * * `severity: &str` -- A comma-separated list of severities. If specified, only alerts with these severities will be returned.
     *   
     *   Can be: `low`, `medium`, `high`, `critical`.
     * * `ecosystem: &str` -- A comma-separated list of ecosystems. If specified, only alerts for these ecosystems will be returned.
     *   
     *   Can be: `composer`, `go`, `maven`, `npm`, `nuget`, `pip`, `pub`, `rubygems`, `rust`.
     * * `package: &str` -- A comma-separated list of package names. If specified, only alerts for these packages will be returned.
     * * `epss_percentage: &str` -- CVE Exploit Prediction Scoring System (EPSS) percentage. Can be specified as:
     *   - An exact number (`n`)
     *   - Comparators such as `>n`, `<n`, `>=n`, `<=n`
     *   - A range like `n..n`, where `n` is a number from 0.0 to 1.0
     *   
     *   Filters the list of alerts based on EPSS percentages. If specified, only alerts with the provided EPSS percentages will be returned.
     * * `has: &str` -- Filters the list of alerts based on whether the alert has the given value. If specified, only alerts meeting this criterion will be returned.
     *   Multiple `has` filters can be passed to filter for alerts that have all of the values. Currently, only `patch` is supported.
     * * `assignee: &str` -- Filter alerts by assignees.
     *   Provide a comma-separated list of user handles (e.g., `octocat` or `octocat,hubot`) to return alerts assigned to any of the specified users.
     *   Use `*` to list alerts with at least one assignee or `none` to list alerts with no assignees.
     * * `scope: crate::types::Scope` -- The scope of the vulnerable dependency. If specified, only alerts with this scope will be returned.
     * * `sort: crate::types::Sort` -- The property by which to sort the results.
     *  `created` means when the alert was created.
     *  `updated` means when the alert's state last changed.
     *  `epss_percentage` sorts alerts by the Exploit Prediction Scoring System (EPSS) percentage.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_alerts_for_enterprise(
        &self,
        enterprise: &str,
        state: &str,
        severity: &str,
        ecosystem: &str,
        package: &str,
        epss_percentage: &str,
        has: &str,
        assignee: &str,
        scope: crate::types::Scope,
        sort: crate::types::Sort,
        direction: crate::types::Order,
        before: &str,
        after: &str,
        per_page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::DependabotAlertWithRepository>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !assignee.is_empty() {
            query_args.push(("assignee".to_string(), assignee.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !ecosystem.is_empty() {
            query_args.push(("ecosystem".to_string(), ecosystem.to_string()));
        }
        if !epss_percentage.is_empty() {
            query_args.push(("epss_percentage".to_string(), epss_percentage.to_string()));
        }
        if !has.is_empty() {
            query_args.push(("has".to_string(), has.to_string()));
        }
        if !package.is_empty() {
            query_args.push(("package".to_string(), package.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !scope.to_string().is_empty() {
            query_args.push(("scope".to_string(), scope.to_string()));
        }
        if !severity.is_empty() {
            query_args.push(("severity".to_string(), severity.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/enterprises/{}/dependabot/alerts?{}",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
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
     * List Dependabot alerts for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/dependabot/alerts` endpoint.
     *
     * As opposed to `list_alerts_for_enterprise`, this function returns all the pages of the request at once.
     *
     * Lists Dependabot alerts for repositories that are owned by the specified enterprise.
     *
     * The authenticated user must be a member of the enterprise to use this endpoint.
     *
     * Alerts are only returned for organizations in the enterprise for which you are an organization owner or a security manager. For more information about security managers, see "[Managing security managers in your organization](https://docs.github.com/organizations/managing-peoples-access-to-your-organization-with-roles/managing-security-managers-in-your-organization)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` or `security_events` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/dependabot/alerts#list-dependabot-alerts-for-an-enterprise>
     */
    pub async fn list_all_alerts_for_enterprise(
        &self,
        enterprise: &str,
        state: &str,
        severity: &str,
        ecosystem: &str,
        package: &str,
        epss_percentage: &str,
        has: &str,
        assignee: &str,
        scope: crate::types::Scope,
        sort: crate::types::Sort,
        direction: crate::types::Order,
        before: &str,
        after: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::DependabotAlertWithRepository>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !assignee.is_empty() {
            query_args.push(("assignee".to_string(), assignee.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !ecosystem.is_empty() {
            query_args.push(("ecosystem".to_string(), ecosystem.to_string()));
        }
        if !epss_percentage.is_empty() {
            query_args.push(("epss_percentage".to_string(), epss_percentage.to_string()));
        }
        if !has.is_empty() {
            query_args.push(("has".to_string(), has.to_string()));
        }
        if !package.is_empty() {
            query_args.push(("package".to_string(), package.to_string()));
        }
        if !scope.to_string().is_empty() {
            query_args.push(("scope".to_string(), scope.to_string()));
        }
        if !severity.is_empty() {
            query_args.push(("severity".to_string(), severity.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/enterprises/{}/dependabot/alerts?{}",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
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
     * Lists the repositories Dependabot can access in an organization.
     *
     * This function performs a `GET` to the `/organizations/{org}/dependabot/repository-access` endpoint.
     *
     * Lists repositories that organization admins have allowed Dependabot to access when updating dependencies.
     * > [!NOTE]
     * >    This operation supports both server-to-server and user-to-server access.
     * Unauthorized users will not see the existence of this endpoint.
     *
     * FROM: <https://docs.github.com/rest/dependabot/repository-access#lists-the-repositories-dependabot-can-access-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `page: i64` -- The page number of results to fetch.
     * * `per_page: i64` -- Number of results per page.
     */
    pub async fn repository_access_for_org(
        &self,
        org: &str,
        page: i64,
        per_page: i64,
    ) -> ClientResult<crate::Response<crate::types::DependabotRepositoryAccessDetails>> {
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
                "/organizations/{}/dependabot/repository-access?{}",
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
     * Updates Dependabot's repository access list for an organization.
     *
     * This function performs a `PATCH` to the `/organizations/{org}/dependabot/repository-access` endpoint.
     *
     * Updates repositories according to the list of repositories that organization admins have given Dependabot access to when they've updated dependencies.
     *
     * > [!NOTE]
     * >    This operation supports both server-to-server and user-to-server access.
     * Unauthorized users will not see the existence of this endpoint.
     *
     * **Example request body:**
     * ```json
     * {
     *   "repository_ids_to_add": [123, 456],
     *   "repository_ids_to_remove": [789]
     * }
     * ```
     *
     * FROM: <https://docs.github.com/rest/dependabot/repository-access#updates-dependabots-repository-access-list-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn update_repository_access_for_org(
        &self,
        org: &str,
        body: &crate::types::DependabotUpdateRepositoryAccessOrgRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/organizations/{}/dependabot/repository-access",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Set the default repository access level for Dependabot.
     *
     * This function performs a `PUT` to the `/organizations/{org}/dependabot/repository-access/default-level` endpoint.
     *
     * Sets the default level of repository access Dependabot will have while performing an update.  Available values are:
     * - 'public' - Dependabot will only have access to public repositories, unless access is explicitly granted to non-public repositories.
     * - 'internal' - Dependabot will only have access to public and internal repositories, unless access is explicitly granted to private repositories.
     *
     * Unauthorized users will not see the existence of this endpoint.
     *
     * This operation supports both server-to-server and user-to-server access.
     *
     * FROM: <https://docs.github.com/rest/dependabot/repository-access#set-the-default-repository-access-level-for-dependabot>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn set_repository_access_default_level(
        &self,
        org: &str,
        body: &crate::types::DependabotSetRepositoryAccessDefaultLevelRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/organizations/{}/dependabot/repository-access/default-level",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List Dependabot alerts for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/dependabot/alerts` endpoint.
     *
     * Lists Dependabot alerts for an organization.
     *
     * The authenticated user must be an owner or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
     *
     * FROM: <https://docs.github.com/rest/dependabot/alerts#list-dependabot-alerts-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `state: &str` -- A comma-separated list of states. If specified, only alerts with these states will be returned.
     *   
     *   Can be: `auto_dismissed`, `dismissed`, `fixed`, `open`.
     * * `severity: &str` -- A comma-separated list of severities. If specified, only alerts with these severities will be returned.
     *   
     *   Can be: `low`, `medium`, `high`, `critical`.
     * * `ecosystem: &str` -- A comma-separated list of ecosystems. If specified, only alerts for these ecosystems will be returned.
     *   
     *   Can be: `composer`, `go`, `maven`, `npm`, `nuget`, `pip`, `pub`, `rubygems`, `rust`.
     * * `package: &str` -- A comma-separated list of package names. If specified, only alerts for these packages will be returned.
     * * `epss_percentage: &str` -- CVE Exploit Prediction Scoring System (EPSS) percentage. Can be specified as:
     *   - An exact number (`n`)
     *   - Comparators such as `>n`, `<n`, `>=n`, `<=n`
     *   - A range like `n..n`, where `n` is a number from 0.0 to 1.0
     *   
     *   Filters the list of alerts based on EPSS percentages. If specified, only alerts with the provided EPSS percentages will be returned.
     * * `artifact_registry_url: &str` -- A comma-separated list of artifact registry URLs. If specified, only alerts for repositories with storage records matching these URLs will be returned.
     * * `artifact_registry: &str` -- A comma-separated list of Artifact Registry name strings. If specified, only alerts for repositories with storage records matching these registries will be returned.
     *   
     *   Can be: `jfrog-artifactory`.
     * * `has: &str` -- Filters the list of alerts based on whether the alert has the given value. If specified, only alerts meeting this criterion will be returned.
     *   Multiple `has` filters can be passed to filter for alerts that have all of the values.
     * * `assignee: &str` -- Filter alerts by assignees.
     *   Provide a comma-separated list of user handles (e.g., `octocat` or `octocat,hubot`) to return alerts assigned to any of the specified users.
     *   Use `*` to list alerts with at least one assignee or `none` to list alerts with no assignees.
     * * `runtime_risk: &str` -- A comma-separated list of runtime risk strings. If specified, only alerts for repositories with deployment records matching these risks will be returned.
     *   
     *   Can be: `critical-resource`, `internet-exposed`, `sensitive-data`, `lateral-movement`.
     * * `scope: crate::types::Scope` -- The scope of the vulnerable dependency. If specified, only alerts with this scope will be returned.
     * * `sort: crate::types::Sort` -- The property by which to sort the results.
     *  `created` means when the alert was created.
     *  `updated` means when the alert's state last changed.
     *  `epss_percentage` sorts alerts by the Exploit Prediction Scoring System (EPSS) percentage.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_alerts_for_org(
        &self,
        org: &str,
        state: &str,
        severity: &str,
        ecosystem: &str,
        package: &str,
        epss_percentage: &str,
        artifact_registry_url: &str,
        artifact_registry: &str,
        has: &str,
        assignee: &str,
        runtime_risk: &str,
        scope: crate::types::Scope,
        sort: crate::types::Sort,
        direction: crate::types::Order,
        before: &str,
        after: &str,
        per_page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::DependabotAlertWithRepository>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !artifact_registry.is_empty() {
            query_args.push((
                "artifact_registry".to_string(),
                artifact_registry.to_string(),
            ));
        }
        if !artifact_registry_url.is_empty() {
            query_args.push((
                "artifact_registry_url".to_string(),
                artifact_registry_url.to_string(),
            ));
        }
        if !assignee.is_empty() {
            query_args.push(("assignee".to_string(), assignee.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !ecosystem.is_empty() {
            query_args.push(("ecosystem".to_string(), ecosystem.to_string()));
        }
        if !epss_percentage.is_empty() {
            query_args.push(("epss_percentage".to_string(), epss_percentage.to_string()));
        }
        if !has.is_empty() {
            query_args.push(("has".to_string(), has.to_string()));
        }
        if !package.is_empty() {
            query_args.push(("package".to_string(), package.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !runtime_risk.is_empty() {
            query_args.push(("runtime_risk".to_string(), runtime_risk.to_string()));
        }
        if !scope.to_string().is_empty() {
            query_args.push(("scope".to_string(), scope.to_string()));
        }
        if !severity.is_empty() {
            query_args.push(("severity".to_string(), severity.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/dependabot/alerts?{}",
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
     * List Dependabot alerts for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/dependabot/alerts` endpoint.
     *
     * As opposed to `list_alerts_for_org`, this function returns all the pages of the request at once.
     *
     * Lists Dependabot alerts for an organization.
     *
     * The authenticated user must be an owner or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
     *
     * FROM: <https://docs.github.com/rest/dependabot/alerts#list-dependabot-alerts-for-an-organization>
     */
    pub async fn list_all_alerts_for_org(
        &self,
        org: &str,
        state: &str,
        severity: &str,
        ecosystem: &str,
        package: &str,
        epss_percentage: &str,
        artifact_registry_url: &str,
        artifact_registry: &str,
        has: &str,
        assignee: &str,
        runtime_risk: &str,
        scope: crate::types::Scope,
        sort: crate::types::Sort,
        direction: crate::types::Order,
        before: &str,
        after: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::DependabotAlertWithRepository>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !artifact_registry.is_empty() {
            query_args.push((
                "artifact_registry".to_string(),
                artifact_registry.to_string(),
            ));
        }
        if !artifact_registry_url.is_empty() {
            query_args.push((
                "artifact_registry_url".to_string(),
                artifact_registry_url.to_string(),
            ));
        }
        if !assignee.is_empty() {
            query_args.push(("assignee".to_string(), assignee.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !ecosystem.is_empty() {
            query_args.push(("ecosystem".to_string(), ecosystem.to_string()));
        }
        if !epss_percentage.is_empty() {
            query_args.push(("epss_percentage".to_string(), epss_percentage.to_string()));
        }
        if !has.is_empty() {
            query_args.push(("has".to_string(), has.to_string()));
        }
        if !package.is_empty() {
            query_args.push(("package".to_string(), package.to_string()));
        }
        if !runtime_risk.is_empty() {
            query_args.push(("runtime_risk".to_string(), runtime_risk.to_string()));
        }
        if !scope.to_string().is_empty() {
            query_args.push(("scope".to_string(), scope.to_string()));
        }
        if !severity.is_empty() {
            query_args.push(("severity".to_string(), severity.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/dependabot/alerts?{}",
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
     * List organization secrets.
     *
     * This function performs a `GET` to the `/orgs/{org}/dependabot/secrets` endpoint.
     *
     * Lists all secrets available in an organization without revealing their
     * encrypted values.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/dependabot/secrets#list-organization-secrets>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_org_secrets(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::DependabotListOrgSecretsResponse>> {
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
                "/orgs/{}/dependabot/secrets?{}",
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
     * Get an organization public key.
     *
     * This function performs a `GET` to the `/orgs/{org}/dependabot/secrets/public-key` endpoint.
     *
     * Gets your public key, which you need to encrypt secrets. You need to
     * encrypt a secret before you can create or update secrets.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/dependabot/secrets#get-an-organization-public-key>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_org_public_key(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::DependabotPublicKey>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/dependabot/secrets/public-key",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Get an organization secret.
     *
     * This function performs a `GET` to the `/orgs/{org}/dependabot/secrets/{secret_name}` endpoint.
     *
     * Gets a single organization secret without revealing its encrypted value.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/dependabot/secrets#get-an-organization-secret>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `secret_name: &str` -- The name of the secret.
     */
    pub async fn get_org_secret(
        &self,
        org: &str,
        secret_name: &str,
    ) -> ClientResult<crate::Response<crate::types::OrganizationDependabotSecret>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/dependabot/secrets/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&secret_name.to_string()),
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
     * Create or update an organization secret.
     *
     * This function performs a `PUT` to the `/orgs/{org}/dependabot/secrets/{secret_name}` endpoint.
     *
     * Creates or updates an organization secret with an encrypted value. Encrypt your secret using
     * [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/dependabot/secrets#create-or-update-an-organization-secret>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `secret_name: &str` -- The name of the secret.
     */
    pub async fn create_or_update_org_secret(
        &self,
        org: &str,
        secret_name: &str,
        body: &crate::types::DependabotCreateUpdateOrgSecretRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/dependabot/secrets/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&secret_name.to_string()),
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
     * Delete an organization secret.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/dependabot/secrets/{secret_name}` endpoint.
     *
     * Deletes a secret in an organization using the secret name.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/dependabot/secrets#delete-an-organization-secret>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `secret_name: &str` -- The name of the secret.
     */
    pub async fn delete_org_secret(
        &self,
        org: &str,
        secret_name: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/dependabot/secrets/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&secret_name.to_string()),
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
     * List selected repositories for an organization secret.
     *
     * This function performs a `GET` to the `/orgs/{org}/dependabot/secrets/{secret_name}/repositories` endpoint.
     *
     * Lists all repositories that have been selected when the `visibility`
     * for repository access to a secret is set to `selected`.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/dependabot/secrets#list-selected-repositories-for-an-organization-secret>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `secret_name: &str` -- The name of the secret.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_selected_repos_for_org_secret(
        &self,
        org: &str,
        secret_name: &str,
        page: i64,
        per_page: i64,
    ) -> ClientResult<crate::Response<crate::types::CodespacesListRepositoriesSecretResponse>> {
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
                "/orgs/{}/dependabot/secrets/{}/repositories?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&secret_name.to_string()),
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
     * Set selected repositories for an organization secret.
     *
     * This function performs a `PUT` to the `/orgs/{org}/dependabot/secrets/{secret_name}/repositories` endpoint.
     *
     * Replaces all repositories for an organization secret when the `visibility`
     * for repository access is set to `selected`. The visibility is set when you [Create
     * or update an organization secret](https://docs.github.com/rest/dependabot/secrets#create-or-update-an-organization-secret).
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/dependabot/secrets#set-selected-repositories-for-an-organization-secret>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `secret_name: &str` -- The name of the secret.
     */
    pub async fn set_selected_repos_for_org_secret(
        &self,
        org: &str,
        secret_name: &str,
        body: &crate::types::CodespacesSetRepositoriesSecretRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/dependabot/secrets/{}/repositories",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&secret_name.to_string()),
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
     * Add selected repository to an organization secret.
     *
     * This function performs a `PUT` to the `/orgs/{org}/dependabot/secrets/{secret_name}/repositories/{repository_id}` endpoint.
     *
     * Adds a repository to an organization secret when the `visibility` for
     * repository access is set to `selected`. The visibility is set when you [Create or
     * update an organization secret](https://docs.github.com/rest/dependabot/secrets#create-or-update-an-organization-secret).
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/dependabot/secrets#add-selected-repository-to-an-organization-secret>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `secret_name: &str` -- The name of the secret.
     * * `repository_id: i64`
     */
    pub async fn add_selected_repo_to_org_secret(
        &self,
        org: &str,
        secret_name: &str,
        repository_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/dependabot/secrets/{}/repositories/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&secret_name.to_string()),
                crate::progenitor_support::encode_path(&repository_id.to_string()),
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
     * Remove selected repository from an organization secret.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/dependabot/secrets/{secret_name}/repositories/{repository_id}` endpoint.
     *
     * Removes a repository from an organization secret when the `visibility`
     * for repository access is set to `selected`. The visibility is set when you [Create
     * or update an organization secret](https://docs.github.com/rest/dependabot/secrets#create-or-update-an-organization-secret).
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/dependabot/secrets#remove-selected-repository-from-an-organization-secret>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `secret_name: &str` -- The name of the secret.
     * * `repository_id: i64`
     */
    pub async fn remove_selected_repo_from_org_secret(
        &self,
        org: &str,
        secret_name: &str,
        repository_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/dependabot/secrets/{}/repositories/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&secret_name.to_string()),
                crate::progenitor_support::encode_path(&repository_id.to_string()),
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
     * List Dependabot alerts for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/dependabot/alerts` endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
     *
     * FROM: <https://docs.github.com/rest/dependabot/alerts#list-dependabot-alerts-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `state: &str` -- A comma-separated list of states. If specified, only alerts with these states will be returned.
     *   
     *   Can be: `auto_dismissed`, `dismissed`, `fixed`, `open`.
     * * `severity: &str` -- A comma-separated list of severities. If specified, only alerts with these severities will be returned.
     *   
     *   Can be: `low`, `medium`, `high`, `critical`.
     * * `ecosystem: &str` -- A comma-separated list of ecosystems. If specified, only alerts for these ecosystems will be returned.
     *   
     *   Can be: `composer`, `go`, `maven`, `npm`, `nuget`, `pip`, `pub`, `rubygems`, `rust`.
     * * `package: &str` -- A comma-separated list of package names. If specified, only alerts for these packages will be returned.
     * * `manifest: &str` -- A comma-separated list of full manifest paths. If specified, only alerts for these manifests will be returned.
     * * `epss_percentage: &str` -- CVE Exploit Prediction Scoring System (EPSS) percentage. Can be specified as:
     *   - An exact number (`n`)
     *   - Comparators such as `>n`, `<n`, `>=n`, `<=n`
     *   - A range like `n..n`, where `n` is a number from 0.0 to 1.0
     *   
     *   Filters the list of alerts based on EPSS percentages. If specified, only alerts with the provided EPSS percentages will be returned.
     * * `has: &str` -- Filters the list of alerts based on whether the alert has the given value. If specified, only alerts meeting this criterion will be returned.
     *   Multiple `has` filters can be passed to filter for alerts that have all of the values. Currently, only `patch` is supported.
     * * `assignee: &str` -- Filter alerts by assignees.
     *   Provide a comma-separated list of user handles (e.g., `octocat` or `octocat,hubot`) to return alerts assigned to any of the specified users.
     *   Use `*` to list alerts with at least one assignee or `none` to list alerts with no assignees.
     * * `scope: crate::types::Scope` -- The scope of the vulnerable dependency. If specified, only alerts with this scope will be returned.
     * * `sort: crate::types::Sort` -- The property by which to sort the results.
     *  `created` means when the alert was created.
     *  `updated` means when the alert's state last changed.
     *  `epss_percentage` sorts alerts by the Exploit Prediction Scoring System (EPSS) percentage.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_alerts_for_repo(
        &self,
        owner: &str,
        repo: &str,
        state: &str,
        severity: &str,
        ecosystem: &str,
        package: &str,
        manifest: &str,
        epss_percentage: &str,
        has: &str,
        assignee: &str,
        scope: crate::types::Scope,
        sort: crate::types::Sort,
        direction: crate::types::Order,
        before: &str,
        after: &str,
        per_page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::DependabotAlert>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !assignee.is_empty() {
            query_args.push(("assignee".to_string(), assignee.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !ecosystem.is_empty() {
            query_args.push(("ecosystem".to_string(), ecosystem.to_string()));
        }
        if !epss_percentage.is_empty() {
            query_args.push(("epss_percentage".to_string(), epss_percentage.to_string()));
        }
        if !has.is_empty() {
            query_args.push(("has".to_string(), has.to_string()));
        }
        if !manifest.is_empty() {
            query_args.push(("manifest".to_string(), manifest.to_string()));
        }
        if !package.is_empty() {
            query_args.push(("package".to_string(), package.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !scope.to_string().is_empty() {
            query_args.push(("scope".to_string(), scope.to_string()));
        }
        if !severity.is_empty() {
            query_args.push(("severity".to_string(), severity.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/dependabot/alerts?{}",
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
     * List Dependabot alerts for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/dependabot/alerts` endpoint.
     *
     * As opposed to `list_alerts_for_repo`, this function returns all the pages of the request at once.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
     *
     * FROM: <https://docs.github.com/rest/dependabot/alerts#list-dependabot-alerts-for-a-repository>
     */
    pub async fn list_all_alerts_for_repo(
        &self,
        owner: &str,
        repo: &str,
        state: &str,
        severity: &str,
        ecosystem: &str,
        package: &str,
        manifest: &str,
        epss_percentage: &str,
        has: &str,
        assignee: &str,
        scope: crate::types::Scope,
        sort: crate::types::Sort,
        direction: crate::types::Order,
        before: &str,
        after: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::DependabotAlert>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !assignee.is_empty() {
            query_args.push(("assignee".to_string(), assignee.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !ecosystem.is_empty() {
            query_args.push(("ecosystem".to_string(), ecosystem.to_string()));
        }
        if !epss_percentage.is_empty() {
            query_args.push(("epss_percentage".to_string(), epss_percentage.to_string()));
        }
        if !has.is_empty() {
            query_args.push(("has".to_string(), has.to_string()));
        }
        if !manifest.is_empty() {
            query_args.push(("manifest".to_string(), manifest.to_string()));
        }
        if !package.is_empty() {
            query_args.push(("package".to_string(), package.to_string()));
        }
        if !scope.to_string().is_empty() {
            query_args.push(("scope".to_string(), scope.to_string()));
        }
        if !severity.is_empty() {
            query_args.push(("severity".to_string(), severity.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/dependabot/alerts?{}",
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
     * Get a Dependabot alert.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/dependabot/alerts/{alert_number}` endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
     *
     * FROM: <https://docs.github.com/rest/dependabot/alerts#get-a-dependabot-alert>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `alert_number: i64` -- The number that identifies a Dependabot alert in its repository.
     *   You can find this at the end of the URL for a Dependabot alert within GitHub,
     *   or in `number` fields in the response from the
     *   `GET /repos/{owner}/{repo}/dependabot/alerts` operation.
     */
    pub async fn get_alert(
        &self,
        owner: &str,
        repo: &str,
        alert_number: i64,
    ) -> ClientResult<crate::Response<crate::types::DependabotAlert>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/dependabot/alerts/{}",
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
     * Update a Dependabot alert.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/dependabot/alerts/{alert_number}` endpoint.
     *
     * The authenticated user must have access to security alerts for the repository to use this endpoint. For more information, see "[Granting access to security alerts](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-security-and-analysis-settings-for-your-repository#granting-access-to-security-alerts)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `security_events` scope to use this endpoint. If this endpoint is only used with public repositories, the token can use the `public_repo` scope instead.
     *
     * FROM: <https://docs.github.com/rest/dependabot/alerts#update-a-dependabot-alert>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `alert_number: i64` -- The number that identifies a Dependabot alert in its repository.
     *   You can find this at the end of the URL for a Dependabot alert within GitHub,
     *   or in `number` fields in the response from the
     *   `GET /repos/{owner}/{repo}/dependabot/alerts` operation.
     */
    pub async fn update_alert(
        &self,
        owner: &str,
        repo: &str,
        alert_number: i64,
        body: &crate::types::DependabotUpdateAlertRequest,
    ) -> ClientResult<crate::Response<crate::types::DependabotAlert>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/dependabot/alerts/{}",
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
     * List repository secrets.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/dependabot/secrets` endpoint.
     *
     * Lists all secrets available in a repository without revealing their encrypted
     * values.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/dependabot/secrets#list-repository-secrets>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_repo_secrets(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::DependabotListRepoSecretsResponse>> {
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
                "/repos/{}/{}/dependabot/secrets?{}",
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
     * Get a repository public key.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/dependabot/secrets/public-key` endpoint.
     *
     * Gets your public key, which you need to encrypt secrets. You need to
     * encrypt a secret before you can create or update secrets. Anyone with read access
     * to the repository can use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint if the repository is private.
     *
     * FROM: <https://docs.github.com/rest/dependabot/secrets#get-a-repository-public-key>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_repo_public_key(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::DependabotPublicKey>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/dependabot/secrets/public-key",
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
     * Get a repository secret.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/dependabot/secrets/{secret_name}` endpoint.
     *
     * Gets a single repository secret without revealing its encrypted value.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/dependabot/secrets#get-a-repository-secret>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `secret_name: &str` -- The name of the secret.
     */
    pub async fn get_repo_secret(
        &self,
        owner: &str,
        repo: &str,
        secret_name: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsSecret>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/dependabot/secrets/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&secret_name.to_string()),
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
     * Create or update a repository secret.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/dependabot/secrets/{secret_name}` endpoint.
     *
     * Creates or updates a repository secret with an encrypted value. Encrypt your secret using
     * [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/dependabot/secrets#create-or-update-a-repository-secret>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `secret_name: &str` -- The name of the secret.
     */
    pub async fn create_or_update_repo_secret(
        &self,
        owner: &str,
        repo: &str,
        secret_name: &str,
        body: &crate::types::CodespacesCreateUpdateRepoSecretRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/dependabot/secrets/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&secret_name.to_string()),
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
     * Delete a repository secret.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/dependabot/secrets/{secret_name}` endpoint.
     *
     * Deletes a secret in a repository using the secret name.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/dependabot/secrets#delete-a-repository-secret>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `secret_name: &str` -- The name of the secret.
     */
    pub async fn delete_repo_secret(
        &self,
        owner: &str,
        repo: &str,
        secret_name: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/dependabot/secrets/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&secret_name.to_string()),
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
