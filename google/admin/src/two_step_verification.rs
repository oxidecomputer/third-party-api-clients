use crate::Client;
use crate::ClientResult;

pub struct TwoStepVerification {
    pub client: Client,
}

impl TwoStepVerification {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TwoStepVerification { client }
    }

    /**
     * This function performs a `POST` to the `/admin/directory/v1/users/{userKey}/twoStepVerification/turnOff` endpoint.
     *
     * Turns off 2-Step Verification for user.
     *
     * **Parameters:**
     *
     * * `user_key: &str` -- Identifies the user in the API request. The value can be the user's primary email address, alias email address, or unique user ID.
     */
    pub async fn turn_off(&self, user_key: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/admin/directory/v1/users/{}/twoStepVerification/turnOff",
                crate::progenitor_support::encode_path(user_key),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
}
