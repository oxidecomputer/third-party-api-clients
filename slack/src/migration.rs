use crate::Client;
use crate::ClientResult;

pub struct Migration {
    pub client: Client,
}

impl Migration {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Migration { client }
    }

    /**
     * This function performs a `GET` to the `/migration.exchange` endpoint.
     *
     * For Enterprise Grid workspaces, map local user IDs to global user IDs
     *
     * FROM: <https://api.slack.com/methods/migration.exchange>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `tokens.basic`.
     * * `users: &str` -- A comma-separated list of user ids, up to 400 per request.
     * * `team_id: &str` -- Specify team_id starts with `T` in case of Org Token.
     * * `to_old: bool` -- Specify `true` to convert `W` global user IDs to workspace-specific `U` IDs. Defaults to `false`.
     */
    pub async fn exchange(
        &self,
        users: &str,
        team_id: &str,
        to_old: bool,
    ) -> ClientResult<crate::types::MigrationExchangeSuccessSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !team_id.is_empty() {
            query_args.push(("team_id".to_string(), team_id.to_string()));
        }
        if to_old {
            query_args.push(("to_old".to_string(), to_old.to_string()));
        }
        if !users.is_empty() {
            query_args.push(("users".to_string(), users.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/migration.exchange?{}", query_), None);
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
