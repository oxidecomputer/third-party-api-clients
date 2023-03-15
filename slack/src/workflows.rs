use crate::Client;
use crate::ClientResult;

pub struct Workflows {
    pub client: Client,
}

impl Workflows {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Workflows { client }
    }

    /**
     * This function performs a `GET` to the `/workflows.stepCompleted` endpoint.
     *
     * Indicate that an app's step in a workflow completed execution.
     *
     * FROM: <https://api.slack.com/methods/workflows.stepCompleted>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `workflow.steps:execute`.
     * * `workflow_step_execute_id: &str` -- Context identifier that maps to the correct workflow step execution.
     * * `outputs: &str` -- Key-value object of outputs from your step. Keys of this object reflect the configured `key` properties of your [`outputs`](/reference/workflows/workflow_step#output) array from your `workflow_step` object.
     */
    pub async fn step_completed(
        &self,
        workflow_step_execute_id: &str,
        outputs: &str,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !outputs.is_empty() {
            query_args.push(("outputs".to_string(), outputs.to_string()));
        }
        if !workflow_step_execute_id.is_empty() {
            query_args.push((
                "workflow_step_execute_id".to_string(),
                workflow_step_execute_id.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/workflows.stepCompleted?{}", query_), None);
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
     * This function performs a `GET` to the `/workflows.stepFailed` endpoint.
     *
     * Indicate that an app's step in a workflow failed to execute.
     *
     * FROM: <https://api.slack.com/methods/workflows.stepFailed>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `workflow.steps:execute`.
     * * `workflow_step_execute_id: &str` -- Context identifier that maps to the correct workflow step execution.
     * * `error: &str` -- A JSON-based object with a `message` property that should contain a human readable error message.
     */
    pub async fn step_failed(
        &self,
        workflow_step_execute_id: &str,
        error: &str,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !error.is_empty() {
            query_args.push(("error".to_string(), error.to_string()));
        }
        if !workflow_step_execute_id.is_empty() {
            query_args.push((
                "workflow_step_execute_id".to_string(),
                workflow_step_execute_id.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/workflows.stepFailed?{}", query_), None);
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
     * This function performs a `GET` to the `/workflows.updateStep` endpoint.
     *
     * Update the configuration for a workflow extension step.
     *
     * FROM: <https://api.slack.com/methods/workflows.updateStep>
     *
     * **Parameters:**
     *
     * * `token: &str` -- Authentication token. Requires scope: `workflow.steps:execute`.
     * * `workflow_step_edit_id: &str` -- A context identifier provided with `view_submission` payloads used to call back to `workflows.updateStep`.
     * * `inputs: &str` -- A JSON key-value map of inputs required from a user during configuration. This is the data your app expects to receive when the workflow step starts. **Please note**: the embedded variable format is set and replaced by the workflow system. You cannot create custom variables that will be replaced at runtime. [Read more about variables in workflow steps here](/workflows/steps#variables).
     * * `outputs: &str` -- An JSON array of output objects used during step execution. This is the data your app agrees to provide when your workflow step was executed.
     * * `step_name: &str` -- An optional field that can be used to override the step name that is shown in the Workflow Builder.
     * * `step_image_url: &str` -- An optional field that can be used to override app image that is shown in the Workflow Builder.
     */
    pub async fn update_step(
        &self,
        workflow_step_edit_id: &str,
        inputs: &str,
        outputs: &str,
        step_name: &str,
        step_image_url: &str,
    ) -> ClientResult<crate::types::DndEndSchema> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !inputs.is_empty() {
            query_args.push(("inputs".to_string(), inputs.to_string()));
        }
        if !outputs.is_empty() {
            query_args.push(("outputs".to_string(), outputs.to_string()));
        }
        if !step_image_url.is_empty() {
            query_args.push(("step_image_url".to_string(), step_image_url.to_string()));
        }
        if !step_name.is_empty() {
            query_args.push(("step_name".to_string(), step_name.to_string()));
        }
        if !workflow_step_edit_id.is_empty() {
            query_args.push((
                "workflow_step_edit_id".to_string(),
                workflow_step_edit_id.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/workflows.updateStep?{}", query_), None);
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
