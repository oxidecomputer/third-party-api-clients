use anyhow::Result;

use crate::Client;

pub struct Asps {
    pub client: Client,
}

impl Asps {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Asps { client }
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/users/{userKey}/asps` endpoint.
     *
     * Lists the ASPs issued by a user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn list(&self, user_key: &str) -> Result<crate::types::Asps> {
        let url = format!(
            "/admin/directory/v1/users/{}/asps",
            crate::progenitor_support::encode_path(&user_key.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/admin/directory/v1/users/{userKey}/asps/{codeId}` endpoint.
     *
     * Gets information about an ASP issued by a user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     * * `code_id: i64` -- The unique ID of the ASP.
     */
    pub async fn get(&self, user_key: &str, code_id: i64) -> Result<crate::types::Asp> {
        let url = format!(
            "/admin/directory/v1/users/{}/asps/{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
            crate::progenitor_support::encode_path(&code_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `DELETE` to the `/admin/directory/v1/users/{userKey}/asps/{codeId}` endpoint.
     *
     * Deletes an ASP issued by a user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     * * `code_id: i64` -- The unique ID of the ASP to be deleted.
     */
    pub async fn delete(&self, user_key: &str, code_id: i64) -> Result<()> {
        let url = format!(
            "/admin/directory/v1/users/{}/asps/{}",
            crate::progenitor_support::encode_path(&user_key.to_string()),
            crate::progenitor_support::encode_path(&code_id.to_string()),
        );

        self.client.delete(&url, None).await
    }
}
