use anyhow::Result;

use crate::Client;

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
    pub async fn get_user_profile(&self) -> Result<crate::types::GetUserProfileResponse> {
        let url = "/user/profile".to_string();
        self.client.get(&url, None).await
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
    ) -> Result<crate::types::UserProfile> {
        let url = "/user/profile".to_string();
        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
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
    pub async fn get_user_account(&self) -> Result<crate::types::GetUserAccountResponse> {
        let url = "/user/account".to_string();
        self.client.get(&url, None).await
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
    pub async fn get_user_email(&self) -> Result<crate::types::GetUserEmailResponse> {
        let url = "/user/email".to_string();
        self.client.get(&url, None).await
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
    ) -> Result<crate::types::GetUserEmailResponse> {
        let url = "/user/email".to_string();
        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
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
    pub async fn get_user_username(&self) -> Result<crate::types::Users> {
        let url = "/user/username".to_string();
        self.client.get(&url, None).await
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
    ) -> Result<crate::types::PutUserUsernameResponse> {
        let url = "/user/username".to_string();
        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
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
    pub async fn get_user_credits(&self) -> Result<crate::types::GetUserCreditsResponse> {
        let url = "/user/credits".to_string();
        self.client.get(&url, None).await
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
    ) -> Result<crate::types::Help> {
        let url = "/user/password".to_string();
        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
