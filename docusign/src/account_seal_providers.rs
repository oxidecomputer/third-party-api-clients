use crate::Client;
use crate::ClientResult;

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
    ) -> ClientResult<crate::types::AccountSealProviders> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/seals",
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
}
