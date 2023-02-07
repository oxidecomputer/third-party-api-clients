use anyhow::Result;

use crate::Client;

pub struct UsersProfile {
    pub client: Client,
}

impl UsersProfile {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsersProfile { client }
    }

    /**
    * This function performs a `GET` to the `/users.profile.get` endpoint.
    *
    * Retrieves a user's profile information.
    *
    * FROM: <https://api.slack.com/methods/users.profile.get>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `users.profile:read`.
    * * `include_labels: bool` -- Include labels for each ID in custom profile fields.
    * * `user: &str` -- User to retrieve profile info for.
    */
    pub async fn get(
        &self,
        include_labels: bool,
        user: &str,
    ) -> Result<crate::types::UsersProfileGetSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if include_labels {
            query_args.push(("include_labels".to_string(), include_labels.to_string()));
        }
        if !user.is_empty() {
            query_args.push(("user".to_string(), user.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/users.profile.get?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/users.profile.set` endpoint.
    *
    * Set the profile information for a user.
    *
    * FROM: <https://api.slack.com/methods/users.profile.set>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `users.profile:write`.
    */
    pub async fn set(&self) -> Result<crate::types::UsersProfileSetSchema> {
        let url = "/users.profile.set".to_string();
        self.client.post(&url, None).await
    }
}
