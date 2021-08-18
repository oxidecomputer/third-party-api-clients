use anyhow::Result;

use crate::Client;

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
    pub async fn e_note_configuration_get(
        &self,
        account_id: &str,
    ) -> Result<crate::types::ENoteConfiguration> {
        let url = format!(
            "/v2.1/accounts/{}/settings/enote_configuration",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client.get(&url, None).await
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
    pub async fn e_note_configuration_put(
        &self,
        account_id: &str,
        body: &crate::types::ENoteConfiguration,
    ) -> Result<crate::types::ENoteConfiguration> {
        let url = format!(
            "/v2.1/accounts/{}/settings/enote_configuration",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
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
    pub async fn e_note_configuration_delete(&self, account_id: &str) -> Result<()> {
        let url = format!(
            "/v2.1/accounts/{}/settings/enote_configuration",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client.delete(&url, None).await
    }
}
