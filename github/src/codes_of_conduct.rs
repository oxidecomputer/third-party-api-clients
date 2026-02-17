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
     * Returns array of all GitHub's codes of conduct.
     *
     * FROM: <https://docs.github.com/rest/codes-of-conduct/codes-of-conduct#get-all-codes-of-conduct>
     */
    pub async fn get_all_codes_of_conduct(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeOfConduct>>> {
        let url = self.client.url(&"/codes_of_conduct".to_string(), None);
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
     * Returns array of all GitHub's codes of conduct.
     *
     * FROM: <https://docs.github.com/rest/codes-of-conduct/codes-of-conduct#get-all-codes-of-conduct>
     */
    pub async fn get_all_all_codes_of_conduct(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::CodeOfConduct>>> {
        let url = self.client.url(&"/codes_of_conduct".to_string(), None);
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
     * Returns information about the specified GitHub code of conduct.
     *
     * FROM: <https://docs.github.com/rest/codes-of-conduct/codes-of-conduct#get-a-code-of-conduct>
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
                crate::progenitor_support::encode_path(&key.to_string()),
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
