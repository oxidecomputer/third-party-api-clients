use anyhow::Result;

use crate::Client;

pub struct Pulls {
    client: Client,
}

impl Pulls {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Pulls { client }
    }

    /**
     * Update a pull request branch.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/pulls/{pull_number}/update-branch` endpoint.
     *
     * Updates the pull request branch with the latest upstream changes by merging HEAD from the base branch into the pull request branch.
     *
     * FROM: <https://docs.github.com/rest/reference/pulls#update-a-pull-request-branch>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `pull_number: i64`
     */
    pub async fn update_branch(
        &self,
        owner: &str,
        repo: &str,
        pull_number: i64,
        body: &crate::types::PullsUpdateBranchRequest,
    ) -> Result<crate::types::PutPullsUpdateBranchAcceptedResponse> {
        let url = format!(
            "/repos/{}/{}/pulls/{}/update-branch",
            crate::progenitor_support::encode_path(&owner.to_string()),
            crate::progenitor_support::encode_path(&repo.to_string()),
            crate::progenitor_support::encode_path(&pull_number.to_string()),
        );

        self.client
            .put(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
