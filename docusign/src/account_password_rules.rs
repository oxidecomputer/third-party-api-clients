use crate::Client;
use crate::ClientResult;

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
    pub async fn get(
        &self,
        account_id: &str,
    ) -> ClientResult<crate::types::AccountPasswordRulesData> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/settings/password_rules",
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
    ) -> ClientResult<crate::types::AccountPasswordRulesData> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/settings/password_rules",
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
     * Gets membership account password rules.
     *
     * This function performs a `GET` to the `/v2.1/current_user/password_rules` endpoint.
     *
     *
     */
    pub async fn password_rules_get(&self) -> ClientResult<crate::types::UserPasswordRules> {
        let url = self.client.url("/v2.1/current_user/password_rules", None);
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
