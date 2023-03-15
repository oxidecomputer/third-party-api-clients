use crate::Client;
use crate::ClientResult;

pub struct ENoteConfigurations {
    pub client: Client,
}

impl ENoteConfigurations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ENoteConfigurations { client }
    }

    /**
     * Returns the configuration information for the eNote eOriginal integration.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/settings/enote_configuration` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn get(&self, account_id: &str) -> ClientResult<crate::types::ENoteConfiguration> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/settings/enote_configuration",
                crate::progenitor_support::encode_path(account_id),
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
     * Updates configuration information for the eNote eOriginal integration.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/settings/enote_configuration` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn put(
        &self,
        account_id: &str,
        body: &crate::types::ENoteConfiguration,
    ) -> ClientResult<crate::types::ENoteConfiguration> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/settings/enote_configuration",
                crate::progenitor_support::encode_path(account_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Deletes configuration information for the eNote eOriginal integration.
     *
     * This function performs a `DELETE` to the `/v2.1/accounts/{accountId}/settings/enote_configuration` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn delete(&self, account_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/settings/enote_configuration",
                crate::progenitor_support::encode_path(account_id),
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
}
