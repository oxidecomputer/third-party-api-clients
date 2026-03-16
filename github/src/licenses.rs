use crate::Client;
use crate::ClientResult;

pub struct Licenses {
    pub client: Client,
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
     * Lists the most commonly used licenses on GitHub. For more information, see "[Licensing a repository ](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/licensing-a-repository)."
     *
     * FROM: <https://docs.github.com/rest/licenses/licenses#get-all-commonly-used-licenses>
     *
     * **Parameters:**
     *
     * * `featured: bool`
     * * `per_page: i64` -- The number of results per page (max 100). For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     * * `page: i64` -- The page number of the results to fetch. For more information, see "[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).".
     */
    pub async fn get_all_commonly_used(
        &self,
        featured: bool,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::LicenseSimpleData>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if featured {
            query_args.push(("featured".to_string(), featured.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/licenses?{}", query_), None);
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
     * Get all commonly used licenses.
     *
     * This function performs a `GET` to the `/licenses` endpoint.
     *
     * As opposed to `get_all_commonly_used`, this function returns all the pages of the request at once.
     *
     * Lists the most commonly used licenses on GitHub. For more information, see "[Licensing a repository ](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/licensing-a-repository)."
     *
     * FROM: <https://docs.github.com/rest/licenses/licenses#get-all-commonly-used-licenses>
     */
    pub async fn get_all_all_commonly_used(
        &self,
        featured: bool,
    ) -> ClientResult<crate::Response<Vec<crate::types::LicenseSimpleData>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if featured {
            query_args.push(("featured".to_string(), featured.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/licenses?{}", query_), None);
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
     * Get a license.
     *
     * This function performs a `GET` to the `/licenses/{license}` endpoint.
     *
     * Gets information about a specific license. For more information, see "[Licensing a repository ](https://docs.github.com/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/licensing-a-repository)."
     *
     * FROM: <https://docs.github.com/rest/licenses/licenses#get-a-license>
     *
     * **Parameters:**
     *
     * * `license: &str`
     */
    pub async fn get(&self, license: &str) -> ClientResult<crate::Response<crate::types::License>> {
        let url = self.client.url(
            &format!(
                "/licenses/{}",
                crate::progenitor_support::encode_path(&license.to_string()),
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
     * Get the license for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/license` endpoint.
     *
     * This method returns the contents of the repository's license file, if one is detected.
     *
     * This endpoint supports the following custom media types. For more information, see "[Media types](https://docs.github.com/rest/using-the-rest-api/getting-started-with-the-rest-api#media-types)."
     *
     * - **`application/vnd.github.raw+json`**: Returns the raw contents of the license.
     * - **`application/vnd.github.html+json`**: Returns the license contents in HTML. Markup languages are rendered to HTML using GitHub's open-source [Markup library](https://github.com/github/markup).
     *
     * FROM: <https://docs.github.com/rest/licenses/licenses#get-the-license-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `ref_: &str` -- The Git reference for the results you want to list. The `ref` for a branch can be formatted either as `refs/heads/<branch name>` or simply `<branch name>`. To reference a pull request use `refs/pull/<number>/merge`.
     */
    pub async fn get_for_repo(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> ClientResult<crate::Response<crate::types::LicenseContent>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !ref_.is_empty() {
            query_args.push(("ref".to_string(), ref_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/license?{}",
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
}
