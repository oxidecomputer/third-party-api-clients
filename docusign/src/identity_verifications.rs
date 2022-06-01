use anyhow::Result;

use crate::Client;

pub struct IdentityVerifications {
    pub client: Client,
}

impl IdentityVerifications {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        IdentityVerifications { client }
    }

    /**
    * Retrieves the Identity Verification workflows available to an account.
    *
    * This function performs a `GET` to the `/v2.1/accounts/{accountId}/identity_verification` endpoint.
    *
    * This method returns a list of Identity Verification workflows that are available to an account.
    *
    * **Note**: To use this method, you must either be an account administrator or a sender.
    *
    * **Parameters:**
    *
    * * `account_id: &str` -- The brand that envelope recipients see when a brand is not explicitly set.
    */
    pub async fn account_get(
        &self,
        account_id: &str,
    ) -> Result<crate::types::AccountIdentityVerificationResponse> {
        let url = format!(
            "/v2.1/accounts/{}/identity_verification",
            crate::progenitor_support::encode_path(account_id),
        );

        self.client.get(&url, None).await
    }
}
