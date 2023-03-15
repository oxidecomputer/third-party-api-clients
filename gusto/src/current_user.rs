use crate::Client;
use crate::ClientResult;

pub struct CurrentUser {
    pub client: Client,
}

impl CurrentUser {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CurrentUser { client }
    }

    /**
     * Get the current user.
     *
     * This function performs a `GET` to the `/v1/me` endpoint.
     *
     * Returns information pertaining to the user associated with the provided access token.
     */
    pub async fn get_me(&self) -> ClientResult<crate::types::CurrentUser> {
        let url = self.client.url("/v1/me", None);
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
