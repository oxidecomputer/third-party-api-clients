use crate::Client;
use crate::ClientResult;

pub struct FilesRemote {
    pub client: Client,
}

impl FilesRemote {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        FilesRemote { client }
    }

    /**
     * This function performs a `POST` to the `/files.remote.add` endpoint.
     *
     * Adds a file from a remote service
     *
     * FROM: <https://api.slack.com/methods/files.remote.add>
     */
    pub async fn add(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/files.remote.add", None);
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
     * This function performs a `GET` to the `/files.remote.info` endpoint.
     *
     * Retrieve information about a remote file added to Slack
     *
     * FROM: <https://api.slack.com/methods/files.remote.info>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `remote_files:read`.
     * * `file: &str` -- Specify a file by providing its ID.
     * * `external_id: &str` -- Creator defined GUID for the file.
     */
    pub async fn info(
        &self,
        file: &str,
        external_id: &str,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !external_id.is_empty() {
            query_args.push(("external_id".to_string(), external_id.to_string()));
        }
        if !file.is_empty() {
            query_args.push(("file".to_string(), file.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/files.remote.info?{}", query_), None);
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
     * This function performs a `GET` to the `/files.remote.list` endpoint.
     *
     * Retrieve information about a remote file added to Slack
     *
     * FROM: <https://api.slack.com/methods/files.remote.list>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `remote_files:read`.
     * * `channel: &str` -- Filter files appearing in a specific channel, indicated by its ID.
     * * `ts_from: f64` -- Filter files created after this timestamp (inclusive).
     * * `ts_to: f64` -- Filter files created before this timestamp (inclusive).
     * * `limit: i64` -- The maximum number of items to return.
     * * `cursor: &str` -- Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](/docs/pagination) for more detail.
     */
    pub async fn list(
        &self,
        channel: &str,
        ts_from: f64,
        ts_to: f64,
        limit: i64,
        cursor: &str,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !channel.is_empty() {
            query_args.push(("channel".to_string(), channel.to_string()));
        }
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !ts_from.to_string().is_empty() {
            query_args.push(("ts_from".to_string(), ts_from.to_string()));
        }
        if !ts_to.to_string().is_empty() {
            query_args.push(("ts_to".to_string(), ts_to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/files.remote.list?{}", query_), None);
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
     * This function performs a `POST` to the `/files.remote.remove` endpoint.
     *
     * Remove a remote file.
     *
     * FROM: <https://api.slack.com/methods/files.remote.remove>
     */
    pub async fn remove(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/files.remote.remove", None);
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
     * This function performs a `GET` to the `/files.remote.share` endpoint.
     *
     * Share a remote file into a channel.
     *
     * FROM: <https://api.slack.com/methods/files.remote.share>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `remote_files:share`.
     * * `file: &str` -- Specify a file registered with Slack by providing its ID. Either this field or `external_id` or both are required.
     * * `external_id: &str` -- The globally unique identifier (GUID) for the file, as set by the app registering the file with Slack.  Either this field or `file` or both are required.
     * * `channels: &str` -- Comma-separated list of channel IDs where the file will be shared.
     */
    pub async fn share(
        &self,
        file: &str,
        external_id: &str,
        channels: &str,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !channels.is_empty() {
            query_args.push(("channels".to_string(), channels.to_string()));
        }
        if !external_id.is_empty() {
            query_args.push(("external_id".to_string(), external_id.to_string()));
        }
        if !file.is_empty() {
            query_args.push(("file".to_string(), file.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/files.remote.share?{}", query_), None);
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
     * This function performs a `POST` to the `/files.remote.update` endpoint.
     *
     * Updates an existing remote file.
     *
     * FROM: <https://api.slack.com/methods/files.remote.update>
     */
    pub async fn update(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/files.remote.update", None);
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
