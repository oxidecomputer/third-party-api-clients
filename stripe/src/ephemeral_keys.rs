use crate::Client;
use crate::ClientResult;

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
    pub async fn post(&self) -> ClientResult<crate::types::EphemeralKey> {
        let url = self.client.url("/v1/ephemeral_keys", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
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
    pub async fn delete_key(&self, key: &str) -> ClientResult<crate::types::EphemeralKey> {
        let url = self.client.url(
            &format!(
                "/v1/ephemeral_keys/{}",
                crate::progenitor_support::encode_path(key),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
}
