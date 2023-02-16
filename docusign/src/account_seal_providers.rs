use anyhow::Result;

use crate::Client;

pub struct AccountSealProviders {
    pub client: Client,
}

impl AccountSealProviders {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AccountSealProviders { client }
    }

    /**
     * Returns available seals for specified account.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/seals` endpoint.
     *
     *
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn account_signature_providers_get_seal(
        &self,
        account_id: &str,
    ) -> Result<crate::types::AccountSealProviders> {
        let url = format!(
            "/v2.1/accounts/{}/seals",
            crate::progenitor_support::encode_path(&account_id.to_string()),
        );
        let url = self.client.url(&url, None);
        self.client.get(&url, None).await
    }
}
