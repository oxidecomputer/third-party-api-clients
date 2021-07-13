use anyhow::Result;

use crate::Client;

pub struct Licenses {
    client: Client,
}

impl Licenses {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Licenses { client }
    }

    /**
     * Get the license for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/license` endpoint.
     *
     * This method returns the contents of the repository's license file, if one is detected.
     *
     * Similar to [Get repository content](https://docs.github.com/rest/reference/repos#get-repository-content), this method also supports [custom media types](https://docs.github.com/rest/overview/media-types) for retrieving the raw license content or rendered license HTML.
     *
     * FROM: <https://docs.github.com/rest/reference/licenses/#get-the-license-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get_for_repo(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<crate::types::LicenseContent> {
        let url = format!(
            "/repos/{}/{}/license",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
        );

        self.client.get(&url).await
    }
}
