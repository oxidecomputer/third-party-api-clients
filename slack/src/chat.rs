use crate::Client;
use crate::ClientResult;

pub struct Chat {
    pub client: Client,
}

impl Chat {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Chat { client }
    }

    /**
     * This function performs a `POST` to the `/chat.delete` endpoint.
     *
     * Deletes a message.
     *
     * FROM: <https://api.slack.com/methods/chat.delete>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `chat:write`.
     */
    pub async fn delete(&self) -> ClientResult<crate::types::ChatDeleteSuccessSchema> {
        let url = self.client.url("/chat.delete", None);
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
     * This function performs a `POST` to the `/chat.deleteScheduledMessage` endpoint.
     *
     * Deletes a pending scheduled message from the queue.
     *
     * FROM: <https://api.slack.com/methods/chat.deleteScheduledMessage>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `chat:write`.
     */
    pub async fn delete_scheduled_message(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/chat.deleteScheduledMessage", None);
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
     * This function performs a `GET` to the `/chat.getPermalink` endpoint.
     *
     * Retrieve a permalink URL for a specific extant message
     *
     * FROM: <https://api.slack.com/methods/chat.getPermalink>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `none`.
     * * `channel: &str` -- The ID of the conversation or channel containing the message.
     * * `message_ts: &str` -- A message's `ts` value, uniquely identifying it within a channel.
     */
    pub async fn get_permalink(
        &self,
        channel: &str,
        message_ts: &str,
    ) -> ClientResult<crate::types::ChatGetPermalinkSuccessSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !channel.is_empty() {
            query_args.push(("channel".to_string(), channel.to_string()));
        }
        if !message_ts.is_empty() {
            query_args.push(("message_ts".to_string(), message_ts.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/chat.getPermalink?{}", query_), None);
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
     * This function performs a `POST` to the `/chat.meMessage` endpoint.
     *
     * Share a me message into a channel.
     *
     * FROM: <https://api.slack.com/methods/chat.meMessage>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `chat:write`.
     */
    pub async fn me_message(&self) -> ClientResult<crate::types::ChatMeMessageSchema> {
        let url = self.client.url("/chat.meMessage", None);
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
     * This function performs a `POST` to the `/chat.postEphemeral` endpoint.
     *
     * Sends an ephemeral message to a user in a channel.
     *
     * FROM: <https://api.slack.com/methods/chat.postEphemeral>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `chat:write`.
     */
    pub async fn post_ephemeral(
        &self,
    ) -> ClientResult<crate::types::ChatPostEphemeralSuccessSchema> {
        let url = self.client.url("/chat.postEphemeral", None);
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
     * This function performs a `POST` to the `/chat.postMessage` endpoint.
     *
     * Sends a message to a channel.
     *
     * FROM: <https://api.slack.com/methods/chat.postMessage>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `chat:write`.
     */
    pub async fn post_message(&self) -> ClientResult<crate::types::ChatPostMessageSuccessSchema> {
        let url = self.client.url("/chat.postMessage", None);
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
     * This function performs a `POST` to the `/chat.scheduleMessage` endpoint.
     *
     * Schedules a message to be sent to a channel.
     *
     * FROM: <https://api.slack.com/methods/chat.scheduleMessage>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `chat:write`.
     */
    pub async fn schedule_message(
        &self,
    ) -> ClientResult<crate::types::ChatScheduleMessageSuccessSchema> {
        let url = self.client.url("/chat.scheduleMessage", None);
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
     * This function performs a `POST` to the `/chat.unfurl` endpoint.
     *
     * Provide custom unfurl behavior for user-posted URLs
     *
     * FROM: <https://api.slack.com/methods/chat.unfurl>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `links:write`.
     */
    pub async fn unfurl(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/chat.unfurl", None);
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
     * This function performs a `POST` to the `/chat.update` endpoint.
     *
     * Updates a message.
     *
     * FROM: <https://api.slack.com/methods/chat.update>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `chat:write`.
     */
    pub async fn update(&self) -> ClientResult<crate::types::ChatUpdateSuccessSchema> {
        let url = self.client.url("/chat.update", None);
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
