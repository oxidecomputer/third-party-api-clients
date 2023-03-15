use crate::Client;
use crate::ClientResult;

pub struct AdminConversationsEkm {
    pub client: Client,
}

impl AdminConversationsEkm {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AdminConversationsEkm { client }
    }

    /**
     * This function performs a `GET` to the `/admin.conversations.ekm.listOriginalConnectedChannelInfo` endpoint.
     *
     * List all disconnected channels—i.e., channels that were once connected to other workspaces and then disconnected—and the corresponding original channel IDs for key revocation with EKM.
     *
     * FROM: <https://api.slack.com/methods/admin.conversations.ekm.listOriginalConnectedChannelInfo>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.conversations:read`.
     * * `channel_ids: &str` -- A comma-separated list of channels to filter to.
     * * `team_ids: &str` -- A comma-separated list of the workspaces to which the channels you would like returned belong.
     * * `limit: i64` -- The maximum number of items to return. Must be between 1 - 1000 both inclusive.
     * * `cursor: &str` -- Set `cursor` to `next_cursor` returned by the previous call to list items in the next page.
     */
    pub async fn list_original_connected_channel_info(
        &self,
        channel_ids: &str,
        team_ids: &str,
        limit: i64,
        cursor: &str,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !channel_ids.is_empty() {
            query_args.push(("channel_ids".to_string(), channel_ids.to_string()));
        }
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !team_ids.is_empty() {
            query_args.push(("team_ids".to_string(), team_ids.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!(
                "/admin.conversations.ekm.listOriginalConnectedChannelInfo?{}",
                query_
            ),
            None,
        );
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
