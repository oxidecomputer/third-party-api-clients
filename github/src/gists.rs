use anyhow::Result;

use crate::Client;

pub struct Gists {
    client: Client,
}

impl Gists {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Gists { client }
    }

    /**
     * List gists for a user.
     *
     * This function performs a `GET` to the `/users/{username}/gists` endpoint.
     *
     * Lists public gists for the specified user:
     *
     * FROM: <https://docs.github.com/rest/reference/gists#list-gists-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `since: chrono::DateTime<chrono::Utc>` -- Only show notifications updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_user(
        &self,
        username: &str,
        since: chrono::DateTime<chrono::Utc>,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::BaseGist>> {
        let url = format!(
            "/users/{}/gists?page={}&per_page={}&since={}",
            crate::progenitor_support::encode_path(&username.to_string()),
            format!("{}", page),
            format!("{}", per_page),
            since,
        );

        self.client.get_all_pages(&url).await
    }
}
