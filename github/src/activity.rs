use anyhow::Result;

use crate::Client;

pub struct Activity {
    client: Client,
}

impl Activity {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Activity { client }
    }

    /**
     * List repositories watched by a user.
     *
     * This function performs a `GET` to the `/users/{username}/subscriptions` endpoint.
     *
     * Lists repositories a user is watching.
     *
     * FROM: <https://docs.github.com/rest/reference/activity#list-repositories-watched-by-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_repos_watched_by_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::MinimalRepository>> {
        let url = format!(
            "/users/{}/subscriptions?page={}&per_page={}",
            crate::progenitor_support::encode_path(&username.to_string()),
            format!("{}", page),
            format!("{}", per_page),
        );

        self.client.get_all_pages(&url).await
    }
}
