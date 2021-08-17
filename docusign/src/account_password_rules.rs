use anyhow::Result;

use crate::Client;

pub struct AccountPasswordRules {
    pub client: Client,
}

impl AccountPasswordRules {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AccountPasswordRules { client }
    }

    /**
     * Gets the password rules for an account.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/settings/password_rules` endpoint.
     *
     * This method retrieves the password rules for an account.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn get(&self, account_id: &str) -> Result<crate::types::AccountPasswordRulesData> {
        let url = format!(
            "/v2.1/accounts/{}/settings/password_rules",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Updates the password rules for an account.
     *
     * This function performs a `PUT` to the `/v2.1/accounts/{accountId}/settings/password_rules` endpoint.
     *
     * This method updates the password rules for an account.
     *
     * **Note**: To update the password rules for an account, you must be an account administrator.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn put(
        &self,
        account_id: &str,
        body: &crate::types::AccountPasswordRulesData,
    ) -> Result<crate::types::AccountPasswordRulesData> {
        let url = format!(
            "/v2.1/accounts/{}/settings/password_rules",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Gets membership account password rules.
     *
     * This function performs a `GET` to the `/v2.1/current_user/password_rules` endpoint.
     *
     *
     */
    pub async fn password_rules_get(&self) -> Result<crate::types::UserPasswordRules> {
        let url = "/v2.1/current_user/password_rules".to_string();
        self.client.get(&url, None).await
    }
}
