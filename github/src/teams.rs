use anyhow::Result;

use crate::Client;

pub struct Teams {
    client: Client,
}

impl Teams {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Teams { client }
    }

    /**
     * List teams for the authenticated user.
     *
     * This function performs a `GET` to the `/user/teams` endpoint.
     *
     * List all of the teams across all of the organizations to which the authenticated user belongs. This method requires `user`, `repo`, or `read:org` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/) when authenticating via [OAuth](https://docs.github.com/apps/building-oauth-apps/).
     *
     * FROM: <https://docs.github.com/rest/reference/teams#list-teams-for-the-authenticated-user>
     *
     * **Parameters:**
     *
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_authenticated_user(
        &self,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::TeamFull>> {
        let url = format!(
            "/user/teams?page={}&per_page={}",
            format!("{}", page),
            format!("{}", per_page),
        );

        self.client.get_all_pages(&url).await
    }
}
