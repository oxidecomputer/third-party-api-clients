use anyhow::Result;

use crate::Client;

pub struct Projects {
    client: Client,
}

impl Projects {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Projects { client }
    }

    /**
     * List user projects.
     *
     * This function performs a `GET` to the `/users/{username}/projects` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/projects#list-user-projects>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `state: crate::types::IssuesListState` -- Indicates the state of the projects to return. Can be either `open`, `closed`, or `all`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_user(
        &self,
        username: &str,
        state: crate::types::IssuesListState,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::Project>> {
        let url = format!(
            "/users/{}/projects?page={}&per_page={}&state={}",
            crate::progenitor_support::encode_path(&username.to_string()),
            format!("{}", page),
            format!("{}", per_page),
            state,
        );

        self.client.get_all_pages(&url).await
    }
}
