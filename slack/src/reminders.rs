use crate::Client;
use crate::ClientResult;

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
    pub async fn add(&self) -> ClientResult<crate::types::RemindersAddSchema> {
        let url = self.client.url("/reminders.add", None);
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
    pub async fn complete(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/reminders.complete", None);
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
    pub async fn delete(&self) -> ClientResult<crate::types::DndEndSchema> {
        let url = self.client.url("/reminders.delete", None);
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
    pub async fn info(&self, reminder: &str) -> ClientResult<crate::types::RemindersAddSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !reminder.is_empty() {
            query_args.push(("reminder".to_string(), reminder.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/reminders.info?{}", query_), None);
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
    pub async fn list(&self) -> ClientResult<crate::types::RemindersListSchema> {
        let url = self.client.url("/reminders.list", None);
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
