use crate::Client;
use crate::ClientResult;

pub struct InlineHooks {
    pub client: Client,
}

impl InlineHooks {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        InlineHooks { client }
    }

    /**
     * This function performs a `GET` to the `/api/v1/inlineHooks` endpoint.
     *
     * Success
     *
     * **Parameters:**
     *
     * * `type_: &str`
     */
    pub async fn list(&self, type_: &str) -> ClientResult<Vec<crate::types::InlineHook>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/api/v1/inlineHooks?{}", query_), None);
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
     * This function performs a `GET` to the `/api/v1/inlineHooks` endpoint.
     *
     * As opposed to `list`, this function returns all the pages of the request at once.
     *
     * Success
     */
    pub async fn list_all(&self, type_: &str) -> ClientResult<Vec<crate::types::InlineHook>> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !type_.is_empty() {
            query_args.push(("type".to_string(), type_.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = self
            .client
            .url(&format!("/api/v1/inlineHooks?{}", query_), None);
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
     * This function performs a `POST` to the `/api/v1/inlineHooks` endpoint.
     *
     * Success
     */
    pub async fn create(
        &self,
        body: &crate::types::InlineHook,
    ) -> ClientResult<crate::types::InlineHook> {
        let url = self.client.url("/api/v1/inlineHooks", None);
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * This function performs a `GET` to the `/api/v1/inlineHooks/{inlineHookId}` endpoint.
     *
     * Gets an inline hook by ID
     *
     * **Parameters:**
     *
     * * `inline_hook_id: &str`
     */
    pub async fn get(&self, inline_hook_id: &str) -> ClientResult<crate::types::InlineHook> {
        let url = self.client.url(
            &format!(
                "/api/v1/inlineHooks/{}",
                crate::progenitor_support::encode_path(inline_hook_id),
            ),
            None,
        );
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
     * This function performs a `PUT` to the `/api/v1/inlineHooks/{inlineHookId}` endpoint.
     *
     * Updates an inline hook by ID
     *
     * **Parameters:**
     *
     * * `inline_hook_id: &str`
     */
    pub async fn update(
        &self,
        inline_hook_id: &str,
        body: &crate::types::InlineHook,
    ) -> ClientResult<crate::types::InlineHook> {
        let url = self.client.url(
            &format!(
                "/api/v1/inlineHooks/{}",
                crate::progenitor_support::encode_path(inline_hook_id),
            ),
            None,
        );
        self.client
            .put(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: None,
                },
            )
            .await
    }
    /**
     * This function performs a `DELETE` to the `/api/v1/inlineHooks/{inlineHookId}` endpoint.
     *
     * Deletes the Inline Hook matching the provided id. Once deleted, the Inline Hook is unrecoverable. As a safety precaution, only Inline Hooks with a status of INACTIVE are eligible for deletion.
     *
     * **Parameters:**
     *
     * * `inline_hook_id: &str`
     */
    pub async fn delete(&self, inline_hook_id: &str) -> ClientResult<()> {
        let url = self.client.url(
            &format!(
                "/api/v1/inlineHooks/{}",
                crate::progenitor_support::encode_path(inline_hook_id),
            ),
            None,
        );
        self.client
            .delete(
                &url,
                crate::Message {
                    body: None,
                    content_type: None,
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/api/v1/inlineHooks/{inlineHookId}/execute` endpoint.
     *
     * Executes the Inline Hook matching the provided inlineHookId using the request body as the input. This will send the provided data through the Channel and return a response if it matches the correct data contract. This execution endpoint should only be used for testing purposes.
     *
     * **Parameters:**
     *
     * * `inline_hook_id: &str`
     */
    pub async fn execute(
        &self,
        inline_hook_id: &str,
        body: &crate::types::Links,
    ) -> ClientResult<crate::types::InlineHookResponse> {
        let url = self.client.url(
            &format!(
                "/api/v1/inlineHooks/{}/execute",
                crate::progenitor_support::encode_path(inline_hook_id),
            ),
            None,
        );
        self.client
            .post(
                &url,
                crate::Message {
                    body: Some(reqwest::Body::from(serde_json::to_vec(body)?)),
                    content_type: Some("application/json".to_string()),
                },
            )
            .await
    }
    /**
     * This function performs a `POST` to the `/api/v1/inlineHooks/{inlineHookId}/lifecycle/activate` endpoint.
     *
     * Activates the Inline Hook matching the provided id
     *
     * **Parameters:**
     *
     * * `inline_hook_id: &str`
     */
    pub async fn activate(&self, inline_hook_id: &str) -> ClientResult<crate::types::InlineHook> {
        let url = self.client.url(
            &format!(
                "/api/v1/inlineHooks/{}/lifecycle/activate",
                crate::progenitor_support::encode_path(inline_hook_id),
            ),
            None,
        );
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
     * This function performs a `POST` to the `/api/v1/inlineHooks/{inlineHookId}/lifecycle/deactivate` endpoint.
     *
     * Deactivates the Inline Hook matching the provided id
     *
     * **Parameters:**
     *
     * * `inline_hook_id: &str`
     */
    pub async fn deactivate(&self, inline_hook_id: &str) -> ClientResult<crate::types::InlineHook> {
        let url = self.client.url(
            &format!(
                "/api/v1/inlineHooks/{}/lifecycle/deactivate",
                crate::progenitor_support::encode_path(inline_hook_id),
            ),
            None,
        );
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
}
