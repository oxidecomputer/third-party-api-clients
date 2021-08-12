use anyhow::Result;

use crate::Client;

pub struct ChatChannels {
    client: Client,
}

impl ChatChannels {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self
    {
        ChatChannels {
            client,
        }
    }

    /**
* List user's channels.
*
* This function performs a `GET` to the `/chat/users/{userId}/channels` endpoint.
*
* Use this API to list a user's chat channels. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
* 
* Zoom chat [channels](https://support.zoom.us/hc/en-us/articles/200912909-Getting-Started-With-Channels-Group-Messaging-) allow users to communicate via chat in private or public groups.
* 
* **Scopes:** `chat_channel:read` or `chat_channel:read:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
* 
* <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note:</b> This API supports both user-managed apps and account-level apps. However, in an <b>account-level</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>, to list channels of another user in the same Zoom account, the user calling this API must have a <a href="https://support.zoom.us/hc/en-us/articles/115001078646-Using-role-management#:~:text=Each%20user%20in%20a%20Zoom,owner%2C%20administrator%2C%20or%20member.&text=Role%2Dbased%20access%20control%20enables,needs%20to%20view%20or%20edit.">role</a> that has the <b>View</b> or <b>Edit</b> permission for the <b>Chat channels</b> feature.</p>
*
* **Parameters:**
*
* * `page_size: i64` -- The number of records returned from a single API call.
* * `next_page_token: &str` -- The next page token is used to paginate through large result sets. The expiration period for this token is 15 minutes.
* * `user_id: &str` -- Unique identifier of the user.
*/
pub async fn get_channels(
&self,
user_id: &str, page_size: i64, next_page_token: &str, user_id: &str,
) -> Result<crate::types::GetChannelsResponse> {
let mut query = String::new();
let mut query_args: Vec<String> = Default::default();
if !next_page_token.is_empty() { query_args.push(format!("next_page_token={}", next_page_token)); }
if page_size > 0 { query_args.push(format!("page_size={}", page_size)); }
for (i, n) in query_args.iter().enumerate() {
                    if i > 0 {
                        query.push('&');
                    }
                    query.push_str(n);
                }
let url =
format!("/chat/users/{}/channels?{}",
crate::progenitor_support::encode_path(&user_id.to_string()),query);

self.client.get(&url, None).await
}

/**
* Create a channel.
*
* This function performs a `POST` to the `/chat/users/{userId}/channels` endpoint.
*
* Use this API to create a channel for a user. For user-level apps, pass [the `me` value](https://marketplace.zoom.us/docs/api-reference/using-zoom-apis#mekeyword) instead of the `userId` parameter.
* 
* Zoom chat channels allow users to communicate via chat in private or public groups.
* 
* **Scopes:** `chat_channel:write` or `chat_channel:write:admin`<br>**[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
* 
* <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note:</b> This API supports both user-managed apps and account-level apps. However, in an <b>account-level</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>, to create a channel on behalf of another user in the same Zoom account, the user calling this API must have a <a href="https://support.zoom.us/hc/en-us/articles/115001078646-Using-role-management#:~:text=Each%20user%20in%20a%20Zoom,owner%2C%20administrator%2C%20or%20member.&text=Role%2Dbased%20access%20control%20enables,needs%20to%20view%20or%20edit.">role</a> that has the <b>Edit</b> permission for the <b>Chat channels</b> feature.</p>
*
* **Parameters:**
*
* * `user_id: &str` -- Unique identifier of the user.
*/
pub async fn create_channel(
&self,
user_id: &str, user_id: &str,
body: &crate::types::CreateChannelRequest
) -> Result<crate::types::CreateChannelResponse> {
let url =
format!("/chat/users/{}/channels",
crate::progenitor_support::encode_path(&user_id.to_string()),);

self.client.post(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Get a channel.
*
* This function performs a `GET` to the `/chat/channels/{channelId}` endpoint.
*
* Zoom chat [channels](https://support.zoom.us/hc/en-us/articles/200912909-Getting-Started-With-Channels-Group-Messaging-) allow users to communicate via chat in private or public groups. Use this API to get information about a specific channel. 
* 
* **Scope:** `chat_channel:read`	<br>
* **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
* 
* <p style="background-color:#e1f5fe; color:#000000; padding:8px"> <b>Note: </b> This API supports only <b>user-managed</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth apps</a>. If you use an Account-Level OAuth Access token, you can only retrieve the channel information for the authorized user. You can't retrieve the channel information of other account users. Use the <a href="https://marketplace.zoom.us/docs/api-reference/zoom-api/chat-channels-account-level/getchannel">Account-Level Get Channel Info API</a> to retrieve the channel information of other account users.</p><br>
* 
* 
*  
*
* **Parameters:**
*
* * `channel_id: &str` -- Channel ID: Unique Identifier of a channel.
*/
pub async fn get_user_level_channel(
&self,
channel_id: &str, channel_id: &str,
) -> Result<crate::types::Channel> {
let url =
format!("/chat/channels/{}",
crate::progenitor_support::encode_path(&channel_id.to_string()),);

self.client.get(&url, None).await
}

/**
* Delete a channel.
*
* This function performs a `DELETE` to the `/chat/channels/{channelId}` endpoint.
*
* Zoom chat [channels](https://support.zoom.us/hc/en-us/articles/200912909-Getting-Started-With-Channels-Group-Messaging-) allow users to communicate via chat in private or public groups. Use this API to delete a specific channel. 
* 
* **Scope:** `chat_channel:write`<br>
* **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
* 
* <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note: </b> This API only supports <b>user-managed</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>.</p><br>
* 
* 
*  
*
* **Parameters:**
*
* * `channel_id: &str` -- Channel ID: Unique Identifier of a channel.
*/
pub async fn delete_user_level_channel(
&self,
channel_id: &str, channel_id: &str,
) -> Result<()> {
let url =
format!("/chat/channels/{}",
crate::progenitor_support::encode_path(&channel_id.to_string()),);

self.client.delete(&url, None).await
}

/**
* Update a channel.
*
* This function performs a `PATCH` to the `/chat/channels/{channelId}` endpoint.
*
* Zoom chat channels allow users to communicate via chat in private or public channels. Use this API to update the name of a specific channel that you created. 
* 
* **Scope:** `chat_channel:write`	<br>
* **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
* 
* <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note: </b> This API only supports <b>user-managed</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>.</p><br>
* 
* 
*
* **Parameters:**
*
* * `channel_id: &str` -- Channel Id.
*/
pub async fn update_user_level_channel(
&self,
channel_id: &str, channel_id: &str,
body: &crate::types::UpdateChannelBodyRequest
) -> Result<()> {
let url =
format!("/chat/channels/{}",
crate::progenitor_support::encode_path(&channel_id.to_string()),);

self.client.patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body).unwrap()))).await
}

/**
* Remove a member.
*
* This function performs a `DELETE` to the `/chat/channels/{channelId}/members/{memberId}` endpoint.
*
*  A [channel](https://support.zoom.us/hc/en-us/articles/200912909-Getting-Started-With-Channels-Group-Messaging-) can have one or multiple members. Use this API to remove a member from a chat channel.<br><br>
*  **Scopes:** `chat_channel:write`<br>
*  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
*  
*  <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note: </b> This API only supports <b>user-managed</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>.</p><br>
* 
* 
* 
*
* **Parameters:**
*
* * `channel_id: &str` -- Unique Identifier of the Channel from where you would like to remove a member. This can be retrieved from the [List Channels API](https://marketplace.zoom.us/docs/api-reference/zoom-api/chat-channels/getchannels).
* * `member_id: &str` -- Email address of the member whom you would like to be remove from the channel.
*/
pub async fn remove_a_user_level_channel_member(
&self,
channel_id: &str, member_id: &str, channel_id: &str, member_id: &str,
) -> Result<()> {
let url =
format!("/chat/channels/{}/members/{}",
crate::progenitor_support::encode_path(&channel_id.to_string()),crate::progenitor_support::encode_path(&member_id.to_string()),);

self.client.delete(&url, None).await
}

/**
* Join a channel.
*
* This function performs a `POST` to the `/chat/channels/{channelId}/members/me` endpoint.
*
* A [channel](https://support.zoom.us/hc/en-us/articles/200912909-Getting-Started-With-Channels-Group-Messaging-) can have one or multiple members. Use this API to join a channel that is open for anyone in the same organization to join. You cannot use this API to join private channels that only allows invited members to be a part of it.
* 
* <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note: </b>This API only supports <b>user-managed</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>.</p><br>
* 
* **Scope:** `chat_channel:write`<br>
* **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
*
* **Parameters:**
*
* * `channel_id: &str` -- Channel Id.
*/
pub async fn join_channel(
&self,
channel_id: &str, channel_id: &str,
) -> Result<crate::types::JoinChannelResponse> {
let url =
format!("/chat/channels/{}/members/me",
crate::progenitor_support::encode_path(&channel_id.to_string()),);

self.client.post(&url, None).await
}

/**
* Leave a channel.
*
* This function performs a `DELETE` to the `/chat/channels/{channelId}/members/me` endpoint.
*
* If you're no longer interested in being a member of an existing channel, you can leave the channel at any time. Use this API to leave a specific channel. After leaving the channel, you can no longer access information from that channel.
* 
* <p style="background-color:#e1f5fe; color:#01579b; padding:8px"> <b>Note: </b>This API only supports <b>user-managed</b> <a href="https://marketplace.zoom.us/docs/guides/getting-started/app-types/create-oauth-app">OAuth app</a>.</p><br>
* 
* **Scope:** `chat_channel:write`<br>
* **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
*
* **Parameters:**
*
* * `channel_id: &str` -- Channel ID: Unique Identifier of a channel.
*/
pub async fn leave_channel(
&self,
channel_id: &str, channel_id: &str,
) -> Result<()> {
let url =
format!("/chat/channels/{}/members/me",
crate::progenitor_support::encode_path(&channel_id.to_string()),);

self.client.delete(&url, None).await
}


}