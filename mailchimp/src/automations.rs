use anyhow::Result;

use crate::Client;

pub struct Automations {
    pub client: Client,
}

impl Automations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Automations { client }
    }

    /**
    * List automations.
    *
    * This function performs a `GET` to the `/automations` endpoint.
    *
    * Get a summary of an account's classic automations.
    *
    * **Parameters:**
    *
    * * `count: i64` -- The number of records to return. Default value is 10. Maximum value is 1000.
    * * `offset: i64` -- Used for [pagination](https://mailchimp.com/developer/marketing/docs/methods-parameters/#pagination), this it the number of records from a collection to skip. Default value is 0.
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `before_create_time: chrono::DateTime<chrono::Utc>` -- Restrict the response to automations created before this time. Uses the ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `since_create_time: chrono::DateTime<chrono::Utc>` -- Restrict the response to automations created after this time. Uses the ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `before_start_time: chrono::DateTime<chrono::Utc>` -- Restrict the response to automations started before this time. Uses the ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `since_start_time: chrono::DateTime<chrono::Utc>` -- Restrict the response to automations started after this time. Uses the ISO 8601 time format: 2015-10-21T15:41:36+00:00.
    * * `status: crate::types::Status` -- Restrict the results to automations with the specified status.
    */
    pub async fn get(
        &self,
        count: i64,
        offset: i64,
        fields: &[String],
        exclude_fields: &[String],
        before_create_time: Option<chrono::DateTime<chrono::Utc>>,
        since_create_time: Option<chrono::DateTime<chrono::Utc>>,
        before_start_time: Option<chrono::DateTime<chrono::Utc>>,
        since_start_time: Option<chrono::DateTime<chrono::Utc>>,
        status: crate::types::Status,
    ) -> Result<crate::types::GetAutomationsResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if let Some(date) = before_create_time {
            query_args.push(("before_create_time".to_string(), date.to_rfc3339()));
        }
        if let Some(date) = before_start_time {
            query_args.push(("before_start_time".to_string(), date.to_rfc3339()));
        }
        if count > 0 {
            query_args.push(("count".to_string(), count.to_string()));
        }
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        if offset > 0 {
            query_args.push(("offset".to_string(), offset.to_string()));
        }
        if let Some(date) = since_create_time {
            query_args.push(("since_create_time".to_string(), date.to_rfc3339()));
        }
        if let Some(date) = since_start_time {
            query_args.push(("since_start_time".to_string(), date.to_rfc3339()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/automations?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * Add automation.
    *
    * This function performs a `POST` to the `/automations` endpoint.
    *
    * Create a new classic automation in your Mailchimp account.
    */
    pub async fn post(
        &self,
        body: &crate::types::AutomationWorkflow,
    ) -> Result<crate::types::Automations> {
        let url = "/automations".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get automation info.
    *
    * This function performs a `GET` to the `/automations/{workflow_id}` endpoint.
    *
    * Get a summary of an individual classic automation workflow's settings and content. The `trigger_settings` object returns information for the first email in the workflow.
    *
    * **Parameters:**
    *
    * * `fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `exclude_fields: &[String]` -- A comma-separated list of fields to return. Reference parameters of sub-objects with dot notation.
    * * `workflow_id: &str` -- The unique id for the Automation workflow.
    */
    pub async fn get_automations(
        &self,
        fields: &[String],
        exclude_fields: &[String],
        workflow_id: &str,
    ) -> Result<crate::types::Automations> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !exclude_fields.is_empty() {
            query_args.push(("exclude_fields".to_string(), exclude_fields.join(" ")));
        }
        if !fields.is_empty() {
            query_args.push(("fields".to_string(), fields.join(" ")));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/automations/{}?{}",
            crate::progenitor_support::encode_path(&workflow_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Pause automation emails.
    *
    * This function performs a `POST` to the `/automations/{workflow_id}/actions/pause-all-emails` endpoint.
    *
    * Pause all emails in a specific classic automation workflow.
    *
    * **Parameters:**
    *
    * * `workflow_id: &str` -- The unique id for the Automation workflow.
    */
    pub async fn post_actions_pause_all_email(&self, workflow_id: &str) -> Result<()> {
        let url = format!(
            "/automations/{}/actions/pause-all-emails",
            crate::progenitor_support::encode_path(&workflow_id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * Start automation emails.
    *
    * This function performs a `POST` to the `/automations/{workflow_id}/actions/start-all-emails` endpoint.
    *
    * Start all emails in a classic automation workflow.
    *
    * **Parameters:**
    *
    * * `workflow_id: &str` -- The unique id for the Automation workflow.
    */
    pub async fn post_actions_start_all_email(&self, workflow_id: &str) -> Result<()> {
        let url = format!(
            "/automations/{}/actions/start-all-emails",
            crate::progenitor_support::encode_path(&workflow_id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * Archive automation.
    *
    * This function performs a `POST` to the `/automations/{workflow_id}/actions/archive` endpoint.
    *
    * Archiving will permanently end your automation and keep the report data. You’ll be able to replicate your archived automation, but you can’t restart it.
    *
    * **Parameters:**
    *
    * * `workflow_id: &str` -- The unique id for the Automation workflow.
    */
    pub async fn archive(&self, workflow_id: &str) -> Result<()> {
        let url = format!(
            "/automations/{}/actions/archive",
            crate::progenitor_support::encode_path(&workflow_id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * List automated emails.
    *
    * This function performs a `GET` to the `/automations/{workflow_id}/emails` endpoint.
    *
    * Get a summary of the emails in a classic automation workflow.
    *
    * **Parameters:**
    *
    * * `workflow_id: &str` -- The unique id for the Automation workflow.
    */
    pub async fn get_email(&self, workflow_id: &str) -> Result<crate::types::AutomationEmails> {
        let url = format!(
            "/automations/{}/emails",
            crate::progenitor_support::encode_path(&workflow_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * Get workflow email info.
    *
    * This function performs a `GET` to the `/automations/{workflow_id}/emails/{workflow_email_id}` endpoint.
    *
    * Get information about an individual classic automation workflow email.
    *
    * **Parameters:**
    *
    * * `workflow_id: &str` -- The unique id for the Automation workflow.
    * * `workflow_email_id: &str` -- The unique id for the Automation workflow email.
    */
    pub async fn get_email_automations(
        &self,
        workflow_id: &str,
        workflow_email_id: &str,
    ) -> Result<crate::types::Emails> {
        let url = format!(
            "/automations/{}/emails/{}",
            crate::progenitor_support::encode_path(&workflow_id.to_string()),
            crate::progenitor_support::encode_path(&workflow_email_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * Delete workflow email.
    *
    * This function performs a `DELETE` to the `/automations/{workflow_id}/emails/{workflow_email_id}` endpoint.
    *
    * Removes an individual classic automation workflow email. Emails from certain workflow types, including the Abandoned Cart Email (abandonedCart) and Product Retargeting Email (abandonedBrowse) Workflows, cannot be deleted.
    *
    * **Parameters:**
    *
    * * `workflow_id: &str` -- The unique id for the Automation workflow.
    * * `workflow_email_id: &str` -- The unique id for the Automation workflow email.
    */
    pub async fn delete_emails(&self, workflow_id: &str, workflow_email_id: &str) -> Result<()> {
        let url = format!(
            "/automations/{}/emails/{}",
            crate::progenitor_support::encode_path(&workflow_id.to_string()),
            crate::progenitor_support::encode_path(&workflow_email_id.to_string()),
        );

        self.client.delete(&url, None).await
    }

    /**
    * Update workflow email.
    *
    * This function performs a `PATCH` to the `/automations/{workflow_id}/emails/{workflow_email_id}` endpoint.
    *
    * Update settings for a classic automation workflow email
    *
    * **Parameters:**
    *
    * * `workflow_id: &str` -- The unique id for the Automation workflow.
    * * `workflow_email_id: &str` -- The unique id for the Automation workflow email.
    */
    pub async fn patch_email_workflow(
        &self,
        workflow_id: &str,
        workflow_email_id: &str,
        body: &crate::types::UpdateInformationAboutASpecificWorkflowEmail,
    ) -> Result<crate::types::Emails> {
        let url = format!(
            "/automations/{}/emails/{}",
            crate::progenitor_support::encode_path(&workflow_id.to_string()),
            crate::progenitor_support::encode_path(&workflow_email_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List automated email subscribers.
    *
    * This function performs a `GET` to the `/automations/{workflow_id}/emails/{workflow_email_id}/queue` endpoint.
    *
    * Get information about a classic automation email queue.
    *
    * **Parameters:**
    *
    * * `workflow_id: &str` -- The unique id for the Automation workflow.
    * * `workflow_email_id: &str` -- The unique id for the Automation workflow email.
    */
    pub async fn get_emails_queue(
        &self,
        workflow_id: &str,
        workflow_email_id: &str,
    ) -> Result<crate::types::GetAutomationsEmailsQueueResponse> {
        let url = format!(
            "/automations/{}/emails/{}/queue",
            crate::progenitor_support::encode_path(&workflow_id.to_string()),
            crate::progenitor_support::encode_path(&workflow_email_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * Add subscriber to workflow email.
    *
    * This function performs a `POST` to the `/automations/{workflow_id}/emails/{workflow_email_id}/queue` endpoint.
    *
    * Manually add a subscriber to a workflow, bypassing the default trigger settings. You can also use this endpoint to trigger a series of automated emails in an API 3.0 workflow type.
    *
    * **Parameters:**
    *
    * * `workflow_id: &str` -- The unique id for the Automation workflow.
    * * `workflow_email_id: &str` -- The unique id for the Automation workflow email.
    */
    pub async fn post_emails_queue(
        &self,
        workflow_id: &str,
        workflow_email_id: &str,
        body: &crate::types::SubscriberInAutomationQueue,
    ) -> Result<crate::types::SubscriberInAutomationQueueData> {
        let url = format!(
            "/automations/{}/emails/{}/queue",
            crate::progenitor_support::encode_path(&workflow_id.to_string()),
            crate::progenitor_support::encode_path(&workflow_email_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get automated email subscriber.
    *
    * This function performs a `GET` to the `/automations/{workflow_id}/emails/{workflow_email_id}/queue/{subscriber_hash}` endpoint.
    *
    * Get information about a specific subscriber in a classic automation email queue.
    *
    * **Parameters:**
    *
    * * `workflow_id: &str` -- The unique id for the Automation workflow.
    * * `workflow_email_id: &str` -- The unique id for the Automation workflow email.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    */
    pub async fn get_emails_queue_automations(
        &self,
        workflow_id: &str,
        workflow_email_id: &str,
        subscriber_hash: &str,
    ) -> Result<crate::types::SubscriberInAutomationQueueData> {
        let url = format!(
            "/automations/{}/emails/{}/queue/{}",
            crate::progenitor_support::encode_path(&workflow_id.to_string()),
            crate::progenitor_support::encode_path(&workflow_email_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * Pause automated email.
    *
    * This function performs a `POST` to the `/automations/{workflow_id}/emails/{workflow_email_id}/actions/pause` endpoint.
    *
    * Pause an automated email.
    *
    * **Parameters:**
    *
    * * `workflow_id: &str` -- The unique id for the Automation workflow.
    * * `workflow_email_id: &str` -- The unique id for the Automation workflow email.
    */
    pub async fn post_emails_actions_pause(
        &self,
        workflow_id: &str,
        workflow_email_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/automations/{}/emails/{}/actions/pause",
            crate::progenitor_support::encode_path(&workflow_id.to_string()),
            crate::progenitor_support::encode_path(&workflow_email_id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * Start automated email.
    *
    * This function performs a `POST` to the `/automations/{workflow_id}/emails/{workflow_email_id}/actions/start` endpoint.
    *
    * Start an automated email.
    *
    * **Parameters:**
    *
    * * `workflow_id: &str` -- The unique id for the Automation workflow.
    * * `workflow_email_id: &str` -- The unique id for the Automation workflow email.
    */
    pub async fn post_emails_actions_start(
        &self,
        workflow_id: &str,
        workflow_email_id: &str,
    ) -> Result<()> {
        let url = format!(
            "/automations/{}/emails/{}/actions/start",
            crate::progenitor_support::encode_path(&workflow_id.to_string()),
            crate::progenitor_support::encode_path(&workflow_email_id.to_string()),
        );

        self.client.post(&url, None).await
    }

    /**
    * List subscribers removed from workflow.
    *
    * This function performs a `GET` to the `/automations/{workflow_id}/removed-subscribers` endpoint.
    *
    * Get information about subscribers who were removed from a classic automation workflow.
    *
    * **Parameters:**
    *
    * * `workflow_id: &str` -- The unique id for the Automation workflow.
    */
    pub async fn get_removed_subscriber(
        &self,
        workflow_id: &str,
    ) -> Result<crate::types::RemovedSubscribers> {
        let url = format!(
            "/automations/{}/removed-subscribers",
            crate::progenitor_support::encode_path(&workflow_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * Remove subscriber from workflow.
    *
    * This function performs a `POST` to the `/automations/{workflow_id}/removed-subscribers` endpoint.
    *
    * Remove a subscriber from a specific classic automation workflow. You can remove a subscriber at any point in an automation workflow, regardless of how many emails they've been sent from that workflow. Once they're removed, they can never be added back to the same workflow.
    *
    * **Parameters:**
    *
    * * `workflow_id: &str` -- The unique id for the Automation workflow.
    */
    pub async fn post_removed_subscriber(
        &self,
        workflow_id: &str,
        body: &crate::types::SubscriberInAutomationQueue,
    ) -> Result<crate::types::Subscribers> {
        let url = format!(
            "/automations/{}/removed-subscribers",
            crate::progenitor_support::encode_path(&workflow_id.to_string()),
        );

        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Get subscriber removed from workflow.
    *
    * This function performs a `GET` to the `/automations/{workflow_id}/removed-subscribers/{subscriber_hash}` endpoint.
    *
    * Get information about a specific subscriber who was removed from a classic automation workflow.
    *
    * **Parameters:**
    *
    * * `workflow_id: &str` -- The unique id for the Automation workflow.
    * * `subscriber_hash: &str` -- The MD5 hash of the lowercase version of the list member's email address.
    */
    pub async fn get_removed_subscriber_automations(
        &self,
        workflow_id: &str,
        subscriber_hash: &str,
    ) -> Result<crate::types::Subscribers> {
        let url = format!(
            "/automations/{}/removed-subscribers/{}",
            crate::progenitor_support::encode_path(&workflow_id.to_string()),
            crate::progenitor_support::encode_path(&subscriber_hash.to_string()),
        );

        self.client.get(&url, None).await
    }
}
