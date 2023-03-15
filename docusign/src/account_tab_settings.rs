use crate::Client;
use crate::ClientResult;

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
    ) -> ClientResult<crate::types::TabAccountSettings> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/settings/tabs",
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
    ) -> ClientResult<crate::types::TabAccountSettings> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/settings/tabs",
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
}
