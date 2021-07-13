use anyhow::Result;

use crate::Client;

pub struct Checks {
    client: Client,
}

impl Checks {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Checks { client }
    }

    /**
     * List check suites for a Git reference.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/commits/{ref}/check-suites` endpoint.
     *
     * **Note:** The Checks API only looks for pushes in the repository where the check suite or check run were created. Pushes to a branch in a forked repository are not detected and return an empty `pull_requests` array and a `null` value for `head_branch`.
     *
     * Lists check suites for a commit `ref`. The `ref` can be a SHA, branch name, or a tag name. GitHub Apps must have the `checks:read` permission on a private repository or pull access to a public repository to list check suites. OAuth Apps and authenticated users must have the `repo` scope to get check suites in a private repository.
     *
     * FROM: <https://docs.github.com/rest/reference/checks#list-check-suites-for-a-git-reference>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `ref_: &str` -- ref parameter.
     * * `app_id: i64` -- Filters check suites by GitHub App `id`.
     * * `check_name: &str` -- Returns check runs with the specified `name`.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_suites_for_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        app_id: i64,
        check_name: &str,
        per_page: i64,
        page: i64,
    ) -> Result<crate::types::GetChecksListSuitesRefOkResponse> {
        let url = format!(
            "/repos/{}/{}/commits/{}/check-suites?app_id={}&check_name={}&page={}&per_page={}",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&ref_.to_string()),
            format!("{}", app_id),
            check_name.to_string(),
            format!("{}", page),
            format!("{}", per_page),
        );

        self.client.get(&url).await
    }
}
