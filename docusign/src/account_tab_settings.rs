use anyhow::Result;

use crate::Client;

pub struct AccountTabSettings {
    pub client: Client,
}

impl AccountTabSettings {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AccountTabSettings { client }
    }

    /**
     * Returns tab settings list for specified account.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/settings/tabs` endpoint.
     *
     * This method returns information about the tab types and tab functionality that is currently enabled for an account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn tab_settings_get(
        &self,
        account_id: &str,
    ) -> Result<crate::types::TabAccountSettings> {
        let url = format!(
            "/v2.1/accounts/{}/settings/tabs",
            crate::progenitor_support::encode_path(account_id),
        );
        let url = self.client.url(&url, None);
        self.client.get(&url, None, None).await
    }
    /**
     * Modifies tab settings for specified account.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/settings/tabs` endpoint.
     *
     * This method modifies the tab types and tab functionality that is enabled for an account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn tab_settings_put(
        &self,
        account_id: &str,
        body: &crate::types::TabAccountSettings,
    ) -> Result<crate::types::TabAccountSettings> {
        let url = format!(
            "/v2.1/accounts/{}/settings/tabs",
            crate::progenitor_support::encode_path(account_id),
        );
        let url = self.client.url(&url, None);
        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                Some("application/json"),
            )
            .await
    }
}
