use anyhow::Result;

use crate::Client;

pub struct ActivityFeed {
    pub client: Client,
}

impl ActivityFeed {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ActivityFeed { client }
    }

    /**
    * Get latest chimp chatter.
    *
    * This function performs a `GET` to the `/activity-feed/chimp-chatter` endpoint.
    *
    * Return the Chimp Chatter for this account ordered by most recent.
    *
    * **Parameters:**
    *
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    */
    pub async fn get_chimp_chatter(
        &self,
        count: i64,
        offset: i64,
    ) -> Result<crate::types::GetActivityFeedChimpChatterResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/activity-feed/chimp-chatter?{}", query_);

        self.client.get(&url, None).await
    }
}
