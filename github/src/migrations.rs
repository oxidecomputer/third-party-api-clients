use crate::Client;
use crate::ClientResult;

pub struct Migrations {
    pub client: Client,
}

impl Migrations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Migrations { client }
    }

    /**
     * List organization migrations.
     *
     * This function performs a `GET` to the `/orgs/{org}/migrations` endpoint.
     *
     * Lists the most recent migrations.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#list-organization-migrations>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     * * `exclude: &[String]` -- Exclude attributes from the API response to improve performance.
     */
    pub async fn list_for_org(
        &self,
        org: &str,
        per_page: i64,
        page: i64,
        exclude: &[String],
    ) -> ClientResult<crate::Response<Vec<crate::types::Migration>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude.is_empty() {
            query_args.push(("exclude".to_string(), exclude.join(" ")));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/migrations?{}",
                crate::progenitor_support::encode_path(org),
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
     * List organization migrations.
     *
     * This function performs a `GET` to the `/orgs/{org}/migrations` endpoint.
     *
     * As opposed to `list_for_org`, this function returns all the pages of the request at once.
     *
     * Lists the most recent migrations.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#list-organization-migrations>
     */
    pub async fn list_all_for_org(
        &self,
        org: &str,
        exclude: &[String],
    ) -> ClientResult<crate::Response<Vec<crate::types::Migration>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude.is_empty() {
            query_args.push(("exclude".to_string(), exclude.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/migrations?{}",
                crate::progenitor_support::encode_path(org),
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
     * Start an organization migration.
     *
     * This function performs a `POST` to the `/orgs/{org}/migrations` endpoint.
     *
     * Initiates the generation of a migration archive.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#start-an-organization-migration>
     *
     * **Parameters:**
     *
     * * `org: &str`
     */
    pub async fn start_for_org(
        &self,
        org: &str,
        body: &crate::types::MigrationsStartRequest,
    ) -> ClientResult<crate::Response<crate::types::Migration>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/migrations",
                crate::progenitor_support::encode_path(org),
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
    /**
     * Get an organization migration status.
     *
     * This function performs a `GET` to the `/orgs/{org}/migrations/{migration_id}` endpoint.
     *
     * Fetches the status of a migration.
     *
     * The `state` of a migration can be one of the following values:
     *
     * *   `pending`, which means the migration hasn't started yet.
     * *   `exporting`, which means the migration is in progress.
     * *   `exported`, which means the migration finished successfully.
     * *   `failed`, which means the migration failed.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#get-an-organization-migration-status>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `migration_id: i64` -- migration_id parameter.
     * * `exclude: &[String]` -- Exclude attributes from the API response to improve performance.
     */
    pub async fn get_status_for_org(
        &self,
        org: &str,
        migration_id: i64,
        exclude: &[String],
    ) -> ClientResult<crate::Response<crate::types::Migration>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude.is_empty() {
            query_args.push(("exclude".to_string(), exclude.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/migrations/{}?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&migration_id.to_string()),
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
     * Download an organization migration archive.
     *
     * This function performs a `GET` to the `/orgs/{org}/migrations/{migration_id}/archive` endpoint.
     *
     * Fetches the URL to a migration archive.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#download-an-organization-migration-archive>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `migration_id: i64` -- migration_id parameter.
     */
    pub async fn download_archive_for_org(
        &self,
        org: &str,
        migration_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/migrations/{}/archive",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&migration_id.to_string()),
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
     * Delete an organization migration archive.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/migrations/{migration_id}/archive` endpoint.
     *
     * Deletes a previous migration archive. Migration archives are automatically deleted after seven days.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#delete-an-organization-migration-archive>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `migration_id: i64` -- migration_id parameter.
     */
    pub async fn delete_archive_for_org(
        &self,
        org: &str,
        migration_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/migrations/{}/archive",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&migration_id.to_string()),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Unlock an organization repository.
     *
     * This function performs a `DELETE` to the `/orgs/{org}/migrations/{migration_id}/repos/{repo_name}/lock` endpoint.
     *
     * Unlocks a repository that was locked for migration. You should unlock each migrated repository and [delete them](https://docs.github.com/rest/reference/repos#delete-a-repository) when the migration is complete and you no longer need the source data.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#unlock-an-organization-repository>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `migration_id: i64` -- migration_id parameter.
     * * `repo_name: &str` -- repo_name parameter.
     */
    pub async fn unlock_repo_for_org(
        &self,
        org: &str,
        migration_id: i64,
        repo_name: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/migrations/{}/repos/{}/lock",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&migration_id.to_string()),
                crate::progenitor_support::encode_path(repo_name),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * List repositories in an organization migration.
     *
     * This function performs a `GET` to the `/orgs/{org}/migrations/{migration_id}/repositories` endpoint.
     *
     * List all the repositories for this organization migration.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#list-repositories-in-an-organization-migration>
     *
     * **Parameters:**
     *
     * * `org: &str`
     * * `migration_id: i64` -- migration_id parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_repos_for_org(
        &self,
        org: &str,
        migration_id: i64,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::MinimalRepository>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/orgs/{}/migrations/{}/repositories?{}",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&migration_id.to_string()),
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
     * List repositories in an organization migration.
     *
     * This function performs a `GET` to the `/orgs/{org}/migrations/{migration_id}/repositories` endpoint.
     *
     * As opposed to `list_repos_for_org`, this function returns all the pages of the request at once.
     *
     * List all the repositories for this organization migration.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#list-repositories-in-an-organization-migration>
     */
    pub async fn list_all_repos_for_org(
        &self,
        org: &str,
        migration_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::MinimalRepository>>> {
        let url = self.client.url(
            &format!(
                "/orgs/{}/migrations/{}/repositories",
                crate::progenitor_support::encode_path(org),
                crate::progenitor_support::encode_path(&migration_id.to_string()),
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
     * Get an import status.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/import` endpoint.
     *
     * View the progress of an import.
     *
     * **Import status**
     *
     * This section includes details about the possible values of the `status` field of the Import Progress response.
     *
     * An import that does not have errors will progress through these steps:
     *
     * *   `detecting` - the "detection" step of the import is in progress because the request did not include a `vcs` parameter. The import is identifying the type of source control present at the URL.
     * *   `importing` - the "raw" step of the import is in progress. This is where commit data is fetched from the original repository. The import progress response will include `commit_count` (the total number of raw commits that will be imported) and `percent` (0 - 100, the current progress through the import).
     * *   `mapping` - the "rewrite" step of the import is in progress. This is where SVN branches are converted to Git branches, and where author updates are applied. The import progress response does not include progress information.
     * *   `pushing` - the "push" step of the import is in progress. This is where the importer updates the repository on GitHub. The import progress response will include `push_percent`, which is the percent value reported by `git push` when it is "Writing objects".
     * *   `complete` - the import is complete, and the repository is ready on GitHub.
     *
     * If there are problems, you will see one of these in the `status` field:
     *
     * *   `auth_failed` - the import requires authentication in order to connect to the original repository. To update authentication for the import, please see the [Update an import](https://docs.github.com/rest/reference/migrations#update-an-import) section.
     * *   `error` - the import encountered an error. The import progress response will include the `failed_step` and an error message. Contact [GitHub Support](https://support.github.com/contact?tags=rest-api) for more information.
     * *   `detection_needs_auth` - the importer requires authentication for the originating repository to continue detection. To update authentication for the import, please see the [Update an import](https://docs.github.com/rest/reference/migrations#update-an-import) section.
     * *   `detection_found_nothing` - the importer didn't recognize any source control at the URL. To resolve, [Cancel the import](https://docs.github.com/rest/reference/migrations#cancel-an-import) and [retry](https://docs.github.com/rest/reference/migrations#start-an-import) with the correct URL.
     * *   `detection_found_multiple` - the importer found several projects or repositories at the provided URL. When this is the case, the Import Progress response will also include a `project_choices` field with the possible project choices as values. To update project choice, please see the [Update an import](https://docs.github.com/rest/reference/migrations#update-an-import) section.
     *
     * **The project_choices field**
     *
     * When multiple projects are found at the provided URL, the response hash will include a `project_choices` field, the value of which is an array of hashes each representing a project choice. The exact key/value pairs of the project hashes will differ depending on the version control type.
     *
     * **Git LFS related fields**
     *
     * This section includes details about Git LFS related fields that may be present in the Import Progress response.
     *
     * *   `use_lfs` - describes whether the import has been opted in or out of using Git LFS. The value can be `opt_in`, `opt_out`, or `undecided` if no action has been taken.
     * *   `has_large_files` - the boolean value describing whether files larger than 100MB were found during the `importing` step.
     * *   `large_files_size` - the total size in gigabytes of files larger than 100MB found in the originating repository.
     * *   `large_files_count` - the total number of files larger than 100MB found in the originating repository. To see a list of these files, make a "Get Large Files" request.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#get-an-import-status>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get_import_status(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<crate::types::Import>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/import",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Start an import.
     *
     * This function performs a `PUT` to the `/repos/{owner}/{repo}/import` endpoint.
     *
     * Start a source import to a GitHub repository using GitHub Importer.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#start-an-import>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn start_import(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::MigrationsStartImportRequest,
    ) -> ClientResult<crate::Response<crate::types::Import>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/import",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Cancel an import.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/import` endpoint.
     *
     * Stop an import for a repository.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#cancel-an-import>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn cancel_import(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/import",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Update an import.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/import` endpoint.
     *
     * An import can be updated with credentials or a project choice by passing in the appropriate parameters in this API
     * request. If no parameters are provided, the import will be restarted.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#update-an-import>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn update_import(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::MigrationsUpdateImportRequest,
    ) -> ClientResult<crate::Response<crate::types::Import>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/import",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Get commit authors.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/import/authors` endpoint.
     *
     * Each type of source control system represents authors in a different way. For example, a Git commit author has a display name and an email address, but a Subversion commit author just has a username. The GitHub Importer will make the author information valid, but the author might not be correct. For example, it will change the bare Subversion username `hubot` into something like `hubot <hubot@12341234-abab-fefe-8787-fedcba987654>`.
     *
     * This endpoint and the [Map a commit author](https://docs.github.com/rest/reference/migrations#map-a-commit-author) endpoint allow you to provide correct Git author information.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#get-commit-authors>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `since: i64` -- A user ID. Only return users with an ID greater than this ID.
     */
    pub async fn get_commit_authors(
        &self,
        owner: &str,
        repo: &str,
        since: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::PorterAuthor>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if since > 0 {
            query_args.push(("since".to_string(), since.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/import/authors?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Get commit authors.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/import/authors` endpoint.
     *
     * As opposed to `get_commit_authors`, this function returns all the pages of the request at once.
     *
     * Each type of source control system represents authors in a different way. For example, a Git commit author has a display name and an email address, but a Subversion commit author just has a username. The GitHub Importer will make the author information valid, but the author might not be correct. For example, it will change the bare Subversion username `hubot` into something like `hubot <hubot@12341234-abab-fefe-8787-fedcba987654>`.
     *
     * This endpoint and the [Map a commit author](https://docs.github.com/rest/reference/migrations#map-a-commit-author) endpoint allow you to provide correct Git author information.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#get-commit-authors>
     */
    pub async fn get_all_commit_authors(
        &self,
        owner: &str,
        repo: &str,
        since: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::PorterAuthor>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if since > 0 {
            query_args.push(("since".to_string(), since.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/import/authors?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Map a commit author.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/import/authors/{author_id}` endpoint.
     *
     * Update an author's identity for the import. Your application can continue updating authors any time before you push new commits to the repository.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#map-a-commit-author>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `author_id: i64`
     */
    pub async fn map_commit_author(
        &self,
        owner: &str,
        repo: &str,
        author_id: i64,
        body: &crate::types::Author,
    ) -> ClientResult<crate::Response<crate::types::PorterAuthor>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/import/authors/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(&author_id.to_string()),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Get large files.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/import/large_files` endpoint.
     *
     * List files larger than 100MB found during the import
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#get-large-files>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn get_large_files(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::PorterLargeFile>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/import/large_files",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Get large files.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/import/large_files` endpoint.
     *
     * As opposed to `get_large_files`, this function returns all the pages of the request at once.
     *
     * List files larger than 100MB found during the import
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#get-large-files>
     */
    pub async fn get_all_large_files(
        &self,
        owner: &str,
        repo: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::PorterLargeFile>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/import/large_files",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Update Git LFS preference.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/import/lfs` endpoint.
     *
     * You can import repositories from Subversion, Mercurial, and TFS that include files larger than 100MB. This ability is powered by [Git LFS](https://git-lfs.github.com). You can learn more about our LFS feature and working with large files [on our help site](https://help.github.com/articles/versioning-large-files/).
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#update-git-lfs-preference>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn set_lfs_preference(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::MigrationsSetLfsPreferenceRequest,
    ) -> ClientResult<crate::Response<crate::types::Import>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/import/lfs",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
            ),
            None,
        );
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * List user migrations.
     *
     * This function performs a `GET` to the `/user/migrations` endpoint.
     *
     * Lists all migrations a user has started.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#list-user-migrations>
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
    ) -> ClientResult<crate::Response<Vec<crate::types::Migration>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/user/migrations?{}", query_), None);
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
     * List user migrations.
     *
     * This function performs a `GET` to the `/user/migrations` endpoint.
     *
     * As opposed to `list_for_authenticated_user`, this function returns all the pages of the request at once.
     *
     * Lists all migrations a user has started.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#list-user-migrations>
     */
    pub async fn list_all_for_authenticated_user(
        &self,
    ) -> ClientResult<crate::Response<Vec<crate::types::Migration>>> {
        let url = self.client.url("/user/migrations", None);
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
     * Start a user migration.
     *
     * This function performs a `POST` to the `/user/migrations` endpoint.
     *
     * Initiates the generation of a user migration archive.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#start-a-user-migration>
     */
    pub async fn start_for_authenticated_user(
        &self,
        body: &crate::types::MigrationsStartRequest,
    ) -> ClientResult<crate::Response<crate::types::Migration>> {
        let url = self.client.url("/user/migrations", None);
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
    /**
     * Get a user migration status.
     *
     * This function performs a `GET` to the `/user/migrations/{migration_id}` endpoint.
     *
     * Fetches a single user migration. The response includes the `state` of the migration, which can be one of the following values:
     *
     * *   `pending` - the migration hasn't started yet.
     * *   `exporting` - the migration is in progress.
     * *   `exported` - the migration finished successfully.
     * *   `failed` - the migration failed.
     *
     * Once the migration has been `exported` you can [download the migration archive](https://docs.github.com/rest/reference/migrations#download-a-user-migration-archive).
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#get-a-user-migration-status>
     *
     * **Parameters:**
     *
     * * `migration_id: i64` -- migration_id parameter.
     * * `exclude: &[String]` -- The list of events for the GitHub app.
     */
    pub async fn get_status_for_authenticated_user(
        &self,
        migration_id: i64,
        exclude: &[String],
    ) -> ClientResult<crate::Response<crate::types::Migration>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude.is_empty() {
            query_args.push(("exclude".to_string(), exclude.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/user/migrations/{}?{}",
                crate::progenitor_support::encode_path(&migration_id.to_string()),
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
     * Download a user migration archive.
     *
     * This function performs a `GET` to the `/user/migrations/{migration_id}/archive` endpoint.
     *
     * Fetches the URL to download the migration archive as a `tar.gz` file. Depending on the resources your repository uses, the migration archive can contain JSON files with data for these objects:
     *
     * *   attachments
     * *   bases
     * *   commit\_comments
     * *   issue\_comments
     * *   issue\_events
     * *   issues
     * *   milestones
     * *   organizations
     * *   projects
     * *   protected\_branches
     * *   pull\_request\_reviews
     * *   pull\_requests
     * *   releases
     * *   repositories
     * *   review\_comments
     * *   schema
     * *   users
     *
     * The archive will also contain an `attachments` directory that includes all attachment files uploaded to GitHub.com and a `repositories` directory that contains the repository's Git data.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#download-a-user-migration-archive>
     *
     * **Parameters:**
     *
     * * `migration_id: i64` -- migration_id parameter.
     */
    pub async fn get_archive_for_authenticated_user(
        &self,
        migration_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/migrations/{}/archive",
                crate::progenitor_support::encode_path(&migration_id.to_string()),
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
     * Delete a user migration archive.
     *
     * This function performs a `DELETE` to the `/user/migrations/{migration_id}/archive` endpoint.
     *
     * Deletes a previous migration archive. Downloadable migration archives are automatically deleted after seven days. Migration metadata, which is returned in the [List user migrations](https://docs.github.com/rest/reference/migrations#list-user-migrations) and [Get a user migration status](https://docs.github.com/rest/reference/migrations#get-a-user-migration-status) endpoints, will continue to be available even after an archive is deleted.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#delete-a-user-migration-archive>
     *
     * **Parameters:**
     *
     * * `migration_id: i64` -- migration_id parameter.
     */
    pub async fn delete_archive_for_authenticated_user(
        &self,
        migration_id: i64,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/migrations/{}/archive",
                crate::progenitor_support::encode_path(&migration_id.to_string()),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * Unlock a user repository.
     *
     * This function performs a `DELETE` to the `/user/migrations/{migration_id}/repos/{repo_name}/lock` endpoint.
     *
     * Unlocks a repository. You can lock repositories when you [start a user migration](https://docs.github.com/rest/reference/migrations#start-a-user-migration). Once the migration is complete you can unlock each repository to begin using it again or [delete the repository](https://docs.github.com/rest/reference/repos#delete-a-repository) if you no longer need the source data. Returns a status of `404 Not Found` if the repository is not locked.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#unlock-a-user-repository>
     *
     * **Parameters:**
     *
     * * `migration_id: i64` -- migration_id parameter.
     * * `repo_name: &str` -- repo_name parameter.
     */
    pub async fn unlock_repo_for_authenticated_user(
        &self,
        migration_id: i64,
        repo_name: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/user/migrations/{}/repos/{}/lock",
                crate::progenitor_support::encode_path(&migration_id.to_string()),
                crate::progenitor_support::encode_path(repo_name),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
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
    ) -> ClientResult<crate::Response<Vec<crate::types::MinimalRepository>>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if per_page > 0 {
            query_args.push(("per_page".to_string(), per_page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/user/migrations/{}/repositories?{}",
                crate::progenitor_support::encode_path(&migration_id.to_string()),
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
     * List repositories for a user migration.
     *
     * This function performs a `GET` to the `/user/migrations/{migration_id}/repositories` endpoint.
     *
     * As opposed to `list_repos_for_user`, this function returns all the pages of the request at once.
     *
     * Lists all the repositories for this user migration.
     *
     * FROM: <https://docs.github.com/rest/reference/migrations#list-repositories-for-a-user-migration>
     */
    pub async fn list_all_repos_for_user(
        &self,
        migration_id: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::MinimalRepository>>> {
        let url = self.client.url(
            &format!(
                "/user/migrations/{}/repositories",
                crate::progenitor_support::encode_path(&migration_id.to_string()),
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
}
