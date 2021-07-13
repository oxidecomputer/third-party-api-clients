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
     * Get all commonly used licenses.
     *
     * This function performs a `GET` to the `/licenses` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/licenses#get-all-commonly-used-licenses>
     *
     * **Parameters:**
     *
     * * `featured: bool`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn get_all_commonly_used(
        &self,
        featured: bool,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::License>> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if featured {
            query_args.push(format!("featured={}", featured));
        }
        if page > 0 {
            query_args.push(format!("page={}", page));
        }
        if per_page > 0 {
            query_args.push(format!("per_page={}", per_page));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/licenses?{}", query);

        self.client.get(&url).await
    }

    /**
     * Get all commonly used licenses.
     *
     * This function performs a `GET` to the `/licenses` endpoint.
     *
     * As opposed to `get_all_commonly_used`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/licenses#get-all-commonly-used-licenses>
     */
    pub async fn get_all_all_commonly_used(
        &self,
        featured: bool,
    ) -> Result<Vec<crate::types::License>> {
        let mut query = String::new();
        let mut query_args: Vec<String> = Default::default();
        if featured {
            query_args.push(format!("featured={}", featured));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query.push('&');
            }
            query.push_str(n);
        }
        let url = format!("/licenses?{}", query);

        self.client.get_all_pages(&url).await
    }

    /**
     * Get a license.
     *
     * This function performs a `GET` to the `/licenses/{license}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/licenses#get-a-license>
     *
     * **Parameters:**
     *
     * * `license: &str`
     */
    pub async fn get(&self, license: &str) -> Result<crate::types::License> {
        let url = format!(
            "/licenses/{}",
            crate::progenitor_support::encode_path(&license.to_string()),
        );

        self.client.get(&url).await
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
