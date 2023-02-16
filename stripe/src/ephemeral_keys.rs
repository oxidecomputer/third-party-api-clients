use anyhow::Result;

use crate::Client;

pub struct EphemeralKeys {
    pub client: Client,
}

impl EphemeralKeys {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        EphemeralKeys { client }
    }

    /**
     * This function performs a `POST` to the `/v1/ephemeral_keys` endpoint.
     *
     * <p>Creates a short-lived API key for a given resource.</p>
     */
    pub async fn post(&self) -> Result<crate::types::EphemeralKey> {
        let url = "/v1/ephemeral_keys".to_string();
        let url = self.client.url(&url, None);
        self.client
            .post(&url, None, Some("application/x-www-form-urlencoded"))
            .await
    }
    /**
     * This function performs a `DELETE` to the `/v1/ephemeral_keys/{key}` endpoint.
     *
     * <p>Invalidates a short-lived API key for a given resource.</p>
     *
     * **Parameters:**
     *
     * * `key: &str` -- The account's country.
     */
    pub async fn delete_key(&self, key: &str) -> Result<crate::types::EphemeralKey> {
        let url = format!(
            "/v1/ephemeral_keys/{}",
            crate::progenitor_support::encode_path(key),
        );
        let url = self.client.url(&url, None);
        self.client
            .delete(&url, None, Some("application/x-www-form-urlencoded"))
            .await
    }
}
