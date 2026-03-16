use crate::Client;
use crate::ClientResult;

pub struct Codespaces {
    pub client: Client,
}

impl Codespaces {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Codespaces { client }
    }

    /**
     * List codespaces for the organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/codespaces` endpoint.
     *
     * Lists the codespaces associated to a specified organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/organizations#list-codespaces-for-the-organization>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn list_in_organization(
        &self,
        per_page: i64,
        page: i64,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::CodespacesListResponse>> {
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
                "/orgs/{}/codespaces?{}",
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
     * Manage access control for organization codespaces.
     *
     * This function performs a `PUT` to the `/orgs/{org}/codespaces/access` endpoint.
     *
     * Sets which users can access codespaces in an organization. This is synonymous with granting or revoking codespaces access permissions for users according to the visibility.
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/organizations#manage-access-control-for-organization-codespaces>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn set_codespaces_access(
        &self,
        org: &str,
        body: &crate::types::CodespacesSetAccessRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/codespaces/access",
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
     * Add users to Codespaces access for an organization.
     *
     * This function performs a `POST` to the `/orgs/{org}/codespaces/access/selected_users` endpoint.
     *
     * Codespaces for the specified users will be billed to the organization.
     *
     * To use this endpoint, the access settings for the organization must be set to `selected_members`.
     * For information on how to change this setting, see "[Manage access control for organization codespaces](https://docs.github.com/rest/codespaces/organizations#manage-access-control-for-organization-codespaces)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/organizations#add-users-to-codespaces-access-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn set_codespaces_access_users(
        &self,
        org: &str,
        body: &crate::types::CopilotAddSeatsUsersRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/codespaces/access/selected_users",
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
     * Remove users from Codespaces access for an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/codespaces/access/selected_users` endpoint.
     *
     * Codespaces for the specified users will no longer be billed to the organization.
     *
     * To use this endpoint, the access settings for the organization must be set to `selected_members`.
     * For information on how to change this setting, see "[Manage access control for organization codespaces](https://docs.github.com/rest/codespaces/organizations#manage-access-control-for-organization-codespaces)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/organizations#remove-users-from-codespaces-access-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn delete_codespaces_access_users(
        &self,
        org: &str,
        body: &crate::types::CopilotAddSeatsUsersRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/codespaces/access/selected_users",
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
     * List organization secrets.
     *
     * This function performs a `GET` to the `/orgs/{org}/codespaces/secrets` endpoint.
     *
     * Lists all Codespaces development environment secrets available at the organization-level without revealing their encrypted
     * values.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/organization-secrets#list-organization-secrets>
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
    ) -> ClientResult<crate::Response<crate::types::CodespacesListOrgSecretsResponse>> {
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
                "/orgs/{}/codespaces/secrets?{}",
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
     * This function performs a `GET` to the `/orgs/{org}/codespaces/secrets/public-key` endpoint.
     *
     * Gets a public key for an organization, which is required in order to encrypt secrets. You need to encrypt the value of a secret before you can create or update secrets.
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/organization-secrets#get-an-organization-public-key>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_org_public_key(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::CodespacesPublicKey>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/codespaces/secrets/public-key",
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
     * This function performs a `GET` to the `/orgs/{org}/codespaces/secrets/{secret_name}` endpoint.
     *
     * Gets an organization development environment secret without revealing its encrypted value.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/organization-secrets#get-an-organization-secret>
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
    ) -> ClientResult<crate::Response<crate::types::CodespacesSecret>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/codespaces/secrets/{}",
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
     * This function performs a `PUT` to the `/orgs/{org}/codespaces/secrets/{secret_name}` endpoint.
     *
     * Creates or updates an organization development environment secret with an encrypted value. Encrypt your secret using
     * [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/organization-secrets#create-or-update-an-organization-secret>
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
        body: &crate::types::CodespacesCreateUpdateOrgSecretRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/codespaces/secrets/{}",
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
     * This function performs a `DELETE` to the `/orgs/{org}/codespaces/secrets/{secret_name}` endpoint.
     *
     * Deletes an organization development environment secret using the secret name.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/organization-secrets#delete-an-organization-secret>
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
                "/orgs/{}/codespaces/secrets/{}",
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
     * This function performs a `GET` to the `/orgs/{org}/codespaces/secrets/{secret_name}/repositories` endpoint.
     *
     * Lists all repositories that have been selected when the `visibility`
     * for repository access to a secret is set to `selected`.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/organization-secrets#list-selected-repositories-for-an-organization-secret>
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
                "/orgs/{}/codespaces/secrets/{}/repositories?{}",
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
     * This function performs a `PUT` to the `/orgs/{org}/codespaces/secrets/{secret_name}/repositories` endpoint.
     *
     * Replaces all repositories for an organization development environment secret when the `visibility`
     * for repository access is set to `selected`. The visibility is set when you [Create
     * or update an organization secret](https://docs.github.com/rest/codespaces/organization-secrets#create-or-update-an-organization-secret).
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/organization-secrets#set-selected-repositories-for-an-organization-secret>
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
                "/orgs/{}/codespaces/secrets/{}/repositories",
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
     * This function performs a `PUT` to the `/orgs/{org}/codespaces/secrets/{secret_name}/repositories/{repository_id}` endpoint.
     *
     * Adds a repository to an organization development environment secret when the `visibility` for repository access is set to `selected`. The visibility is set when you [Create or update an organization secret](https://docs.github.com/rest/codespaces/organization-secrets#create-or-update-an-organization-secret).
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/organization-secrets#add-selected-repository-to-an-organization-secret>
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
                "/orgs/{}/codespaces/secrets/{}/repositories/{}",
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
     * This function performs a `DELETE` to the `/orgs/{org}/codespaces/secrets/{secret_name}/repositories/{repository_id}` endpoint.
     *
     * Removes a repository from an organization development environment secret when the `visibility`
     * for repository access is set to `selected`. The visibility is set when you [Create
     * or update an organization secret](https://docs.github.com/rest/codespaces/organization-secrets#create-or-update-an-organization-secret).
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/organization-secrets#remove-selected-repository-from-an-organization-secret>
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
                "/orgs/{}/codespaces/secrets/{}/repositories/{}",
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
     * List codespaces for a user in organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/members/{username}/codespaces` endpoint.
     *
     * Lists the codespaces that a member of an organization has for repositories in that organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/organizations#list-codespaces-for-a-user-in-organization>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn get_codespaces_for_user_in_org(
        &self,
        per_page: i64,
        page: i64,
        org: &str,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::CodespacesListResponse>> {
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
                "/orgs/{}/members/{}/codespaces?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Delete a codespace from the organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/members/{username}/codespaces/{codespace_name}` endpoint.
     *
     * Deletes a user's codespace.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/organizations#delete-a-codespace-from-the-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     * * `codespace_name: &str` -- The name of the codespace.
     */
    pub async fn delete_from_organization(
        &self,
        org: &str,
        username: &str,
        codespace_name: &str,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/members/{}/codespaces/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&codespace_name.to_string()),
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
     * Stop a codespace for an organization user.
     *
     * This function performs a `POST` to the `/orgs/{org}/members/{username}/codespaces/{codespace_name}/stop` endpoint.
     *
     * Stops a user's codespace.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/organizations#stop-a-codespace-for-an-organization-user>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `username: &str` -- The handle for the GitHub user account.
     * * `codespace_name: &str` -- The name of the codespace.
     */
    pub async fn stop_in_organization(
        &self,
        org: &str,
        username: &str,
        codespace_name: &str,
    ) -> ClientResult<crate::Response<crate::types::Codespace>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/members/{}/codespaces/{}/stop",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&codespace_name.to_string()),
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
     * List codespaces in a repository for the authenticated user.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/codespaces` endpoint.
     *
     * Lists the codespaces associated to a specified repository and the authenticated user.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/codespaces#list-codespaces-in-a-repository-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn list_in_repository_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::CodespacesListResponse>> {
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
                "/repos/{}/{}/codespaces?{}",
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
     * Create a codespace in a repository.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/codespaces` endpoint.
     *
     * Creates a codespace owned by the authenticated user in the specified repository.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/codespaces#create-a-codespace-in-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_with_repo_for_authenticated_user(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::CodespacesCreateWithRepoRequest,
    ) -> ClientResult<crate::Response<crate::types::Codespace>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/codespaces",
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
     * List devcontainer configurations in a repository for the authenticated user.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/codespaces/devcontainers` endpoint.
     *
     * Lists the devcontainer.json files associated with a specified repository and the authenticated user. These files
     * specify launchpoint configurations for codespaces created within the repository.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/codespaces#list-devcontainer-configurations-in-a-repository-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn list_devcontainers_in_repository_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::CodespacesListDevcontainersInRepositoryResponse>>
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
                "/repos/{}/{}/codespaces/devcontainers?{}",
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
     * List available machine types for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/codespaces/machines` endpoint.
     *
     * List the machine types available for a given repository based on its configuration.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/machines#list-available-machine-types-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `location: &str` -- The location to check for available machines. Assigned by IP if not provided.
     * * `client_ip: &str` -- IP for location auto-detection when proxying a request.
     * * `ref_: &str` -- The branch or commit to check for prebuild availability and devcontainer restrictions.
     */
    pub async fn repo_machines_for_authenticated_user(
        &self,
        owner: &str,
        repo: &str,
        location: &str,
        client_ip: &str,
        ref_: &str,
    ) -> ClientResult<crate::Response<crate::types::CodespacesRepoMachinesResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !client_ip.is_empty() {
            query_args.push(("client_ip".to_string(), client_ip.to_string()));
        }
        if !location.is_empty() {
            query_args.push(("location".to_string(), location.to_string()));
        }
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/codespaces/machines?{}",
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
     * Get default attributes for a codespace.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/codespaces/new` endpoint.
     *
     * Gets the default attributes for codespaces created by the user with the repository.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/codespaces#get-default-attributes-for-a-codespace>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `ref_: &str` -- The branch or commit to check for a default devcontainer path. If not specified, the default branch will be checked.
     * * `client_ip: &str` -- An alternative IP for default location auto-detection, such as when proxying a request.
     */
    pub async fn pre_flight_with_repo_for_authenticated_user(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        client_ip: &str,
    ) -> ClientResult<crate::Response<crate::types::CodespacesPreFlightWithRepoResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !client_ip.is_empty() {
            query_args.push(("client_ip".to_string(), client_ip.to_string()));
        }
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/codespaces/new?{}",
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
     * Check if permissions defined by a devcontainer have been accepted by the authenticated user.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/codespaces/permissions_check` endpoint.
     *
     * Checks whether the permissions defined by a given devcontainer configuration have been accepted by the authenticated user.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/codespaces#check-if-permissions-defined-by-a-devcontainer-have-been-accepted-by-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `ref_: &str` -- The git reference that points to the location of the devcontainer configuration to use for the permission check. The value of `ref` will typically be a branch name (`heads/BRANCH_NAME`). For more information, see "[Git References](https://git-scm.com/book/en/v2/Git-Internals-Git-References)" in the Git documentation.
     * * `devcontainer_path: &str` -- Path to the devcontainer.json configuration to use for the permission check.
     */
    pub async fn check_permissions_for_devcontainer(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        devcontainer_path: &str,
    ) -> ClientResult<crate::Response<crate::types::Submission>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !devcontainer_path.is_empty() {
            query_args.push((
                "devcontainer_path".to_string(),
                devcontainer_path.to_string(),
            ));
        }
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/codespaces/permissions_check?{}",
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
     * List repository secrets.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/codespaces/secrets` endpoint.
     *
     * Lists all development environment secrets available in a repository without revealing their encrypted
     * values.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/repository-secrets#list-repository-secrets>
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
    ) -> ClientResult<crate::Response<crate::types::CodespacesListRepoSecretsResponse>> {
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
                "/repos/{}/{}/codespaces/secrets?{}",
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
     * This function performs a `GET` to the `/repos/{owner}/{repo}/codespaces/secrets/public-key` endpoint.
     *
     * Gets your public key, which you need to encrypt secrets. You need to
     * encrypt a secret before you can create or update secrets.
     *
     * If the repository is private, OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/repository-secrets#get-a-repository-public-key>
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
    ) -> ClientResult<crate::Response<crate::types::CodespacesPublicKey>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/codespaces/secrets/public-key",
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
     * This function performs a `GET` to the `/repos/{owner}/{repo}/codespaces/secrets/{secret_name}` endpoint.
     *
     * Gets a single repository development environment secret without revealing its encrypted value.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/repository-secrets#get-a-repository-secret>
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
                "/repos/{}/{}/codespaces/secrets/{}",
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
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/codespaces/secrets/{secret_name}` endpoint.
     *
     * Creates or updates a repository development environment secret with an encrypted value. Encrypt your secret using
     * [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint. The associated user must be a repository admin.
     *
     * FROM: <https://docs.github.com/rest/codespaces/repository-secrets#create-or-update-a-repository-secret>
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
                "/repos/{}/{}/codespaces/secrets/{}",
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
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/codespaces/secrets/{secret_name}` endpoint.
     *
     * Deletes a development environment secret in a repository using the secret name.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint. The associated user must be a repository admin.
     *
     * FROM: <https://docs.github.com/rest/codespaces/repository-secrets#delete-a-repository-secret>
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
                "/repos/{}/{}/codespaces/secrets/{}",
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
    /**
     * Create a codespace from a pull request.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/pulls/{pull_number}/codespaces` endpoint.
     *
     * Creates a codespace owned by the authenticated user for the specified pull request.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/codespaces#create-a-codespace-from-a-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `pull_number: i64` -- The number that identifies the pull request.
     */
    pub async fn create_with_pr_for_authenticated_user(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        body: &crate::types::CodespacesCreateWithPrRequest,
    ) -> ClientResult<crate::Response<crate::types::Codespace>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/pulls/{}/codespaces",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&pull_number.to_string()),
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
     * List codespaces for the authenticated user.
     *
     * This function performs a `GET` to the `/user/codespaces` endpoint.
     *
     * Lists the authenticated user's codespaces.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/codespaces#list-codespaces-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `repository_id: i64` -- ID of the Repository to filter on.
     */
    pub async fn list_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
        repository_id: i64,
    ) -> ClientResult<crate::Response<crate::types::CodespacesListResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if repository_id > 0 {
            query_args.push(("repository_id".to_string(), repository_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/user/codespaces?{}", query_), None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Create a codespace for the authenticated user.
     *
     * This function performs a `POST` to the `/user/codespaces` endpoint.
     *
     * Creates a new codespace, owned by the authenticated user.
     *
     * This endpoint requires either a `repository_id` OR a `pull_request` but not both.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/codespaces#create-a-codespace-for-the-authenticated-user>
     */
    pub async fn create_for_authenticated_user(
        &self,
        body: &crate::types::CodespacesCreateRequestOneOf,
    ) -> ClientResult<crate::Response<crate::types::Codespace>> {
        let url = self.client.url(&"/user/codespaces".to_string(), None);
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
     * List secrets for the authenticated user.
     *
     * This function performs a `GET` to the `/user/codespaces/secrets` endpoint.
     *
     * Lists all development environment secrets available for a user's codespaces without revealing their
     * encrypted values.
     *
     * The authenticated user must have Codespaces access to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/secrets#list-secrets-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_secrets_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::CodespacesListSecretsResponse>> {
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
            .url(&format!("/user/codespaces/secrets?{}", query_), None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get public key for the authenticated user.
     *
     * This function performs a `GET` to the `/user/codespaces/secrets/public-key` endpoint.
     *
     * Gets your public key, which you need to encrypt secrets. You need to encrypt a secret before you can create or update secrets.
     *
     * The authenticated user must have Codespaces access to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/secrets#get-public-key-for-the-authenticated-user>
     */
    pub async fn get_public_key_for_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<crate::types::DependabotPublicKey>> {
        let url = self
            .client
            .url(&"/user/codespaces/secrets/public-key".to_string(), None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get a secret for the authenticated user.
     *
     * This function performs a `GET` to the `/user/codespaces/secrets/{secret_name}` endpoint.
     *
     * Gets a development environment secret available to a user's codespaces without revealing its encrypted value.
     *
     * The authenticated user must have Codespaces access to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/secrets#get-a-secret-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `secret_name: &str` -- The name of the secret.
     */
    pub async fn get_secret_for_authenticated_user(
        &self,
        secret_name: &str,
    ) -> ClientResult<crate::Response<crate::types::CodespacesSecretData>> {
        let url = self.client.url(
            &format!(
                "/user/codespaces/secrets/{}",
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
     * Create or update a secret for the authenticated user.
     *
     * This function performs a `PUT` to the `/user/codespaces/secrets/{secret_name}` endpoint.
     *
     * Creates or updates a development environment secret for a user's codespace with an encrypted value. Encrypt your secret using
     * [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
     *
     * The authenticated user must have Codespaces access to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/secrets#create-or-update-a-secret-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `secret_name: &str` -- The name of the secret.
     */
    pub async fn create_or_update_secret_for_authenticated_user(
        &self,
        secret_name: &str,
        body: &crate::types::CodespacesCreateUpdateSecretRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/user/codespaces/secrets/{}",
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
     * Delete a secret for the authenticated user.
     *
     * This function performs a `DELETE` to the `/user/codespaces/secrets/{secret_name}` endpoint.
     *
     * Deletes a development environment secret from a user's codespaces using the secret name. Deleting the secret will remove access from all codespaces that were allowed to access the secret.
     *
     * The authenticated user must have Codespaces access to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/secrets#delete-a-secret-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `secret_name: &str` -- The name of the secret.
     */
    pub async fn delete_secret_for_authenticated_user(
        &self,
        secret_name: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/codespaces/secrets/{}",
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
     * List selected repositories for a user secret.
     *
     * This function performs a `GET` to the `/user/codespaces/secrets/{secret_name}/repositories` endpoint.
     *
     * List the repositories that have been granted the ability to use a user's development environment secret.
     *
     * The authenticated user must have Codespaces access to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/secrets#list-selected-repositories-for-a-user-secret>
     *
     * **Parameters:**
     *
     * * `secret_name: &str` -- The name of the secret.
     */
    pub async fn list_repositories_for_secret_for_authenticated_user(
        &self,
        secret_name: &str,
    ) -> ClientResult<crate::Response<crate::types::CodespacesListRepositoriesSecretResponse>> {
        let url = self.client.url(
            &format!(
                "/user/codespaces/secrets/{}/repositories",
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
     * Set selected repositories for a user secret.
     *
     * This function performs a `PUT` to the `/user/codespaces/secrets/{secret_name}/repositories` endpoint.
     *
     * Select the repositories that will use a user's development environment secret.
     *
     * The authenticated user must have Codespaces access to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/secrets#set-selected-repositories-for-a-user-secret>
     *
     * **Parameters:**
     *
     * * `secret_name: &str` -- The name of the secret.
     */
    pub async fn set_repositories_for_secret_for_authenticated_user(
        &self,
        secret_name: &str,
        body: &crate::types::CodespacesSetRepositoriesSecretRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/codespaces/secrets/{}/repositories",
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
     * Add a selected repository to a user secret.
     *
     * This function performs a `PUT` to the `/user/codespaces/secrets/{secret_name}/repositories/{repository_id}` endpoint.
     *
     * Adds a repository to the selected repositories for a user's development environment secret.
     *
     * The authenticated user must have Codespaces access to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/secrets#add-a-selected-repository-to-a-user-secret>
     *
     * **Parameters:**
     *
     * * `secret_name: &str` -- The name of the secret.
     * * `repository_id: i64`
     */
    pub async fn add_repository_for_secret_for_authenticated_user(
        &self,
        secret_name: &str,
        repository_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/codespaces/secrets/{}/repositories/{}",
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
     * Remove a selected repository from a user secret.
     *
     * This function performs a `DELETE` to the `/user/codespaces/secrets/{secret_name}/repositories/{repository_id}` endpoint.
     *
     * Removes a repository from the selected repositories for a user's development environment secret.
     *
     * The authenticated user must have Codespaces access to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` or `codespace:secrets` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/secrets#remove-a-selected-repository-from-a-user-secret>
     *
     * **Parameters:**
     *
     * * `secret_name: &str` -- The name of the secret.
     * * `repository_id: i64`
     */
    pub async fn remove_repository_for_secret_for_authenticated_user(
        &self,
        secret_name: &str,
        repository_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/codespaces/secrets/{}/repositories/{}",
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
     * Get a codespace for the authenticated user.
     *
     * This function performs a `GET` to the `/user/codespaces/{codespace_name}` endpoint.
     *
     * Gets information about a user's codespace.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/codespaces#get-a-codespace-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `codespace_name: &str` -- The name of the codespace.
     */
    pub async fn get_for_authenticated_user(
        &self,
        codespace_name: &str,
    ) -> ClientResult<crate::Response<crate::types::Codespace>> {
        let url = self.client.url(
            &format!(
                "/user/codespaces/{}",
                crate::progenitor_support::encode_path(&codespace_name.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Delete a codespace for the authenticated user.
     *
     * This function performs a `DELETE` to the `/user/codespaces/{codespace_name}` endpoint.
     *
     * Deletes a user's codespace.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/codespaces#delete-a-codespace-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `codespace_name: &str` -- The name of the codespace.
     */
    pub async fn delete_for_authenticated_user(
        &self,
        codespace_name: &str,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/user/codespaces/{}",
                crate::progenitor_support::encode_path(&codespace_name.to_string()),
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
     * Update a codespace for the authenticated user.
     *
     * This function performs a `PATCH` to the `/user/codespaces/{codespace_name}` endpoint.
     *
     * Updates a codespace owned by the authenticated user. Currently only the codespace's machine type and recent folders can be modified using this endpoint.
     *
     * If you specify a new machine type it will be applied the next time your codespace is started.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/codespaces#update-a-codespace-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `codespace_name: &str` -- The name of the codespace.
     */
    pub async fn update_for_authenticated_user(
        &self,
        codespace_name: &str,
        body: &crate::types::CodespacesUpdateRequest,
    ) -> ClientResult<crate::Response<crate::types::Codespace>> {
        let url = self.client.url(
            &format!(
                "/user/codespaces/{}",
                crate::progenitor_support::encode_path(&codespace_name.to_string()),
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
     * Export a codespace for the authenticated user.
     *
     * This function performs a `POST` to the `/user/codespaces/{codespace_name}/exports` endpoint.
     *
     * Triggers an export of the specified codespace and returns a URL and ID where the status of the export can be monitored.
     *
     * If changes cannot be pushed to the codespace's repository, they will be pushed to a new or previously-existing fork instead.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/codespaces#export-a-codespace-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `codespace_name: &str` -- The name of the codespace.
     */
    pub async fn export_for_authenticated_user(
        &self,
        codespace_name: &str,
    ) -> ClientResult<crate::Response<crate::types::CodespaceExportDetails>> {
        let url = self.client.url(
            &format!(
                "/user/codespaces/{}/exports",
                crate::progenitor_support::encode_path(&codespace_name.to_string()),
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
     * Get details about a codespace export.
     *
     * This function performs a `GET` to the `/user/codespaces/{codespace_name}/exports/{export_id}` endpoint.
     *
     * Gets information about an export of a codespace.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/codespaces#get-details-about-a-codespace-export>
     *
     * **Parameters:**
     *
     * * `codespace_name: &str` -- The name of the codespace.
     * * `export_id: &str` -- The ID of the export operation, or `latest`. Currently only `latest` is currently supported.
     */
    pub async fn get_export_details_for_authenticated_user(
        &self,
        codespace_name: &str,
        export_id: &str,
    ) -> ClientResult<crate::Response<crate::types::CodespaceExportDetails>> {
        let url = self.client.url(
            &format!(
                "/user/codespaces/{}/exports/{}",
                crate::progenitor_support::encode_path(&codespace_name.to_string()),
                crate::progenitor_support::encode_path(&export_id.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * List machine types for a codespace.
     *
     * This function performs a `GET` to the `/user/codespaces/{codespace_name}/machines` endpoint.
     *
     * List the machine types a codespace can transition to use.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/machines#list-machine-types-for-a-codespace>
     *
     * **Parameters:**
     *
     * * `codespace_name: &str` -- The name of the codespace.
     */
    pub async fn codespace_machines_for_authenticated_user(
        &self,
        codespace_name: &str,
    ) -> ClientResult<crate::Response<crate::types::CodespacesRepoMachinesResponse>> {
        let url = self.client.url(
            &format!(
                "/user/codespaces/{}/machines",
                crate::progenitor_support::encode_path(&codespace_name.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Create a repository from an unpublished codespace.
     *
     * This function performs a `POST` to the `/user/codespaces/{codespace_name}/publish` endpoint.
     *
     * Publishes an unpublished codespace, creating a new repository and assigning it to the codespace.
     *
     * The codespace's token is granted write permissions to the repository, allowing the user to push their changes.
     *
     * This will fail for a codespace that is already published, meaning it has an associated repository.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/codespaces#create-a-repository-from-an-unpublished-codespace>
     *
     * **Parameters:**
     *
     * * `codespace_name: &str` -- The name of the codespace.
     */
    pub async fn publish_for_authenticated_user(
        &self,
        codespace_name: &str,
        body: &crate::types::CodespacesPublishRequest,
    ) -> ClientResult<crate::Response<crate::types::CodespaceWithFullRepository>> {
        let url = self.client.url(
            &format!(
                "/user/codespaces/{}/publish",
                crate::progenitor_support::encode_path(&codespace_name.to_string()),
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
     * Start a codespace for the authenticated user.
     *
     * This function performs a `POST` to the `/user/codespaces/{codespace_name}/start` endpoint.
     *
     * Starts a user's codespace.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/codespaces#start-a-codespace-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `codespace_name: &str` -- The name of the codespace.
     */
    pub async fn start_for_authenticated_user(
        &self,
        codespace_name: &str,
    ) -> ClientResult<crate::Response<crate::types::Codespace>> {
        let url = self.client.url(
            &format!(
                "/user/codespaces/{}/start",
                crate::progenitor_support::encode_path(&codespace_name.to_string()),
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
     * Stop a codespace for the authenticated user.
     *
     * This function performs a `POST` to the `/user/codespaces/{codespace_name}/stop` endpoint.
     *
     * Stops a user's codespace.
     *
     * OAuth app tokens and personal access tokens (classic) need the `codespace` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/codespaces/codespaces#stop-a-codespace-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `codespace_name: &str` -- The name of the codespace.
     */
    pub async fn stop_for_authenticated_user(
        &self,
        codespace_name: &str,
    ) -> ClientResult<crate::Response<crate::types::Codespace>> {
        let url = self.client.url(
            &format!(
                "/user/codespaces/{}/stop",
                crate::progenitor_support::encode_path(&codespace_name.to_string()),
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
