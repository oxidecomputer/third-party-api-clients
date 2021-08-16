use anyhow::Result;

use crate::Client;

pub struct VerificationCodes {
    client: Client,
}

impl VerificationCodes {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        VerificationCodes { client }
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/users/{userKey}/verificationCodes` endpoint.
     *
     * Returns the current set of valid backup verification codes for the specified user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn directory_list(&self, user_key: &str) -> Result<crate::types::VerificationCodes> {
        let url = format!(
            "/admin/directory/v1/users/{}/verificationCodes",
            crate::progenitor_support::encode_path(&user_key.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/admin/directory/v1/users/{userKey}/verificationCodes/generate` endpoint.
     *
     * Generates new backup verification codes for the user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Email or immutable ID of the user.
     */
    pub async fn directory_generate(&self, user_key: &str) -> Result<()> {
        let url = format!(
            "/admin/directory/v1/users/{}/verificationCodes/generate",
            crate::progenitor_support::encode_path(&user_key.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/admin/directory/v1/users/{userKey}/verificationCodes/invalidate` endpoint.
     *
     * Invalidates the current backup verification codes for the user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Email or immutable ID of the user.
     */
    pub async fn directory_invalidate(&self, user_key: &str) -> Result<()> {
        let url = format!(
            "/admin/directory/v1/users/{}/verificationCodes/invalidate",
            crate::progenitor_support::encode_path(&user_key.to_string()),
        );

        self.client.post(&url, None).await
    }
}
