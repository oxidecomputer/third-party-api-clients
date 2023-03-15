use crate::Client;
use crate::ClientResult;

pub struct TeamProfile {
    pub client: Client,
}

impl TeamProfile {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TeamProfile { client }
    }

    /**
     * This function performs a `GET` to the `/team.profile.get` endpoint.
     *
     * Retrieve a team's profile.
     *
     * FROM: <https://api.slack.com/methods/team.profile.get>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `users.profile:read`.
     * * `visibility: &str` -- Filter by visibility.
     */
    pub async fn get(
        &self,
        visibility: &str,
    ) -> ClientResult<crate::types::TeamProfileGetSuccessSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !visibility.is_empty() {
            query_args.push(("visibility".to_string(), visibility.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/team.profile.get?{}", query_), None);
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
