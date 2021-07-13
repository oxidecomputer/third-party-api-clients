use anyhow::Result;

use crate::Client;

pub struct Orgs {
    client: Client,
}

impl Orgs {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Orgs { client }
    }

    /**
     * List organizations for a user.
     *
     * This function performs a `GET` to the `/users/{username}/orgs` endpoint.
     *
     * List [public organization memberships](https://help.github.com/articles/publicizing-or-concealing-organization-membership) for the specified user.
     *
     * This method only lists _public_ memberships, regardless of authentication. If you need to fetch all of the organization memberships (public and private) for the authenticated user, use the [List organizations for the authenticated user](https://docs.github.com/rest/reference/orgs#list-organizations-for-the-authenticated-user) API instead.
     *
     * FROM: <https://docs.github.com/rest/reference/orgs#list-organizations-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_user(
        &self,
        username: &str,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::OrganizationSimple>> {
        let url = format!(
            "/users/{}/orgs?page={}&per_page={}",
            crate::progenitor_support::encode_path(&username.to_string()),
            format!("{}", page),
            format!("{}", per_page),
        );

        self.client.get_all_pages(&url).await
    }
}
