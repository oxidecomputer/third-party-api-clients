use crate::Client;
use crate::ClientResult;

pub struct ApiKeyPermissions {
    pub client: Client,
}

impl ApiKeyPermissions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ApiKeyPermissions { client }
    }

    /**
     * Retrieve a list of scopes for which this user has access.
     *
     * This function performs a `GET` to the `/scopes` endpoint.
     *
     * **This endpoint returns a list of all scopes that this user has access to.**
     *
     * API Keys are used to authenticate with [SendGrid's v3 API](https://sendgrid.api-docs.io/v3.0/how-to-use-the-sendgrid-v3-api/api-authorization).
     *
     * API Keys may be assigned certain permissions, or scopes, that limit which API endpoints they are able to access.
     *
     * This endpoint returns all the scopes assigned to the key you use to authenticate with it. To retrieve the scopes assigned to another key, you can pass an API key ID to the "Retrieve an existing API key" endpoint.
     *
     * For a more detailed explanation of how you can use API Key permissions, please visit our [API Keys documentation](https://sendgrid.com/docs/ui/account-and-settings/api-keys/).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_scopes(&self) -> ClientResult<crate::types::GetScopesResponse> {
        let url = self.client.url("/scopes", None);
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
