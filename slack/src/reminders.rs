use anyhow::Result;

use crate::Client;

pub struct Reminders {
    pub client: Client,
}

impl Reminders {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Reminders { client }
    }

    /**
     * This function performs a `POST` to the `/reminders.add` endpoint.
     *
     * Creates a reminder.
     *
     * FROM: <https://api.slack.com/methods/reminders.add>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `reminders:write`.
     */
    pub async fn add(&self, token: &str) -> Result<crate::types::RemindersAddSchema> {
        let url = "/reminders.add".to_string();
        self.client.post(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/reminders.complete` endpoint.
     *
     * Marks a reminder as complete.
     *
     * FROM: <https://api.slack.com/methods/reminders.complete>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `reminders:write`.
     */
    pub async fn complete(&self, token: &str) -> Result<crate::types::DndEndSchema> {
        let url = "/reminders.complete".to_string();
        self.client.post(&url, None).await
    }

    /**
     * This function performs a `POST` to the `/reminders.delete` endpoint.
     *
     * Deletes a reminder.
     *
     * FROM: <https://api.slack.com/methods/reminders.delete>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `reminders:write`.
     */
    pub async fn delete(&self, token: &str) -> Result<crate::types::DndEndSchema> {
        let url = "/reminders.delete".to_string();
        self.client.post(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/reminders.info` endpoint.
     *
     * Gets information about a reminder.
     *
     * FROM: <https://api.slack.com/methods/reminders.info>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `reminders:read`.
     * * `reminder: &str` -- The ID of the reminder.
     */
    pub async fn info(
        &self,
        token: &str,
        reminder: &str,
    ) -> Result<crate::types::RemindersAddSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !reminder.is_empty() {
            query_args.push(("reminder".to_string(), reminder.to_string()));
        }
        if !token.is_empty() {
            query_args.push(("token".to_string(), token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/reminders.info?{}", query_);

        self.client.get(&url, None).await
    }

    /**
     * This function performs a `GET` to the `/reminders.list` endpoint.
     *
     * Lists all reminders created by or for a given user.
     *
     * FROM: <https://api.slack.com/methods/reminders.list>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `reminders:read`.
     */
    pub async fn list(&self, token: &str) -> Result<crate::types::RemindersListSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !token.is_empty() {
            query_args.push(("token".to_string(), token.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/reminders.list?{}", query_);

        self.client.get(&url, None).await
    }
}
