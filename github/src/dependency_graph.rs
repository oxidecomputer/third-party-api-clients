use crate::Client;
use crate::ClientResult;

pub struct DependencyGraph {
    pub client: Client,
}

impl DependencyGraph {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        DependencyGraph { client }
    }

    /**
     * Get a diff of the dependencies between commits.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/dependency-graph/compare/{basehead}` endpoint.
     *
     * Gets the diff of the dependency changes between two commits of a repository, based on the changes to the dependency manifests made in those commits.
     *
     * FROM: <https://docs.github.com/rest/dependency-graph/dependency-review#get-a-diff-of-the-dependencies-between-commits>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     * * `basehead: &str` -- The base and head Git revisions to compare. The Git revisions will be resolved to commit SHAs. Named revisions will be resolved to their corresponding HEAD commits, and an appropriate merge base will be determined. This parameter expects the format `{base}...{head}`.
     * * `name: &str` -- The full path, relative to the repository root, of the dependency manifest file.
     */
    pub async fn diff_range(
        &self,
        owner: &str,
        repo: &str,
        basehead: &str,
        name: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::DependencyGraphDiff>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !name.is_empty() {
            query_args.push(("name".to_string(), name.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/dependency-graph/compare/{}?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&basehead.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Get a diff of the dependencies between commits.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/dependency-graph/compare/{basehead}` endpoint.
     *
     * As opposed to `diff_range`, this function returns all the pages of the request at once.
     *
     * Gets the diff of the dependency changes between two commits of a repository, based on the changes to the dependency manifests made in those commits.
     *
     * FROM: <https://docs.github.com/rest/dependency-graph/dependency-review#get-a-diff-of-the-dependencies-between-commits>
     */
    pub async fn get_all_diff_range(
        &self,
        owner: &str,
        repo: &str,
        basehead: &str,
        name: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::DependencyGraphDiff>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !name.is_empty() {
            query_args.push(("name".to_string(), name.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/dependency-graph/compare/{}?{}",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
                crate::progenitor_support::encode_path(&basehead.to_string()),
                query_
            ),
            None,
        );
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Export a software bill of materials (SBOM) for a repository.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/dependency-graph/sbom` endpoint.
     *
     * Exports the software bill of materials (SBOM) for a repository in SPDX JSON format.
     *
     * FROM: <https://docs.github.com/rest/dependency-graph/sboms#export-a-software-bill-of-materials-sbom-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn export_sbom(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::DependencyGraphSpdxSbom>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/dependency-graph/sbom",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
            ),
            None,
        );
        self.client
            .get(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Create a snapshot of dependencies for a repository.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/dependency-graph/snapshots` endpoint.
     *
     * Create a new snapshot of a repository's dependencies.
     *
     * The authenticated user must have access to the repository.
     *
     * OAuth app tokens and personal access tokens (classic) need the `repo` scope to use this endpoint.
     *
     * FROM: <https://docs.github.com/rest/dependency-graph/dependency-submission#create-a-snapshot-of-dependencies-for-a-repository>
     *
     * **Parameters:**
     *
     * * `owner: &str` -- The account owner of the repository. The name is not case sensitive.
     * * `repo: &str` -- The name of the repository without the `.git` extension. The name is not case sensitive.
     */
    pub async fn create_repository_snapshot(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::Snapshot,
    ) -> ClientResult<crate::Response<crate::types::DependencyGraphCreateRepositorySnapshotResponse>>
    {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/dependency-graph/snapshots",
                crate::progenitor_support::encode_path(&owner.to_string()),
                crate::progenitor_support::encode_path(&repo.to_string()),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
}
