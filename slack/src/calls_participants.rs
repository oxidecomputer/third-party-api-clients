use crate::Client;
use crate::ClientResult;

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
    pub async fn add(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/calls.participants.add", None);
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
    pub async fn remove(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/calls.participants.remove", None);
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
