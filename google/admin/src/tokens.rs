use anyhow::Result;

use crate::Client;

pub struct Tokens {
    client: Client,
}

impl Tokens {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Tokens { client }
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/users/{userKey}/tokens` endpoint.
     *
     * Returns the set of tokens specified user has issued to 3rd party applications.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn directory_list(&self, user_key: &str) -> Result<crate::types::Tokens> {
        let url = format!(
            "/admin/directory/v1/users/{}/tokens",
            crate::progenitor_support::encode_path(&user_key.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/users/{userKey}/tokens/{clientId}` endpoint.
     *
     * Gets information about an access token issued by a user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     * * `client_id: &str` -- The Client ID of the application the token is issued to.
     */
    pub async fn directory_get(
        &self,
        user_key: &str,
        client_id: &str,
    ) -> Result<crate::types::Token> {
        let url = format!(
            "/admin/directory/v1/users/{}/tokens/{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
            crate::progenitor_support::encode_path(&client_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `DELETE` to the `/admin/directory/v1/users/{userKey}/tokens/{clientId}` endpoint.
     *
     * Deletes all access tokens issued by a user for an application.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     * * `client_id: &str` -- The Client ID of the application the token is issued to.
     */
    pub async fn directory_delete(&self, user_key: &str, client_id: &str) -> Result<()> {
        let url = format!(
            "/admin/directory/v1/users/{}/tokens/{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
            crate::progenitor_support::encode_path(&client_id.to_string()),
        );

        self.client.delete(&url, None).await
    }
}
