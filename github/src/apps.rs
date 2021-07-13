use anyhow::Result;

use crate::Client;

pub struct Apps {
    client: Client,
}

impl Apps {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Apps { client }
    }

    /**
     * Get a user installation for the authenticated app.
     *
     * This function performs a `GET` to the `/users/{username}/installation` endpoint.
     *
     * Enables an authenticated GitHub App to find the user’s installation information.
     *
     * You must use a [JWT](https://docs.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app) to access this endpoint.
     *
     * FROM: <https://docs.github.com/rest/reference/apps#get-a-user-installation-for-the-authenticated-app>
     *
     * **Parameters:**
     *
     * * `username: &str`
     */
    pub async fn get_user_installation(
        &self,
        username: &str,
    ) -> Result<crate::types::Installation> {
        let url = format!(
            "/users/{}/installation",
            crate::progenitor_support::encode_path(&username.to_string()),
        );

        self.client.get(&url).await
    }
}
