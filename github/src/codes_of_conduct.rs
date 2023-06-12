use crate::Client;
use crate::ClientResult;

pub struct CodesOfConduct {
    pub client: Client,
}

impl CodesOfConduct {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CodesOfConduct { client }
    }

    /**
     * Get all codes of conduct.
     *
     * This function performs a `GET` to the `/codes_of_conduct` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/codes-of-conduct#get-all-codes-of-conduct>
     */
    pub async fn get_all_codes_of_conduct(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeOfConduct>>> {
        let url = self.client.url("/codes_of_conduct", None);
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
     * Get all codes of conduct.
     *
     * This function performs a `GET` to the `/codes_of_conduct` endpoint.
     *
     * As opposed to `get_all_codes_of_conduct`, this function returns all the pages of the request at once.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/codes-of-conduct#get-all-codes-of-conduct>
     */
    pub async fn get_all_all_codes_of_conduct(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeOfConduct>>> {
        let url = self.client.url("/codes_of_conduct", None);
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
     * Get a code of conduct.
     *
     * This function performs a `GET` to the `/codes_of_conduct/{key}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/codes-of-conduct#get-a-code-of-conduct>
     *
     * **Parameters:**
     *
     * * `key: &str`
     */
    pub async fn get_conduct_code(
        &self,
        key: &str,
    ) -> ClientResult<crate::Response<crate::types::CodeOfConduct>> {
        let url = self.client.url(
            &format!(
                "/codes_of_conduct/{}",
                crate::progenitor_support::encode_path(key),
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
     * Get the code of conduct for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/community/code_of_conduct` endpoint.
     *
     * Returns the contents of the repository's code of conduct file, if one is detected.
     *
     * A code of conduct is detected if there is a file named `CODE_OF_CONDUCT` in the root directory of the repository. GitHub detects which code of conduct it is using fuzzy matching.
     *
     * FROM: <https://docs.github.com/rest/reference/codes-of-conduct#get-the-code-of-conduct-for-a-repository>
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
    ) -> ClientResult<crate::Response<crate::types::CodeOfConduct>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/community/code_of_conduct",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
