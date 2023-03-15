use crate::Client;
use crate::ClientResult;

pub struct Users {
    pub client: Client,
}

impl Users {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Users { client }
    }

    /**
     * This function performs a `GET` to the `/users.conversations` endpoint.
     *
     * List conversations the calling user may access.
     *
     * FROM: <https://api.slack.com/methods/users.conversations>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `conversations:read`.
     * * `user: &str` -- Browse conversations by a specific user ID's membership. Non-public channels are restricted to those where the calling user shares membership.
     * * `types: &str` -- Mix and match channel types by providing a comma-separated list of any combination of `public_channel`, `private_channel`, `mpim`, `im`.
     * * `exclude_archived: bool` -- Set to `true` to exclude archived channels from the list.
     * * `limit: i64` -- The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the list hasn't been reached. Must be an integer no larger than 1000.
     * * `cursor: &str` -- Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](/docs/pagination) for more detail.
     */
    pub async fn conversation(
        &self,
        user: &str,
        types: &str,
        exclude_archived: bool,
        limit: i64,
        cursor: &str,
    ) -> ClientResult<crate::types::UsersConversationsSuccessSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if exclude_archived {
            query_args.push(("exclude_archived".to_string(), exclude_archived.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        if !types.is_empty() {
            query_args.push(("types".to_string(), types.to_string()));
        }
        if !user.is_empty() {
            query_args.push(("user".to_string(), user.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/users.conversations?{}", query_), None);
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
     * This function performs a `POST` to the `/users.deletePhoto` endpoint.
     *
     * Delete the user profile photo
     *
     * FROM: <https://api.slack.com/methods/users.deletePhoto>
     */
    pub async fn delete_photo(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/users.deletePhoto", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/users.getPresence` endpoint.
     *
     * Gets user presence information.
     *
     * FROM: <https://api.slack.com/methods/users.getPresence>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `users:read`.
     * * `user: &str` -- User to get presence info on. Defaults to the authed user.
     */
    pub async fn get_presence(
        &self,
        user: &str,
    ) -> ClientResult<crate::types::ApiMethodUsersGetPresence> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !user.is_empty() {
            query_args.push(("user".to_string(), user.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/users.getPresence?{}", query_), None);
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
     * This function performs a `GET` to the `/users.identity` endpoint.
     *
     * Get a user's identity.
     *
     * FROM: <https://api.slack.com/methods/users.identity>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `identity.basic`.
     */
    pub async fn identity(&self) -> ClientResult<Vec<crate::types::UsersIdentityResponseAnyOf>> {
        let url = self.client.url("/users.identity", None);
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
     * This function performs a `GET` to the `/users.identity` endpoint.
     *
     * As opposed to `identity`, this function returns all the pages of the request at once.
     *
     * Get a user's identity.
     *
     * FROM: <https://api.slack.com/methods/users.identity>
     */
    pub async fn get_all_identity(
        &self,
    ) -> ClientResult<Vec<crate::types::UsersIdentityResponseAnyOf>> {
        let url = self.client.url("/users.identity", None);
        self.client
            .get_all_pages(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/users.info` endpoint.
     *
     * Gets information about a user.
     *
     * FROM: <https://api.slack.com/methods/users.info>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `users:read`.
     * * `include_locale: bool` -- Set this to `true` to receive the locale for this user. Defaults to `false`.
     * * `user: &str` -- User to get info on.
     */
    pub async fn info(
        &self,
        include_locale: bool,
        user: &str,
    ) -> ClientResult<crate::types::UsersInfoSuccessSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if include_locale {
            query_args.push(("include_locale".to_string(), include_locale.to_string()));
        }
        if !user.is_empty() {
            query_args.push(("user".to_string(), user.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/users.info?{}", query_), None);
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
     * This function performs a `GET` to the `/users.list` endpoint.
     *
     * Lists all users in a Slack team.
     *
     * FROM: <https://api.slack.com/methods/users.list>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `users:read`.
     * * `limit: i64` -- The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached. Providing no `limit` value will result in Slack attempting to deliver you the entire result set. If the collection is too large you may experience `limit_required` or HTTP 500 errors.
     * * `cursor: &str` -- Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See [pagination](/docs/pagination) for more detail.
     * * `include_locale: bool` -- Set this to `true` to receive the locale for users. Defaults to `false`.
     */
    pub async fn list(
        &self,
        limit: i64,
        cursor: &str,
        include_locale: bool,
    ) -> ClientResult<crate::types::UsersListSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !cursor.is_empty() {
            query_args.push(("cursor".to_string(), cursor.to_string()));
        }
        if include_locale {
            query_args.push(("include_locale".to_string(), include_locale.to_string()));
        }
        if limit > 0 {
            query_args.push(("limit".to_string(), limit.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self.client.url(&format!("/users.list?{}", query_), None);
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
     * This function performs a `GET` to the `/users.lookupByEmail` endpoint.
     *
     * Find a user with an email address.
     *
     * FROM: <https://api.slack.com/methods/users.lookupByEmail>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `users:read.email`.
     * * `email: &str` -- An email address belonging to a user in the workspace.
     */
    pub async fn lookup_email(
        &self,
        email: &str,
    ) -> ClientResult<crate::types::UsersInfoSuccessSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !email.is_empty() {
            query_args.push(("email".to_string(), email.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/users.lookupByEmail?{}", query_), None);
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
     * This function performs a `POST` to the `/users.setActive` endpoint.
     *
     * Marked a user as active. Deprecated and non-functional.
     *
     * FROM: <https://api.slack.com/methods/users.setActive>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `users:write`.
     */
    pub async fn set_active(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/users.setActive", None);
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
    /**
     * This function performs a `POST` to the `/users.setPhoto` endpoint.
     *
     * Set the user profile photo
     *
     * FROM: <https://api.slack.com/methods/users.setPhoto>
     */
    pub async fn set_photo(&self) -> ClientResult<crate::types::UsersSetPhotoSchema> {
        let url = self.client.url("/users.setPhoto", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/users.setPresence` endpoint.
     *
     * Manually sets user presence.
     *
     * FROM: <https://api.slack.com/methods/users.setPresence>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `users:write`.
     */
    pub async fn set_presence(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/users.setPresence", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: None,
                    content_type: Some("application/x-www-form-urlencoded".to_string()),
                },
            )
            .await
    }
}
