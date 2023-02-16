use anyhow::Result;

use crate::Client;

pub struct CallsParticipants {
    pub client: Client,
}

impl CallsParticipants {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CallsParticipants { client }
    }

    /**
     * This function performs a `POST` to the `/calls.participants.add` endpoint.
     *
     * Registers new participants added to a Call.
     *
     * FROM: <https://api.slack.com/methods/calls.participants.add>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `calls:write`.
     */
    pub async fn add(&self) -> Result<crate::types::DndEndSchema> {
        let url = "/calls.participants.add".to_string();
        let url = self.client.url(&url, None);
        self.client.post(&url, None).await
    }
    /**
     * This function performs a `POST` to the `/calls.participants.remove` endpoint.
     *
     * Registers participants removed from a Call.
     *
     * FROM: <https://api.slack.com/methods/calls.participants.remove>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `calls:write`.
     */
    pub async fn remove(&self) -> Result<crate::types::DndEndSchema> {
        let url = "/calls.participants.remove".to_string();
        let url = self.client.url(&url, None);
        self.client.post(&url, None).await
    }
}
