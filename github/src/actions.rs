use crate::Client;
use crate::ClientResult;

pub struct Actions {
    pub client: Client,
}

impl Actions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Actions { client }
    }

    /**
     * Get GitHub Actions permissions for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/permissions` endpoint.
     *
     * Gets the GitHub Actions permissions policy for repositories and allowed actions in an organization.
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `administration` organization permission to use this API.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-github-actions-permissions-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn get_github_actions_permissions_organization(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsOrganizationPermissions>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions",
                crate::progenitor_support::encode_path(org),
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
     * Set GitHub Actions permissions for an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/permissions` endpoint.
     *
     * Sets the GitHub Actions permissions policy for repositories and allowed actions in an organization.
     *
     * If the organization belongs to an enterprise that has set restrictive permissions at the enterprise level, such as `allowed_actions` to `selected` actions, then you cannot override them for the organization.
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `administration` organization permission to use this API.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#set-github-actions-permissions-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn set_github_actions_permissions_organization(
        &self,
        org: &str,
        body: &crate::types::ActionsSetGithubPermissionsOrganizationRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions",
                crate::progenitor_support::encode_path(org),
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
     * List selected repositories enabled for GitHub Actions in an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/permissions/repositories` endpoint.
     *
     * Lists the selected repositories that are enabled for GitHub Actions in an organization. To use this endpoint, the organization permission policy for `enabled_repositories` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `administration` organization permission to use this API.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#list-selected-repositories-enabled-for-github-actions-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_selected_repositories_enabled_github_actions_organization(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<
        crate::Response<
            crate::types::ActionsListSelectedRepositoriesEnabledGithubOrganizationResponse,
        >,
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
                "/orgs/{}/actions/permissions/repositories?{}",
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
     * Set selected repositories enabled for GitHub Actions in an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/permissions/repositories` endpoint.
     *
     * Replaces the list of selected repositories that are enabled for GitHub Actions in an organization. To use this endpoint, the organization permission policy for `enabled_repositories` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `administration` organization permission to use this API.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#set-selected-repositories-enabled-for-github-actions-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn set_selected_repositories_enabled_github_actions_organization(
        &self,
        org: &str,
        body: &crate::types::ActionsSetRepoAccessSelfHostedRunnerGroupInOrgRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/repositories",
                crate::progenitor_support::encode_path(org),
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
     * Enable a selected repository for GitHub Actions in an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/permissions/repositories/{repository_id}` endpoint.
     *
     * Adds a repository to the list of selected repositories that are enabled for GitHub Actions in an organization. To use this endpoint, the organization permission policy for `enabled_repositories` must be must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `administration` organization permission to use this API.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#enable-a-selected-repository-for-github-actions-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `repository_id: i64`
     */
    pub async fn enable_selected_repository_github_actions_organization(
        &self,
        org: &str,
        repository_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/repositories/{}",
                crate::progenitor_support::encode_path(org),
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
     * Disable a selected repository for GitHub Actions in an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/actions/permissions/repositories/{repository_id}` endpoint.
     *
     * Removes a repository from the list of selected repositories that are enabled for GitHub Actions in an organization. To use this endpoint, the organization permission policy for `enabled_repositories` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `administration` organization permission to use this API.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#disable-a-selected-repository-for-github-actions-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `repository_id: i64`
     */
    pub async fn disable_selected_repository_github_actions_organization(
        &self,
        org: &str,
        repository_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/repositories/{}",
                crate::progenitor_support::encode_path(org),
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
     * Get allowed actions for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/permissions/selected-actions` endpoint.
     *
     * Gets the selected actions that are allowed in an organization. To use this endpoint, the organization permission policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization).""
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `administration` organization permission to use this API.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-allowed-actions-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn get_allowed_actions_organization(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::SelectedActions>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/selected-actions",
                crate::progenitor_support::encode_path(org),
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
     * Set allowed actions for an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/permissions/selected-actions` endpoint.
     *
     * Sets the actions that are allowed in an organization. To use this endpoint, the organization permission policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
     *
     * If the organization belongs to an enterprise that has `selected` actions set at the enterprise level, then you cannot override any of the enterprise's allowed actions settings.
     *
     * To use the `patterns_allowed` setting for private repositories, the organization must belong to an enterprise. If the organization does not belong to an enterprise, then the `patterns_allowed` setting only applies to public repositories in the organization.
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `administration` organization permission to use this API.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#set-allowed-actions-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn set_allowed_actions_organization(
        &self,
        org: &str,
        body: &crate::types::SelectedActions,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/selected-actions",
                crate::progenitor_support::encode_path(org),
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
     * List self-hosted runner groups for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/runner-groups` endpoint.
     *
     * The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
     *
     * Lists all self-hosted runner groups configured in an organization and inherited from an enterprise.
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#list-self-hosted-runner-groups-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_self_hosted_runner_groups_for_org(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsListSelfHostedRunnerGroupsOrgResponse>>
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
                "/orgs/{}/actions/runner-groups?{}",
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
     * Create a self-hosted runner group for an organization.
     *
     * This function performs a `POST` to the `/orgs/{org}/actions/runner-groups` endpoint.
     *
     * The self-hosted runner groups REST API is available with GitHub Enterprise Cloud and GitHub Enterprise Server. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
     *
     * Creates a new self-hosted runner group for an organization.
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#create-a-self-hosted-runner-group-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn create_self_hosted_runner_group_for_org(
        &self,
        org: &str,
        body: &crate::types::ActionsCreateSelfHostedRunnerGroupOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::RunnerGroupsOrg>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runner-groups",
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
     * Get a self-hosted runner group for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/runner-groups/{runner_group_id}` endpoint.
     *
     * The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
     *
     * Gets a specific self-hosted runner group for an organization.
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-a-self-hosted-runner-group-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     */
    pub async fn get_self_hosted_runner_group_for_org(
        &self,
        org: &str,
        runner_group_id: i64,
    ) -> ClientResult<crate::Response<crate::types::RunnerGroupsOrg>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runner-groups/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&runner_group_id.to_string()),
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
     * Delete a self-hosted runner group from an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/actions/runner-groups/{runner_group_id}` endpoint.
     *
     * The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
     *
     * Deletes a self-hosted runner group for an organization.
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#delete-a-self-hosted-runner-group-from-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     */
    pub async fn delete_self_hosted_runner_group_from_org(
        &self,
        org: &str,
        runner_group_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runner-groups/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&runner_group_id.to_string()),
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
     * Update a self-hosted runner group for an organization.
     *
     * This function performs a `PATCH` to the `/orgs/{org}/actions/runner-groups/{runner_group_id}` endpoint.
     *
     * The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
     *
     * Updates the `name` and `visibility` of a self-hosted runner group in an organization.
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#update-a-self-hosted-runner-group-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     */
    pub async fn update_self_hosted_runner_group_for_org(
        &self,
        org: &str,
        runner_group_id: i64,
        body: &crate::types::ActionsUpdateSelfHostedRunnerGroupOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::RunnerGroupsOrg>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runner-groups/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&runner_group_id.to_string()),
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
     * List repository access to a self-hosted runner group in an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/runner-groups/{runner_group_id}/repositories` endpoint.
     *
     * The self-hosted runner groups REST API is available with GitHub Enterprise Cloud and GitHub Enterprise Server. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
     *
     * Lists the repositories with access to a self-hosted runner group configured in an organization.
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#list-repository-access-to-a-self-hosted-runner-group-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     * * `page: i64` -- Page number of the results to fetch.
     * * `per_page: i64` -- Results per page (max 100).
     */
    pub async fn list_repo_access_to_self_hosted_runner_group_in_org(
        &self,
        org: &str,
        runner_group_id: i64,
        page: i64,
        per_page: i64,
    ) -> ClientResult<
        crate::Response<crate::types::ActionsListRepoAccessSelfHostedRunnerGroupInOrgResponse>,
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
                "/orgs/{}/actions/runner-groups/{}/repositories?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&runner_group_id.to_string()),
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
     * Set repository access for a self-hosted runner group in an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/runner-groups/{runner_group_id}/repositories` endpoint.
     *
     * The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
     *
     * Replaces the list of repositories that have access to a self-hosted runner group configured in an organization.
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#set-repository-access-to-a-self-hosted-runner-group-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     */
    pub async fn set_repo_access_to_self_hosted_runner_group_in_org(
        &self,
        org: &str,
        runner_group_id: i64,
        body: &crate::types::ActionsSetRepoAccessSelfHostedRunnerGroupInOrgRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runner-groups/{}/repositories",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&runner_group_id.to_string()),
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
     * Add repository access to a self-hosted runner group in an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/runner-groups/{runner_group_id}/repositories/{repository_id}` endpoint.
     *
     * The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
     *
     *
     * Adds a repository to the list of selected repositories that can access a self-hosted runner group. The runner group must have `visibility` set to `selected`. For more information, see "[Create a self-hosted runner group for an organization](#create-a-self-hosted-runner-group-for-an-organization)."
     *
     * You must authenticate using an access token with the `admin:org`
     * scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#add-repository-acess-to-a-self-hosted-runner-group-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     * * `repository_id: i64`
     */
    pub async fn add_repo_access_to_self_hosted_runner_group_in_org(
        &self,
        org: &str,
        runner_group_id: i64,
        repository_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runner-groups/{}/repositories/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&runner_group_id.to_string()),
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
     * Remove repository access to a self-hosted runner group in an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/actions/runner-groups/{runner_group_id}/repositories/{repository_id}` endpoint.
     *
     * The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
     *
     *
     * Removes a repository from the list of selected repositories that can access a self-hosted runner group. The runner group must have `visibility` set to `selected`. For more information, see "[Create a self-hosted runner group for an organization](#create-a-self-hosted-runner-group-for-an-organization)."
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#remove-repository-access-to-a-self-hosted-runner-group-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     * * `repository_id: i64`
     */
    pub async fn remove_repo_access_to_self_hosted_runner_group_in_org(
        &self,
        org: &str,
        runner_group_id: i64,
        repository_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runner-groups/{}/repositories/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&runner_group_id.to_string()),
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
     * List self-hosted runners in a group for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/runner-groups/{runner_group_id}/runners` endpoint.
     *
     * The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
     *
     * Lists self-hosted runners that are in a specific organization group.
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#list-self-hosted-runners-in-a-group-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_self_hosted_runners_in_group_for_org(
        &self,
        org: &str,
        runner_group_id: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsListSelfHostedRunnersInGroupOrgResponse>>
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
                "/orgs/{}/actions/runner-groups/{}/runners?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&runner_group_id.to_string()),
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
     * Set self-hosted runners in a group for an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/runner-groups/{runner_group_id}/runners` endpoint.
     *
     * The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
     *
     * Replaces the list of self-hosted runners that are part of an organization runner group.
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#set-self-hosted-runners-in-a-group-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     */
    pub async fn set_self_hosted_runners_in_group_for_org(
        &self,
        org: &str,
        runner_group_id: i64,
        body: &crate::types::ActionsSetSelfHostedRunnersInGroupOrgRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runner-groups/{}/runners",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&runner_group_id.to_string()),
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
     * Add a self-hosted runner to a group for an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/runner-groups/{runner_group_id}/runners/{runner_id}` endpoint.
     *
     * The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
     *
     *
     * Adds a self-hosted runner to a runner group configured in an organization.
     *
     * You must authenticate using an access token with the `admin:org`
     * scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#add-a-self-hosted-runner-to-a-group-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     * * `runner_id: i64` -- Unique identifier of the self-hosted runner.
     */
    pub async fn add_self_hosted_runner_to_group_for_org(
        &self,
        org: &str,
        runner_group_id: i64,
        runner_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runner-groups/{}/runners/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&runner_group_id.to_string()),
                crate::progenitor_support::encode_path(&runner_id.to_string()),
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
     * Remove a self-hosted runner from a group for an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/actions/runner-groups/{runner_group_id}/runners/{runner_id}` endpoint.
     *
     * The self-hosted runner groups REST API is available with GitHub Enterprise Cloud. For more information, see "[GitHub's products](https://docs.github.com/github/getting-started-with-github/githubs-products)."
     *
     *
     * Removes a self-hosted runner from a group configured in an organization. The runner is then returned to the default group.
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#remove-a-self-hosted-runner-from-a-group-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     * * `runner_id: i64` -- Unique identifier of the self-hosted runner.
     */
    pub async fn remove_self_hosted_runner_from_group_for_org(
        &self,
        org: &str,
        runner_group_id: i64,
        runner_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runner-groups/{}/runners/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&runner_group_id.to_string()),
                crate::progenitor_support::encode_path(&runner_id.to_string()),
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
     * List self-hosted runners for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/runners` endpoint.
     *
     * Lists all self-hosted runners configured in an organization.
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#list-self-hosted-runners-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_self_hosted_runners_for_org(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsListSelfHostedRunnersOrgResponse>> {
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
                "/orgs/{}/actions/runners?{}",
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
     * List runner applications for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/runners/downloads` endpoint.
     *
     * Lists binaries for the runner application that you can download and run.
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#list-runner-applications-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn list_runner_applications_for_org(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::RunnerApplication>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runners/downloads",
                crate::progenitor_support::encode_path(org),
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
     * List runner applications for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/runners/downloads` endpoint.
     *
     * As opposed to `list_runner_applications_for_org`, this function returns all the pages of the request at once.
     *
     * Lists binaries for the runner application that you can download and run.
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#list-runner-applications-for-an-organization>
     */
    pub async fn list_all_runner_applications_for_org(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::RunnerApplication>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runners/downloads",
                crate::progenitor_support::encode_path(org),
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
     * Create a registration token for an organization.
     *
     * This function performs a `POST` to the `/orgs/{org}/actions/runners/registration-token` endpoint.
     *
     * Returns a token that you can pass to the `config` script. The token expires after one hour.
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint.
     *
     * #### Example using registration token
     *
     * Configure your self-hosted runner, replacing `TOKEN` with the registration token provided by this endpoint.
     *
     * ```
     * ./config.sh --url https://github.com/octo-org --token TOKEN
     * ```
     *
     * FROM: <https://docs.github.com/rest/reference/actions#create-a-registration-token-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn create_registration_token_for_org(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::AuthenticationToken>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runners/registration-token",
                crate::progenitor_support::encode_path(org),
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
     * Create a remove token for an organization.
     *
     * This function performs a `POST` to the `/orgs/{org}/actions/runners/remove-token` endpoint.
     *
     * Returns a token that you can pass to the `config` script to remove a self-hosted runner from an organization. The token expires after one hour.
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint.
     *
     * #### Example using remove token
     *
     * To remove your self-hosted runner from an organization, replace `TOKEN` with the remove token provided by this
     * endpoint.
     *
     * ```
     * ./config.sh remove --token TOKEN
     * ```
     *
     * FROM: <https://docs.github.com/rest/reference/actions#create-a-remove-token-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn create_remove_token_for_org(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::AuthenticationToken>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runners/remove-token",
                crate::progenitor_support::encode_path(org),
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
     * Get a self-hosted runner for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/runners/{runner_id}` endpoint.
     *
     * Gets a specific self-hosted runner configured in an organization.
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-a-self-hosted-runner-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `runner_id: i64` -- Unique identifier of the self-hosted runner.
     */
    pub async fn get_self_hosted_runner_for_org(
        &self,
        org: &str,
        runner_id: i64,
    ) -> ClientResult<crate::Response<crate::types::Runner>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runners/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&runner_id.to_string()),
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
     * Delete a self-hosted runner from an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/actions/runners/{runner_id}` endpoint.
     *
     * Forces the removal of a self-hosted runner from an organization. You can use this endpoint to completely remove the runner when the machine you were using no longer exists.
     *
     * You must authenticate using an access token with the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#delete-a-self-hosted-runner-from-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `runner_id: i64` -- Unique identifier of the self-hosted runner.
     */
    pub async fn delete_self_hosted_runner_from_org(
        &self,
        org: &str,
        runner_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runners/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&runner_id.to_string()),
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
     * List organization secrets.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/secrets` endpoint.
     *
     * Lists all secrets available in an organization without revealing their encrypted values. You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#list-organization-secrets>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_org_secrets(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsListOrgSecretsResponse>> {
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
                "/orgs/{}/actions/secrets?{}",
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
     * Get an organization public key.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/secrets/public-key` endpoint.
     *
     * Gets your public key, which you need to encrypt secrets. You need to encrypt a secret before you can create or update secrets. You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-an-organization-public-key>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn get_org_public_key(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsPublicKey>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/secrets/public-key",
                crate::progenitor_support::encode_path(org),
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
     * This function performs a `GET` to the `/orgs/{org}/actions/secrets/{secret_name}` endpoint.
     *
     * Gets a single organization secret without revealing its encrypted value. You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-an-organization-secret>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `secret_name: &str` -- secret_name parameter.
     */
    pub async fn get_org_secret(
        &self,
        org: &str,
        secret_name: &str,
    ) -> ClientResult<crate::Response<crate::types::OrganizationActionsSecret>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/secrets/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(secret_name),
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
     * This function performs a `PUT` to the `/orgs/{org}/actions/secrets/{secret_name}` endpoint.
     *
     * Creates or updates an organization secret with an encrypted value. Encrypt your secret using
     * [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). You must authenticate using an access
     * token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to
     * use this endpoint.
     *
     * #### Example encrypting a secret using Node.js
     *
     * Encrypt your secret using the [tweetsodium](https://github.com/github/tweetsodium) library.
     *
     * ```
     * const sodium = require('tweetsodium');
     *
     * const key = "base64-encoded-public-key";
     * const value = "plain-text-secret";
     *
     * // Convert the message and key to Uint8Array's (Buffer implements that interface)
     * const messageBytes = Buffer.from(value);
     * const keyBytes = Buffer.from(key, 'base64');
     *
     * // Encrypt using LibSodium.
     * const encryptedBytes = sodium.seal(messageBytes, keyBytes);
     *
     * // Base64 the encrypted secret
     * const encrypted = Buffer.from(encryptedBytes).toString('base64');
     *
     * console.log(encrypted);
     * ```
     *
     *
     * #### Example encrypting a secret using Python
     *
     * Encrypt your secret using [pynacl](https://pynacl.readthedocs.io/en/stable/public/#nacl-public-sealedbox) with Python 3.
     *
     * ```
     * from base64 import b64encode
     * from nacl import encoding, public
     *
     * def encrypt(public_key: str, secret_value: str) -> str:
     *   """Encrypt a Unicode string using the public key."""
     *   public_key = public.PublicKey(public_key.encode("utf-8"), encoding.Base64Encoder())
     *   sealed_box = public.SealedBox(public_key)
     *   encrypted = sealed_box.encrypt(secret_value.encode("utf-8"))
     *   return b64encode(encrypted).decode("utf-8")
     * ```
     *
     * #### Example encrypting a secret using C#
     *
     * Encrypt your secret using the [Sodium.Core](https://www.nuget.org/packages/Sodium.Core/) package.
     *
     * ```
     * var secretValue = System.Text.Encoding.UTF8.GetBytes("mySecret");
     * var publicKey = Convert.FromBase64String("2Sg8iYjAxxmI2LvUXpJjkYrMxURPc8r+dB7TJyvvcCU=");
     *
     * var sealedPublicKeyBox = Sodium.SealedPublicKeyBox.Create(secretValue, publicKey);
     *
     * Console.WriteLine(Convert.ToBase64String(sealedPublicKeyBox));
     * ```
     *
     * #### Example encrypting a secret using Ruby
     *
     * Encrypt your secret using the [rbnacl](https://github.com/RubyCrypto/rbnacl) gem.
     *
     * ```ruby
     * require "rbnacl"
     * require "base64"
     *
     * key = Base64.decode64("+ZYvJDZMHUfBkJdyq5Zm9SKqeuBQ4sj+6sfjlH4CgG0=")
     * public_key = RbNaCl::PublicKey.new(key)
     *
     * box = RbNaCl::Boxes::Sealed.from_public_key(public_key)
     * encrypted_secret = box.encrypt("my_secret")
     *
     * # Print the base64 encoded secret
     * puts Base64.strict_encode64(encrypted_secret)
     * ```
     *
     * FROM: <https://docs.github.com/rest/reference/actions#create-or-update-an-organization-secret>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `secret_name: &str` -- secret_name parameter.
     */
    pub async fn create_or_update_org_secret(
        &self,
        org: &str,
        secret_name: &str,
        body: &crate::types::ActionsCreateUpdateOrgSecretRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/secrets/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(secret_name),
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
     * This function performs a `DELETE` to the `/orgs/{org}/actions/secrets/{secret_name}` endpoint.
     *
     * Deletes a secret in an organization using the secret name. You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#delete-an-organization-secret>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `secret_name: &str` -- secret_name parameter.
     */
    pub async fn delete_org_secret(
        &self,
        org: &str,
        secret_name: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/secrets/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(secret_name),
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
     * This function performs a `GET` to the `/orgs/{org}/actions/secrets/{secret_name}/repositories` endpoint.
     *
     * Lists all repositories that have been selected when the `visibility` for repository access to a secret is set to `selected`. You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#list-selected-repositories-for-an-organization-secret>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `secret_name: &str` -- secret_name parameter.
     * * `page: i64` -- Page number of the results to fetch.
     * * `per_page: i64` -- Results per page (max 100).
     */
    pub async fn list_selected_repos_for_org_secret(
        &self,
        org: &str,
        secret_name: &str,
        page: i64,
        per_page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsListSelectedReposOrgSecretResponse>>
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
                "/orgs/{}/actions/secrets/{}/repositories?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(secret_name),
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
     * This function performs a `PUT` to the `/orgs/{org}/actions/secrets/{secret_name}/repositories` endpoint.
     *
     * Replaces all repositories for an organization secret when the `visibility` for repository access is set to `selected`. The visibility is set when you [Create or update an organization secret](https://docs.github.com/rest/reference/actions#create-or-update-an-organization-secret). You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#set-selected-repositories-for-an-organization-secret>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `secret_name: &str` -- secret_name parameter.
     */
    pub async fn set_selected_repos_for_org_secret(
        &self,
        org: &str,
        secret_name: &str,
        body: &crate::types::ActionsSetSelectedReposOrgSecretRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/secrets/{}/repositories",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(secret_name),
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
     * This function performs a `PUT` to the `/orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id}` endpoint.
     *
     * Adds a repository to an organization secret when the `visibility` for repository access is set to `selected`. The visibility is set when you [Create or update an organization secret](https://docs.github.com/rest/reference/actions#create-or-update-an-organization-secret). You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#add-selected-repository-to-an-organization-secret>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `secret_name: &str` -- secret_name parameter.
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
                "/orgs/{}/actions/secrets/{}/repositories/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(secret_name),
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
     * This function performs a `DELETE` to the `/orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id}` endpoint.
     *
     * Removes a repository from an organization secret when the `visibility` for repository access is set to `selected`. The visibility is set when you [Create or update an organization secret](https://docs.github.com/rest/reference/actions#create-or-update-an-organization-secret). You must authenticate using an access token with the `admin:org` scope to use this endpoint. GitHub Apps must have the `secrets` organization permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#remove-selected-repository-from-an-organization-secret>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `secret_name: &str` -- secret_name parameter.
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
                "/orgs/{}/actions/secrets/{}/repositories/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(secret_name),
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
     * List artifacts for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/artifacts` endpoint.
     *
     * Lists all artifacts for a repository. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#list-artifacts-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_artifacts_for_repo(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsListArtifactsRepoResponse>> {
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
                "/repos/{}/{}/actions/artifacts?{}",
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
     * Get an artifact.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/artifacts/{artifact_id}` endpoint.
     *
     * Gets a specific artifact for a workflow run. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-an-artifact>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `artifact_id: i64` -- artifact_id parameter.
     */
    pub async fn get_artifact(
        &self,
        owner: &str,
        repo: &str,
        artifact_id: i64,
    ) -> ClientResult<crate::Response<crate::types::Artifact>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/artifacts/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&artifact_id.to_string()),
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
     * Delete an artifact.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/actions/artifacts/{artifact_id}` endpoint.
     *
     * Deletes an artifact for a workflow run. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `actions:write` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#delete-an-artifact>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `artifact_id: i64` -- artifact_id parameter.
     */
    pub async fn delete_artifact(
        &self,
        owner: &str,
        repo: &str,
        artifact_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/artifacts/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&artifact_id.to_string()),
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
     * Download an artifact.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/artifacts/{artifact_id}/{archive_format}` endpoint.
     *
     * Gets a redirect URL to download an archive for a repository. This URL expires after 1 minute. Look for `Location:` in
     * the response header to find the URL for the download. The `:archive_format` must be `zip`. Anyone with read access to
     * the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope.
     * GitHub Apps must have the `actions:read` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#download-an-artifact>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `artifact_id: i64` -- artifact_id parameter.
     * * `archive_format: &str`
     */
    pub async fn download_artifact(
        &self,
        owner: &str,
        repo: &str,
        artifact_id: i64,
        archive_format: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/artifacts/{}/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&artifact_id.to_string()),
                crate::progenitor_support::encode_path(archive_format),
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
     * Get a job for a workflow run.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/jobs/{job_id}` endpoint.
     *
     * Gets a specific job in a workflow run. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-a-job-for-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `job_id: i64` -- job_id parameter.
     */
    pub async fn get_job_for_workflow_run(
        &self,
        owner: &str,
        repo: &str,
        job_id: i64,
    ) -> ClientResult<crate::Response<crate::types::Job>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/jobs/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&job_id.to_string()),
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
     * Download job logs for a workflow run.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/jobs/{job_id}/logs` endpoint.
     *
     * Gets a redirect URL to download a plain text file of logs for a workflow job. This link expires after 1 minute. Look
     * for `Location:` in the response header to find the URL for the download. Anyone with read access to the repository can
     * use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must
     * have the `actions:read` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#download-job-logs-for-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `job_id: i64` -- job_id parameter.
     */
    pub async fn download_job_logs_for_workflow_run(
        &self,
        owner: &str,
        repo: &str,
        job_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/jobs/{}/logs",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&job_id.to_string()),
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
     * Get GitHub Actions permissions for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/permissions` endpoint.
     *
     * Gets the GitHub Actions permissions policy for a repository, including whether GitHub Actions is enabled and the actions allowed to run in the repository.
     *
     * You must authenticate using an access token with the `repo` scope to use this
     * endpoint. GitHub Apps must have the `administration` repository permission to use this API.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-github-actions-permissions-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get_github_actions_permissions_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsRepositoryPermissions>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/permissions",
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
     * Set GitHub Actions permissions for a repository.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/actions/permissions` endpoint.
     *
     * Sets the GitHub Actions permissions policy for enabling GitHub Actions and allowed actions in the repository.
     *
     * If the repository belongs to an organization or enterprise that has set restrictive permissions at the organization or enterprise levels, such as `allowed_actions` to `selected` actions, then you cannot override them for the repository.
     *
     * You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `administration` repository permission to use this API.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#set-github-actions-permissions-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn set_github_actions_permissions_repository(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ActionsSetGithubPermissionsRepositoryRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/permissions",
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
     * Get allowed actions for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/permissions/selected-actions` endpoint.
     *
     * Gets the settings for selected actions that are allowed in a repository. To use this endpoint, the repository policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for a repository](#set-github-actions-permissions-for-a-repository)."
     *
     * You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `administration` repository permission to use this API.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-allowed-actions-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get_allowed_actions_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::SelectedActions>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/permissions/selected-actions",
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
     * Set allowed actions for a repository.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/actions/permissions/selected-actions` endpoint.
     *
     * Sets the actions that are allowed in a repository. To use this endpoint, the repository permission policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for a repository](#set-github-actions-permissions-for-a-repository)."
     *
     * If the repository belongs to an organization or enterprise that has `selected` actions set at the organization or enterprise levels, then you cannot override any of the allowed actions settings.
     *
     * To use the `patterns_allowed` setting for private repositories, the repository must belong to an enterprise. If the repository does not belong to an enterprise, then the `patterns_allowed` setting only applies to public repositories.
     *
     * You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `administration` repository permission to use this API.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#set-allowed-actions-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn set_allowed_actions_repository(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::SelectedActions,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/permissions/selected-actions",
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
     * List self-hosted runners for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runners` endpoint.
     *
     * Lists all self-hosted runners configured in a repository. You must authenticate using an access token with the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#list-self-hosted-runners-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_self_hosted_runners_for_repo(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsListSelfHostedRunnersOrgResponse>> {
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
                "/repos/{}/{}/actions/runners?{}",
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
     * List runner applications for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runners/downloads` endpoint.
     *
     * Lists binaries for the runner application that you can download and run.
     *
     * You must authenticate using an access token with the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#list-runner-applications-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn list_runner_applications_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::RunnerApplication>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runners/downloads",
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
     * List runner applications for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runners/downloads` endpoint.
     *
     * As opposed to `list_runner_applications_for_repo`, this function returns all the pages of the request at once.
     *
     * Lists binaries for the runner application that you can download and run.
     *
     * You must authenticate using an access token with the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#list-runner-applications-for-a-repository>
     */
    pub async fn list_all_runner_applications_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::RunnerApplication>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runners/downloads",
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
     * Create a registration token for a repository.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/actions/runners/registration-token` endpoint.
     *
     * Returns a token that you can pass to the `config` script. The token expires after one hour. You must authenticate
     * using an access token with the `repo` scope to use this endpoint.
     *
     * #### Example using registration token
     *  
     * Configure your self-hosted runner, replacing `TOKEN` with the registration token provided by this endpoint.
     *
     * ```
     * ./config.sh --url https://github.com/octo-org/octo-repo-artifacts --token TOKEN
     * ```
     *
     * FROM: <https://docs.github.com/rest/reference/actions#create-a-registration-token-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn create_registration_token_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::AuthenticationToken>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runners/registration-token",
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
     * Create a remove token for a repository.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/actions/runners/remove-token` endpoint.
     *
     * Returns a token that you can pass to remove a self-hosted runner from a repository. The token expires after one hour.
     * You must authenticate using an access token with the `repo` scope to use this endpoint.
     *
     * #### Example using remove token
     *  
     * To remove your self-hosted runner from a repository, replace TOKEN with the remove token provided by this endpoint.
     *
     * ```
     * ./config.sh remove --token TOKEN
     * ```
     *
     * FROM: <https://docs.github.com/rest/reference/actions#create-a-remove-token-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn create_remove_token_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::AuthenticationToken>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runners/remove-token",
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
     * Get a self-hosted runner for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runners/{runner_id}` endpoint.
     *
     * Gets a specific self-hosted runner configured in a repository.
     *
     * You must authenticate using an access token with the `repo` scope to use this
     * endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-a-self-hosted-runner-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `runner_id: i64` -- Unique identifier of the self-hosted runner.
     */
    pub async fn get_self_hosted_runner_for_repo(
        &self,
        owner: &str,
        repo: &str,
        runner_id: i64,
    ) -> ClientResult<crate::Response<crate::types::Runner>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runners/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&runner_id.to_string()),
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
     * Delete a self-hosted runner from a repository.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/actions/runners/{runner_id}` endpoint.
     *
     * Forces the removal of a self-hosted runner from a repository. You can use this endpoint to completely remove the runner when the machine you were using no longer exists.
     *
     * You must authenticate using an access token with the `repo`
     * scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#delete-a-self-hosted-runner-from-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `runner_id: i64` -- Unique identifier of the self-hosted runner.
     */
    pub async fn delete_self_hosted_runner_from_repo(
        &self,
        owner: &str,
        repo: &str,
        runner_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runners/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&runner_id.to_string()),
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
     * List workflow runs for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runs` endpoint.
     *
     * Lists all workflow runs for a repository. You can use parameters to narrow the list of results. For more information about using parameters, see [Parameters](https://docs.github.com/rest/overview/resources-in-the-rest-api#parameters).
     *
     * Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#list-workflow-runs-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `actor: &str` -- Returns someone's workflow runs. Use the login for the user who created the `push` associated with the check suite or workflow run.
     * * `branch: &str` -- Returns workflow runs associated with a branch. Use the name of the branch of the `push`.
     * * `event: &str` -- Returns workflow run triggered by the event you specify. For example, `push`, `pull_request` or `issue`. For more information, see "[Events that trigger workflows](https://help.github.com/en/actions/automating-your-workflow-with-github-actions/events-that-trigger-workflows).".
     * * `status: crate::types::WorkflowRunStatus` -- Returns workflow runs with the check run `status` or `conclusion` that you specify. For example, a conclusion can be `success` or a status can be `in_progress`. Only GitHub can set a status of `waiting` or `requested`. For a list of the possible `status` and `conclusion` options, see "[Create a check run](https://docs.github.com/rest/reference/checks#create-a-check-run).".
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     * * `created: &str`
     */
    pub async fn list_workflow_runs_for_repo(
        &self,
        owner: &str,
        repo: &str,
        actor: &str,
        branch: &str,
        event: &str,
        status: crate::types::WorkflowRunStatus,
        per_page: i64,
        page: i64,
        created: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsListWorkflowRunsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !actor.is_empty() {
            query_args.push(("actor".to_string(), actor.to_string()));
        }
        if !branch.is_empty() {
            query_args.push(("branch".to_string(), branch.to_string()));
        }
        if !created.is_empty() {
            query_args.push(("created".to_string(), created.to_string()));
        }
        if !event.is_empty() {
            query_args.push(("event".to_string(), event.to_string()));
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
                "/repos/{}/{}/actions/runs?{}",
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
     * Get a workflow run.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runs/{run_id}` endpoint.
     *
     * Gets a specific workflow run. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `run_id: i64` -- The id of the workflow run.
     */
    pub async fn get_workflow_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> ClientResult<crate::Response<crate::types::WorkflowRun>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&run_id.to_string()),
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
     * Delete a workflow run.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/actions/runs/{run_id}` endpoint.
     *
     * Delete a specific workflow run. Anyone with write access to the repository can use this endpoint. If the repository is
     * private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:write` permission to use
     * this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#delete-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `run_id: i64` -- The id of the workflow run.
     */
    pub async fn delete_workflow_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&run_id.to_string()),
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
     * Get the review history for a workflow run.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runs/{run_id}/approvals` endpoint.
     *
     * Anyone with read access to the repository can use this endpoint. If the repository is private, you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-the-review-history-for-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `run_id: i64` -- The id of the workflow run.
     */
    pub async fn get_reviews_for_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::EnvironmentApproval>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}/approvals",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&run_id.to_string()),
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
     * Get the review history for a workflow run.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runs/{run_id}/approvals` endpoint.
     *
     * As opposed to `get_reviews_for_run`, this function returns all the pages of the request at once.
     *
     * Anyone with read access to the repository can use this endpoint. If the repository is private, you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-the-review-history-for-a-workflow-run>
     */
    pub async fn get_all_reviews_for_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::EnvironmentApproval>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}/approvals",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&run_id.to_string()),
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
     * Approve a workflow run for a fork pull request.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/actions/runs/{run_id}/approve` endpoint.
     *
     * Approves a workflow run for a pull request from a public fork of a first time contributor. For more information, see ["Approving workflow runs from public forks](https://docs.github.com/actions/managing-workflow-runs/approving-workflow-runs-from-public-forks)."
     *
     * You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `actions:write` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#approve-a-workflow-run-for-a-fork-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `run_id: i64` -- The id of the workflow run.
     */
    pub async fn approve_workflow_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}/approve",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&run_id.to_string()),
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
     * List workflow run artifacts.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runs/{run_id}/artifacts` endpoint.
     *
     * Lists artifacts for a workflow run. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#list-workflow-run-artifacts>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `run_id: i64` -- The id of the workflow run.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_workflow_run_artifacts(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsListArtifactsRepoResponse>> {
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
                "/repos/{}/{}/actions/runs/{}/artifacts?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&run_id.to_string()),
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
     * Cancel a workflow run.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/actions/runs/{run_id}/cancel` endpoint.
     *
     * Cancels a workflow run using its `id`. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `actions:write` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#cancel-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `run_id: i64` -- The id of the workflow run.
     */
    pub async fn cancel_workflow_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}/cancel",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&run_id.to_string()),
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
     * List jobs for a workflow run.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runs/{run_id}/jobs` endpoint.
     *
     * Lists jobs for a workflow run. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint. You can use parameters to narrow the list of results. For more information about using parameters, see [Parameters](https://docs.github.com/rest/overview/resources-in-the-rest-api#parameters).
     *
     * FROM: <https://docs.github.com/rest/reference/actions#list-jobs-for-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `run_id: i64` -- The id of the workflow run.
     * * `filter: crate::types::ActionsListJobsWorkflowRunFilter` -- Filters jobs by their `completed_at` timestamp. Can be one of:  
     *  \\* `latest`: Returns jobs from the most recent execution of the workflow run.  
     *  \\* `all`: Returns all jobs for a workflow run, including from old executions of the workflow run.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_jobs_for_workflow_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
        filter: crate::types::ActionsListJobsWorkflowRunFilter,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsListJobsWorkflowRunResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !filter.to_string().is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
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
                "/repos/{}/{}/actions/runs/{}/jobs?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&run_id.to_string()),
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
     * Download workflow run logs.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runs/{run_id}/logs` endpoint.
     *
     * Gets a redirect URL to download an archive of log files for a workflow run. This link expires after 1 minute. Look for
     * `Location:` in the response header to find the URL for the download. Anyone with read access to the repository can use
     * this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have
     * the `actions:read` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#download-workflow-run-logs>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `run_id: i64` -- The id of the workflow run.
     */
    pub async fn download_workflow_run_logs(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}/logs",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&run_id.to_string()),
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
     * Delete workflow run logs.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/actions/runs/{run_id}/logs` endpoint.
     *
     * Deletes all logs for a workflow run. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `actions:write` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#delete-workflow-run-logs>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `run_id: i64` -- The id of the workflow run.
     */
    pub async fn delete_workflow_run_logs(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}/logs",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&run_id.to_string()),
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
     * Get pending deployments for a workflow run.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runs/{run_id}/pending_deployments` endpoint.
     *
     * Get all deployment environments for a workflow run that are waiting for protection rules to pass.
     *
     * Anyone with read access to the repository can use this endpoint. If the repository is private, you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-pending-deployments-for-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `run_id: i64` -- The id of the workflow run.
     */
    pub async fn get_pending_deployments_for_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::PendingDeployment>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}/pending_deployments",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&run_id.to_string()),
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
     * Get pending deployments for a workflow run.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runs/{run_id}/pending_deployments` endpoint.
     *
     * As opposed to `get_pending_deployments_for_run`, this function returns all the pages of the request at once.
     *
     * Get all deployment environments for a workflow run that are waiting for protection rules to pass.
     *
     * Anyone with read access to the repository can use this endpoint. If the repository is private, you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-pending-deployments-for-a-workflow-run>
     */
    pub async fn get_all_pending_deployments_for_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::PendingDeployment>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}/pending_deployments",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&run_id.to_string()),
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
     * Review pending deployments for a workflow run.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/actions/runs/{run_id}/pending_deployments` endpoint.
     *
     * Approve or reject pending deployments that are waiting on approval by a required reviewer.
     *
     * Anyone with read access to the repository contents and deployments can use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#review-pending-deployments-for-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `run_id: i64` -- The id of the workflow run.
     */
    pub async fn review_pending_deployments_for_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
        body: &crate::types::ActionsReviewPendingDeploymentsRunRequest,
    ) -> ClientResult<crate::Response<Vec<crate::types::Deployment>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}/pending_deployments",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&run_id.to_string()),
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
     * Re-run a workflow.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/actions/runs/{run_id}/rerun` endpoint.
     *
     * Re-runs your workflow run using its `id`. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `actions:write` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#re-run-a-workflow>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `run_id: i64` -- The id of the workflow run.
     */
    pub async fn re_run_workflow(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}/rerun",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&run_id.to_string()),
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
     * Re-run failed jobs from a workflow run.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/actions/runs/{run_id}/rerun-failed-jobs` endpoint.
     *
     * Re-run all of the failed jobs and their dependent jobs in a workflow run using the `id` of the workflow run. You must authenticate using an access token with the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#re-run-workflow-failed-jobs>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `run_id: i64` -- The id of the workflow run.
     */
    pub async fn re_run_workflow_failed_jobs(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}/rerun-failed-jobs",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&run_id.to_string()),
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
     * Get workflow run usage.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runs/{run_id}/timing` endpoint.
     *
     * Gets the number of billable minutes and total run time for a specific workflow run. Billable minutes only apply to workflows in private repositories that use GitHub-hosted runners. Usage is listed for each GitHub-hosted runner operating system in milliseconds. Any job re-runs are also included in the usage. The usage does not include the multiplier for macOS and Windows runners and is not rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
     *
     * Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-workflow-run-usage>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `run_id: i64` -- The id of the workflow run.
     */
    pub async fn get_workflow_run_usage(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> ClientResult<crate::Response<crate::types::WorkflowRunUsage>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}/timing",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&run_id.to_string()),
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
     * List repository secrets.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/secrets` endpoint.
     *
     * Lists all secrets available in a repository without revealing their encrypted values. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#list-repository-secrets>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_repo_secrets(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsListRepoSecretsResponse>> {
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
                "/repos/{}/{}/actions/secrets?{}",
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
     * Get a repository public key.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/secrets/public-key` endpoint.
     *
     * Gets your public key, which you need to encrypt secrets. You need to encrypt a secret before you can create or update secrets. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `secrets` repository permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-a-repository-public-key>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get_repo_public_key(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsPublicKey>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/secrets/public-key",
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
     * Get a repository secret.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/secrets/{secret_name}` endpoint.
     *
     * Gets a single repository secret without revealing its encrypted value. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-a-repository-secret>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `secret_name: &str` -- secret_name parameter.
     */
    pub async fn get_repo_secret(
        &self,
        owner: &str,
        repo: &str,
        secret_name: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsSecret>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/secrets/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(secret_name),
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
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/actions/secrets/{secret_name}` endpoint.
     *
     * Creates or updates a repository secret with an encrypted value. Encrypt your secret using
     * [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). You must authenticate using an access
     * token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use
     * this endpoint.
     *
     * #### Example encrypting a secret using Node.js
     *
     * Encrypt your secret using the [tweetsodium](https://github.com/github/tweetsodium) library.
     *
     * ```
     * const sodium = require('tweetsodium');
     *
     * const key = "base64-encoded-public-key";
     * const value = "plain-text-secret";
     *
     * // Convert the message and key to Uint8Array's (Buffer implements that interface)
     * const messageBytes = Buffer.from(value);
     * const keyBytes = Buffer.from(key, 'base64');
     *
     * // Encrypt using LibSodium.
     * const encryptedBytes = sodium.seal(messageBytes, keyBytes);
     *
     * // Base64 the encrypted secret
     * const encrypted = Buffer.from(encryptedBytes).toString('base64');
     *
     * console.log(encrypted);
     * ```
     *
     *
     * #### Example encrypting a secret using Python
     *
     * Encrypt your secret using [pynacl](https://pynacl.readthedocs.io/en/stable/public/#nacl-public-sealedbox) with Python 3.
     *
     * ```
     * from base64 import b64encode
     * from nacl import encoding, public
     *
     * def encrypt(public_key: str, secret_value: str) -> str:
     *   """Encrypt a Unicode string using the public key."""
     *   public_key = public.PublicKey(public_key.encode("utf-8"), encoding.Base64Encoder())
     *   sealed_box = public.SealedBox(public_key)
     *   encrypted = sealed_box.encrypt(secret_value.encode("utf-8"))
     *   return b64encode(encrypted).decode("utf-8")
     * ```
     *
     * #### Example encrypting a secret using C#
     *
     * Encrypt your secret using the [Sodium.Core](https://www.nuget.org/packages/Sodium.Core/) package.
     *
     * ```
     * var secretValue = System.Text.Encoding.UTF8.GetBytes("mySecret");
     * var publicKey = Convert.FromBase64String("2Sg8iYjAxxmI2LvUXpJjkYrMxURPc8r+dB7TJyvvcCU=");
     *
     * var sealedPublicKeyBox = Sodium.SealedPublicKeyBox.Create(secretValue, publicKey);
     *
     * Console.WriteLine(Convert.ToBase64String(sealedPublicKeyBox));
     * ```
     *
     * #### Example encrypting a secret using Ruby
     *
     * Encrypt your secret using the [rbnacl](https://github.com/RubyCrypto/rbnacl) gem.
     *
     * ```ruby
     * require "rbnacl"
     * require "base64"
     *
     * key = Base64.decode64("+ZYvJDZMHUfBkJdyq5Zm9SKqeuBQ4sj+6sfjlH4CgG0=")
     * public_key = RbNaCl::PublicKey.new(key)
     *
     * box = RbNaCl::Boxes::Sealed.from_public_key(public_key)
     * encrypted_secret = box.encrypt("my_secret")
     *
     * # Print the base64 encoded secret
     * puts Base64.strict_encode64(encrypted_secret)
     * ```
     *
     * FROM: <https://docs.github.com/rest/reference/actions#create-or-update-a-repository-secret>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `secret_name: &str` -- secret_name parameter.
     */
    pub async fn create_or_update_repo_secret(
        &self,
        owner: &str,
        repo: &str,
        secret_name: &str,
        body: &crate::types::ActionsCreateUpdateRepoSecretRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/secrets/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(secret_name),
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
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/actions/secrets/{secret_name}` endpoint.
     *
     * Deletes a secret in a repository using the secret name. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#delete-a-repository-secret>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `secret_name: &str` -- secret_name parameter.
     */
    pub async fn delete_repo_secret(
        &self,
        owner: &str,
        repo: &str,
        secret_name: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/secrets/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(secret_name),
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
     * List repository workflows.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/workflows` endpoint.
     *
     * Lists the workflows in a repository. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#list-repository-workflows>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_repo_workflows(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsListRepoWorkflowsResponse>> {
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
                "/repos/{}/{}/actions/workflows?{}",
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
     * Get a workflow.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/workflows/{workflow_id}` endpoint.
     *
     * Gets a specific workflow. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-a-workflow>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `workflow_id: &str` -- The ID of the workflow. You can also pass the workflow file name as a string.
     */
    pub async fn get_workflow(
        &self,
        owner: &str,
        repo: &str,
        workflow_id: &str,
    ) -> ClientResult<crate::Response<crate::types::Workflow>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/workflows/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(workflow_id),
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
     * Disable a workflow.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/actions/workflows/{workflow_id}/disable` endpoint.
     *
     * Disables a workflow and sets the `state` of the workflow to `disabled_manually`. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`.
     *
     * You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `actions:write` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#disable-a-workflow>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `workflow_id: &str` -- The ID of the workflow. You can also pass the workflow file name as a string.
     */
    pub async fn disable_workflow(
        &self,
        owner: &str,
        repo: &str,
        workflow_id: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/workflows/{}/disable",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(workflow_id),
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
     * Create a workflow dispatch event.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/actions/workflows/{workflow_id}/dispatches` endpoint.
     *
     * You can use this endpoint to manually trigger a GitHub Actions workflow run. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`.
     *
     * You must configure your GitHub Actions workflow to run when the [`workflow_dispatch` webhook](/developers/webhooks-and-events/webhook-events-and-payloads#workflow_dispatch) event occurs. The `inputs` are configured in the workflow file. For more information about how to configure the `workflow_dispatch` event in the workflow file, see "[Events that trigger workflows](/actions/reference/events-that-trigger-workflows#workflow_dispatch)."
     *
     * You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `actions:write` permission to use this endpoint. For more information, see "[Creating a personal access token for the command line](https://help.github.com/articles/creating-a-personal-access-token-for-the-command-line)."
     *
     * FROM: <https://docs.github.com/rest/reference/actions#create-a-workflow-dispatch-event>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `workflow_id: &str` -- The ID of the workflow. You can also pass the workflow file name as a string.
     */
    pub async fn create_workflow_dispatch(
        &self,
        owner: &str,
        repo: &str,
        workflow_id: &str,
        body: &crate::types::ActionsCreateWorkflowDispatchRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/workflows/{}/dispatches",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(workflow_id),
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
     * Enable a workflow.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/actions/workflows/{workflow_id}/enable` endpoint.
     *
     * Enables a workflow and sets the `state` of the workflow to `active`. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`.
     *
     * You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `actions:write` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#enable-a-workflow>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `workflow_id: &str` -- The ID of the workflow. You can also pass the workflow file name as a string.
     */
    pub async fn enable_workflow(
        &self,
        owner: &str,
        repo: &str,
        workflow_id: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/workflows/{}/enable",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(workflow_id),
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
     * List workflow runs.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/workflows/{workflow_id}/runs` endpoint.
     *
     * List all workflow runs for a workflow. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`. You can use parameters to narrow the list of results. For more information about using parameters, see [Parameters](https://docs.github.com/rest/overview/resources-in-the-rest-api#parameters).
     *
     * Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#list-workflow-runs>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `workflow_id: &str` -- The ID of the workflow. You can also pass the workflow file name as a string.
     * * `actor: &str` -- Returns someone's workflow runs. Use the login for the user who created the `push` associated with the check suite or workflow run.
     * * `branch: &str` -- Returns workflow runs associated with a branch. Use the name of the branch of the `push`.
     * * `event: &str` -- Returns workflow run triggered by the event you specify. For example, `push`, `pull_request` or `issue`. For more information, see "[Events that trigger workflows](https://help.github.com/en/actions/automating-your-workflow-with-github-actions/events-that-trigger-workflows).".
     * * `status: crate::types::WorkflowRunStatus` -- Returns workflow runs with the check run `status` or `conclusion` that you specify. For example, a conclusion can be `success` or a status can be `in_progress`. Only GitHub can set a status of `waiting` or `requested`. For a list of the possible `status` and `conclusion` options, see "[Create a check run](https://docs.github.com/rest/reference/checks#create-a-check-run).".
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     * * `created: &str`
     */
    pub async fn list_workflow_runs(
        &self,
        owner: &str,
        repo: &str,
        workflow_id: &str,
        actor: &str,
        branch: &str,
        event: &str,
        status: crate::types::WorkflowRunStatus,
        per_page: i64,
        page: i64,
        created: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsListWorkflowRunsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !actor.is_empty() {
            query_args.push(("actor".to_string(), actor.to_string()));
        }
        if !branch.is_empty() {
            query_args.push(("branch".to_string(), branch.to_string()));
        }
        if !created.is_empty() {
            query_args.push(("created".to_string(), created.to_string()));
        }
        if !event.is_empty() {
            query_args.push(("event".to_string(), event.to_string()));
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
                "/repos/{}/{}/actions/workflows/{}/runs?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(workflow_id),
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
     * Get workflow usage.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/workflows/{workflow_id}/timing` endpoint.
     *
     * Gets the number of billable minutes used by a specific workflow during the current billing cycle. Billable minutes only apply to workflows in private repositories that use GitHub-hosted runners. Usage is listed for each GitHub-hosted runner operating system in milliseconds. Any job re-runs are also included in the usage. The usage does not include the multiplier for macOS and Windows runners and is not rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://help.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
     *
     * You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `actions:read` permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-workflow-usage>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `workflow_id: &str` -- The ID of the workflow. You can also pass the workflow file name as a string.
     */
    pub async fn get_workflow_usage(
        &self,
        owner: &str,
        repo: &str,
        workflow_id: &str,
    ) -> ClientResult<crate::Response<crate::types::WorkflowUsage>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/workflows/{}/timing",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(workflow_id),
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
     * List environment secrets.
     *
     * This function performs a `GET` to the `/repositories/{repository_id}/environments/{environment_name}/secrets` endpoint.
     *
     * Lists all secrets available in an environment without revealing their encrypted values. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#list-environment-secrets>
     *
     * **Parameters:**
     *
     * * `repository_id: i64`
     * * `environment_name: &str` -- The name of the environment.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_environment_secrets(
        &self,
        repository_id: i64,
        environment_name: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsListRepoSecretsResponse>> {
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
                "/repositories/{}/environments/{}/secrets?{}",
                crate::progenitor_support::encode_path(&repository_id.to_string()),
                crate::progenitor_support::encode_path(environment_name),
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
     * Get an environment public key.
     *
     * This function performs a `GET` to the `/repositories/{repository_id}/environments/{environment_name}/secrets/public-key` endpoint.
     *
     * Get the public key for an environment, which you need to encrypt environment secrets. You need to encrypt a secret before you can create or update secrets. Anyone with read access to the repository can use this endpoint. If the repository is private you must use an access token with the `repo` scope. GitHub Apps must have the `secrets` repository permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-an-environment-public-key>
     *
     * **Parameters:**
     *
     * * `repository_id: i64`
     * * `environment_name: &str` -- The name of the environment.
     */
    pub async fn get_environment_public_key(
        &self,
        repository_id: i64,
        environment_name: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsPublicKey>> {
        let url = self.client.url(
            &format!(
                "/repositories/{}/environments/{}/secrets/public-key",
                crate::progenitor_support::encode_path(&repository_id.to_string()),
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
     * Get an environment secret.
     *
     * This function performs a `GET` to the `/repositories/{repository_id}/environments/{environment_name}/secrets/{secret_name}` endpoint.
     *
     * Gets a single environment secret without revealing its encrypted value. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#get-an-environment-secret>
     *
     * **Parameters:**
     *
     * * `repository_id: i64`
     * * `environment_name: &str` -- The name of the environment.
     * * `secret_name: &str` -- secret_name parameter.
     */
    pub async fn get_environment_secret(
        &self,
        repository_id: i64,
        environment_name: &str,
        secret_name: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsSecret>> {
        let url = self.client.url(
            &format!(
                "/repositories/{}/environments/{}/secrets/{}",
                crate::progenitor_support::encode_path(&repository_id.to_string()),
                crate::progenitor_support::encode_path(environment_name),
                crate::progenitor_support::encode_path(secret_name),
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
     * Create or update an environment secret.
     *
     * This function performs a `PUT` to the `/repositories/{repository_id}/environments/{environment_name}/secrets/{secret_name}` endpoint.
     *
     * Creates or updates an environment secret with an encrypted value. Encrypt your secret using
     * [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). You must authenticate using an access
     * token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use
     * this endpoint.
     *
     * #### Example encrypting a secret using Node.js
     *
     * Encrypt your secret using the [tweetsodium](https://github.com/github/tweetsodium) library.
     *
     * ```
     * const sodium = require('tweetsodium');
     *
     * const key = "base64-encoded-public-key";
     * const value = "plain-text-secret";
     *
     * // Convert the message and key to Uint8Array's (Buffer implements that interface)
     * const messageBytes = Buffer.from(value);
     * const keyBytes = Buffer.from(key, 'base64');
     *
     * // Encrypt using LibSodium.
     * const encryptedBytes = sodium.seal(messageBytes, keyBytes);
     *
     * // Base64 the encrypted secret
     * const encrypted = Buffer.from(encryptedBytes).toString('base64');
     *
     * console.log(encrypted);
     * ```
     *
     *
     * #### Example encrypting a secret using Python
     *
     * Encrypt your secret using [pynacl](https://pynacl.readthedocs.io/en/stable/public/#nacl-public-sealedbox) with Python 3.
     *
     * ```
     * from base64 import b64encode
     * from nacl import encoding, public
     *
     * def encrypt(public_key: str, secret_value: str) -> str:
     *   """Encrypt a Unicode string using the public key."""
     *   public_key = public.PublicKey(public_key.encode("utf-8"), encoding.Base64Encoder())
     *   sealed_box = public.SealedBox(public_key)
     *   encrypted = sealed_box.encrypt(secret_value.encode("utf-8"))
     *   return b64encode(encrypted).decode("utf-8")
     * ```
     *
     * #### Example encrypting a secret using C#
     *
     * Encrypt your secret using the [Sodium.Core](https://www.nuget.org/packages/Sodium.Core/) package.
     *
     * ```
     * var secretValue = System.Text.Encoding.UTF8.GetBytes("mySecret");
     * var publicKey = Convert.FromBase64String("2Sg8iYjAxxmI2LvUXpJjkYrMxURPc8r+dB7TJyvvcCU=");
     *
     * var sealedPublicKeyBox = Sodium.SealedPublicKeyBox.Create(secretValue, publicKey);
     *
     * Console.WriteLine(Convert.ToBase64String(sealedPublicKeyBox));
     * ```
     *
     * #### Example encrypting a secret using Ruby
     *
     * Encrypt your secret using the [rbnacl](https://github.com/RubyCrypto/rbnacl) gem.
     *
     * ```ruby
     * require "rbnacl"
     * require "base64"
     *
     * key = Base64.decode64("+ZYvJDZMHUfBkJdyq5Zm9SKqeuBQ4sj+6sfjlH4CgG0=")
     * public_key = RbNaCl::PublicKey.new(key)
     *
     * box = RbNaCl::Boxes::Sealed.from_public_key(public_key)
     * encrypted_secret = box.encrypt("my_secret")
     *
     * # Print the base64 encoded secret
     * puts Base64.strict_encode64(encrypted_secret)
     * ```
     *
     * FROM: <https://docs.github.com/rest/reference/actions#create-or-update-an-environment-secret>
     *
     * **Parameters:**
     *
     * * `repository_id: i64`
     * * `environment_name: &str` -- The name of the environment.
     * * `secret_name: &str` -- secret_name parameter.
     */
    pub async fn create_or_update_environment_secret(
        &self,
        repository_id: i64,
        environment_name: &str,
        secret_name: &str,
        body: &crate::types::ActionsCreateUpdateRepoSecretRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repositories/{}/environments/{}/secrets/{}",
                crate::progenitor_support::encode_path(&repository_id.to_string()),
                crate::progenitor_support::encode_path(environment_name),
                crate::progenitor_support::encode_path(secret_name),
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
     * Delete an environment secret.
     *
     * This function performs a `DELETE` to the `/repositories/{repository_id}/environments/{environment_name}/secrets/{secret_name}` endpoint.
     *
     * Deletes a secret in an environment using the secret name. You must authenticate using an access token with the `repo` scope to use this endpoint. GitHub Apps must have the `secrets` repository permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/actions#delete-an-environment-secret>
     *
     * **Parameters:**
     *
     * * `repository_id: i64`
     * * `environment_name: &str` -- The name of the environment.
     * * `secret_name: &str` -- secret_name parameter.
     */
    pub async fn delete_environment_secret(
        &self,
        repository_id: i64,
        environment_name: &str,
        secret_name: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repositories/{}/environments/{}/secrets/{}",
                crate::progenitor_support::encode_path(&repository_id.to_string()),
                crate::progenitor_support::encode_path(environment_name),
                crate::progenitor_support::encode_path(secret_name),
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
