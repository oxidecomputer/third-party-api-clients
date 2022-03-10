use anyhow::Result;

use crate::Client;

pub struct ChatMessages {
    pub client: Client,
}

impl ChatMessages {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ChatMessages { client }
    }

    /**
     * List user's chat messages.
     *
     * This function performs a `GET` to the `/chat/users/{userId}/messages` endpoint.
     *
     * Use this API to list the current user's chat messages between the user and an individual contact or a chat channel. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * In the query parameter, you must provide one of the following:
     *
     * * `to_contact`: The email address of the contact with whom the user conversed by sending or receiving messages.
     * * `to_channel`: The channel ID of the channel to or from which the user has sent and/or received messages.
     *
     * <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note:</b> For an <b>account-level</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>, this API can only be used on behalf of a user who is assigned with a <a href="https://support.zoom.us/hc/en-us/articles/115001078646-Using-role-management#:~:text=Each%20user%20in%20a%20Zoom,owner%2C%20administrator%2C%20or%20member.&text=Role%2Dbased%20access%20control%20enables,needs%20to%20view%20or%20edit.">role</a> that has the <b>View</b> or <b>Edit</b> permission for <b>Chat Messages</b>.</p>
     *
     * **Scopes:** `chat_message:read`, `chat_message:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `to_contact: &str` -- The email address of a chat contact with whom the current user chatted. Messages that were sent and/or received between the user and the contact is displayed.
     *   
     *   Note: You must provide either `contact` or `channel` as a query parameter to retrieve messages either from an individual or a chat channel. .
     * * `to_channel: &str` -- The channel Id of a channel inside which the current user had chat conversations. Messages that were sent and/or received between the user and the channel is displayed.
     *   
     *   Note: You must provide either `contact` or `channel` as a query parameter to retrieve messages either from an individual or a chat channel. .
     * * `date: chrono::NaiveDate` -- The query date for which you would like to get the chat messages. This value defaults to the current date.
     * * `page_size: i64` -- The number of records returned with a single API call. .
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `include_deleted_and_edited_message: &str` -- **Optional** <br>
     *   Set the value of this field to `true` to include edited and deleted messages in the response.
     */
    pub async fn get_page(
        &self,
        user_id: &str,
        to_contact: &str,
        to_channel: &str,
        date: chrono::NaiveDate,
        page_size: i64,
        next_page_token: &str,
        include_deleted_and_edited_message: &str,
    ) -> Result<Vec<crate::types::Messages>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !date.to_string().is_empty() {
            query_args.push(("date".to_string(), date.to_string()));
        }
        if !include_deleted_and_edited_message.is_empty() {
            query_args.push((
                "include_deleted_and_edited_message".to_string(),
                include_deleted_and_edited_message.to_string(),
            ));
        }
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !to_channel.is_empty() {
            query_args.push(("to_channel".to_string(), to_channel.to_string()));
        }
        if !to_contact.is_empty() {
            query_args.push(("to_contact".to_string(), to_contact.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/chat/users/{}/messages?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        let resp: crate::types::GetChatMessagesResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.messages)
    }

    /**
     * List user's chat messages.
     *
     * This function performs a `GET` to the `/chat/users/{userId}/messages` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * Use this API to list the current user's chat messages between the user and an individual contact or a chat channel. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * In the query parameter, you must provide one of the following:
     *
     * * `to_contact`: The email address of the contact with whom the user conversed by sending or receiving messages.
     * * `to_channel`: The channel ID of the channel to or from which the user has sent and/or received messages.
     *
     * <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note:</b> For an <b>account-level</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>, this API can only be used on behalf of a user who is assigned with a <a href="https://support.zoom.us/hc/en-us/articles/115001078646-Using-role-management#:~:text=Each%20user%20in%20a%20Zoom,owner%2C%20administrator%2C%20or%20member.&text=Role%2Dbased%20access%20control%20enables,needs%20to%20view%20or%20edit.">role</a> that has the <b>View</b> or <b>Edit</b> permission for <b>Chat Messages</b>.</p>
     *
     * **Scopes:** `chat_message:read`, `chat_message:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     */
    pub async fn get_all(
        &self,
        user_id: &str,
        to_contact: &str,
        to_channel: &str,
        date: chrono::NaiveDate,
        include_deleted_and_edited_message: &str,
    ) -> Result<Vec<crate::types::Messages>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !date.to_string().is_empty() {
            query_args.push(("date".to_string(), date.to_string()));
        }
        if !include_deleted_and_edited_message.is_empty() {
            query_args.push((
                "include_deleted_and_edited_message".to_string(),
                include_deleted_and_edited_message.to_string(),
            ));
        }
        if !to_channel.is_empty() {
            query_args.push(("to_channel".to_string(), to_channel.to_string()));
        }
        if !to_contact.is_empty() {
            query_args.push(("to_contact".to_string(), to_contact.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/chat/users/{}/messages?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        let mut resp: crate::types::GetChatMessagesResponse = self.client.get(&url, None).await?;

        let mut messages = resp.messages;
        let mut page = resp.next_page_token;

        // Paginate if we should.
        while !page.is_empty() {
            // Check if we already have URL params and need to concat the token.
            if !url.contains('?') {
                resp = self
                    .client
                    .get(&format!("{}?next_page_token={}", url, page), None)
                    .await?;
            } else {
                resp = self
                    .client
                    .get(&format!("{}&next_page_token={}", url, page), None)
                    .await?;
            }

            messages.append(&mut resp.messages);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(messages)
    }

    /**
     * Send a chat message.
     *
     * This function performs a `POST` to the `/chat/users/{userId}/messages` endpoint.
     *
     * Send chat messages on Zoom to either an individual user who is in your contact list or to a [channel](https://support.zoom.us/hc/en-us/articles/200912909-Getting-Started-With-Channels-Group-Messaging-) of which you are a member. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * To send a message to a contact, provide the contact's email address in the `to_contact` field. To send a message to a channel, provide the channel's ID in `to_channel` parameter
     *
     * **Scopes:** `chat_message:write`, `chat_message:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     *  <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note:</b> For an <b>account-level</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>, this API can only be used on behalf of a user who is assigned with a <a href="https://support.zoom.us/hc/en-us/articles/115001078646-Using-role-management#:~:text=Each%20user%20in%20a%20Zoom,owner%2C%20administrator%2C%20or%20member.&text=Role%2Dbased%20access%20control%20enables,needs%20to%20view%20or%20edit."> role</a> that has the <b>Edit</b> permission for <b>Chat Messages</b>.</p>
     */
    pub async fn senda(
        &self,
        user_id: &str,
        body: &crate::types::SendaChatMessageRequest,
    ) -> Result<crate::types::Groups> {
        let url = format!(
            "/chat/users/{}/messages",
            crate::progenitor_support::encode_path(&user_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Mark message read or unread.
     *
     * This function performs a `PATCH` to the `/chat/users/{userId}/messages/{messageId}/status` endpoint.
     *
     * Mark a message as read or unread. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note:</b> For an <b>account-level</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>, this API can only be used on behalf of a user who is assigned with a <a href="https://support.zoom.us/hc/en-us/articles/115001078646-Using-role-management#:~:text=Each%20user%20in%20a%20Zoom,owner%2C%20administrator%2C%20or%20member.&text=Role%2Dbased%20access%20control%20enables,needs%20to%20view%20or%20edit.">role</a> that has the <b>Edit</b> permission for <b>Chat Messages</b>.</p>
     *
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- Unique identifier of the user.
     * * `message_id: &str` -- Unique identifier of the message.
     */
    pub async fn mark_message(
        &self,
        user_id: &str,
        message_id: &str,
        body: &crate::types::MarkMessageRequest,
    ) -> Result<()> {
        let url = format!(
            "/chat/users/{}/messages/{}/status",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&message_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * React to a chat message.
     *
     * This function performs a `PATCH` to the `/chat/users/{userId}/messages/{messageId}/emoji_reactions` endpoint.
     *
     * Use this API to react (add or remove) to a chat message with an emoji.
     *
     * For an **account-level** [OAuth app](https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app), this API can only be used on behalf of a user who is assigned with a [role](https://support.zoom.us/hc/en-us/articles/115001078646-Using-role-management#:~:text=Each%20user%20in%20a%20Zoom,owner%2C%20administrator%2C%20or%20member.&text=Role%2Dbased%20access%20control%20enables,needs%20to%20view%20or%20edit.) that has the **Edit** permission for **Chat Messages**.
     *
     * **Scopes:** `chat_message:write`, `chat_message:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user's unique ID.
     * * `message_id: &str` -- The message's unique ID.
     */
    pub async fn react_message(
        &self,
        user_id: &str,
        message_id: &str,
        body: &crate::types::ReactMessageRequest,
    ) -> Result<()> {
        let url = format!(
            "/chat/users/{}/messages/{}/emoji_reactions",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&message_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Get a message.
     *
     * This function performs a `GET` to the `/chat/users/{userId}/messages/{messageId}` endpoint.
     *
     * Get a chat message previously sent to a contact or a channel. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * You must provide one of the following query parameters:<br>
     * * `to_contact` — The email address of the Zoom contact to whom you sent the message.
     * * `to_channel` — The ID of the Zoom channel where you sent the message.
     *
     * **Scopes:** `chat_message:read`, `chat_message:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note:</b> For an <b>account-level</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>, you can only use this API for a user assigned <a href="https://support.zoom.us/hc/en-us/articles/115001078646-Using-role-management#:~:text=Each%20user%20in%20a%20Zoom,owner%2C%20administrator%2C%20or%20member.&text=Role%2Dbased%20access%20control%20enables,needs%20to%20view%20or%20edit.">the <b>Edit</b> permission for the <b>Chat message</b> role setting</a>.</p>
     *
     * **Parameters:**
     *
     * * `message_id: &str` -- User's first name.
     * * `to_contact: &str` -- The `userId` or email address of a Zoom Chat contact to whom you sent the message.\n\n**Note:** You must use this query parameter to delete a message sent to a Zoom Chat contact. .
     * * `to_channel: &str` -- The `channelId` of the Zoom Chat channel where sent the message.\n\n**Note:** You must use this query parameter to delete a message sent to Zoom Chat channel.
     */
    pub async fn get(
        &self,
        user_id: &str,
        message_id: &str,
        to_contact: &str,
        to_channel: &str,
    ) -> Result<crate::types::GetChatMessageResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !to_channel.is_empty() {
            query_args.push(("to_channel".to_string(), to_channel.to_string()));
        }
        if !to_contact.is_empty() {
            query_args.push(("to_contact".to_string(), to_contact.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/chat/users/{}/messages/{}?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&message_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Update a message.
     *
     * This function performs a `PUT` to the `/chat/users/{userId}/messages/{messageId}` endpoint.
     *
     * Use this API to edit a chat message that you previously sent to either a contact or a channel in Zoom by providing the ID of the message as the value of the `messageId` parameter. You can get the ID from the **List User's Chat Messages** API. Additionally, as a query parameter, you must provide either the contact's **email address** of the contact or the **Channel ID** of the channel where the message was sent.
     *
     * For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note:</b> For an <b>account-level</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>, this API can only be used on behalf of a user who is assigned with a <a href="https://support.zoom.us/hc/en-us/articles/115001078646-Using-role-management#:~:text=Each%20user%20in%20a%20Zoom,owner%2C%20administrator%2C%20or%20member.&text=Role%2Dbased%20access%20control%20enables,needs%20to%20view%20or%20edit."> role</a> that has the <b>Edit</b> permission for <b>Chat Messages</b>.</p>
     *
     * **Scope:** `chat_message:write`,`chat_message:write:admin`	<br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `message_id: &str` -- Message ID: Unique Identifier of the message.
     */
    pub async fn edit_message(
        &self,
        user_id: &str,
        message_id: &str,
        body: &crate::types::EditMessageRequest,
    ) -> Result<()> {
        let url = format!(
            "/chat/users/{}/messages/{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&message_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete a message.
     *
     * This function performs a `DELETE` to the `/chat/users/{userId}/messages/{messageId}` endpoint.
     *
     * Delete a chat message that you previously sent to a contact or a channel. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * In the query parameter, you must provide either of the following:
     *
     * * `to_contact`: The email address of the contact to whom you sent the message. Use this parameter to delete a message sent to an individual contact in Zoom.
     * * `to_channel`: The channel ID of the channel where you sent the message. Use this parameter to delete a message sent to a channel in Zoom.
     *
     *
     *
     * **Scopes:** `chat_message:write`, `chat_message:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note:</b> For an <b>account-level</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>, this API can only be used on behalf of a user who is assigned with a <a href="https://support.zoom.us/hc/en-us/articles/115001078646-Using-role-management#:~:text=Each%20user%20in%20a%20Zoom,owner%2C%20administrator%2C%20or%20member.&text=Role%2Dbased%20access%20control%20enables,needs%20to%20view%20or%20edit."> role</a> that has the <b>Edit</b> permission for <b>Chat Messages</b>.</p>
     *
     * **Parameters:**
     *
     * * `message_id: &str` -- User's first name.
     * * `to_contact: &str` -- The userId or email address of a chat contact to whom you previously sent the message.
     *   
     *   Note: You must provide either `to_contact` or `to_channel` as a query parameter to delete a message that was previously sent to either an individual or a chat channel respectively. .
     * * `to_channel: &str` -- The channel Id of the channel where you would like to send the message.
     *   
     *   You must provide either `to_contact` or `to_channel` as a query parameter to delete a message that was previously sent to either an individual or a chat channel .
     */
    pub async fn delete(
        &self,
        user_id: &str,
        message_id: &str,
        to_contact: &str,
        to_channel: &str,
    ) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !to_channel.is_empty() {
            query_args.push(("to_channel".to_string(), to_channel.to_string()));
        }
        if !to_contact.is_empty() {
            query_args.push(("to_contact".to_string(), to_contact.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/chat/users/{}/messages/{}?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&message_id.to_string()),
            query_
        );

        self.client.delete(&url, None).await
    }
}
