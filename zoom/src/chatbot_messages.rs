use anyhow::Result;

use crate::Client;

pub struct ChatbotMessages {
    pub client: Client,
}

impl ChatbotMessages {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ChatbotMessages { client }
    }

    /**
     * Send chatbot messages.
     *
     * This function performs a `POST` to the `/im/chat/messages` endpoint.
     *
     * Send chatbot messages from your marketplace chatbot app.<br><br>
     * **Scopes:** `imchat:bot`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     * **Authorization Flow**: Client Credentials Flow<br><br>
     * To get authorized, make a POST request to `/oauth/token` endpoint with grant type as `client_credentials`. <br>Use `https://api.zoom.us/oauth/token?grant_type=client_credentials` as the endpoint for the request.
     * You will need to send your ClientID and Secret as a Basic base64 encoded AUthorization header. Ex. `Basic base64Encode({client_id}:{client_sceret})`<br><br> Next, use the token recieved (access_token) as a bearer token while making the POST /im/chat/messages request to send chatbot messages.<br><br>
     * Learn more about how to authorize chatbots in the [Chatbot Authorization](https://marketplace.zoom.us/docs/guides/chatbots/authorization) guide.
     */
    pub async fn sendchatbot(&self, body: &crate::types::SendchatbotRequest) -> Result<()> {
        let url = "/im/chat/messages".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Edit a chatbot message.
     *
     * This function performs a `PUT` to the `/im/chat/messages/{message_id}` endpoint.
     *
     * Edit a message that was [sent](https://marketplace.zoom.us/docs/api-reference/zoom-api/im-chat/sendchatbot) by your Chatbot app.<br> After sending a message using the Send Chatbot Message API, you must store the messageId returned in the response so that you can make edits to the associated message using this API.
     *
     * **Scope:** `imchat:bot`<br>
     *  **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>
     * **Authorization Flow**: Client Credentials Flow<br><br>
     * To get authorized, make a POST request to `/oauth/token` endpoint with grant type as `client_credentials`. <br>Use `https://api.zoom.us/oauth/token?grant_type=client_credentials` as the endpoint for the request.
     * You will need to send your ClientID and Secret as a Basic base64 encoded AUthorization header. Ex. `Basic base64Encode({client_id}:{client_sceret})`<br><br> Next, use the token received (access_token) as a bearer token while making the PUT /im/chat/messages/{message_id} request to edit a chatbot message.<br><br>
     * Learn more about how to authotize chatbots in the [Chatbot Authorization](https://marketplace.zoom.us/docs/guides/chatbots/authorization) guide.
     *
     * **Parameters:**
     *
     * * `message_id: &str` -- Unique Identifier of the message that needs to be updated. This should be retrieved from the response of [Send Chatbot Message API](https://marketplace.zoom.us/docs/api-reference/zoom-api/im-chat/sendchatbot).
     */
    pub async fn edit(
        &self,
        message_id: &str,
        body: &crate::types::EditChatbotMessageRequest,
    ) -> Result<crate::types::EditChatbotMessageResponse> {
        let url = format!(
            "/im/chat/messages/{}",
            crate::progenitor_support::encode_path(&message_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete a chatbot message.
     *
     * This function performs a `DELETE` to the `/im/chat/messages/{message_id}` endpoint.
     *
     * Delete a message that was sent by your chatbot app.<br><br> **Scopes:** `imchat:bot`<br> **[Rate Limit Label](https://marketplace.zoom.us/docs/api-reference/rate-limits#rate-limits):** `Medium`<br>**Authorization Flow**: Client Credentials Flow<br><br>To get authorized, make a POST request to `/oauth/token` endpoint with grant type as `client_credentials`. <br>Use `https://api.zoom.us/oauth/token?grant_type=client_credentials` as the endpoint for the request.
     * You will need to send your ClientID and Secret as a Basic base64 encoded AUthorization header. Ex. `Basic base64Encode({client_id}:{client_sceret})`<br><br> Next, use the token received (access_token) as a bearer token while making the DELETE /im/chat/messages/{message_id} request to delete a message.<br><br>
     * Learn more about how to authotize chatbots in the [Chatbot Authorization](https://marketplace.zoom.us/docs/guides/chatbots/authorization) guide.
     */
    pub async fn delete(
        &self,
        message_id: &str,
        body: &crate::types::DeleteChatbotMessageRequest,
    ) -> Result<crate::types::DeleteChatbotMessageResponse> {
        let url = format!(
            "/im/chat/messages/{}",
            crate::progenitor_support::encode_path(&message_id.to_string()),
        );

        self.client
            .delete(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
