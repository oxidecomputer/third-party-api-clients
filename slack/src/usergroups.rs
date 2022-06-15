use anyhow::Result;

use crate::Client;

pub struct Usergroups {
    pub client: Client,
}

impl Usergroups {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Usergroups { client }
    }

    /**
     * This function performs a `POST` to the `/usergroups.create` endpoint.
     *
     * Create a User Group
     *
     * FROM: <https://api.slack.com/methods/usergroups.create>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `usergroups:write`.
     */
    pub async fn create(&self) -> Result<crate::types::UsergroupsCreateSchema> {
        let url = "/usergroups.create".to_string();
        self.client.post(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/usergroups.disable` endpoint.
     *
     * Disable an existing User Group
     *
     * FROM: <https://api.slack.com/methods/usergroups.disable>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `usergroups:write`.
     */
    pub async fn disable(&self) -> Result<crate::types::UsergroupsCreateSchema> {
        let url = "/usergroups.disable".to_string();
        self.client.post(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/usergroups.enable` endpoint.
     *
     * Enable a User Group
     *
     * FROM: <https://api.slack.com/methods/usergroups.enable>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `usergroups:write`.
     */
    pub async fn enable(&self) -> Result<crate::types::UsergroupsCreateSchema> {
        let url = "/usergroups.enable".to_string();
        self.client.post(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/usergroups.list` endpoint.
     *
     * List all User Groups for a team
     *
     * FROM: <https://api.slack.com/methods/usergroups.list>
     *
     * **Parameters:**
     *
     * * `include_users: bool` -- Include the list of users for each User Group.
     * * `token: &str` -- Authentication token. Requires scope: `usergroups:read`.
     * * `include_count: bool` -- Include the number of users in each User Group.
     * * `include_disabled: bool` -- Include disabled User Groups.
     */
    pub async fn list(
        &self,
        include_users: bool,
        include_count: bool,
        include_disabled: bool,
    ) -> Result<crate::types::UsergroupsListSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if include_count {
            query_args.push(("include_count".to_string(), include_count.to_string()));
        }
        if include_disabled {
            query_args.push(("include_disabled".to_string(), include_disabled.to_string()));
        }
        if include_users {
            query_args.push(("include_users".to_string(), include_users.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/usergroups.list?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/usergroups.update` endpoint.
     *
     * Update an existing User Group
     *
     * FROM: <https://api.slack.com/methods/usergroups.update>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `usergroups:write`.
     */
    pub async fn update(&self) -> Result<crate::types::UsergroupsCreateSchema> {
        let url = "/usergroups.update".to_string();
        self.client.post(&url, None).await
    }
}
