use crate::Client;
use crate::ClientResult;

pub struct CodeSecurity {
    pub client: Client,
}

impl CodeSecurity {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CodeSecurity { client }
    }

    /**
     * Get code security configurations for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/code-security/configurations` endpoint.
     *
     * Lists all code security configurations available in an enterprise.
     *
     * The authenticated user must be an administrator of the enterprise in order to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#get-code-security-configurations-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn get_configurations_for_enterprise(
        &self,
        enterprise: &str,
        per_page: i64,
        before: &str,
        after: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeSecurityConfiguration>>> {
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
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/enterprises/{}/code-security/configurations?{}",
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
     * Get code security configurations for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/code-security/configurations` endpoint.
     *
     * As opposed to `get_configurations_for_enterprise`, this function returns all the pages of the request at once.
     *
     * Lists all code security configurations available in an enterprise.
     *
     * The authenticated user must be an administrator of the enterprise in order to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#get-code-security-configurations-for-an-enterprise>
     */
    pub async fn get_all_configurations_for_enterprise(
        &self,
        enterprise: &str,
        before: &str,
        after: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeSecurityConfiguration>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/enterprises/{}/code-security/configurations?{}",
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
     * Create a code security configuration for an enterprise.
     *
     * This function performs a `POST` to the `/enterprises/{enterprise}/code-security/configurations` endpoint.
     *
     * Creates a code security configuration in an enterprise.
     *
     * The authenticated user must be an administrator of the enterprise in order to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#create-a-code-security-configuration-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     */
    pub async fn create_configuration_for_enterprise(
        &self,
        enterprise: &str,
        body: &crate::types::CodeSecurityCreateConfigurationEnterpriseRequest,
    ) -> ClientResult<crate::Response<crate::types::CodeSecurityConfiguration>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/code-security/configurations",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
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
     * Get default code security configurations for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/code-security/configurations/defaults` endpoint.
     *
     * Lists the default code security configurations for an enterprise.
     *
     * The authenticated user must be an administrator of the enterprise in order to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#get-default-code-security-configurations-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     */
    pub async fn get_default_configurations_for_enterprise(
        &self,
        enterprise: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeSecurityDefaultConfigurations>>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/code-security/configurations/defaults",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get default code security configurations for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/code-security/configurations/defaults` endpoint.
     *
     * As opposed to `get_default_configurations_for_enterprise`, this function returns all the pages of the request at once.
     *
     * Lists the default code security configurations for an enterprise.
     *
     * The authenticated user must be an administrator of the enterprise in order to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#get-default-code-security-configurations-for-an-enterprise>
     */
    pub async fn get_all_default_configurations_for_enterprise(
        &self,
        enterprise: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeSecurityDefaultConfigurations>>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/code-security/configurations/defaults",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
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
     * Retrieve a code security configuration of an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/code-security/configurations/{configuration_id}` endpoint.
     *
     * Gets a code security configuration available in an enterprise.
     *
     * The authenticated user must be an administrator of the enterprise in order to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#retrieve-a-code-security-configuration-of-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `configuration_id: i64` -- The unique identifier of the code security configuration.
     */
    pub async fn get_single_configuration_for_enterprise(
        &self,
        enterprise: &str,
        configuration_id: i64,
    ) -> ClientResult<crate::Response<crate::types::CodeSecurityConfiguration>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/code-security/configurations/{}",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
                crate::progenitor_support::encode_path(&configuration_id.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Delete a code security configuration for an enterprise.
     *
     * This function performs a `DELETE` to the `/enterprises/{enterprise}/code-security/configurations/{configuration_id}` endpoint.
     *
     * Deletes a code security configuration from an enterprise.
     * Repositories attached to the configuration will retain their settings but will no longer be associated with
     * the configuration.
     *
     * The authenticated user must be an administrator for the enterprise to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#delete-a-code-security-configuration-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `configuration_id: i64` -- The unique identifier of the code security configuration.
     */
    pub async fn delete_configuration_for_enterprise(
        &self,
        enterprise: &str,
        configuration_id: i64,
    ) -> ClientResult<crate::Response<String>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/code-security/configurations/{}",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
                crate::progenitor_support::encode_path(&configuration_id.to_string()),
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
     * Update a custom code security configuration for an enterprise.
     *
     * This function performs a `PATCH` to the `/enterprises/{enterprise}/code-security/configurations/{configuration_id}` endpoint.
     *
     * Updates a code security configuration in an enterprise.
     *
     * The authenticated user must be an administrator of the enterprise in order to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#update-a-custom-code-security-configuration-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `configuration_id: i64` -- The unique identifier of the code security configuration.
     */
    pub async fn update_enterprise_configuration(
        &self,
        enterprise: &str,
        configuration_id: i64,
        body: &crate::types::CodeSecurityUpdateEnterpriseConfigurationRequest,
    ) -> ClientResult<crate::Response<crate::types::CodeSecurityConfiguration>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/code-security/configurations/{}",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
                crate::progenitor_support::encode_path(&configuration_id.to_string()),
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
     * Attach an enterprise configuration to repositories.
     *
     * This function performs a `POST` to the `/enterprises/{enterprise}/code-security/configurations/{configuration_id}/attach` endpoint.
     *
     * Attaches an enterprise code security configuration to repositories. If the repositories specified are already attached to a configuration, they will be re-attached to the provided configuration.
     *
     * If insufficient GHAS licenses are available to attach the configuration to a repository, only free features will be enabled.
     *
     * The authenticated user must be an administrator for the enterprise to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#attach-an-enterprise-configuration-to-repositories>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `configuration_id: i64` -- The unique identifier of the code security configuration.
     */
    pub async fn attach_enterprise_configuration(
        &self,
        enterprise: &str,
        configuration_id: i64,
        body: &crate::types::CodeSecurityAttachEnterpriseConfigurationRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/code-security/configurations/{}/attach",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
                crate::progenitor_support::encode_path(&configuration_id.to_string()),
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
     * Set a code security configuration as a default for an enterprise.
     *
     * This function performs a `PUT` to the `/enterprises/{enterprise}/code-security/configurations/{configuration_id}/defaults` endpoint.
     *
     * Sets a code security configuration as a default to be applied to new repositories in your enterprise.
     *
     * This configuration will be applied by default to the matching repository type when created, but only for organizations within the enterprise that do not already have a default code security configuration set.
     *
     * The authenticated user must be an administrator for the enterprise to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#set-a-code-security-configuration-as-a-default-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `configuration_id: i64` -- The unique identifier of the code security configuration.
     */
    pub async fn set_configuration_as_default_for_enterprise(
        &self,
        enterprise: &str,
        configuration_id: i64,
        body: &crate::types::CodeSecuritySetConfigurationAsDefaultRequest,
    ) -> ClientResult<crate::Response<crate::types::CodeSecuritySetConfigurationAsDefaultResponse>>
    {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/code-security/configurations/{}/defaults",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
                crate::progenitor_support::encode_path(&configuration_id.to_string()),
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
     * Get repositories associated with an enterprise code security configuration.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/code-security/configurations/{configuration_id}/repositories` endpoint.
     *
     * Lists the repositories associated with an enterprise code security configuration in an organization.
     *
     * The authenticated user must be an administrator of the enterprise in order to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#get-repositories-associated-with-an-enterprise-code-security-configuration>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     * * `configuration_id: i64` -- The unique identifier of the code security configuration.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `status: &str` -- A comma-separated list of statuses. If specified, only repositories with these attachment statuses will be returned.
     *   
     *   Can be: `all`, `attached`, `attaching`, `removed`, `enforced`, `failed`, `updating`, `removed_by_enterprise`.
     */
    pub async fn get_repositories_for_enterprise_configuration(
        &self,
        enterprise: &str,
        configuration_id: i64,
        per_page: i64,
        before: &str,
        after: &str,
        status: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeSecurityConfigurationRepositories>>>
    {
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
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/enterprises/{}/code-security/configurations/{}/repositories?{}",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
                crate::progenitor_support::encode_path(&configuration_id.to_string()),
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
     * Get repositories associated with an enterprise code security configuration.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/code-security/configurations/{configuration_id}/repositories` endpoint.
     *
     * As opposed to `get_repositories_for_enterprise_configuration`, this function returns all the pages of the request at once.
     *
     * Lists the repositories associated with an enterprise code security configuration in an organization.
     *
     * The authenticated user must be an administrator of the enterprise in order to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#get-repositories-associated-with-an-enterprise-code-security-configuration>
     */
    pub async fn get_all_repositories_for_enterprise_configuration(
        &self,
        enterprise: &str,
        configuration_id: i64,
        before: &str,
        after: &str,
        status: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeSecurityConfigurationRepositories>>>
    {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/enterprises/{}/code-security/configurations/{}/repositories?{}",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
                crate::progenitor_support::encode_path(&configuration_id.to_string()),
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
     * Get code security configurations for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/code-security/configurations` endpoint.
     *
     * Lists all code security configurations available in an organization.
     *
     * The authenticated user must be an administrator or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#get-code-security-configurations-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `target_type: crate::types::CodeSecurityGetConfigurationsOrgTargetType` -- The target type of the code security configuration.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn get_configurations_for_org(
        &self,
        org: &str,
        target_type: crate::types::CodeSecurityGetConfigurationsOrgTargetType,
        per_page: i64,
        before: &str,
        after: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeSecurityConfiguration>>> {
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
        if !target_type.to_string().is_empty() {
            query_args.push(("target_type".to_string(), target_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/code-security/configurations?{}",
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
     * Get code security configurations for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/code-security/configurations` endpoint.
     *
     * As opposed to `get_configurations_for_org`, this function returns all the pages of the request at once.
     *
     * Lists all code security configurations available in an organization.
     *
     * The authenticated user must be an administrator or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#get-code-security-configurations-for-an-organization>
     */
    pub async fn get_all_configurations_for_org(
        &self,
        org: &str,
        target_type: crate::types::CodeSecurityGetConfigurationsOrgTargetType,
        before: &str,
        after: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeSecurityConfiguration>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !target_type.to_string().is_empty() {
            query_args.push(("target_type".to_string(), target_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/code-security/configurations?{}",
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
     * Create a code security configuration.
     *
     * This function performs a `POST` to the `/orgs/{org}/code-security/configurations` endpoint.
     *
     * Creates a code security configuration in an organization.
     *
     * The authenticated user must be an administrator or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#create-a-code-security-configuration>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn create_configuration(
        &self,
        org: &str,
        body: &crate::types::CodeSecurityCreateConfigurationRequest,
    ) -> ClientResult<crate::Response<crate::types::CodeSecurityConfiguration>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/code-security/configurations",
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
     * Get default code security configurations.
     *
     * This function performs a `GET` to the `/orgs/{org}/code-security/configurations/defaults` endpoint.
     *
     * Lists the default code security configurations for an organization.
     *
     * The authenticated user must be an administrator or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#get-default-code-security-configurations>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_default_configurations(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeSecurityDefaultConfigurations>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/code-security/configurations/defaults",
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
     * Get default code security configurations.
     *
     * This function performs a `GET` to the `/orgs/{org}/code-security/configurations/defaults` endpoint.
     *
     * As opposed to `get_default_configurations`, this function returns all the pages of the request at once.
     *
     * Lists the default code security configurations for an organization.
     *
     * The authenticated user must be an administrator or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#get-default-code-security-configurations>
     */
    pub async fn get_all_default_configurations(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeSecurityDefaultConfigurations>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/code-security/configurations/defaults",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Detach configurations from repositories.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/code-security/configurations/detach` endpoint.
     *
     * Detach code security configuration(s) from a set of repositories.
     * Repositories will retain their settings but will no longer be associated with the configuration.
     *
     * The authenticated user must be an administrator or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#detach-configurations-from-repositories>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn detach_configuration(
        &self,
        org: &str,
        body: &crate::types::CodeSecurityDetachConfigurationRequest,
    ) -> ClientResult<crate::Response<String>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/code-security/configurations/detach",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Get a code security configuration.
     *
     * This function performs a `GET` to the `/orgs/{org}/code-security/configurations/{configuration_id}` endpoint.
     *
     * Gets a code security configuration available in an organization.
     *
     * The authenticated user must be an administrator or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#get-a-code-security-configuration>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `configuration_id: i64` -- The unique identifier of the code security configuration.
     */
    pub async fn get_configuration(
        &self,
        org: &str,
        configuration_id: i64,
    ) -> ClientResult<crate::Response<crate::types::CodeSecurityConfiguration>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/code-security/configurations/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&configuration_id.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Delete a code security configuration.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/code-security/configurations/{configuration_id}` endpoint.
     *
     * Deletes the desired code security configuration from an organization.
     * Repositories attached to the configuration will retain their settings but will no longer be associated with
     * the configuration.
     *
     * The authenticated user must be an administrator or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#delete-a-code-security-configuration>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `configuration_id: i64` -- The unique identifier of the code security configuration.
     */
    pub async fn delete_configuration(
        &self,
        org: &str,
        configuration_id: i64,
    ) -> ClientResult<crate::Response<String>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/code-security/configurations/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&configuration_id.to_string()),
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
     * Update a code security configuration.
     *
     * This function performs a `PATCH` to the `/orgs/{org}/code-security/configurations/{configuration_id}` endpoint.
     *
     * Updates a code security configuration in an organization.
     *
     * The authenticated user must be an administrator or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#update-a-code-security-configuration>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `configuration_id: i64` -- The unique identifier of the code security configuration.
     */
    pub async fn update_configuration(
        &self,
        org: &str,
        configuration_id: i64,
        body: &crate::types::CodeSecurityUpdateConfigurationRequest,
    ) -> ClientResult<crate::Response<crate::types::CodeSecurityConfiguration>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/code-security/configurations/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&configuration_id.to_string()),
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
     * Attach a configuration to repositories.
     *
     * This function performs a `POST` to the `/orgs/{org}/code-security/configurations/{configuration_id}/attach` endpoint.
     *
     * Attach a code security configuration to a set of repositories. If the repositories specified are already attached to a configuration, they will be re-attached to the provided configuration.
     *
     * If insufficient GHAS licenses are available to attach the configuration to a repository, only free features will be enabled.
     *
     * The authenticated user must be an administrator or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#attach-a-configuration-to-repositories>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `configuration_id: i64` -- The unique identifier of the code security configuration.
     */
    pub async fn attach_configuration(
        &self,
        org: &str,
        configuration_id: i64,
        body: &crate::types::CodeSecurityAttachConfigurationRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/code-security/configurations/{}/attach",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&configuration_id.to_string()),
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
     * Set a code security configuration as a default for an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/code-security/configurations/{configuration_id}/defaults` endpoint.
     *
     * Sets a code security configuration as a default to be applied to new repositories in your organization.
     *
     * This configuration will be applied to the matching repository type (all, none, public, private and internal) by default when they are created.
     *
     * The authenticated user must be an administrator or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `write:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#set-a-code-security-configuration-as-a-default-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `configuration_id: i64` -- The unique identifier of the code security configuration.
     */
    pub async fn set_configuration_as_default(
        &self,
        org: &str,
        configuration_id: i64,
        body: &crate::types::CodeSecuritySetConfigurationAsDefaultRequest,
    ) -> ClientResult<crate::Response<crate::types::CodeSecuritySetConfigurationAsDefaultResponse>>
    {
        let url = self.client.url(
            &format!(
                "/orgs/{}/code-security/configurations/{}/defaults",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&configuration_id.to_string()),
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
     * Get repositories associated with a code security configuration.
     *
     * This function performs a `GET` to the `/orgs/{org}/code-security/configurations/{configuration_id}/repositories` endpoint.
     *
     * Lists the repositories associated with a code security configuration in an organization.
     *
     * The authenticated user must be an administrator or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#get-repositories-associated-with-a-code-security-configuration>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `configuration_id: i64` -- The unique identifier of the code security configuration.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `before: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results before this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `after: &str` -- A cursor, as given in the [Link header](https://docs.github.com/rest/guides/using-pagination-in-the-rest-api#using-link-headers). If specified, the query only searches for results after this cursor. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `status: &str` -- A comma-separated list of statuses. If specified, only repositories with these attachment statuses will be returned.
     *   
     *   Can be: `all`, `attached`, `attaching`, `detached`, `removed`, `enforced`, `failed`, `updating`, `removed_by_enterprise`.
     */
    pub async fn get_repositories_for_configuration(
        &self,
        org: &str,
        configuration_id: i64,
        per_page: i64,
        before: &str,
        after: &str,
        status: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeSecurityConfigurationRepositories>>>
    {
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
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/code-security/configurations/{}/repositories?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&configuration_id.to_string()),
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
     * Get repositories associated with a code security configuration.
     *
     * This function performs a `GET` to the `/orgs/{org}/code-security/configurations/{configuration_id}/repositories` endpoint.
     *
     * As opposed to `get_repositories_for_configuration`, this function returns all the pages of the request at once.
     *
     * Lists the repositories associated with a code security configuration in an organization.
     *
     * The authenticated user must be an administrator or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#get-repositories-associated-with-a-code-security-configuration>
     */
    pub async fn get_all_repositories_for_configuration(
        &self,
        org: &str,
        configuration_id: i64,
        before: &str,
        after: &str,
        status: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeSecurityConfigurationRepositories>>>
    {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !after.is_empty() {
            query_args.push(("after".to_string(), after.to_string()));
        }
        if !before.is_empty() {
            query_args.push(("before".to_string(), before.to_string()));
        }
        if !status.is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/code-security/configurations/{}/repositories?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&configuration_id.to_string()),
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
     * Get the code security configuration associated with a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/code-security-configuration` endpoint.
     *
     * Get the code security configuration that manages a repository's code security settings.
     *
     * The authenticated user must be an administrator or security manager for the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/code-security/configurations#get-the-code-security-configuration-associated-with-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_configuration_for_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::CodeSecurityConfigurationRepository>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/code-security-configuration",
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
}
