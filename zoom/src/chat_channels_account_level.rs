use anyhow::Result;

use crate::Client;

pub struct ChatChannelsAccountLevel {
    pub client: Client,
}

impl ChatChannelsAccountLevel {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ChatChannelsAccountLevel { client }
    }

    /**
     * Get a channel.
     *
     * This function performs a `GET` to the `/chat/users/{userId}/channels/{channelId}` endpoint.
     *
     * Use this API to get information about a specific channel. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * Zoom chat [channels](https://support.zoom.us/hc/en-us/articles/200912909-Getting-Started-With-Channels-Group-Messaging-) allow users to communicate via chat in private or public groups.
     *
     * **Scopes:** `chat_channel:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note:</b> For an <b>account-level</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>, this API can only be used on behalf of a user who is assigned with a <a href="https://support.zoom.us/hc/en-us/articles/115001078646-Using-role-management#:~:text=Each%20user%20in%20a%20Zoom,owner%2C%20administrator%2C%20or%20member.&text=Role%2Dbased%20access%20control%20enables,needs%20to%20view%20or%20edit."> role</a>  that has the <b>View</b> or <b>Edit</b> permission for <b>Chat Channels</b>.</p>
     *
     * **Parameters:**
     *
     * * `channel_id: &str` -- Channel ID: Unique Identifier of a channel.
     * * `user_id: &str` -- Unique identifier of the user who is the owner of the channel.
     */
    pub async fn get_channel(
        &self,
        user_id: &str,
        channel_id: &str,
    ) -> Result<crate::types::Channel> {
        let url = format!(
            "/chat/users/{}/channels/{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&channel_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a channel.
     *
     * This function performs a `DELETE` to the `/chat/users/{userId}/channels/{channelId}` endpoint.
     *
     * Use this API to delete a specific channel. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * Zoom chat [channels](https://support.zoom.us/hc/en-us/articles/200912909-Getting-Started-With-Channels-Group-Messaging-) allow users to communicate via chat in private or public groups.
     *
     * **Scopes:** `chat_channel:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note:</b> This API only supports a <b>user-managed</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>.</p>
     *
     * **Parameters:**
     *
     * * `channel_id: &str` -- Channel ID: Unique Identifier of a channel.
     */
    pub async fn delete_channel(&self, user_id: &str, channel_id: &str) -> Result<()> {
        let url = format!(
            "/chat/users/{}/channels/{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&channel_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
     * Update a channel.
     *
     * This function performs a `PATCH` to the `/chat/users/{userId}/channels/{channelId}` endpoint.
     *
     * Use this API to update the name of a specific channel created by a user. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * Zoom chat channels allow users to communicate via chat in private or public channels.
     *
     * **Scopes:** `chat_channel:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note:</b> For an <b>account-level</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>, this API can only be used on behalf of a user who is assigned with a <a href="https://support.zoom.us/hc/en-us/articles/115001078646-Using-role-management#:~:text=Each%20user%20in%20a%20Zoom,owner%2C%20administrator%2C%20or%20member.&text=Role%2Dbased%20access%20control%20enables,needs%20to%20view%20or%20edit."> role</a> that has the <b>Edit</b> permission for <b>Chat Channel</b>.</p>
     *
     * **Parameters:**
     *
     * * `channel_id: &str` -- User's first name.
     * * `user_id: &str` -- Unique Identifier of the Zoom user who is the owner of the channel.
     */
    pub async fn update_channel(
        &self,
        user_id: &str,
        channel_id: &str,
        body: &crate::types::Attendees,
    ) -> Result<()> {
        let url = format!(
            "/chat/users/{}/channels/{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&channel_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * List channel members.
     *
     * This function performs a `GET` to the `/chat/users/{userId}/channels/{channelId}/members` endpoint.
     *
     * Use this API to list all members of a channel. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `chat_channel:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note:</b> For an<b> account-level</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>, this API can only be used on behalf of a user who is assigned with a <a href="https://support.zoom.us/hc/en-us/articles/115001078646-Using-role-management#:~:text=Each%20user%20in%20a%20Zoom,owner%2C%20administrator%2C%20or%20member.&text=Role%2Dbased%20access%20control%20enables,needs%20to%20view%20or%20edit."> role</a> that has the <b>View</b> or <b>Edit</b> permission for <b>Chat Channels</b>.</p>
     *
     * **Parameters:**
     *
     * * `channel_id: &str` -- User's first name.
     * * `page_size: i64` -- The number of records returned with a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     * * `user_id: &str` -- Unique identifier of the user who is the owner of this channel.
     */
    pub async fn list_channel_members(
        &self,
        user_id: &str,
        channel_id: &str,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<Vec<crate::types::ListChannelMembersResponse>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/chat/users/{}/channels/{}/members?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&channel_id.to_string()),
            query_
        );

        let resp: crate::types::ListChannelMembersResponseData =
            self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.members)
    }

    /**
     * List channel members.
     *
     * This function performs a `GET` to the `/chat/users/{userId}/channels/{channelId}/members` endpoint.
     *
     * As opposed to `list_channel_members`, this function returns all the pages of the request at once.
     *
     * Use this API to list all members of a channel. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `chat_channel:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note:</b> For an<b> account-level</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>, this API can only be used on behalf of a user who is assigned with a <a href="https://support.zoom.us/hc/en-us/articles/115001078646-Using-role-management#:~:text=Each%20user%20in%20a%20Zoom,owner%2C%20administrator%2C%20or%20member.&text=Role%2Dbased%20access%20control%20enables,needs%20to%20view%20or%20edit."> role</a> that has the <b>View</b> or <b>Edit</b> permission for <b>Chat Channels</b>.</p>
     */
    pub async fn list_all_channel_members(
        &self,
        user_id: &str,
        channel_id: &str,
    ) -> Result<Vec<crate::types::ListChannelMembersResponse>> {
        let url = format!(
            "/chat/users/{}/channels/{}/members",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&channel_id.to_string()),
        );

        let mut resp: crate::types::ListChannelMembersResponseData =
            self.client.get(&url, None).await?;

        let mut members = resp.members;
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

            members.append(&mut resp.members);

            if !resp.next_page_token.is_empty() && resp.next_page_token != page {
                page = resp.next_page_token.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(members)
    }

    /**
     * Invite channel members.
     *
     * This function performs a `POST` to the `/chat/users/{userId}/channels/{channelId}/members` endpoint.
     *
     * Use this API to invite members that are in a user's contact list to a channel. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `chat_channel:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note:</b> For an <b>account-level</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>, this API can only be used on behalf of a user who is assigned with a <a href="https://support.zoom.us/hc/en-us/articles/115001078646-Using-role-management#:~:text=Each%20user%20in%20a%20Zoom,owner%2C%20administrator%2C%20or%20member.&text=Role%2Dbased%20access%20control%20enables,needs%20to%20view%20or%20edit."> role</a> that has the <b>Edit</b> permission for <b>Chat Channels</b>.</p>
     *
     * **Parameters:**
     *
     * * `channel_id: &str` -- Channel ID: Unique Identifier of the channel.
     * * `user_id: &str` -- Unique identifier of the user who is the owner of this channel.
     */
    pub async fn invite_channel_members(
        &self,
        user_id: &str,
        channel_id: &str,
        body: &crate::types::InviteChannelMembersRequest,
    ) -> Result<crate::types::InviteChannelMembersResponse> {
        let url = format!(
            "/chat/users/{}/channels/{}/members",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&channel_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Remove a member.
     *
     * This function performs a `DELETE` to the `/chat/users/{userId}/channels/{channelId}/members/{memberId}` endpoint.
     *
     *  A [channel](https://support.zoom.us/hc/en-us/articles/200912909-Getting-Started-With-Channels-Group-Messaging-) can have one or multiple members. Use this API to remove a member from a chat channel. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
     *
     * **Scopes:** `chat_channel:write:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium` <br>
     *  
     *  
     * <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note: </b> For an<b> account-level</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>, this API can only be used on behalf of a user who is assigned with a <b><a href="https://support.zoom.us/hc/en-us/articles/115001078646-Using-role-management#:~:text=Each%20user%20in%20a%20Zoom,owner%2C%20administrator%2C%20or%20member.&text=Role%2Dbased%20access%20control%20enables,needs%20to%20view%20or%20edit."> role</a> that has Edit permission for Chat Channels</b>.</p>
     *
     * **Parameters:**
     *
     * * `channel_id: &str` -- Unique Identifier of the Channel from where you would like to remove a member. This can be retrieved from the [List Channels API](https://marketplace.zoom.us/docs/api-reference/zoom-api/chat-channels/getchannels).
     * * `member_id: &str` -- Email address of the member whom you would like to be remove from the channel.
     * * `user_id: &str` -- Unique identifier of the channel owner.
     */
    pub async fn remove_channel_member(
        &self,
        user_id: &str,
        channel_id: &str,
        member_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/chat/users/{}/channels/{}/members/{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            crate::progenitor_support::encode_path(&channel_id.to_string()),
            crate::progenitor_support::encode_path(&member_id.to_string()),
        );

        self.client.delete(&url, None).await
    }
}
