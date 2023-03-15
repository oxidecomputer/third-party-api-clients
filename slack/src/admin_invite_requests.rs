use crate::Client;
use crate::ClientResult;

pub struct AdminInviteRequests {
    pub client: Client,
}

impl AdminInviteRequests {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        AdminInviteRequests { client }
    }

    /**
     * This function performs a `POST` to the `/admin.inviteRequests.approve` endpoint.
     *
     * Approve a workspace invite request.
     *
     * FROM: <https://api.slack.com/methods/admin.inviteRequests.approve>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.invites:write`.
     */
    pub async fn approve(
        &self,
        body: &crate::types::AdminInviteRequestsApproveRequest,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.inviteRequests.approve", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/admin.inviteRequests.deny` endpoint.
     *
     * Deny a workspace invite request.
     *
     * FROM: <https://api.slack.com/methods/admin.inviteRequests.deny>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.invites:write`.
     */
    pub async fn deny(
        &self,
        body: &crate::types::AdminInviteRequestsApproveRequest,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/admin.inviteRequests.deny", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/admin.inviteRequests.list` endpoint.
     *
     * List all pending workspace invite requests.
     *
     * FROM: <https://api.slack.com/methods/admin.inviteRequests.list>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `admin.invites:read`.
     * * `team_id: &str` -- ID for the workspace where the invite requests were made.
     * * `cursor: &str` -- Value of the `next_cursor` field sent as part of the previous API response.
     * * `limit: i64` -- The number of results that will be returned by the API on each invocation. Must be between 1 - 1000, both inclusive.
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
        let url = self
            .client
            .url(&format!("/admin.inviteRequests.list?{}", query_), None);
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
