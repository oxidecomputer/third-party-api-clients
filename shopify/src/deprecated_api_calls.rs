use crate::Client;
use crate::ClientResult;

pub struct DeprecatedApiCalls {
    pub client: Client,
}

impl DeprecatedApiCalls {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        DeprecatedApiCalls { client }
    }

    /**
     * Retrieves a list of deprecated API calls made by the authenticated private app in the past 30 days.
     *
     * This function performs a `GET` to the `/admin/api/2021-01/deprecated_api_calls.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/deprecated_api_calls#index-2021-01
     */
    pub async fn deprecated_202101_get_call(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/2021-01/deprecated_api_calls.json", None);
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
     * Retrieves a list of deprecated API calls made by the authenticated private app in the past 30 days.
     *
     * This function performs a `GET` to the `/admin/api/unstable/deprecated_api_calls.json` endpoint.
     *
     * https://shopify.dev/docs/admin-api/rest/reference/deprecated_api_calls#index-unstable
     */
    pub async fn deprecated_unstable_get_call(&self) -> ClientResult<()> {
        let url = self
            .client
            .url("/admin/api/unstable/deprecated_api_calls.json", None);
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
