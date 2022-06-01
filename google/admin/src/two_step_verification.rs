use anyhow::Result;

use crate::Client;

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
    pub async fn turn_off(&self, user_key: &str) -> Result<()> {
        let url = format!(
            "/admin/directory/v1/users/{}/twoStepVerification/turnOff",
            crate::progenitor_support::encode_path(&user_key.to_string()),
        );

        self.client.post(&url, None).await
    }
}
