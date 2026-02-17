use crate::Client;
use crate::ClientResult;

pub struct Meta {
    pub client: Client,
}

impl Meta {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Meta { client }
    }

    /**
     * GitHub API Root.
     *
     * This function performs a `GET` to the `/` endpoint.
     *
     * Get Hypermedia links to resources accessible in GitHub's REST API
     *
     * FROM: <https://docs.github.com/rest/meta/meta#github-api-root>
     */
    pub async fn root(&self) -> ClientResult<crate::Response<crate::types::Root>> {
        let url = self.client.url("", None);
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
     * Get GitHub meta information.
     *
     * This function performs a `GET` to the `/meta` endpoint.
     *
     * Returns meta information about GitHub, including a list of GitHub's IP addresses. For more information, see "[About GitHub's IP addresses](https://docs.github.com/articles/about-github-s-ip-addresses/)."
     *
     * The API's response also includes a list of GitHub's domain names.
     *
     * The values shown in the documentation's response are example values. You must always query the API directly to get the latest values.
     *
     * > [!NOTE]
     * > This endpoint returns both IPv4 and IPv6 addresses. However, not all features support IPv6. You should refer to the specific documentation for each feature to determine if IPv6 is supported.
     *
     * FROM: <https://docs.github.com/rest/meta/meta#get-apiname-meta-information>
     */
    pub async fn get(&self) -> ClientResult<crate::Response<crate::types::ApiOverview>> {
        let url = self.client.url(&"/meta".to_string(), None);
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
     * Get Octocat.
     *
     * This function performs a `GET` to the `/octocat` endpoint.
     *
     * Get the octocat as ASCII art
     *
     * FROM: <https://docs.github.com/rest/meta/meta#get-octocat>
     *
     * **Parameters:**
     *
     * * `s: &str` -- The words to show in Octocat's speech bubble.
     */
    pub async fn get_octocat(&self, s: &str) -> ClientResult<crate::Response<String>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !s.is_empty() {
            query_args.push(("s".to_string(), s.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/octocat?{}", query_), None);
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
     * Get all API versions.
     *
     * This function performs a `GET` to the `/versions` endpoint.
     *
     * Get all supported GitHub API versions.
     *
     * FROM: <https://docs.github.com/rest/meta/meta#get-all-api-versions>
     */
    pub async fn get_all_versions(
        &self,
    ) -> ClientResult<crate::Response<Vec<Option<chrono::NaiveDate>>>> {
        let url = self.client.url(&"/versions".to_string(), None);
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
     * Get all API versions.
     *
     * This function performs a `GET` to the `/versions` endpoint.
     *
     * As opposed to `get_all_versions`, this function returns all the pages of the request at once.
     *
     * Get all supported GitHub API versions.
     *
     * FROM: <https://docs.github.com/rest/meta/meta#get-all-api-versions>
     */
    pub async fn get_all_all_versions(
        &self,
    ) -> ClientResult<crate::Response<Vec<Option<chrono::NaiveDate>>>> {
        let url = self.client.url(&"/versions".to_string(), None);
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
     * Get the Zen of GitHub.
     *
     * This function performs a `GET` to the `/zen` endpoint.
     *
     * Get a random sentence from the Zen of GitHub
     *
     * FROM: <https://docs.github.com/rest/meta/meta#get-the-zen-of-github>
     */
    pub async fn get_zen(&self) -> ClientResult<crate::Response<String>> {
        let url = self.client.url(&"/zen".to_string(), None);
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
