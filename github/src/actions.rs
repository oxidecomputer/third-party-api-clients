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
     * Get GitHub Actions cache retention limit for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/actions/cache/retention-limit` endpoint.
     *
     * Gets GitHub Actions cache retention limit for an enterprise. All organizations and repositories under this
     * enterprise may not set a higher cache retention limit.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/cache#get-github-actions-cache-retention-limit-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     */
    pub async fn get_actions_cache_retention_limit_for_enterprise(
        &self,
        enterprise: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsCacheRetentionLimitEnterprise>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/cache/retention-limit",
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
     * Set GitHub Actions cache retention limit for an enterprise.
     *
     * This function performs a `PUT` to the `/enterprises/{enterprise}/actions/cache/retention-limit` endpoint.
     *
     * Sets GitHub Actions cache retention limit for an enterprise. All organizations and repositories under this
     * enterprise may not set a higher cache retention limit.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/cache#set-github-actions-cache-retention-limit-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     */
    pub async fn set_actions_cache_retention_limit_for_enterprise(
        &self,
        enterprise: &str,
        body: &crate::types::ActionsCacheRetentionLimitEnterprise,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/cache/retention-limit",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
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
     * Get GitHub Actions cache storage limit for an enterprise.
     *
     * This function performs a `GET` to the `/enterprises/{enterprise}/actions/cache/storage-limit` endpoint.
     *
     * Gets GitHub Actions cache storage limit for an enterprise. All organizations and repositories under this
     * enterprise may not set a higher cache storage limit.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/cache#get-github-actions-cache-storage-limit-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     */
    pub async fn get_actions_cache_storage_limit_for_enterprise(
        &self,
        enterprise: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsCacheStorageLimitEnterprise>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/cache/storage-limit",
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
     * Set GitHub Actions cache storage limit for an enterprise.
     *
     * This function performs a `PUT` to the `/enterprises/{enterprise}/actions/cache/storage-limit` endpoint.
     *
     * Sets GitHub Actions cache storage limit for an enterprise. All organizations and repositories under this
     * enterprise may not set a higher cache storage limit.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:enterprise` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/cache#set-github-actions-cache-storage-limit-for-an-enterprise>
     *
     * **Parameters:**
     *
     * * `enterprise: &str` -- The slug version of the enterprise name.
     */
    pub async fn set_actions_cache_storage_limit_for_enterprise(
        &self,
        enterprise: &str,
        body: &crate::types::ActionsCacheStorageLimitEnterprise,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/enterprises/{}/actions/cache/storage-limit",
                crate::progenitor_support::encode_path(&enterprise.to_string()),
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
     * Get GitHub Actions cache retention limit for an organization.
     *
     * This function performs a `GET` to the `/organizations/{org}/actions/cache/retention-limit` endpoint.
     *
     * Gets GitHub Actions cache retention limit for an organization. All repositories under this
     * organization may not set a higher cache retention limit.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:organization` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/cache#get-github-actions-cache-retention-limit-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_actions_cache_retention_limit_for_organization(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsCacheRetentionLimitOrganization>> {
        let url = self.client.url(
            &format!(
                "/organizations/{}/actions/cache/retention-limit",
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
     * Set GitHub Actions cache retention limit for an organization.
     *
     * This function performs a `PUT` to the `/organizations/{org}/actions/cache/retention-limit` endpoint.
     *
     * Sets GitHub Actions cache retention limit for an organization. All repositories under this
     * organization may not set a higher cache retention limit.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:organization` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/cache#set-github-actions-cache-retention-limit-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn set_actions_cache_retention_limit_for_organization(
        &self,
        org: &str,
        body: &crate::types::ActionsCacheRetentionLimitOrganization,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/organizations/{}/actions/cache/retention-limit",
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
     * Get GitHub Actions cache storage limit for an organization.
     *
     * This function performs a `GET` to the `/organizations/{org}/actions/cache/storage-limit` endpoint.
     *
     * Gets GitHub Actions cache storage limit for an organization. All repositories under this
     * organization may not set a higher cache storage limit.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:organization` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/cache#get-github-actions-cache-storage-limit-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_actions_cache_storage_limit_for_organization(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsCacheStorageLimitOrganization>> {
        let url = self.client.url(
            &format!(
                "/organizations/{}/actions/cache/storage-limit",
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
     * Set GitHub Actions cache storage limit for an organization.
     *
     * This function performs a `PUT` to the `/organizations/{org}/actions/cache/storage-limit` endpoint.
     *
     * Sets GitHub Actions cache storage limit for an organization. All organizations and repositories under this
     * organization may not set a higher cache storage limit.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:organization` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/cache#set-github-actions-cache-storage-limit-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn set_actions_cache_storage_limit_for_organization(
        &self,
        org: &str,
        body: &crate::types::ActionsCacheStorageLimitOrganization,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/organizations/{}/actions/cache/storage-limit",
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
     * Get GitHub Actions cache usage for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/cache/usage` endpoint.
     *
     * Gets the total GitHub Actions cache usage for an organization.
     * The data fetched using this API is refreshed approximately every 5 minutes, so values returned from this endpoint may take at least 5 minutes to get updated.
     *
     * OAuth tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/cache#get-github-actions-cache-usage-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_actions_cache_usage_for_org(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsCacheUsageOrgEnterprise>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/cache/usage",
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
     * List repositories with GitHub Actions cache usage for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/cache/usage-by-repository` endpoint.
     *
     * Lists repositories and their GitHub Actions cache usage for an organization.
     * The data fetched using this API is refreshed approximately every 5 minutes, so values returned from this endpoint may take at least 5 minutes to get updated.
     *
     * OAuth tokens and personal access tokens (classic) need the `read:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/cache#list-repositories-with-github-actions-cache-usage-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn get_actions_cache_usage_by_repo_for_org(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsGetCacheUsageByRepoOrgResponse>> {
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
                "/orgs/{}/actions/cache/usage-by-repository?{}",
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
     * List GitHub-hosted runners for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/hosted-runners` endpoint.
     *
     * Lists all GitHub-hosted runners configured in an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `manage_runner:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/hosted-runners#list-github-hosted-runners-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_hosted_runners_for_org(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsListHostedRunnersOrgResponse>> {
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
                "/orgs/{}/actions/hosted-runners?{}",
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
     * Create a GitHub-hosted runner for an organization.
     *
     * This function performs a `POST` to the `/orgs/{org}/actions/hosted-runners` endpoint.
     *
     * Creates a GitHub-hosted runner for an organization.
     * OAuth tokens and personal access tokens (classic) need the `manage_runners:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/hosted-runners#create-a-github-hosted-runner-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn create_hosted_runner_for_org(
        &self,
        org: &str,
        body: &crate::types::ActionsCreateHostedRunnerOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::ActionsHostedRunner>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/hosted-runners",
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
     * List custom images for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/hosted-runners/images/custom` endpoint.
     *
     * List custom images for an organization.
     *
     * OAuth tokens and personal access tokens (classic) need the `manage_runners:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/hosted-runners#list-custom-images-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn list_custom_images_for_org(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsListCustomImagesOrgResponse>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/hosted-runners/images/custom",
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
     * Get a custom image definition for GitHub Actions Hosted Runners.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/hosted-runners/images/custom/{image_definition_id}` endpoint.
     *
     * Get a custom image definition for GitHub Actions Hosted Runners.
     *
     * OAuth tokens and personal access tokens (classic) need the `manage_runners:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/hosted-runners#get-a-custom-image-definition-for-github-actions-hosted-runners>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `image_definition_id: i64` -- Image definition ID of custom image.
     */
    pub async fn get_custom_image_for_org(
        &self,
        org: &str,
        image_definition_id: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsHostedRunnerCustomImage>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/hosted-runners/images/custom/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&image_definition_id.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Delete a custom image from the organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/actions/hosted-runners/images/custom/{image_definition_id}` endpoint.
     *
     * Delete a custom image from the organization.
     *
     * OAuth tokens and personal access tokens (classic) need the `manage_runners:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/hosted-runners#delete-a-custom-image-from-the-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `image_definition_id: i64` -- Image definition ID of custom image.
     */
    pub async fn delete_custom_image_from_org(
        &self,
        org: &str,
        image_definition_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/hosted-runners/images/custom/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&image_definition_id.to_string()),
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
     * List image versions of a custom image for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/hosted-runners/images/custom/{image_definition_id}/versions` endpoint.
     *
     * List image versions of a custom image for an organization.
     *
     * OAuth tokens and personal access tokens (classic) need the `manage_runners:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/hosted-runners#list-image-versions-of-a-custom-image-for-an-organization>
     *
     * **Parameters:**
     *
     * * `image_definition_id: i64` -- Image definition ID of custom image.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn list_custom_image_versions_for_org(
        &self,
        image_definition_id: i64,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsListCustomImageVersionsOrgResponse>>
    {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/hosted-runners/images/custom/{}/versions",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&image_definition_id.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get an image version of a custom image for GitHub Actions Hosted Runners.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/hosted-runners/images/custom/{image_definition_id}/versions/{version}` endpoint.
     *
     * Get an image version of a custom image for GitHub Actions Hosted Runners.
     *
     * OAuth tokens and personal access tokens (classic) need the `manage_runners:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/hosted-runners#get-an-image-version-of-a-custom-image-for-github-actions-hosted-runners>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `image_definition_id: i64` -- Image definition ID of custom image.
     * * `version: &str` -- Version of a custom image.
     */
    pub async fn get_custom_image_version_for_org(
        &self,
        org: &str,
        image_definition_id: i64,
        version: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsHostedRunnerCustomImageVersion>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/hosted-runners/images/custom/{}/versions/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&image_definition_id.to_string()),
                crate::progenitor_support::encode_path(&version.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Delete an image version of custom image from the organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/actions/hosted-runners/images/custom/{image_definition_id}/versions/{version}` endpoint.
     *
     * Delete an image version of custom image from the organization.
     *
     * OAuth tokens and personal access tokens (classic) need the `manage_runners:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/hosted-runners#delete-an-image-version-of-custom-image-from-the-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `image_definition_id: i64` -- Image definition ID of custom image.
     * * `version: &str` -- Version of a custom image.
     */
    pub async fn delete_custom_image_version_from_org(
        &self,
        org: &str,
        image_definition_id: i64,
        version: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/hosted-runners/images/custom/{}/versions/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&image_definition_id.to_string()),
                crate::progenitor_support::encode_path(&version.to_string()),
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
     * Get GitHub-owned images for GitHub-hosted runners in an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/hosted-runners/images/github-owned` endpoint.
     *
     * Get the list of GitHub-owned images available for GitHub-hosted runners for an organization.
     *
     * FROM: <https://docs.github.com/rest/actions/hosted-runners#get-github-owned-images-for-github-hosted-runners-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_hosted_runners_github_owned_images_for_org(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsGetHostedRunnersPartnerImagesOrgResponse>>
    {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/hosted-runners/images/github-owned",
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
     * Get partner images for GitHub-hosted runners in an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/hosted-runners/images/partner` endpoint.
     *
     * Get the list of partner images available for GitHub-hosted runners for an organization.
     *
     * FROM: <https://docs.github.com/rest/actions/hosted-runners#get-partner-images-for-github-hosted-runners-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_hosted_runners_partner_images_for_org(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsGetHostedRunnersPartnerImagesOrgResponse>>
    {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/hosted-runners/images/partner",
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
     * Get limits on GitHub-hosted runners for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/hosted-runners/limits` endpoint.
     *
     * Get the GitHub-hosted runners limits for an organization.
     *
     * FROM: <https://docs.github.com/rest/actions/hosted-runners#get-limits-on-github-hosted-runners-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_hosted_runners_limits_for_org(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsHostedRunnerLimits>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/hosted-runners/limits",
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
     * Get GitHub-hosted runners machine specs for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/hosted-runners/machine-sizes` endpoint.
     *
     * Get the list of machine specs available for GitHub-hosted runners for an organization.
     *
     * FROM: <https://docs.github.com/rest/actions/hosted-runners#get-github-hosted-runners-machine-specs-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_hosted_runners_machine_specs_for_org(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsGetHostedRunnersMachineSpecsOrgResponse>>
    {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/hosted-runners/machine-sizes",
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
     * Get platforms for GitHub-hosted runners in an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/hosted-runners/platforms` endpoint.
     *
     * Get the list of platforms available for GitHub-hosted runners for an organization.
     *
     * FROM: <https://docs.github.com/rest/actions/hosted-runners#get-platforms-for-github-hosted-runners-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_hosted_runners_platforms_for_org(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsGetHostedRunnersPlatformsOrgResponse>>
    {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/hosted-runners/platforms",
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
     * Get a GitHub-hosted runner for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/hosted-runners/{hosted_runner_id}` endpoint.
     *
     * Gets a GitHub-hosted runner configured in an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `manage_runners:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/hosted-runners#get-a-github-hosted-runner-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `hosted_runner_id: i64` -- Unique identifier of the GitHub-hosted runner.
     */
    pub async fn get_hosted_runner_for_org(
        &self,
        org: &str,
        hosted_runner_id: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsHostedRunner>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/hosted-runners/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&hosted_runner_id.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Delete a GitHub-hosted runner for an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/actions/hosted-runners/{hosted_runner_id}` endpoint.
     *
     * Deletes a GitHub-hosted runner for an organization.
     *
     * FROM: <https://docs.github.com/rest/actions/hosted-runners#delete-a-github-hosted-runner-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `hosted_runner_id: i64` -- Unique identifier of the GitHub-hosted runner.
     */
    pub async fn delete_hosted_runner_for_org(
        &self,
        org: &str,
        hosted_runner_id: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsHostedRunner>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/hosted-runners/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&hosted_runner_id.to_string()),
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
     * Update a GitHub-hosted runner for an organization.
     *
     * This function performs a `PATCH` to the `/orgs/{org}/actions/hosted-runners/{hosted_runner_id}` endpoint.
     *
     * Updates a GitHub-hosted runner for an organization.
     * OAuth app tokens and personal access tokens (classic) need the `manage_runners:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/hosted-runners#update-a-github-hosted-runner-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `hosted_runner_id: i64` -- Unique identifier of the GitHub-hosted runner.
     */
    pub async fn update_hosted_runner_for_org(
        &self,
        org: &str,
        hosted_runner_id: i64,
        body: &crate::types::ActionsUpdateHostedRunnerOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::ActionsHostedRunner>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/hosted-runners/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&hosted_runner_id.to_string()),
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
     * Get GitHub Actions permissions for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/permissions` endpoint.
     *
     * Gets the GitHub Actions permissions policy for repositories and allowed actions and reusable workflows in an organization.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#get-github-actions-permissions-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_github_actions_permissions_organization(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsOrganizationPermissions>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions",
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
     * Set GitHub Actions permissions for an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/permissions` endpoint.
     *
     * Sets the GitHub Actions permissions policy for repositories and allowed actions and reusable workflows in an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#set-github-actions-permissions-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn set_github_actions_permissions_organization(
        &self,
        org: &str,
        body: &crate::types::ActionsSetGithubPermissionsOrganizationRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions",
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
     * Get artifact and log retention settings for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/permissions/artifact-and-log-retention` endpoint.
     *
     * Gets artifact and log retention settings for an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope or the "Actions policies" fine-grained permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#get-artifact-and-log-retention-settings-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_artifact_and_log_retention_settings_organization(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsArtifactLogRetentionResponse>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/artifact-and-log-retention",
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
     * Set artifact and log retention settings for an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/permissions/artifact-and-log-retention` endpoint.
     *
     * Sets artifact and log retention settings for an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope or the "Actions policies" fine-grained permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#set-artifact-and-log-retention-settings-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn set_artifact_and_log_retention_settings_organization(
        &self,
        org: &str,
        body: &crate::types::ActionsArtifactLogRetention,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/artifact-and-log-retention",
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
     * Get fork PR contributor approval permissions for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/permissions/fork-pr-contributor-approval` endpoint.
     *
     * Gets the fork PR contributor approval policy for an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope or the "Actions policies" fine-grained permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#get-fork-pr-contributor-approval-permissions-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_fork_pr_contributor_approval_permissions_organization(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsForkPrContributorApproval>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/fork-pr-contributor-approval",
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
     * Set fork PR contributor approval permissions for an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/permissions/fork-pr-contributor-approval` endpoint.
     *
     * Sets the fork PR contributor approval policy for an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#set-fork-pr-contributor-approval-permissions-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn set_fork_pr_contributor_approval_permissions_organization(
        &self,
        org: &str,
        body: &crate::types::ActionsForkPrContributorApproval,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/fork-pr-contributor-approval",
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
     * Get private repo fork PR workflow settings for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/permissions/fork-pr-workflows-private-repos` endpoint.
     *
     * Gets the settings for whether workflows from fork pull requests can run on private repositories in an organization.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#get-private-repo-fork-pr-workflow-settings-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_private_repo_fork_pr_workflows_settings_organization(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsForkPrWorkflowsPrivateRepos>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/fork-pr-workflows-private-repos",
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
     * Set private repo fork PR workflow settings for an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/permissions/fork-pr-workflows-private-repos` endpoint.
     *
     * Sets the settings for whether workflows from fork pull requests can run on private repositories in an organization.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#set-private-repo-fork-pr-workflow-settings-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn set_private_repo_fork_pr_workflows_settings_organization(
        &self,
        org: &str,
        body: &crate::types::ActionsForkPrWorkflowsPrivateReposRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/fork-pr-workflows-private-repos",
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
     * List selected repositories enabled for GitHub Actions in an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/permissions/repositories` endpoint.
     *
     * Lists the selected repositories that are enabled for GitHub Actions in an organization. To use this endpoint, the organization permission policy for `enabled_repositories` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#list-selected-repositories-enabled-for-github-actions-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
     * Set selected repositories enabled for GitHub Actions in an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/permissions/repositories` endpoint.
     *
     * Replaces the list of selected repositories that are enabled for GitHub Actions in an organization. To use this endpoint, the organization permission policy for `enabled_repositories` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
     *
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#set-selected-repositories-enabled-for-github-actions-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn set_selected_repositories_enabled_github_actions_organization(
        &self,
        org: &str,
        body: &crate::types::CodespacesSetRepositoriesSecretRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/repositories",
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
     * Enable a selected repository for GitHub Actions in an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/permissions/repositories/{repository_id}` endpoint.
     *
     * Adds a repository to the list of selected repositories that are enabled for GitHub Actions in an organization. To use this endpoint, the organization permission policy for `enabled_repositories` must be must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#enable-a-selected-repository-for-github-actions-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `repository_id: i64` -- The unique identifier of the repository.
     */
    pub async fn enable_selected_repository_github_actions_organization(
        &self,
        org: &str,
        repository_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/repositories/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#disable-a-selected-repository-for-github-actions-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `repository_id: i64` -- The unique identifier of the repository.
     */
    pub async fn disable_selected_repository_github_actions_organization(
        &self,
        org: &str,
        repository_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/repositories/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Get allowed actions and reusable workflows for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/permissions/selected-actions` endpoint.
     *
     * Gets the selected actions and reusable workflows that are allowed in an organization. To use this endpoint, the organization permission policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#get-allowed-actions-and-reusable-workflows-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_allowed_actions_organization(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::SelectedActions>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/selected-actions",
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
     * Set allowed actions and reusable workflows for an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/permissions/selected-actions` endpoint.
     *
     * Sets the actions and reusable workflows that are allowed in an organization. To use this endpoint, the organization permission policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for an organization](#set-github-actions-permissions-for-an-organization)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#set-allowed-actions-and-reusable-workflows-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn set_allowed_actions_organization(
        &self,
        org: &str,
        body: &crate::types::SelectedActions,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/selected-actions",
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
     * Get self-hosted runners settings for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/permissions/self-hosted-runners` endpoint.
     *
     * Gets the settings for self-hosted runners for an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope or the "Actions policies" fine-grained permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#get-self-hosted-runners-settings-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_self_hosted_runners_permissions_organization(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::SelfHostedRunnersSettings>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/self-hosted-runners",
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
     * Set self-hosted runners settings for an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/permissions/self-hosted-runners` endpoint.
     *
     * Sets the settings for self-hosted runners for an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope or the "Actions policies" fine-grained permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#set-self-hosted-runners-settings-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn set_self_hosted_runners_permissions_organization(
        &self,
        org: &str,
        body: &crate::types::ActionsSetSelfHostedRunnersPermissionsOrganizationRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/self-hosted-runners",
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
     * List repositories allowed to use self-hosted runners in an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/permissions/self-hosted-runners/repositories` endpoint.
     *
     * Lists repositories that are allowed to use self-hosted runners in an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope or the "Actions policies" fine-grained permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#list-repositories-allowed-to-use-self-hosted-runners-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_selected_repositories_self_hosted_runners_organization(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<
        crate::Response<
            crate::types::ActionsListSelectedRepositoriesSelfHostedRunnersOrganizationResponse,
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
                "/orgs/{}/actions/permissions/self-hosted-runners/repositories?{}",
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
     * Set repositories allowed to use self-hosted runners in an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/permissions/self-hosted-runners/repositories` endpoint.
     *
     * Sets repositories that are allowed to use self-hosted runners in an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope or the "Actions policies" fine-grained permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#set-repositories-allowed-to-use-self-hosted-runners-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn set_selected_repositories_self_hosted_runners_organization(
        &self,
        org: &str,
        body: &crate::types::CodespacesSetRepositoriesSecretRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/self-hosted-runners/repositories",
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
     * Add a repository to the list of repositories allowed to use self-hosted runners in an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/permissions/self-hosted-runners/repositories/{repository_id}` endpoint.
     *
     * Adds a repository to the list of repositories that are allowed to use self-hosted runners in an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope or the "Actions policies" fine-grained permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#add-a-repository-to-the-list-of-repositories-allowed-to-use-self-hosted-runners-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `repository_id: i64` -- The unique identifier of the repository.
     */
    pub async fn enable_selected_repository_self_hosted_runners_organization(
        &self,
        org: &str,
        repository_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/self-hosted-runners/repositories/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Remove a repository from the list of repositories allowed to use self-hosted runners in an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/actions/permissions/self-hosted-runners/repositories/{repository_id}` endpoint.
     *
     * Removes a repository from the list of repositories that are allowed to use self-hosted runners in an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope or the "Actions policies" fine-grained permission to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#remove-a-repository-from-the-list-of-repositories-allowed-to-use-self-hosted-runners-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `repository_id: i64` -- The unique identifier of the repository.
     */
    pub async fn disable_selected_repository_self_hosted_runners_organization(
        &self,
        org: &str,
        repository_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/self-hosted-runners/repositories/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Get default workflow permissions for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/permissions/workflow` endpoint.
     *
     * Gets the default workflow permissions granted to the `GITHUB_TOKEN` when running workflows in an organization,
     * as well as whether GitHub Actions can submit approving pull request reviews. For more information, see
     * "[Setting the permissions of the GITHUB_TOKEN for your organization](https://docs.github.com/organizations/managing-organization-settings/disabling-or-limiting-github-actions-for-your-organization#setting-the-permissions-of-the-github_token-for-your-organization)."
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#get-default-workflow-permissions-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_github_actions_default_workflow_permissions_organization(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsGetDefaultWorkflowPermissions>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/workflow",
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
     * Set default workflow permissions for an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/permissions/workflow` endpoint.
     *
     * Sets the default workflow permissions granted to the `GITHUB_TOKEN` when running workflows in an organization, and sets if GitHub Actions
     * can submit approving pull request reviews. For more information, see
     * "[Setting the permissions of the GITHUB_TOKEN for your organization](https://docs.github.com/organizations/managing-organization-settings/disabling-or-limiting-github-actions-for-your-organization#setting-the-permissions-of-the-github_token-for-your-organization)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#set-default-workflow-permissions-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn set_github_actions_default_workflow_permissions_organization(
        &self,
        org: &str,
        body: &crate::types::ActionsSetDefaultWorkflowPermissions,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/permissions/workflow",
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
     * List self-hosted runner groups for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/runner-groups` endpoint.
     *
     * Lists all self-hosted runner groups configured in an organization and inherited from an enterprise.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runner-groups#list-self-hosted-runner-groups-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `visible_to_repository: &str` -- Only return runner groups that are allowed to be used by this repository.
     */
    pub async fn list_self_hosted_runner_groups_for_org(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
        visible_to_repository: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsListSelfHostedRunnerGroupsOrgResponse>>
    {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !visible_to_repository.is_empty() {
            query_args.push((
                "visible_to_repository".to_string(),
                visible_to_repository.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runner-groups?{}",
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
     * Create a self-hosted runner group for an organization.
     *
     * This function performs a `POST` to the `/orgs/{org}/actions/runner-groups` endpoint.
     *
     * Creates a new self-hosted runner group for an organization.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runner-groups#create-a-self-hosted-runner-group-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn create_self_hosted_runner_group_for_org(
        &self,
        org: &str,
        body: &crate::types::ActionsCreateSelfHostedRunnerGroupOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::RunnerGroupsOrg>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runner-groups",
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
     * Get a self-hosted runner group for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/runner-groups/{runner_group_id}` endpoint.
     *
     * Gets a specific self-hosted runner group for an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runner-groups#get-a-self-hosted-runner-group-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Deletes a self-hosted runner group for an organization.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runner-groups#delete-a-self-hosted-runner-group-from-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Updates the `name` and `visibility` of a self-hosted runner group in an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runner-groups#update-a-self-hosted-runner-group-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List GitHub-hosted runners in a group for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/runner-groups/{runner_group_id}/hosted-runners` endpoint.
     *
     * Lists the GitHub-hosted runners in an organization group.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runner-groups#list-github-hosted-runners-in-a-group-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_github_hosted_runners_in_group_for_org(
        &self,
        org: &str,
        runner_group_id: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsListGithubHostedRunnersInGroupOrgResponse>>
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
                "/orgs/{}/actions/runner-groups/{}/hosted-runners?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List repository access to a self-hosted runner group in an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/runner-groups/{runner_group_id}/repositories` endpoint.
     *
     * Lists the repositories with access to a self-hosted runner group configured in an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runner-groups#list-repository-access-to-a-self-hosted-runner-group-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Replaces the list of repositories that have access to a self-hosted runner group configured in an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runner-groups#set-repository-access-for-a-self-hosted-runner-group-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     */
    pub async fn set_repo_access_to_self_hosted_runner_group_in_org(
        &self,
        org: &str,
        runner_group_id: i64,
        body: &crate::types::CodespacesSetRepositoriesSecretRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runner-groups/{}/repositories",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Adds a repository to the list of repositories that can access a self-hosted runner group. The runner group must have `visibility` set to `selected`. For more information, see "[Create a self-hosted runner group for an organization](#create-a-self-hosted-runner-group-for-an-organization)."
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runner-groups#add-repository-access-to-a-self-hosted-runner-group-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     * * `repository_id: i64` -- The unique identifier of the repository.
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
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Removes a repository from the list of selected repositories that can access a self-hosted runner group. The runner group must have `visibility` set to `selected`. For more information, see "[Create a self-hosted runner group for an organization](#create-a-self-hosted-runner-group-for-an-organization)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runner-groups#remove-repository-access-to-a-self-hosted-runner-group-in-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     * * `repository_id: i64` -- The unique identifier of the repository.
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
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Lists self-hosted runners that are in a specific organization group.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runner-groups#list-self-hosted-runners-in-a-group-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `runner_group_id: i64` -- Unique identifier of the self-hosted runner group.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Replaces the list of self-hosted runners that are part of an organization runner group.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runner-groups#set-self-hosted-runners-in-a-group-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Adds a self-hosted runner to a runner group configured in an organization.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runner-groups#add-a-self-hosted-runner-to-a-group-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Removes a self-hosted runner from a group configured in an organization. The runner is then returned to the default group.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runner-groups#remove-a-self-hosted-runner-from-a-group-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Authenticated users must have admin access to the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#list-self-hosted-runners-for-an-organization>
     *
     * **Parameters:**
     *
     * * `name: &str` -- The name of a self-hosted runner.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_self_hosted_runners_for_org(
        &self,
        name: &str,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsListSelfHostedRunnersOrgResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !name.is_empty() {
            query_args.push(("name".to_string(), name.to_string()));
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
                "/orgs/{}/actions/runners?{}",
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
     * List runner applications for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/runners/downloads` endpoint.
     *
     * Lists binaries for the runner application that you can download and run.
     *
     * Authenticated users must have admin access to the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.  If the repository is private, the `repo` scope is also required.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#list-runner-applications-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn list_runner_applications_for_org(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::RunnerApplication>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runners/downloads",
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
     * List runner applications for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/runners/downloads` endpoint.
     *
     * As opposed to `list_runner_applications_for_org`, this function returns all the pages of the request at once.
     *
     * Lists binaries for the runner application that you can download and run.
     *
     * Authenticated users must have admin access to the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.  If the repository is private, the `repo` scope is also required.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#list-runner-applications-for-an-organization>
     */
    pub async fn list_all_runner_applications_for_org(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::RunnerApplication>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runners/downloads",
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
     * Create configuration for a just-in-time runner for an organization.
     *
     * This function performs a `POST` to the `/orgs/{org}/actions/runners/generate-jitconfig` endpoint.
     *
     * Generates a configuration that can be passed to the runner application at startup.
     *
     * The authenticated user must have admin access to the organization.
     *
     * OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#create-configuration-for-a-just-in-time-runner-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn generate_runner_jitconfig_for_org(
        &self,
        org: &str,
        body: &crate::types::ActionsGenerateRunnerJitconfigOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::ActionsRunnerJitconfigResponse>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runners/generate-jitconfig",
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
     * Create a registration token for an organization.
     *
     * This function performs a `POST` to the `/orgs/{org}/actions/runners/registration-token` endpoint.
     *
     * Returns a token that you can pass to the `config` script. The token expires after one hour.
     *
     * For example, you can replace `TOKEN` in the following example with the registration token provided by this endpoint to configure your self-hosted runner:
     *
     * ```
     * ./config.sh --url https://github.com/octo-org --token TOKEN
     * ```
     *
     * Authenticated users must have admin access to the organization to use this endpoint.
     *
     * OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#create-a-registration-token-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn create_registration_token_for_org(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::AuthenticationToken>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runners/registration-token",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * For example, you can replace `TOKEN` in the following example with the registration token provided by this endpoint to remove your self-hosted runner from an organization:
     *
     * ```
     * ./config.sh remove --token TOKEN
     * ```
     *
     * Authenticated users must have admin access to the organization to use this endpoint.
     *
     * OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#create-a-remove-token-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn create_remove_token_for_org(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::AuthenticationToken>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runners/remove-token",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Authenticated users must have admin access to the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#get-a-self-hosted-runner-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Authenticated users must have admin access to the organization to use this endpoint.
     *
     * OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#delete-a-self-hosted-runner-from-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * List labels for a self-hosted runner for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/runners/{runner_id}/labels` endpoint.
     *
     * Lists all labels for a self-hosted runner configured in an organization.
     *
     * Authenticated users must have admin access to the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#list-labels-for-a-self-hosted-runner-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `runner_id: i64` -- Unique identifier of the self-hosted runner.
     */
    pub async fn list_labels_for_self_hosted_runner_for_org(
        &self,
        org: &str,
        runner_id: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsRunnerLabelsResponse>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runners/{}/labels",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Set custom labels for a self-hosted runner for an organization.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/runners/{runner_id}/labels` endpoint.
     *
     * Remove all previous custom labels and set the new custom labels for a specific
     * self-hosted runner configured in an organization.
     *
     * Authenticated users must have admin access to the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#set-custom-labels-for-a-self-hosted-runner-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `runner_id: i64` -- Unique identifier of the self-hosted runner.
     */
    pub async fn set_custom_labels_for_self_hosted_runner_for_org(
        &self,
        org: &str,
        runner_id: i64,
        body: &crate::types::IssuesAddLabelsRequest,
    ) -> ClientResult<crate::Response<crate::types::ActionsRunnerLabelsResponse>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runners/{}/labels",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&runner_id.to_string()),
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
     * Add custom labels to a self-hosted runner for an organization.
     *
     * This function performs a `POST` to the `/orgs/{org}/actions/runners/{runner_id}/labels` endpoint.
     *
     * Adds custom labels to a self-hosted runner configured in an organization.
     *
     * Authenticated users must have admin access to the organization to use this endpoint.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#add-custom-labels-to-a-self-hosted-runner-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `runner_id: i64` -- Unique identifier of the self-hosted runner.
     */
    pub async fn add_custom_labels_to_self_hosted_runner_for_org(
        &self,
        org: &str,
        runner_id: i64,
        body: &crate::types::IssuesAddLabelsRequest,
    ) -> ClientResult<crate::Response<crate::types::ActionsRunnerLabelsResponse>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runners/{}/labels",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&runner_id.to_string()),
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
     * Remove all custom labels from a self-hosted runner for an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/actions/runners/{runner_id}/labels` endpoint.
     *
     * Remove all custom labels from a self-hosted runner configured in an
     * organization. Returns the remaining read-only labels from the runner.
     *
     * Authenticated users must have admin access to the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#remove-all-custom-labels-from-a-self-hosted-runner-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `runner_id: i64` -- Unique identifier of the self-hosted runner.
     */
    pub async fn remove_all_custom_labels_from_self_hosted_runner_for_org(
        &self,
        org: &str,
        runner_id: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsRunnerLabelsResponse>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runners/{}/labels",
                crate::progenitor_support::encode_path(&org.to_string()),
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
     * Remove a custom label from a self-hosted runner for an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/actions/runners/{runner_id}/labels/{name}` endpoint.
     *
     * Remove a custom label from a self-hosted runner configured
     * in an organization. Returns the remaining labels from the runner.
     *
     * This endpoint returns a `404 Not Found` status if the custom label is not
     * present on the runner.
     *
     * Authenticated users must have admin access to the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#remove-a-custom-label-from-a-self-hosted-runner-for-an-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `runner_id: i64` -- Unique identifier of the self-hosted runner.
     * * `name: &str` -- The name of a self-hosted runner's custom label.
     */
    pub async fn remove_custom_label_from_self_hosted_runner_for_org(
        &self,
        org: &str,
        runner_id: i64,
        name: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsRunnerLabelsResponse>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/runners/{}/labels/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&runner_id.to_string()),
                crate::progenitor_support::encode_path(&name.to_string()),
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
     * Lists all secrets available in an organization without revealing their
     * encrypted values.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read secrets.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
     *
     * FROM: <https://docs.github.com/rest/actions/secrets#list-organization-secrets>
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
     * This function performs a `GET` to the `/orgs/{org}/actions/secrets/public-key` endpoint.
     *
     * Gets your public key, which you need to encrypt secrets. You need to
     * encrypt a secret before you can create or update secrets.
     *
     * The authenticated user must have collaborator access to a repository to create, update, or read secrets.
     *
     * OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/secrets#get-an-organization-public-key>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_org_public_key(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsPublicKey>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/secrets/public-key",
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
     * This function performs a `GET` to the `/orgs/{org}/actions/secrets/{secret_name}` endpoint.
     *
     * Gets a single organization secret without revealing its encrypted value.
     *
     * The authenticated user must have collaborator access to a repository to create, update, or read secrets
     *
     * OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/secrets#get-an-organization-secret>
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
    ) -> ClientResult<crate::Response<crate::types::OrganizationActionsSecret>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/secrets/{}",
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
     * This function performs a `PUT` to the `/orgs/{org}/actions/secrets/{secret_name}` endpoint.
     *
     * Creates or updates an organization secret with an encrypted value. Encrypt your secret using
     * [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read secrets.
     *
     * OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/secrets#create-or-update-an-organization-secret>
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
        body: &crate::types::ActionsCreateUpdateOrgSecretRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/secrets/{}",
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
     * This function performs a `DELETE` to the `/orgs/{org}/actions/secrets/{secret_name}` endpoint.
     *
     * Deletes a secret in an organization using the secret name.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read secrets.
     *
     * OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/secrets#delete-an-organization-secret>
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
                "/orgs/{}/actions/secrets/{}",
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
     * This function performs a `GET` to the `/orgs/{org}/actions/secrets/{secret_name}/repositories` endpoint.
     *
     * Lists all repositories that have been selected when the `visibility`
     * for repository access to a secret is set to `selected`.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read secrets.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
     *
     * FROM: <https://docs.github.com/rest/actions/secrets#list-selected-repositories-for-an-organization-secret>
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
                "/orgs/{}/actions/secrets/{}/repositories?{}",
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
     * This function performs a `PUT` to the `/orgs/{org}/actions/secrets/{secret_name}/repositories` endpoint.
     *
     * Replaces all repositories for an organization secret when the `visibility`
     * for repository access is set to `selected`. The visibility is set when you [Create
     * or update an organization secret](https://docs.github.com/rest/actions/secrets#create-or-update-an-organization-secret).
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read secrets.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
     *
     * FROM: <https://docs.github.com/rest/actions/secrets#set-selected-repositories-for-an-organization-secret>
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
                "/orgs/{}/actions/secrets/{}/repositories",
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
     * This function performs a `PUT` to the `/orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id}` endpoint.
     *
     * Adds a repository to an organization secret when the `visibility` for
     * repository access is set to `selected`. For more information about setting the visibility, see [Create or
     * update an organization secret](https://docs.github.com/rest/actions/secrets#create-or-update-an-organization-secret).
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read secrets.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/secrets#add-selected-repository-to-an-organization-secret>
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
                "/orgs/{}/actions/secrets/{}/repositories/{}",
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
     * This function performs a `DELETE` to the `/orgs/{org}/actions/secrets/{secret_name}/repositories/{repository_id}` endpoint.
     *
     * Removes a repository from an organization secret when the `visibility`
     * for repository access is set to `selected`. The visibility is set when you [Create
     * or update an organization secret](https://docs.github.com/rest/actions/secrets#create-or-update-an-organization-secret).
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read secrets.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
     *
     * FROM: <https://docs.github.com/rest/actions/secrets#remove-selected-repository-from-an-organization-secret>
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
                "/orgs/{}/actions/secrets/{}/repositories/{}",
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
     * List organization variables.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/variables` endpoint.
     *
     * Lists all organization variables.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read variables.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
     *
     * FROM: <https://docs.github.com/rest/actions/variables#list-organization-variables>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 30). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_org_variables(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsListOrgVariablesResponse>> {
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
                "/orgs/{}/actions/variables?{}",
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
     * Create an organization variable.
     *
     * This function performs a `POST` to the `/orgs/{org}/actions/variables` endpoint.
     *
     * Creates an organization variable that you can reference in a GitHub Actions workflow.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read variables.
     *
     * OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/variables#create-an-organization-variable>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn create_org_variable(
        &self,
        org: &str,
        body: &crate::types::ActionsCreateOrgVariableRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/variables",
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
     * Get an organization variable.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/variables/{name}` endpoint.
     *
     * Gets a specific variable in an organization.
     *
     * The authenticated user must have collaborator access to a repository to create, update, or read variables.
     *
     * OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/variables#get-an-organization-variable>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `name: &str` -- The name of the variable.
     */
    pub async fn get_org_variable(
        &self,
        org: &str,
        name: &str,
    ) -> ClientResult<crate::Response<crate::types::OrganizationActionsVariable>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/variables/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&name.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Delete an organization variable.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/actions/variables/{name}` endpoint.
     *
     * Deletes an organization variable using the variable name.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read variables.
     *
     * OAuth tokens and personal access tokens (classic) need the`admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/variables#delete-an-organization-variable>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `name: &str` -- The name of the variable.
     */
    pub async fn delete_org_variable(
        &self,
        org: &str,
        name: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/variables/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&name.to_string()),
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
     * Update an organization variable.
     *
     * This function performs a `PATCH` to the `/orgs/{org}/actions/variables/{name}` endpoint.
     *
     * Updates an organization variable that you can reference in a GitHub Actions workflow.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read variables.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
     *
     * FROM: <https://docs.github.com/rest/actions/variables#update-an-organization-variable>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `name: &str` -- The name of the variable.
     */
    pub async fn update_org_variable(
        &self,
        org: &str,
        name: &str,
        body: &crate::types::ActionsUpdateOrgVariableRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/variables/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&name.to_string()),
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
     * List selected repositories for an organization variable.
     *
     * This function performs a `GET` to the `/orgs/{org}/actions/variables/{name}/repositories` endpoint.
     *
     * Lists all repositories that can access an organization variable
     * that is available to selected repositories.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read variables.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
     *
     * FROM: <https://docs.github.com/rest/actions/variables#list-selected-repositories-for-an-organization-variable>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `name: &str` -- The name of the variable.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_selected_repos_for_org_variable(
        &self,
        org: &str,
        name: &str,
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
                "/orgs/{}/actions/variables/{}/repositories?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&name.to_string()),
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
     * Set selected repositories for an organization variable.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/variables/{name}/repositories` endpoint.
     *
     * Replaces all repositories for an organization variable that is available
     * to selected repositories. Organization variables that are available to selected
     * repositories have their `visibility` field set to `selected`.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read variables.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
     *
     * FROM: <https://docs.github.com/rest/actions/variables#set-selected-repositories-for-an-organization-variable>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `name: &str` -- The name of the variable.
     */
    pub async fn set_selected_repos_for_org_variable(
        &self,
        org: &str,
        name: &str,
        body: &crate::types::CodespacesSetRepositoriesSecretRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/variables/{}/repositories",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&name.to_string()),
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
     * Add selected repository to an organization variable.
     *
     * This function performs a `PUT` to the `/orgs/{org}/actions/variables/{name}/repositories/{repository_id}` endpoint.
     *
     * Adds a repository to an organization variable that is available to selected repositories.
     * Organization variables that are available to selected repositories have their `visibility` field set to `selected`.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read secrets.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/variables#add-selected-repository-to-an-organization-variable>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `name: &str` -- The name of the variable.
     * * `repository_id: i64`
     */
    pub async fn add_selected_repo_to_org_variable(
        &self,
        org: &str,
        name: &str,
        repository_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/variables/{}/repositories/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&name.to_string()),
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
     * Remove selected repository from an organization variable.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/actions/variables/{name}/repositories/{repository_id}` endpoint.
     *
     * Removes a repository from an organization variable that is
     * available to selected repositories. Organization variables that are available to
     * selected repositories have their `visibility` field set to `selected`.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read variables.
     *
     * OAuth app tokens and personal access tokens (classic) need the `admin:org` scope to use this endpoint. If the repository is private, the `repo` scope is also required.
     *
     * FROM: <https://docs.github.com/rest/actions/variables#remove-selected-repository-from-an-organization-variable>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `name: &str` -- The name of the variable.
     * * `repository_id: i64`
     */
    pub async fn remove_selected_repo_from_org_variable(
        &self,
        org: &str,
        name: &str,
        repository_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/actions/variables/{}/repositories/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&name.to_string()),
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
     * Lists all artifacts for a repository.
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
     *
     * FROM: <https://docs.github.com/rest/actions/artifacts#list-artifacts-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `name: &str` -- The name field of an artifact. When specified, only artifacts with this name will be returned.
     */
    pub async fn list_artifacts_for_repo(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
        name: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsListArtifactsRepoResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !name.is_empty() {
            query_args.push(("name".to_string(), name.to_string()));
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
                "/repos/{}/{}/actions/artifacts?{}",
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
     * Get an artifact.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/artifacts/{artifact_id}` endpoint.
     *
     * Gets a specific artifact for a workflow run.
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/artifacts#get-an-artifact>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `artifact_id: i64` -- The unique identifier of the artifact.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Deletes an artifact for a workflow run.
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/artifacts#delete-an-artifact>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `artifact_id: i64` -- The unique identifier of the artifact.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * the response header to find the URL for the download. The `:archive_format` must be `zip`.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/artifacts#download-an-artifact>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `artifact_id: i64` -- The unique identifier of the artifact.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&artifact_id.to_string()),
                crate::progenitor_support::encode_path(&archive_format.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get GitHub Actions cache retention limit for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/cache/retention-limit` endpoint.
     *
     * Gets GitHub Actions cache retention limit for a repository. This determines how long caches will be retained for, if
     * not manually removed or evicted due to size constraints.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:repository` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/cache#get-github-actions-cache-retention-limit-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_actions_cache_retention_limit_for_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsCacheRetentionLimitRepository>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/cache/retention-limit",
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
     * Set GitHub Actions cache retention limit for a repository.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/actions/cache/retention-limit` endpoint.
     *
     * Sets GitHub Actions cache retention limit for a repository. This determines how long caches will be retained for, if
     * not manually removed or evicted due to size constraints.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:repository` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/cache#set-github-actions-cache-retention-limit-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn set_actions_cache_retention_limit_for_repository(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ActionsCacheRetentionLimitRepository,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/cache/retention-limit",
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
     * Get GitHub Actions cache storage limit for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/cache/storage-limit` endpoint.
     *
     * Gets GitHub Actions cache storage limit for a repository. This determines the maximum size of caches that can be
     * stored before eviction occurs.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:repository` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/cache#get-github-actions-cache-storage-limit-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_actions_cache_storage_limit_for_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsCacheStorageLimitRepository>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/cache/storage-limit",
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
     * Set GitHub Actions cache storage limit for a repository.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/actions/cache/storage-limit` endpoint.
     *
     * Sets GitHub Actions cache storage limit for a repository. This determines the maximum size of caches that can be
     * stored before eviction occurs.
     *
     * OAuth tokens and personal access tokens (classic) need the `admin:repository` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/cache#set-github-actions-cache-storage-limit-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn set_actions_cache_storage_limit_for_repository(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ActionsCacheStorageLimitRepository,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/cache/storage-limit",
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
     * Get GitHub Actions cache usage for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/cache/usage` endpoint.
     *
     * Gets GitHub Actions cache usage for a repository.
     * The data fetched using this API is refreshed approximately every 5 minutes, so values returned from this endpoint may take at least 5 minutes to get updated.
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/cache#get-github-actions-cache-usage-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_actions_cache_usage(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsCacheUsageByRepository>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/cache/usage",
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
     * List GitHub Actions caches for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/caches` endpoint.
     *
     * Lists the GitHub Actions caches for a repository.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/cache#list-github-actions-caches-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `ref_: &str` -- The full Git reference for narrowing down the cache. The `ref` for a branch should be formatted as `refs/heads/<branch name>`. To reference a pull request use `refs/pull/<number>/merge`.
     * * `key: &str` -- An explicit key or prefix for identifying the cache.
     * * `sort: crate::types::ActionsCacheListSort` -- The property to sort the results by. `created_at` means when the cache was created. `last_accessed_at` means when the cache was last accessed. `size_in_bytes` is the size of the cache in bytes.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     */
    pub async fn get_actions_cache_list(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
        ref_: &str,
        key: &str,
        sort: crate::types::ActionsCacheListSort,
        direction: crate::types::Order,
    ) -> ClientResult<crate::Response<crate::types::ActionsCacheList>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !key.is_empty() {
            query_args.push(("key".to_string(), key.to_string()));
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
        if !sort.to_string().is_empty() {
            query_args.push(("sort".to_string(), sort.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/caches?{}",
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
     * Delete GitHub Actions caches for a repository (using a cache key).
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/actions/caches` endpoint.
     *
     * Deletes one or more GitHub Actions caches for a repository, using a complete cache key. By default, all caches that match the provided key are deleted, but you can optionally provide a Git ref to restrict deletions to caches that match both the provided key and the Git ref.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/cache#delete-github-actions-caches-for-a-repository-using-a-cache-key>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `key: &str` -- A key for identifying the cache.
     * * `ref_: &str` -- The full Git reference for narrowing down the cache. The `ref` for a branch should be formatted as `refs/heads/<branch name>`. To reference a pull request use `refs/pull/<number>/merge`.
     */
    pub async fn delete_actions_cache_by_key(
        &self,
        owner: &str,
        repo: &str,
        key: &str,
        ref_: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsCacheList>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !key.is_empty() {
            query_args.push(("key".to_string(), key.to_string()));
        }
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/caches?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Delete a GitHub Actions cache for a repository (using a cache ID).
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/actions/caches/{cache_id}` endpoint.
     *
     * Deletes a GitHub Actions cache for a repository, using a cache ID.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/cache#delete-a-github-actions-cache-for-a-repository-using-a-cache-id>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `cache_id: i64` -- The unique identifier of the GitHub Actions cache.
     */
    pub async fn delete_actions_cache_by_id(
        &self,
        owner: &str,
        repo: &str,
        cache_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/caches/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&cache_id.to_string()),
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
     * Get a job for a workflow run.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/jobs/{job_id}` endpoint.
     *
     * Gets a specific job in a workflow run.
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-jobs#get-a-job-for-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `job_id: i64` -- The unique identifier of the job.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * for `Location:` in the response header to find the URL for the download.
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-jobs#download-job-logs-for-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `job_id: i64` -- The unique identifier of the job.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Re-run a job from a workflow run.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/actions/jobs/{job_id}/rerun` endpoint.
     *
     * Re-run a job and its dependent jobs in a workflow run.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-runs#re-run-a-job-from-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `job_id: i64` -- The unique identifier of the job.
     */
    pub async fn re_run_job_for_workflow_run(
        &self,
        owner: &str,
        repo: &str,
        job_id: i64,
        body: &crate::types::ActionsReRunWorkflowRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/jobs/{}/rerun",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&job_id.to_string()),
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
     * Get the customization template for an OIDC subject claim for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/oidc/customization/sub` endpoint.
     *
     * Gets the customization template for an OpenID Connect (OIDC) subject claim.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/oidc#get-the-customization-template-for-an-oidc-subject-claim-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_custom_oidc_sub_claim_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::OidcCustomSubRepo>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/oidc/customization/sub",
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
     * Set the customization template for an OIDC subject claim for a repository.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/actions/oidc/customization/sub` endpoint.
     *
     * Sets the customization template and `opt-in` or `opt-out` flag for an OpenID Connect (OIDC) subject claim for a repository.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/oidc#set-the-customization-template-for-an-oidc-subject-claim-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn set_custom_oidc_sub_claim_for_repo(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::OidcCustomSubRepo,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/oidc/customization/sub",
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
     * List repository organization secrets.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/organization-secrets` endpoint.
     *
     * Lists all organization secrets shared with a repository without revealing their encrypted
     * values.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read secrets.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/secrets#list-repository-organization-secrets>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_repo_organization_secrets(
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
                "/repos/{}/{}/actions/organization-secrets?{}",
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
     * List repository organization variables.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/organization-variables` endpoint.
     *
     * Lists all organization variables shared with a repository.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read variables.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/variables#list-repository-organization-variables>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 30). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_repo_organization_variables(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsListRepoVariablesResponse>> {
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
                "/repos/{}/{}/actions/organization-variables?{}",
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
     * Get GitHub Actions permissions for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/permissions` endpoint.
     *
     * Gets the GitHub Actions permissions policy for a repository, including whether GitHub Actions is enabled and the actions and reusable workflows allowed to run in the repository.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#get-github-actions-permissions-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_github_actions_permissions_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsRepositoryPermissions>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/permissions",
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
     * Set GitHub Actions permissions for a repository.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/actions/permissions` endpoint.
     *
     * Sets the GitHub Actions permissions policy for enabling GitHub Actions and allowed actions and reusable workflows in the repository.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#set-github-actions-permissions-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
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
     * Get the level of access for workflows outside of the repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/permissions/access` endpoint.
     *
     * Gets the level of access that workflows outside of the repository have to actions and reusable workflows in the repository.
     * This endpoint only applies to private repositories.
     * For more information, see "[Allowing access to components in a private repository](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository#allowing-access-to-components-in-a-private-repository)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#get-the-level-of-access-for-workflows-outside-of-the-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_workflow_access_to_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsWorkflowAccessRepository>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/permissions/access",
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
     * Set the level of access for workflows outside of the repository.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/actions/permissions/access` endpoint.
     *
     * Sets the level of access that workflows outside of the repository have to actions and reusable workflows in the repository.
     * This endpoint only applies to private repositories.
     * For more information, see "[Allowing access to components in a private repository](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository#allowing-access-to-components-in-a-private-repository)".
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#set-the-level-of-access-for-workflows-outside-of-the-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn set_workflow_access_to_repository(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ActionsWorkflowAccessRepository,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/permissions/access",
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
     * Get artifact and log retention settings for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/permissions/artifact-and-log-retention` endpoint.
     *
     * Gets artifact and log retention settings for a repository.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#get-artifact-and-log-retention-settings-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_artifact_and_log_retention_settings_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsArtifactLogRetentionResponse>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/permissions/artifact-and-log-retention",
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
     * Set artifact and log retention settings for a repository.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/actions/permissions/artifact-and-log-retention` endpoint.
     *
     * Sets artifact and log retention settings for a repository.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#set-artifact-and-log-retention-settings-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn set_artifact_and_log_retention_settings_repository(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ActionsArtifactLogRetention,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/permissions/artifact-and-log-retention",
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
     * Get fork PR contributor approval permissions for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/permissions/fork-pr-contributor-approval` endpoint.
     *
     * Gets the fork PR contributor approval policy for a repository.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#get-fork-pr-contributor-approval-permissions-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_fork_pr_contributor_approval_permissions_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsForkPrContributorApproval>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/permissions/fork-pr-contributor-approval",
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
     * Set fork PR contributor approval permissions for a repository.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/actions/permissions/fork-pr-contributor-approval` endpoint.
     *
     * Sets the fork PR contributor approval policy for a repository.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#set-fork-pr-contributor-approval-permissions-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn set_fork_pr_contributor_approval_permissions_repository(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ActionsForkPrContributorApproval,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/permissions/fork-pr-contributor-approval",
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
     * Get private repo fork PR workflow settings for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/permissions/fork-pr-workflows-private-repos` endpoint.
     *
     * Gets the settings for whether workflows from fork pull requests can run on a private repository.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#get-private-repo-fork-pr-workflow-settings-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_private_repo_fork_pr_workflows_settings_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsForkPrWorkflowsPrivateRepos>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/permissions/fork-pr-workflows-private-repos",
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
     * Set private repo fork PR workflow settings for a repository.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/actions/permissions/fork-pr-workflows-private-repos` endpoint.
     *
     * Sets the settings for whether workflows from fork pull requests can run on a private repository.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#set-private-repo-fork-pr-workflow-settings-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn set_private_repo_fork_pr_workflows_settings_repository(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ActionsForkPrWorkflowsPrivateReposRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/permissions/fork-pr-workflows-private-repos",
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
     * Get allowed actions and reusable workflows for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/permissions/selected-actions` endpoint.
     *
     * Gets the settings for selected actions and reusable workflows that are allowed in a repository. To use this endpoint, the repository policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for a repository](#set-github-actions-permissions-for-a-repository)."
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#get-allowed-actions-and-reusable-workflows-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_allowed_actions_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::SelectedActions>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/permissions/selected-actions",
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
     * Set allowed actions and reusable workflows for a repository.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/actions/permissions/selected-actions` endpoint.
     *
     * Sets the actions and reusable workflows that are allowed in a repository. To use this endpoint, the repository permission policy for `allowed_actions` must be configured to `selected`. For more information, see "[Set GitHub Actions permissions for a repository](#set-github-actions-permissions-for-a-repository)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#set-allowed-actions-and-reusable-workflows-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
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
     * Get default workflow permissions for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/permissions/workflow` endpoint.
     *
     * Gets the default workflow permissions granted to the `GITHUB_TOKEN` when running workflows in a repository,
     * as well as if GitHub Actions can submit approving pull request reviews.
     * For more information, see "[Setting the permissions of the GITHUB_TOKEN for your repository](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository#setting-the-permissions-of-the-github_token-for-your-repository)."
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#get-default-workflow-permissions-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn get_github_actions_default_workflow_permissions_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsGetDefaultWorkflowPermissions>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/permissions/workflow",
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
     * Set default workflow permissions for a repository.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/actions/permissions/workflow` endpoint.
     *
     * Sets the default workflow permissions granted to the `GITHUB_TOKEN` when running workflows in a repository, and sets if GitHub Actions
     * can submit approving pull request reviews.
     * For more information, see "[Setting the permissions of the GITHUB_TOKEN for your repository](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-github-actions-settings-for-a-repository#setting-the-permissions-of-the-github_token-for-your-repository)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/permissions#set-default-workflow-permissions-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn set_github_actions_default_workflow_permissions_repository(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ActionsSetDefaultWorkflowPermissions,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/permissions/workflow",
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
     * List self-hosted runners for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runners` endpoint.
     *
     * Lists all self-hosted runners configured in a repository.
     *
     * Authenticated users must have admin access to the repository to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#list-self-hosted-runners-for-a-repository>
     *
     * **Parameters:**
     *
     * * `name: &str` -- The name of a self-hosted runner.
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_self_hosted_runners_for_repo(
        &self,
        name: &str,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsListSelfHostedRunnersOrgResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !name.is_empty() {
            query_args.push(("name".to_string(), name.to_string()));
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
                "/repos/{}/{}/actions/runners?{}",
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
     * List runner applications for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runners/downloads` endpoint.
     *
     * Lists binaries for the runner application that you can download and run.
     *
     * Authenticated users must have admin access to the repository to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#list-runner-applications-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn list_runner_applications_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::RunnerApplication>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runners/downloads",
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
     * List runner applications for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runners/downloads` endpoint.
     *
     * As opposed to `list_runner_applications_for_repo`, this function returns all the pages of the request at once.
     *
     * Lists binaries for the runner application that you can download and run.
     *
     * Authenticated users must have admin access to the repository to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#list-runner-applications-for-a-repository>
     */
    pub async fn list_all_runner_applications_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::RunnerApplication>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runners/downloads",
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
     * Create configuration for a just-in-time runner for a repository.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/actions/runners/generate-jitconfig` endpoint.
     *
     * Generates a configuration that can be passed to the runner application at startup.
     *
     * The authenticated user must have admin access to the repository.
     *
     * OAuth tokens and personal access tokens (classic) need the`repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#create-configuration-for-a-just-in-time-runner-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn generate_runner_jitconfig_for_repo(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ActionsGenerateRunnerJitconfigOrgRequest,
    ) -> ClientResult<crate::Response<crate::types::ActionsRunnerJitconfigResponse>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runners/generate-jitconfig",
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
     * Create a registration token for a repository.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/actions/runners/registration-token` endpoint.
     *
     * Returns a token that you can pass to the `config` script. The token expires after one hour.
     *
     * For example, you can replace `TOKEN` in the following example with the registration token provided by this endpoint to configure your self-hosted runner:
     *
     * ```
     * ./config.sh --url https://github.com/octo-org --token TOKEN
     * ```
     *
     * Authenticated users must have admin access to the repository to use this endpoint.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#create-a-registration-token-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_registration_token_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::AuthenticationToken>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runners/registration-token",
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
     * Create a remove token for a repository.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/actions/runners/remove-token` endpoint.
     *
     * Returns a token that you can pass to the `config` script to remove a self-hosted runner from an repository. The token expires after one hour.
     *
     * For example, you can replace `TOKEN` in the following example with the registration token provided by this endpoint to remove your self-hosted runner from an organization:
     *
     * ```
     * ./config.sh remove --token TOKEN
     * ```
     *
     * Authenticated users must have admin access to the repository to use this endpoint.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#create-a-remove-token-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_remove_token_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::AuthenticationToken>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runners/remove-token",
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
     * Get a self-hosted runner for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runners/{runner_id}` endpoint.
     *
     * Gets a specific self-hosted runner configured in a repository.
     *
     * Authenticated users must have admin access to the repository to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#get-a-self-hosted-runner-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Authenticated users must have admin access to the repository to use this endpoint.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#delete-a-self-hosted-runner-from-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * List labels for a self-hosted runner for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runners/{runner_id}/labels` endpoint.
     *
     * Lists all labels for a self-hosted runner configured in a repository.
     *
     * Authenticated users must have admin access to the repository to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#list-labels-for-a-self-hosted-runner-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `runner_id: i64` -- Unique identifier of the self-hosted runner.
     */
    pub async fn list_labels_for_self_hosted_runner_for_repo(
        &self,
        owner: &str,
        repo: &str,
        runner_id: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsRunnerLabelsResponse>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runners/{}/labels",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Set custom labels for a self-hosted runner for a repository.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/actions/runners/{runner_id}/labels` endpoint.
     *
     * Remove all previous custom labels and set the new custom labels for a specific
     * self-hosted runner configured in a repository.
     *
     * Authenticated users must have admin access to the repository to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#set-custom-labels-for-a-self-hosted-runner-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `runner_id: i64` -- Unique identifier of the self-hosted runner.
     */
    pub async fn set_custom_labels_for_self_hosted_runner_for_repo(
        &self,
        owner: &str,
        repo: &str,
        runner_id: i64,
        body: &crate::types::IssuesAddLabelsRequest,
    ) -> ClientResult<crate::Response<crate::types::ActionsRunnerLabelsResponse>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runners/{}/labels",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&runner_id.to_string()),
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
     * Add custom labels to a self-hosted runner for a repository.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/actions/runners/{runner_id}/labels` endpoint.
     *
     * Adds custom labels to a self-hosted runner configured in a repository.
     *
     * Authenticated users must have admin access to the organization to use this endpoint.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#add-custom-labels-to-a-self-hosted-runner-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `runner_id: i64` -- Unique identifier of the self-hosted runner.
     */
    pub async fn add_custom_labels_to_self_hosted_runner_for_repo(
        &self,
        owner: &str,
        repo: &str,
        runner_id: i64,
        body: &crate::types::IssuesAddLabelsRequest,
    ) -> ClientResult<crate::Response<crate::types::ActionsRunnerLabelsResponse>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runners/{}/labels",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&runner_id.to_string()),
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
     * Remove all custom labels from a self-hosted runner for a repository.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/actions/runners/{runner_id}/labels` endpoint.
     *
     * Remove all custom labels from a self-hosted runner configured in a
     * repository. Returns the remaining read-only labels from the runner.
     *
     * Authenticated users must have admin access to the repository to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#remove-all-custom-labels-from-a-self-hosted-runner-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `runner_id: i64` -- Unique identifier of the self-hosted runner.
     */
    pub async fn remove_all_custom_labels_from_self_hosted_runner_for_repo(
        &self,
        owner: &str,
        repo: &str,
        runner_id: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsRunnerLabelsResponse>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runners/{}/labels",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Remove a custom label from a self-hosted runner for a repository.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/actions/runners/{runner_id}/labels/{name}` endpoint.
     *
     * Remove a custom label from a self-hosted runner configured
     * in a repository. Returns the remaining labels from the runner.
     *
     * This endpoint returns a `404 Not Found` status if the custom label is not
     * present on the runner.
     *
     * Authenticated users must have admin access to the repository to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/self-hosted-runners#remove-a-custom-label-from-a-self-hosted-runner-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `runner_id: i64` -- Unique identifier of the self-hosted runner.
     * * `name: &str` -- The name of a self-hosted runner's custom label.
     */
    pub async fn remove_custom_label_from_self_hosted_runner_for_repo(
        &self,
        owner: &str,
        repo: &str,
        runner_id: i64,
        name: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsRunnerLabelsResponse>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runners/{}/labels/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&runner_id.to_string()),
                crate::progenitor_support::encode_path(&name.to_string()),
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
     * Lists all workflow runs for a repository. You can use parameters to narrow the list of results. For more information about using parameters, see [Parameters](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#parameters).
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
     *
     * This endpoint will return up to 1,000 results for each search when using the following parameters: `actor`, `branch`, `check_suite_id`, `created`, `event`, `head_sha`, `status`.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-runs#list-workflow-runs-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `actor: &str` -- Returns someone's workflow runs. Use the login for the user who created the `push` associated with the check suite or workflow run.
     * * `branch: &str` -- Returns workflow runs associated with a branch. Use the name of the branch of the `push`.
     * * `event: &str` -- Returns workflow run triggered by the event you specify. For example, `push`, `pull_request` or `issue`. For more information, see "[Events that trigger workflows](https://docs.github.com/actions/automating-your-workflow-with-github-actions/events-that-trigger-workflows).".
     * * `status: crate::types::WorkflowRunStatus` -- Returns workflow runs with the check run `status` or `conclusion` that you specify. For example, a conclusion can be `success` or a status can be `in_progress`. Only GitHub Actions can set a status of `waiting`, `pending`, or `requested`.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `created: chrono::DateTime<chrono::Utc>` -- Returns workflow runs created within the given date-time range. For more information on the syntax, see "[Understanding the search syntax](https://docs.github.com/search-github/getting-started-with-searching-on-github/understanding-the-search-syntax#query-for-dates).".
     * * `exclude_pull_requests: bool` -- If `true` pull requests are omitted from the response (empty array).
     * * `check_suite_id: i64` -- Returns workflow runs with the `check_suite_id` that you specify.
     * * `head_sha: &str` -- Only returns workflow runs that are associated with the specified `head_sha`.
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
        created: Option<chrono::DateTime<chrono::Utc>>,
        exclude_pull_requests: bool,
        check_suite_id: i64,
        head_sha: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsListWorkflowRunsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !actor.is_empty() {
            query_args.push(("actor".to_string(), actor.to_string()));
        }
        if !branch.is_empty() {
            query_args.push(("branch".to_string(), branch.to_string()));
        }
        if check_suite_id > 0 {
            query_args.push(("check_suite_id".to_string(), check_suite_id.to_string()));
        }
        if let Some(date) = created {
            query_args.push(("created".to_string(), date.to_rfc3339()));
        }
        if !event.is_empty() {
            query_args.push(("event".to_string(), event.to_string()));
        }
        if exclude_pull_requests {
            query_args.push((
                "exclude_pull_requests".to_string(),
                exclude_pull_requests.to_string(),
            ));
        }
        if !head_sha.is_empty() {
            query_args.push(("head_sha".to_string(), head_sha.to_string()));
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
     * Get a workflow run.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runs/{run_id}` endpoint.
     *
     * Gets a specific workflow run.
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-runs#get-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `run_id: i64` -- The unique identifier of the workflow run.
     * * `exclude_pull_requests: bool` -- If `true` pull requests are omitted from the response (empty array).
     */
    pub async fn get_workflow_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
        exclude_pull_requests: bool,
    ) -> ClientResult<crate::Response<crate::types::WorkflowRunData>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if exclude_pull_requests {
            query_args.push((
                "exclude_pull_requests".to_string(),
                exclude_pull_requests.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Delete a workflow run.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/actions/runs/{run_id}` endpoint.
     *
     * Deletes a specific workflow run.
     *
     * Anyone with write access to the repository can use this endpoint.
     *
     * If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-runs#delete-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `run_id: i64` -- The unique identifier of the workflow run.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Anyone with read access to the repository can use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-runs#get-the-review-history-for-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `run_id: i64` -- The unique identifier of the workflow run.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Anyone with read access to the repository can use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-runs#get-the-review-history-for-a-workflow-run>
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-runs#approve-a-workflow-run-for-a-fork-pull-request>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `run_id: i64` -- The unique identifier of the workflow run.
     */
    pub async fn approve_workflow_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}/approve",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Lists artifacts for a workflow run.
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
     *
     * FROM: <https://docs.github.com/rest/actions/artifacts#list-workflow-run-artifacts>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `run_id: i64` -- The unique identifier of the workflow run.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `name: &str` -- The name field of an artifact. When specified, only artifacts with this name will be returned.
     * * `direction: crate::types::Order` -- Determines whether the first search result returned is the highest number of matches (`desc`) or lowest number of matches (`asc`). This parameter is ignored unless you provide `sort`.
     */
    pub async fn list_workflow_run_artifacts(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
        per_page: i64,
        page: i64,
        name: &str,
        direction: crate::types::Order,
    ) -> ClientResult<crate::Response<crate::types::ActionsListArtifactsRepoResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !direction.to_string().is_empty() {
            query_args.push(("direction".to_string(), direction.to_string()));
        }
        if !name.is_empty() {
            query_args.push(("name".to_string(), name.to_string()));
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
                "/repos/{}/{}/actions/runs/{}/artifacts?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Get a workflow run attempt.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number}` endpoint.
     *
     * Gets a specific workflow run attempt.
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-runs#get-a-workflow-run-attempt>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `run_id: i64` -- The unique identifier of the workflow run.
     * * `attempt_number: i64` -- The attempt number of the workflow run.
     * * `exclude_pull_requests: bool` -- If `true` pull requests are omitted from the response (empty array).
     */
    pub async fn get_workflow_run_attempt(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
        attempt_number: i64,
        exclude_pull_requests: bool,
    ) -> ClientResult<crate::Response<crate::types::WorkflowRunData>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if exclude_pull_requests {
            query_args.push((
                "exclude_pull_requests".to_string(),
                exclude_pull_requests.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}/attempts/{}?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&run_id.to_string()),
                crate::progenitor_support::encode_path(&attempt_number.to_string()),
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
     * List jobs for a workflow run attempt.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number}/jobs` endpoint.
     *
     * Lists jobs for a specific workflow run attempt. You can use parameters to narrow the list of results. For more information
     * about using parameters, see [Parameters](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#parameters).
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint  with a private repository.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-jobs#list-jobs-for-a-workflow-run-attempt>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `run_id: i64` -- The unique identifier of the workflow run.
     * * `attempt_number: i64` -- The attempt number of the workflow run.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_jobs_for_workflow_run_attempt(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
        attempt_number: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsListJobsWorkflowRunResponse>> {
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
                "/repos/{}/{}/actions/runs/{}/attempts/{}/jobs?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&run_id.to_string()),
                crate::progenitor_support::encode_path(&attempt_number.to_string()),
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
     * Download workflow run attempt logs.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runs/{run_id}/attempts/{attempt_number}/logs` endpoint.
     *
     * Gets a redirect URL to download an archive of log files for a specific workflow run attempt. This link expires after
     * 1 minute. Look for `Location:` in the response header to find the URL for the download.
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-runs#download-workflow-run-attempt-logs>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `run_id: i64` -- The unique identifier of the workflow run.
     * * `attempt_number: i64` -- The attempt number of the workflow run.
     */
    pub async fn download_workflow_run_attempt_logs(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
        attempt_number: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}/attempts/{}/logs",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&run_id.to_string()),
                crate::progenitor_support::encode_path(&attempt_number.to_string()),
            ),
            None,
        );
        self.client
            .get(
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
     * Cancels a workflow run using its `id`.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-runs#cancel-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `run_id: i64` -- The unique identifier of the workflow run.
     */
    pub async fn cancel_workflow_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}/cancel",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Review custom deployment protection rules for a workflow run.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/actions/runs/{run_id}/deployment_protection_rule` endpoint.
     *
     * Approve or reject custom deployment protection rules provided by a GitHub App for a workflow run. For more information, see "[Using environments for deployment](https://docs.github.com/actions/deployment/targeting-different-environments/using-environments-for-deployment)."
     *
     * > [!NOTE]
     * > GitHub Apps can only review their own custom deployment protection rules. To approve or reject pending deployments that are waiting for review from a specific person or team, see [`POST /repos/{owner}/{repo}/actions/runs/{run_id}/pending_deployments`](/rest/actions/workflow-runs#review-pending-deployments-for-a-workflow-run).
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-runs#review-custom-deployment-protection-rules-for-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `run_id: i64` -- The unique identifier of the workflow run.
     */
    pub async fn review_custom_gates_for_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
        body: &crate::types::ActionsReviewCustomGatesRunRequestAnyOf,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}/deployment_protection_rule",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Force cancel a workflow run.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/actions/runs/{run_id}/force-cancel` endpoint.
     *
     * Cancels a workflow run and bypasses conditions that would otherwise cause a workflow execution to continue, such as an `always()` condition on a job.
     * You should only use this endpoint to cancel a workflow run when the workflow run is not responding to [`POST /repos/{owner}/{repo}/actions/runs/{run_id}/cancel`](/rest/actions/workflow-runs#cancel-a-workflow-run).
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-runs#force-cancel-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `run_id: i64` -- The unique identifier of the workflow run.
     */
    pub async fn force_cancel_workflow_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}/force-cancel",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Lists jobs for a workflow run. You can use parameters to narrow the list of results. For more information
     * about using parameters, see [Parameters](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#parameters).
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-jobs#list-jobs-for-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `run_id: i64` -- The unique identifier of the workflow run.
     * * `filter: crate::types::ActionsListJobsWorkflowRunFilter` -- Filters jobs by their `completed_at` timestamp. `latest` returns jobs from the most recent execution of the workflow run. `all` returns all jobs for a workflow run, including from old executions of the workflow run.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * `Location:` in the response header to find the URL for the download.
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-runs#download-workflow-run-logs>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `run_id: i64` -- The unique identifier of the workflow run.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Deletes all logs for a workflow run.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-runs#delete-workflow-run-logs>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `run_id: i64` -- The unique identifier of the workflow run.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Anyone with read access to the repository can use this endpoint.
     *
     * If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-runs#get-pending-deployments-for-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `run_id: i64` -- The unique identifier of the workflow run.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Anyone with read access to the repository can use this endpoint.
     *
     * If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-runs#get-pending-deployments-for-a-workflow-run>
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Required reviewers with read access to the repository contents and deployments can use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-runs#review-pending-deployments-for-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `run_id: i64` -- The unique identifier of the workflow run.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Re-runs your workflow run using its `id`.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-runs#re-run-a-workflow>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `run_id: i64` -- The unique identifier of the workflow run.
     */
    pub async fn re_run_workflow(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
        body: &crate::types::ActionsReRunWorkflowRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}/rerun",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Re-run failed jobs from a workflow run.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/actions/runs/{run_id}/rerun-failed-jobs` endpoint.
     *
     * Re-run all of the failed jobs and their dependent jobs in a workflow run using the `id` of the workflow run.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-runs#re-run-failed-jobs-from-a-workflow-run>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `run_id: i64` -- The unique identifier of the workflow run.
     */
    pub async fn re_run_workflow_failed_jobs(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
        body: &crate::types::ActionsReRunWorkflowRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/runs/{}/rerun-failed-jobs",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Get workflow run usage.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/runs/{run_id}/timing` endpoint.
     *
     * > [!WARNING]  
     * > This endpoint is in the process of closing down. Refer to "[Actions Get workflow usage and Get workflow run usage endpoints closing down](https://github.blog/changelog/2025-02-02-actions-get-workflow-usage-and-get-workflow-run-usage-endpoints-closing-down/)" for more information.
     *
     * Gets the number of billable minutes and total run time for a specific workflow run. Billable minutes only apply to workflows in private repositories that use GitHub-hosted runners. Usage is listed for each GitHub-hosted runner operating system in milliseconds. Any job re-runs are also included in the usage. The usage does not include the multiplier for macOS and Windows runners and is not rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-runs#get-workflow-run-usage>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `run_id: i64` -- The unique identifier of the workflow run.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
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
     * Lists all secrets available in a repository without revealing their encrypted
     * values.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read secrets.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/secrets#list-repository-secrets>
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
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/secrets/public-key` endpoint.
     *
     * Gets your public key, which you need to encrypt secrets. You need to
     * encrypt a secret before you can create or update secrets.
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/secrets#get-a-repository-public-key>
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
    ) -> ClientResult<crate::Response<crate::types::ActionsPublicKey>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/secrets/public-key",
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
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/secrets/{secret_name}` endpoint.
     *
     * Gets a single repository secret without revealing its encrypted value.
     *
     * The authenticated user must have collaborator access to the repository to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/secrets#get-a-repository-secret>
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
                "/repos/{}/{}/actions/secrets/{}",
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
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/actions/secrets/{secret_name}` endpoint.
     *
     * Creates or updates a repository secret with an encrypted value. Encrypt your secret using
     * [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read secrets.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/secrets#create-or-update-a-repository-secret>
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
        body: &crate::types::ActionsCreateUpdateRepoSecretRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/secrets/{}",
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
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/actions/secrets/{secret_name}` endpoint.
     *
     * Deletes a secret in a repository using the secret name.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read secrets.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/secrets#delete-a-repository-secret>
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
                "/repos/{}/{}/actions/secrets/{}",
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
     * List repository variables.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/variables` endpoint.
     *
     * Lists all repository variables.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read variables.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/variables#list-repository-variables>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 30). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_repo_variables(
        &self,
        owner: &str,
        repo: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsListRepoVariablesResponse>> {
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
                "/repos/{}/{}/actions/variables?{}",
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
     * Create a repository variable.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/actions/variables` endpoint.
     *
     * Creates a repository variable that you can reference in a GitHub Actions workflow.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read variables.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/variables#create-a-repository-variable>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_repo_variable(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::ActionsCreateRepoVariableRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/variables",
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
     * Get a repository variable.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/variables/{name}` endpoint.
     *
     * Gets a specific variable in a repository.
     *
     * The authenticated user must have collaborator access to the repository to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/variables#get-a-repository-variable>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `name: &str` -- The name of the variable.
     */
    pub async fn get_repo_variable(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsVariable>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/variables/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&name.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Delete a repository variable.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/actions/variables/{name}` endpoint.
     *
     * Deletes a repository variable using the variable name.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read variables.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/variables#delete-a-repository-variable>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `name: &str` -- The name of the variable.
     */
    pub async fn delete_repo_variable(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/variables/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&name.to_string()),
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
     * Update a repository variable.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/actions/variables/{name}` endpoint.
     *
     * Updates a repository variable that you can reference in a GitHub Actions workflow.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read variables.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/variables#update-a-repository-variable>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `name: &str` -- The name of the variable.
     */
    pub async fn update_repo_variable(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
        body: &crate::types::ActionsUpdateRepoVariableRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/actions/variables/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&name.to_string()),
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
     * List repository workflows.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/workflows` endpoint.
     *
     * Lists the workflows in a repository.
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
     *
     * FROM: <https://docs.github.com/rest/actions/workflows#list-repository-workflows>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
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
     * Get a workflow.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/workflows/{workflow_id}` endpoint.
     *
     * Gets a specific workflow. You can replace `workflow_id` with the workflow
     * file name. For example, you could use `main.yaml`.
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
     *
     * FROM: <https://docs.github.com/rest/actions/workflows#get-a-workflow>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&workflow_id.to_string()),
            ),
            None,
        );
        self.client
            .get(
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
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/workflows#disable-a-workflow>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&workflow_id.to_string()),
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
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/workflows#create-a-workflow-dispatch-event>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&workflow_id.to_string()),
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
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/workflows#enable-a-workflow>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&workflow_id.to_string()),
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
     * List workflow runs for a workflow.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/actions/workflows/{workflow_id}/runs` endpoint.
     *
     * List all workflow runs for a workflow. You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`. You can use parameters to narrow the list of results. For more information about using parameters, see [Parameters](https://docs.github.com/rest/guides/getting-started-with-the-rest-api#parameters).
     *
     * Anyone with read access to the repository can use this endpoint
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
     *
     * This endpoint will return up to 1,000 results for each search when using the following parameters: `actor`, `branch`, `check_suite_id`, `created`, `event`, `head_sha`, `status`.
     *
     * FROM: <https://docs.github.com/rest/actions/workflow-runs#list-workflow-runs-for-a-workflow>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `workflow_id: &str` -- The ID of the workflow. You can also pass the workflow file name as a string.
     * * `actor: &str` -- Returns someone's workflow runs. Use the login for the user who created the `push` associated with the check suite or workflow run.
     * * `branch: &str` -- Returns workflow runs associated with a branch. Use the name of the branch of the `push`.
     * * `event: &str` -- Returns workflow run triggered by the event you specify. For example, `push`, `pull_request` or `issue`. For more information, see "[Events that trigger workflows](https://docs.github.com/actions/automating-your-workflow-with-github-actions/events-that-trigger-workflows).".
     * * `status: crate::types::WorkflowRunStatus` -- Returns workflow runs with the check run `status` or `conclusion` that you specify. For example, a conclusion can be `success` or a status can be `in_progress`. Only GitHub Actions can set a status of `waiting`, `pending`, or `requested`.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `created: chrono::DateTime<chrono::Utc>` -- Returns workflow runs created within the given date-time range. For more information on the syntax, see "[Understanding the search syntax](https://docs.github.com/search-github/getting-started-with-searching-on-github/understanding-the-search-syntax#query-for-dates).".
     * * `exclude_pull_requests: bool` -- If `true` pull requests are omitted from the response (empty array).
     * * `check_suite_id: i64` -- Returns workflow runs with the `check_suite_id` that you specify.
     * * `head_sha: &str` -- Only returns workflow runs that are associated with the specified `head_sha`.
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
        created: Option<chrono::DateTime<chrono::Utc>>,
        exclude_pull_requests: bool,
        check_suite_id: i64,
        head_sha: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsListWorkflowRunsResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !actor.is_empty() {
            query_args.push(("actor".to_string(), actor.to_string()));
        }
        if !branch.is_empty() {
            query_args.push(("branch".to_string(), branch.to_string()));
        }
        if check_suite_id > 0 {
            query_args.push(("check_suite_id".to_string(), check_suite_id.to_string()));
        }
        if let Some(date) = created {
            query_args.push(("created".to_string(), date.to_rfc3339()));
        }
        if !event.is_empty() {
            query_args.push(("event".to_string(), event.to_string()));
        }
        if exclude_pull_requests {
            query_args.push((
                "exclude_pull_requests".to_string(),
                exclude_pull_requests.to_string(),
            ));
        }
        if !head_sha.is_empty() {
            query_args.push(("head_sha".to_string(), head_sha.to_string()));
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&workflow_id.to_string()),
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
     * > [!WARNING]  
     * > This endpoint is in the process of closing down. Refer to "[Actions Get workflow usage and Get workflow run usage endpoints closing down](https://github.blog/changelog/2025-02-02-actions-get-workflow-usage-and-get-workflow-run-usage-endpoints-closing-down/)" for more information.
     *
     * Gets the number of billable minutes used by a specific workflow during the current billing cycle. Billable minutes only apply to workflows in private repositories that use GitHub-hosted runners. Usage is listed for each GitHub-hosted runner operating system in milliseconds. Any job re-runs are also included in the usage. The usage does not include the multiplier for macOS and Windows runners and is not rounded up to the nearest whole minute. For more information, see "[Managing billing for GitHub Actions](https://docs.github.com/github/setting-up-and-managing-billing-and-payments-on-github/managing-billing-for-github-actions)".
     *
     * You can replace `workflow_id` with the workflow file name. For example, you could use `main.yaml`.
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint with a private repository.
     *
     * FROM: <https://docs.github.com/rest/actions/workflows#get-workflow-usage>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
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
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&workflow_id.to_string()),
            ),
            None,
        );
        self.client
            .get(
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
     * This function performs a `GET` to the `/repos/{owner}/{repo}/environments/{environment_name}/secrets` endpoint.
     *
     * Lists all secrets available in an environment without revealing their
     * encrypted values.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read secrets.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/secrets#list-environment-secrets>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_environment_secrets(
        &self,
        owner: &str,
        repo: &str,
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
                "/repos/{}/{}/environments/{}/secrets?{}",
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
     * Get an environment public key.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/environments/{environment_name}/secrets/public-key` endpoint.
     *
     * Get the public key for an environment, which you need to encrypt environment
     * secrets. You need to encrypt a secret before you can create or update secrets.
     *
     * Anyone with read access to the repository can use this endpoint.
     *
     * If the repository is private, OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/secrets#get-an-environment-public-key>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     */
    pub async fn get_environment_public_key(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsPublicKey>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}/secrets/public-key",
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
     * Get an environment secret.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/environments/{environment_name}/secrets/{secret_name}` endpoint.
     *
     * Gets a single environment secret without revealing its encrypted value.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read secrets.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/secrets#get-an-environment-secret>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     * * `secret_name: &str` -- The name of the secret.
     */
    pub async fn get_environment_secret(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
        secret_name: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsSecret>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}/secrets/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&environment_name.to_string()),
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
     * Create or update an environment secret.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/environments/{environment_name}/secrets/{secret_name}` endpoint.
     *
     * Creates or updates an environment secret with an encrypted value. Encrypt your secret using
     * [LibSodium](https://libsodium.gitbook.io/doc/bindings_for_other_languages). For more information, see "[Encrypting secrets for the REST API](https://docs.github.com/rest/guides/encrypting-secrets-for-the-rest-api)."
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read secrets.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/secrets#create-or-update-an-environment-secret>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     * * `secret_name: &str` -- The name of the secret.
     */
    pub async fn create_or_update_environment_secret(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
        secret_name: &str,
        body: &crate::types::ActionsCreateUpdateRepoSecretRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}/secrets/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&environment_name.to_string()),
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
     * Delete an environment secret.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/environments/{environment_name}/secrets/{secret_name}` endpoint.
     *
     * Deletes a secret in an environment using the secret name.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read secrets.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/secrets#delete-an-environment-secret>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     * * `secret_name: &str` -- The name of the secret.
     */
    pub async fn delete_environment_secret(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
        secret_name: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}/secrets/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&environment_name.to_string()),
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
     * List environment variables.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/environments/{environment_name}/variables` endpoint.
     *
     * Lists all environment variables.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read variables.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/variables#list-environment-variables>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     * * `per_page: i64` -- The number of results per page (max 30). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_environment_variables(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<crate::types::ActionsListRepoVariablesResponse>> {
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
                "/repos/{}/{}/environments/{}/variables?{}",
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
     * Create an environment variable.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/environments/{environment_name}/variables` endpoint.
     *
     * Create an environment variable that you can reference in a GitHub Actions workflow.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read variables.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/variables#create-an-environment-variable>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     */
    pub async fn create_environment_variable(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
        body: &crate::types::ActionsCreateRepoVariableRequest,
    ) -> ClientResult<crate::Response<crate::types::OrgRules>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}/variables",
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
     * Get an environment variable.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/environments/{environment_name}/variables/{name}` endpoint.
     *
     * Gets a specific variable in an environment.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read variables.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/variables#get-an-environment-variable>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     * * `name: &str` -- The name of the variable.
     */
    pub async fn get_environment_variable(
        &self,
        owner: &str,
        repo: &str,
        environment_name: &str,
        name: &str,
    ) -> ClientResult<crate::Response<crate::types::ActionsVariable>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}/variables/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&environment_name.to_string()),
                crate::progenitor_support::encode_path(&name.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Delete an environment variable.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/environments/{environment_name}/variables/{name}` endpoint.
     *
     * Deletes an environment variable using the variable name.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read variables.
     *
     * OAuth tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/variables#delete-an-environment-variable>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `name: &str` -- The name of the variable.
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     */
    pub async fn delete_environment_variable(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
        environment_name: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}/variables/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&environment_name.to_string()),
                crate::progenitor_support::encode_path(&name.to_string()),
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
     * Update an environment variable.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/environments/{environment_name}/variables/{name}` endpoint.
     *
     * Updates an environment variable that you can reference in a GitHub Actions workflow.
     *
     * Authenticated users must have collaborator access to a repository to create, update, or read variables.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/actions/variables#update-an-environment-variable>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `name: &str` -- The name of the variable.
     * * `environment_name: &str` -- The name of the environment. The name must be URL encoded. For example, any slashes in the name must be replaced with `%2F`.
     */
    pub async fn update_environment_variable(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
        environment_name: &str,
        body: &crate::types::ActionsUpdateRepoVariableRequest,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/environments/{}/variables/{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&environment_name.to_string()),
                crate::progenitor_support::encode_path(&name.to_string()),
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
