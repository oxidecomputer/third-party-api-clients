use crate::Client;
use crate::ClientResult;

pub struct FilesComments {
    pub client: Client,
}

impl FilesComments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        FilesComments { client }
    }

    /**
     * This function performs a `POST` to the `/files.comments.delete` endpoint.
     *
     * Deletes an existing comment on a file.
     *
     * FROM: <https://api.slack.com/methods/files.comments.delete>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `files:write:user`.
     */
    pub async fn delete(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/files.comments.delete", None);
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
