use anyhow::Result;

use crate::Client;

pub struct Repos {
    client: Client,
}

impl Repos {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Repos { client }
    }

    /**
     * List repositories for a user.
     *
     * This function performs a `GET` to the `/users/{username}/repos` endpoint.
     *
     * Lists public repositories for the specified user. Note: For GitHub AE, this endpoint will list internal repositories for the specified user.
     *
     * FROM: <https://docs.github.com/rest/reference/repos#list-repositories-for-a-user>
     *
     * **Parameters:**
     *
     * * `username: &str`
     * * `type_: crate::types::ReposListUserType` -- Can be one of `all`, `owner`, `member`.
     * * `sort: crate::types::ReposListOrgSort` -- Can be one of `created`, `updated`, `pushed`, `full_name`.
     * * `direction: crate::types::Direction` -- Can be one of `asc` or `desc`. Default: `asc` when using `full_name`, otherwise `desc`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_for_user(
        &self,
        username: &str,
        type_: crate::types::ReposListUserType,
        sort: crate::types::ReposListOrgSort,
        direction: crate::types::Direction,
        per_page: i64,
        page: i64,
    ) -> Result<Vec<crate::types::MinimalRepository>> {
        let url = format!(
            "/users/{}/repos?direction={}&page={}&per_page={}&sort={}&type={}",
            crate::progenitor_support::encode_path(&username.to_string()),
            direction,
            format!("{}", page),
            format!("{}", per_page),
            sort,
            type_,
        );

        self.client.get_all_pages(&url).await
    }
}
