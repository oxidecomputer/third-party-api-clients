use crate::Client;
use crate::ClientResult;

pub struct Rtm {
    pub client: Client,
}

impl Rtm {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Rtm { client }
    }

    /**
     * This function performs a `GET` to the `/rtm.connect` endpoint.
     *
     * Starts a Real Time Messaging session.
     *
     * FROM: <https://api.slack.com/methods/rtm.connect>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `rtm:stream`.
     * * `batch_presence_aware: bool` -- Batch presence deliveries via subscription. Enabling changes the shape of `presence_change` events. See [batch presence](/docs/presence-and-status#batching).
     * * `presence_sub: bool` -- Only deliver presence events when requested by subscription. See [presence subscriptions](/docs/presence-and-status#subscriptions).
     */
    pub async fn connect(
        &self,
        batch_presence_aware: bool,
        presence_sub: bool,
    ) -> ClientResult<crate::types::RtmConnectSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if batch_presence_aware {
            query_args.push((
                "batch_presence_aware".to_string(),
                batch_presence_aware.to_string(),
            ));
        }
        if presence_sub {
            query_args.push(("presence_sub".to_string(), presence_sub.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/rtm.connect?{}", query_), None);
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
