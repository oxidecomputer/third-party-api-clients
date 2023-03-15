use crate::Client;
use crate::ClientResult;

pub struct Bots {
    pub client: Client,
}

impl Bots {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Bots { client }
    }

    /**
     * This function performs a `GET` to the `/bots.info` endpoint.
     *
     * Gets information about a bot user.
     *
     * FROM: <https://api.slack.com/methods/bots.info>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `users:read`.
     * * `bot: &str` -- Bot user to get info on.
     */
    pub async fn info(&self, bot: &str) -> ClientResult<crate::types::BotsInfoSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !bot.is_empty() {
            query_args.push(("bot".to_string(), bot.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/bots.info?{}", query_), None);
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
