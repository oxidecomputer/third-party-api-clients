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
     * Get a package for an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/packages/{package_type}/{package_name}` endpoint.
     *
     * Gets a specific package in an organization.
     *
     * To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
     * If `package_type` is not `container`, your token must also include the `repo` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/packages#get-a-package-for-an-organization>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Can be one of `npm`, `maven`, `rubygems`, `nuget`, `docker`, or `container`. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `org: &str`
     */
    pub async fn get_package_for_organization(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        org: &str,
    ) -> ClientResult<crate::Response<crate::types::Package>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/packages/{}/{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(package_name),
            ),
            None,
        );
        self.client
            .get(
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
     * To use this endpoint, you must have admin permissions in the organization and authenticate using an access token with the `packages:read` and `packages:delete` scopes. In addition:
     * - If `package_type` is not `container`, your token must also include the `repo` scope.
     * - If `package_type` is `container`, you must also have admin permissions to the container you want to delete.
     *
     * FROM: <https://docs.github.com/rest/reference/packages#delete-a-package-for-an-organization>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Can be one of `npm`, `maven`, `rubygems`, `nuget`, `docker`, or `container`. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `org: &str`
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
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(package_name),
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
     * To use this endpoint, you must have admin permissions in the organization and authenticate using an access token with the `packages:read` and `packages:write` scopes. In addition:
     * - If `package_type` is not `container`, your token must also include the `repo` scope.
     * - If `package_type` is `container`, you must also have admin permissions to the container that you want to restore.
     *
     * FROM: <https://docs.github.com/rest/reference/packages#restore-a-package-for-an-organization>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Can be one of `npm`, `maven`, `rubygems`, `nuget`, `docker`, or `container`. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `org: &str`
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
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(package_name),
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
     * Get all package versions for a package owned by an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/packages/{package_type}/{package_name}/versions` endpoint.
     *
     * Returns all package versions for a package owned by an organization.
     *
     * To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
     * If `package_type` is not `container`, your token must also include the `repo` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/packages#get-all-package-versions-for-a-package-owned-by-an-organization>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Can be one of `npm`, `maven`, `rubygems`, `nuget`, `docker`, or `container`. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `org: &str`
     * * `page: i64` -- Page number of the results to fetch.
     * * `per_page: i64` -- Results per page (max 100).
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
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(package_name),
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
     * Get all package versions for a package owned by an organization.
     *
     * This function performs a `GET` to the `/orgs/{org}/packages/{package_type}/{package_name}/versions` endpoint.
     *
     * As opposed to `get_all_package_versions_for_package_owned_by_org`, this function returns all the pages of the request at once.
     *
     * Returns all package versions for a package owned by an organization.
     *
     * To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
     * If `package_type` is not `container`, your token must also include the `repo` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/packages#get-all-package-versions-for-a-package-owned-by-an-organization>
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
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(package_name),
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
     * You must authenticate using an access token with the `packages:read` scope.
     * If `package_type` is not `container`, your token must also include the `repo` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/packages#get-a-package-version-for-an-organization>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Can be one of `npm`, `maven`, `rubygems`, `nuget`, `docker`, or `container`. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `org: &str`
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
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(package_name),
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
     * To use this endpoint, you must have admin permissions in the organization and authenticate using an access token with the `packages:read` and `packages:delete` scopes. In addition:
     * - If `package_type` is not `container`, your token must also include the `repo` scope.
     * - If `package_type` is `container`, you must also have admin permissions to the container you want to delete.
     *
     * FROM: <https://docs.github.com/rest/reference/packages#delete-a-package-version-for-an-organization>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Can be one of `npm`, `maven`, `rubygems`, `nuget`, `docker`, or `container`. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `org: &str`
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
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(package_name),
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
     * To use this endpoint, you must have admin permissions in the organization and authenticate using an access token with the `packages:read` and `packages:write` scopes. In addition:
     * - If `package_type` is not `container`, your token must also include the `repo` scope.
     * - If `package_type` is `container`, you must also have admin permissions to the container that you want to restore.
     *
     * FROM: <https://docs.github.com/rest/reference/packages#restore-a-package-version-for-an-organization>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Can be one of `npm`, `maven`, `rubygems`, `nuget`, `docker`, or `container`. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `org: &str`
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
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(package_name),
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
     * Get a package for the authenticated user.
     *
     * This function performs a `GET` to the `/user/packages/{package_type}/{package_name}` endpoint.
     *
     * Gets a specific package for a package owned by the authenticated user.
     *
     * To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
     * If `package_type` is not `container`, your token must also include the `repo` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/packages#get-a-package-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Can be one of `npm`, `maven`, `rubygems`, `nuget`, `docker`, or `container`. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     */
    pub async fn get_package_for_authenticated_user(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
    ) -> ClientResult<crate::Response<crate::types::Package>> {
        let url = self.client.url(
            &format!(
                "/user/packages/{}/{}",
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(package_name),
            ),
            None,
        );
        self.client
            .get(
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
     * To use this endpoint, you must authenticate using an access token with the `packages:read` and `packages:delete` scopes.
     * If `package_type` is not `container`, your token must also include the `repo` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/packages#delete-a-package-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Can be one of `npm`, `maven`, `rubygems`, `nuget`, `docker`, or `container`. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
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
                crate::progenitor_support::encode_path(package_name),
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
     * To use this endpoint, you must authenticate using an access token with the `packages:read` and `packages:write` scopes. If `package_type` is not `container`, your token must also include the `repo` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/packages#restore-a-package-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Can be one of `npm`, `maven`, `rubygems`, `nuget`, `docker`, or `container`. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
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
                crate::progenitor_support::encode_path(package_name),
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
     * Get all package versions for a package owned by the authenticated user.
     *
     * This function performs a `GET` to the `/user/packages/{package_type}/{package_name}/versions` endpoint.
     *
     * Returns all package versions for a package owned by the authenticated user.
     *
     * To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
     * If `package_type` is not `container`, your token must also include the `repo` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/packages#get-all-package-versions-for-a-package-owned-by-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Can be one of `npm`, `maven`, `rubygems`, `nuget`, `docker`, or `container`. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `page: i64` -- Page number of the results to fetch.
     * * `per_page: i64` -- Results per page (max 100).
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
                crate::progenitor_support::encode_path(package_name),
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
     * Get all package versions for a package owned by the authenticated user.
     *
     * This function performs a `GET` to the `/user/packages/{package_type}/{package_name}/versions` endpoint.
     *
     * As opposed to `get_all_package_versions_for_package_owned_by_authenticated_user`, this function returns all the pages of the request at once.
     *
     * Returns all package versions for a package owned by the authenticated user.
     *
     * To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
     * If `package_type` is not `container`, your token must also include the `repo` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/packages#get-all-package-versions-for-a-package-owned-by-the-authenticated-user>
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
                crate::progenitor_support::encode_path(package_name),
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
     * To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
     * If `package_type` is not `container`, your token must also include the `repo` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/packages#get-a-package-version-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Can be one of `npm`, `maven`, `rubygems`, `nuget`, `docker`, or `container`. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
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
                crate::progenitor_support::encode_path(package_name),
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
     * To use this endpoint, you must have admin permissions in the organization and authenticate using an access token with the `packages:read` and `packages:delete` scopes.
     * If `package_type` is not `container`, your token must also include the `repo` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/packages#delete-a-package-version-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Can be one of `npm`, `maven`, `rubygems`, `nuget`, `docker`, or `container`. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
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
                crate::progenitor_support::encode_path(package_name),
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
     * To use this endpoint, you must authenticate using an access token with the `packages:read` and `packages:write` scopes. If `package_type` is not `container`, your token must also include the `repo` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/packages#restore-a-package-version-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Can be one of `npm`, `maven`, `rubygems`, `nuget`, `docker`, or `container`. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
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
                crate::progenitor_support::encode_path(package_name),
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
     * Get a package for a user.
     *
     * This function performs a `GET` to the `/users/{username}/packages/{package_type}/{package_name}` endpoint.
     *
     * Gets a specific package metadata for a public package owned by a user.
     *
     * To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
     * If `package_type` is not `container`, your token must also include the `repo` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/packages#get-a-package-for-a-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Can be one of `npm`, `maven`, `rubygems`, `nuget`, `docker`, or `container`. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `username: &str`
     */
    pub async fn get_package_for_user(
        &self,
        package_type: crate::types::PackageType,
        package_name: &str,
        username: &str,
    ) -> ClientResult<crate::Response<crate::types::Package>> {
        let url = self.client.url(
            &format!(
                "/users/{}/packages/{}/{}",
                crate::progenitor_support::encode_path(username),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(package_name),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get all package versions for a package owned by a user.
     *
     * This function performs a `GET` to the `/users/{username}/packages/{package_type}/{package_name}/versions` endpoint.
     *
     * Returns all package versions for a public package owned by a specified user.
     *
     * To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
     * If `package_type` is not `container`, your token must also include the `repo` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/packages#get-all-package-versions-for-a-package-owned-by-a-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Can be one of `npm`, `maven`, `rubygems`, `nuget`, `docker`, or `container`. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `username: &str`
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
                crate::progenitor_support::encode_path(username),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(package_name),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get all package versions for a package owned by a user.
     *
     * This function performs a `GET` to the `/users/{username}/packages/{package_type}/{package_name}/versions` endpoint.
     *
     * As opposed to `get_all_package_versions_for_package_owned_by_user`, this function returns all the pages of the request at once.
     *
     * Returns all package versions for a public package owned by a specified user.
     *
     * To use this endpoint, you must authenticate using an access token with the `packages:read` scope.
     * If `package_type` is not `container`, your token must also include the `repo` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/packages#get-all-package-versions-for-a-package-owned-by-a-user>
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
                crate::progenitor_support::encode_path(username),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(package_name),
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
     * At this time, to use this endpoint, you must authenticate using an access token with the `packages:read` scope.
     * If `package_type` is not `container`, your token must also include the `repo` scope.
     *
     * FROM: <https://docs.github.com/rest/reference/packages#get-a-package-version-for-a-user>
     *
     * **Parameters:**
     *
     * * `package_type: crate::types::PackageType` -- The type of supported package. Can be one of `npm`, `maven`, `rubygems`, `nuget`, `docker`, or `container`. Packages in GitHub's Gradle registry have the type `maven`. Docker images pushed to GitHub's Container registry (`ghcr.io`) have the type `container`. You can use the type `docker` to find images that were pushed to GitHub's Docker registry (`docker.pkg.github.com`), even if these have now been migrated to the Container registry.
     * * `package_name: &str` -- The name of the package.
     * * `package_version_id: i64` -- Unique identifier of the package version.
     * * `username: &str`
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
                crate::progenitor_support::encode_path(username),
                crate::progenitor_support::encode_path(&package_type.to_string()),
                crate::progenitor_support::encode_path(package_name),
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
}
