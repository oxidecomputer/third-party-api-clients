use crate::Client;
use crate::ClientResult;

pub struct Files {
    pub client: Client,
}

impl Files {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Files { client }
    }

    /**
     * This function performs a `POST` to the `/files.delete` endpoint.
     *
     * Deletes a file.
     *
     * FROM: <https://api.slack.com/methods/files.delete>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `files:write:user`.
     */
    pub async fn delete(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/files.delete", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/files.info` endpoint.
     *
     * Gets information about a file.
     *
     * FROM: <https://api.slack.com/methods/files.info>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `files:read`.
     * * `file: &str` -- Specify a file by providing its ID.
     * * `count: &str`
     * * `page: &str`
     * * `limit: i64` -- The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached.
     * * `cursor: &str` -- Parameter for pagination. File comments are paginated for a single file. Set `cursor` equal to the `next_cursor` attribute returned by the previous request's `response_metadata`. This parameter is optional, but pagination is mandatory: the default value simply fetches the first "page" of the collection of comments. See [pagination](/docs/pagination) for more details.
     */
    pub async fn info(
        &self,
        file: &str,
        count: &str,
        page: &str,
        limit: i64,
        cursor: &str,
    ) -> ClientResult<crate::types::FilesInfoSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !count.is_empty() {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if !file.is_empty() {
            query_args.push(("file".to_string(), file.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !page.is_empty() {
            query_args.push(("page".to_string(), page.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/files.info?{}", query_), None);
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
     * This function performs a `GET` to the `/files.list` endpoint.
     *
     * List for a team, in a channel, or from a user with applied filters.
     *
     * FROM: <https://api.slack.com/methods/files.list>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `files:read`.
     * * `user: &str` -- Filter files created by a single user.
     * * `channel: &str` -- Filter files appearing in a specific channel, indicated by its ID.
     * * `ts_from: f64` -- Filter files created after this timestamp (inclusive).
     * * `ts_to: f64` -- Filter files created before this timestamp (inclusive).
     * * `types: &str` -- Filter files by type ([see below](#file_types)). You can pass multiple values in the types argument, like `types=spaces,snippets`.The default value is `all`, which does not filter the list.
     * * `count: &str`
     * * `page: &str`
     * * `show_files_hidden_by_limit: bool` -- Show truncated file info for files hidden due to being too old, and the team who owns the file being over the file limit.
     */
    pub async fn list(
        &self,
        user: &str,
        channel: &str,
        ts_from: f64,
        ts_to: f64,
        types: &str,
        count: &str,
        page: &str,
        show_files_hidden_by_limit: bool,
    ) -> ClientResult<crate::types::FilesListSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !channel.is_empty() {
            query_args.push(("channel".to_string(), channel.to_string()));
        }
        if !count.is_empty() {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !page.is_empty() {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if show_files_hidden_by_limit {
            query_args.push((
                "show_files_hidden_by_limit".to_string(),
                show_files_hidden_by_limit.to_string(),
            ));
        }
        if !ts_from.to_string().is_empty() {
            query_args.push(("ts_from".to_string(), ts_from.to_string()));
        }
        if !ts_to.to_string().is_empty() {
            query_args.push(("ts_to".to_string(), ts_to.to_string()));
        }
        if !types.is_empty() {
            query_args.push(("types".to_string(), types.to_string()));
        }
        if !user.is_empty() {
            query_args.push(("user".to_string(), user.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/files.list?{}", query_), None);
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
     * This function performs a `POST` to the `/files.revokePublicURL` endpoint.
     *
     * Revokes public/external sharing access for a file
     *
     * FROM: <https://api.slack.com/methods/files.revokePublicURL>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `files:write:user`.
     */
    pub async fn revoke_public_url(&self) -> ClientResult<crate::types::FilesUploadSchema> {
        let url = self.client.url("/files.revokePublicURL", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/files.sharedPublicURL` endpoint.
     *
     * Enables a file for public/external sharing.
     *
     * FROM: <https://api.slack.com/methods/files.sharedPublicURL>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `files:write:user`.
     */
    pub async fn shared_public_url(&self) -> ClientResult<crate::types::FilesUploadSchema> {
        let url = self.client.url("/files.sharedPublicURL", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/files.upload` endpoint.
     *
     * Uploads or creates a file.
     *
     * FROM: <https://api.slack.com/methods/files.upload>
     */
    pub async fn upload(&self) -> ClientResult<crate::types::FilesUploadSchema> {
        let url = self.client.url("/files.upload", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
}
