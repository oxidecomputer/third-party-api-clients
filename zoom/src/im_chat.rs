use anyhow::Result;

use crate::Client;

pub struct ImChat {
    pub client: Client,
}

impl ImChat {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ImChat { client }
    }

    /**
     * Get IM chat sessions.
     *
     * This function performs a `GET` to the `/im/chat/sessions` endpoint.
     *
     * Retrieve IM Chat sessions for a specified period of time. This API only supports Oauth2.<br>
     *
     * **Scopes:** `imchat:read, imchat:read:admin`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Heavy`
     *  <br><br>
     *   <p style="background-color:#e1f5fe; color:#000000; padding:8px"><b>Deprecated:</b> By end of 2021, Zoom is deprecating this API in favor of a consolidated set of APIs. The API will still be available for you to use, though Zoom will no longer provide support for it. For further information, see <a href="https://marketplace.zoom.us/docs/guides/stay-up-to-date/announcements#im-api-notice">Announcements: IM APIs Deprecation</a>.</p>
     *  
     *
     *
     * **Parameters:**
     *
     * * `from: chrono::NaiveDate` -- Start date in 'yyyy-mm-dd' format. The date range defined by the "from" and "to" parameters should only be one month as the report includes only one month worth of data at once.
     * * `to: chrono::NaiveDate` -- Start Date.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn session(
        &self,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<crate::types::ImChatSessionsResponseAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !to.to_string().is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/im/chat/sessions?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * Get IM chat messages.
     *
     * This function performs a `GET` to the `/im/chat/sessions/{sessionId}` endpoint.
     *
     * Retrieve IM chat messages for a specified period of time. This API only supports oauth2.<br>
     *
     * **Scopes:** `imchat:read`<br>
     *
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     *  
     *  <br>
     *  
     *  <p style="background-color:#e1f5fe; color:#000000; padding:8px"><b>Deprecated:</b> By end of 2021, Zoom is deprecating this API in favor of a consolidated set of APIs. The API will still be available for you to use, though Zoom will no longer provide support for it. For further information, see <a href="https://marketplace.zoom.us/docs/guides/stay-up-to-date/announcements#im-api-notice">Announcements: IM APIs Deprecation</a>.</p>
     *
     *
     *
     *
     * **Parameters:**
     *
     * * `session_id: &str` -- User's first name.
     * * `from: chrono::NaiveDate` -- Start date in 'yyyy-mm-dd' format. The date range defined by the "from" and "to" parameters should only be one month as the report includes only one month worth of data at once.
     * * `to: chrono::NaiveDate` -- Start Date.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn message(
        &self,
        session_id: &str,
        from: chrono::NaiveDate,
        to: chrono::NaiveDate,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<crate::types::ImChatMessagesResponseAllOf> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !from.to_string().is_empty() {
            query_args.push(("from".to_string(), from.to_string()));
        }
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !to.to_string().is_empty() {
            query_args.push(("to".to_string(), to.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/im/chat/sessions/{}?{}",
            crate::progenitor_support::encode_path(&session_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Get user’s IM messages.
     *
     * This function performs a `GET` to the `/im/users/{userId}/chat/messages` endpoint.
     *
     * Get IM Chat messages for a specified period of time. This API only supports Oauth2.<br>
     * **Scopes:** `imchat:read`<br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     * <br><br>
     *   <p style="background-color:#e1f5fe; color:#000000; padding:8px"><b>Deprecated:</b> By end of 2021, Zoom is deprecating this API in favor of a consolidated set of APIs. The API will still be available for you to use, though Zoom will no longer provide support for it. For further information, see <a href="https://marketplace.zoom.us/docs/guides/stay-up-to-date/announcements#im-api-notice">Announcements: IM APIs Deprecation</a>.</p>
     *
     *
     * **Parameters:**
     *
     * * `user_id: &str` -- The user ID or email address.
     * * `chat_user: &str` -- Chat user's ID or email address.
     * * `channel: &str` -- User's first name.
     * * `date: &str` -- IM message's query date time, format as yyyy-MM-dd.
     * * `page_size: i64` -- The number of records returned within a single API call.
     * * `next_page_token: &str` -- The next page token is used to paginate through large result sets. A next page token will be returned whenever the set of available results exceeds the current page size. The expiration period for this token is 15 minutes.
     */
    pub async fn list_im_messages(
        &self,
        user_id: &str,
        chat_user: &str,
        channel: &str,
        date: &str,
        page_size: i64,
        next_page_token: &str,
    ) -> Result<Vec<crate::types::ListimmessagesResponseMessages>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !channel.is_empty() {
            query_args.push(("channel".to_string(), channel.to_string()));
        }
        if !chat_user.is_empty() {
            query_args.push(("chat_user".to_string(), chat_user.to_string()));
        }
        if !date.is_empty() {
            query_args.push(("date".to_string(), date.to_string()));
        }
        if !next_page_token.is_empty() {
            query_args.push(("next_page_token".to_string(), next_page_token.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/im/users/{}/chat/messages?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        let resp: crate::types::ListimmessagesResponse = self.client.get(&url, None).await?;

        // Return our response data.
        Ok(resp.messages)
    }

    /**
     * Get user’s IM messages.
     *
     * This function performs a `GET` to the `/im/users/{userId}/chat/messages` endpoint.
     *
     * As opposed to `list_im_messages`, this function returns all the pages of the request at once.
     *
     * Get IM Chat messages for a specified period of time. This API only supports Oauth2.<br>
     * **Scopes:** `imchat:read`<br>
     * **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     * <br><br>
     *   <p style="background-color:#e1f5fe; color:#000000; padding:8px"><b>Deprecated:</b> By end of 2021, Zoom is deprecating this API in favor of a consolidated set of APIs. The API will still be available for you to use, though Zoom will no longer provide support for it. For further information, see <a href="https://marketplace.zoom.us/docs/guides/stay-up-to-date/announcements#im-api-notice">Announcements: IM APIs Deprecation</a>.</p>
     *
     */
    pub async fn list_all_im_messages(
        &self,
        user_id: &str,
        chat_user: &str,
        channel: &str,
        date: &str,
    ) -> Result<Vec<crate::types::ListimmessagesResponseMessages>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !channel.is_empty() {
            query_args.push(("channel".to_string(), channel.to_string()));
        }
        if !chat_user.is_empty() {
            query_args.push(("chat_user".to_string(), chat_user.to_string()));
        }
        if !date.is_empty() {
            query_args.push(("date".to_string(), date.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/im/users/{}/chat/messages?{}",
            crate::progenitor_support::encode_path(&user_id.to_string()),
            query_
        );

        let mut resp: crate::types::ListimmessagesResponse = self.client.get(&url, None).await?;

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
     * Send IM messages.
     *
     * This function performs a `POST` to the `/im/users/me/chat/messages` endpoint.
     *
     * Send chat message to a user. <aside>Note: This API only supports OAuth 2.0.</aside><br><br>**Scope:** `imchat:write`
     *  
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`
     *
     * **Parameters:**
     *
     * * `chat_user: &str` -- The email address (registered with Zoom) or the userId of the chat user.
     */
    pub async fn send_im_messages(
        &self,
        chat_user: &str,
        body: &crate::types::SendimmessagesRequest,
    ) -> Result<crate::types::SendimmessagesResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !chat_user.is_empty() {
            query_args.push(("chat_user".to_string(), chat_user.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/im/users/me/chat/messages?{}", query_);

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
