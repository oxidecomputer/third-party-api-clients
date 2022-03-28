use anyhow::Result;

use crate::Client;

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
    pub async fn delete(&self) -> Result<crate::types::DndEndSchema> {
        let url = "/files.comments.delete".to_string();
        self.client.post(&url, None).await
    }
}
