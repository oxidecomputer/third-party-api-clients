use anyhow::Result;

use crate::Client;

pub struct Packages {
    client: Client,
}

impl Packages {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Packages { client }
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
    ) -> Result<crate::types::PackageVersion> {
        let url = format!(
            "/users/{}/packages/{}/{}/versions/{}",
            crate::progenitor_support::encode_path(&username.to_string()),
            crate::progenitor_support::encode_path(&package_type.to_string()),
            crate::progenitor_support::encode_path(&package_name.to_string()),
            crate::progenitor_support::encode_path(&package_version_id.to_string()),
        );

        self.client.get(&url).await
    }
}
