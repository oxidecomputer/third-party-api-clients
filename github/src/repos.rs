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
     * FROM: <https://docs.github.com/rest/reference/repos#list-organization-repositories>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `type_: crate::types::ReposListOrgType` -- Specifies the types of repositories you want returned. Can be one of `all`, `public`, `private`, `forks`, `sources`, `member`, `internal`. Note: For GitHub AE, can be one of `all`, `private`, `forks`, `sources`, `member`, `internal`. Default: `all`. If your organization is associated with an enterprise account using GitHub Enterprise Cloud or GitHub Enterprise Server 2.20+, `type` can also be `internal`. However, the `internal` value is not yet supported when a GitHub App calls this API with an installation access token.
     * * `sort: crate::types::ReposListOrgSort` -- Can be one of `created`, `updated`, `pushed`, `full_name`.
     * * `direction: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_org(
        &self,
        org: &str,
        type_: crate::types::ReposListOrgType,
        sort: crate::types::ReposListOrgSort,
        direction: crate::types::Order,
        per_page: i64,
        page: i64,
    ) -> ClientResult<Vec<crate::types::MinimalRepository>> {
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
                crate::progenitor_support::encode_path(org),
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
     * FROM: <https://docs.github.com/rest/reference/repos#list-organization-repositories>
     */
    pub async fn list_all_for_org(
        &self,
        org: &str,
        type_: crate::types::ReposListOrgType,
        sort: crate::types::ReposListOrgSort,
        direction: crate::types::Order,
    ) -> ClientResult<Vec<crate::types::MinimalRepository>> {
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
                crate::progenitor_support::encode_path(org),
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
     * **OAuth scope requirements**
     *
     * When using [OAuth](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/), authorizations must include:
     *
     * *   `public_repo` scope or `repo` scope to create a public repository. Note: For GitHub AE, use `repo` scope to create an internal repository.
     * *   `repo` scope to create a private repository
     *
     * FROM: <https://docs.github.com/rest/reference/repos#create-an-organization-repository>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn create_in_org(
        &self,
        org: &str,
        body: &crate::types::ReposCreateInOrgRequest,
    ) -> ClientResult<crate::types::Repository> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/repos",
                crate::progenitor_support::encode_path(org),
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
     * Get a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}` endpoint.
     *
     * When you pass the `scarlet-witch-preview` media type, requests to get a repository will also return the repository's code of conduct if it can be detected from the repository's code of conduct file.
     *
     * The `parent` and `source` objects are present when the repository is a fork. `parent` is the repository this repository was forked from, `source` is the ultimate source for the network.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get(&self, owner: &str, repo: &str) -> ClientResult<crate::types::FullRepository> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Deleting a repository requires admin access. If OAuth is used, the `delete_repo` scope is required.
     *
     * If an organization owner has configured the organization to prevent members from deleting organization-owned
     * repositories, you will get a `403 Forbidden` response.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#delete-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn delete(&self, owner: &str, repo: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * **Note**: To edit a repository's topics, use the [Replace all repository topics](https://docs.github.com/rest/reference/repos#replace-all-repository-topics) endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/repos/#update-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn update(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposUpdateRequest,
    ) -> ClientResult<crate::types::FullRepository> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}",
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
     * List all autolinks of a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/autolinks` endpoint.
     *
     * This returns a list of autolinks configured for the given repository.
     *
     * Information about autolinks are only available to repository administrators.
     *
     * FROM: <https://docs.github.com/v3/repos#list-autolinks>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_autolinks(
        &self,
        owner: &str,
        repo: &str,
        page: i64,
    ) -> ClientResult<Vec<crate::types::Autolink>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/autolinks?{}",
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
     * List all autolinks of a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/autolinks` endpoint.
     *
     * As opposed to `list_autolinks`, this function returns all the pages of the request at once.
     *
     * This returns a list of autolinks configured for the given repository.
     *
     * Information about autolinks are only available to repository administrators.
     *
     * FROM: <https://docs.github.com/v3/repos#list-autolinks>
     */
    pub async fn list_all_autolinks(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<Vec<crate::types::Autolink>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/autolinks",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/v3/repos#create-an-autolink>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn create_autolink(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposCreateAutolinkRequest,
    ) -> ClientResult<crate::types::Autolink> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/autolinks",
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
     * Get an autolink reference of a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/autolinks/{autolink_id}` endpoint.
     *
     * This returns a single autolink reference by ID that was configured for the given repository.
     *
     * Information about autolinks are only available to repository administrators.
     *
     * FROM: <https://docs.github.com/v3/repos#get-autolink>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `autolink_id: i64` -- autolink_id parameter.
     */
    pub async fn get_autolink(
        &self,
        owner: &str,
        repo: &str,
        autolink_id: i64,
    ) -> ClientResult<crate::types::Autolink> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/autolinks/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/v3/repos#delete-autolink>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `autolink_id: i64` -- autolink_id parameter.
     */
    pub async fn delete_autolink(
        &self,
        owner: &str,
        repo: &str,
        autolink_id: i64,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/autolinks/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Enable automated security fixes.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/automated-security-fixes` endpoint.
     *
     * Enables automated security fixes for a repository. The authenticated user must have admin access to the repository. For more information, see "[Configuring automated security fixes](https://help.github.com/en/articles/configuring-automated-security-fixes)".
     *
     * FROM: <https://docs.github.com/rest/reference/repos#enable-automated-security-fixes>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn enable_automated_security_fixes(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/automated-security-fixes",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Disable automated security fixes.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/automated-security-fixes` endpoint.
     *
     * Disables automated security fixes for a repository. The authenticated user must have admin access to the repository. For more information, see "[Configuring automated security fixes](https://help.github.com/en/articles/configuring-automated-security-fixes)".
     *
     * FROM: <https://docs.github.com/rest/reference/repos#disable-automated-security-fixes>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn disable_automated_security_fixes(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/automated-security-fixes",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#list-branches>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `protected: bool` -- Setting to `true` returns only protected branches. When set to `false`, only unprotected branches are returned. Omitting this parameter returns all branches.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_branches(
        &self,
        owner: &str,
        repo: &str,
        protected: bool,
        per_page: i64,
        page: i64,
    ) -> ClientResult<Vec<crate::types::ShortBranch>> {
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
     * List branches.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/branches` endpoint.
     *
     * As opposed to `list_branches`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-branches>
     */
    pub async fn list_all_branches(
        &self,
        owner: &str,
        repo: &str,
        protected: bool,
    ) -> ClientResult<Vec<crate::types::ShortBranch>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if protected {
            query_args.push(("protected".to_string(), protected.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches?{}",
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
     * Get a branch.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/branches/{branch}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-a-branch>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn get_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::types::BranchWithProtection> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-branch-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn get_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::types::BranchProtection> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Protecting a branch requires admin or owner permissions to the repository.
     *
     * **Note**: Passing new arrays of `users` and `teams` replaces their previous values.
     *
     * **Note**: The list of users, apps, and teams in total is limited to 100 items.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#update-branch-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn update_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposUpdateBranchProtectionRequest,
    ) -> ClientResult<crate::types::ProtectedBranch> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#delete-branch-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn delete_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-admin-branch-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn get_admin_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::types::EnforceAdmins> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/enforce_admins",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Adding admin enforcement requires admin or owner permissions to the repository and branch protection to be enabled.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#set-admin-branch-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn set_admin_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::types::EnforceAdmins> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/enforce_admins",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Removing admin enforcement requires admin or owner permissions to the repository and branch protection to be enabled.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#delete-admin-branch-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn delete_admin_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/enforce_admins",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-pull-request-review-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn get_pull_request_review_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::types::ProtectedBranchPullRequestReview> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_pull_request_reviews",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#delete-pull-request-review-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn delete_pull_request_review_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_pull_request_reviews",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Updating pull request review enforcement requires admin or owner permissions to the repository and branch protection to be enabled.
     *
     * **Note**: Passing new arrays of `users` and `teams` replaces their previous values.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#update-pull-request-review-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn update_pull_request_review_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposUpdatePullRequestReviewProtection,
    ) -> ClientResult<crate::types::ProtectedBranchPullRequestReview> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_pull_request_reviews",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * When authenticated with admin or owner permissions to the repository, you can use this endpoint to check whether a branch requires signed commits. An enabled status of `true` indicates you must sign commits on this branch. For more information, see [Signing commits with GPG](https://help.github.com/articles/signing-commits-with-gpg) in GitHub Help.
     *
     * **Note**: You must enable branch protection to require signed commits.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-commit-signature-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn get_commit_signature_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::types::EnforceAdmins> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_signatures",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * When authenticated with admin or owner permissions to the repository, you can use this endpoint to require signed commits on a branch. You must enable branch protection to require signed commits.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#create-commit-signature-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn create_commit_signature_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::types::EnforceAdmins> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_signatures",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * When authenticated with admin or owner permissions to the repository, you can use this endpoint to disable required signed commits on a branch. You must enable branch protection to require signed commits.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#delete-commit-signature-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn delete_commit_signature_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_signatures",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-status-checks-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn get_status_checks_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::types::StatusCheckPolicy> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_status_checks",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#remove-status-check-protection>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn remove_status_check_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_status_checks",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Updating required status checks requires admin or owner permissions to the repository and branch protection to be enabled.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#update-status-check-potection>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn update_status_check_protection(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposUpdateStatusCheckProtectionRequest,
    ) -> ClientResult<crate::types::StatusCheckPolicy> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_status_checks",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-all-status-check-contexts>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn get_all_status_check_contexts(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<Vec<String>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_status_checks/contexts",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-all-status-check-contexts>
     */
    pub async fn get_all_all_status_check_contexts(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<Vec<String>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_status_checks/contexts",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#set-status-check-contexts>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn set_status_check_contexts(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddStatusCheckContextsRequestOneOf,
    ) -> ClientResult<Vec<String>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_status_checks/contexts",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#add-status-check-contexts>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn add_status_check_contexts(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddStatusCheckContextsRequestOneOf,
    ) -> ClientResult<Vec<String>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_status_checks/contexts",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#remove-status-check-contexts>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn remove_status_check_contexts(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddStatusCheckContextsRequestOneOf,
    ) -> ClientResult<Vec<String>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/required_status_checks/contexts",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Lists who has access to this protected branch.
     *
     * **Note**: Users, apps, and teams `restrictions` are only available for organization-owned repositories.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn get_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<crate::types::BranchRestrictionPolicy> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Disables the ability to restrict who can push to this branch.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#delete-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn delete_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Lists the GitHub Apps that have push access to this branch. Only installed GitHub Apps with `write` access to the `contents` permission can be added as authorized actors on a protected branch.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-apps-with-access-to-the-protected-branch>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn get_apps_with_access_to_protected_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<Vec<crate::types::GitHubApp>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/apps",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Lists the GitHub Apps that have push access to this branch. Only installed GitHub Apps with `write` access to the `contents` permission can be added as authorized actors on a protected branch.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-apps-with-access-to-the-protected-branch>
     */
    pub async fn get_all_apps_with_access_to_protected_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<Vec<crate::types::GitHubApp>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/apps",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Replaces the list of apps that have push access to this branch. This removes all apps that previously had push access and grants push access to the new list of apps. Only installed GitHub Apps with `write` access to the `contents` permission can be added as authorized actors on a protected branch.
     *
     * | Type    | Description                                                                                                                                                |
     * | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
     * | `array` | The GitHub Apps that have push access to this branch. Use the app's `slug`. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
     *
     * FROM: <https://docs.github.com/rest/reference/repos#set-app-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn set_app_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddAppAccessRestrictionsRequestOneOf,
    ) -> ClientResult<Vec<crate::types::GitHubApp>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/apps",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Grants the specified apps push access for this branch. Only installed GitHub Apps with `write` access to the `contents` permission can be added as authorized actors on a protected branch.
     *
     * | Type    | Description                                                                                                                                                |
     * | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
     * | `array` | The GitHub Apps that have push access to this branch. Use the app's `slug`. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
     *
     * FROM: <https://docs.github.com/rest/reference/repos#add-app-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn add_app_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddAppAccessRestrictionsRequestOneOf,
    ) -> ClientResult<Vec<crate::types::GitHubApp>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/apps",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Removes the ability of an app to push to this branch. Only installed GitHub Apps with `write` access to the `contents` permission can be added as authorized actors on a protected branch.
     *
     * | Type    | Description                                                                                                                                                |
     * | ------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
     * | `array` | The GitHub Apps that have push access to this branch. Use the app's `slug`. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
     *
     * FROM: <https://docs.github.com/rest/reference/repos#remove-app-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn remove_app_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddAppAccessRestrictionsRequestOneOf,
    ) -> ClientResult<Vec<crate::types::GitHubApp>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/apps",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Lists the teams who have push access to this branch. The list includes child teams.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-teams-with-access-to-the-protected-branch>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn get_teams_with_access_to_protected_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<Vec<crate::types::Team>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/teams",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Lists the teams who have push access to this branch. The list includes child teams.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-teams-with-access-to-the-protected-branch>
     */
    pub async fn get_all_teams_with_access_to_protected_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<Vec<crate::types::Team>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/teams",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Replaces the list of teams that have push access to this branch. This removes all teams that previously had push access and grants push access to the new list of teams. Team restrictions include child teams.
     *
     * | Type    | Description                                                                                                                                |
     * | ------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
     * | `array` | The teams that can have push access. Use the team's `slug`. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
     *
     * FROM: <https://docs.github.com/rest/reference/repos#set-team-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn set_team_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddTeamAccessRestrictionsRequestOneOf,
    ) -> ClientResult<Vec<crate::types::Team>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/teams",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Grants the specified teams push access for this branch. You can also give push access to child teams.
     *
     * | Type    | Description                                                                                                                                |
     * | ------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
     * | `array` | The teams that can have push access. Use the team's `slug`. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
     *
     * FROM: <https://docs.github.com/rest/reference/repos#add-team-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn add_team_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddTeamAccessRestrictionsRequestOneOf,
    ) -> ClientResult<Vec<crate::types::Team>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/teams",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Removes the ability of a team to push to this branch. You can also remove push access for child teams.
     *
     * | Type    | Description                                                                                                                                         |
     * | ------- | --------------------------------------------------------------------------------------------------------------------------------------------------- |
     * | `array` | Teams that should no longer have push access. Use the team's `slug`. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
     *
     * FROM: <https://docs.github.com/rest/reference/repos#remove-team-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn remove_team_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddTeamAccessRestrictionsRequestOneOf,
    ) -> ClientResult<Vec<crate::types::Team>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/teams",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Lists the people who have push access to this branch.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-users-with-access-to-the-protected-branch>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn get_users_with_access_to_protected_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<Vec<crate::types::SimpleUser>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/users",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Lists the people who have push access to this branch.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-users-with-access-to-the-protected-branch>
     */
    pub async fn get_all_users_with_access_to_protected_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> ClientResult<Vec<crate::types::SimpleUser>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/users",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Replaces the list of people that have push access to this branch. This removes all people that previously had push access and grants push access to the new list of people.
     *
     * | Type    | Description                                                                                                                   |
     * | ------- | ----------------------------------------------------------------------------------------------------------------------------- |
     * | `array` | Usernames for people who can have push access. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
     *
     * FROM: <https://docs.github.com/rest/reference/repos#set-user-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn set_user_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddUserAccessRestrictionsRequestOneOf,
    ) -> ClientResult<Vec<crate::types::SimpleUser>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/users",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Grants the specified people push access for this branch.
     *
     * | Type    | Description                                                                                                                   |
     * | ------- | ----------------------------------------------------------------------------------------------------------------------------- |
     * | `array` | Usernames for people who can have push access. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
     *
     * FROM: <https://docs.github.com/rest/reference/repos#add-user-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn add_user_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddUserAccessRestrictionsRequestOneOf,
    ) -> ClientResult<Vec<crate::types::SimpleUser>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/users",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Removes the ability of a user to push to this branch.
     *
     * | Type    | Description                                                                                                                                   |
     * | ------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
     * | `array` | Usernames of the people who should no longer have push access. **Note**: The list of users, apps, and teams in total is limited to 100 items. |
     *
     * FROM: <https://docs.github.com/rest/reference/repos#remove-user-access-restrictions>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn remove_user_access_restrictions(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposAddUserAccessRestrictionsRequestOneOf,
    ) -> ClientResult<Vec<crate::types::SimpleUser>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/protection/restrictions/users",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * **Note:** Although the API responds immediately, the branch rename process might take some extra time to complete in the background. You won't be able to push to the old branch name while the rename process is in progress. For more information, see "[Renaming a branch](https://docs.github.com/github/administering-a-repository/renaming-a-branch)".
     *
     * The permissions required to use this endpoint depends on whether you are renaming the default branch.
     *
     * To rename a non-default branch:
     *
     * * Users must have push access.
     * * GitHub Apps must have the `contents:write` repository permission.
     *
     * To rename the default branch:
     *
     * * Users must have admin or owner permissions.
     * * GitHub Apps must have the `administration:write` repository permission.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#rename-a-branch>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `branch: &str` -- The name of the branch.
     */
    pub async fn rename_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        body: &crate::types::ReposRenameBranchRequest,
    ) -> ClientResult<crate::types::BranchWithProtection> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/branches/{}/rename",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(branch),
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
     * List repository collaborators.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/collaborators` endpoint.
     *
     * For organization-owned repositories, the list of collaborators includes outside collaborators, organization members that are direct collaborators, organization members with access through team memberships, organization members with access through default organization permissions, and organization owners.
     *
     * Team members will include the members of child teams.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-repository-collaborators>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `affiliation: crate::types::Affiliation` -- Filters the collaborators by their affiliation. Can be one of:  
     *  \\* `outside`: Outside collaborators of a project that are not a member of the project's organization.  
     *  \\* `direct`: Collaborators with permissions to a project, regardless of organization membership status.  
     *  \\* `all`: All collaborators the authenticated user can see.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_collaborators(
        &self,
        owner: &str,
        repo: &str,
        affiliation: crate::types::Affiliation,
        per_page: i64,
        page: i64,
    ) -> ClientResult<Vec<crate::types::Collaborator>> {
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
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/collaborators?{}",
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
     * List repository collaborators.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/collaborators` endpoint.
     *
     * As opposed to `list_collaborators`, this function returns all the pages of the request at once.
     *
     * For organization-owned repositories, the list of collaborators includes outside collaborators, organization members that are direct collaborators, organization members with access through team memberships, organization members with access through default organization permissions, and organization owners.
     *
     * Team members will include the members of child teams.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-repository-collaborators>
     */
    pub async fn list_all_collaborators(
        &self,
        owner: &str,
        repo: &str,
        affiliation: crate::types::Affiliation,
    ) -> ClientResult<Vec<crate::types::Collaborator>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !affiliation.to_string().is_empty() {
            query_args.push(("affiliation".to_string(), affiliation.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/collaborators?{}",
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
     * Check if a user is a repository collaborator.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/collaborators/{username}` endpoint.
     *
     * For organization-owned repositories, the list of collaborators includes outside collaborators, organization members that are direct collaborators, organization members with access through team memberships, organization members with access through default organization permissions, and organization owners.
     *
     * Team members will include the members of child teams.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#check-if-a-user-is-a-repository-collaborator>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `username: &str`
     */
    pub async fn check_collaborator(
        &self,
        owner: &str,
        repo: &str,
        username: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/collaborators/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(username),
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
     * This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
     *
     * For more information the permission levels, see "[Repository permission levels for an organization](https://help.github.com/en/github/setting-up-and-managing-organizations-and-teams/repository-permission-levels-for-an-organization#permission-levels-for-repositories-owned-by-an-organization)".
     *
     * Note that, if you choose not to pass any parameters, you'll need to set `Content-Length` to zero when calling out to this endpoint. For more information, see "[HTTP verbs](https://docs.github.com/rest/overview/resources-in-the-rest-api#http-verbs)."
     *
     * The invitee will receive a notification that they have been invited to the repository, which they must accept or decline. They may do this via the notifications page, the email they receive, or by using the [repository invitations API endpoints](https://docs.github.com/rest/reference/repos#invitations).
     *
     * **Rate limits**
     *
     * You are limited to sending 50 invitations to a repository per 24 hour period. Note there is no limit if you are inviting organization members to an organization repository.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#add-a-repository-collaborator>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `username: &str`
     */
    pub async fn add_collaborator(
        &self,
        owner: &str,
        repo: &str,
        username: &str,
        body: &crate::types::ReposAddCollaboratorRequest,
    ) -> ClientResult<crate::types::RepositoryInvitation> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/collaborators/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(username),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#remove-a-repository-collaborator>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `username: &str`
     */
    pub async fn remove_collaborator(
        &self,
        owner: &str,
        repo: &str,
        username: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/collaborators/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(username),
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
     * Checks the repository permission of a collaborator. The possible repository permissions are `admin`, `write`, `read`, and `none`.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-repository-permissions-for-a-user>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `username: &str`
     */
    pub async fn get_collaborator_permission_level(
        &self,
        owner: &str,
        repo: &str,
        username: &str,
    ) -> ClientResult<crate::types::RepositoryCollaboratorPermission> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/collaborators/{}/permission",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(username),
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
     * Commit Comments use [these custom media types](https://docs.github.com/rest/reference/repos#custom-media-types). You can read more about the use of media types in the API [here](https://docs.github.com/rest/overview/media-types/).
     *
     * Comments are ordered by ascending ID.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-commit-comments-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_commit_comments_for_repo(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<Vec<crate::types::CommitComment>> {
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
     * List commit comments for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/comments` endpoint.
     *
     * As opposed to `list_commit_comments_for_repo`, this function returns all the pages of the request at once.
     *
     * Commit Comments use [these custom media types](https://docs.github.com/rest/reference/repos#custom-media-types). You can read more about the use of media types in the API [here](https://docs.github.com/rest/overview/media-types/).
     *
     * Comments are ordered by ascending ID.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-commit-comments-for-a-repository>
     */
    pub async fn list_all_commit_comments_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<Vec<crate::types::CommitComment>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/comments",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-a-commit-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `comment_id: i64` -- comment_id parameter.
     */
    pub async fn get_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> ClientResult<crate::types::CommitComment> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/comments/{}",
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
     * Delete a commit comment.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/comments/{comment_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#delete-a-commit-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `comment_id: i64` -- comment_id parameter.
     */
    pub async fn delete_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/comments/{}",
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
     * Update a commit comment.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/comments/{comment_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#update-a-commit-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `comment_id: i64` -- comment_id parameter.
     */
    pub async fn update_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        body: &crate::types::PullsUpdateReviewRequest,
    ) -> ClientResult<crate::types::CommitComment> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/comments/{}",
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
     * | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on her/his account. |
     * | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
     * | `unknown_key` | The key that made the signature has not been registered with any user's account. |
     * | `malformed_signature` | There was an error parsing the signature. |
     * | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
     * | `valid` | None of the above errors applied, so the signature is considered to be verified. |
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-commits>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `sha: &str` -- SHA or branch to start listing commits from. Default: the repository’s default branch (usually `master`).
     * * `path: &str` -- Only commits containing this file path will be returned.
     * * `author: &str` -- GitHub login or email address by which to filter by commit author.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `until: chrono::DateTime<chrono::Utc>` -- Only commits before this date will be returned. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_commits(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        path: &str,
        author: &str,
        since: Option<chrono::DateTime<chrono::Utc>>,
        until: Option<chrono::DateTime<chrono::Utc>>,
        per_page: i64,
        page: i64,
    ) -> ClientResult<Vec<crate::types::CommitDataType>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !author.is_empty() {
            query_args.push(("author".to_string(), author.to_string()));
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
     * | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on her/his account. |
     * | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
     * | `unknown_key` | The key that made the signature has not been registered with any user's account. |
     * | `malformed_signature` | There was an error parsing the signature. |
     * | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
     * | `valid` | None of the above errors applied, so the signature is considered to be verified. |
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-commits>
     */
    pub async fn list_all_commits(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        path: &str,
        author: &str,
        since: Option<chrono::DateTime<chrono::Utc>>,
        until: Option<chrono::DateTime<chrono::Utc>>,
    ) -> ClientResult<Vec<crate::types::CommitDataType>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !author.is_empty() {
            query_args.push(("author".to_string(), author.to_string()));
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
     * List branches for HEAD commit.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/commits/{commit_sha}/branches-where-head` endpoint.
     *
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Returns all branches where the given commit SHA is the HEAD, or latest commit for the branch.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-branches-for-head-commit>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `commit_sha: &str` -- commit_sha parameter.
     */
    pub async fn list_branches_for_head_commit(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
    ) -> ClientResult<Vec<crate::types::BranchShort>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/commits/{}/branches-where-head",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(commit_sha),
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
     * Protected branches are available in public repositories with GitHub Free and GitHub Free for organizations, and in public and private repositories with GitHub Pro, GitHub Team, GitHub Enterprise Cloud, and GitHub Enterprise Server. For more information, see [GitHub's products](https://help.github.com/github/getting-started-with-github/githubs-products) in the GitHub Help documentation.
     *
     * Returns all branches where the given commit SHA is the HEAD, or latest commit for the branch.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-branches-for-head-commit>
     */
    pub async fn list_all_branches_for_head_commit(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
    ) -> ClientResult<Vec<crate::types::BranchShort>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/commits/{}/branches-where-head",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(commit_sha),
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
     * Use the `:commit_sha` to specify the commit that will have its comments listed.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-commit-comments>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `commit_sha: &str` -- commit_sha parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_comments_for_commit(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<Vec<crate::types::CommitComment>> {
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(commit_sha),
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
     * Use the `:commit_sha` to specify the commit that will have its comments listed.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-commit-comments>
     */
    pub async fn list_all_comments_for_commit(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
    ) -> ClientResult<Vec<crate::types::CommitComment>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/commits/{}/comments",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(commit_sha),
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
     * This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#create-a-commit-comment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `commit_sha: &str` -- commit_sha parameter.
     */
    pub async fn create_commit_comment(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
        body: &crate::types::ReposCreateCommitCommentRequest,
    ) -> ClientResult<crate::types::CommitComment> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/commits/{}/comments",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(commit_sha),
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
     * Lists the merged pull request that introduced the commit to the repository. If the commit is not present in the default branch, additionally returns open pull requests associated with the commit. The results may include open and closed pull requests. Additional preview headers may be required to see certain details for associated pull requests, such as whether a pull request is in a draft state. For more information about previews that might affect this endpoint, see the [List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests) endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-pull-requests-associated-with-a-commit>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `commit_sha: &str` -- commit_sha parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_pull_requests_associated_with_commit(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<Vec<crate::types::PullRequestSimple>> {
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(commit_sha),
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
     * Lists the merged pull request that introduced the commit to the repository. If the commit is not present in the default branch, additionally returns open pull requests associated with the commit. The results may include open and closed pull requests. Additional preview headers may be required to see certain details for associated pull requests, such as whether a pull request is in a draft state. For more information about previews that might affect this endpoint, see the [List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests) endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-pull-requests-associated-with-a-commit>
     */
    pub async fn list_all_pull_requests_associated_with_commit(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
    ) -> ClientResult<Vec<crate::types::PullRequestSimple>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/commits/{}/pulls",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(commit_sha),
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
     * **Note:** If there are more than 300 files in the commit diff, the response will include pagination link headers for the remaining files, up to a limit of 3000 files. Each page contains the static commit information, and the only changes are to the file listing.
     *
     * You can pass the appropriate [media type](https://docs.github.com/rest/overview/media-types/#commits-commit-comparison-and-pull-requests) to  fetch `diff` and `patch` formats. Diffs with binary data will have no `patch` property.
     *
     * To return only the SHA-1 hash of the commit reference, you can provide the `sha` custom [media type](https://docs.github.com/rest/overview/media-types/#commits-commit-comparison-and-pull-requests) in the `Accept` header. You can use this endpoint to check if a remote reference's SHA-1 hash is the same as your local reference's SHA-1 hash by providing the local SHA-1 reference as the ETag.
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
     * | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on her/his account. |
     * | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
     * | `unknown_key` | The key that made the signature has not been registered with any user's account. |
     * | `malformed_signature` | There was an error parsing the signature. |
     * | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
     * | `valid` | None of the above errors applied, so the signature is considered to be verified. |
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-a-commit>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `page: i64` -- Page number of the results to fetch.
     * * `per_page: i64` -- Results per page (max 100).
     * * `ref_: &str` -- ref parameter.
     */
    pub async fn get_commit(
        &self,
        owner: &str,
        repo: &str,
        page: i64,
        per_page: i64,
        ref_: &str,
    ) -> ClientResult<crate::types::CommitDataType> {
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
     * Get the combined status for a specific reference.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/commits/{ref}/status` endpoint.
     *
     * Users with pull access in a repository can access a combined view of commit statuses for a given ref. The ref can be a SHA, a branch name, or a tag name.
     *
     * The most recent status for each context is returned, up to 100. This field [paginates](https://docs.github.com/rest/overview/resources-in-the-rest-api#pagination) if there are over 100 contexts.
     *
     * Additionally, a combined `state` is returned. The `state` is one of:
     *
     * *   **failure** if any of the contexts report as `error` or `failure`
     * *   **pending** if there are no statuses or a context is `pending`
     * *   **success** if the latest status for all contexts is `success`
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-the-combined-status-for-a-specific-reference>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `ref_: &str` -- ref parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn get_combined_status_for_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::types::CombinedCommitStatus> {
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
     * List commit statuses for a reference.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/commits/{ref}/statuses` endpoint.
     *
     * Users with pull access in a repository can view commit statuses for a given ref. The ref can be a SHA, a branch name, or a tag name. Statuses are returned in reverse chronological order. The first status in the list will be the latest one.
     *
     * This resource is also available via a legacy route: `GET /repos/:owner/:repo/statuses/:ref`.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-commit-statuses-for-a-reference>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `ref_: &str` -- ref parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_commit_statuses_for_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<Vec<crate::types::StatusData>> {
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
     * FROM: <https://docs.github.com/rest/reference/repos#list-commit-statuses-for-a-reference>
     */
    pub async fn list_all_commit_statuses_for_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> ClientResult<Vec<crate::types::StatusData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/commits/{}/statuses",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(ref_),
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
     * This endpoint will return all community profile metrics, including an
     * overall health score, repository description, the presence of documentation, detected
     * code of conduct, detected license, and the presence of ISSUE\_TEMPLATE, PULL\_REQUEST\_TEMPLATE,
     * README, and CONTRIBUTING files.
     *
     * The `health_percentage` score is defined as a percentage of how many of
     * these four documents are present: README, CONTRIBUTING, LICENSE, and
     * CODE_OF_CONDUCT. For example, if all four documents are present, then
     * the `health_percentage` is `100`. If only one is present, then the
     * `health_percentage` is `25`.
     *
     * `content_reports_enabled` is only returned for organization-owned repositories.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-community-profile-metrics>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get_community_profile_metrics(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::types::CommunityProfile> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/community/profile",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * The `basehead` param is comprised of two parts: `base` and `head`. Both must be branch names in `repo`. To compare branches across other repositories in the same network as `repo`, use the format `<USERNAME>:branch`.
     *
     * The response from the API is equivalent to running the `git log base..head` command; however, commits are returned in chronological order. Pass the appropriate [media type](https://docs.github.com/rest/overview/media-types/#commits-commit-comparison-and-pull-requests) to fetch diff and patch formats.
     *
     * The response also includes details on the files that were changed between the two commits. This includes the status of the change (for example, if a file was added, removed, modified, or renamed), and details of the change itself. For example, files with a `renamed` status have a `previous_filename` field showing the previous filename of the file, and files with a `modified` status have a `patch` field showing the changes made to the file.
     *
     * **Working with large comparisons**
     *
     * To process a response with a large number of commits, you can use (`per_page` or `page`) to paginate the results. When using paging, the list of changed files is only returned with page 1, but includes all changed files for the entire comparison. For more information on working with pagination, see "[Traversing with pagination](/rest/guides/traversing-with-pagination)."
     *
     * When calling this API without any paging parameters (`per_page` or `page`), the returned list is limited to 250 commits and the last commit in the list is the most recent of the entire comparison. When a paging parameter is specified, the first commit in the returned list of each page is the earliest.
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
     * | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on her/his account. |
     * | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
     * | `unknown_key` | The key that made the signature has not been registered with any user's account. |
     * | `malformed_signature` | There was an error parsing the signature. |
     * | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
     * | `valid` | None of the above errors applied, so the signature is considered to be verified. |
     *
     * FROM: <https://docs.github.com/rest/reference/repos#compare-two-commits>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `page: i64` -- Page number of the results to fetch.
     * * `per_page: i64` -- Results per page (max 100).
     * * `basehead: &str` -- The base branch and head branch to compare. This parameter expects the format `{base}...{head}`.
     */
    pub async fn compare_commits(
        &self,
        owner: &str,
        repo: &str,
        page: i64,
        per_page: i64,
        basehead: &str,
    ) -> ClientResult<crate::types::CommitComparison> {
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(basehead),
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
     * Gets the contents of a file or directory in a repository. Specify the file path or directory in `:path`. If you omit
     * `:path`, you will receive the contents of the repository's root directory. See the description below regarding what the API response includes for directories.
     *
     * Files and symlinks support [a custom media type](https://docs.github.com/rest/reference/repos#custom-media-types) for
     * retrieving the raw content or rendered HTML (when supported). All content types support [a custom media
     * type](https://docs.github.com/rest/reference/repos#custom-media-types) to ensure the content is returned in a consistent
     * object format.
     *
     * **Note**:
     * *   To get a repository's contents recursively, you can [recursively get the tree](https://docs.github.com/rest/reference/git#trees).
     * *   This API has an upper limit of 1,000 files for a directory. If you need to retrieve more files, use the [Git Trees
     * API](https://docs.github.com/rest/reference/git#get-a-tree).
     * *   This API supports files up to 1 megabyte in size.
     *
     * #### If the content is a directory
     * The response will be an array of objects, one object for each item in the directory.
     * When listing the contents of a directory, submodules have their "type" specified as "file". Logically, the value
     * _should_ be "submodule". This behavior exists in API v3 [for backwards compatibility purposes](https://git.io/v1YCW).
     * In the next major version of the API, the type will be returned as "submodule".
     *
     * #### If the content is a symlink
     * If the requested `:path` points to a symlink, and the symlink's target is a normal file in the repository, then the
     * API responds with the content of the file (in the format shown in the example. Otherwise, the API responds with an object
     * describing the symlink itself.
     *
     * #### If the content is a submodule
     * The `submodule_git_url` identifies the location of the submodule repository, and the `sha` identifies a specific
     * commit within the submodule repository. Git uses the given URL when cloning the submodule repository, and checks out
     * the submodule at that specific commit.
     *
     * If the submodule repository is not hosted on github.com, the Git URLs (`git_url` and `_links["git"]`) and the
     * github.com URLs (`html_url` and `_links["html"]`) will have null values.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-repository-content>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `path: &str` -- path parameter.
     * * `ref_: &str` -- The name of the commit/branch/tag. Default: the repository’s default branch (usually `master`).
     */
    pub async fn get_content_vec_entries(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        ref_: &str,
    ) -> ClientResult<Vec<crate::types::Entries>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/contents/{}?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(path),
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
     * Gets the contents of a file or directory in a repository. Specify the file path or directory in `:path`. If you omit
     * `:path`, you will receive the contents of the repository's root directory. See the description below regarding what the API response includes for directories.
     *
     * Files and symlinks support [a custom media type](https://docs.github.com/rest/reference/repos#custom-media-types) for
     * retrieving the raw content or rendered HTML (when supported). All content types support [a custom media
     * type](https://docs.github.com/rest/reference/repos#custom-media-types) to ensure the content is returned in a consistent
     * object format.
     *
     * **Note**:
     * *   To get a repository's contents recursively, you can [recursively get the tree](https://docs.github.com/rest/reference/git#trees).
     * *   This API has an upper limit of 1,000 files for a directory. If you need to retrieve more files, use the [Git Trees
     * API](https://docs.github.com/rest/reference/git#get-a-tree).
     * *   This API supports files up to 1 megabyte in size.
     *
     * #### If the content is a directory
     * The response will be an array of objects, one object for each item in the directory.
     * When listing the contents of a directory, submodules have their "type" specified as "file". Logically, the value
     * _should_ be "submodule". This behavior exists in API v3 [for backwards compatibility purposes](https://git.io/v1YCW).
     * In the next major version of the API, the type will be returned as "submodule".
     *
     * #### If the content is a symlink
     * If the requested `:path` points to a symlink, and the symlink's target is a normal file in the repository, then the
     * API responds with the content of the file (in the format shown in the example. Otherwise, the API responds with an object
     * describing the symlink itself.
     *
     * #### If the content is a submodule
     * The `submodule_git_url` identifies the location of the submodule repository, and the `sha` identifies a specific
     * commit within the submodule repository. Git uses the given URL when cloning the submodule repository, and checks out
     * the submodule at that specific commit.
     *
     * If the submodule repository is not hosted on github.com, the Git URLs (`git_url` and `_links["git"]`) and the
     * github.com URLs (`html_url` and `_links["html"]`) will have null values.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-repository-content>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `path: &str` -- path parameter.
     * * `ref_: &str` -- The name of the commit/branch/tag. Default: the repository’s default branch (usually `master`).
     */
    pub async fn get_content_file(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        ref_: &str,
    ) -> ClientResult<crate::types::ContentFile> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/contents/{}?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(path),
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
     * Gets the contents of a file or directory in a repository. Specify the file path or directory in `:path`. If you omit
     * `:path`, you will receive the contents of the repository's root directory. See the description below regarding what the API response includes for directories.
     *
     * Files and symlinks support [a custom media type](https://docs.github.com/rest/reference/repos#custom-media-types) for
     * retrieving the raw content or rendered HTML (when supported). All content types support [a custom media
     * type](https://docs.github.com/rest/reference/repos#custom-media-types) to ensure the content is returned in a consistent
     * object format.
     *
     * **Note**:
     * *   To get a repository's contents recursively, you can [recursively get the tree](https://docs.github.com/rest/reference/git#trees).
     * *   This API has an upper limit of 1,000 files for a directory. If you need to retrieve more files, use the [Git Trees
     * API](https://docs.github.com/rest/reference/git#get-a-tree).
     * *   This API supports files up to 1 megabyte in size.
     *
     * #### If the content is a directory
     * The response will be an array of objects, one object for each item in the directory.
     * When listing the contents of a directory, submodules have their "type" specified as "file". Logically, the value
     * _should_ be "submodule". This behavior exists in API v3 [for backwards compatibility purposes](https://git.io/v1YCW).
     * In the next major version of the API, the type will be returned as "submodule".
     *
     * #### If the content is a symlink
     * If the requested `:path` points to a symlink, and the symlink's target is a normal file in the repository, then the
     * API responds with the content of the file (in the format shown in the example. Otherwise, the API responds with an object
     * describing the symlink itself.
     *
     * #### If the content is a submodule
     * The `submodule_git_url` identifies the location of the submodule repository, and the `sha` identifies a specific
     * commit within the submodule repository. Git uses the given URL when cloning the submodule repository, and checks out
     * the submodule at that specific commit.
     *
     * If the submodule repository is not hosted on github.com, the Git URLs (`git_url` and `_links["git"]`) and the
     * github.com URLs (`html_url` and `_links["html"]`) will have null values.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-repository-content>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `path: &str` -- path parameter.
     * * `ref_: &str` -- The name of the commit/branch/tag. Default: the repository’s default branch (usually `master`).
     */
    pub async fn get_content_symlink(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        ref_: &str,
    ) -> ClientResult<crate::types::SymlinkContent> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/contents/{}?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(path),
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
     * Gets the contents of a file or directory in a repository. Specify the file path or directory in `:path`. If you omit
     * `:path`, you will receive the contents of the repository's root directory. See the description below regarding what the API response includes for directories.
     *
     * Files and symlinks support [a custom media type](https://docs.github.com/rest/reference/repos#custom-media-types) for
     * retrieving the raw content or rendered HTML (when supported). All content types support [a custom media
     * type](https://docs.github.com/rest/reference/repos#custom-media-types) to ensure the content is returned in a consistent
     * object format.
     *
     * **Note**:
     * *   To get a repository's contents recursively, you can [recursively get the tree](https://docs.github.com/rest/reference/git#trees).
     * *   This API has an upper limit of 1,000 files for a directory. If you need to retrieve more files, use the [Git Trees
     * API](https://docs.github.com/rest/reference/git#get-a-tree).
     * *   This API supports files up to 1 megabyte in size.
     *
     * #### If the content is a directory
     * The response will be an array of objects, one object for each item in the directory.
     * When listing the contents of a directory, submodules have their "type" specified as "file". Logically, the value
     * _should_ be "submodule". This behavior exists in API v3 [for backwards compatibility purposes](https://git.io/v1YCW).
     * In the next major version of the API, the type will be returned as "submodule".
     *
     * #### If the content is a symlink
     * If the requested `:path` points to a symlink, and the symlink's target is a normal file in the repository, then the
     * API responds with the content of the file (in the format shown in the example. Otherwise, the API responds with an object
     * describing the symlink itself.
     *
     * #### If the content is a submodule
     * The `submodule_git_url` identifies the location of the submodule repository, and the `sha` identifies a specific
     * commit within the submodule repository. Git uses the given URL when cloning the submodule repository, and checks out
     * the submodule at that specific commit.
     *
     * If the submodule repository is not hosted on github.com, the Git URLs (`git_url` and `_links["git"]`) and the
     * github.com URLs (`html_url` and `_links["html"]`) will have null values.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-repository-content>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `path: &str` -- path parameter.
     * * `ref_: &str` -- The name of the commit/branch/tag. Default: the repository’s default branch (usually `master`).
     */
    pub async fn get_content_submodule(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        ref_: &str,
    ) -> ClientResult<crate::types::ContentSubmodule> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/contents/{}?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(path),
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
     * Gets the contents of a file or directory in a repository. Specify the file path or directory in `:path`. If you omit
     * `:path`, you will receive the contents of the repository's root directory. See the description below regarding what the API response includes for directories.
     *
     * Files and symlinks support [a custom media type](https://docs.github.com/rest/reference/repos#custom-media-types) for
     * retrieving the raw content or rendered HTML (when supported). All content types support [a custom media
     * type](https://docs.github.com/rest/reference/repos#custom-media-types) to ensure the content is returned in a consistent
     * object format.
     *
     * **Note**:
     * *   To get a repository's contents recursively, you can [recursively get the tree](https://docs.github.com/rest/reference/git#trees).
     * *   This API has an upper limit of 1,000 files for a directory. If you need to retrieve more files, use the [Git Trees
     * API](https://docs.github.com/rest/reference/git#get-a-tree).
     * *   This API supports files up to 1 megabyte in size.
     *
     * #### If the content is a directory
     * The response will be an array of objects, one object for each item in the directory.
     * When listing the contents of a directory, submodules have their "type" specified as "file". Logically, the value
     * _should_ be "submodule". This behavior exists in API v3 [for backwards compatibility purposes](https://git.io/v1YCW).
     * In the next major version of the API, the type will be returned as "submodule".
     *
     * #### If the content is a symlink
     * If the requested `:path` points to a symlink, and the symlink's target is a normal file in the repository, then the
     * API responds with the content of the file (in the format shown in the example. Otherwise, the API responds with an object
     * describing the symlink itself.
     *
     * #### If the content is a submodule
     * The `submodule_git_url` identifies the location of the submodule repository, and the `sha` identifies a specific
     * commit within the submodule repository. Git uses the given URL when cloning the submodule repository, and checks out
     * the submodule at that specific commit.
     *
     * If the submodule repository is not hosted on github.com, the Git URLs (`git_url` and `_links["git"]`) and the
     * github.com URLs (`html_url` and `_links["html"]`) will have null values.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-repository-content>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `path: &str` -- path parameter.
     * * `ref_: &str` -- The name of the commit/branch/tag. Default: the repository’s default branch (usually `master`).
     */
    pub async fn get_content(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        ref_: &str,
    ) -> ClientResult<crate::types::ReposGetContentResponseOneOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/contents/{}?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(path),
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
     * Create or update file contents.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/contents/{path}` endpoint.
     *
     * Creates a new file or replaces an existing file in a repository.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#create-or-update-file-contents>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `path: &str` -- path parameter.
     */
    pub async fn create_or_update_file_contents(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        body: &crate::types::ReposCreateUpdateFileContentsRequest,
    ) -> ClientResult<crate::types::FileCommitData> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/contents/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(path),
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
     * FROM: <https://docs.github.com/rest/reference/repos#delete-a-file>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `path: &str` -- path parameter.
     */
    pub async fn delete_file(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        body: &crate::types::ReposDeleteFileRequest,
    ) -> ClientResult<crate::types::FileCommitData> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/contents/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(path),
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
     * Lists contributors to the specified repository and sorts them by the number of commits per contributor in descending order. This endpoint may return information that is a few hours old because the GitHub REST API v3 caches contributor data to improve performance.
     *
     * GitHub identifies contributors by author email address. This endpoint groups contribution counts by GitHub user, which includes all associated email addresses. To improve performance, only the first 500 author email addresses in the repository link to GitHub users. The rest will appear as anonymous contributors without associated GitHub user information.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-repository-contributors>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `anon: &str` -- Set to `1` or `true` to include anonymous contributors in results.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_contributors(
        &self,
        owner: &str,
        repo: &str,
        anon: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<Vec<crate::types::Contributor>> {
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
     * List repository contributors.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/contributors` endpoint.
     *
     * As opposed to `list_contributors`, this function returns all the pages of the request at once.
     *
     * Lists contributors to the specified repository and sorts them by the number of commits per contributor in descending order. This endpoint may return information that is a few hours old because the GitHub REST API v3 caches contributor data to improve performance.
     *
     * GitHub identifies contributors by author email address. This endpoint groups contribution counts by GitHub user, which includes all associated email addresses. To improve performance, only the first 500 author email addresses in the repository link to GitHub users. The rest will appear as anonymous contributors without associated GitHub user information.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-repository-contributors>
     */
    pub async fn list_all_contributors(
        &self,
        owner: &str,
        repo: &str,
        anon: &str,
    ) -> ClientResult<Vec<crate::types::Contributor>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !anon.is_empty() {
            query_args.push(("anon".to_string(), anon.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/contributors?{}",
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
     * List deployments.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/deployments` endpoint.
     *
     * Simple filtering of deployments is available via query parameters:
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-deployments>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `sha: &str` -- The SHA recorded at creation time.
     * * `ref_: &str` -- The name of the ref. This can be a branch, tag, or SHA.
     * * `task: &str` -- The name of the task for the deployment (e.g., `deploy` or `deploy:migrations`).
     * * `environment: &str` -- The name of the environment that was deployed to (e.g., `staging` or `production`).
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
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
    ) -> ClientResult<Vec<crate::types::Deployment>> {
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
     * List deployments.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/deployments` endpoint.
     *
     * As opposed to `list_deployments`, this function returns all the pages of the request at once.
     *
     * Simple filtering of deployments is available via query parameters:
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-deployments>
     */
    pub async fn list_all_deployments(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        ref_: &str,
        task: &str,
        environment: &str,
    ) -> ClientResult<Vec<crate::types::Deployment>> {
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
     * By default, [commit statuses](https://docs.github.com/rest/reference/repos#statuses) for every submitted context must be in a `success`
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
     * Users with `repo` or `repo_deployment` scopes can create a deployment for a given ref.
     *
     * #### Merged branch response
     * You will see this response when GitHub automatically merges the base branch into the topic branch instead of creating
     * a deployment. This auto-merge happens when:
     * *   Auto-merge option is enabled in the repository
     * *   Topic branch does not include the latest changes on the base branch, which is `master` in the response example
     * *   There are no merge conflicts
     *
     * If there are no new commits in the base branch, a new request to create a deployment should give a successful
     * response.
     *
     * #### Merge conflict response
     * This error happens when the `auto_merge` option is enabled and when the default branch (in this case `master`), can't
     * be merged into the branch that's being deployed (in this case `topic-branch`), due to merge conflicts.
     *
     * #### Failed commit status checks
     * This error happens when the `required_contexts` parameter indicates that one or more contexts need to have a `success`
     * status for the commit to be deployed, but one or more of the required contexts do not have a state of `success`.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#create-a-deployment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn create_deployment(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposCreateDeploymentRequest,
    ) -> ClientResult<crate::types::Deployment> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/deployments",
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
     * Get a deployment.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/deployments/{deployment_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-a-deployment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `deployment_id: i64` -- deployment_id parameter.
     */
    pub async fn get_deployment(
        &self,
        owner: &str,
        repo: &str,
        deployment_id: i64,
    ) -> ClientResult<crate::types::Deployment> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/deployments/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * To ensure there can always be an active deployment, you can only delete an _inactive_ deployment. Anyone with `repo` or `repo_deployment` scopes can delete an inactive deployment.
     *
     * To set a deployment as inactive, you must:
     *
     * *   Create a new deployment that is active so that the system has a record of the current state, then delete the previously active deployment.
     * *   Mark the active deployment as inactive by adding any non-successful deployment status.
     *
     * For more information, see "[Create a deployment](https://docs.github.com/rest/reference/repos/#create-a-deployment)" and "[Create a deployment status](https://docs.github.com/rest/reference/repos#create-a-deployment-status)."
     *
     * FROM: <https://docs.github.com/rest/reference/repos#delete-a-deployment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `deployment_id: i64` -- deployment_id parameter.
     */
    pub async fn delete_deployment(
        &self,
        owner: &str,
        repo: &str,
        deployment_id: i64,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/deployments/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#list-deployment-statuses>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `deployment_id: i64` -- deployment_id parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_deployment_statuses(
        &self,
        owner: &str,
        repo: &str,
        deployment_id: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<Vec<crate::types::DeploymentStatus>> {
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#list-deployment-statuses>
     */
    pub async fn list_all_deployment_statuses(
        &self,
        owner: &str,
        repo: &str,
        deployment_id: i64,
    ) -> ClientResult<Vec<crate::types::DeploymentStatus>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/deployments/{}/statuses",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * GitHub Apps require `read & write` access to "Deployments" and `read-only` access to "Repo contents" (for private repos). OAuth Apps require the `repo_deployment` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#create-a-deployment-status>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `deployment_id: i64` -- deployment_id parameter.
     */
    pub async fn create_deployment_status(
        &self,
        owner: &str,
        repo: &str,
        deployment_id: i64,
        body: &crate::types::ReposCreateDeploymentStatusRequest,
    ) -> ClientResult<crate::types::DeploymentStatus> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/deployments/{}/statuses",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#get-a-deployment-status>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `deployment_id: i64` -- deployment_id parameter.
     * * `status_id: i64`
     */
    pub async fn get_deployment_status(
        &self,
        owner: &str,
        repo: &str,
        deployment_id: i64,
        status_id: i64,
    ) -> ClientResult<crate::types::DeploymentStatus> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/deployments/{}/statuses/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * This endpoint requires write access to the repository by providing either:
     *
     *   - Personal access tokens with `repo` scope. For more information, see "[Creating a personal access token for the command line](https://help.github.com/articles/creating-a-personal-access-token-for-the-command-line)" in the GitHub Help documentation.
     *   - GitHub Apps with both `metadata:read` and `contents:read&write` permissions.
     *
     * This input example shows how you can use the `client_payload` as a test to debug your workflow.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#create-a-repository-dispatch-event>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn create_dispatch_event(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposCreateDispatchEventRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/dispatches",
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
     * Get all environments.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/environments` endpoint.
     *
     * Get all environments for a repository.
     *
     * Anyone with read access to the repository can use this endpoint. If the repository is private, you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-all-environments>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get_all_environments(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::types::ReposGetAllEnvironmentsResponse> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Anyone with read access to the repository can use this endpoint. If the repository is private, you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-an-environment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `environment_name: &str` -- The name of the environment.
     */
    pub async fn get_environment(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
    ) -> ClientResult<crate::types::EnvironmentData> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(environment_name),
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
     * **Note:** Although you can use this operation to specify that only branches that match specified name patterns can deploy to this environment, you must use the UI to set the name patterns. For more information, see "[Environments](/actions/reference/environments#deployment-branches)."
     *
     * **Note:** To create or update secrets for an environment, see "[Secrets](/rest/reference/actions#secrets)."
     *
     * You must authenticate using an access token with the repo scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#create-or-update-an-environment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `environment_name: &str` -- The name of the environment.
     */
    pub async fn create_or_update_environment(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
        body: &crate::types::ReposCreateUpdateEnvironmentRequest,
    ) -> ClientResult<crate::types::EnvironmentData> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(environment_name),
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
     * You must authenticate using an access token with the repo scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#delete-an-environment>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `environment_name: &str` -- The name of the environment.
     */
    pub async fn delete_an_environment(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(environment_name),
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
     * FROM: <https://docs.github.com/rest/reference/repos#list-forks>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `sort: crate::types::ReposListForksSort` -- The sort order. Can be either `newest`, `oldest`, or `stargazers`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_forks(
        &self,
        owner: &str,
        repo: &str,
        sort: crate::types::ReposListForksSort,
        per_page: i64,
        page: i64,
    ) -> ClientResult<Vec<crate::types::MinimalRepository>> {
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
     * List forks.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/forks` endpoint.
     *
     * As opposed to `list_forks`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-forks>
     */
    pub async fn list_all_forks(
        &self,
        owner: &str,
        repo: &str,
        sort: crate::types::ReposListForksSort,
    ) -> ClientResult<Vec<crate::types::MinimalRepository>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/forks?{}",
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
     * Create a fork.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/forks` endpoint.
     *
     * Create a fork for the authenticated user.
     *
     * **Note**: Forking a Repository happens asynchronously. You may have to wait a short period of time before you can access the git objects. If this takes longer than 5 minutes, be sure to contact [GitHub Support](https://support.github.com/contact?tags=rest-api).
     *
     * FROM: <https://docs.github.com/rest/reference/repos#create-a-fork>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn create_fork(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposCreateForkRequest,
    ) -> ClientResult<crate::types::FullRepository> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/forks",
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
     * List repository webhooks.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/hooks` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-repository-webhooks>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_webhooks(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<Vec<crate::types::Hook>> {
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
     * List repository webhooks.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/hooks` endpoint.
     *
     * As opposed to `list_webhooks`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-repository-webhooks>
     */
    pub async fn list_all_webhooks(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<Vec<crate::types::Hook>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#create-a-repository-webhook>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn create_webhook(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposCreateWebhookRequest,
    ) -> ClientResult<crate::types::Hook> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks",
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
     * Get a repository webhook.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/hooks/{hook_id}` endpoint.
     *
     * Returns a webhook configured in a repository. To get only the webhook `config` properties, see "[Get a webhook configuration for a repository](/rest/reference/repos#get-a-webhook-configuration-for-a-repository)."
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-a-repository-webhook>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `hook_id: i64`
     */
    pub async fn get_webhook(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
    ) -> ClientResult<crate::types::Hook> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#delete-a-repository-webhook>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `hook_id: i64`
     */
    pub async fn delete_webhook(&self, owner: &str, repo: &str, hook_id: i64) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Updates a webhook configured in a repository. If you previously had a `secret` set, you must provide the same `secret` or set a new `secret` or the secret will be removed. If you are only updating individual webhook `config` properties, use "[Update a webhook configuration for a repository](/rest/reference/repos#update-a-webhook-configuration-for-a-repository)."
     *
     * FROM: <https://docs.github.com/rest/reference/repos#update-a-repository-webhook>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `hook_id: i64`
     */
    pub async fn update_webhook(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
        body: &crate::types::ReposUpdateWebhookRequest,
    ) -> ClientResult<crate::types::Hook> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Returns the webhook configuration for a repository. To get more information about the webhook, including the `active` state and `events`, use "[Get a repository webhook](/rest/reference/orgs#get-a-repository-webhook)."
     *
     * Access tokens must have the `read:repo_hook` or `repo` scope, and GitHub Apps must have the `repository_hooks:read` permission.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-a-webhook-configuration-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `hook_id: i64`
     */
    pub async fn get_webhook_config_for_repo(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
    ) -> ClientResult<crate::types::WebhookConfig> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks/{}/config",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Updates the webhook configuration for a repository. To update more information about the webhook, including the `active` state and `events`, use "[Update a repository webhook](/rest/reference/orgs#update-a-repository-webhook)."
     *
     * Access tokens must have the `write:repo_hook` or `repo` scope, and GitHub Apps must have the `repository_hooks:write` permission.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#update-a-webhook-configuration-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `hook_id: i64`
     */
    pub async fn update_webhook_config_for_repo(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
        body: &crate::types::AppsUpdateWebhookConfigAppRequest,
    ) -> ClientResult<crate::types::WebhookConfig> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks/{}/config",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#list-deliveries-for-a-repository-webhook>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `hook_id: i64`
     * * `per_page: i64` -- Results per page (max 100).
     * * `cursor: &str` -- Used for pagination: the starting delivery from which the page of deliveries is fetched. Refer to the `link` header for the next and previous page cursors.
     */
    pub async fn list_webhook_deliveries(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
        per_page: i64,
        cursor: &str,
    ) -> ClientResult<Vec<crate::types::HookDeliveryItem>> {
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#list-deliveries-for-a-repository-webhook>
     */
    pub async fn list_all_webhook_deliveries(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
        cursor: &str,
    ) -> ClientResult<Vec<crate::types::HookDeliveryItem>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks/{}/deliveries?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#get-a-delivery-for-a-repository-webhook>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `hook_id: i64`
     * * `delivery_id: i64`
     */
    pub async fn get_webhook_delivery(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
        delivery_id: i64,
    ) -> ClientResult<crate::types::HookDelivery> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks/{}/deliveries/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#redeliver-a-delivery-for-a-repository-webhook>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `hook_id: i64`
     * * `delivery_id: i64`
     */
    pub async fn redeliver_webhook_delivery(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
        delivery_id: i64,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks/{}/deliveries/{}/attempts",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#ping-a-repository-webhook>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `hook_id: i64`
     */
    pub async fn ping_webhook(&self, owner: &str, repo: &str, hook_id: i64) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks/{}/pings",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * **Note**: Previously `/repos/:owner/:repo/hooks/:hook_id/test`
     *
     * FROM: <https://docs.github.com/rest/reference/repos#test-the-push-repository-webhook>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `hook_id: i64`
     */
    pub async fn test_push_webhook(
        &self,
        owner: &str,
        repo: &str,
        hook_id: i64,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/hooks/{}/tests",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * List repository invitations.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/invitations` endpoint.
     *
     * When authenticating as a user with admin rights to a repository, this endpoint will list all currently open repository invitations.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-repository-invitations>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_invitations(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<Vec<crate::types::RepositoryInvitation>> {
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
     * List repository invitations.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/invitations` endpoint.
     *
     * As opposed to `list_invitations`, this function returns all the pages of the request at once.
     *
     * When authenticating as a user with admin rights to a repository, this endpoint will list all currently open repository invitations.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-repository-invitations>
     */
    pub async fn list_all_invitations(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<Vec<crate::types::RepositoryInvitation>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/invitations",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#delete-a-repository-invitation>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `invitation_id: i64` -- invitation_id parameter.
     */
    pub async fn delete_invitation(
        &self,
        owner: &str,
        repo: &str,
        invitation_id: i64,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/invitations/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#update-a-repository-invitation>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `invitation_id: i64` -- invitation_id parameter.
     */
    pub async fn update_invitation(
        &self,
        owner: &str,
        repo: &str,
        invitation_id: i64,
        body: &crate::types::ReposUpdateInvitationRequest,
    ) -> ClientResult<crate::types::RepositoryInvitation> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/invitations/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#list-deploy-keys>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_deploy_keys(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<Vec<crate::types::DeployKey>> {
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
     * List deploy keys.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/keys` endpoint.
     *
     * As opposed to `list_deploy_keys`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-deploy-keys>
     */
    pub async fn list_all_deploy_keys(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<Vec<crate::types::DeployKey>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/keys",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#create-a-deploy-key>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn create_deploy_key(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposCreateDeployKeyRequest,
    ) -> ClientResult<crate::types::DeployKey> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/keys",
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
     * Get a deploy key.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/keys/{key_id}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-a-deploy-key>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `key_id: i64` -- key_id parameter.
     */
    pub async fn get_deploy_key(
        &self,
        owner: &str,
        repo: &str,
        key_id: i64,
    ) -> ClientResult<crate::types::DeployKey> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/keys/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#delete-a-deploy-key>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `key_id: i64` -- key_id parameter.
     */
    pub async fn delete_deploy_key(
        &self,
        owner: &str,
        repo: &str,
        key_id: i64,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/keys/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#list-repository-languages>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn list_languages(&self, owner: &str, repo: &str) -> ClientResult<i64> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/languages",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Merge a branch.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/merges` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#merge-a-branch>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn merge(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposMergeRequest,
    ) -> ClientResult<crate::types::CommitDataType> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/merges",
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
     * Get a GitHub Pages site.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pages` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-a-github-pages-site>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get_pages(&self, owner: &str, repo: &str) -> ClientResult<crate::types::Page> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pages",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#update-information-about-a-github-pages-site>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn update_information_about_pages_site(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposUpdateInformationAboutPagesSiteRequest,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pages",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#create-a-github-pages-site>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn create_pages_site(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposCreatePagesSiteRequest,
    ) -> ClientResult<crate::types::Page> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pages",
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
     * Delete a GitHub Pages site.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/pages` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#delete-a-github-pages-site>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn delete_pages_site(&self, owner: &str, repo: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pages",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-github-pages-builds>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_pages_builds(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<Vec<crate::types::PageBuild>> {
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
     * List GitHub Pages builds.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pages/builds` endpoint.
     *
     * As opposed to `list_pages_builds`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-github-pages-builds>
     */
    pub async fn list_all_pages_builds(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<Vec<crate::types::PageBuild>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pages/builds",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#request-a-github-pages-build>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn request_pages_build(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::types::PageBuildStatus> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pages/builds",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-latest-pages-build>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get_latest_pages_build(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::types::PageBuild> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pages/builds/latest",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-github-pages-build>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `build_id: i64`
     */
    pub async fn get_pages_build(
        &self,
        owner: &str,
        repo: &str,
        build_id: i64,
    ) -> ClientResult<crate::types::PageBuild> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pages/builds/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Get a DNS health check for GitHub Pages.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/pages/health` endpoint.
     *
     * Gets a health check of the DNS settings for the `CNAME` record configured for a repository's GitHub Pages.
     *
     * The first request to this endpoint returns a `202 Accepted` status and starts an asynchronous background task to get the results for the domain. After the background task completes, subsequent requests to this endpoint return a `200 OK` status with the health check results in the response.
     *
     * Users must have admin or owner permissions. GitHub Apps must have the `pages:write` and `administration:write` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-a-dns-health-check-for-github-pages>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get_pages_health_check(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::types::PagesHealthCheck> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pages/health",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Get a repository README.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/readme` endpoint.
     *
     * Gets the preferred README for a repository.
     *
     * READMEs support [custom media types](https://docs.github.com/rest/reference/repos#custom-media-types) for retrieving the raw content or rendered HTML.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-a-repository-readme>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `ref_: &str` -- The name of the commit/branch/tag. Default: the repository’s default branch (usually `master`).
     */
    pub async fn get_readme(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> ClientResult<crate::types::ContentFile> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/readme?{}",
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
     * Get a repository README for a directory.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/readme/{dir}` endpoint.
     *
     * Gets the README from a repository directory.
     *
     * READMEs support [custom media types](https://docs.github.com/rest/reference/repos#custom-media-types) for retrieving the raw content or rendered HTML.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-a-repository-directory-readme>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `dir: &str` -- The alternate path to look for a README file.
     * * `ref_: &str` -- The name of the commit/branch/tag. Default: the repository’s default branch (usually `master`).
     */
    pub async fn get_readme_in_directory(
        &self,
        owner: &str,
        repo: &str,
        dir: &str,
        ref_: &str,
    ) -> ClientResult<crate::types::ContentFile> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/readme/{}?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(dir),
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
     * This returns a list of releases, which does not include regular Git tags that have not been associated with a release. To get a list of Git tags, use the [Repository Tags API](https://docs.github.com/rest/reference/repos#list-repository-tags).
     *
     * Information about published releases are available to everyone. Only users with push access will receive listings for draft releases.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-releases>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_releases(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<Vec<crate::types::Release>> {
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
     * List releases.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/releases` endpoint.
     *
     * As opposed to `list_releases`, this function returns all the pages of the request at once.
     *
     * This returns a list of releases, which does not include regular Git tags that have not been associated with a release. To get a list of Git tags, use the [Repository Tags API](https://docs.github.com/rest/reference/repos#list-repository-tags).
     *
     * Information about published releases are available to everyone. Only users with push access will receive listings for draft releases.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-releases>
     */
    pub async fn list_all_releases(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<Vec<crate::types::Release>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#create-a-release>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn create_release(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposCreateReleaseRequest,
    ) -> ClientResult<crate::types::Release> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases",
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
     * Get a release asset.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/releases/assets/{asset_id}` endpoint.
     *
     * To download the asset's binary content, set the `Accept` header of the request to [`application/octet-stream`](https://docs.github.com/rest/overview/media-types). The API will either redirect the client to the location, or stream it directly if possible. API clients should handle both a `200` or `302` response.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-a-release-asset>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `asset_id: i64` -- asset_id parameter.
     */
    pub async fn get_release_asset(
        &self,
        owner: &str,
        repo: &str,
        asset_id: i64,
    ) -> ClientResult<crate::types::ReleaseAsset> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/assets/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#delete-a-release-asset>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `asset_id: i64` -- asset_id parameter.
     */
    pub async fn delete_release_asset(
        &self,
        owner: &str,
        repo: &str,
        asset_id: i64,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/assets/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#update-a-release-asset>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `asset_id: i64` -- asset_id parameter.
     */
    pub async fn update_release_asset(
        &self,
        owner: &str,
        repo: &str,
        asset_id: i64,
        body: &crate::types::ReposUpdateReleaseAssetRequest,
    ) -> ClientResult<crate::types::ReleaseAsset> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/assets/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Get the latest release.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/releases/latest` endpoint.
     *
     * View the latest published full release for the repository.
     *
     * The latest release is the most recent non-prerelease, non-draft release, sorted by the `created_at` attribute. The `created_at` attribute is the date of the commit used for the release, and not the date when the release was drafted or published.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-the-latest-release>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get_latest_release(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::types::Release> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/latest",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#get-a-release-by-tag-name>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `tag: &str` -- tag parameter.
     */
    pub async fn get_release_by_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> ClientResult<crate::types::Release> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/tags/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(tag),
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
     * **Note:** This returns an `upload_url` key corresponding to the endpoint for uploading release assets. This key is a [hypermedia resource](https://docs.github.com/rest/overview/resources-in-the-rest-api#hypermedia).
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-a-release>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `release_id: i64` -- release_id parameter.
     */
    pub async fn get_release(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
    ) -> ClientResult<crate::types::Release> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#delete-a-release>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `release_id: i64` -- release_id parameter.
     */
    pub async fn delete_release(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#update-a-release>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `release_id: i64` -- release_id parameter.
     */
    pub async fn update_release(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
        body: &crate::types::ReposCreateReleaseRequest,
    ) -> ClientResult<crate::types::Release> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#list-release-assets>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `release_id: i64` -- release_id parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_release_assets(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<Vec<crate::types::ReleaseAsset>> {
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#list-release-assets>
     */
    pub async fn list_all_release_assets(
        &self,
        owner: &str,
        repo: &str,
        release_id: i64,
    ) -> ClientResult<Vec<crate::types::ReleaseAsset>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/releases/{}/assets",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * This endpoint makes use of [a Hypermedia relation](https://docs.github.com/rest/overview/resources-in-the-rest-api#hypermedia) to determine which URL to access. The endpoint you call to upload release assets is specific to your release. Use the `upload_url` returned in
     * the response of the [Create a release endpoint](https://docs.github.com/rest/reference/repos#create-a-release) to upload a release asset.
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
     * *   GitHub renames asset filenames that have special characters, non-alphanumeric characters, and leading or trailing periods. The "[List assets for a release](https://docs.github.com/rest/reference/repos#list-assets-for-a-release)"
     * endpoint lists the renamed filenames. For more information and help, contact [GitHub Support](https://support.github.com/contact?tags=rest-api).
     * *   If you upload an asset with the same filename as another uploaded asset, you'll receive an error and must delete the old file before you can re-upload the new asset.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#upload-a-release-asset>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `release_id: i64` -- release_id parameter.
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
    ) -> ClientResult<crate::types::ReleaseAsset> {
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
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Get the weekly commit activity.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/stats/code_frequency` endpoint.
     *
     * Returns a weekly aggregate of the number of additions and deletions pushed to a repository.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-the-weekly-commit-activity>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get_code_frequency_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<Vec<Vec<i64>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/stats/code_frequency",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#get-the-weekly-commit-activity>
     */
    pub async fn get_all_code_frequency_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<Vec<Vec<i64>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/stats/code_frequency",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#get-the-last-year-of-commit-activity>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get_commit_activity_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<Vec<crate::types::CommitActivity>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/stats/commit_activity",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#get-the-last-year-of-commit-activity>
     */
    pub async fn get_all_commit_activity_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<Vec<crate::types::CommitActivity>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/stats/commit_activity",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * *   `w` - Start of the week, given as a [Unix timestamp](http://en.wikipedia.org/wiki/Unix_time).
     * *   `a` - Number of additions
     * *   `d` - Number of deletions
     * *   `c` - Number of commits
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-all-contributor-commit-activity>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get_contributors_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<Vec<crate::types::ContributorActivity>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/stats/contributors",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * *   `w` - Start of the week, given as a [Unix timestamp](http://en.wikipedia.org/wiki/Unix_time).
     * *   `a` - Number of additions
     * *   `d` - Number of deletions
     * *   `c` - Number of commits
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-all-contributor-commit-activity>
     */
    pub async fn get_all_contributors_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<Vec<crate::types::ContributorActivity>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/stats/contributors",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#get-the-weekly-commit-count>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get_participation_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::types::ParticipationStats> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/stats/participation",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#get-the-hourly-commit-count-for-each-day>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get_punch_card_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<Vec<Vec<i64>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/stats/punch_card",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#get-the-hourly-commit-count-for-each-day>
     */
    pub async fn get_all_punch_card_stats(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<Vec<Vec<i64>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/stats/punch_card",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#create-a-commit-status>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `sha: &str`
     */
    pub async fn create_commit_status(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        body: &crate::types::ReposCreateCommitStatusRequest,
    ) -> ClientResult<crate::types::StatusData> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/statuses/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(sha),
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
     * FROM: <https://docs.github.com/rest/reference/repos#list-repository-tags>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_tags(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<Vec<crate::types::Tag>> {
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
     * List repository tags.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/tags` endpoint.
     *
     * As opposed to `list_tags`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-repository-tags>
     */
    pub async fn list_all_tags(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<Vec<crate::types::Tag>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/tags",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * `master`) will be used. Please make sure your HTTP framework is configured to follow redirects or you will need to use
     * the `Location` header to make a second `GET` request.
     * **Note**: For private repositories, these links are temporary and expire after five minutes.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#download-a-repository-archive>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `ref_: &str`
     */
    pub async fn download_tarball_archive(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/tarball/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(ref_),
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
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-repository-teams>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_teams(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<Vec<crate::types::Team>> {
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
     * List repository teams.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/teams` endpoint.
     *
     * As opposed to `list_teams`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-repository-teams>
     */
    pub async fn list_all_teams(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<Vec<crate::types::Team>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/teams",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#get-all-repository-topics>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `page: i64` -- Page number of the results to fetch.
     * * `per_page: i64` -- Results per page (max 100).
     */
    pub async fn get_all_topics(
        &self,
        owner: &str,
        repo: &str,
        page: i64,
        per_page: i64,
    ) -> ClientResult<crate::types::Topic> {
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
     * Replace all repository topics.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/topics` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/repos#replace-all-repository-topics>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn replace_all_topics(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::Topic,
    ) -> ClientResult<crate::types::Topic> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/topics",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#get-repository-clones>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per: crate::types::Per` -- Must be one of: `day`, `week`.
     */
    pub async fn get_clones(
        &self,
        owner: &str,
        repo: &str,
        per: crate::types::Per,
    ) -> ClientResult<crate::types::CloneTraffic> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !per.to_string().is_empty() {
            query_args.push(("per".to_string(), per.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/traffic/clones?{}",
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
     * Get top referral paths.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/traffic/popular/paths` endpoint.
     *
     * Get the top 10 popular contents over the last 14 days.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#get-top-referral-paths>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get_top_paths(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<Vec<crate::types::ContentTraffic>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/traffic/popular/paths",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#get-top-referral-paths>
     */
    pub async fn get_all_top_paths(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<Vec<crate::types::ContentTraffic>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/traffic/popular/paths",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#get-top-referral-sources>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get_top_referrers(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<Vec<crate::types::ReferrerTraffic>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/traffic/popular/referrers",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#get-top-referral-sources>
     */
    pub async fn get_all_top_referrers(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<Vec<crate::types::ReferrerTraffic>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/traffic/popular/referrers",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * FROM: <https://docs.github.com/rest/reference/repos#get-page-views>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per: crate::types::Per` -- Must be one of: `day`, `week`.
     */
    pub async fn get_views(
        &self,
        owner: &str,
        repo: &str,
        per: crate::types::Per,
    ) -> ClientResult<crate::types::ViewTraffic> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !per.to_string().is_empty() {
            query_args.push(("per".to_string(), per.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/traffic/views?{}",
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
     * Transfer a repository.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/transfer` endpoint.
     *
     * A transfer request will need to be accepted by the new owner when transferring a personal repository to another user. The response will contain the original `owner`, and the transfer will continue asynchronously. For more details on the requirements to transfer personal and organization-owned repositories, see [about repository transfers](https://help.github.com/articles/about-repository-transfers/).
     *
     * FROM: <https://docs.github.com/rest/reference/repos#transfer-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn transfer(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ReposTransferRequest,
    ) -> ClientResult<crate::types::MinimalRepository> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/transfer",
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
     * Check if vulnerability alerts are enabled for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/vulnerability-alerts` endpoint.
     *
     * Shows whether dependency alerts are enabled or disabled for a repository. The authenticated user must have admin access to the repository. For more information, see "[About security alerts for vulnerable dependencies](https://help.github.com/en/articles/about-security-alerts-for-vulnerable-dependencies)".
     *
     * FROM: <https://docs.github.com/rest/reference/repos#check-if-vulnerability-alerts-are-enabled-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn check_vulnerability_alerts(&self, owner: &str, repo: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/vulnerability-alerts",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Enables dependency alerts and the dependency graph for a repository. The authenticated user must have admin access to the repository. For more information, see "[About security alerts for vulnerable dependencies](https://help.github.com/en/articles/about-security-alerts-for-vulnerable-dependencies)".
     *
     * FROM: <https://docs.github.com/rest/reference/repos#enable-vulnerability-alerts>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn enable_vulnerability_alerts(&self, owner: &str, repo: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/vulnerability-alerts",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Disables dependency alerts and the dependency graph for a repository. The authenticated user must have admin access to the repository. For more information, see "[About security alerts for vulnerable dependencies](https://help.github.com/en/articles/about-security-alerts-for-vulnerable-dependencies)".
     *
     * FROM: <https://docs.github.com/rest/reference/repos#disable-vulnerability-alerts>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn disable_vulnerability_alerts(&self, owner: &str, repo: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/vulnerability-alerts",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * `master`) will be used. Please make sure your HTTP framework is configured to follow redirects or you will need to use
     * the `Location` header to make a second `GET` request.
     * **Note**: For private repositories, these links are temporary and expire after five minutes.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#download-a-repository-archive>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `ref_: &str`
     */
    pub async fn download_zipball_archive(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/zipball/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(ref_),
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
     * Creates a new repository using a repository template. Use the `template_owner` and `template_repo` route parameters to specify the repository to use as the template. The authenticated user must own or be a member of an organization that owns the repository. To check if a repository is available to use as a template, get the repository's information using the [Get a repository](https://docs.github.com/rest/reference/repos#get-a-repository) endpoint and check that the `is_template` key is `true`.
     *
     * **OAuth scope requirements**
     *
     * When using [OAuth](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/), authorizations must include:
     *
     * *   `public_repo` scope or `repo` scope to create a public repository. Note: For GitHub AE, use `repo` scope to create an internal repository.
     * *   `repo` scope to create a private repository
     *
     * FROM: <https://docs.github.com/rest/reference/repos#create-a-repository-using-a-template>
     *
     * **Parameters:**
     *
     * * `template_owner: &str`
     * * `template_repo: &str`
     */
    pub async fn create_using_template(
        &self,
        template_owner: &str,
        template_repo: &str,
        body: &crate::types::ReposCreateUsingTemplateRequest,
    ) -> ClientResult<crate::types::Repository> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/generate",
                crate::progenitor_support::encode_path(template_owner),
                crate::progenitor_support::encode_path(template_repo),
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
     * - Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/overview/resources-in-the-rest-api#link-header) to get the URL for the next page of repositories.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-public-repositories>
     *
     * **Parameters:**
     *
     * * `since: i64` -- A repository ID. Only return repositories with an ID greater than this ID.
     */
    pub async fn list_public(
        &self,
        since: i64,
    ) -> ClientResult<Vec<crate::types::MinimalRepository>> {
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
     * - Pagination is powered exclusively by the `since` parameter. Use the [Link header](https://docs.github.com/rest/overview/resources-in-the-rest-api#link-header) to get the URL for the next page of repositories.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-public-repositories>
     */
    pub async fn list_all_public(
        &self,
        since: i64,
    ) -> ClientResult<Vec<crate::types::MinimalRepository>> {
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
     * FROM: <https://docs.github.com/rest/reference/repos#list-repositories-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `visibility: crate::types::ReposListVisibility` -- Can be one of `all`, `public`, or `private`. Note: For GitHub AE, can be one of `all`, `internal`, or `private`.
     * * `affiliation: &str` -- Comma-separated list of values. Can include:  
     *   \* `owner`: Repositories that are owned by the authenticated user.  
     *   \* `collaborator`: Repositories that the user has been added to as a collaborator.  
     *   \* `organization_member`: Repositories that the user has access to through being a member of an organization. This includes every repository on every team that the user is on.
     * * `type_: crate::types::ReposListType` -- Can be one of `all`, `owner`, `public`, `private`, `member`. Note: For GitHub AE, can be one of `all`, `owner`, `internal`, `private`, `member`. Default: `all`  
     *    
     *  Will cause a `422` error if used in the same request as \*\*visibility\*\* or \*\*affiliation\*\*. Will cause a `422` error if used in the same request as \*\*visibility\*\* or \*\*affiliation\*\*.
     * * `sort: crate::types::ReposListOrgSort` -- Can be one of `created`, `updated`, `pushed`, `full_name`.
     * * `direction: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `before: chrono::DateTime<chrono::Utc>` -- Only show notifications updated before the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
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
    ) -> ClientResult<Vec<crate::types::Repository>> {
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
     * FROM: <https://docs.github.com/rest/reference/repos#list-repositories-for-the-authenticated-user>
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
    ) -> ClientResult<Vec<crate::types::Repository>> {
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
     * **OAuth scope requirements**
     *
     * When using [OAuth](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/), authorizations must include:
     *
     * *   `public_repo` scope or `repo` scope to create a public repository. Note: For GitHub AE, use `repo` scope to create an internal repository.
     * *   `repo` scope to create a private repository.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#create-a-repository-for-the-authenticated-user>
     */
    pub async fn create_for_authenticated_user(
        &self,
        body: &crate::types::ReposCreateRequest,
    ) -> ClientResult<crate::types::Repository> {
        let url = self.client.url("/user/repos", None);
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
     * FROM: <https://docs.github.com/rest/reference/repos#list-repository-invitations-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_invitations_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<Vec<crate::types::RepositoryInvitation>> {
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
     * FROM: <https://docs.github.com/rest/reference/repos#list-repository-invitations-for-the-authenticated-user>
     */
    pub async fn list_all_invitations_for_authenticated_user(
        &self,
    ) -> ClientResult<Vec<crate::types::RepositoryInvitation>> {
        let url = self.client.url("/user/repository_invitations", None);
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
     * FROM: <https://docs.github.com/rest/reference/repos#decline-a-repository-invitation>
     *
     * **Parameters:**
     *
     * * `invitation_id: i64` -- invitation_id parameter.
     */
    pub async fn decline_invitation(&self, invitation_id: i64) -> ClientResult<()> {
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
     * FROM: <https://docs.github.com/rest/reference/repos#accept-a-repository-invitation>
     *
     * **Parameters:**
     *
     * * `invitation_id: i64` -- invitation_id parameter.
     */
    pub async fn accept_invitation(&self, invitation_id: i64) -> ClientResult<()> {
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
     * Lists public repositories for the specified user. Note: For GitHub AE, this endpoint will list internal repositories for the specified user.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-repositories-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `type_: crate::types::ReposListUserType` -- Can be one of `all`, `owner`, `member`.
     * * `sort: crate::types::ReposListOrgSort` -- Can be one of `created`, `updated`, `pushed`, `full_name`.
     * * `direction: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_user(
        &self,
        username: &str,
        type_: crate::types::ReposListUserType,
        sort: crate::types::ReposListOrgSort,
        direction: crate::types::Order,
        per_page: i64,
        page: i64,
    ) -> ClientResult<Vec<crate::types::MinimalRepository>> {
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
                crate::progenitor_support::encode_path(username),
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
     * Lists public repositories for the specified user. Note: For GitHub AE, this endpoint will list internal repositories for the specified user.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-repositories-for-a-user>
     */
    pub async fn list_all_for_user(
        &self,
        username: &str,
        type_: crate::types::ReposListUserType,
        sort: crate::types::ReposListOrgSort,
        direction: crate::types::Order,
    ) -> ClientResult<Vec<crate::types::MinimalRepository>> {
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
                crate::progenitor_support::encode_path(username),
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
