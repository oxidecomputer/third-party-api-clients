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
     * FROM: <https://docs.github.com/rest/overview/resources-in-the-rest-api#root-endpoint>
     */
    pub async fn root(&self) -> ClientResult<crate::Response<crate::types::MetaRootResponse>> {
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
     * Returns meta information about GitHub, including a list of GitHub's IP addresses. For more information, see "[About GitHub's IP addresses](https://help.github.com/articles/about-github-s-ip-addresses/)."
     *
     * **Note:** The IP addresses shown in the documentation's response are only example values. You must always query the API directly to get the latest list of IP addresses.
     *
     * FROM: <https://docs.github.com/rest/reference/meta#get-github-meta-information>
     */
    pub async fn get(&self) -> ClientResult<crate::Response<crate::types::ApiOverview>> {
        let url = self.client.url("/meta", None);
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
     * FROM: <https://docs.github.com/rest/reference/meta#get-octocat>
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
     * Get the Zen of GitHub.
     *
     * This function performs a `GET` to the `/zen` endpoint.
     *
     * Get a random sentence from the Zen of GitHub
     */
    pub async fn get_zen(&self) -> ClientResult<crate::Response<String>> {
        let url = self.client.url("/zen", None);
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
