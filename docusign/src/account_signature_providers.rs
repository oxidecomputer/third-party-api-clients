use crate::Client;
use crate::ClientResult;

pub struct AccountSignatureProviders {
    pub client: Client,
}

impl AccountSignatureProviders {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AccountSignatureProviders { client }
    }

    /**
     * Gets the available signature providers for an account.
     *
     * This function performs a `GET` to the `/v2.1/accounts/{accountId}/signatureProviders` endpoint.
     *
     * Returns a list of signature providers that the specified account can use.
     *
     * **Parameters:**
     *
     * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
     */
    pub async fn get(
        &self,
        account_id: &str,
    ) -> ClientResult<crate::types::AccountSignatureProvidersData> {
        let url = self.client.url(
            &format!(
                "/v2.1/accounts/{}/signatureProviders",
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
