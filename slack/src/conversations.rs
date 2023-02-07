use anyhow::Result;

use crate::Client;

pub struct Conversations {
    pub client: Client,
}

impl Conversations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Conversations { client }
    }

    /**
    * This function performs a `POST` to the `/conversations.archive` endpoint.
    *
    * Archives a conversation.
    *
    * FROM: <https://api.slack.com/methods/conversations.archive>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `conversations:write`.
    */
    pub async fn archive(&self) -> Result<crate::types::DndEndSchema> {
        let url = "/conversations.archive".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/conversations.close` endpoint.
    *
    * Closes a direct message or multi-person direct message.
    *
    * FROM: <https://api.slack.com/methods/conversations.close>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `conversations:write`.
    */
    pub async fn close(&self) -> Result<crate::types::ConversationsCloseSuccessSchema> {
        let url = "/conversations.close".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/conversations.create` endpoint.
    *
    * Initiates a public or private channel-based conversation
    *
    * FROM: <https://api.slack.com/methods/conversations.create>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `conversations:write`.
    */
    pub async fn create(&self) -> Result<crate::types::ConversationsInfoSuccessSchema> {
        let url = "/conversations.create".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/conversations.history` endpoint.
    *
    * Fetches a conversation's history of messages and events.
    *
    * FROM: <https://api.slack.com/methods/conversations.history>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `conversations:history`.
    * * `channel: &str` -- Conversation ID to fetch history for.
    * * `latest: f64` -- End of time range of messages to include in results.
    * * `oldest: f64` -- Start of time range of messages to include in results.
    * * `inclusive: bool` -- Include messages with latest or oldest timestamp in results only when either timestamp is specified.
    * * `limit: i64` -- The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached.
    * * `cursor: &str` -- Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](/docs/pagination) for more detail.
    */
    pub async fn history(
        &self,
        channel: &str,
        latest: f64,
        oldest: f64,
        inclusive: bool,
        limit: i64,
        cursor: &str,
    ) -> Result<crate::types::ConversationsHistorySuccessSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !channel.is_empty() {
            query_args.push(("channel".to_string(), channel.to_string()));
        }
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if inclusive {
            query_args.push(("inclusive".to_string(), inclusive.to_string()));
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
        let url = format!("/conversations.history?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/conversations.info` endpoint.
    *
    * Retrieve information about a conversation.
    *
    * FROM: <https://api.slack.com/methods/conversations.info>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `conversations:read`.
    * * `channel: &str` -- Conversation ID to learn more about.
    * * `include_locale: bool` -- Set this to `true` to receive the locale for this conversation. Defaults to `false`.
    * * `include_num_members: bool` -- Set to `true` to include the member count for the specified conversation. Defaults to `false`.
    */
    pub async fn info(
        &self,
        channel: &str,
        include_locale: bool,
        include_num_members: bool,
    ) -> Result<crate::types::ConversationsInfoSuccessSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !channel.is_empty() {
            query_args.push(("channel".to_string(), channel.to_string()));
        }
        if include_locale {
            query_args.push(("include_locale".to_string(), include_locale.to_string()));
        }
        if include_num_members {
            query_args.push((
                "include_num_members".to_string(),
                include_num_members.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/conversations.info?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/conversations.invite` endpoint.
    *
    * Invites users to a channel.
    *
    * FROM: <https://api.slack.com/methods/conversations.invite>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `conversations:write`.
    */
    pub async fn invite(&self) -> Result<crate::types::ConversationsInfoSuccessSchema> {
        let url = "/conversations.invite".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/conversations.join` endpoint.
    *
    * Joins an existing conversation.
    *
    * FROM: <https://api.slack.com/methods/conversations.join>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `channels:write`.
    */
    pub async fn join(&self) -> Result<crate::types::ConversationsJoinSuccessSchema> {
        let url = "/conversations.join".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/conversations.kick` endpoint.
    *
    * Removes a user from a conversation.
    *
    * FROM: <https://api.slack.com/methods/conversations.kick>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `conversations:write`.
    */
    pub async fn kick(&self) -> Result<crate::types::DndEndSchema> {
        let url = "/conversations.kick".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/conversations.leave` endpoint.
    *
    * Leaves a conversation.
    *
    * FROM: <https://api.slack.com/methods/conversations.leave>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `conversations:write`.
    */
    pub async fn leave(&self) -> Result<crate::types::ConversationsLeaveSuccessSchema> {
        let url = "/conversations.leave".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/conversations.list` endpoint.
    *
    * Lists all channels in a Slack team.
    *
    * FROM: <https://api.slack.com/methods/conversations.list>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `conversations:read`.
    * * `exclude_archived: bool` -- Set to `true` to exclude archived channels from the list.
    * * `types: &str` -- Mix and match channel types by providing a comma-separated list of any combination of `public_channel`, `private_channel`, `mpim`, `im`.
    * * `limit: i64` -- The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. Must be an integer no larger than 1000.
    * * `cursor: &str` -- Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](/docs/pagination) for more detail.
    */
    pub async fn list(
        &self,
        exclude_archived: bool,
        types: &str,
        limit: i64,
        cursor: &str,
    ) -> Result<crate::types::ConversationsListSuccessSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if exclude_archived {
            query_args.push(("exclude_archived".to_string(), exclude_archived.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !types.is_empty() {
            query_args.push(("types".to_string(), types.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/conversations.list?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/conversations.mark` endpoint.
    *
    * Sets the read cursor in a channel.
    *
    * FROM: <https://api.slack.com/methods/conversations.mark>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `conversations:write`.
    */
    pub async fn mark(&self) -> Result<crate::types::DndEndSchema> {
        let url = "/conversations.mark".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/conversations.members` endpoint.
    *
    * Retrieve members of a conversation.
    *
    * FROM: <https://api.slack.com/methods/conversations.members>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `conversations:read`.
    * * `channel: &str` -- ID of the conversation to retrieve members for.
    * * `limit: i64` -- The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached.
    * * `cursor: &str` -- Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](/docs/pagination) for more detail.
    */
    pub async fn member(
        &self,
        channel: &str,
        limit: i64,
        cursor: &str,
    ) -> Result<crate::types::ConversationsMembersSuccessSchema> {
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
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/conversations.members?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/conversations.open` endpoint.
    *
    * Opens or resumes a direct message or multi-person direct message.
    *
    * FROM: <https://api.slack.com/methods/conversations.open>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `conversations:write`.
    */
    pub async fn open(&self) -> Result<crate::types::ConversationsOpenSuccessSchema> {
        let url = "/conversations.open".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/conversations.rename` endpoint.
    *
    * Renames a conversation.
    *
    * FROM: <https://api.slack.com/methods/conversations.rename>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `conversations:write`.
    */
    pub async fn rename(&self) -> Result<crate::types::ConversationsInfoSuccessSchema> {
        let url = "/conversations.rename".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `GET` to the `/conversations.replies` endpoint.
    *
    * Retrieve a thread of messages posted to a conversation
    *
    * FROM: <https://api.slack.com/methods/conversations.replies>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `conversations:history`.
    * * `channel: &str` -- Conversation ID to fetch thread from.
    * * `ts: f64` -- Unique identifier of a thread's parent message. `ts` must be the timestamp of an existing message with 0 or more replies. If there are no replies then just the single message referenced by `ts` will return - it is just an ordinary, unthreaded message.
    * * `latest: f64` -- End of time range of messages to include in results.
    * * `oldest: f64` -- Start of time range of messages to include in results.
    * * `inclusive: bool` -- Include messages with latest or oldest timestamp in results only when either timestamp is specified.
    * * `limit: i64` -- The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached.
    * * `cursor: &str` -- Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](/docs/pagination) for more detail.
    */
    pub async fn replie(
        &self,
        channel: &str,
        ts: f64,
        latest: f64,
        oldest: f64,
        inclusive: bool,
        limit: i64,
        cursor: &str,
    ) -> Result<crate::types::ConversationsRepliesSuccessSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !channel.is_empty() {
            query_args.push(("channel".to_string(), channel.to_string()));
        }
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if inclusive {
            query_args.push(("inclusive".to_string(), inclusive.to_string()));
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
        if !ts.to_string().is_empty() {
            query_args.push(("ts".to_string(), ts.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/conversations.replies?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/conversations.setPurpose` endpoint.
    *
    * Sets the purpose for a conversation.
    *
    * FROM: <https://api.slack.com/methods/conversations.setPurpose>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `conversations:write`.
    */
    pub async fn set_purpose(&self) -> Result<crate::types::ConversationsInfoSuccessSchema> {
        let url = "/conversations.setPurpose".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/conversations.setTopic` endpoint.
    *
    * Sets the topic for a conversation.
    *
    * FROM: <https://api.slack.com/methods/conversations.setTopic>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `conversations:write`.
    */
    pub async fn set_topic(&self) -> Result<crate::types::ConversationsInfoSuccessSchema> {
        let url = "/conversations.setTopic".to_string();
        self.client.post(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/conversations.unarchive` endpoint.
    *
    * Reverses conversation archival.
    *
    * FROM: <https://api.slack.com/methods/conversations.unarchive>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `conversations:write`.
    */
    pub async fn unarchive(&self) -> Result<crate::types::DndEndSchema> {
        let url = "/conversations.unarchive".to_string();
        self.client.post(&url, None).await
    }
}
