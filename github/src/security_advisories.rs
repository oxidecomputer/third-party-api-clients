use crate::Client;
use crate::ClientResult;

pub struct SecurityAdvisories {
    pub client: Client,
}

impl SecurityAdvisories {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        SecurityAdvisories { client }
    }

    /**
     * List global security advisories.
     *
     * This function performs a `GET` to the `/advisories` endpoint.
     *
     * Lists all global security advisories that match the specified parameters. If no other parameters are defined, the request will return only GitHub-reviewed advisories that are not malware.
     *
     * By default, all responses will exclude advisories for malware, because malware are not standard vulnerabilities. To list advisories for malware, you must include the `type` parameter in your request, with the value `malware`. For more information about the different types of security advisories, see "[About the GitHub Advisory database](https://docs.github.com/code-security/security-advisories/global-security-advisories/about-the-github-advisory-database#about-types-of-security-advisories)."
     *
     * FROM: <https://docs.github.com/rest/security-advisories/global-advisories#list-global-security-advisories>
     *
     * **Parameters:**
     *
     * * `ghsa_id: &str` -- If specified, only advisories with this GHSA (GitHub Security Advisory) identifier will be returned.
     * * `type_: crate::types::Type` -- If specified, only advisories of this type will be returned. By default, a request with no other parameters defined will only return reviewed advisories that are not malware.
     * * `cve_id: &str` -- If specified, only advisories with this CVE (Common Vulnerabilities and Exposures) identifier will be returned.
     * * `ecosystem: crate::types::SecurityAdvisoryEcosystems` -- If specified, only advisories for these ecosystems will be returned.
     * * `severity: crate::types::Severity` -- If specified, only advisories with these severities will be returned.
     * * `cwes: &str` -- If specified, only advisories with these Common Weakness Enumerations (CWEs) will be returned.
     *   
     *   Example: `cwes=79,284,22` or `cwes[]=79&cwes[]=284&cwes[]=22`.
     * * `is_withdrawn: bool` -- Whether to only return advisories that have been withdrawn.
     * * `affects: &str` -- If specified, only return advisories that affect any of `package` or `package@version`. A maximum of 1000 packages can be specified.
     *   If the query parameter causes the URL to exceed the maximum URL length supported by your client, you must specify fewer packages.
     *   
     *   Example: `affects=package1,package2@1.0.0,package3@2.0.0` or `affects[]=package1&affects[]=package2@1.0.0`.
     * * `published: &str` -- If specified, only return advisories that were published on a date or date range.
     *   
     *   For more information on the syntax of the date range, see "[Understanding the search syntax](https://docs.github.com/search-github/getting-started-with-searching-on-github/understanding-the-search-syntax#query-for-dates).".
     * * `updated: &str` -- If specified, only return advisories that were updated on a date or date range.
     *   
     *   For more information on the syntax of the date range, see "[Understanding the search syntax](https://docs.github.com/search-github/getting-started-with-searching-on-github/understanding-the-search-syntax#query-for-dates).".
     * * `modified: &str` -- If specified, only show advisories that were updated or published on a date or date range.
     *   
     *   For more information on the syntax of the date range, see "[Understanding the search syntax](https://docs.github.com/search-github/getting-started-with-searching-on-github/understanding-the-search-syntax#query-for-dates).".
     * * `epss_percentage: &str` -- If specified, only return advisories that have an EPSS percentage score that matches the provided value.
     *   The EPSS percentage represents the likelihood of a CVE being exploited.
     * * `epss_percentile: &str` -- If specified, only return advisories that have an EPSS percentile score that matches the provided value.
     *   The EPSS percentile represents the relative rank of the CVE's likelihood of being exploited compared to other CVEs.
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `sort: crate::types::SecurityAdvisoriesListGlobalSort` -- The property to sort the results by.
     */
    pub async fn list_global_advisories(
        &self,
        ghsa_id: &str,
        type_: crate::types::Type,
        cve_id: &str,
        ecosystem: crate::types::SecurityAdvisoryEcosystems,
        severity: crate::types::Severity,
        cwes: &str,
        is_withdrawn: bool,
        affects: &str,
        published: &str,
        updated: &str,
        modified: &str,
        epss_percentage: &str,
        epss_percentile: &str,
        before: &str,
        after: &str,
        direction: crate::types::Order,
        per_page: i64,
        sort: crate::types::SecurityAdvisoriesListGlobalSort,
    ) -> ClientResult<crate::Response<Vec<crate::types::GlobalAdvisory>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !affects.is_empty() {
            query_args.push(("affects".to_string(), affects.to_string()));
        }
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !cve_id.is_empty() {
            query_args.push(("cve_id".to_string(), cve_id.to_string()));
        }
        if !cwes.is_empty() {
            query_args.push(("cwes".to_string(), cwes.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !ecosystem.to_string().is_empty() {
            query_args.push(("ecosystem".to_string(), ecosystem.to_string()));
        }
        if !epss_percentage.is_empty() {
            query_args.push(("epss_percentage".to_string(), epss_percentage.to_string()));
        }
        if !epss_percentile.is_empty() {
            query_args.push(("epss_percentile".to_string(), epss_percentile.to_string()));
        }
        if !ghsa_id.is_empty() {
            query_args.push(("ghsa_id".to_string(), ghsa_id.to_string()));
        }
        if is_withdrawn {
            query_args.push(("is_withdrawn".to_string(), is_withdrawn.to_string()));
        }
        if !modified.is_empty() {
            query_args.push(("modified".to_string(), modified.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !published.is_empty() {
            query_args.push(("published".to_string(), published.to_string()));
        }
        if !severity.to_string().is_empty() {
            query_args.push(("severity".to_string(), severity.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        if !updated.is_empty() {
            query_args.push(("updated".to_string(), updated.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/advisories?{}", query_), None);
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
     * List global security advisories.
     *
     * This function performs a `GET` to the `/advisories` endpoint.
     *
     * As opposed to `list_global_advisories`, this function returns all the pages of the request at once.
     *
     * Lists all global security advisories that match the specified parameters. If no other parameters are defined, the request will return only GitHub-reviewed advisories that are not malware.
     *
     * By default, all responses will exclude advisories for malware, because malware are not standard vulnerabilities. To list advisories for malware, you must include the `type` parameter in your request, with the value `malware`. For more information about the different types of security advisories, see "[About the GitHub Advisory database](https://docs.github.com/code-security/security-advisories/global-security-advisories/about-the-github-advisory-database#about-types-of-security-advisories)."
     *
     * FROM: <https://docs.github.com/rest/security-advisories/global-advisories#list-global-security-advisories>
     */
    pub async fn list_all_global_advisories(
        &self,
        ghsa_id: &str,
        type_: crate::types::Type,
        cve_id: &str,
        ecosystem: crate::types::SecurityAdvisoryEcosystems,
        severity: crate::types::Severity,
        cwes: &str,
        is_withdrawn: bool,
        affects: &str,
        published: &str,
        updated: &str,
        modified: &str,
        epss_percentage: &str,
        epss_percentile: &str,
        before: &str,
        after: &str,
        direction: crate::types::Order,
        sort: crate::types::SecurityAdvisoriesListGlobalSort,
    ) -> ClientResult<crate::Response<Vec<crate::types::GlobalAdvisory>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !affects.is_empty() {
            query_args.push(("affects".to_string(), affects.to_string()));
        }
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !cve_id.is_empty() {
            query_args.push(("cve_id".to_string(), cve_id.to_string()));
        }
        if !cwes.is_empty() {
            query_args.push(("cwes".to_string(), cwes.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !ecosystem.to_string().is_empty() {
            query_args.push(("ecosystem".to_string(), ecosystem.to_string()));
        }
        if !epss_percentage.is_empty() {
            query_args.push(("epss_percentage".to_string(), epss_percentage.to_string()));
        }
        if !epss_percentile.is_empty() {
            query_args.push(("epss_percentile".to_string(), epss_percentile.to_string()));
        }
        if !ghsa_id.is_empty() {
            query_args.push(("ghsa_id".to_string(), ghsa_id.to_string()));
        }
        if is_withdrawn {
            query_args.push(("is_withdrawn".to_string(), is_withdrawn.to_string()));
        }
        if !modified.is_empty() {
            query_args.push(("modified".to_string(), modified.to_string()));
        }
        if !published.is_empty() {
            query_args.push(("published".to_string(), published.to_string()));
        }
        if !severity.to_string().is_empty() {
            query_args.push(("severity".to_string(), severity.to_string()));
        }
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !type_.to_string().is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        if !updated.is_empty() {
            query_args.push(("updated".to_string(), updated.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/advisories?{}", query_), None);
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
     * Get a global security advisory.
     *
     * This function performs a `GET` to the `/advisories/{ghsa_id}` endpoint.
     *
     * Gets a global security advisory using its GitHub Security Advisory (GHSA) identifier.
     *
     * FROM: <https://docs.github.com/rest/security-advisories/global-advisories#get-a-global-security-advisory>
     *
     * **Parameters:**
     *
     * * `ghsa_id: &str` -- The GHSA (GitHub Security Advisory) identifier of the advisory.
     */
    pub async fn get_global_advisory(
        &self,
        ghsa_id: &str,
    ) -> ClientResult<crate::Response<crate::types::GlobalAdvisory>> {
        let url = self.client.url(
            &format!(
                "/advisories/{}",
                crate::progenitor_support::encode_path(&ghsa_id.to_string()),
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
     * List repository security advisories for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/security-advisories` endpoint.
     *
     * Lists repository security advisories for an organization.
     *
     * The authenticated user must be an owner or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` or `repository_advisories:write` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/security-advisories/repository-advisories#list-repository-security-advisories-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `sort: crate::types::SecurityAdvisoriesListOrgRepositorySort` -- The property to sort the results by.
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of advisories to return per page. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `state: crate::types::SecurityAdvisoriesListOrgRepositoryState` -- Filter by the state of the repository advisories. Only advisories of this state will be returned.
     */
    pub async fn list_org_repository_advisories(
        &self,
        org: &str,
        direction: crate::types::Order,
        sort: crate::types::SecurityAdvisoriesListOrgRepositorySort,
        before: &str,
        after: &str,
        per_page: i64,
        state: crate::types::SecurityAdvisoriesListOrgRepositoryState,
    ) -> ClientResult<crate::Response<Vec<crate::types::RepositoryAdvisory>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
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
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/security-advisories?{}",
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
     * List repository security advisories for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/security-advisories` endpoint.
     *
     * As opposed to `list_org_repository_advisories`, this function returns all the pages of the request at once.
     *
     * Lists repository security advisories for an organization.
     *
     * The authenticated user must be an owner or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` or `repository_advisories:write` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/security-advisories/repository-advisories#list-repository-security-advisories-for-an-organization>
     */
    pub async fn list_all_org_repository_advisories(
        &self,
        org: &str,
        direction: crate::types::Order,
        sort: crate::types::SecurityAdvisoriesListOrgRepositorySort,
        before: &str,
        after: &str,
        state: crate::types::SecurityAdvisoriesListOrgRepositoryState,
    ) -> ClientResult<crate::Response<Vec<crate::types::RepositoryAdvisory>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
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
                "/orgs/{}/security-advisories?{}",
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
     * List repository security advisories.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/security-advisories` endpoint.
     *
     * Lists security advisories in a repository.
     *
     * The authenticated user can access unpublished security advisories from a repository if they are a security manager or administrator of that repository, or if they are a collaborator on any security advisory.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` or `repository_advisories:read` scope to to get a published security advisory in a private repository, or any unpublished security advisory that the authenticated user has access to.
     *
     * FROM: <https://docs.github.com/rest/security-advisories/repository-advisories#list-repository-security-advisories>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     * * `sort: crate::types::SecurityAdvisoriesListOrgRepositorySort` -- The property to sort the results by.
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of advisories to return per page. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `state: crate::types::SecurityAdvisoriesListOrgRepositoryState` -- Filter by the state of the repository advisories. Only advisories of this state will be returned.
     */
    pub async fn list_repository_advisories(
        &self,
        owner: &str,
        repo: &str,
        direction: crate::types::Order,
        sort: crate::types::SecurityAdvisoriesListOrgRepositorySort,
        before: &str,
        after: &str,
        per_page: i64,
        state: crate::types::SecurityAdvisoriesListOrgRepositoryState,
    ) -> ClientResult<crate::Response<Vec<crate::types::RepositoryAdvisory>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
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
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/security-advisories?{}",
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
     * List repository security advisories.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/security-advisories` endpoint.
     *
     * As opposed to `list_repository_advisories`, this function returns all the pages of the request at once.
     *
     * Lists security advisories in a repository.
     *
     * The authenticated user can access unpublished security advisories from a repository if they are a security manager or administrator of that repository, or if they are a collaborator on any security advisory.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` or `repository_advisories:read` scope to to get a published security advisory in a private repository, or any unpublished security advisory that the authenticated user has access to.
     *
     * FROM: <https://docs.github.com/rest/security-advisories/repository-advisories#list-repository-security-advisories>
     */
    pub async fn list_all_repository_advisories(
        &self,
        owner: &str,
        repo: &str,
        direction: crate::types::Order,
        sort: crate::types::SecurityAdvisoriesListOrgRepositorySort,
        before: &str,
        after: &str,
        state: crate::types::SecurityAdvisoriesListOrgRepositoryState,
    ) -> ClientResult<crate::Response<Vec<crate::types::RepositoryAdvisory>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
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
                "/repos/{}/{}/security-advisories?{}",
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
     * Create a repository security advisory.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/security-advisories` endpoint.
     *
     * Creates a new repository security advisory.
     *
     * In order to create a draft repository security advisory, the authenticated user must be a security manager or administrator of that repository.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` or `repository_advisories:write` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/security-advisories/repository-advisories#create-a-repository-security-advisory>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_repository_advisory(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::RepositoryAdvisoryCreate,
    ) -> ClientResult<crate::Response<crate::types::RepositoryAdvisory>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/security-advisories",
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
     * Privately report a security vulnerability.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/security-advisories/reports` endpoint.
     *
     * Report a security vulnerability to the maintainers of the repository.
     * See "[Privately reporting a security vulnerability](https://docs.github.com/code-security/security-advisories/guidance-on-reporting-and-writing/privately-reporting-a-security-vulnerability)" for more information about private vulnerability reporting.
     *
     * FROM: <https://docs.github.com/rest/security-advisories/repository-advisories#privately-report-a-security-vulnerability>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_private_vulnerability_report(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::PrivateVulnerabilityReportCreate,
    ) -> ClientResult<crate::Response<crate::types::RepositoryAdvisory>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/security-advisories/reports",
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
     * Get a repository security advisory.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/security-advisories/{ghsa_id}` endpoint.
     *
     * Get a repository security advisory using its GitHub Security Advisory (GHSA) identifier.
     *
     * Anyone can access any published security advisory on a public repository.
     *
     * The authenticated user can access an unpublished security advisory from a repository if they are a security manager or administrator of that repository, or if they are a
     * collaborator on the security advisory.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` or `repository_advisories:read` scope to to get a published security advisory in a private repository, or any unpublished security advisory that the authenticated user has access to.
     *
     * FROM: <https://docs.github.com/rest/security-advisories/repository-advisories#get-a-repository-security-advisory>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `ghsa_id: &str` -- The GHSA (GitHub Security Advisory) identifier of the advisory.
     */
    pub async fn get_repository_advisory(
        &self,
        owner: &str,
        repo: &str,
        ghsa_id: &str,
    ) -> ClientResult<crate::Response<crate::types::RepositoryAdvisory>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/security-advisories/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&ghsa_id.to_string()),
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
     * Update a repository security advisory.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/security-advisories/{ghsa_id}` endpoint.
     *
     * Update a repository security advisory using its GitHub Security Advisory (GHSA) identifier.
     *
     * In order to update any security advisory, the authenticated user must be a security manager or administrator of that repository,
     * or a collaborator on the repository security advisory.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` or `repository_advisories:write` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/security-advisories/repository-advisories#update-a-repository-security-advisory>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `ghsa_id: &str` -- The GHSA (GitHub Security Advisory) identifier of the advisory.
     */
    pub async fn update_repository_advisory(
        &self,
        owner: &str,
        repo: &str,
        ghsa_id: &str,
        body: &crate::types::RepositoryAdvisoryUpdate,
    ) -> ClientResult<crate::Response<crate::types::RepositoryAdvisory>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/security-advisories/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&ghsa_id.to_string()),
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
     * Request a CVE for a repository security advisory.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/security-advisories/{ghsa_id}/cve` endpoint.
     *
     * If you want a CVE identification number for the security vulnerability in your project, and don't already have one, you can request a CVE identification number from GitHub. For more information see "[Requesting a CVE identification number](https://docs.github.com/code-security/security-advisories/repository-security-advisories/publishing-a-repository-security-advisory#requesting-a-cve-identification-number-optional)."
     *
     * You may request a CVE for public repositories, but cannot do so for private repositories.
     *
     * In order to request a CVE for a repository security advisory, the authenticated user must be a security manager or administrator of that repository.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` or `repository_advisories:write` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/security-advisories/repository-advisories#request-a-cve-for-a-repository-security-advisory>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `ghsa_id: &str` -- The GHSA (GitHub Security Advisory) identifier of the advisory.
     */
    pub async fn create_repository_advisory_cve_request(
        &self,
        owner: &str,
        repo: &str,
        ghsa_id: &str,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/security-advisories/{}/cve",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&ghsa_id.to_string()),
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
     * Create a temporary private fork.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/security-advisories/{ghsa_id}/forks` endpoint.
     *
     * Create a temporary private fork to collaborate on fixing a security vulnerability in your repository.
     *
     * > [!NOTE]
     * > Forking a repository happens asynchronously. You may have to wait up to 5 minutes before you can access the fork.
     *
     * FROM: <https://docs.github.com/rest/security-advisories/repository-advisories#create-a-temporary-private-fork>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `ghsa_id: &str` -- The GHSA (GitHub Security Advisory) identifier of the advisory.
     */
    pub async fn create_fork(
        &self,
        owner: &str,
        repo: &str,
        ghsa_id: &str,
    ) -> ClientResult<crate::Response<crate::types::FullRepository>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/security-advisories/{}/forks",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&ghsa_id.to_string()),
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
}
