use anyhow::Result;

use crate::Client;

pub struct Users {
    client: Client,
}

impl Users {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Users { client }
    }

    /**
     * List public keys for a user.
     *
     * This function performs a `GET` to the `/users/{username}/keys` endpoint.
     *
     * Lists the _verified_ public SSH keys for a user. This is accessible by anyone.
     *
     * FROM: <https://docs.github.com/rest/reference/users#list-public-keys-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_public_keys_for_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::KeySimple>> {
        let url = format!(
            "/users/{}/keys?page={}&per_page={}",
            crate::progenitor_support::encode_path(&username.to_string()),
            format!("{}", page),
            format!("{}", per_page),
        );

        self.client.get_all_pages(&url).await
    }
}
