use crate::Client;
use crate::ClientResult;

pub struct PrivateRegistries {
    pub client: Client,
}

impl PrivateRegistries {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        PrivateRegistries { client }
    }

    /**
     * List private registries for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/private-registries` endpoint.
     *
     *
     * Lists all private registry configurations available at the organization-level without revealing their encrypted
     * values.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/private-registries/organization-configurations#list-private-registries-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_org_private_registries(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::PrivateRegistriesListOrgResponse>> {
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
                "/orgs/{}/private-registries?{}",
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
     * Create a private registry for an organization.
     *
     * This function performs a `POST` to the `/orgs/{org}/private-registries` endpoint.
     *
     *
     * Creates a private registry configuration with an encrypted value for an organization. Encrypt your secret using [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/private-registries/organization-configurations#create-a-private-registry-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn create_org_private_registry(
        &self,
        org: &str,
        body: &crate::types::PrivateRegistriesCreateOrgRegistryRequest,
    ) -> ClientResult<
        crate::Response<crate::types::OrgPrivateRegistryConfigurationWithSelectedRepositories>,
    > {
        let url = self.client.url(
            &format!(
                "/orgs/{}/private-registries",
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
     * Get private registries public key for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/private-registries/public-key` endpoint.
     *
     *
     * Gets the org public key, which is needed to encrypt private registry secrets. You need to encrypt a secret before you can create or update secrets.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/private-registries/organization-configurations#get-private-registries-public-key-for-an-organization>
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
                "/orgs/{}/private-registries/public-key",
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
     * Get a private registry for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/private-registries/{secret_name}` endpoint.
     *
     *
     * Get the configuration of a single private registry defined for an organization, omitting its encrypted value.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/private-registries/organization-configurations#get-a-private-registry-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `secret_name: &str` -- The name of the secret.
     */
    pub async fn get_org_private_registry(
        &self,
        org: &str,
        secret_name: &str,
    ) -> ClientResult<crate::Response<crate::types::OrganizationPrivateRegistry>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/private-registries/{}",
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
     * Delete a private registry for an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/private-registries/{secret_name}` endpoint.
     *
     *
     * Delete a private registry configuration at the organization-level.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/private-registries/organization-configurations#delete-a-private-registry-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `secret_name: &str` -- The name of the secret.
     */
    pub async fn delete_org_private_registry(
        &self,
        org: &str,
        secret_name: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/private-registries/{}",
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
     * Update a private registry for an organization.
     *
     * This function performs a `PATCH` to the `/orgs/{org}/private-registries/{secret_name}` endpoint.
     *
     *
     * Updates a private registry configuration with an encrypted value for an organization. Encrypt your secret using [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/private-registries/organization-configurations#update-a-private-registry-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `secret_name: &str` -- The name of the secret.
     */
    pub async fn update_org_private_registry(
        &self,
        org: &str,
        secret_name: &str,
        body: &crate::types::PrivateRegistriesUpdateOrgRegistryRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/private-registries/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&secret_name.to_string()),
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
