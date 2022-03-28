use anyhow::Result;

use crate::Client;

pub struct RateLimit {
    pub client: Client,
}

impl RateLimit {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        RateLimit { client }
    }

    /**
    * Get rate limit status for the authenticated user.
    *
    * This function performs a `GET` to the `/rate_limit` endpoint.
    *
    * **Note:** Accessing this endpoint does not count against your REST API rate limit.
    *
    * **Note:** The `rate` object is deprecated. If you're writing new API client code or updating existing code, you should use the `core` object instead of the `rate` object. The `core` object contains the same information that is present in the `rate` object.
    *
    * FROM: <https://docs.github.com/rest/reference/rate-limit#get-rate-limit-status-for-the-authenticated-user>
    */
    pub async fn get(&self) -> Result<crate::types::RateLimitOverview> {
        let url = "/rate_limit".to_string();
        self.client.get(&url, None).await
    }
}
