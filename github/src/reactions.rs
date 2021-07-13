use anyhow::Result;

use crate::Client;

pub struct Reactions {
    client: Client,
}

impl Reactions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Reactions { client }
    }

    /**
     * Create reaction for a team discussion (Legacy).
     *
     * This function performs a `POST` to the `/teams/{team_id}/discussions/{discussion_number}/reactions` endpoint.
     *
     * **Deprecation Notice:** This endpoint route is deprecated and will be removed from the Teams API. We recommend migrating your existing code to use the new [`Create reaction for a team discussion`](https://docs.github.com/rest/reference/reactions#create-reaction-for-a-team-discussion) endpoint.
     *
     * Create a reaction to a [team discussion](https://docs.github.com/rest/reference/teams#discussions). OAuth access tokens require the `write:discussion` [scope](https://docs.github.com/apps/building-oauth-apps/understanding-scopes-for-oauth-apps/). A response with an HTTP `200` status means that you already added the reaction type to this team discussion.
     *
     * FROM: <https://docs.github.com/rest/reference/reactions/#create-reaction-for-a-team-discussion-legacy>
     *
     * **Parameters:**
     *
     * * `team_id: i64`
     * * `discussion_number: i64`
     */
    pub async fn create_for_team_discussion_legacy(
        &self,
        team_id: i64,
        discussion_number: i64,
        body: &crate::types::ReactionsCreateTeamDiscussionInOrgRequest,
    ) -> Result<crate::types::Reaction> {
        let url = format!(
            "/teams/{}/discussions/{}/reactions",
            crate::progenitor_support::encode_path(&team_id.to_string()),
            crate::progenitor_support::encode_path(&discussion_number.to_string()),
        );

        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }
}
