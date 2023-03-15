use crate::Client;
use crate::ClientResult;

pub struct ChatScheduledMessages {
    pub client: Client,
}

impl ChatScheduledMessages {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ChatScheduledMessages { client }
    }

    /**
     * This function performs a `GET` to the `/chat.scheduledMessages.list` endpoint.
     *
     * Returns a list of scheduled messages.
     *
     * FROM: <https://api.slack.com/methods/chat.scheduledMessages.list>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `none`.
     * * `channel: &str` -- The channel of the scheduled messages.
     * * `latest: f64` -- A UNIX timestamp of the latest value in the time range.
     * * `oldest: f64` -- A UNIX timestamp of the oldest value in the time range.
     * * `limit: i64` -- Maximum number of original entries to return.
     * * `cursor: &str` -- For pagination purposes, this is the `cursor` value returned from a previous call to `chat.scheduledmessages.list` indicating where you want to start this call from.
     */
    pub async fn list(
        &self,
        channel: &str,
        latest: f64,
        oldest: f64,
        limit: i64,
        cursor: &str,
    ) -> ClientResult<crate::types::ChatScheduledMessagesListSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !channel.is_empty() {
            query_args.push(("channel".to_string(), channel.to_string()));
        }
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if !latest.to_string().is_empty() {
            query_args.push(("latest".to_string(), latest.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !oldest.to_string().is_empty() {
            query_args.push(("oldest".to_string(), oldest.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/chat.scheduledMessages.list?{}", query_), None);
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
