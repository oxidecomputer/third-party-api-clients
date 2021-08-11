use anyhow::Result;

use crate::Client;

pub struct Business {
    client: Client,
}

impl Business {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Business { client }
    }

    /**
     * GET business metadata.
     *
     * This function performs a `GET` to the `/business` endpoint.
     *
     * Gets metadata about a business.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn get_resources_business(
        &self,
    ) -> Result<crate::types::GetResourcesBusinessResponse> {
        let url = "/business".to_string();
        self.client.get(&url, None).await
    }

    /**
     * GET current info about a business.
     *
     * This function performs a `GET` to the `/business/balance` endpoint.
     *
     * Gets current information about a business.
     *
     * **Parameters:**
     *
     * * `authorization: &str` -- The OAuth2 token header.
     */
    pub async fn get_resources_business_current(
        &self,
    ) -> Result<crate::types::GetResourcesBusinessCurrentResponse> {
        let url = "/business/balance".to_string();
        self.client.get(&url, None).await
    }
}
