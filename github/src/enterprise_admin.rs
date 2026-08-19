use crate::Client;
use crate::ClientResult;

pub struct EnterpriseAdmin {
    pub client: Client,
}

impl EnterpriseAdmin {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EnterpriseAdmin { client }
    }

    /**
     * Get GitHub Actions permissions for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/actions/permissions` endpoint.
     *
     * Gets the GitHub Actions permissions policy for organizations and allowed actions in an enterprise.
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#get-github-actions-permissions-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     */
    pub async fn get_github_actions_permissions_enterprise(
        &self,
        enterprise: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsEnterprisePermissions>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/permissions",
                crate::progenitor_support::encode_path(enterprise),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Set GitHub Actions permissions for an enterprise.
     *
     * This function performs a `PUT` to the `/enterprises/{enterprise}/actions/permissions` endpoint.
     *
     * Sets the GitHub Actions permissions policy for organizations and allowed actions in an enterprise.
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#set-github-actions-permissions-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     */
    pub async fn set_github_actions_permissions_enterprise(
        &self,
        enterprise: &str,
        body: &crate::types::EnterpriseAdminSetGithubActionsPermissionsRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/permissions",
                crate::progenitor_support::encode_path(enterprise),
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
     * List selected organizations enabled for GitHub Actions in an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/actions/permissions/organizations` endpoint.
     *
     * Lists the organizations that are selected to have GitHub Actions enabled in an enterprise. To use this endpoint, the enterprise permission policy for `enabled_organizations` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an enterprise](#set-github-actions-permissions-for-an-enterprise)."
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#list-selected-organizations-enabled-for-github-actions-in-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_selected_organizations_enabled_github_actions_enterprise(
        &self,
        enterprise: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<
        crate::Response<crate::types::EnterpriseAdminListOrgAccessSelfHostedRunnerGroupInResponse>,
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
                "/enterprises/{}/actions/permissions/organizations?{}",
                crate::progenitor_support::encode_path(enterprise),
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
     * Set selected organizations enabled for GitHub Actions in an enterprise.
     *
     * This function performs a `PUT` to the `/enterprises/{enterprise}/actions/permissions/organizations` endpoint.
     *
     * Replaces the list of selected organizations that are enabled for GitHub Actions in an enterprise. To use this endpoint, the enterprise permission policy for `enabled_organizations` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an enterprise](#set-github-actions-permissions-for-an-enterprise)."
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#set-selected-organizations-enabled-for-github-actions-in-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     */
    pub async fn set_selected_organizations_enabled_github_actions_enterprise(
        &self,
        enterprise: &str,
        body: &crate::types::EnterpriseAdminSetOrgAccessSelfHostedRunnerGroupInRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/permissions/organizations",
                crate::progenitor_support::encode_path(enterprise),
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
     * Enable a selected organization for GitHub Actions in an enterprise.
     *
     * This function performs a `PUT` to the `/enterprises/{enterprise}/actions/permissions/organizations/{org_id}` endpoint.
     *
     * Adds an organization to the list of selected organizations that are enabled for GitHub Actions in an enterprise. To use this endpoint, the enterprise permission policy for `enabled_organizations` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an enterprise](#set-github-actions-permissions-for-an-enterprise)."
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#enable-a-selected-organization-for-github-actions-in-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `org_id: i64` -- Unique identifier of an organization.
     */
    pub async fn enable_selected_organization_github_actions_enterprise(
        &self,
        enterprise: &str,
        org_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/permissions/organizations/{}",
                crate::progenitor_support::encode_path(enterprise),
                crate::progenitor_support::encode_path(&org_id.to_string()),
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
     * Disable a selected organization for GitHub Actions in an enterprise.
     *
     * This function performs a `DELETE` to the `/enterprises/{enterprise}/actions/permissions/organizations/{org_id}` endpoint.
     *
     * Removes an organization from the list of selected organizations that are enabled for GitHub Actions in an enterprise. To use this endpoint, the enterprise permission policy for `enabled_organizations` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an enterprise](#set-github-actions-permissions-for-an-enterprise)."
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#disable-a-selected-organization-for-github-actions-in-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `org_id: i64` -- Unique identifier of an organization.
     */
    pub async fn disable_selected_organization_github_actions_enterprise(
        &self,
        enterprise: &str,
        org_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/permissions/organizations/{}",
                crate::progenitor_support::encode_path(enterprise),
                crate::progenitor_support::encode_path(&org_id.to_string()),
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
     * Get allowed actions for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/actions/permissions/selected-actions` endpoint.
     *
     * Gets the selected actions that are allowed in an enterprise. To use this endpoint, the enterprise permission policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an enterprise](#set-github-actions-permissions-for-an-enterprise)."
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#get-allowed-actions-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     */
    pub async fn get_allowed_actions_enterprise(
        &self,
        enterprise: &str,
    ) -> ClientResult<crate::Response<crate::types::SelectedActions>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/permissions/selected-actions",
                crate::progenitor_support::encode_path(enterprise),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Set allowed actions for an enterprise.
     *
     * This function performs a `PUT` to the `/enterprises/{enterprise}/actions/permissions/selected-actions` endpoint.
     *
     * Sets the actions that are allowed in an enterprise. To use this endpoint, the enterprise permission policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an enterprise](#set-github-actions-permissions-for-an-enterprise)."
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#set-allowed-actions-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     */
    pub async fn set_allowed_actions_enterprise(
        &self,
        enterprise: &str,
        body: &crate::types::SelectedActions,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/permissions/selected-actions",
                crate::progenitor_support::encode_path(enterprise),
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
     * List self-hosted runner groups for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/actions/runner-groups` endpoint.
     *
     * Lists all self-hosted runner groups for an enterprise.
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#list-self-hosted-runner-groups-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_self_hosted_runner_groups_for_enterprise(
        &self,
        enterprise: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<
        crate::Response<crate::types::EnterpriseAdminListSelfHostedRunnerGroupsResponse>,
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
                "/enterprises/{}/actions/runner-groups?{}",
                crate::progenitor_support::encode_path(enterprise),
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
     * Create a self-hosted runner group for an enterprise.
     *
     * This function performs a `POST` to the `/enterprises/{enterprise}/actions/runner-groups` endpoint.
     *
     * Creates a new self-hosted runner group for an enterprise.
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#create-self-hosted-runner-group-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     */
    pub async fn create_self_hosted_runner_group_for_enterprise(
        &self,
        enterprise: &str,
        body: &crate::types::EnterpriseAdminCreateSelfHostedRunnerGroupRequest,
    ) -> ClientResult<crate::Response<crate::types::RunnerGroupsEnterprise>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/runner-groups",
                crate::progenitor_support::encode_path(enterprise),
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
     * Get a self-hosted runner group for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}` endpoint.
     *
     * Gets a specific self-hosted runner group for an enterprise.
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#get-a-self-hosted-runner-group-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     */
    pub async fn get_self_hosted_runner_group_for_enterprise(
        &self,
        enterprise: &str,
        runner_group_id: i64,
    ) -> ClientResult<crate::Response<crate::types::RunnerGroupsEnterprise>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/runner-groups/{}",
                crate::progenitor_support::encode_path(enterprise),
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
     * Delete a self-hosted runner group from an enterprise.
     *
     * This function performs a `DELETE` to the `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}` endpoint.
     *
     * Deletes a self-hosted runner group for an enterprise.
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#delete-a-self-hosted-runner-group-from-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     */
    pub async fn delete_self_hosted_runner_group_from_enterprise(
        &self,
        enterprise: &str,
        runner_group_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/runner-groups/{}",
                crate::progenitor_support::encode_path(enterprise),
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
     * Update a self-hosted runner group for an enterprise.
     *
     * This function performs a `PATCH` to the `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}` endpoint.
     *
     * Updates the `name` and `visibility` of a self-hosted runner group in an enterprise.
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#update-a-self-hosted-runner-group-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     */
    pub async fn update_self_hosted_runner_group_for_enterprise(
        &self,
        enterprise: &str,
        runner_group_id: i64,
        body: &crate::types::EnterpriseAdminUpdateSelfHostedRunnerGroupRequest,
    ) -> ClientResult<crate::Response<crate::types::RunnerGroupsEnterprise>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/runner-groups/{}",
                crate::progenitor_support::encode_path(enterprise),
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
     * List organization access to a self-hosted runner group in an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/organizations` endpoint.
     *
     * Lists the organizations with access to a self-hosted runner group.
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#list-organization-access-to-a-self-hosted-runner-group-in-a-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_org_access_to_self_hosted_runner_group_in_enterprise(
        &self,
        enterprise: &str,
        runner_group_id: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<
        crate::Response<crate::types::EnterpriseAdminListOrgAccessSelfHostedRunnerGroupInResponse>,
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
                "/enterprises/{}/actions/runner-groups/{}/organizations?{}",
                crate::progenitor_support::encode_path(enterprise),
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
     * Set organization access for a self-hosted runner group in an enterprise.
     *
     * This function performs a `PUT` to the `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/organizations` endpoint.
     *
     * Replaces the list of organizations that have access to a self-hosted runner configured in an enterprise.
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#set-organization-access-to-a-self-hosted-runner-group-in-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     */
    pub async fn set_org_access_to_self_hosted_runner_group_in_enterprise(
        &self,
        enterprise: &str,
        runner_group_id: i64,
        body: &crate::types::EnterpriseAdminSetOrgAccessSelfHostedRunnerGroupInRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/runner-groups/{}/organizations",
                crate::progenitor_support::encode_path(enterprise),
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
     * Add organization access to a self-hosted runner group in an enterprise.
     *
     * This function performs a `PUT` to the `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/organizations/{org_id}` endpoint.
     *
     * Adds an organization to the list of selected organizations that can access a self-hosted runner group. The runner group must have `visibility` set to `selected`. For more information, see "[Create a self-hosted runner group for an enterprise](#create-a-self-hosted-runner-group-for-an-enterprise)."
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#add-organization-access-to-a-self-hosted-runner-group-in-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     * * `org_id: i64` -- Unique identifier of an organization.
     */
    pub async fn add_org_access_to_self_hosted_runner_group_in_enterprise(
        &self,
        enterprise: &str,
        runner_group_id: i64,
        org_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/runner-groups/{}/organizations/{}",
                crate::progenitor_support::encode_path(enterprise),
                crate::progenitor_support::encode_path(&runner_group_id.to_string()),
                crate::progenitor_support::encode_path(&org_id.to_string()),
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
     * Remove organization access to a self-hosted runner group in an enterprise.
     *
     * This function performs a `DELETE` to the `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/organizations/{org_id}` endpoint.
     *
     * Removes an organization from the list of selected organizations that can access a self-hosted runner group. The runner group must have `visibility` set to `selected`. For more information, see "[Create a self-hosted runner group for an enterprise](#create-a-self-hosted-runner-group-for-an-enterprise)."
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#remove-organization-access-to-a-self-hosted-runner-group-in-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     * * `org_id: i64` -- Unique identifier of an organization.
     */
    pub async fn remove_org_access_to_self_hosted_runner_group_in_enterprise(
        &self,
        enterprise: &str,
        runner_group_id: i64,
        org_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/runner-groups/{}/organizations/{}",
                crate::progenitor_support::encode_path(enterprise),
                crate::progenitor_support::encode_path(&runner_group_id.to_string()),
                crate::progenitor_support::encode_path(&org_id.to_string()),
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
     * List self-hosted runners in a group for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/runners` endpoint.
     *
     * Lists the self-hosted runners that are in a specific enterprise group.
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#list-self-hosted-runners-in-a-group-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_self_hosted_runners_in_group_for_enterprise(
        &self,
        enterprise: &str,
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
                "/enterprises/{}/actions/runner-groups/{}/runners?{}",
                crate::progenitor_support::encode_path(enterprise),
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
     * Set self-hosted runners in a group for an enterprise.
     *
     * This function performs a `PUT` to the `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/runners` endpoint.
     *
     * Replaces the list of self-hosted runners that are part of an enterprise runner group.
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#set-self-hosted-runners-in-a-group-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     */
    pub async fn set_self_hosted_runners_in_group_for_enterprise(
        &self,
        enterprise: &str,
        runner_group_id: i64,
        body: &crate::types::ActionsSetSelfHostedRunnersInGroupOrgRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/runner-groups/{}/runners",
                crate::progenitor_support::encode_path(enterprise),
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
     * Add a self-hosted runner to a group for an enterprise.
     *
     * This function performs a `PUT` to the `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/runners/{runner_id}` endpoint.
     *
     * Adds a self-hosted runner to a runner group configured in an enterprise.
     *
     * You must authenticate using an access token with the `admin:enterprise`
     * scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#add-a-self-hosted-runner-to-a-group-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     * * `runner_id: i64` -- Unique identifier of the self-hosted runner.
     */
    pub async fn add_self_hosted_runner_to_group_for_enterprise(
        &self,
        enterprise: &str,
        runner_group_id: i64,
        runner_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/runner-groups/{}/runners/{}",
                crate::progenitor_support::encode_path(enterprise),
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
     * Remove a self-hosted runner from a group for an enterprise.
     *
     * This function performs a `DELETE` to the `/enterprises/{enterprise}/actions/runner-groups/{runner_group_id}/runners/{runner_id}` endpoint.
     *
     * Removes a self-hosted runner from a group configured in an enterprise. The runner is then returned to the default group.
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#remove-a-self-hosted-runner-from-a-group-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     * * `runner_id: i64` -- Unique identifier of the self-hosted runner.
     */
    pub async fn remove_self_hosted_runner_from_group_for_enterprise(
        &self,
        enterprise: &str,
        runner_group_id: i64,
        runner_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/runner-groups/{}/runners/{}",
                crate::progenitor_support::encode_path(enterprise),
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
     * List self-hosted runners for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/actions/runners` endpoint.
     *
     * Lists all self-hosted runners configured for an enterprise.
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#list-self-hosted-runners-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_self_hosted_runners_for_enterprise(
        &self,
        enterprise: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::EnterpriseAdminListSelfHostedRunnersResponse>>
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
                "/enterprises/{}/actions/runners?{}",
                crate::progenitor_support::encode_path(enterprise),
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
     * List runner applications for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/actions/runners/downloads` endpoint.
     *
     * Lists binaries for the runner application that you can download and run.
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#list-runner-applications-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     */
    pub async fn list_runner_applications_for_enterprise(
        &self,
        enterprise: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::RunnerApplication>>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/runners/downloads",
                crate::progenitor_support::encode_path(enterprise),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * List runner applications for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/actions/runners/downloads` endpoint.
     *
     * As opposed to `list_runner_applications_for_enterprise`, this function returns all the pages of the request at once.
     *
     * Lists binaries for the runner application that you can download and run.
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#list-runner-applications-for-an-enterprise>
     */
    pub async fn list_all_runner_applications_for_enterprise(
        &self,
        enterprise: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::RunnerApplication>>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/runners/downloads",
                crate::progenitor_support::encode_path(enterprise),
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
     * Create a registration token for an enterprise.
     *
     * This function performs a `POST` to the `/enterprises/{enterprise}/actions/runners/registration-token` endpoint.
     *
     * Returns a token that you can pass to the `config` script. The token expires after one hour.
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * #### Example using registration token
     *
     * Configure your self-hosted runner, replacing `TOKEN` with the registration token provided by this endpoint.
     *
     * ```
     * ./config.sh --url https://github.com/enterprises/octo-enterprise --token TOKEN
     * ```
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#create-a-registration-token-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     */
    pub async fn create_registration_token_for_enterprise(
        &self,
        enterprise: &str,
    ) -> ClientResult<crate::Response<crate::types::AuthenticationToken>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/runners/registration-token",
                crate::progenitor_support::encode_path(enterprise),
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
     * Create a remove token for an enterprise.
     *
     * This function performs a `POST` to the `/enterprises/{enterprise}/actions/runners/remove-token` endpoint.
     *
     * Returns a token that you can pass to the `config` script to remove a self-hosted runner from an enterprise. The token expires after one hour.
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * #### Example using remove token
     *
     * To remove your self-hosted runner from an enterprise, replace `TOKEN` with the remove token provided by this
     * endpoint.
     *
     * ```
     * ./config.sh remove --token TOKEN
     * ```
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#create-a-remove-token-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     */
    pub async fn create_remove_token_for_enterprise(
        &self,
        enterprise: &str,
    ) -> ClientResult<crate::Response<crate::types::AuthenticationToken>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/runners/remove-token",
                crate::progenitor_support::encode_path(enterprise),
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
     * Get a self-hosted runner for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/actions/runners/{runner_id}` endpoint.
     *
     * Gets a specific self-hosted runner configured in an enterprise.
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#get-a-self-hosted-runner-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `runner_id: i64` -- Unique identifier of the self-hosted runner.
     */
    pub async fn get_self_hosted_runner_for_enterprise(
        &self,
        enterprise: &str,
        runner_id: i64,
    ) -> ClientResult<crate::Response<crate::types::Runner>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/runners/{}",
                crate::progenitor_support::encode_path(enterprise),
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
     * Delete a self-hosted runner from an enterprise.
     *
     * This function performs a `DELETE` to the `/enterprises/{enterprise}/actions/runners/{runner_id}` endpoint.
     *
     * Forces the removal of a self-hosted runner from an enterprise. You can use this endpoint to completely remove the runner when the machine you were using no longer exists.
     *
     * You must authenticate using an access token with the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#delete-self-hosted-runner-from-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `runner_id: i64` -- Unique identifier of the self-hosted runner.
     */
    pub async fn delete_self_hosted_runner_from_enterprise(
        &self,
        enterprise: &str,
        runner_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/runners/{}",
                crate::progenitor_support::encode_path(enterprise),
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
     * Get the audit log for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/audit-log` endpoint.
     *
     * Gets the audit log for an enterprise. To use this endpoint, you must be an enterprise admin, and you must use an access token with the `admin:enterprise` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#get-the-audit-log-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `phrase: &str` -- A search phrase. For more information, see [Searching the audit log](https://docs.github.com/github/setting-up-and-managing-organizations-and-teams/reviewing-the-audit-log-for-your-organization#searching-the-audit-log).
     * * `include: crate::types::Include` -- The event types to include:
     *  
     *  - `web` - returns web (non-Git) events
     *  - `git` - returns Git events
     *  - `all` - returns both web and Git events
     *  
     *  The default is `web`.
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/overview/resources-in-the-rest-api#link-header). If specified, the query only searches for events after this cursor.
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/overview/resources-in-the-rest-api#link-header). If specified, the query only searches for events before this cursor.
     * * `order: crate::types::Order` -- The order of audit log events. To list newest events first, specify `desc`. To list oldest events first, specify `asc`.
     *  
     *  The default is `desc`.
     * * `page: i64` -- Page number of the results to fetch.
     * * `per_page: i64` -- Results per page (max 100).
     */
    pub async fn get_audit_log(
        &self,
        enterprise: &str,
        phrase: &str,
        include: crate::types::Include,
        after: &str,
        before: &str,
        order: crate::types::Order,
        page: i64,
        per_page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::AuditLogEvent>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !include.to_string().is_empty() {
            query_args.push(("include".to_string(), include.to_string()));
        }
        if !order.to_string().is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !phrase.is_empty() {
            query_args.push(("phrase".to_string(), phrase.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/enterprises/{}/audit-log?{}",
                crate::progenitor_support::encode_path(enterprise),
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
     * Get the audit log for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/audit-log` endpoint.
     *
     * As opposed to `get_audit_log`, this function returns all the pages of the request at once.
     *
     * Gets the audit log for an enterprise. To use this endpoint, you must be an enterprise admin, and you must use an access token with the `admin:enterprise` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#get-the-audit-log-for-an-enterprise>
     */
    pub async fn get_all_audit_log(
        &self,
        enterprise: &str,
        phrase: &str,
        include: crate::types::Include,
        after: &str,
        before: &str,
        order: crate::types::Order,
    ) -> ClientResult<crate::Response<Vec<crate::types::AuditLogEvent>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !include.to_string().is_empty() {
            query_args.push(("include".to_string(), include.to_string()));
        }
        if !order.to_string().is_empty() {
            query_args.push(("order".to_string(), order.to_string()));
        }
        if !phrase.is_empty() {
            query_args.push(("phrase".to_string(), phrase.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/enterprises/{}/audit-log?{}",
                crate::progenitor_support::encode_path(enterprise),
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
     * List provisioned SCIM groups for an enterprise.
     *
     * This function performs a `GET` to the `/scim/v2/enterprises/{enterprise}/Groups` endpoint.
     *
     * **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#list-provisioned-scim-groups-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `start_index: i64` -- Used for pagination: the index of the first result to return.
     * * `count: i64` -- Used for pagination: the number of results to return.
     * * `filter: &str` -- filter results.
     * * `excluded_attributes: &str` -- attributes to exclude.
     */
    pub async fn list_provisioned_groups_enterprise(
        &self,
        enterprise: &str,
        start_index: i64,
        count: i64,
        filter: &str,
        excluded_attributes: &str,
    ) -> ClientResult<crate::Response<crate::types::ScimGroupListEnterprise>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !excluded_attributes.is_empty() {
            query_args.push((
                "excludedAttributes".to_string(),
                excluded_attributes.to_string(),
            ));
        }
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if start_index > 0 {
            query_args.push(("startIndex".to_string(), start_index.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/scim/v2/enterprises/{}/Groups?{}",
                crate::progenitor_support::encode_path(enterprise),
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
     * Provision a SCIM enterprise group and invite users.
     *
     * This function performs a `POST` to the `/scim/v2/enterprises/{enterprise}/Groups` endpoint.
     *
     * **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
     *
     * Provision an enterprise group, and invite users to the group. This sends invitation emails to the email address of the invited users to join the GitHub organization that the SCIM group corresponds to.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#provision-a-scim-enterprise-group-and-invite-users>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     */
    pub async fn provision_and_invite_enterprise_group(
        &self,
        enterprise: &str,
        body: &crate::types::EnterpriseAdminProvisionInviteGroupRequest,
    ) -> ClientResult<crate::Response<crate::types::ScimEnterpriseGroup>> {
        let url = self.client.url(
            &format!(
                "/scim/v2/enterprises/{}/Groups",
                crate::progenitor_support::encode_path(enterprise),
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
     * Get SCIM provisioning information for an enterprise group.
     *
     * This function performs a `GET` to the `/scim/v2/enterprises/{enterprise}/Groups/{scim_group_id}` endpoint.
     *
     * **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#get-scim-provisioning-information-for-an-enterprise-group>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `scim_group_id: &str` -- Identifier generated by the GitHub SCIM endpoint.
     * * `excluded_attributes: &str` -- Attributes to exclude.
     */
    pub async fn get_provisioning_information_for_enterprise_group(
        &self,
        enterprise: &str,
        scim_group_id: &str,
        excluded_attributes: &str,
    ) -> ClientResult<crate::Response<crate::types::ScimEnterpriseGroup>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !excluded_attributes.is_empty() {
            query_args.push((
                "excludedAttributes".to_string(),
                excluded_attributes.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/scim/v2/enterprises/{}/Groups/{}?{}",
                crate::progenitor_support::encode_path(enterprise),
                crate::progenitor_support::encode_path(scim_group_id),
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
     * Set SCIM information for a provisioned enterprise group.
     *
     * This function performs a `PUT` to the `/scim/v2/enterprises/{enterprise}/Groups/{scim_group_id}` endpoint.
     *
     * **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
     *
     * Replaces an existing provisioned group’s information. You must provide all the information required for the group as if you were provisioning it for the first time. Any existing group information that you don't provide will be removed, including group membership. If you want to only update a specific attribute, use the [Update an attribute for a SCIM enterprise group](#update-an-attribute-for-a-scim-enterprise-group) endpoint instead.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#set-scim-information-for-a-provisioned-enterprise-group>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `scim_group_id: &str` -- Identifier generated by the GitHub SCIM endpoint.
     */
    pub async fn set_information_for_provisioned_enterprise_group(
        &self,
        enterprise: &str,
        scim_group_id: &str,
        body: &crate::types::EnterpriseAdminProvisionInviteGroupRequest,
    ) -> ClientResult<crate::Response<crate::types::ScimEnterpriseGroup>> {
        let url = self.client.url(
            &format!(
                "/scim/v2/enterprises/{}/Groups/{}",
                crate::progenitor_support::encode_path(enterprise),
                crate::progenitor_support::encode_path(scim_group_id),
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
     * Delete a SCIM group from an enterprise.
     *
     * This function performs a `DELETE` to the `/scim/v2/enterprises/{enterprise}/Groups/{scim_group_id}` endpoint.
     *
     * **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#delete-a-scim-group-from-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `scim_group_id: &str` -- Identifier generated by the GitHub SCIM endpoint.
     */
    pub async fn delete_scim_group_from_enterprise(
        &self,
        enterprise: &str,
        scim_group_id: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/scim/v2/enterprises/{}/Groups/{}",
                crate::progenitor_support::encode_path(enterprise),
                crate::progenitor_support::encode_path(scim_group_id),
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
     * Update an attribute for a SCIM enterprise group.
     *
     * This function performs a `PATCH` to the `/scim/v2/enterprises/{enterprise}/Groups/{scim_group_id}` endpoint.
     *
     * **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
     *
     * Allows you to change a provisioned group’s individual attributes. To change a group’s values, you must provide a specific Operations JSON format that contains at least one of the add, remove, or replace operations. For examples and more information on the SCIM operations format, see the [SCIM specification](https://tools.ietf.org/html/rfc7644#section-3.5.2).
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#update-an-attribute-for-a-scim-enterprise-group>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `scim_group_id: &str` -- Identifier generated by the GitHub SCIM endpoint.
     */
    pub async fn update_attribute_for_enterprise_group(
        &self,
        enterprise: &str,
        scim_group_id: &str,
        body: &crate::types::EnterpriseAdminUpdateAttributeGroupRequest,
    ) -> ClientResult<crate::Response<crate::types::ScimEnterpriseGroup>> {
        let url = self.client.url(
            &format!(
                "/scim/v2/enterprises/{}/Groups/{}",
                crate::progenitor_support::encode_path(enterprise),
                crate::progenitor_support::encode_path(scim_group_id),
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
     * List SCIM provisioned identities for an enterprise.
     *
     * This function performs a `GET` to the `/scim/v2/enterprises/{enterprise}/Users` endpoint.
     *
     * **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
     *
     * Retrieves a paginated list of all provisioned enterprise members, including pending invitations.
     *
     * When a user with a SAML-provisioned external identity leaves (or is removed from) an enterprise, the account's metadata is immediately removed. However, the returned list of user accounts might not always match the organization or enterprise member list you see on GitHub. This can happen in certain cases where an external identity associated with an organization will not match an organization member:
     *   - When a user with a SCIM-provisioned external identity is removed from an enterprise, the account's metadata is preserved to allow the user to re-join the organization in the future.
     *   - When inviting a user to join an organization, you can expect to see their external identity in the results before they accept the invitation, or if the invitation is cancelled (or never accepted).
     *   - When a user is invited over SCIM, an external identity is created that matches with the invitee's email address. However, this identity is only linked to a user account when the user accepts the invitation by going through SAML SSO.
     *
     * The returned list of external identities can include an entry for a `null` user. These are unlinked SAML identities that are created when a user goes through the following Single Sign-On (SSO) process but does not sign in to their GitHub account after completing SSO:
     *
     * 1. The user is granted access by the IdP and is not a member of the GitHub enterprise.
     *
     * 1. The user attempts to access the GitHub enterprise and initiates the SAML SSO process, and is not currently signed in to their GitHub account.
     *
     * 1. After successfully authenticating with the SAML SSO IdP, the `null` external identity entry is created and the user is prompted to sign in to their GitHub account:
     *    - If the user signs in, their GitHub account is linked to this entry.
     *    - If the user does not sign in (or does not create a new account when prompted), they are not added to the GitHub enterprise, and the external identity `null` entry remains in place.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#list-scim-provisioned-identities-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `start_index: i64` -- Used for pagination: the index of the first result to return.
     * * `count: i64` -- Used for pagination: the number of results to return.
     * * `filter: &str` -- filter results.
     */
    pub async fn list_provisioned_identities_enterprise(
        &self,
        enterprise: &str,
        start_index: i64,
        count: i64,
        filter: &str,
    ) -> ClientResult<crate::Response<crate::types::ScimUserListEnterprise>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !filter.is_empty() {
            query_args.push(("filter".to_string(), filter.to_string()));
        }
        if start_index > 0 {
            query_args.push(("startIndex".to_string(), start_index.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/scim/v2/enterprises/{}/Users?{}",
                crate::progenitor_support::encode_path(enterprise),
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
     * Provision and invite a SCIM enterprise user.
     *
     * This function performs a `POST` to the `/scim/v2/enterprises/{enterprise}/Users` endpoint.
     *
     * **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
     *
     * Provision enterprise membership for a user, and send organization invitation emails to the email address.
     *
     * You can optionally include the groups a user will be invited to join. If you do not provide a list of `groups`, the user is provisioned for the enterprise, but no organization invitation emails will be sent.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#provision-and-invite-a-scim-enterprise-user>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     */
    pub async fn provision_and_invite_enterprise_user(
        &self,
        enterprise: &str,
        body: &crate::types::EnterpriseAdminProvisionInviteUserRequest,
    ) -> ClientResult<crate::Response<crate::types::ScimEnterpriseUser>> {
        let url = self.client.url(
            &format!(
                "/scim/v2/enterprises/{}/Users",
                crate::progenitor_support::encode_path(enterprise),
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
     * Get SCIM provisioning information for an enterprise user.
     *
     * This function performs a `GET` to the `/scim/v2/enterprises/{enterprise}/Users/{scim_user_id}` endpoint.
     *
     * **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#get-scim-provisioning-information-for-an-enterprise-user>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `scim_user_id: &str` -- scim_user_id parameter.
     */
    pub async fn get_provisioning_information_for_enterprise_user(
        &self,
        enterprise: &str,
        scim_user_id: &str,
    ) -> ClientResult<crate::Response<crate::types::ScimEnterpriseUser>> {
        let url = self.client.url(
            &format!(
                "/scim/v2/enterprises/{}/Users/{}",
                crate::progenitor_support::encode_path(enterprise),
                crate::progenitor_support::encode_path(scim_user_id),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Set SCIM information for a provisioned enterprise user.
     *
     * This function performs a `PUT` to the `/scim/v2/enterprises/{enterprise}/Users/{scim_user_id}` endpoint.
     *
     * **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
     *
     * Replaces an existing provisioned user's information. You must provide all the information required for the user as if you were provisioning them for the first time. Any existing user information that you don't provide will be removed. If you want to only update a specific attribute, use the [Update an attribute for a SCIM user](#update-an-attribute-for-an-enterprise-scim-user) endpoint instead.
     *
     * You must at least provide the required values for the user: `userName`, `name`, and `emails`.
     *
     * **Warning:** Setting `active: false` removes the user from the enterprise, deletes the external identity, and deletes the associated `{scim_user_id}`.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#set-scim-information-for-a-provisioned-enterprise-user>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `scim_user_id: &str` -- scim_user_id parameter.
     */
    pub async fn set_information_for_provisioned_enterprise_user(
        &self,
        enterprise: &str,
        scim_user_id: &str,
        body: &crate::types::EnterpriseAdminProvisionInviteUserRequest,
    ) -> ClientResult<crate::Response<crate::types::ScimEnterpriseUser>> {
        let url = self.client.url(
            &format!(
                "/scim/v2/enterprises/{}/Users/{}",
                crate::progenitor_support::encode_path(enterprise),
                crate::progenitor_support::encode_path(scim_user_id),
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
     * Delete a SCIM user from an enterprise.
     *
     * This function performs a `DELETE` to the `/scim/v2/enterprises/{enterprise}/Users/{scim_user_id}` endpoint.
     *
     * **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#delete-a-scim-user-from-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `scim_user_id: &str` -- scim_user_id parameter.
     */
    pub async fn delete_user_from_enterprise(
        &self,
        enterprise: &str,
        scim_user_id: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/scim/v2/enterprises/{}/Users/{}",
                crate::progenitor_support::encode_path(enterprise),
                crate::progenitor_support::encode_path(scim_user_id),
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
     * Update an attribute for a SCIM enterprise user.
     *
     * This function performs a `PATCH` to the `/scim/v2/enterprises/{enterprise}/Users/{scim_user_id}` endpoint.
     *
     * **Note:** The SCIM API endpoints for enterprise accounts are currently in beta and are subject to change.
     *
     * Allows you to change a provisioned user's individual attributes. To change a user's values, you must provide a specific `Operations` JSON format that contains at least one of the `add`, `remove`, or `replace` operations. For examples and more information on the SCIM operations format, see the [SCIM specification](https://tools.ietf.org/html/rfc7644#section-3.5.2).
     *
     * **Note:** Complicated SCIM `path` selectors that include filters are not supported. For example, a `path` selector defined as `"path": "emails[type eq \"work\"]"` will not work.
     *
     * **Warning:** If you set `active:false` using the `replace` operation (as shown in the JSON example below), it removes the user from the enterprise, deletes the external identity, and deletes the associated `:scim_user_id`.
     *
     * ```
     * {
     *   "Operations":[{
     *     "op":"replace",
     *     "value":{
     *       "active":false
     *     }
     *   }]
     * }
     * ```
     *
     * FROM: <https://docs.github.com/rest/reference/enterprise-admin#update-an-attribute-for-a-scim-enterprise-user>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name. You can also substitute this value with the enterprise id.
     * * `scim_user_id: &str` -- scim_user_id parameter.
     */
    pub async fn update_attribute_for_enterprise_user(
        &self,
        enterprise: &str,
        scim_user_id: &str,
        body: &crate::types::EnterpriseAdminUpdateAttributeUserRequest,
    ) -> ClientResult<crate::Response<crate::types::ScimEnterpriseUser>> {
        let url = self.client.url(
            &format!(
                "/scim/v2/enterprises/{}/Users/{}",
                crate::progenitor_support::encode_path(enterprise),
                crate::progenitor_support::encode_path(scim_user_id),
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
}
