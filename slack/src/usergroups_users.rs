use anyhow::Result;

use crate::Client;

pub struct UsergroupsUsers {
    pub client: Client,
}

impl UsergroupsUsers {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        UsergroupsUsers { client }
    }

    /**
    * This function performs a `GET` to the `/usergroups.users.list` endpoint.
    *
    * List all users in a User Group
    *
    * FROM: <https://api.slack.com/methods/usergroups.users.list>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `usergroups:read`.
    * * `include_disabled: bool` -- Allow results that involve disabled User Groups.
    * * `usergroup: &str` -- The encoded ID of the User Group to update.
    */
    pub async fn list(
        &self,
        include_disabled: bool,
        usergroup: &str,
    ) -> Result<crate::types::UsergroupsUsersListSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if include_disabled {
            query_args.push(("include_disabled".to_string(), include_disabled.to_string()));
        }
        if !usergroup.is_empty() {
            query_args.push(("usergroup".to_string(), usergroup.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/usergroups.users.list?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * This function performs a `POST` to the `/usergroups.users.update` endpoint.
    *
    * Update the list of users for a User Group
    *
    * FROM: <https://api.slack.com/methods/usergroups.users.update>
    *
    * **Parameters:**
    *
    * * `token: &str` -- Authentication token. Requires scope: `usergroups:write`.
    */
    pub async fn update(&self) -> Result<crate::types::UsergroupsCreateSchema> {
        let url = "/usergroups.users.update".to_string();
        self.client.post(&url, None).await
    }
}
