use crate::Client;
use crate::ClientResult;

pub struct Packages {
    pub client: Client,
}

impl Packages {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Packages { client }
    }

    /**
     * Get list of conflicting packages during Docker migration for organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/docker/conflicts` endpoint.
     *
     * Lists all packages that are in a specific organization, are readable by the requesting user, and that encountered a conflict during a Docker migration.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/packages/packages#get-list-of-conflicting-packages-during-docker-migration-for-organization>
     *
     * **Parameters:**
     *
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn list_docker_migration_conflicting_packages_for_organization(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::PackageData>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/docker/conflicts",
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
     * Get list of conflicting packages during Docker migration for organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/docker/conflicts` endpoint.
     *
     * As opposed to `list_docker_migration_conflicting_packages_for_organization`, this function returns all the pages of the request at once.
     *
     * Lists all packages that are in a specific organization, are readable by the requesting user, and that encountered a conflict during a Docker migration.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/packages/packages#get-list-of-conflicting-packages-during-docker-migration-for-organization>
     */
    pub async fn list_all_docker_migration_conflicting_packages_for_organization(
        &self,
        org: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::PackageData>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/docker/conflicts",
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
     * List packages for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/packages` endpoint.
     *
     * Lists packages in an organization readable by the user.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#list-packages-for-an-organization>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `visibility: crate::types::PackageVisibilityData` -- The selected visibility of the packages.  This parameter is optional and only filters an existing result set.
     *  
     *  The `internal` visibility is only supported for GitHub Packages registries that allow for granular permissions. For other ecosystems `internal` is synonymous with `private`.
     *  For the list of GitHub Packages registries that support granular permissions, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_packages_for_organization(
        &self,
        package_type: crate::types::PackageType,
        org: &str,
        visibility: crate::types::PackageVisibilityData,
        page: i64,
        per_page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::PackageData>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !package_type.to_string().is_empty() {
            query_args.push(("package_type".to_string(), package_type.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !visibility.to_string().is_empty() {
            query_args.push(("visibility".to_string(), visibility.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/packages?{}",
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
     * List packages for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/packages` endpoint.
     *
     * As opposed to `list_packages_for_organization`, this function returns all the pages of the request at once.
     *
     * Lists packages in an organization readable by the user.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#list-packages-for-an-organization>
     */
    pub async fn list_all_packages_for_organization(
        &self,
        package_type: crate::types::PackageType,
        org: &str,
        visibility: crate::types::PackageVisibilityData,
    ) -> ClientResult<crate::Response<Vec<crate::types::PackageData>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !package_type.to_string().is_empty() {
            query_args.push(("package_type".to_string(), package_type.to_string()));
        }
        if !visibility.to_string().is_empty() {
            query_args.push(("visibility".to_string(), visibility.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/packages?{}",
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
     * Get a package for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/packages/{package_type}/{package_name}` endpoint.
     *
     * Gets a specific package in an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#get-a-package-for-an-organization>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn get_package_for_organization(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::PackageData>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/packages/{}/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Delete a package for an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/packages/{package_type}/{package_name}` endpoint.
     *
     * Deletes an entire package in an organization. You cannot delete a public package if any version of the package has more than 5,000 downloads. In this scenario, contact GitHub support for further assistance.
     *
     * The authenticated user must have admin permissions in the organization to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must also have admin permissions to the package. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` and `delete:packages` scopes to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#delete-a-package-for-an-organization>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     */
    pub async fn delete_package_for_org(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        org: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/packages/{}/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
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
     * Restore a package for an organization.
     *
     * This function performs a `POST` to the `/orgs/{org}/packages/{package_type}/{package_name}/restore` endpoint.
     *
     * Restores an entire package in an organization.
     *
     * You can restore a deleted package under the following conditions:
     *   - The package was deleted within the last 30 days.
     *   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.
     *
     * The authenticated user must have admin permissions in the organization to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must also have admin permissions to the package. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` and `write:packages` scopes to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#restore-a-package-for-an-organization>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `token: &str` -- package token.
     */
    pub async fn restore_package_for_org(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        org: &str,
        token: &str,
    ) -> ClientResult<crate::Response<()>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !token.is_empty() {
            query_args.push(("token".to_string(), token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/packages/{}/{}/restore?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
                query_
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
     * List package versions for a package owned by an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/packages/{package_type}/{package_name}/versions` endpoint.
     *
     * Lists package versions for a package owned by an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#list-package-versions-for-a-package-owned-by-an-organization>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `state: crate::types::PackagesGetAllPackageVersionsOwnedByOrgState` -- The state of the package, either active or deleted.
     */
    pub async fn get_all_package_versions_for_package_owned_by_org(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        org: &str,
        page: i64,
        per_page: i64,
        state: crate::types::PackagesGetAllPackageVersionsOwnedByOrgState,
    ) -> ClientResult<crate::Response<Vec<crate::types::PackageVersion>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/packages/{}/{}/versions?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
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
     * List package versions for a package owned by an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/packages/{package_type}/{package_name}/versions` endpoint.
     *
     * As opposed to `get_all_package_versions_for_package_owned_by_org`, this function returns all the pages of the request at once.
     *
     * Lists package versions for a package owned by an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#list-package-versions-for-a-package-owned-by-an-organization>
     */
    pub async fn get_all_all_package_versions_for_package_owned_by_org(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        org: &str,
        state: crate::types::PackagesGetAllPackageVersionsOwnedByOrgState,
    ) -> ClientResult<crate::Response<Vec<crate::types::PackageVersion>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/packages/{}/{}/versions?{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
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
     * Get a package version for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/packages/{package_type}/{package_name}/versions/{package_version_id}` endpoint.
     *
     * Gets a specific package version in an organization.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#get-a-package-version-for-an-organization>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `package_version_id: i64` -- Unique identifier of the package version.
     */
    pub async fn get_package_version_for_organization(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        org: &str,
        package_version_id: i64,
    ) -> ClientResult<crate::Response<crate::types::PackageVersion>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/packages/{}/{}/versions/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
                crate::progenitor_support::encode_path(&package_version_id.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Delete package version for an organization.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/packages/{package_type}/{package_name}/versions/{package_version_id}` endpoint.
     *
     * Deletes a specific package version in an organization. If the package is public and the package version has more than 5,000 downloads, you cannot delete the package version. In this scenario, contact GitHub support for further assistance.
     *
     * The authenticated user must have admin permissions in the organization to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must also have admin permissions to the package. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` and `delete:packages` scopes to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#delete-package-version-for-an-organization>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `package_version_id: i64` -- Unique identifier of the package version.
     */
    pub async fn delete_package_version_for_org(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        org: &str,
        package_version_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/packages/{}/{}/versions/{}",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
                crate::progenitor_support::encode_path(&package_version_id.to_string()),
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
     * Restore package version for an organization.
     *
     * This function performs a `POST` to the `/orgs/{org}/packages/{package_type}/{package_name}/versions/{package_version_id}/restore` endpoint.
     *
     * Restores a specific package version in an organization.
     *
     * You can restore a deleted package under the following conditions:
     *   - The package was deleted within the last 30 days.
     *   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.
     *
     * The authenticated user must have admin permissions in the organization to use this endpoint. If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must also have admin permissions to the package. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` and `write:packages` scopes to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#restore-package-version-for-an-organization>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `org: &str` -- The organization name. The name is not case sensitive.
     * * `package_version_id: i64` -- Unique identifier of the package version.
     */
    pub async fn restore_package_version_for_org(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        org: &str,
        package_version_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/packages/{}/{}/versions/{}/restore",
                crate::progenitor_support::encode_path(&org.to_string()),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
                crate::progenitor_support::encode_path(&package_version_id.to_string()),
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
     * Get list of conflicting packages during Docker migration for authenticated-user.
     *
     * This function performs a `GET` to the `/user/docker/conflicts` endpoint.
     *
     * Lists all packages that are owned by the authenticated user within the user's namespace, and that encountered a conflict during a Docker migration.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/packages/packages#get-list-of-conflicting-packages-during-docker-migration-for-authenticated-user>
     */
    pub async fn list_docker_migration_conflicting_packages_for_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::PackageData>>> {
        let url = self.client.url(&"/user/docker/conflicts".to_string(), None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get list of conflicting packages during Docker migration for authenticated-user.
     *
     * This function performs a `GET` to the `/user/docker/conflicts` endpoint.
     *
     * As opposed to `list_docker_migration_conflicting_packages_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * Lists all packages that are owned by the authenticated user within the user's namespace, and that encountered a conflict during a Docker migration.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/packages/packages#get-list-of-conflicting-packages-during-docker-migration-for-authenticated-user>
     */
    pub async fn list_all_docker_migration_conflicting_packages_for_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::PackageData>>> {
        let url = self.client.url(&"/user/docker/conflicts".to_string(), None);
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
     * List packages for the authenticated user's namespace.
     *
     * This function performs a `GET` to the `/user/packages` endpoint.
     *
     * Lists packages owned by the authenticated user within the user's namespace.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#list-packages-for-the-authenticated-users-namespace>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `visibility: crate::types::PackageVisibilityData` -- The selected visibility of the packages.  This parameter is optional and only filters an existing result set.
     *  
     *  The `internal` visibility is only supported for GitHub Packages registries that allow for granular permissions. For other ecosystems `internal` is synonymous with `private`.
     *  For the list of GitHub Packages registries that support granular permissions, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_packages_for_authenticated_user(
        &self,
        package_type: crate::types::PackageType,
        visibility: crate::types::PackageVisibilityData,
        page: i64,
        per_page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::PackageData>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !package_type.to_string().is_empty() {
            query_args.push(("package_type".to_string(), package_type.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !visibility.to_string().is_empty() {
            query_args.push(("visibility".to_string(), visibility.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/user/packages?{}", query_), None);
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * List packages for the authenticated user's namespace.
     *
     * This function performs a `GET` to the `/user/packages` endpoint.
     *
     * As opposed to `list_packages_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * Lists packages owned by the authenticated user within the user's namespace.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#list-packages-for-the-authenticated-users-namespace>
     */
    pub async fn list_all_packages_for_authenticated_user(
        &self,
        package_type: crate::types::PackageType,
        visibility: crate::types::PackageVisibilityData,
    ) -> ClientResult<crate::Response<Vec<crate::types::PackageData>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !package_type.to_string().is_empty() {
            query_args.push(("package_type".to_string(), package_type.to_string()));
        }
        if !visibility.to_string().is_empty() {
            query_args.push(("visibility".to_string(), visibility.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/user/packages?{}", query_), None);
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
     * Get a package for the authenticated user.
     *
     * This function performs a `GET` to the `/user/packages/{package_type}/{package_name}` endpoint.
     *
     * Gets a specific package for a package owned by the authenticated user.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#get-a-package-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     */
    pub async fn get_package_for_authenticated_user(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
    ) -> ClientResult<crate::Response<crate::types::PackageData>> {
        let url = self.client.url(
            &format!(
                "/user/packages/{}/{}",
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Delete a package for the authenticated user.
     *
     * This function performs a `DELETE` to the `/user/packages/{package_type}/{package_name}` endpoint.
     *
     * Deletes a package owned by the authenticated user. You cannot delete a public package if any version of the package has more than 5,000 downloads. In this scenario, contact GitHub support for further assistance.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` and `delete:packages` scopes to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#delete-a-package-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     */
    pub async fn delete_package_for_authenticated_user(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/packages/{}/{}",
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
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
     * Restore a package for the authenticated user.
     *
     * This function performs a `POST` to the `/user/packages/{package_type}/{package_name}/restore` endpoint.
     *
     * Restores a package owned by the authenticated user.
     *
     * You can restore a deleted package under the following conditions:
     *   - The package was deleted within the last 30 days.
     *   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` and `write:packages` scopes to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#restore-a-package-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `token: &str` -- package token.
     */
    pub async fn restore_package_for_authenticated_user(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        token: &str,
    ) -> ClientResult<crate::Response<()>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !token.is_empty() {
            query_args.push(("token".to_string(), token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/user/packages/{}/{}/restore?{}",
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
                query_
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
     * List package versions for a package owned by the authenticated user.
     *
     * This function performs a `GET` to the `/user/packages/{package_type}/{package_name}/versions` endpoint.
     *
     * Lists package versions for a package owned by the authenticated user.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#list-package-versions-for-a-package-owned-by-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `state: crate::types::PackagesGetAllPackageVersionsOwnedByOrgState` -- The state of the package, either active or deleted.
     */
    pub async fn get_all_package_versions_for_package_owned_by_authenticated_user(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        page: i64,
        per_page: i64,
        state: crate::types::PackagesGetAllPackageVersionsOwnedByOrgState,
    ) -> ClientResult<crate::Response<Vec<crate::types::PackageVersion>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/user/packages/{}/{}/versions?{}",
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
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
     * List package versions for a package owned by the authenticated user.
     *
     * This function performs a `GET` to the `/user/packages/{package_type}/{package_name}/versions` endpoint.
     *
     * As opposed to `get_all_package_versions_for_package_owned_by_authenticated_user`, this function returns all the pages of the request at once.
     *
     * Lists package versions for a package owned by the authenticated user.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#list-package-versions-for-a-package-owned-by-the-authenticated-user>
     */
    pub async fn get_all_all_package_versions_for_package_owned_by_authenticated_user(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        state: crate::types::PackagesGetAllPackageVersionsOwnedByOrgState,
    ) -> ClientResult<crate::Response<Vec<crate::types::PackageVersion>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !state.to_string().is_empty() {
            query_args.push(("state".to_string(), state.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/user/packages/{}/{}/versions?{}",
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
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
     * Get a package version for the authenticated user.
     *
     * This function performs a `GET` to the `/user/packages/{package_type}/{package_name}/versions/{package_version_id}` endpoint.
     *
     * Gets a specific package version for a package owned by the authenticated user.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#get-a-package-version-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `package_version_id: i64` -- Unique identifier of the package version.
     */
    pub async fn get_package_version_for_authenticated_user(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        package_version_id: i64,
    ) -> ClientResult<crate::Response<crate::types::PackageVersion>> {
        let url = self.client.url(
            &format!(
                "/user/packages/{}/{}/versions/{}",
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
                crate::progenitor_support::encode_path(&package_version_id.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Delete a package version for the authenticated user.
     *
     * This function performs a `DELETE` to the `/user/packages/{package_type}/{package_name}/versions/{package_version_id}` endpoint.
     *
     * Deletes a specific package version for a package owned by the authenticated user.  If the package is public and the package version has more than 5,000 downloads, you cannot delete the package version. In this scenario, contact GitHub support for further assistance.
     *
     * The authenticated user must have admin permissions in the organization to use this endpoint.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` and `delete:packages` scopes to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#delete-a-package-version-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `package_version_id: i64` -- Unique identifier of the package version.
     */
    pub async fn delete_package_version_for_authenticated_user(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        package_version_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/packages/{}/{}/versions/{}",
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
                crate::progenitor_support::encode_path(&package_version_id.to_string()),
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
     * Restore a package version for the authenticated user.
     *
     * This function performs a `POST` to the `/user/packages/{package_type}/{package_name}/versions/{package_version_id}/restore` endpoint.
     *
     * Restores a package version owned by the authenticated user.
     *
     * You can restore a deleted package version under the following conditions:
     *   - The package was deleted within the last 30 days.
     *   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` and `write:packages` scopes to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#restore-a-package-version-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `package_version_id: i64` -- Unique identifier of the package version.
     */
    pub async fn restore_package_version_for_authenticated_user(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        package_version_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/packages/{}/{}/versions/{}/restore",
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
                crate::progenitor_support::encode_path(&package_version_id.to_string()),
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
     * Get list of conflicting packages during Docker migration for user.
     *
     * This function performs a `GET` to the `/users/{username}/docker/conflicts` endpoint.
     *
     * Lists all packages that are in a specific user's namespace, that the requesting user has access to, and that encountered a conflict during Docker migration.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/packages/packages#get-list-of-conflicting-packages-during-docker-migration-for-user>
     *
     * **Parameters:**
     *
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn list_docker_migration_conflicting_packages_for_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::PackageData>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/docker/conflicts",
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
     * Get list of conflicting packages during Docker migration for user.
     *
     * This function performs a `GET` to the `/users/{username}/docker/conflicts` endpoint.
     *
     * As opposed to `list_docker_migration_conflicting_packages_for_user`, this function returns all the pages of the request at once.
     *
     * Lists all packages that are in a specific user's namespace, that the requesting user has access to, and that encountered a conflict during Docker migration.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/packages/packages#get-list-of-conflicting-packages-during-docker-migration-for-user>
     */
    pub async fn list_all_docker_migration_conflicting_packages_for_user(
        &self,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::PackageData>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/docker/conflicts",
                crate::progenitor_support::encode_path(&username.to_string()),
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
     * List packages for a user.
     *
     * This function performs a `GET` to the `/users/{username}/packages` endpoint.
     *
     * Lists all packages in a user's namespace for which the requesting user has access.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#list-packages-for-a-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `visibility: crate::types::PackageVisibilityData` -- The selected visibility of the packages.  This parameter is optional and only filters an existing result set.
     *  
     *  The `internal` visibility is only supported for GitHub Packages registries that allow for granular permissions. For other ecosystems `internal` is synonymous with `private`.
     *  For the list of GitHub Packages registries that support granular permissions, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages).".
     * * `username: &str` -- The handle for the GitHub user account.
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn list_packages_for_user(
        &self,
        package_type: crate::types::PackageType,
        visibility: crate::types::PackageVisibilityData,
        username: &str,
        page: i64,
        per_page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::PackageData>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !package_type.to_string().is_empty() {
            query_args.push(("package_type".to_string(), package_type.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        if !visibility.to_string().is_empty() {
            query_args.push(("visibility".to_string(), visibility.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/packages?{}",
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
     * List packages for a user.
     *
     * This function performs a `GET` to the `/users/{username}/packages` endpoint.
     *
     * As opposed to `list_packages_for_user`, this function returns all the pages of the request at once.
     *
     * Lists all packages in a user's namespace for which the requesting user has access.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#list-packages-for-a-user>
     */
    pub async fn list_all_packages_for_user(
        &self,
        package_type: crate::types::PackageType,
        visibility: crate::types::PackageVisibilityData,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::PackageData>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !package_type.to_string().is_empty() {
            query_args.push(("package_type".to_string(), package_type.to_string()));
        }
        if !visibility.to_string().is_empty() {
            query_args.push(("visibility".to_string(), visibility.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/packages?{}",
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
    /**
     * Get a package for a user.
     *
     * This function performs a `GET` to the `/users/{username}/packages/{package_type}/{package_name}` endpoint.
     *
     * Gets a specific package metadata for a public package owned by a user.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#get-a-package-for-a-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn get_package_for_user(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::PackageData>> {
        let url = self.client.url(
            &format!(
                "/users/{}/packages/{}/{}",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Delete a package for a user.
     *
     * This function performs a `DELETE` to the `/users/{username}/packages/{package_type}/{package_name}` endpoint.
     *
     * Deletes an entire package for a user. You cannot delete a public package if any version of the package has more than 5,000 downloads. In this scenario, contact GitHub support for further assistance.
     *
     * If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must have admin permissions to the package. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` and `delete:packages` scopes to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#delete-a-package-for-a-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn delete_package_for_user(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        username: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/users/{}/packages/{}/{}",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
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
     * Restore a package for a user.
     *
     * This function performs a `POST` to the `/users/{username}/packages/{package_type}/{package_name}/restore` endpoint.
     *
     * Restores an entire package for a user.
     *
     * You can restore a deleted package under the following conditions:
     *   - The package was deleted within the last 30 days.
     *   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.
     *
     * If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must have admin permissions to the package. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` and `write:packages` scopes to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#restore-a-package-for-a-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `username: &str` -- The handle for the GitHub user account.
     * * `token: &str` -- package token.
     */
    pub async fn restore_package_for_user(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        username: &str,
        token: &str,
    ) -> ClientResult<crate::Response<()>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !token.is_empty() {
            query_args.push(("token".to_string(), token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/users/{}/packages/{}/{}/restore?{}",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
                query_
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
     * List package versions for a package owned by a user.
     *
     * This function performs a `GET` to the `/users/{username}/packages/{package_type}/{package_name}/versions` endpoint.
     *
     * Lists package versions for a public package owned by a specified user.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#list-package-versions-for-a-package-owned-by-a-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn get_all_package_versions_for_package_owned_by_user(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::PackageVersion>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/packages/{}/{}/versions",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * List package versions for a package owned by a user.
     *
     * This function performs a `GET` to the `/users/{username}/packages/{package_type}/{package_name}/versions` endpoint.
     *
     * As opposed to `get_all_package_versions_for_package_owned_by_user`, this function returns all the pages of the request at once.
     *
     * Lists package versions for a public package owned by a specified user.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#list-package-versions-for-a-package-owned-by-a-user>
     */
    pub async fn get_all_all_package_versions_for_package_owned_by_user(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        username: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::PackageVersion>>> {
        let url = self.client.url(
            &format!(
                "/users/{}/packages/{}/{}/versions",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
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
     * Get a package version for a user.
     *
     * This function performs a `GET` to the `/users/{username}/packages/{package_type}/{package_name}/versions/{package_version_id}` endpoint.
     *
     * Gets a specific package version for a public package owned by a specified user.
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` scope to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#get-a-package-version-for-a-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `package_version_id: i64` -- Unique identifier of the package version.
     * * `username: &str` -- The handle for the GitHub user account.
     */
    pub async fn get_package_version_for_user(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        package_version_id: i64,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::PackageVersion>> {
        let url = self.client.url(
            &format!(
                "/users/{}/packages/{}/{}/versions/{}",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
                crate::progenitor_support::encode_path(&package_version_id.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Delete package version for a user.
     *
     * This function performs a `DELETE` to the `/users/{username}/packages/{package_type}/{package_name}/versions/{package_version_id}` endpoint.
     *
     * Deletes a specific package version for a user. If the package is public and the package version has more than 5,000 downloads, you cannot delete the package version. In this scenario, contact GitHub support for further assistance.
     *
     * If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must have admin permissions to the package. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` and `delete:packages` scopes to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#delete-package-version-for-a-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `username: &str` -- The handle for the GitHub user account.
     * * `package_version_id: i64` -- Unique identifier of the package version.
     */
    pub async fn delete_package_version_for_user(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        username: &str,
        package_version_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/users/{}/packages/{}/{}/versions/{}",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
                crate::progenitor_support::encode_path(&package_version_id.to_string()),
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
     * Restore package version for a user.
     *
     * This function performs a `POST` to the `/users/{username}/packages/{package_type}/{package_name}/versions/{package_version_id}/restore` endpoint.
     *
     * Restores a specific package version for a user.
     *
     * You can restore a deleted package under the following conditions:
     *   - The package was deleted within the last 30 days.
     *   - The same package namespace and version is still available and not reused for a new package. If the same package namespace is not available, you will not be able to restore your package. In this scenario, to restore the deleted package, you must delete the new package that uses the deleted package's namespace first.
     *
     * If the `package_type` belongs to a GitHub Packages registry that supports granular permissions, the authenticated user must have admin permissions to the package. For the list of these registries, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#granular-permissions-for-userorganization-scoped-packages)."
     *
     * OAuth app tokens and personal access tokens (classic) need the `read:packages` and `write:packages` scopes to use this endpoint. For more information, see "[About permissions for GitHub Packages](https://docs.github.com/packages/learn-github-packages/about-permissions-for-github-packages#permissions-for-repository-scoped-packages)."
     *
     * FROM: <https://docs.github.com/rest/packages/packages#restore-package-version-for-a-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `username: &str` -- The handle for the GitHub user account.
     * * `package_version_id: i64` -- Unique identifier of the package version.
     */
    pub async fn restore_package_version_for_user(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        username: &str,
        package_version_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/users/{}/packages/{}/{}/versions/{}/restore",
                crate::progenitor_support::encode_path(&username.to_string()),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(&package_name.to_string()),
                crate::progenitor_support::encode_path(&package_version_id.to_string()),
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
