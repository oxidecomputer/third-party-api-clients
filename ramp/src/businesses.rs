use crate::Client;
use crate::ClientResult;

pub struct Businesses {
    pub client: Client,
}

impl Businesses {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Businesses { client }
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
    pub async fn get_resources_busine(&self) -> ClientResult<crate::types::Business> {
        let url = self.client.url("/business", None);
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
    ) -> ClientResult<crate::types::BusinessCurrentStatus> {
        let url = self.client.url("/business/balance", None);
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
