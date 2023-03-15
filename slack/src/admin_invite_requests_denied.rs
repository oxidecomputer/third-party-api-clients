use crate::Client;
use crate::ClientResult;

pub struct AdminInviteRequestsDenied {
    pub client: Client,
}

impl AdminInviteRequestsDenied {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AdminInviteRequestsDenied { client }
    }

    /**
     * This function performs a `GET` to the `/admin.inviteRequests.denied.list` endpoint.
     *
     * List all denied workspace invite requests.
     *
     * FROM: <https://api.slack.com/methods/admin.inviteRequests.denied.list>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.invites:read`.
     * * `team_id: &str` -- ID for the workspace where the invite requests were made.
     * * `cursor: &str` -- Value of the `next_cursor` field sent as part of the previous api response.
     * * `limit: i64` -- The number of results that will be returned by the API on each invocation. Must be between 1 - 1000 both inclusive.
     */
    pub async fn list(
        &self,
        team_id: &str,
        cursor: &str,
        limit: i64,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !team_id.is_empty() {
            query_args.push(("team_id".to_string(), team_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(
            &format!("/admin.inviteRequests.denied.list?{}", query_),
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
