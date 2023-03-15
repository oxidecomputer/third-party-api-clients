use crate::Client;
use crate::ClientResult;

pub struct UsersApi {
    pub client: Client,
}

impl UsersApi {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersApi { client }
    }

    /**
     * Get a user's profile.
     *
     * This function performs a `GET` to the `/user/profile` endpoint.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_user_profile(&self) -> ClientResult<crate::types::GetUserProfileResponse> {
        let url = self.client.url("/user/profile", None);
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
     * Update a user's profile.
     *
     * This function performs a `PATCH` to the `/user/profile` endpoint.
     *
     * **This endpoint allows you to update your current profile details.**
     *
     * Any one or more of the parameters can be updated via the PATCH `/user/profile` endpoint. You must include at least one when you PATCH.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_user_profile(
        &self,
        body: &crate::types::UserProfile,
    ) -> ClientResult<crate::types::UserProfile> {
        let url = self.client.url("/user/profile", None);
        self.client
            .patch(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Get a user's account information.
     *
     * This function performs a `GET` to the `/user/account` endpoint.
     *
     * **This endpoint allows you to retrieve your user account details.**
     *
     * Your user's account information includes the user's account type and reputation.
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_user_account(&self) -> ClientResult<crate::types::GetUserAccountResponse> {
        let url = self.client.url("/user/account", None);
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
     * Retrieve your account email address.
     *
     * This function performs a `GET` to the `/user/email` endpoint.
     *
     * **This endpoint allows you to retrieve the email address currently on file for your account.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_user_email(&self) -> ClientResult<crate::types::GetUserEmailResponse> {
        let url = self.client.url("/user/email", None);
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
     * Update your account email address.
     *
     * This function performs a `PUT` to the `/user/email` endpoint.
     *
     * **This endpoint allows you to update the email address currently on file for your account.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn put_user_email(
        &self,
        body: &crate::types::PutUserEmailRequest,
    ) -> ClientResult<crate::types::GetUserEmailResponse> {
        let url = self.client.url("/user/email", None);
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieve your username.
     *
     * This function performs a `GET` to the `/user/username` endpoint.
     *
     * **This endpoint allows you to retrieve your current account username.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_user_username(&self) -> ClientResult<crate::types::Users> {
        let url = self.client.url("/user/username", None);
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
     * Update your username.
     *
     * This function performs a `PUT` to the `/user/username` endpoint.
     *
     * **This endpoint allows you to update the username for your account.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn put_user_username(
        &self,
        body: &crate::types::PutUserUsernameRequest,
    ) -> ClientResult<crate::types::PutUserUsernameResponse> {
        let url = self.client.url("/user/username", None);
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * Retrieve your credit balance.
     *
     * This function performs a `GET` to the `/user/credits` endpoint.
     *
     * **This endpoint allows you to retrieve the current credit balance for your account.**
     *
     * Each account has a credit balance, which is a base number of emails it can send before receiving per-email charges. For more information about credits and billing, see [Billing and Plan details information](https://sendgrid.com/docs/ui/account-and-settings/billing/).
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_user_credits(&self) -> ClientResult<crate::types::GetUserCreditsResponse> {
        let url = self.client.url("/user/credits", None);
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
     * Update your password.
     *
     * This function performs a `PUT` to the `/user/password` endpoint.
     *
     * **This endpoint allows you to update your password.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn put_user_password(
        &self,
        body: &crate::types::PutUserPasswordRequest,
    ) -> ClientResult<crate::types::Help> {
        let url = self.client.url("/user/password", None);
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
}
