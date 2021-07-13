use anyhow::Result;

use crate::Client;

pub struct Migrations {
    client: Client,
}

impl Migrations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Migrations { client }
    }

    /**
     * List repositories for a user migration.
     *
     * This function performs a `GET` to the `/user/migrations/{migration_id}/repositories` endpoint.
     *
     * Lists all the repositories for this user migration.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#list-repositories-for-a-user-migration>
     *
     * **Parameters:**
     *
     * * `migration_id: i64` -- migration_id parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_repos_for_user(
        &self,
        migration_id: i64,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::MinimalRepository>> {
        let url = format!(
            "/user/migrations/{}/repositories?page={}&per_page={}",
            crate::progenitor_support::encode_path(&migration_id.to_string()),
            format!("{}", page),
            format!("{}", per_page),
        );

        self.client.get_all_pages(&url).await
    }
}
