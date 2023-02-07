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
    * List conversations.
    *
    * This function performs a `GET` to the `/conversations` endpoint.
    *
    * Get a list of conversations for the account. Conversations has been deprecated in favor of Inbox and these endpoints don't include Inbox data. Past Conversations are still available via this endpoint, but new campaign replies and other Inbox messages aren’t available using this endpoint.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `has_unread_messages: crate::types::IsRead` -- Whether a conversation message has been marked as read.
    * * `list_id: &str` -- The unique id for the list.
    * * `campaign_id: &str` -- The unique id for the campaign.
    */
    pub async fn get(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        count: i64,
        offset: i64,
        has_unread_messages: crate::types::IsRead,
        list_id: &str,
        campaign_id: &str,
    ) -> Result<crate::types::TrackedConversations> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !campaign_id.is_empty() {
            query_args.push(("campaign_id".to_string(), campaign_id.to_string()));
        }
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if !has_unread_messages.to_string().is_empty() {
            query_args.push((
                "has_unread_messages".to_string(),
                has_unread_messages.to_string(),
            ));
        }
        if !list_id.is_empty() {
            query_args.push(("list_id".to_string(), list_id.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/conversations?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * Get conversation.
    *
    * This function performs a `GET` to the `/conversations/{conversation_id}` endpoint.
    *
    * Get details about an individual conversation. Conversations has been deprecated in favor of Inbox and these endpoints don't include Inbox data. Past Conversations are still available via this endpoint, but new campaign replies and other Inbox messages aren’t available using this endpoint.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `conversation_id: &str` -- The unique id for the conversation.
    */
    pub async fn get_conversations(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        conversation_id: &str,
    ) -> Result<crate::types::Conversation> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/conversations/{}?{}",
            crate::progenitor_support::encode_path(&conversation_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * List messages.
    *
    * This function performs a `GET` to the `/conversations/{conversation_id}/messages` endpoint.
    *
    * Get messages from a specific conversation. Conversations has been deprecated in favor of Inbox and these endpoints don't include Inbox data. Past Conversations are still available via this endpoint, but new campaign replies and other Inbox messages aren’t available using this endpoint.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `conversation_id: &str` -- The unique id for the conversation.
    * * `is_read: crate::types::IsRead` -- Whether a conversation message has been marked as read.
    * * `before_timestamp: chrono::DateTime<chrono::Utc>` -- Restrict the response to messages created before the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `since_timestamp: chrono::DateTime<chrono::Utc>` -- Restrict the response to messages created after the set time. Uses ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    */
    pub async fn get_message(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        conversation_id: &str,
        is_read: crate::types::IsRead,
        before_timestamp: Option<chrono::DateTime<chrono::Utc>>,
        since_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<crate::types::CollectionOfConversationMessages> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if let Some(date) = before_timestamp {
            query_args.push(("before_timestamp".to_string(), date.to_rfc3339()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if !is_read.to_string().is_empty() {
            query_args.push(("is_read".to_string(), is_read.to_string()));
        }
        if let Some(date) = since_timestamp {
            query_args.push(("since_timestamp".to_string(), date.to_rfc3339()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/conversations/{}/messages?{}",
            crate::progenitor_support::encode_path(&conversation_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Get message.
    *
    * This function performs a `GET` to the `/conversations/{conversation_id}/messages/{message_id}` endpoint.
    *
    * Get an individual message in a conversation. Conversations has been deprecated in favor of Inbox and these endpoints don't include Inbox data. Past Conversations are still available via this endpoint, but new campaign replies and other Inbox messages aren’t available using this endpoint.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `conversation_id: &str` -- The unique id for the conversation.
    * * `message_id: &str` -- The unique id for the conversation message.
    */
    pub async fn get_message_conversations(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        conversation_id: &str,
        message_id: &str,
    ) -> Result<crate::types::ConversationMessage> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/conversations/{}/messages/{}?{}",
            crate::progenitor_support::encode_path(&conversation_id.to_string()),
            crate::progenitor_support::encode_path(&message_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }
}
