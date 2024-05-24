use crate::Client;
use crate::ClientResult;

pub struct Git {
    pub client: Client,
}

impl Git {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Git { client }
    }

    /**
     * Create a blob.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/git/blobs` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/git#create-a-blob>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn create_blob(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::GitCreateBlobRequest,
    ) -> ClientResult<crate::Response<crate::types::Tree>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/git/blobs",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Get a blob.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/git/blobs/{file_sha}` endpoint.
     *
     * The `content` in the response will always be Base64 encoded.
     *
     * _Note_: This API supports blobs up to 100 megabytes in size.
     *
     * FROM: <https://docs.github.com/rest/reference/git#get-a-blob>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `file_sha: &str`
     */
    pub async fn get_blob(
        &self,
        owner: &str,
        repo: &str,
        file_sha: &str,
    ) -> ClientResult<crate::Response<crate::types::Blob>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/git/blobs/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(file_sha),
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
     * Create a commit.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/git/commits` endpoint.
     *
     * Creates a new Git [commit object](https://git-scm.com/book/en/v1/Git-Internals-Git-Objects#Commit-Objects).
     *
     * **Signature verification object**
     *
     * The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
     *
     * | Name | Type | Description |
     * | ---- | ---- | ----------- |
     * | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
     * | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
     * | `signature` | `string` | The signature that was extracted from the commit. |
     * | `payload` | `string` | The value that was signed. |
     *
     * These are the possible values for `reason` in the `verification` object:
     *
     * | Value | Description |
     * | ----- | ----------- |
     * | `expired_key` | The key that made the signature is expired. |
     * | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
     * | `gpgverify_error` | There was an error communicating with the signature verification service. |
     * | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
     * | `unsigned` | The object does not include a signature. |
     * | `unknown_signature_type` | A non-PGP signature was found in the commit. |
     * | `no_user` | No user was associated with the `committer` email address in the commit. |
     * | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on her/his account. |
     * | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
     * | `unknown_key` | The key that made the signature has not been registered with any user's account. |
     * | `malformed_signature` | There was an error parsing the signature. |
     * | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
     * | `valid` | None of the above errors applied, so the signature is considered to be verified. |
     *
     * FROM: <https://docs.github.com/rest/reference/git#create-a-commit>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn create_commit(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::GitCreateCommitRequest,
    ) -> ClientResult<crate::Response<crate::types::GitCommit>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/git/commits",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Get a commit.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/git/commits/{commit_sha}` endpoint.
     *
     * Gets a Git [commit object](https://git-scm.com/book/en/v1/Git-Internals-Git-Objects#Commit-Objects).
     *
     * **Signature verification object**
     *
     * The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
     *
     * | Name | Type | Description |
     * | ---- | ---- | ----------- |
     * | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
     * | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
     * | `signature` | `string` | The signature that was extracted from the commit. |
     * | `payload` | `string` | The value that was signed. |
     *
     * These are the possible values for `reason` in the `verification` object:
     *
     * | Value | Description |
     * | ----- | ----------- |
     * | `expired_key` | The key that made the signature is expired. |
     * | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
     * | `gpgverify_error` | There was an error communicating with the signature verification service. |
     * | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
     * | `unsigned` | The object does not include a signature. |
     * | `unknown_signature_type` | A non-PGP signature was found in the commit. |
     * | `no_user` | No user was associated with the `committer` email address in the commit. |
     * | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on her/his account. |
     * | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
     * | `unknown_key` | The key that made the signature has not been registered with any user's account. |
     * | `malformed_signature` | There was an error parsing the signature. |
     * | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
     * | `valid` | None of the above errors applied, so the signature is considered to be verified. |
     *
     * FROM: <https://docs.github.com/rest/reference/git#get-a-commit>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `commit_sha: &str` -- commit_sha parameter.
     */
    pub async fn get_commit(
        &self,
        owner: &str,
        repo: &str,
        commit_sha: &str,
    ) -> ClientResult<crate::Response<crate::types::GitCommit>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/git/commits/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(commit_sha),
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
     * List matching references.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/git/matching-refs/{ref}` endpoint.
     *
     * Returns an array of references from your Git database that match the supplied name. The `:ref` in the URL must be formatted as `heads/<branch name>` for branches and `tags/<tag name>` for tags. If the `:ref` doesn't exist in the repository, but existing refs start with `:ref`, they will be returned as an array.
     *
     * When you use this endpoint without providing a `:ref`, it will return an array of all the references from your Git database, including notes and stashes if they exist on the server. Anything in the namespace is returned, not just `heads` and `tags`.
     *
     * **Note:** You need to explicitly [request a pull request](https://docs.github.com/rest/reference/pulls#get-a-pull-request) to trigger a test merge commit, which checks the mergeability of pull requests. For more information, see "[Checking mergeability of pull requests](https://docs.github.com/rest/guides/getting-started-with-the-git-database-api#checking-mergeability-of-pull-requests)".
     *
     * If you request matching references for a branch named `feature` but the branch `feature` doesn't exist, the response can still include other matching head refs that start with the word `feature`, such as `featureA` and `featureB`.
     *
     * FROM: <https://docs.github.com/rest/reference/git#list-matching-references>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `ref_: &str` -- ref parameter.
     * * `per_page: i64` -- Results per page (max 100).
     * * `page: i64` -- Page number of the results to fetch.
     */
    pub async fn list_matching_refs(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        per_page: i64,
        page: i64,
    ) -> ClientResult<crate::Response<Vec<crate::types::GitRef>>> {
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
                "/repos/{}/{}/git/matching-refs/{}?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(ref_),
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
     * List matching references.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/git/matching-refs/{ref}` endpoint.
     *
     * As opposed to `list_matching_refs`, this function returns all the pages of the request at once.
     *
     * Returns an array of references from your Git database that match the supplied name. The `:ref` in the URL must be formatted as `heads/<branch name>` for branches and `tags/<tag name>` for tags. If the `:ref` doesn't exist in the repository, but existing refs start with `:ref`, they will be returned as an array.
     *
     * When you use this endpoint without providing a `:ref`, it will return an array of all the references from your Git database, including notes and stashes if they exist on the server. Anything in the namespace is returned, not just `heads` and `tags`.
     *
     * **Note:** You need to explicitly [request a pull request](https://docs.github.com/rest/reference/pulls#get-a-pull-request) to trigger a test merge commit, which checks the mergeability of pull requests. For more information, see "[Checking mergeability of pull requests](https://docs.github.com/rest/guides/getting-started-with-the-git-database-api#checking-mergeability-of-pull-requests)".
     *
     * If you request matching references for a branch named `feature` but the branch `feature` doesn't exist, the response can still include other matching head refs that start with the word `feature`, such as `featureA` and `featureB`.
     *
     * FROM: <https://docs.github.com/rest/reference/git#list-matching-references>
     */
    pub async fn list_all_matching_refs(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> ClientResult<crate::Response<Vec<crate::types::GitRef>>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/git/matching-refs/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(ref_),
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
     * Get a reference.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/git/ref/{ref}` endpoint.
     *
     * Returns a single reference from your Git database. The `:ref` in the URL must be formatted as `heads/<branch name>` for branches and `tags/<tag name>` for tags. If the `:ref` doesn't match an existing ref, a `404` is returned.
     *
     * **Note:** You need to explicitly [request a pull request](https://docs.github.com/rest/reference/pulls#get-a-pull-request) to trigger a test merge commit, which checks the mergeability of pull requests. For more information, see "[Checking mergeability of pull requests](https://docs.github.com/rest/guides/getting-started-with-the-git-database-api#checking-mergeability-of-pull-requests)".
     *
     * FROM: <https://docs.github.com/rest/reference/git#get-a-reference>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `ref_: &str` -- ref parameter.
     */
    pub async fn get_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> ClientResult<crate::Response<crate::types::GitRef>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/git/ref/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(ref_),
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
     * Create a reference.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/git/refs` endpoint.
     *
     * Creates a reference for your repository. You are unable to create new references for empty repositories, even if the commit SHA-1 hash used exists. Empty repositories are repositories without branches.
     *
     * FROM: <https://docs.github.com/rest/reference/git#create-a-reference>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn create_ref(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::GitCreateRefRequest,
    ) -> ClientResult<crate::Response<crate::types::GitRef>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/git/refs",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Delete a reference.
     *
     * This function performs a `DELETE` to the `/repos/{owner}/{repo}/git/refs/{ref}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/git#delete-a-reference>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `ref_: &str` -- ref parameter.
     */
    pub async fn delete_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> ClientResult<crate::Response<()>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/git/refs/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(ref_),
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
     * Update a reference.
     *
     * This function performs a `PATCH` to the `/repos/{owner}/{repo}/git/refs/{ref}` endpoint.
     *
     *
     *
     * FROM: <https://docs.github.com/rest/reference/git#update-a-reference>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `ref_: &str` -- ref parameter.
     */
    pub async fn update_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        body: &crate::types::GitUpdateRefRequest,
    ) -> ClientResult<crate::Response<crate::types::GitRef>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/git/refs/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(ref_),
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
     * Create a tag object.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/git/tags` endpoint.
     *
     * Note that creating a tag object does not create the reference that makes a tag in Git. If you want to create an annotated tag in Git, you have to do this call to create the tag object, and then [create](https://docs.github.com/rest/reference/git#create-a-reference) the `refs/tags/[tag]` reference. If you want to create a lightweight tag, you only have to [create](https://docs.github.com/rest/reference/git#create-a-reference) the tag reference - this call would be unnecessary.
     *
     * **Signature verification object**
     *
     * The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
     *
     * | Name | Type | Description |
     * | ---- | ---- | ----------- |
     * | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
     * | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
     * | `signature` | `string` | The signature that was extracted from the commit. |
     * | `payload` | `string` | The value that was signed. |
     *
     * These are the possible values for `reason` in the `verification` object:
     *
     * | Value | Description |
     * | ----- | ----------- |
     * | `expired_key` | The key that made the signature is expired. |
     * | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
     * | `gpgverify_error` | There was an error communicating with the signature verification service. |
     * | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
     * | `unsigned` | The object does not include a signature. |
     * | `unknown_signature_type` | A non-PGP signature was found in the commit. |
     * | `no_user` | No user was associated with the `committer` email address in the commit. |
     * | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on her/his account. |
     * | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
     * | `unknown_key` | The key that made the signature has not been registered with any user's account. |
     * | `malformed_signature` | There was an error parsing the signature. |
     * | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
     * | `valid` | None of the above errors applied, so the signature is considered to be verified. |
     *
     * FROM: <https://docs.github.com/rest/reference/git#create-a-tag-object>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn create_tag(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::GitCreateTagRequest,
    ) -> ClientResult<crate::Response<crate::types::GitTag>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/git/tags",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Get a tag.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/git/tags/{tag_sha}` endpoint.
     *
     * **Signature verification object**
     *
     * The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
     *
     * | Name | Type | Description |
     * | ---- | ---- | ----------- |
     * | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
     * | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
     * | `signature` | `string` | The signature that was extracted from the commit. |
     * | `payload` | `string` | The value that was signed. |
     *
     * These are the possible values for `reason` in the `verification` object:
     *
     * | Value | Description |
     * | ----- | ----------- |
     * | `expired_key` | The key that made the signature is expired. |
     * | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
     * | `gpgverify_error` | There was an error communicating with the signature verification service. |
     * | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
     * | `unsigned` | The object does not include a signature. |
     * | `unknown_signature_type` | A non-PGP signature was found in the commit. |
     * | `no_user` | No user was associated with the `committer` email address in the commit. |
     * | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on her/his account. |
     * | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
     * | `unknown_key` | The key that made the signature has not been registered with any user's account. |
     * | `malformed_signature` | There was an error parsing the signature. |
     * | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
     * | `valid` | None of the above errors applied, so the signature is considered to be verified. |
     *
     * FROM: <https://docs.github.com/rest/reference/git#get-a-tag>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `tag_sha: &str`
     */
    pub async fn get_tag(
        &self,
        owner: &str,
        repo: &str,
        tag_sha: &str,
    ) -> ClientResult<crate::Response<crate::types::GitTag>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/git/tags/{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(tag_sha),
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
     * Create a tree.
     *
     * This function performs a `POST` to the `/repos/{owner}/{repo}/git/trees` endpoint.
     *
     * The tree creation API accepts nested entries. If you specify both a tree and a nested path modifying that tree, this endpoint will overwrite the contents of the tree with the new path contents, and create a new tree structure.
     *
     * If you use this endpoint to add, delete, or modify the file contents in a tree, you will need to commit the tree and then update a branch to point to the commit. For more information see "[Create a commit](https://docs.github.com/rest/reference/git#create-a-commit)" and "[Update a reference](https://docs.github.com/rest/reference/git#update-a-reference)."
     *
     * FROM: <https://docs.github.com/rest/reference/git#create-a-tree>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     */
    pub async fn create_tree(
        &self,
        owner: &str,
        repo: &str,
        body: &crate::types::GitCreateTreeRequestData,
    ) -> ClientResult<crate::Response<crate::types::GitTreeData>> {
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/git/trees",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
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
     * Get a tree.
     *
     * This function performs a `GET` to the `/repos/{owner}/{repo}/git/trees/{tree_sha}` endpoint.
     *
     * Returns a single tree using the SHA1 value for that tree.
     *
     * If `truncated` is `true` in the response then the number of items in the `tree` array exceeded our maximum limit. If you need to fetch more items, use the non-recursive method of fetching trees, and fetch one sub-tree at a time.
     *
     * FROM: <https://docs.github.com/rest/reference/git#get-a-tree>
     *
     * **Parameters:**
     *
     * * `owner: &str`
     * * `repo: &str`
     * * `tree_sha: &str`
     * * `recursive: &str` -- Setting this parameter to any value returns the objects or subtrees referenced by the tree specified in `:tree_sha`. For example, setting `recursive` to any of the following will enable returning objects or subtrees: `0`, `1`, `"true"`, and `"false"`. Omit this parameter to prevent recursively returning objects or subtrees.
     */
    pub async fn get_tree(
        &self,
        owner: &str,
        repo: &str,
        tree_sha: &str,
        recursive: &str,
    ) -> ClientResult<crate::Response<crate::types::GitTreeData>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !recursive.is_empty() {
            query_args.push(("recursive".to_string(), recursive.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/repos/{}/{}/git/trees/{}?{}",
                crate::progenitor_support::encode_path(owner),
                crate::progenitor_support::encode_path(repo),
                crate::progenitor_support::encode_path(tree_sha),
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
}
